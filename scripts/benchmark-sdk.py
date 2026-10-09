#!/usr/bin/env python3
"""Build native consumers and measure contiguous public encrypt/decrypt pairs through real KAS.

Uses existing accepted installed packages; never invokes the Goalchemy compiler.
Raw outputs and private token inputs are placed under ignored .local/benchmarks.
"""
import argparse, base64, hashlib, json, os, platform, shutil, statistics, subprocess, tarfile, time, urllib.request, urllib.parse, zipfile
from pathlib import Path
SDK = Path(__file__).resolve().parents[1]
SRC = SDK / 'tests/bench'
RUNTIME_OPTIONS = ('JAVA_TOOL_OPTIONS', 'JDK_JAVA_OPTIONS', '_JAVA_OPTIONS', 'NODE_OPTIONS', 'GOGC', 'GOMEMLIMIT', 'GOMAXPROCS', 'PYTHONMALLOC', 'PYTHONOPTIMIZE', 'MALLOC_ARENA_MAX')
NATIVE_TARGETS = ['reference', 'go', 'typescript', 'java', 'csharp', 'python', 'rust', 'c']
TARGETS = NATIVE_TARGETS + ['web', 'swift']
SIZES = {'1MiB': ('1', 1024 * 1024), '10MiB': ('10', 10 * 1024 * 1024), '50MiB': ('50', 50 * 1024 * 1024)}

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def invoke(command, cwd, env, timeout=300):
    result = subprocess.run([str(v) for v in command], cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout)
    if result.returncode:
        raise RuntimeError('command failed ' + str(command[0]) + ': ' + result.stderr.decode(errors='replace')[:1800])
    return result.stdout

def normal_environment(environment):
    """Remove inherited runtime tuning; keep paths and ordinary CLI settings."""
    allowed_dotnet = {'DOTNET_ROOT', 'DOTNET_ROOT_X64', 'DOTNET_CLI_HOME', 'DOTNET_NOLOGO', 'DOTNET_CLI_TELEMETRY_OPTOUT', 'DOTNET_SKIP_FIRST_TIME_EXPERIENCE', 'DOTNET_HOST_PATH', 'DOTNET_MULTILEVEL_LOOKUP'}
    return {key: value for key, value in environment.items()
            if key not in RUNTIME_OPTIONS
            and not key.startswith('COMPlus_')
            and not (key.startswith('DOTNET_') and key not in allowed_dotnet)}


def validate_native_result(result, samples, warmups, bulk_warmups):
    for key, count in [('samples_ms', samples), ('warmup_ms', warmups), ('bulk_warmup_ms', bulk_warmups)]:
        values = result.get(key)
        if not isinstance(values, list) or len(values) != count or any(not isinstance(v, (int, float)) or not 0 < v < float('inf') for v in values):
            raise ValueError('incomplete or invalid native history: ' + key)
    if result.get('correct') is not True or result.get('warmup_count') != warmups or result.get('bulk_warmup_count') != bulk_warmups or result.get('kas_calls_expected') != samples + warmups + bulk_warmups:
        raise ValueError('native warmup/correctness counts do not match the frozen policy')


def warmup_statistics(result):
    actual, bulk, samples = result['warmup_ms'], result['bulk_warmup_ms'], result['samples_ms']
    stats = {'bulk_first20_median_ms': statistics.median(bulk[:20]) if bulk else None,
             'bulk_last20_median_ms': statistics.median(bulk[-20:]) if bulk else None,
             'actual_first10_median_ms': statistics.median(actual[:10]),
             'actual_last10_median_ms': statistics.median(actual[-10:]),
             'measured_median_ms': statistics.median(samples)}
    before, after = stats['actual_first10_median_ms'], stats['actual_last10_median_ms']
    stats['actual_downward_change_percent'] = 100 * (1 - after / before)
    stats['investigation_required'] = after < before * .8 and stats['measured_median_ms'] < before * .8
    return stats


