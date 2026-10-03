#!/usr/bin/env python3
"""Build native benchmark consumers, measure public APIs, validate through real KAS.

Uses existing accepted installed packages; never invokes the Goalchemy compiler.
Raw outputs and private token inputs are placed under ignored .local/benchmarks.
"""
import argparse, hashlib, json, os, platform, shutil, statistics, subprocess, time, urllib.request, urllib.parse
from pathlib import Path
SDK = Path(__file__).resolve().parents[1]
SRC = SDK / 'tests/bench'
TARGETS = ['reference', 'go', 'typescript', 'java', 'csharp', 'python', 'rust', 'c']
SIZES = {'10KiB': ('10KiB', 10 * 1024), '100KiB': ('100KiB', 100 * 1024), '1MiB': ('1', 1024 * 1024)}

def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()

def invoke(command, cwd, env, timeout=300):
    result = subprocess.run([str(v) for v in command], cwd=cwd, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout)
    if result.returncode:
        raise RuntimeError('command failed ' + str(command[0]) + ': ' + result.stderr.decode(errors='replace')[:1800])
    return result.stdout

def setup(base, packages):
    build = base / 'build'
    build.mkdir(parents=True, exist_ok=True)
    consumers = packages / 'consumers'
    receipts = {t: json.loads((consumers / t / 'receipt.json').read_text()) for t in TARGETS if t != 'reference'}
    env = os.environ.copy()
    env.update(receipts['go']['environment'])
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
    (csharp / 'benchmark.csproj').write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><Nullable>disable</Nullable></PropertyGroup><ItemGroup><Reference Include="OpenTDF.TDF3"><HintPath>' + str(dll) + '</HintPath></Reference></ItemGroup></Project>')
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
    commands['typescript'] = ['node', str(SRC / 'node.mjs')]
    env['TDF_TS_PACKAGE'] = str((consumers / 'typescript/package/dist/index.js').resolve())
    snapshot = {'sdk_head': invoke(['git', 'rev-parse', 'HEAD'], SDK, env).decode().strip(), 'platform_head': invoke(['git', 'rev-parse', 'HEAD'], SDK.parent / 'platform', env).decode().strip(), 'references_lock_sha256': sha(SDK / 'references.lock.json'), 'packages': identities, 'sources': {str(p.relative_to(SDK)): sha(p) for p in SRC.iterdir() if p.is_file()}, 'commands': commands, 'built_consumers': {t: sha(Path(cmd[0])) for (t, cmd) in commands.items() if t in ('reference', 'go', 'c', 'rust')}, 'system': platform.uname()._asdict(), 'cpu': next((v.partition(':')[2].strip() for v in Path('/proc/cpuinfo').read_text().splitlines() if v.startswith('model name')), ''), 'memory_kib': Path('/proc/meminfo').read_text().splitlines()[0], 'toolchains': {}}
    for (name, args) in [('go', ['go', 'version']), ('node', ['node', '--version']), ('python', [commands['python'][0], '--version']), ('java', [java_command[0], '-version']), ('dotnet', [dotnet, '--version']), ('rust', ['rustc', '--version']), ('cc', ['cc', '--version'])]:
        v = subprocess.run(args, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        snapshot['toolchains'][name] = (v.stdout + v.stderr).decode().strip()
    (base / 'environment.json').write_text(json.dumps(snapshot, indent=2) + '\n')
    return (commands, env)

def configure(base):
    form = urllib.parse.urlencode({'grant_type': 'client_credentials', 'client_id': 'opentdf-sdk', 'client_secret': 'secret'}).encode()
    with urllib.request.urlopen(urllib.request.Request('http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token', data=form), timeout=15) as response:
        token = json.load(response)
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
    safe = {**cfg, 'plaintext': 'byte[i] = (i * 131 + (i >> 8) * 17) & 255', 'segment_bytes': 2 << 20, 'oauth_setup': 'pre-acquired Bearer; native provider per operation', 'warmups': 1}
    (base / 'configuration.json').write_text(json.dumps(safe, indent=2) + '\n')

def record(base, event):
    with (base / 'raw.jsonl').open('a') as stream:
        stream.write(json.dumps(event) + '\n')

def normalized_results(base):
    rows = [json.loads(v) for v in (base / 'raw.jsonl').read_text().splitlines()]
    latest = {}
    for row in rows:
        if row['target'] not in TARGETS:
            continue
        size_bytes = row.get('size_bytes', row.get('size_mib', 0) * 1024 * 1024)
        if size_bytes not in [v[1] for v in SIZES.values()]:
            continue
        value = dict(row)
        value['size_bytes'] = size_bytes
        value['size_label'] = next(label for label, (_, n) in SIZES.items() if n == size_bytes)
        # Original receipts are retained verbatim in raw.jsonl; derived fields
        # normalize units without pretending an old invocation was newly run.
        value['derived_from_raw_sha256'] = hashlib.sha256(json.dumps(row, sort_keys=True).encode()).hexdigest()
        latest[(row['target'], row['operation'], size_bytes)] = value
    return latest


def tables(base):
    latest = normalized_results(base)
    names = {'reference': 'Original OpenTDF Go', 'go': 'Generated Go', 'typescript': 'TypeScript (Node)', 'java': 'Java', 'csharp': 'C#', 'python': 'Python', 'rust': 'Rust', 'c': 'C'}
    parts = []
    for op in ['encrypt', 'decrypt']:
        parts += ['Encryption' if op == 'encrypt' else 'Decryption', '', '| SDK | 10 KiB | 100 KiB | 1 MiB |', '| --- | ---: | ---: | ---: |']
        for target in TARGETS:
            cells = []
            for _, size_bytes in SIZES.values():
                row = latest.get((target, op, size_bytes))
                ref = latest.get(('reference', op, size_bytes))
                if not row:
                    cell = 'Not measured'
                elif row.get('status') != 'ok':
                    cell = 'Failed (' + row.get('failure', 'unknown') + ')'
                elif not ref or ref.get('status') != 'ok':
                    cell = f"{row['median_ms']:.2f} ms"
                else:
                    cell = f"{row['median_ms']:.2f} ms ({row['median_ms'] / ref['median_ms']:.2f}×)"
                cells.append(cell)
            parts.append('| ' + names[target] + ' | ' + ' | '.join(cells) + ' |')
        parts.append('')
    (base / 'tables.md').write_text('\n'.join(parts))
    (base / 'summary.json').write_text(json.dumps(list(latest.values()), indent=2) + '\n')

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--packages', type=Path, default=SDK / '.local/phase7/final-packages-v2')
    parser.add_argument('--sizes', default='10KiB,100KiB,1MiB')
    parser.add_argument('--targets', default=','.join(TARGETS))
    parser.add_argument('--samples', type=int, default=5)
    parser.add_argument('--timeout', type=int, default=1800)
    parser.add_argument('--build-only', action='store_true')
    parser.add_argument('--skip-build', action='store_true')
    args = parser.parse_args()
    base = args.output.resolve()
    base.mkdir(parents=True, exist_ok=True)
    if args.skip_build:
        snapshot = json.loads((base / 'environment.json').read_text())
        commands = snapshot['commands']
        env = os.environ.copy()
        env.update(json.loads((args.packages / 'consumers/go/receipt.json').read_text())['environment'])
        env['TDF_TS_PACKAGE'] = str((args.packages / 'consumers/typescript/package/dist/index.js').resolve())

    else:
        (commands, env) = setup(base, args.packages.resolve())
    if args.build_only:
        print('BUILD_OK', flush=True)
        return
    accepted = normalized_results(base) if (base / 'raw.jsonl').exists() else {}
    for label in args.sizes.split(','):
        size, size_bytes = SIZES[label]
        fixture = base / (str(size) + '.input')
        if not fixture.exists():
            chunk = bytes((i * 131 + (i >> 8) * 17 & 255 for i in range(1 << 20)))
            fixture.write_bytes((chunk * ((size_bytes + len(chunk) - 1) // len(chunk)))[:size_bytes])
        for target in args.targets.split(','):
            for op in ['encrypt', 'decrypt']:
                previous = accepted.get((target, op, size_bytes))
                if previous and previous.get('status') == 'ok':
                    print('REUSE', target, op, label, flush=True)
                    continue
                if op == 'decrypt' and (not (base / (str(size) + '.reference.tdf')).exists()):
                    raise RuntimeError('run original reference encryption first')
                configure(base)
                command = commands[target] + [str(base), op, str(size), str(args.samples)]
                started = time.time()
                print('START', target, op, size, flush=True)
                event = {'target': target, 'operation': op, 'fixture_id': size, 'size_bytes': size_bytes, 'size_label': label, 'samples_requested': args.samples, 'warmup_count': 1, 'payload_sha256': sha(fixture), 'command': command, 'harness_sources': {str(p.relative_to(SDK)): sha(p) for p in SRC.iterdir() if p.is_file()}}
                try:
                    output = invoke(command, SDK, env, args.timeout)
                    result = json.loads(output)
                    samples = result['samples_ms']
                    assert result['correct'] and len(samples) == args.samples and all((v > 0 for v in samples))
                    validation = []
                    if op == 'encrypt':
                        prefix = 'reference' if target == 'reference' else target
                        for i in range(-1, args.samples):
                            archive = base / f'{prefix}-{size}-{i}.tdf'
                            configure(base)
                            check = json.loads(invoke(commands['reference'] + [str(base), 'validate', str(archive), '0', str(size)], SDK, env, 180))
                            assert check['correct']
                            validation.append({'sample': i, 'archive_sha256': sha(archive), 'archive_bytes': archive.stat().st_size, 'stock_go_real_kas': True})
                    event.update(result, status='ok', median_ms=statistics.median(samples), minimum_ms=min(samples), maximum_ms=max(samples), stdev_ms=statistics.stdev(samples) if len(samples) > 1 else 0, encrypted_samples_validated=validation, wall_seconds=time.time() - started)
                except Exception as e:
                    event.update(status='failed', failure=type(e).__name__, diagnostic_sha256=hashlib.sha256(str(e).encode()).hexdigest(), wall_seconds=time.time() - started)
                    print('FAIL', target, op, size, type(e).__name__, flush=True)
                    (base / f'{target}-{op}-{size}.failure.txt').write_text(str(e))
                    (base / f'{target}-{op}-{size}.failure.txt').chmod(384)
                record(base, event)
                tables(base)
                print('DONE', target, op, size, event['status'], round(event.get('median_ms', 0), 2), 'ms', flush=True)
if __name__ == '__main__':
    main()
