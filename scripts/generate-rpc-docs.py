#!/usr/bin/env python3
import json
from pathlib import Path
import re
import runpy
import sys

ROOT = Path(__file__).resolve().parents[1]
PLATFORM, OUT = map(Path, sys.argv[1:3])
sys.argv = ['generate-service-clients.py', str(PLATFORM), str(OUT / 'services')]
model = runpy.run_path(str(ROOT / 'scripts/generate-service-clients.py'))
services = model['services']
snake, camel = model['snake'], model['camel']
lower_first = model['lower_first']
sources = {language: (ROOT / 'src/hosts' / file).read_text() for language, file in {
    'go': 'go/services.go.in', 'java': 'java/PlatformServices.java.in',
    'csharp': 'csharp/Services.cs.in', 'rust': 'rust/services.rs.in',
    'swift': 'swift/Services.swift.in', 'c': 'c/opentdf_services.h',
    'typescript': 'typescript/services.ts.in', 'python': 'python/__init__.py.in',
}.items()}


def matched(pattern, source):
    match = re.search(pattern, source, re.M | re.S)
    if match is None:
        raise ValueError('Missing native declaration: ' + pattern)
    return match[1]


def native_bindings(service, method):
    interface = service['interface']
    go = matched(r'^type ' + interface + r' interface \{\n(.*?)^\}', sources['go'])
    go = matched(r'^\s*(' + method + r'\([^\n]+)', go).strip()
    java = matched(r'^ public interface ' + interface + r' \{\n(.*?)^ \}', sources['java'])
    java = '\n'.join(line.strip() for line in java.splitlines()
                     if re.search(r'\b' + lower_first(method) + r'(?:Async)?\(', line)
                     and not line.strip().startswith('default '))
    csharp = matched(r'^public interface I' + interface + r'\n\{\n(.*?)^\}', sources['csharp'])
    csharp = matched(r'^\s*(Task<[^\n]+\b' + method + r'Async\([^\n]+)', csharp).strip()
    rust = matched(r"^impl " + interface + r"<'_> \{\n(.*?)^\}", sources['rust'])
    rust = '\n'.join(line.strip().removesuffix(' {') for line in rust.splitlines()
                     if re.match(r'\s*pub fn ' + snake(method) + r'(?:_async)?\(', line))
    swift = matched(r'^public struct ' + interface + r' \{\n(.*?)^\}', sources['swift'])
    swift = matched(r'^\s*(public func ' + lower_first(method) + r'\([^\n]+?) \{', swift).strip()
    cname = 'otdf_' + snake(service['accessor']) + '_' + snake(method)
    c = matched(r'^(int ' + cname + r'\([^\n]+)', sources['c'])
    for language, declaration in [('go', go), ('java', java), ('csharp', csharp), ('rust', rust), ('swift', swift), ('c', c)]:
        if not declaration:
            raise ValueError(language + ' missing ' + service['service'] + '/' + method)
    ts_accessor = camel(service['accessor'])
    if not re.search(r'readonly ' + ts_accessor + r':', sources['typescript']):
        raise ValueError('Missing TypeScript service accessor')
    py_sync = matched(r'^    (def call\(self, request=None, \*, timeout=None\):)', sources['python'])
    py_async = matched(r'^    (async def call_async\(self, request=None\):)', sources['python'])
    declarations = {
        'go': ('type ' + interface + ' interface {\n    ' + go + '\n}', interface + '.' + method),
        'typescript': ('type ' + method + ' = PlatformServices["' + ts_accessor + '"]["' + lower_first(method) + '"];', 'PlatformServices.' + ts_accessor + '.' + lower_first(method)),
        'python': (py_sync.replace('def call(', 'def ' + snake(method) + '(') + ' ...\n' + py_async.replace('def call_async(', 'def ' + snake(method) + '_async(') + ' ...', snake(service['accessor']) + '.' + snake(method)),
        'java': (java, interface + '.' + lower_first(method)),
        'csharp': (csharp, 'I' + interface + '.' + method + 'Async'),
        'rust': (rust, interface + '::' + snake(method)),
        'swift': (swift, interface + '.' + lower_first(method)),
        'c': (c, cname),
    }
    return [{'target': target, 'name': name, 'signature': declaration}
            for target, (declaration, name) in declarations.items()]


content = json.loads((ROOT / 'docs-web/examples.json').read_text())
content['rpc_bindings'] = [
    {'method': service['service'] + '/' + method, 'bindings': native_bindings(service, method)}
    for service in services for method, _, _ in service['rpcs']
]
(OUT / 'content.json').write_text(json.dumps(content, indent=2) + '\n')
(OUT / 'services.json').write_text(json.dumps(services, indent=2) + '\n')
print(f'Prepared native declarations for {len(content["rpc_bindings"])} RPC methods across eight languages.')
