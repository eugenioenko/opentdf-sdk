#!/usr/bin/env python3
"""Build native consumers and measure contiguous public encrypt/decrypt pairs through real KAS.

Uses existing accepted installed packages; never invokes the Goalchemy compiler.
Raw outputs and private token inputs are placed under ignored .local/benchmarks.
"""
import argparse, hashlib, json, os, platform, shutil, statistics, subprocess, time, urllib.request, urllib.parse, zipfile
from pathlib import Path
SDK = Path(__file__).resolve().parents[1]
SRC = SDK / 'tests/bench'
RUNTIME_OPTIONS = ('JAVA_TOOL_OPTIONS', 'JDK_JAVA_OPTIONS', '_JAVA_OPTIONS', 'NODE_OPTIONS', 'GOGC', 'GOMEMLIMIT', 'GOMAXPROCS', 'PYTHONMALLOC', 'PYTHONOPTIMIZE', 'MALLOC_ARENA_MAX')
TARGETS = ['reference', 'go', 'typescript', 'java', 'csharp', 'python', 'rust', 'c']
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
    receipts = {t: json.loads((consumers / t / 'receipt.json').read_text()) for t in TARGETS if t != 'reference'}
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

def configure(base, warmups=20, bulk_warmups=40, batches=3):
    form = urllib.parse.urlencode({'grant_type': 'client_credentials', 'client_id': 'opentdf-sdk', 'client_secret': 'secret'}).encode()
    with urllib.request.urlopen(urllib.request.Request('http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token', data=form), timeout=15) as response:
        token = json.load(response)
    if int(token['expires_in']) < 3600:
        raise RuntimeError('benchmark requires a pre-acquired token valid for at least one hour')
    expires = int(time.time()) + int(token['expires_in'])
    req = urllib.request.Request('http://localhost:8080/kas.AccessService/PublicKey', data=json.dumps({'algorithm': 'rsa:2048', 'fmt': 'pkcs8', 'v': '2'}).encode(), headers={'Authorization': 'Bearer ' + token['access_token'], 'Content-Type': 'application/json', 'Connect-Protocol-Version': '1'})
    with urllib.request.urlopen(req, timeout=15) as response:
        key = json.load(response)
    cfg = {'PlatformURL': 'http://localhost:8080', 'KASURL': 'http://localhost:8080/kas', 'AllowedKAS': [{'URL': 'http://localhost:8080/kas', 'APIBaseURL': 'http://localhost:8080'}], 'AllowHTTP': True, 'KASPublicKeyPEM': key['publicKey'], 'KID': key.get('kid', ''), 'KASAlgorithm': 'rsa:2048', 'SessionAlgorithm': 'rsa:2048', 'AuthAlgorithm': 'ES256', 'TokenProviderName': 'access-token'}
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
    names = {'reference': 'Original OpenTDF Go', 'go': 'Generated Go', 'typescript': 'TypeScript (Node)', 'java': 'Java', 'csharp': 'C#', 'python': 'Python', 'rust': 'Rust', 'c': 'C'}
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
    parser.add_argument('--targets', default=','.join(TARGETS))
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
        env.update(json.loads((packages / 'consumers/go/receipt.json').read_text())['environment'])
        env = normal_environment(env)
        env['GOTOOLCHAIN'] = 'go1.25.14'
        env.pop('GOROOT', None)
        env['GOROOT'] = invoke(['go', 'env', 'GOROOT'], SDK, env).decode().strip()
        env['TDF_TS_NODE_PACKAGE'] = str((packages / 'consumers/typescript/package/dist/node-index.js').resolve())
        if snapshot['controller_sha256'] != sha(Path(__file__)) or snapshot['sources'] != {str(p.relative_to(SDK)): sha(p) for p in SRC.iterdir() if p.is_file()}:
            raise RuntimeError('benchmark source changed since the frozen native build')
    else:
        commands, env = setup(base, packages)
    if len(commands['java']) != 4 or commands['java'][1] != '-cp':
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
                configure(directory, args.warmups, args.bulk_warmups, args.batches)
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
                        configure(directory, args.warmups, args.bulk_warmups, args.batches)
                        check = json.loads(invoke(commands['reference'] + [str(directory), 'validate', str(archive), '0', size], SDK, env, 180))
                        if check.get('correct') is not True:
                            raise ValueError('independent stock Go plaintext mismatch')
                        validation.append({'batch_id': batch_id, 'sample': i, 'archive_path': str(archive.relative_to(base)),
                                           'archive_sha256': sha(archive), 'archive_bytes': archive.stat().st_size,
                                           'stock_go_real_kas': True, 'independent_zip_crc': True})
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
