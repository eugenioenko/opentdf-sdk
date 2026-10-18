#!/usr/bin/env python3
"""Install distributed packages and build independent native importing consumers."""
from pathlib import Path
import argparse
import json
import os
import shutil
import subprocess
import tarfile
import importlib.util

SDK = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('packages', SDK/'scripts/delivery-packages.py')
packages = importlib.util.module_from_spec(spec)
spec.loader.exec_module(packages)


def install(target, base):
    build = base/'packages'/target/'relative'
    destination = base/'consumers'/target
    destination.mkdir(parents=True, exist_ok=False)
    package = destination/'package'
    package.mkdir()
    env = os.environ.copy()
    env.update(GOTOOLCHAIN='go1.25.14', JAVA_HOME=str(SDK.parent/'goalchemy/.toolchains/jdk-21.0.12.1+1'))
    source = SDK/'tests/interop'/('generatedlibrary' if target == 'go' else 'generated'+target)
    logs = []

    def run(args, cwd=destination, timeout=1800):
        log = destination/('install-'+str(len(logs))+'.log')
        packages.command(args, cwd, env, log, timeout)
        logs.append({'command': [str(a) for a in args], 'log': str(log), 'status': 0, 'log_sha256': packages.sha(log)})

    if target == 'go':
        for name in packages.members(build, target):
            path = package/name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(build/name, path)
        for path in source.glob('*.go'):
            shutil.copyfile(path, destination/path.name)
        # The package's own requirements (the reference's protocol/go messages)
        # and checksums, as a consumer module importing it would resolve them.
        requirements = (package/'go.mod').read_text().split('\n', 1)[1]
        (destination/'go.mod').write_text('module delivery-native-go\n'+requirements+'\nrequire goalchemyout v0.0.0\n\nreplace goalchemyout => ./package\n')
        shutil.copyfile(package/'go.sum', destination/'go.sum')
        run(['go', 'mod', 'tidy'])
        run(['go', 'build', '-trimpath', '-o', destination/'consumer', '.'])
        command = [str(destination/'consumer')]
    elif target == 'typescript':
        run(['npm', 'pack', '--ignore-scripts', '--pack-destination', str(destination)], build)
        artifact = next(destination.glob('*.tgz'))
        with tarfile.open(artifact) as archive:
            archive.extractall(destination, filter='data')
        for name in ('main.mjs', 'token-provider.mjs', 'controlled-negatives.mjs'):
            shutil.copyfile(source/name, destination/name)
        env['TDF_TS_PACKAGE'] = str(package/'dist/index.js')
        env['TDF_TS_NODE_PACKAGE'] = str(package/'dist/node-index.js')
        scope = destination/'node_modules/@opentdf-local'
        scope.mkdir(parents=True)
        (scope/'tdf3').symlink_to(package, target_is_directory=True)
        # The package's dependencies, installed from the pinned delivery lock.
        tooling = Path(env['TDF_BROWSER_TOOLING'])
        for dependency in json.loads((package/'package.json').read_text())['dependencies']:
            (destination/'node_modules'/dependency).parent.mkdir(parents=True, exist_ok=True)
            (destination/'node_modules'/dependency).symlink_to(tooling/dependency, target_is_directory=True)
        # Type declarations must be usable through the package's public export.
        (destination/'type-consumer.ts').write_text("import {SDK, OpenTDFError, isValidTdf, type Options, type TDFOptions, type Manifest} from '@opentdf-local/tdf3';\nconst options:Options={clientId:'id',clientSecret:'secret',timeoutMillis:1000};\nconst tdfOptions:TDFOptions={attributes:['https://platform.invalid/attr/a/value/b'],segmentSize:16384};\nasync function use():Promise<void>{await using sdk=await SDK.create('https://platform.invalid',options);const object=await sdk.createTdf(new Uint8Array(),tdfOptions);const manifest:Manifest=await object.manifest();await object.close();const reader=await sdk.loadTdf(object.bytes,{kasAllowlist:[manifest.encryptionInformation.keyAccess[0].url]});try{await reader.init();}catch(e){if(e instanceof OpenTDFError)console.log(e.kind,e.httpStatus,e.requiredObligations);}await reader.close();void isValidTdf(object.bytes);}\nvoid use;\n")
        (destination/'package.json').write_text('{"type":"module"}\n')
        run([env.get('TSC_BIN', 'tsc'), '--noEmit', '--target', 'ES2022', '--lib', 'ES2022,DOM,ESNext.Disposable', '--module', 'NodeNext', '--strict', '--skipLibCheck', 'type-consumer.ts'])
        command = ['node', str(destination/'main.mjs')]
    elif target == 'java':
        for name in packages.members(build, target):
            path = package/name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(build/name, path)
        shutil.copyfile(source/'Consumer.java', destination/'Consumer.java')
        cp = ':'.join(str(p) for p in [package/'tdf3-java.jar', *sorted((package/'lib').glob('*.jar'))])
        run([Path(env['JAVA_HOME'])/'bin/javac', '-encoding', 'UTF-8', '-cp', cp, '-d', destination/'classes', 'Consumer.java'])
        command = [str(Path(env['JAVA_HOME'])/'bin/java'), '-cp', str(destination/'classes')+':'+cp, 'consumer.Consumer']
    elif target == 'csharp':
        for name in packages.members(build, target):
            path = package/name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(build/name, path)
        shutil.copyfile(source/'Consumer.cs', destination/'Consumer.cs')
        (destination/'consumer.csproj').write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><Nullable>disable</Nullable></PropertyGroup><ItemGroup><Reference Include="OpenTDF.TDF3"><HintPath>package/lib/OpenTDF.TDF3.dll</HintPath></Reference><Reference Include="System.IO.Hashing"><HintPath>package/lib/System.IO.Hashing.dll</HintPath></Reference><Reference Include="Google.Protobuf"><HintPath>package/lib/Google.Protobuf.dll</HintPath></Reference></ItemGroup></Project>\n')
        dotnet = env.get('DOTNET_BIN', str(SDK.parent/'goalchemy/.toolchains/dotnet/dotnet'))
        run([dotnet, 'build', 'consumer.csproj', '-c', 'Release', '-o', destination/'bin', '--nologo'])
        command = [dotnet, str(destination/'bin/consumer.dll')]
    elif target == 'python':
        run(['python3', '-m', 'venv', destination/'venv'])
        python = destination/'venv/bin/python'
        wheels = Path(env.get('TDF_WHEELHOUSE', str(SDK/'.local/python-tdf-library/wheels')))
        lock = json.loads((SDK/'src/hosts/python/dependencies.lock.json').read_text())
        artifacts = []
        for item in lock:
            wheel = wheels/item['artifact']
            assert packages.sha(wheel) == item['sha256'], wheel
            if item['role'] == 'runtime':
                artifacts.append(wheel)
        artifact = next((build/'dist').glob('*.whl'))
        shutil.copyfile(artifact, package/artifact.name)
        run([python, '-m', 'pip', 'install', '--no-index', '--no-deps', *artifacts, package/artifact.name])
        shutil.copyfile(source/'consumer.py', destination/'consumer.py')
        run([python, '-I', '-c', 'import opentdf_tdf3; print(opentdf_tdf3.__file__)'])
        command = [str(python), '-I', str(destination/'consumer.py')]
    elif target == 'swift':
        for name in packages.members(build, target):
            path = package/name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(build/name, path)
        shutil.copyfile(source/'Consumer.swift', destination/'Consumer.swift')
        (destination/'Package.swift').write_text('// swift-tools-version: 6.0\nimport PackageDescription\nlet package = Package(name: "TDF3Consumer", dependencies: [.package(path: "package")], targets: [.executableTarget(name: "Consumer", dependencies: [.product(name: "OpenTDFTDF3", package: "package")], path: ".", exclude: ["package"], sources: ["Consumer.swift"])], swiftLanguageModes: [.v5])\n')
        run(['swift','build','-c','release'])
        command = [str(destination/'.build/release/Consumer')]
        env['TDF_SWIFT_CONSUMER'] = command[0]
    elif target == 'rust':
        env.update(TDF_RUST_CONSUMER_OUT=str(destination), TDF_RUST_PACKAGE=str(build/'target/package/opentdf-tdf3-0.1.0.crate'))
        run([source/'install-consumer.sh'])
        package = destination/'installed/opentdf-tdf3-0.1.0'
        command = [str(destination/'consumer/target/release/tdf3-native-consumer')]
    else:
        env.update(TDF_C_CONSUMER_OUT=str(destination), TDF_C_PACKAGE=str(build/'opentdf-tdf3-c-0.1.0-linux-x86_64.tar.gz'))
        run([source/'install-consumer.sh'])
        prefix = env.get('TDF3_CURL_PREFIX', str(SDK/'.local/root-c-development-prerequisites/prefix'))
        env['LD_LIBRARY_PATH'] = prefix+'/usr/lib/x86_64-linux-gnu:'+env.get('LD_LIBRARY_PATH', '')
        command = [str(destination/'consumer')]
        # This public importer calls the SDK before any collector fixture setup.
        run([env.get('CC', 'cc'), '-std=c17', '-O2', '-Wall', '-Wextra', '-Werror',
             '-I'+str(package/'include'), source/'forward-consumer.c', package/'lib/libtdf3.a',
             '-L'+prefix+'/usr/lib/x86_64-linux-gnu', '-lcurl', '-lssl', '-lcrypto', '-lz',
             SDK.parent/'goalchemy/.toolchains/bdwgc/lib/libgc.a', '-lpthread', '-ldl',
             '-o', destination/'no-preinit-consumer'])
    exported = {'TDF_TS_PACKAGE','TDF_TS_NODE_PACKAGE','TDF_RUST_CONSUMER_OUT','TDF_RUST_PACKAGE','TDF_C_CONSUMER_OUT','TDF_C_PACKAGE','TDF3_CURL_PREFIX','LD_LIBRARY_PATH','JAVA_HOME','TDF_SWIFT_CONSUMER'}
    receipt = {'target': target, 'consumer_command': command, 'environment': {k: v for k, v in env.items() if k in exported},
               'installed_package_members': {str(p.relative_to(destination)): packages.sha(p) for p in sorted(package.rglob('*')) if p.is_file() and not {'.build', '.swiftpm'}.intersection(p.relative_to(package).parts)},
               'source_consumer_members': {str(p.relative_to(SDK)): packages.sha(p) for p in sorted(source.glob('*')) if p.is_file() and p.suffix in ('.go', '.mjs', '.java', '.cs', '.py', '.rs', '.c', '.in', '.swift')},
               'commands': logs, 'status': 0, 'independent_build_output': True}
    (destination/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
    print('PASS installed native consumer', target, flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('target', choices=(*packages.TARGETS, 'all'))
    parser.add_argument('--base', type=Path, default=SDK/'.local/phase7')
    args = parser.parse_args()
    for target in packages.TARGETS if args.target == 'all' else (args.target,):
        install(target, args.base.resolve())