def validate_manifest(manifest, size_bytes):
    encryption = manifest['encryptionInformation']
    integrity = encryption['integrityInformation']
    segment_bytes = 2 << 20
    if (encryption['method']['algorithm'] != 'AES-256-GCM'
            or integrity['segmentHashAlg'] != 'GMAC'
            or integrity['segmentSizeDefault'] != segment_bytes
            or len(integrity['segments']) != (size_bytes + segment_bytes - 1) // segment_bytes):
        raise ValueError('retained archive differs from the declared encryption/segment profile')
    return {'algorithm': 'AES-256-GCM', 'segment_hash_algorithm': 'GMAC',
            'segment_size_bytes': segment_bytes, 'segment_count': len(integrity['segments'])}


def aggregate_batches(rows):
    rows = sorted(rows, key=lambda row: row['batch_index'])
    first = rows[0]
    value = {key: first[key] for key in ('target', 'operation', 'size_bytes', 'size_label', 'fixture_id', 'payload_sha256', 'batches_requested')}
    expected = first['batches_requested']
    if len(rows) != expected or [r['batch_index'] for r in rows] != list(range(expected)) or any(r['status'] != 'ok' for r in rows):
        return {**value, 'status': 'failed' if any(r['status'] != 'ok' for r in rows) or len(rows) >= expected else 'pending', 'batches': rows}
    if any(r['payload_sha256'] != first['payload_sha256'] or r['samples_requested'] != first['samples_requested'] for r in rows):
        raise ValueError('inconsistent fresh-process batch inputs')
    samples = [sample for row in rows for sample in row['samples_ms']]
    return {**value, 'status': 'ok', 'samples_ms': samples, 'samples_requested': len(samples),
            'median_ms': statistics.median(samples), 'minimum_ms': min(samples), 'maximum_ms': max(samples),
            'stdev_ms': statistics.stdev(samples) if len(samples) > 1 else 0,
            'batches': rows, 'warmup_count': first['warmup_count'], 'bulk_warmup_count': first['bulk_warmup_count'],
            'investigation_required': any(r['warmup_statistics']['investigation_required'] for r in rows),
            'encrypted_samples_validated': [archive for row in rows for archive in row['encrypted_samples_validated']]}


def setup(base, packages):
    build = base / 'build'
    build.mkdir(parents=True, exist_ok=True)
    consumers = packages / 'consumers'
    receipts = {t: json.loads((consumers / t / 'receipt.json').read_text()) for t in NATIVE_TARGETS if t != 'reference'}
    env = os.environ.copy()
    env.update(receipts['go']['environment'])
    env = normal_environment(env)
    env['GOTOOLCHAIN'] = 'go1.25.14'
    env.pop('GOROOT', None)
    env['GOROOT'] = invoke(['go', 'env', 'GOROOT'], SDK, env).decode().strip()
    if not invoke(['go', 'version'], SDK, env).decode().startswith('go version go1.25.14 '):
        raise RuntimeError('matched benchmark Go1.25.14 toolchain required')
    commands = {}
    identities = {}
    for (target, receipt) in receipts.items():
        identities[target] = {'consumer_receipt_sha256': sha(consumers / target / 'receipt.json'), 'package_receipt_sha256': sha(packages / 'packages' / target / 'receipt.json'), 'installed_members': receipt['installed_package_members']}
    for (target, source) in [('go', 'generated-go.go'), ('reference', 'reference-go.go')]:
        directory = build / target
        directory.mkdir(exist_ok=True)
        shutil.copyfile(SRC / source, directory / 'main.go')
        if target == 'go':
            (directory / 'go.mod').write_text('module benchmark-generated\n\ngo 1.25\nrequire goalchemyout v0.0.0\nreplace goalchemyout => ' + str((consumers / 'go/package').resolve()) + '\n')
        else:
            module = (SDK / 'tests/interop/generatedreference/go.mod').read_text().replace('../../../../platform/', str(SDK.parent / 'platform') + '/')
            (directory / 'go.mod').write_text(module)
            shutil.copyfile(SDK / 'tests/interop/generatedreference/go.sum', directory / 'go.sum')
        invoke(['go', 'build', '-trimpath', '-o', directory / 'benchmark', '.'], directory, env)
        commands[target] = [str(directory / 'benchmark')]
    java = build / 'java'
    java.mkdir(exist_ok=True)
    java_command = receipts['java']['consumer_command']
    classpath = ':'.join(java_command[2].split(':')[1:])
    javac = Path(java_command[0]).with_name('javac')
    invoke([javac, '-encoding', 'UTF-8', '-cp', classpath, '-d', java, SRC / 'Benchmark.java', SDK / 'tests/interop/generatedjava/Consumer.java'], SDK, env)
    commands['java'] = [java_command[0], '-cp', str(java) + ':' + classpath, 'consumer.Benchmark']
    csharp = build / 'csharp'
    csharp.mkdir(exist_ok=True)
    shutil.copyfile(SRC / 'Benchmark.cs', csharp / 'Benchmark.cs')
    dll = (consumers / 'csharp/package/lib/OpenTDF.TDF3.dll').resolve()
    (csharp / 'benchmark.csproj').write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><Nullable>disable</Nullable></PropertyGroup><ItemGroup><Reference Include="OpenTDF.TDF3"><HintPath>' + str(dll) + '</HintPath></Reference><Reference Include="System.IO.Hashing"><HintPath>' + str(dll.with_name('System.IO.Hashing.dll')) + '</HintPath></Reference></ItemGroup></Project>')
    dotnet = receipts['csharp']['consumer_command'][0]
    invoke([dotnet, 'build', '-c', 'Release', '-o', csharp / 'bin', '--nologo'], csharp, env)
    commands['csharp'] = [dotnet, str(csharp / 'bin/benchmark.dll')]
    deps = (consumers / 'rust/consumer/target/release/deps').resolve()
    rust = build / 'rust'
    rust.mkdir(exist_ok=True)
    rlib = next(deps.glob('libopentdf_tdf3-*.rlib'))
    serde = next(deps.glob('libserde_json-*.rlib'))
    invoke(['rustc', '--edition=2021', '-C', 'opt-level=3', '-L', 'dependency=' + str(deps), '--extern', 'opentdf_tdf3=' + str(rlib), '--extern', 'serde_json=' + str(serde), SRC / 'rust.rs', '-o', rust / 'benchmark'], SDK, env)
    commands['rust'] = [str(rust / 'benchmark')]
    identities['rust']['linked_rlib_sha256'] = sha(rlib)
    cdir = build / 'c'
    cdir.mkdir(exist_ok=True)
    cpackage = (consumers / 'c/package').resolve()
    curl = SDK / '.local/root-c-development-prerequisites/prefix/usr/lib/x86_64-linux-gnu'
    gc = SDK.parent / 'goalchemy/.toolchains/bdwgc/lib/libgc.a'
    invoke(['cc', '-std=c17', '-O2', '-Wall', '-Wextra', '-Werror', '-I' + str(cpackage / 'include'), SRC / 'c.c', cpackage / 'lib/libtdf3.a', '-L' + str(curl), '-lcurl', '-lssl', '-lcrypto', gc, '-lpthread', '-ldl', '-o', cdir / 'benchmark'], SDK, env)
    commands['c'] = [str(cdir / 'benchmark')]
    commands['python'] = receipts['python']['consumer_command'][:2] + [str(SRC / 'python.py')]
    commands['typescript'] = [shutil.which('node', path=env['PATH']), str(SRC / 'node.mjs')]
    env['TDF_TS_NODE_PACKAGE'] = str((consumers / 'typescript/package/dist/node-index.js').resolve())
    snapshot = {'sdk_head': invoke(['git', 'rev-parse', 'HEAD'], SDK, env).decode().strip(), 'platform_head': invoke(['git', 'rev-parse', 'HEAD'], SDK.parent / 'platform', env).decode().strip(), 'references_lock_sha256': sha(SDK / 'references.lock.json'), 'packages': identities, 'sources': {str(p.relative_to(SDK)): sha(p) for p in SRC.iterdir() if p.is_file()}, 'timing_scope': 'one contiguous encrypt then decrypt interval; fixture/setup/OAuth/validation untimed', 'original_client_lifecycle': 'SDK.New once before warmup, reused client and default RSA2048 response session for all pairs in each cell', 'generated_client_lifecycle': 'stateless public facades; internal per-operation client setup and per-decrypt ephemeral RSA session remain timed', 'commands': commands, 'built_consumers': {t: sha(Path(cmd[0])) for (t, cmd) in commands.items() if t in ('reference', 'go', 'c', 'rust')}, 'system': platform.uname()._asdict(), 'cpu': next((v.partition(':')[2].strip() for v in Path('/proc/cpuinfo').read_text().splitlines() if v.startswith('model name')), ''), 'memory_kib': Path('/proc/meminfo').read_text().splitlines()[0], 'toolchains': {}}
    for (name, args) in [('go', ['go', 'version']), ('node', ['node', '--version']), ('python', [commands['python'][0], '--version']), ('java', [java_command[0], '-version']), ('dotnet', [dotnet, '--version']), ('rust', ['rustc', '--version']), ('cc', ['cc', '--version'])]:
        v = subprocess.run(args, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        snapshot['toolchains'][name] = (v.stdout + v.stderr).decode().strip()
    snapshot['normal_runtime_environment'] = {'policy': 'inherited runtime tuning removed; ordinary paths and CLI settings retained', 'inherited_option_names': sorted(key for key in os.environ if key not in normal_environment(os.environ)), 'known_options_absent': [key for key in RUNTIME_OPTIONS if key not in env], 'normalized_tuning_options_present': sorted(key for key in env if key not in normal_environment(env))}
    snapshot['controller_sha256'] = sha(Path(__file__))
    snapshot['built_source_sha256'] = {t: sha(build / t / 'main.go') for t in ('reference', 'go')}
    snapshot['consumer_artifacts_sha256'] = {str(p.relative_to(base)): sha(p) for p in build.rglob('*') if p.is_file() and (p.name == 'benchmark' or p.suffix in ('.dll', '.class'))}
    (base / 'environment.json').write_text(json.dumps(snapshot, indent=2) + '\n')
    return (commands, env)

def setup_web(base, packages, web_package, web_source, reference_environment, web_revision=None):
    """Prepare only stock Web Node, retaining the original Go oracle's binary identity."""
    locked = json.loads((SDK / 'references.lock.json').read_text())['repositories']['web-sdk']['revision']
    pinned = web_revision or locked
    env = normal_environment(os.environ.copy())
    env.update(json.loads((packages / 'consumers/go/receipt.json').read_text())['environment'])
    env = normal_environment(env)
    node = shutil.which('node', path=env['PATH'])
    if invoke([node, '--version'], SDK, env).decode().strip() != 'v24.15.0':
        raise ValueError('stock Web measurement requires matched Node v24.15.0')
    actual_head = invoke(['git', 'rev-parse', 'HEAD'], web_source, env).decode().strip()
    if actual_head != pinned:
        raise ValueError('stock Web source differs from the declared benchmark revision')
    if invoke(['git', 'status', '--porcelain', '--untracked-files=no'], web_source, env).strip():
        raise ValueError('stock Web source has tracked changes')
    package = json.loads((web_package / 'package.json').read_text())
    tarball = web_source / 'lib' / ('opentdf-sdk-' + package['version'] + '.tgz')
    package_members = {}
    with tarfile.open(tarball) as archive:
        for member in archive.getmembers():
            if member.isfile():
                path = Path(member.name)
                if path.parts[0] != 'package' or '..' in path.parts:
                    raise ValueError('unexpected stock Web package member')
                expected = hashlib.sha256(archive.extractfile(member).read()).hexdigest()
                package_members[str(path.relative_to('package'))] = expected
                if sha(web_package.joinpath(*path.parts[1:])) != expected:
                    raise ValueError('installed stock Web package differs from the source-built artifact')
    if package_members != {str(p.relative_to(web_package)): sha(p) for p in web_package.rglob('*') if p.is_file()}:
        raise ValueError('installed stock Web package member inventory differs from the source-built artifact')
    node_modules = web_package.parent.parent
    lock = node_modules.parent / 'package-lock.json'
    integrity = json.loads(lock.read_text())['packages']['node_modules/@opentdf/sdk']['integrity']
    if integrity != 'sha512-' + base64.b64encode(hashlib.sha512(tarball.read_bytes()).digest()).decode():
        raise ValueError('stock Web installed artifact differs from npm lock integrity')
    original = json.loads(reference_environment.read_text())
    reference = original['commands']['reference']
    if sha(Path(reference[0])) != original['built_consumers']['reference']:
        raise ValueError('frozen original-Go validator binary changed')
    commands = {'reference': reference, 'web': [node, str(SRC / 'web-node.mjs')]}
    snapshot = {'sdk_head': invoke(['git', 'rev-parse', 'HEAD'], SDK, env).decode().strip(),
                'references_lock_sha256': sha(SDK / 'references.lock.json'), 'commands': commands,
                'built_consumers': {'reference': sha(Path(reference[0]))}, 'sources': {str(p.relative_to(SDK)): sha(p) for p in SRC.iterdir() if p.is_file()},
                'consumer_artifacts_sha256': {}, 'controller_sha256': sha(Path(__file__)),
                'reference_environment_sha256': sha(reference_environment),
                'toolchains': {'node': invoke([node, '--version'], SDK, env).decode().strip(), 'go': original['toolchains']['go']},
                'normal_runtime_environment': {'inherited_option_names': sorted(key for key in os.environ if key not in normal_environment(os.environ)), 'known_options_absent': [key for key in RUNTIME_OPTIONS if key not in env]},
                'web': {'package_path': str(web_package), 'source_head': actual_head, 'expected_revision': pinned, 'locked_revision': locked, 'revision_override': web_revision is not None, 'package_version': package['version'], 'tarball_sha256': sha(tarball), 'npm_lock_sha256': sha(lock),
                        'source_npm_lock_sha256': sha(web_source / 'lib/package-lock.json'), 'package_members': package_members,
                        'node_dependency_members': {str(p.relative_to(node_modules)): sha(p) for p in sorted(node_modules.rglob('*')) if p.is_file()},
                        'client_lifecycle': 'one public TDF3Client and ES256 signer per fresh process; stock fresh RSA2048 key generation inside every decrypt'},
                'timing_scope': 'contiguous stock encrypt and full stream consumption, then same-archive decrypt and full owned plaintext consumption; OAuth/discovery untimed'}
    (base / 'environment.json').write_text(json.dumps(snapshot, indent=2) + '\n')
    env['TDF_WEB_PACKAGE'] = str(web_package)
    return commands, env


def setup_swift(base, package, reference_environment=None):
    """Build an independent SwiftPM importer and stock Go validator only."""
    env = normal_environment(os.environ.copy())
    env['GOTOOLCHAIN'] = 'go1.25.14'
    env.pop('GOROOT', None)
    env['GOROOT'] = invoke(['go', 'env', 'GOROOT'], SDK, env).decode().strip()
    build = base / 'build'
    swift = build / 'swift'
    swift.mkdir(parents=True, exist_ok=True)
    members = {str(p.relative_to(package)): sha(p) for p in package.rglob('*')
               if p.is_file() and not any(part in ('.build', '.swiftpm') for part in p.relative_to(package).parts)}
    installed = swift / 'package'
    shutil.copytree(package, installed, ignore=shutil.ignore_patterns('.build', '.swiftpm'))
    shutil.copyfile(SRC / 'Benchmark.swift', swift / 'Benchmark.swift')
    (swift / 'Package.swift').write_text('// swift-tools-version: 6.0\nimport PackageDescription\n'
        'let package = Package(name: "TDF3Benchmark", dependencies: [.package(path: "package")], '
        'targets: [.executableTarget(name: "Benchmark", dependencies: [.product(name: "OpenTDFTDF3", package: "package")], '
        'path: ".", exclude: ["package"], sources: ["Benchmark.swift"])], swiftLanguageModes: [.v5])\n')
    invoke(['swift', 'build', '-c', 'release'], swift, env, 900)
    if reference_environment:
        original = json.loads(reference_environment.read_text())
        reference_command = original['commands']['reference']
        if sha(Path(reference_command[0])) != original['built_consumers']['reference']:
            raise ValueError('frozen original-Go validator binary changed')
    else:
        reference = build / 'reference'
        reference.mkdir()
        shutil.copyfile(SRC / 'reference-go.go', reference / 'main.go')
        module = (SDK / 'tests/interop/generatedreference/go.mod').read_text().replace('../../../../platform/', str(SDK.parent / 'platform') + '/')
        (reference / 'go.mod').write_text(module)
        shutil.copyfile(SDK / 'tests/interop/generatedreference/go.sum', reference / 'go.sum')
        invoke(['go', 'build', '-trimpath', '-o', reference / 'benchmark', '.'], reference, env)
        reference_command = [str(reference / 'benchmark')]
    commands = {'swift': [str(swift / '.build/release/Benchmark')], 'reference': reference_command}
    snapshot = {'sdk_head': invoke(['git', 'rev-parse', 'HEAD'], SDK, env).decode().strip(),
                'platform_head': invoke(['git', 'rev-parse', 'HEAD'], SDK.parent / 'platform', env).decode().strip(),
                'references_lock_sha256': sha(SDK / 'references.lock.json'),
                'compiler_manifest': json.loads((package / 'goalchemy.manifest.json').read_text()),
                'package_members': members, 'commands': commands,
                'sources': {str(p.relative_to(SDK)): sha(p) for p in SRC.iterdir() if p.is_file()},
                'controller_sha256': sha(Path(__file__)),
                'reference_environment_sha256': sha(reference_environment) if reference_environment else None,
                'environment': {key: env[key] for key in ('PATH', 'LD_LIBRARY_PATH', 'PKG_CONFIG_PATH', 'GOTOOLCHAIN', 'GOROOT') if key in env},
                'built_consumers': {name: sha(Path(command[0])) for name, command in commands.items()},
                'system': platform.uname()._asdict(),
                'cpu': next((v.partition(':')[2].strip() for v in Path('/proc/cpuinfo').read_text().splitlines() if v.startswith('model name')), ''),
                'swift_version': invoke(['swift', '--version'], SDK, env).decode().strip(),
                'timing_scope': 'public Data facade encrypt then same-archive decrypt.wait; full owned Data returned before timer stops; equality, I/O, OAuth and discovery untimed'}
    (base / 'environment.json').write_text(json.dumps(snapshot, indent=2) + '\n')
    return commands, env


def configure(base, warmups=20, bulk_warmups=40, batches=3, platform_url='http://localhost:8080', issuer_url='http://localhost:8888/auth/realms/opentdf'):
    form = urllib.parse.urlencode({'grant_type': 'client_credentials', 'client_id': 'opentdf-sdk', 'client_secret': 'secret'}).encode()
    token_url = issuer_url.rstrip('/') + '/protocol/openid-connect/token'
    platform_url = platform_url.rstrip('/')
    with urllib.request.urlopen(urllib.request.Request(token_url, data=form), timeout=15) as response:
        token = json.load(response)
    if int(token['expires_in']) < 3600:
        raise RuntimeError('benchmark requires a pre-acquired token valid for at least one hour')
    expires = int(time.time()) + int(token['expires_in'])
    req = urllib.request.Request(platform_url + '/kas.AccessService/PublicKey', data=json.dumps({'algorithm': 'rsa:2048', 'fmt': 'pkcs8', 'v': '2'}).encode(), headers={'Authorization': 'Bearer ' + token['access_token'], 'Content-Type': 'application/json', 'Connect-Protocol-Version': '1'})
    with urllib.request.urlopen(req, timeout=15) as response:
        key = json.load(response)
    cfg = {'PlatformURL': platform_url, 'KASURL': platform_url + '/kas', 'TokenURL': token_url, 'AllowedKAS': [{'URL': platform_url + '/kas', 'APIBaseURL': platform_url}], 'AllowHTTP': True, 'KASPublicKeyPEM': key['publicKey'], 'KID': key.get('kid', ''), 'KASAlgorithm': 'rsa:2048', 'SessionAlgorithm': 'rsa:2048', 'AuthAlgorithm': 'ES256', 'TokenProviderName': 'access-token'}
    private = base / 'private.json'
    private.write_text(json.dumps({'Config': cfg, 'Token': token['access_token'], 'Expires': expires}))
    private.chmod(384)
    (base / 'kas.pem').write_text(key['publicKey'])
    (base / 'kid').write_text(key.get('kid', ''))
    response = base / 'token-response.json'
    response.write_text(json.dumps({'value': token['access_token'], 'scheme': 'Bearer', 'expiresAt': str(expires), 'confirmationJKT': ''}))
    response.chmod(384)
    safe = {**cfg, 'plaintext': 'byte[i] = (i * 131 + (i >> 8) * 17) & 255', 'segment_bytes': 2 << 20, 'oauth_setup': 'pre-acquired long-lived Bearer; native provider per operation', 'token_lifetime_seconds': int(token['expires_in']), 'warmups': warmups, 'bulk_warmups': bulk_warmups, 'bulk_warmup_size_mib': 50, 'fresh_process_batches': batches}
    (base / 'configuration.json').write_text(json.dumps(safe, indent=2) + '\n')

def record(base, event):
    with (base / 'raw.jsonl').open('a') as stream:
        stream.write(json.dumps(event) + '\n')

def normalized_results(base):
    latest, batches = {}, {}
    for line in (base / 'raw.jsonl').read_text().splitlines():
        row = json.loads(line)
        if row['target'] not in TARGETS or row['operation'] != 'e2e':
            continue
        size_bytes = row.get('size_bytes', row.get('size_mib', 0) * 1024 * 1024)
        if size_bytes not in [v[1] for v in SIZES.values()]:
            continue
        key = (row['target'], row['operation'], size_bytes)
        if row.get('record_type') == 'batch':
            batches.setdefault(key, {})[row['batch_index']] = row
        else:
            latest[key] = {**row, 'size_bytes': size_bytes}
    latest.update({key: aggregate_batches(list(rows.values())) for key, rows in batches.items()})
    return latest


def tables(base):
    latest = normalized_results(base)
    names = {'swift': 'Swift', 'web': 'Original OpenTDF Web (Node)', 'reference': 'Original OpenTDF Go', 'go': 'Generated Go', 'typescript': 'TypeScript (Node)', 'java': 'Java', 'csharp': 'C#', 'python': 'Python', 'rust': 'Rust', 'c': 'C'}
    labels = [label.replace('KiB', ' KiB').replace('MiB', ' MiB') for label in SIZES]
    parts = ['| SDK | ' + ' | '.join(labels) + ' |', '| --- | ' + ' | '.join('---:' for _ in labels) + ' |']
    for target in TARGETS:
        cells = []
        for _, size_bytes in SIZES.values():
            row = latest.get((target, 'e2e', size_bytes))
            if not row:
                cell = 'Not measured'
            elif row.get('status') == 'pending':
                cell = 'Pending batches'
            elif row.get('status') != 'ok':
                cell = 'Failed (' + row.get('failure', 'unknown') + ')'
            else:
                cell = f"{row['median_ms']:.2f} ms"
            cells.append(cell)
        parts.append('| ' + names[target] + ' | ' + ' | '.join(cells) + ' |')
    (base / 'tables.md').write_text('\n'.join(parts) + '\n')
    (base / 'summary.json').write_text(json.dumps(list(latest.values()), indent=2) + '\n')

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--packages', type=Path, default=SDK / '.local/steady-state-2026-10-04/delivery')
    parser.add_argument('--sizes', default='1MiB,10MiB,50MiB')
    parser.add_argument('--targets', default=','.join(NATIVE_TARGETS))
    parser.add_argument('--web-package', type=Path, help='installed pinned @opentdf/sdk package directory for the optional Web Node row')
    parser.add_argument('--web-source', type=Path, default=SDK.parent / 'web-sdk', help='pinned stock Web repository used to build the installed package')
    parser.add_argument('--web-revision', help='explicit full benchmark-only Web source revision; defaults to references.lock.json without changing the oracle pin')
    parser.add_argument('--reference-environment', type=Path, help='frozen original-Go environment.json; Web-only preparation reuses its validated binary without native rebuilds')
    parser.add_argument('--swift-package', type=Path, help='built SwiftPM OpenTDFTDF3 source package for a Swift-only campaign')
    parser.add_argument('--platform-url', default='http://localhost:8080')
    parser.add_argument('--issuer-url', default='http://localhost:8888/auth/realms/opentdf')
    parser.add_argument('--samples', type=int, default=5)
    parser.add_argument('--batches', type=int, default=3)
    parser.add_argument('--warmups', type=int, default=20)
    parser.add_argument('--bulk-warmups', type=int, default=40)
    parser.add_argument('--timeout', type=int, default=1800)
    parser.add_argument('--build-only', action='store_true')
    parser.add_argument('--skip-build', action='store_true')
    parser.add_argument('--fresh', action='store_true')
    parser.add_argument('--rerun', action='append', default=[], metavar='TARGET:OPERATION:SIZE')
    parser.add_argument('--correction-note', default='')
    args = parser.parse_args()
    if args.web_revision and (args.targets != 'web' or len(args.web_revision) != 40 or any(c not in '0123456789abcdef' for c in args.web_revision)):
        parser.error('--web-revision requires a Web-only campaign and a full lowercase Git revision')
    if any(label not in SIZES for label in args.sizes.split(',')):
        parser.error('--sizes must select 1MiB,10MiB,50MiB')
    if any(target not in TARGETS for target in args.targets.split(',')) or min(args.samples, args.batches, args.warmups) < 1 or args.bulk_warmups < 0:
        parser.error('select supported targets and positive samples/batches/warmups')
    reruns = set()
    for selector in args.rerun:
        fields = selector.split(':')
        if len(fields) != 3 or fields[0] not in TARGETS or fields[1] != 'e2e' or fields[2] not in SIZES:
            parser.error('--rerun must name TARGET:e2e:SIZE')
        reruns.add(tuple(fields))
    os.umask(0o077)
    base = args.output.resolve()
    packages = args.packages.resolve()
    base.mkdir(parents=True, exist_ok=True)
    if args.fresh and (base / 'raw.jsonl').exists():
        parser.error('--fresh requires a campaign with no previous raw measurements')
    if args.skip_build:
        snapshot = json.loads((base / 'environment.json').read_text())
        commands = snapshot['commands']
        env = os.environ.copy()
        env.update(snapshot['environment'] if args.targets == 'swift' else json.loads((packages / 'consumers/go/receipt.json').read_text())['environment'])
        env = normal_environment(env)
        env['GOTOOLCHAIN'] = 'go1.25.14'
        env.pop('GOROOT', None)
        env['GOROOT'] = invoke(['go', 'env', 'GOROOT'], SDK, env).decode().strip()
        env['TDF_TS_NODE_PACKAGE'] = str((packages / 'consumers/typescript/package/dist/node-index.js').resolve())
        if snapshot['controller_sha256'] != sha(Path(__file__)) or snapshot['sources'] != {str(p.relative_to(SDK)): sha(p) for p in SRC.iterdir() if p.is_file()}:
            raise RuntimeError('benchmark source changed since the frozen native build')
        if args.targets == 'swift':
            if any(sha(Path(commands[name][0])) != digest for name, digest in snapshot['built_consumers'].items()):
                raise RuntimeError('frozen Swift campaign binary changed')
            installed = base / 'build/swift/package'
            actual = {str(p.relative_to(installed)): sha(p) for p in installed.rglob('*')
                      if p.is_file() and not any(part in ('.build', '.swiftpm') for part in p.relative_to(installed).parts)}
            if actual != snapshot['package_members']:
                raise RuntimeError('frozen Swift campaign package changed')
        elif args.targets == 'web':
            web = snapshot['web']
            expected = args.web_revision or json.loads((SDK / 'references.lock.json').read_text())['repositories']['web-sdk']['revision']
            if expected != web.get('expected_revision', web['source_head']) or invoke(['git', 'rev-parse', 'HEAD'], args.web_source.resolve(), env).decode().strip() != expected:
                raise RuntimeError('frozen Web campaign revision changed')
            node_modules = Path(web['package_path']).parent.parent
            actual = {str(p.relative_to(node_modules)): sha(p) for p in sorted(node_modules.rglob('*')) if p.is_file()}
            if actual != web['node_dependency_members'] or sha(node_modules.parent / 'package-lock.json') != web['npm_lock_sha256']:
                raise RuntimeError('frozen Web campaign installed dependencies changed')
            if sha(Path(commands['reference'][0])) != snapshot['built_consumers']['reference']:
                raise RuntimeError('frozen original-Go validator binary changed')
    elif args.targets == 'swift':
        if not args.swift_package:
            parser.error('Swift-only preparation requires --swift-package')
        commands, env = setup_swift(base, args.swift_package.resolve(), args.reference_environment.resolve() if args.reference_environment else None)
    elif args.targets == 'web':
        if not args.web_package or not args.reference_environment:
            parser.error('Web-only preparation requires --web-package and --reference-environment')
        commands, env = setup_web(base, packages, args.web_package.resolve(), args.web_source.resolve(), args.reference_environment.resolve(), args.web_revision)
    else:
        if any(target in args.targets.split(',') for target in ('web', 'swift')):
            parser.error('measure optional Web and Swift rows separately with --targets web or --targets swift')
        commands, env = setup(base, packages)
    if 'web' in commands:
        env['TDF_WEB_PACKAGE'] = json.loads((base / 'environment.json').read_text())['web']['package_path']
    if 'java' in commands and (len(commands['java']) != 4 or commands['java'][1] != '-cp'):
        raise RuntimeError('normal benchmark JVM command must contain only the classpath and main class')
    policy = {'bulk_warmups': args.bulk_warmups, 'bulk_size_bytes': 50 * 1024 * 1024,
              'actual_size_warmups': args.warmups, 'fresh_process_batches': args.batches, 'samples_per_batch': args.samples,
              'convergence_investigation': 'actual last10 and measured median both more than20% below actual first10; preserve all samples and investigate, never select fastest'}
    (base / 'policy.json').write_text(json.dumps(policy, indent=2) + '\n')
    if args.build_only:
        print('BUILD_OK', flush=True)
        return
    accepted = normalized_results(base) if (base / 'raw.jsonl').exists() else {}
    labels = args.sizes.split(',')
    needed = set(labels + (['50MiB'] if args.bulk_warmups else []))
    for label in needed:
        size, size_bytes = SIZES[label]
        fixture = base / (size + '.input')
        if not fixture.exists():
            chunk = bytes((i * 131 + (i >> 8) * 17 & 255 for i in range(1 << 20)))
            fixture.write_bytes((chunk * ((size_bytes + len(chunk) - 1) // len(chunk)))[:size_bytes])
    for label in labels:
        size, size_bytes = SIZES[label]
        fixture = base / (size + '.input')
        for target in args.targets.split(','):
            previous = accepted.get((target, 'e2e', size_bytes))
            selected_rerun = (target, 'e2e', label) in reruns
            if previous and previous.get('status') == 'ok' and not selected_rerun:
                print('REUSE', target, 'e2e', label, flush=True)
                continue
            attempt = '-rerun-' + str(time.time_ns()) if selected_rerun else ''
            for batch in range(args.batches):
                batch_id = target + '-' + size + '-' + str(batch) + attempt
                directory = base / 'batches' / batch_id
                directory.mkdir(parents=True, exist_ok=False)
                for input_size in set([size] + (['50'] if args.bulk_warmups else [])):
                    os.link(base / (input_size + '.input'), directory / (input_size + '.input'))
                configure(directory, args.warmups, args.bulk_warmups, args.batches, args.platform_url, args.issuer_url)
                command = commands[target] + [str(directory), 'e2e', size, str(args.samples), str(args.warmups), str(args.bulk_warmups)]
                started = time.time()
                print('START', target, label, 'batch', batch + 1, flush=True)
                event = {'record_type': 'batch', 'batch_id': batch_id, 'batch_index': batch, 'batches_requested': args.batches,
                         'target': target, 'operation': 'e2e', 'fixture_id': size, 'size_bytes': size_bytes, 'size_label': label,
                         'samples_requested': args.samples, 'warmup_count': args.warmups, 'bulk_warmup_count': args.bulk_warmups,
                         'payload_sha256': sha(fixture), 'bulk_payload_sha256': sha(base / '50.input') if args.bulk_warmups else None,
                         'command': command, 'configuration_sha256': sha(directory / 'configuration.json'),
                         'harness_sources': {str(p.relative_to(SDK)): sha(p) for p in SRC.iterdir() if p.is_file()},
                         'timing_scope': 'contiguous public encryption then same-archive public decryption; full owned plaintext materialized before timer stops; equality/archive persistence/validation untimed'}
                if selected_rerun:
                    event['rerun_attribution'] = {'reason': args.correction_note, 'supersedes_aggregate_sha256': hashlib.sha256(json.dumps(previous, sort_keys=True).encode()).hexdigest(), 'retired_measurement': True}
                try:
                    output = invoke(command, SDK, env, args.timeout)
                    (directory / 'native.stdout').write_bytes(output)
                    result = json.loads(output)
                    validate_native_result(result, args.samples, args.warmups, args.bulk_warmups)
                    validation = []
                    for i in range(-1, args.samples):
                        archive = directory / (target + '-' + size + '-' + str(i) + '.tdf')
                        with zipfile.ZipFile(archive) as zipped:
                            if set(zipped.namelist()) != {'0.payload', '0.manifest.json'}:
                                raise ValueError('unexpected archive members')
                            for member in zipped.infolist():
                                with zipped.open(member) as stream:
                                    while stream.read(1 << 20):
                                        pass
                            manifest_profile = validate_manifest(json.loads(zipped.read('0.manifest.json')), size_bytes)
                        configure(directory, args.warmups, args.bulk_warmups, args.batches, args.platform_url, args.issuer_url)
                        check = json.loads(invoke(commands['reference'] + [str(directory), 'validate', str(archive), '0', size], SDK, env, 180))
                        if check.get('correct') is not True:
                            raise ValueError('independent stock Go plaintext mismatch')
                        validation.append({'batch_id': batch_id, 'sample': i, 'archive_path': str(archive.relative_to(base)),
                                           'archive_sha256': sha(archive), 'archive_bytes': archive.stat().st_size,
                                           'stock_go_real_kas': True, 'independent_zip_crc': True,
                                           'manifest_profile': manifest_profile})
                    event.update(result, status='ok', warmup_statistics=warmup_statistics(result),
                                 encrypted_samples_validated=validation, wall_seconds=time.time() - started,
                                 reference_client_initializations=result.get('client_initializations'))
                except Exception as error:
                    event.update(status='failed', failure=type(error).__name__, diagnostic_sha256=hashlib.sha256(str(error).encode()).hexdigest(), wall_seconds=time.time() - started)
                    (directory / 'failure.txt').write_text(str(error))
                record(base, event)
                tables(base)
                print('DONE', target, label, 'batch', batch + 1, event['status'], flush=True)
                if event['status'] != 'ok':
                    raise SystemExit('native benchmark failed; retained batch evidence, no automatic retry')
    latest = normalized_results(base)
    if any(latest.get((target, 'e2e', SIZES[label][1]), {}).get('status') != 'ok' for label in labels for target in args.targets.split(',')):
        raise SystemExit('one or more selected aggregate cells are incomplete')
    print('CAMPAIGN_OK', len(labels) * len(args.targets.split(',')), 'cells', flush=True)

if __name__ == '__main__':
    main()
