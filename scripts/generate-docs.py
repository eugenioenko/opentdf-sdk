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


def run(*args, cwd=ROOT, env=None):
    subprocess.run(args, cwd=cwd, env=env, check=True)


def generate():
    STATE.mkdir(parents=True, exist_ok=True)
    run('go', 'run', 'scripts/check-api-docs.go')
    compiler_root = Path(os.environ.get('GOALCHEMY_DOCS_ROOT', STATE / 'compiler')).resolve()
    if not compiler_root.exists():
        run('git', 'clone', '--filter=blob:none', '--no-checkout', LOCK['repository'], str(compiler_root))
        run('git', 'checkout', '--detach', LOCK['revision'], cwd=compiler_root)
    revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=compiler_root, text=True).strip()
    if 'GOALCHEMY_DOCS_ROOT' not in os.environ and revision != LOCK['revision']:
        raise SystemExit('Documentation compiler revision differs from docs-web/goalchemy.lock.json.')
    fingerprint = hashlib.sha256()
    fingerprint.update(revision.encode())
    fingerprint.update(subprocess.check_output(['git', 'diff', 'HEAD'], cwd=compiler_root))
    for path in [ROOT / 'go.mod', Path(__file__), *sorted((ROOT / 'src').rglob('*.go'))]:
        if 'hosts' not in path.parts and not path.name.endswith('_test.go'):
            fingerprint.update(str(path.relative_to(ROOT)).encode())
            fingerprint.update(path.read_bytes())
    stamp = STATE / 'source.sha256'
    digest = fingerprint.hexdigest()
    reference = SITE / 'public/reference'
    if stamp.exists() and stamp.read_text() == digest and (SITE / 'public/api.json').exists() and (reference / 'index.md').exists() and (SITE / 'src/content/docs/api/index.md').exists():
        print('API reference matches the current source and documentation compiler.')
        return
    compiler = STATE / 'goalchemy'
    env = {**os.environ, 'GOTOOLCHAIN': LOCK['go_toolchain'], 'GOWORK': 'off'}
    env.pop('GOFLAGS', None)
    run('go', 'build', '-o', str(compiler), './cmd/goalchemy', cwd=compiler_root, env=env)
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
                '-format', format, '-title', 'OpenTDF SDK', '-out', str(output),
                './src', './src/library', './src/tdf', env=env)
    catalog = json.loads((SITE / 'public/api.json').read_text())
    unavailable = [target for target in catalog['targets'] if target['status'] != 'available']
    missing = [symbol['id'] for symbol in catalog['symbols'] if symbol['root'] and not symbol.get('description')]
    missing += [symbol['id'] + '.' + field['name'] for symbol in catalog['symbols'] if symbol['root']
                for field in symbol.get('fields', []) if not field.get('description')]
    if len(catalog['targets']) != 8 or unavailable or missing:
        raise SystemExit(f'Incomplete API reference: unavailable={unavailable}, undocumented={missing}')
    shutil.copytree(STATE / 'reference', reference, dirs_exist_ok=True)
    for path in reference.rglob('.goalchemy-docs.json'):
        path.unlink()
    expected = {path.relative_to(STATE / 'reference') for path in (STATE / 'reference').rglob('*') if path.is_file()}
    for path in reference.rglob('*'):
        if path.is_file() and path.relative_to(reference) not in expected:
            path.unlink()
    stamp.write_text(digest)
    print(f'Generated {len(catalog["symbols"])} API symbols with all eight native targets available.')


if __name__ == '__main__':
    generate()
