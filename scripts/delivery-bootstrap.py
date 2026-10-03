#!/usr/bin/env python3
"""Pinned native prerequisites in ignored storage; fail on missing/version drift."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys

SDK = Path(__file__).resolve().parents[1]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('target', choices=('go','typescript','java','csharp','python','rust','c','all'))
    parser.add_argument('--base', type=Path, default=SDK/'.local/phase7')
    args = parser.parse_args()
    base = args.base.resolve()
    base.mkdir(parents=True,exist_ok=True)
    environment = os.environ.copy()
    environment['GOTOOLCHAIN'] = 'go1.25.14'
    commands = []

    def run(command, cwd=SDK, timeout=1800):
        log = base/('bootstrap-'+str(len(commands))+'.log')
        with log.open('wb') as stream:
            result = subprocess.run([str(a) for a in command],cwd=cwd,env=environment,stdout=stream,stderr=subprocess.STDOUT,timeout=timeout)
        Path(str(log)+'.status').write_text(str(result.returncode)+'\n')
        commands.append({'command':[str(a) for a in command],'status':result.returncode,'log_sha256':sha(log)})
        if result.returncode:
            raise RuntimeError('bootstrap failed; see '+str(log))

    refs = json.loads((SDK/'references.lock.json').read_text())['repositories']
    for name, pin in refs.items():
        path = SDK/pin['path']
        actual = subprocess.check_output(['git','-C',str(path),'rev-parse','HEAD'],text=True).strip()
        assert actual == pin['revision'], name+' pin mismatch'
        assert not subprocess.check_output(['git','-C',str(path),'status','--porcelain','--untracked-files=no']), name+' tracked source dirty'
    artifacts = {}
    targets = ('go','typescript','java','csharp','python','rust','c') if args.target == 'all' else (args.target,)
    for target, tool in (('java','jdk'),('csharp','dotnet'),('c','bdwgc')):
        if target in targets:
            run([SDK.parent/'goalchemy/scripts/fetch-toolchains.sh',tool])
    if 'typescript' in targets:
        tooling = base/'tooling'
        tooling.mkdir(exist_ok=True)
        for name in ('package.json','package-lock.json'):
            shutil.copyfile(SDK/'hosts/delivery'/name,tooling/name)
        run(['npm','ci','--no-audit','--no-fund'],tooling)
        environment.update(TSC_BIN=str(tooling/'node_modules/.bin/tsc'),TDF_BROWSER_TOOLING=str(tooling/'node_modules'),PLAYWRIGHT_BROWSERS_PATH=str(base/'browsers'))
        run([tooling/'node_modules/.bin/playwright','install','chromium'],tooling)
        artifacts['tooling_lock_sha256'] = sha(tooling/'package-lock.json')
    if 'python' in targets:
        assert sys.version_info[:2] == (3,10), 'locked CFFI wheel requires Python3.10'
        wheels = base/'wheels'
        wheels.mkdir(exist_ok=True)
        lock = json.loads((SDK/'hosts/python/dependencies.lock.json').read_text())
        for item in lock:
            artifact = wheels/item['artifact']
            if not artifact.exists():
                name = next(v[6:] for v in item['metadata'] if v.startswith('Name: '))
                version = next(v[9:] for v in item['metadata'] if v.startswith('Version: '))
                run(['python3','-m','pip','download','--no-deps','--only-binary=:all:','-d',wheels,name+'=='+version],timeout=300)
            assert artifact.is_file() and sha(artifact) == item['sha256'], 'wheel differs from lock: '+item['artifact']
            artifacts[item['artifact']] = sha(artifact)
        venv = base/'python-build-venv'
        run(['python3','-m','venv',venv])
        run([venv/'bin/python','-m','pip','install','--no-index','--no-deps',*sorted(wheels.glob('*.whl'))])
        environment.update(TDF_PYTHON=str(venv/'bin/python'),TDF_WHEELHOUSE=str(wheels))
    if 'rust' in targets:
        assert subprocess.check_output(['rustc','--version'],text=True).startswith('rustc 1.98.0 '), 'Rust1.98.0 required'
    if 'c' in targets:
        lock = json.loads((SDK/'hosts/c/dependencies.lock.json').read_text())
        prefix = base/'curl-prefix'
        downloads = base/'native-downloads'
        downloads.mkdir(exist_ok=True)
        for item in next(d for d in lock['dependencies'] if d['name']=='libcurl')['artifacts']:
            artifact = downloads/item['artifact']
            if not artifact.exists():
                run(['apt-get','download',item['package']+':amd64='+item['version']],downloads,timeout=180)
            assert sha(artifact) == item['sha256'], 'native artifact differs from lock: '+item['artifact']
            run(['dpkg-deb','-x',artifact,prefix])
            artifacts[item['artifact']] = sha(artifact)
        for name, expected in next(d for d in lock['dependencies'] if d['name']=='OpenSSL')['runtime_sha256'].items():
            assert sha(Path('/')/name) == expected, 'OpenSSL runtime differs from tested lock: '+name
            artifacts[name] = expected
        environment['TDF3_CURL_PREFIX'] = str(prefix)
    exported = {key:value for key,value in environment.items() if key in ('GOTOOLCHAIN','TSC_BIN','TDF_BROWSER_TOOLING','PLAYWRIGHT_BROWSERS_PATH','TDF_PYTHON','TDF_WHEELHOUSE','TDF3_CURL_PREFIX')}
    (base/'environment.json').write_text(json.dumps(exported,indent=2)+'\n')
    (base/'bootstrap-receipt.json').write_text(json.dumps({'status':0,'targets':targets,'references':refs,'artifacts':artifacts,'commands':commands,'environment':exported},indent=2)+'\n')
    print('PASS pinned bootstrap',','.join(targets))


if __name__ == '__main__':
    main()
