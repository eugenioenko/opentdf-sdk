#!/usr/bin/env python3
"""Profile both Go SDKs at 1 MiB using separate bounded diagnostic loops."""
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import statistics
import subprocess

SDK = Path(__file__).resolve().parents[1]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def execute(command, cwd, env, output=None, timeout=180):
    result = subprocess.run([str(value) for value in command], cwd=cwd, env=env,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout)
    if result.returncode == 2 and '-list' in command and b'no matches found for regexp' in result.stderr:
        if output:
            output.write_bytes(result.stderr)
        return result.stderr
    if result.returncode:
        raise RuntimeError(f"command failed: {command[0]}, status {result.returncode}")
    if output:
        output.write_bytes(result.stdout)
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--campaign', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--seconds', type=float, default=5)
    parser.add_argument('--resume', action='store_true', help='Reuse captured profiles and the existing consumer binary.')
    args = parser.parse_args()
    campaign, output = args.campaign.resolve(), args.output.resolve()
    output.mkdir(parents=True, exist_ok=args.resume)
    source = SDK / 'tests/bench/go-profile.go'
    build = output / 'build'
    build.mkdir(exist_ok=args.resume)
    shutil.copyfile(source, build / 'main.go')
    module = (campaign / 'build/reference/go.mod').read_text()
    package = (SDK / '.local/phase7/final-packages-v2/consumers/go/package').resolve()
    (build / 'go.mod').write_text(module + '\nrequire goalchemyout v0.0.0\nreplace goalchemyout => ' + str(package) + '\n')
    shutil.copyfile(campaign / 'build/reference/go.sum', build / 'go.sum')
    env = os.environ.copy()
    env['GOTOOLCHAIN'] = 'go1.25.1'
    benchmark = SDK / 'scripts/benchmark-sdk.py'
    spec = importlib.util.spec_from_file_location('benchmark', benchmark)
    controller = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(controller)
    receipt = json.loads((SDK / '.local/phase7/final-packages-v2/consumers/go/receipt.json').read_text())
    env.update(receipt['environment'])
    env['GOTOOLCHAIN'] = 'go1.25.1'
    build_command = ['go', 'build', '-trimpath', '-o', build / 'profiler', '.']
    if not args.resume:
        execute(build_command, build, env)
    build_info = execute(['go', 'version', '-m', build / 'profiler'], SDK, env).decode()
    toolchain_source = Path(execute(['go', 'env', 'GOROOT'], SDK, env).decode().strip()) / 'src'
    identity = {
        'source_sha256': digest(source), 'controller_sha256': digest(Path(__file__)),
        'module_sha256': digest(build / 'go.mod'), 'module_lock_sha256': digest(build / 'go.sum'),
        'binary_sha256': digest(build / 'profiler'), 'build_command': [str(v) for v in build_command],
        'build_info': build_info,
        'payload_sha256': digest(campaign / '1.input'),
        'reference_archive_sha256': digest(campaign / '1.reference.tdf'),
        'sdk_head': execute(['git', 'rev-parse', 'HEAD'], SDK, env).decode().strip(),
        'platform_head': execute(['git', 'rev-parse', 'HEAD'], SDK.parent / 'platform', env).decode().strip(),
        'generated_package_receipt_sha256': digest(SDK / '.local/phase7/final-packages-v2/packages/go/receipt.json'),
    }
    summaries = []
    for target in ('reference', 'go'):
        for operation in ('encrypt', 'decrypt'):
            directory = output / f'{target}-{operation}'
            directory.mkdir(exist_ok=args.resume)
            controller.configure(campaign)
            command = [build / 'profiler', campaign, target, operation, args.seconds, directory]
            if args.resume and (directory / 'instrumented.json').exists():
                print('PROFILE_REUSE', target, operation, flush=True)
            else:
                print('PROFILE_START', target, operation, flush=True)
                execute(command, SDK, env, directory / 'stdout.json')
            result = json.loads((directory / 'instrumented.json').read_text())
            assert result['correct'] and result['operations'] > 0
            profile = directory / 'cpu.pprof'
            reports = {
                'top-flat.txt': ['-top', '-nodecount=25'],
                'top-cumulative.txt': ['-top', '-cum', '-nodecount=25'],
                'calltree.txt': ['-tree', '-nodecount=25'],
                'labels.txt': ['-tags'],
                'rsa-source.txt': ['-list', 'crypto/rsa.GenerateKey|NewRSAKeyPair|NewKeyPair'],
            }
            for name, options in reports.items():
                execute(['go', 'tool', 'pprof', '-source_path', toolchain_source, *options, build / 'profiler', profile], SDK, env, directory / name)
            validation = []
            if operation == 'encrypt':
                for label in ('warmup', 'first', 'last'):
                    archive = directory / f'{label}.tdf'
                    controller.configure(campaign)
                    native_reference = campaign / 'build/reference/benchmark'
                    checked = json.loads(execute([native_reference, campaign, 'validate', archive, '0', '1'], SDK, env))
                    assert checked['correct']
                    validation.append({'archive': archive.name, 'sha256': digest(archive), 'bytes': archive.stat().st_size, 'stock_go_real_kas': True})
            phases = result['phase_timings']
            stats = {}
            for key in phases[0]:
                values = [phase[key] for phase in phases]
                stats[key] = {'mean_ms': statistics.mean(values), 'median_ms': statistics.median(values), 'total_ms': sum(values)}
            summary = {
                'target': target, 'operation': operation, 'operations': result['operations'],
                'profile_seconds_requested': args.seconds, 'wall_loop_ms': result['wall_loop_ms'],
                'kas_calls_expected': result['kas_calls_expected'], 'phase_statistics': stats,
                'scope': result['timing_scope'], 'generated_setup_scope': result['generated_setup_scope'],
                'command': [str(v) for v in command], 'profile_sha256': digest(profile),
                'correctness': {'every_decryption_exact': operation == 'decrypt', 'encryption_archives_validated': validation},
                'report_sha256': {name: digest(directory / name) for name in reports},
            }
            summaries.append(summary)
            print('PROFILE_DONE', target, operation, result['operations'], 'operations', flush=True)
    safe = {'schema_version': 1, 'identity': identity, 'profiles': summaries,
            'limitations': ['Instrumented profiler timings are separate from published benchmark medians.',
                            'CPU samples exclude blocked/waiting wall time; local real KAS is part of operation wall time.',
                            'Generated internal client setup is inside its public API phase; host configuration is not equivalent to SDK.New.',
                            'All decryption outputs exact-checked; warmup/first/last encryption archives independently stock-Go/real-KAS validated.',
                            'CPU profile includes labels, allocation/runtime work, and exact plaintext checks labelled separately.']}
    (output / 'safe-summary.json').write_text(json.dumps(safe, indent=2) + '\n')
    print('PROFILE_EXPORT', output / 'safe-summary.json', flush=True)


if __name__ == '__main__':
    main()
