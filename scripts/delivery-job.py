#!/usr/bin/env python3
"""Executable pinned real-KAS job used locally and by CI.

Default focused execution validates final packages and newly missing cases.
Full accepted matrices are available only through explicit --mode full.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import shutil
import subprocess
import time

SDK = Path(__file__).resolve().parents[1]
TARGETS = ('go','typescript','java','csharp','python','rust','c')


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('target', choices=(*TARGETS,'all'))
    parser.add_argument('--mode', choices=('focused','full'), default='focused')
    parser.add_argument('--profile', choices=('basic','ec','dpop','all'), default='all')
    parser.add_argument('--base', type=Path, default=SDK/'.local/phase7')
    parser.add_argument('--packages-base', type=Path)
    parser.add_argument('--output-base', type=Path)
    parser.add_argument('--services', choices=('start','ready'), default='start')
    parser.add_argument('--resume', action='store_true', help='Reuse matching terminal focused results; never replay passed cases')
    args = parser.parse_args()
    base = args.base.resolve()
    base.mkdir(parents=True,exist_ok=True)
    project = os.environ.get('TDF_COMPOSE_PROJECT','')
    if not project.startswith('phase7-'):
        raise RuntimeError('explicit unique TDF_COMPOSE_PROJECT=phase7-... required')
    environment = os.environ.copy()
    environment['GOTOOLCHAIN'] = 'go1.25.14'
    start = time.time()
    identity = os.environ.get('GITHUB_JOB','local')+'-'+os.environ.get('GITHUB_RUN_ID',str(int(start)))+'-'+args.target+'-'+args.mode+'-'+args.profile
    job = base/'jobs'/identity
    job.mkdir(parents=True,exist_ok=False)
    commands = []
    package_base = args.packages_base.resolve() if args.packages_base else base
    output_base = args.output_base.resolve() if args.output_base else base/'focused'

    def execute(label, command, timeout=1800, cwd=SDK):
        log = job/(label+'.log')
        begun = time.time()
        with log.open('wb') as stream:
            try:
                status = subprocess.run([str(a) for a in command],cwd=cwd,env=environment,stdout=stream,stderr=subprocess.STDOUT,timeout=timeout).returncode
            except subprocess.TimeoutExpired:
                status = 124
        (job/(label+'.status')).write_text(str(status)+'\n')
        commands.append({'label':label,'command':[str(a) for a in command],'status':status,'log_sha256':sha(log),'elapsed_seconds':time.time()-begun})
        (job/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
        if status:
            public = base/'public'/identity
            public.mkdir(parents=True,exist_ok=True)
            (public/'failed-job-receipt.json').write_text(json.dumps({'status':status,'job_identity':identity,'mode':args.mode,'project':project,'scope':'terminal failing job step; raw logs/private fixtures retained separately','commands':commands,'runner_sha256':sha(Path(__file__))},indent=2)+'\n')
            raise RuntimeError(label+' failed; see '+str(log))
        print('PASS job step',identity,label,flush=True)

    targets = TARGETS if args.target == 'all' else (args.target,)
    if not args.packages_base:
        execute('bootstrap',['python3',SDK/'scripts/delivery-bootstrap.py',args.target,'--base',base])
        environment.update(json.loads((base/'environment.json').read_text()))
        compiler = base/'compiler/goalchemy'
        compiler.parent.mkdir(parents=True,exist_ok=True)
        execute('compiler',['go','build','-trimpath','-o',compiler,'./cmd/goalchemy'],cwd=SDK.parent/'goalchemy')
        execute('packages',['python3',SDK/'scripts/delivery-packages.py',args.target,'--base',base,'--compiler',compiler])
        execute('consumers',['python3',SDK/'scripts/delivery-consumers.py',args.target,'--base',base])
    elif (base/'environment.json').exists():
        environment.update(json.loads((base/'environment.json').read_text()))
    if args.services == 'start':
        execute('private-service-up',['bash',SDK/'scripts/platform.sh','up'],1800)
    execute('actual-readiness',['bash',SDK/'scripts/platform.sh','ready'],180)
    reference = SDK/'.local/go-tdf-library/stock-go'
    if not reference.exists():
        reference.parent.mkdir(parents=True,exist_ok=True)
        execute('native-reference',['go','build','-trimpath','-o',reference,'.'],300,cwd=SDK/'tests/interop/generatedreference')
    profiles = ('basic','ec','dpop') if args.profile == 'all' else (args.profile,)
    completed = []
    for profile in profiles:
        current = (SDK/'.local/profiles/active').read_text().strip() if (SDK/'.local/profiles/active').exists() else 'basic'
        if current != profile:
            execute('select-'+profile,['bash',SDK/'scripts/platform-profile.sh',profile],300)
        for target in targets:
            if args.mode == 'focused' and profile != 'basic' and target not in ('go','typescript'):
                continue  # Historical unchanged profile matrices are separately attributed.
            if args.mode == 'focused':
                result_path = output_base/target/profile/'results.json'
                if args.resume and result_path.exists():
                    receipt = json.loads(result_path.read_text())
                    assert receipt['status'] == 0
                    assert receipt['consumer_receipt_sha256'] == sha(package_base/'consumers'/target/'receipt.json')
                    assert receipt['package_receipt_sha256'] == sha(package_base/'packages'/target/'receipt.json')
                    commands.append({'label':target+'-'+profile,'reused_terminal_focused_result':str(result_path),'sha256':sha(result_path),'status':0})
                else:
                    execute(target+'-'+profile,['python3',SDK/'tests/interop/delivery/run-focused.py',target,profile,'--packages-base',package_base,'--output-base',output_base],600)
                completed.append(result_path)
            else:
                directory = SDK/'tests/interop'/('generatedlibrary' if target == 'go' else 'generated'+target)
                consumer = json.loads((package_base/'consumers'/target/'receipt.json').read_text())
                environment.update(consumer['environment'])
                installed = package_base/'consumers'/target
                if target == 'go':
                    shutil.copyfile(installed/'consumer',SDK/'.local/go-tdf-library/consumer')
                elif target == 'java': environment['TDF_JAVA_PACKAGE'] = str(installed/'package')
                elif target == 'csharp': environment['TDF_CSHARP_PACKAGE'] = str(installed/'package')
                elif target == 'python': environment['TDF_PYTHON_CONSUMER'] = str(installed/'venv/bin/python')
                elif target == 'rust': environment['TDF_RUST_CONSUMER'] = str(installed/'consumer/target/release/tdf3-native-consumer')
                elif target == 'c': environment['TDF_C_CONSUMER_OUT'] = str(installed)
                execute(target+'-'+profile,['python3',directory/('run-basic.py' if profile == 'basic' else 'run-profile.py'),*([profile] if profile != 'basic' else [])],1800)
                completed.append(SDK/'.local'/(target+'-tdf-library')/profile/'results.json')
                if profile != 'basic':
                    execute(target+'-'+profile+'-integrity',['python3',SDK/'tests/interop/delivery/run-integrity.py',target,profile,'--packages-base',package_base],600)
                elif target == 'go':
                    execute('go-controlled',[*consumer['consumer_command'],SDK,SDK/'.local/go-tdf-library/basic','controlled','transport'],180)
                elif target == 'typescript': execute('typescript-controlled',['node',directory/'controlled.mjs'],180)
                else:
                    environment['TDF_DELIVERY_ARCHIVE'] = str(SDK/'.local'/(target+'-tdf-library')/'basic/binary.generated.tdf')
                    execute(target+'-controlled',['python3',directory/'controlled.py'],300)
                if profile == 'basic' and target == 'c':
                    archive = SDK/'.local/c-tdf-library/basic/binary.go.tdf'
                    proof = archive.parent/'delivery-no-preinit'
                    proof.mkdir()
                    execute('c-public-no-preinit',[installed/'no-preinit-consumer',archive,'http://localhost:8080',proof/'payload',proof/'metadata',proof/'presence'],180)
                    assert (proof/'payload').read_bytes() == (archive.parent/'binary.input').read_bytes()
                    assert (proof/'metadata').read_bytes() == b''
                    (proof/'receipt.json').write_text(json.dumps({'status':0,'archive_sha256':sha(archive),'consumer_sha256':sha(installed/'no-preinit-consumer'),'payload_sha256':sha(proof/'payload'),'metadata_sha256':sha(proof/'metadata'),'scope':'actual public C SDK importer before fixture collector initialization'},indent=2)+'\n')
        if 'typescript' in targets:
            if args.mode == 'focused':
                # Native resume deliberately never implies browser resume: the
                # browser must have its own independently attributed result.
                execute('browser-'+profile,['node',SDK/'tests/interop/delivery/run-browser-focused.mjs',profile,package_base,output_base],600)
                completed.append(output_base/'typescript-browser'/profile/'results.json')
            else:
                execute('browser-'+profile,['node',SDK/'tests/interop/generatedtypescript/browser.mjs',profile],1800)
                completed.append(SDK/'.local/typescript-tdf-library'/('browser-'+profile)/'results.json')
    execute('coverage',['python3',SDK/'scripts/delivery-coverage.py',args.mode,'--target',args.target,'--profiles',*profiles,'--packages-base',package_base,'--output-base',output_base,'--receipt',job/'coverage.json'],180)
    receipt = {'status':0,'job_identity':identity,'mode':args.mode,'profiles':profiles,'targets':targets,'project':project,'elapsed_seconds':time.time()-start,
               'sdk_checkout_revision':subprocess.check_output(['git','-C',str(SDK),'rev-parse','HEAD'],text=True).strip(),
               'runner_source_sha256':{str(p.relative_to(SDK)):sha(p) for p in [SDK/'scripts/delivery-job.py',SDK/'tests/interop/delivery/run-focused.py',SDK/'tests/interop/delivery/delivery_capture.py',SDK/'tests/interop/delivery/run-browser-focused.mjs']},
               'commands':commands,'results':{str(path):sha(path) for path in completed},'remote_ci':bool(os.environ.get('GITHUB_RUN_ID'))}
    (job/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
    public = base/'public'/identity
    public.mkdir(parents=True,exist_ok=True)
    shutil.copyfile(job/'receipt.json',public/'job-receipt.json')
    shutil.copyfile(job/'coverage.json',public/'coverage.json')
    for target in targets:
        for kind in ('packages','consumers'):
            shutil.copyfile(package_base/kind/target/'receipt.json',public/(target+'-'+kind+'.json'))
    print(job/'receipt.json')


if __name__ == '__main__':
    main()
