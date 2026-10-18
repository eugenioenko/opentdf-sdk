#!/usr/bin/env python3
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / 'docs-web'
STATE = ROOT / '.local/docs'
LOCK = json.loads((SITE / 'goalchemy.lock.json').read_text())
REFERENCES = json.loads((ROOT / 'references.lock.json').read_text())['repositories']


def run(*args, cwd=ROOT, env=None):
    subprocess.run(args, cwd=cwd, env=env, check=True)


def checkout(name, repository, revision, override):
    path = Path(os.environ.get(override, STATE / name)).resolve()
    if not path.exists():
        run('git', 'clone', '--filter=blob:none', '--no-checkout', repository, str(path))
        run('git', 'checkout', '--detach', revision, cwd=path)
    current = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=path, text=True).strip()
    if current != revision:
        if override in os.environ:
            if name != 'compiler':
                raise SystemExit(f'{override} must match references.lock.json revision {revision}.')
        else:
            if subprocess.check_output(['git', 'status', '--porcelain'], cwd=path, text=True).strip():
                raise SystemExit(f'Cannot update modified documentation checkout: {path}')
            run('git', 'fetch', '--depth=1', 'origin', revision, cwd=path)
            run('git', 'checkout', '--detach', revision, cwd=path)
    if name == 'platform' and subprocess.check_output(['git', 'status', '--porcelain'], cwd=path, text=True).strip():
        raise SystemExit('Platform documentation input must be a clean pinned checkout.')
    return path


def generate():
    STATE.mkdir(parents=True, exist_ok=True)
    run('go', 'run', 'scripts/check-api-docs.go')
    compiler_root = checkout('compiler', LOCK['repository'], LOCK['revision'], 'GOALCHEMY_DOCS_ROOT')
    platform_root = checkout('platform', 'https://github.com/opentdf/platform.git',
                             REFERENCES['platform']['revision'], 'PLATFORM_DOCS_ROOT')
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=compiler_root, text=True).strip()
    fingerprint = hashlib.sha256()
    fingerprint.update(revision.encode())
    fingerprint.update(subprocess.check_output(['git', 'diff', 'HEAD'], cwd=compiler_root))
    paths = [ROOT / 'go.mod', ROOT / 'references.lock.json', SITE / 'goalchemy.lock.json',
             SITE / 'examples.json', Path(__file__), ROOT / 'scripts/generate-rpc-docs.py',
             ROOT / 'scripts/generate-service-clients.py']
    paths += [path for path in (ROOT / 'src').rglob('*') if path.is_file() and not path.name.endswith('_test.go')]
    for path in sorted(paths):
        fingerprint.update(str(path.relative_to(ROOT)).encode())
        fingerprint.update(path.read_bytes())
    stamp = STATE / 'source.sha256'
    digest = fingerprint.hexdigest()
    reference = SITE / 'public/reference'
    required = ['public/api.json', 'src/data/rpc.json', 'src/content/docs/api/index.md',
                'src/components/LanguageTabs.astro', 'src/components/RpcFields.astro']
    if stamp.exists() and stamp.read_text() == digest and (reference / 'index.md').exists() and all((SITE / path).exists() for path in required):
        print('API and RPC references match the current source and pinned dependencies.')
        return
    compiler = STATE / 'goalchemy'
    env = {**os.environ, 'GOTOOLCHAIN': LOCK['go_toolchain'], 'GOWORK': 'off'}
    env.pop('GOFLAGS', None)
    run('go', 'build', '-o', str(compiler), './cmd/goalchemy', cwd=compiler_root, env=env)
    binary_dir = STATE / 'bin'
    buf = binary_dir / 'buf'
    expected_version = LOCK['buf_version']
    if not buf.exists() or subprocess.check_output([str(buf), '--version'], text=True).strip() != expected_version:
        binary_dir.mkdir(parents=True, exist_ok=True)
        run('go', 'install', 'github.com/bufbuild/buf/cmd/buf@v' + expected_version,
            env={**env, 'GOBIN': str(binary_dir)})
    descriptor = STATE / 'platform.binpb'
    run(str(buf), 'build', '--as-file-descriptor-set', '-o', str(descriptor), cwd=platform_root)
    run('python3', 'scripts/generate-rpc-docs.py', str(platform_root), str(STATE))
    services = json.loads((STATE / 'services.json').read_text())
    with tempfile.TemporaryDirectory(prefix='source-', dir=STATE) as temporary:
        stage = Path(temporary)
        shutil.copytree(ROOT / 'src', stage / 'src', ignore=shutil.ignore_patterns('hosts', '*_test.go'))
        module = re.sub(r'(?m)^replace github\.com/eugenioenko/goalchemy => .*$',
                        'replace github.com/eugenioenko/goalchemy => ' + json.dumps(str(compiler_root)),
                        (ROOT / 'go.mod').read_text())
        (stage / 'go.mod').write_text(module)
        if (ROOT / 'go.sum').exists():
            shutil.copyfile(ROOT / 'go.sum', stage / 'go.sum')
        for format, output in [('site', SITE), ('markdown', STATE / 'reference')]:
            run(str(compiler), 'docs', '-C', str(stage), '-gate', 'cooperative',
                '-proto-descriptor', str(descriptor),
                '-proto-services', ','.join(service['service'] for service in services),
                '-content', str(STATE / 'content.json'),
                '-format', format, '-title', 'OpenTDF SDK', '-out', str(output),
                './src', cwd=compiler_root, env=env)
    catalog = json.loads((SITE / 'public/api.json').read_text())
    unavailable = [target for target in catalog['targets'] if target['status'] != 'available']
    missing = [symbol['id'] for symbol in catalog['symbols'] if symbol['root'] and not symbol.get('description')]
    missing += [symbol['id'] + '.' + field['name'] for symbol in catalog['symbols'] if symbol['root']
                for field in symbol.get('fields', []) if not field.get('description')]
    if len(catalog['targets']) != 8 or unavailable or missing:
        raise SystemExit(f'Incomplete API reference: unavailable={unavailable}, undocumented={missing}')
    methods = [method for service in catalog['rpc']['services'] for method in service['methods']]
    expected = {service['service'] + '/' + method for service in services for method, _, _ in service['rpcs']}
    if {method['id'] for method in methods} != expected or any(len(method.get('bindings', [])) != 8 for method in methods):
        raise SystemExit('RPC reference must cover every exposed method in all eight native languages.')
    shutil.copytree(STATE / 'reference', reference, dirs_exist_ok=True)
    for path in reference.rglob('.goalchemy-docs.json'):
        path.unlink()
    expected = {path.relative_to(STATE / 'reference') for path in (STATE / 'reference').rglob('*') if path.is_file()}
    for path in reference.rglob('*'):
        if path.is_file() and path.relative_to(reference) not in expected:
            path.unlink()
    stamp.write_text(digest)
    print(f'Generated {len(catalog["symbols"])} SDK symbols and {len(methods)} RPC methods across eight languages.')


if __name__ == '__main__':
    generate()
