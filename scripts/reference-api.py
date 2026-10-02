#!/usr/bin/env python3
"""Capture the pinned Go SDK public API for parity review.

This is an inventory, not a coverage verifier. It includes root declarations,
the SDK's Connect service interfaces and complete public subpackage documents.
Observable behavior still requires separate audits.
"""
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
from concurrent.futures import ThreadPoolExecutor

sdk = Path(__file__).resolve().parents[1]
lock = json.loads((sdk / 'references.lock.json').read_text())
reference = lock['repositories']['platform']
platform = (sdk / reference['path']).resolve()
revision = subprocess.check_output(['git', '-C', str(platform), 'rev-parse', 'HEAD'], text=True).strip()
if revision != reference['revision']:
    raise SystemExit('platform revision differs from references.lock.json')
if subprocess.check_output(['git', '-C', str(platform), 'status', '--porcelain', '--untracked-files=no'], text=True):
    raise SystemExit('tracked platform changes prevent a pinned API capture')
local = sdk / '.local'
local.mkdir(exist_ok=True)
env = dict(os.environ, GOTOOLCHAIN='go1.25.14', GOCACHE=str(local / 'go-build-cache'),
           GOMODCACHE=str(local / 'go-mod-cache'))
doc = subprocess.check_output(['go', 'doc', '-all', '.'], cwd=platform / 'sdk', env=env, text=True)
(local / 'reference-public-api.go.txt').write_text(doc)
lines = doc.splitlines()
functions = []
types = []
clients = []
values = []
i = 0
while i < len(lines):
    line = lines[i]
    if line.startswith('func '):
        signature = line
        depth = line.count('(') - line.count(')')
        while depth > 0:
            i += 1
            if i >= len(lines):
                raise SystemExit('truncated public function signature')
            depth += lines[i].count('(') - lines[i].count(')')
            signature += ' ' + lines[i].strip()
        method = re.match(r'func \(([^)]+)\) (\w+)', line)
        function = re.match(r'func (\w+)', line)
        if method:
            receiver = method.group(1).split()[-1].lstrip('*')
            name = receiver + '.' + method.group(2)
        elif function:
            name = function.group(1)
        else:
            raise SystemExit('unrecognized public function declaration: ' + line)
        functions.append({'symbol': name, 'signature': signature})
    declaration = re.match(r'type (\w+) (.*)', line)
    if declaration:
        item = {'symbol': declaration.group(1), 'declaration': line}
        if line.endswith('{'):
            j = i + 1
            members = []
            hidden = False
            while j < len(lines) and lines[j] != '}':
                member = lines[j].strip()
                if member in ('// Has unexported fields.', '// Has unexported methods.'):
                    hidden = True
                if member and not member.startswith('//'):
                    members.append(member)
                j += 1
            if j == len(lines):
                raise SystemExit('truncated public type declaration: ' + line)
            item['members'] = members
            item['has_unexported_members'] = hidden
        types.append(item)
    value = re.match(r'(const|var) (.*)', line)
    if value:
        members = []
        if value.group(2) == '(':
            j = i + 1
            while j < len(lines) and lines[j] != ')':
                member = lines[j].strip()
                if member and not member.startswith('//'):
                    members.append(member)
                j += 1
            if j == len(lines):
                raise SystemExit('truncated public value declaration: ' + line)
            block = value.group(1) + ' (\n' + '\n'.join(members) + '\n)'
        else:
            members = [value.group(2)]
            block = line
        symbols = []
        for member in members:
            symbol = re.match(r'([A-Z]\w*)\b', member)
            if not symbol:
                raise SystemExit('unsupported public value declaration: ' + member)
            symbols.append(symbol.group(1))
        values.append({'kind': value.group(1), 'symbols': symbols, 'declaration': block})
    if line == 'type SDK struct {':
        j = i + 1
        while j < len(lines) and lines[j] != '}':
            field = lines[j].strip()
            if field and not field.startswith('//'):
                name, go_type = field.split(None, 1)
                clients.append({'field': name, 'go_type': go_type})
            j += 1
    i += 1
service_interfaces = []
for source in sorted((platform / 'sdk' / 'sdkconnect').glob('*.go')):
    if source.name.endswith('_test.go'):
        continue
    contents = source.read_text()
    interface = None
    for line in contents.splitlines():
        declaration = re.fullmatch(r'type ([A-Z]\w*) interface \{', line)
        if declaration:
            if interface is not None:
                raise SystemExit('nested service interface: ' + str(source))
            interface = {
                'symbol': declaration.group(1),
                'source': str(source.relative_to(platform)),
                'source_sha256': hashlib.sha256(contents.encode()).hexdigest(),
                'methods': [],
            }
            continue
        if interface is None:
            continue
        if line == '}':
            names = [method['symbol'] for method in interface['methods']]
            if not names or len(set(names)) != len(names):
                raise SystemExit('empty or duplicate service methods: ' + str(source))
            interface['methods'].sort(key=lambda item: item['symbol'])
            service_interfaces.append(interface)
            interface = None
            continue
        signature = line.strip()
        if not signature or signature.startswith('//'):
            continue
        method = re.fullmatch(r'([A-Z]\w*)\(.*\) \(.*\)', signature)
        if not method:
            raise SystemExit('unsupported service interface declaration: ' + signature)
        interface['methods'].append({'symbol': method.group(1), 'signature': signature})
    if interface is not None:
        raise SystemExit('truncated service interface: ' + str(source))
