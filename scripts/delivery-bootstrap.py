#!/usr/bin/env python3
"""Pinned native prerequisites in ignored storage; fail on missing/version drift."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys

SDK = Path(__file__).resolve().parents[1]


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def tool_version(command, environment, *, stderr_version=False):
    """Keep startup diagnostics out of version output (Java reports on stderr)."""
    result = subprocess.run([str(value) for value in command], env=environment,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.stderr:
        sys.stderr.write(result.stderr)
    if result.returncode and result.stdout:
        sys.stderr.write(result.stdout)
    result.check_returncode()
    return (result.stderr if stderr_version else result.stdout).strip()


def pinned_go_root(environment):
    """Relocated trimpath compilers need an explicit, verified standard library."""
    root = Path(tool_version(['go','env','GOROOT'],environment))
    if not root.is_absolute() or not all((root/path).is_file() for path in
            ('VERSION','src/runtime/runtime2.go','src/unsafe/unsafe.go','bin/go')):
        raise RuntimeError('pinned Go GOROOT must contain its toolchain and standard library')
    if (root/'VERSION').read_text().partition('\n')[0] != 'go1.25.14':
        raise RuntimeError('GOROOT must match Go1.25.14')
    local = {**environment,'GOROOT':str(root),'GOTOOLCHAIN':'local'}
    if not tool_version([root/'bin/go','version'],local).startswith('go version go1.25.14 '):
        raise RuntimeError('GOROOT executable must match Go1.25.14')
    return str(root.resolve())


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
    versions = {}

    version_failure = base/'bootstrap-version-failure.json'
    version_failure.unlink(missing_ok=True)

    def record_version_failure(name, reason, status=None):
        version_failure.write_text(json.dumps({'tool':name,'reason':reason,'exit_status':status})+'\n')

    def version(name, command, *, stderr_version=False):
        try:
            versions[name] = tool_version(command, environment, stderr_version=stderr_version)
        except subprocess.CalledProcessError as error:
            record_version_failure(name, 'command failed', error.returncode)
            raise
        return versions[name]

    def required_version(name, command, expected, requirement, *, prefix=False):
        actual = version(name, command)
        if not (actual.startswith(expected) if prefix else actual == expected):
            record_version_failure(name, 'version mismatch')
            raise RuntimeError(requirement)

    required_version('go',['go','version'],'go version go1.25.14 ','Go1.25.14 required',prefix=True)
    environment['GOROOT'] = pinned_go_root(environment)
    required_version('node',['node','--version'],'v24.15.0','Node24.15.0 required for stock Web and browser tooling')
    version('python',['python3','--version'])

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
    toolchains = SDK.parent/'goalchemy/.toolchains'
    toolchain_lock = dict(re.findall(r"^([A-Z0-9_]+)='([^']*)'$",(SDK.parent/'goalchemy/toolchains.lock').read_text(),re.M))
    native_archives = {'jdk':('JDK','OpenJDK21U-jdk_x64_linux_hotspot_21.0.12.1_1.tar.gz'),
                      'dotnet':('DOTNET','dotnet-sdk-8.0.425-linux-x64.tar.gz'),
                      'bdwgc':('BDWGC','gc-8.2.8.tar.gz')}
    targets = ('go','typescript','java','csharp','python','rust','c') if args.target == 'all' else (args.target,)
    for target, tool in (('java','jdk'),('csharp','dotnet'),('c','bdwgc')):
        if target in targets:
            run([SDK.parent/'goalchemy/scripts/fetch-toolchains.sh',tool])
            prefix,name = native_archives[tool]
            archive = toolchains/'downloads'/name
            assert sha(archive) == toolchain_lock[prefix+'_SHA256']
            artifacts[name] = sha(archive)
            if tool == 'jdk':
                version('java',[toolchains/'jdk-21.0.12.1+1/bin/java','-version'],stderr_version=True)
            elif tool == 'dotnet':
                required_version('dotnet',[toolchains/'dotnet/dotnet','--version'],'8.0.425','.NET8.0.425 required')
            else:
                artifacts['bdwgc/lib/libgc.a'] = sha(toolchains/'bdwgc/lib/libgc.a')
    if 'typescript' in targets:
        tooling = base/'tooling'
        tooling.mkdir(exist_ok=True)
        for name in ('package.json','package-lock.json'):
            shutil.copyfile(SDK/'src/hosts/delivery'/name,tooling/name)
        run(['npm','ci','--no-audit','--no-fund'],tooling)
        environment.update(TSC_BIN=str(tooling/'node_modules/.bin/tsc'),TDF_BROWSER_TOOLING=str(tooling/'node_modules'),PLAYWRIGHT_BROWSERS_PATH=str(base/'browsers'))
        run([tooling/'node_modules/.bin/playwright','install','chromium'],tooling)
        artifacts['tooling_lock_sha256'] = sha(tooling/'package-lock.json')
    if 'python' in targets:
        assert sys.version_info[:2] == (3,10), 'locked CFFI wheel requires Python3.10'
        wheels = base/'wheels'
        wheels.mkdir(exist_ok=True)
        lock = json.loads((SDK/'src/hosts/python/dependencies.lock.json').read_text())
        for item in lock:
            artifact = wheels/item['artifact']
            if not artifact.exists():
                name = next(v[6:] for v in item['metadata'] if v.startswith('Name: '))
                dependency_version = next(v[9:] for v in item['metadata'] if v.startswith('Version: '))
                run(['python3','-m','pip','download','--no-deps','--only-binary=:all:','-d',wheels,name+'=='+dependency_version],timeout=300)
            assert artifact.is_file() and sha(artifact) == item['sha256'], 'wheel differs from lock: '+item['artifact']
            artifacts[item['artifact']] = sha(artifact)
        venv = base/'python-build-venv'
        run(['python3','-m','venv',venv])
        run([venv/'bin/python','-m','pip','install','--no-index','--no-deps',*sorted(wheels.glob('*.whl'))])
        environment.update(TDF_PYTHON=str(venv/'bin/python'),TDF_WHEELHOUSE=str(wheels))
    if 'rust' in targets:
        required_version('rustc',['rustc','--version'],'rustc 1.98.0 ','Rust1.98.0 required',prefix=True)
    if 'c' in targets:
        lock = json.loads((SDK/'src/hosts/c/dependencies.lock.json').read_text())
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
    exported = {key:value for key,value in environment.items() if key in ('GOTOOLCHAIN','GOROOT','TSC_BIN','TDF_BROWSER_TOOLING','PLAYWRIGHT_BROWSERS_PATH','TDF_PYTHON','TDF_WHEELHOUSE','TDF3_CURL_PREFIX')}
    (base/'environment.json').write_text(json.dumps(exported,indent=2)+'\n')
    (base/'bootstrap-receipt.json').write_text(json.dumps({'status':0,'targets':targets,'references':refs,'artifacts':artifacts,'versions':versions,'toolchains_lock_sha256':sha(SDK.parent/'goalchemy/toolchains.lock'),'commands':commands,'environment':exported},indent=2)+'\n')
    print('PASS pinned bootstrap',','.join(targets))


if __name__ == '__main__':
    main()