interfaces = {item['symbol']: item for item in service_interfaces}
if len(interfaces) != len(service_interfaces):
    raise SystemExit('duplicate service interface names')
for client in clients:
    symbol = client['go_type'].removeprefix('sdkconnect.')
    if symbol not in interfaces:
        raise SystemExit('SDK service field has no captured interface: ' + client['field'])
    interfaces[symbol].setdefault('sdk_fields', []).append(client['field'])
inventory = {
    'schema_version': 3,
    'reference_revision': revision,
    'package': 'github.com/opentdf/platform/sdk',
    'toolchain': env['GOTOOLCHAIN'],
    'scope': 'Root package public function/method, type/member, constant and variable '
             'declarations, SDK client fields, and sdkconnect service interface methods. '
             'Excludes other subpackages, unexported implementation details and behavioral coverage.',
    'doc_sha256': hashlib.sha256(doc.encode()).hexdigest(),
    'functions': sorted(functions, key=lambda item: item['symbol']),
    'types': sorted(types, key=lambda item: item['symbol']),
    'values': values,
    'service_clients': clients,
    'service_interfaces': sorted(service_interfaces, key=lambda item: item['symbol']),
}
target = sdk / 'docs' / 'reference-api.json'
target.write_text(json.dumps(inventory, indent=2) + '\n')

# Preserve go doc output for every other importable package, including tooling.
# Keeping the complete declaration/document text avoids silently dropping fields,
# embedded interfaces, aliases or multiline signatures through a second parser.
listed = subprocess.check_output(
    ['go', 'list', '-f', '{{.ImportPath}} {{.Name}} {{.Dir}}', './...'],
    cwd=platform / 'sdk', env=env, text=True, timeout=60,
)
packages = []
excluded = []
for entry in listed.splitlines():
    import_path, package_name, directory = entry.split(' ', 2)
    relative = Path(directory).relative_to(platform)
    if relative == Path('sdk'):
        continue
    if 'internal' in relative.parts or package_name == 'main':
        excluded.append({'package': import_path, 'source_directory': str(relative),
                         'reason': 'internal implementation' if 'internal' in relative.parts
                         else 'command package'})
        continue
    packages.append((import_path, relative))


def capture_subpackage(package):
    import_path, relative = package
    documented = subprocess.check_output(
        ['go', 'doc', '-all', '.'], cwd=platform / relative, env=env,
        text=True, timeout=60,
    )
    sources = []
    for source in sorted((platform / relative).glob('*.go')):
        if source.name.endswith('_test.go'):
            continue
        sources.append({'path': str(source.relative_to(platform)),
                        'sha256': hashlib.sha256(source.read_bytes()).hexdigest()})
    return {'package': import_path, 'source_directory': str(relative),
            'doc_sha256': hashlib.sha256(documented.encode()).hexdigest(),
            'sources': sources, 'public_api_document': documented}


with ThreadPoolExecutor(max_workers=4) as executor:
    subpackages = list(executor.map(capture_subpackage, sorted(packages)))
subpackage_inventory = {
    'schema_version': 1,
    'reference_revision': revision,
    'toolchain': env['GOTOOLCHAIN'],
    'scope': 'Complete go doc -all documents for importable SDK subpackages. '
             'Includes experimental and tooling packages for explicit parity review. '
             'Excludes the root package (captured separately), command packages and '
             'internal implementation packages; behavioral coverage remains pending.',
    'packages': subpackages,
    'excluded_packages': sorted(excluded, key=lambda item: item['package']),
}
(sdk / 'docs' / 'reference-subpackages.json').write_text(
    json.dumps(subpackage_inventory, indent=2) + '\n'
)
print(f'Captured {len(functions)} public functions/methods, {len(types)} types, '
      f'{len(clients)} SDK service client fields, '
      f'{sum(len(item["methods"]) for item in service_interfaces)} methods in '
      f'{len(service_interfaces)} service interfaces, '
      f'{sum(len(item["symbols"]) for item in values)} constants/variables; '
      f'{len(subpackages)} additional importable package documents; '
      'behavioral coverage remains pending.')
