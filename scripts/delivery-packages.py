#!/usr/bin/env python3
"""Build reproducible deliverables with one frozen compiler; no services."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import shutil
import subprocess
import tarfile
import time

SDK = Path(__file__).resolve().parents[1]
TARGETS = ('go', 'typescript', 'java', 'csharp', 'python', 'rust', 'c')


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def members(root, target):
    root = Path(root)
    if target == 'go':
        files = [p for p in root.rglob('*') if p.is_file() and (p.suffix == '.go' or p.name == 'go.mod' or 'licenses' in p.parts)]
    elif target == 'typescript':
        files = [p for p in root.rglob('*') if p.is_file() and ('dist' in p.parts or 'licenses' in p.parts or p.name == 'package.json')]
    elif target == 'java':
        files = [root/'tdf3-java.jar', root/'dependencies.lock.json', *list((root/'lib').glob('*.jar')), *list((root/'licenses').rglob('*'))]
    elif target == 'csharp':
        files = [p for p in (root/'lib').iterdir() if p.suffix in ('.dll', '.json')] + [root/'dependencies.lock.json', root/'packages.lock.json', *list((root/'licenses').rglob('*'))]
    elif target == 'python':
        files = [*list((root/'dist').glob('*.whl')), root/'dependencies.lock.json']
    elif target == 'rust':
        files = [root/'target/package/opentdf-tdf3-0.1.0.crate']
    else:
        files = [root/'opentdf-tdf3-c-0.1.0-linux-x86_64.tar.gz']
    return {str(p.relative_to(root)): sha(p) for p in sorted(files) if p.is_file()}


def production_sources(root, target):
    root = Path(root)
    exclusions = {'classes', 'lib', 'obj', 'target', 'dist', 'licenses', 'package', 'build', '__pycache__', 'opentdf_tdf3.egg-info'}
    extensions = {'.go', '.ts', '.java', '.cs', '.py', '.rs', '.c', '.h'}
    return {str(p.relative_to(root)): sha(p) for p in sorted(root.rglob('*'))
            if p.is_file() and p.suffix in extensions and not exclusions.intersection(p.relative_to(root).parts)
            and p.name != 'build.sh'}


def command(args, cwd, env, log, timeout=1800):
    with Path(log).open('wb') as output:
        result = subprocess.run([str(a) for a in args], cwd=cwd, env=env, stdout=output, stderr=subprocess.STDOUT, timeout=timeout)
    Path(str(log)+'.status').write_text(str(result.returncode)+'\n')
    if result.returncode:
        raise RuntimeError('command failed; see '+str(log))


def build(target, base, compiler, compiler_revision=None):
    destination = base/'packages'/target
    destination.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    env.update(GOTOOLCHAIN='go1.25.14', JAVA_HOME=str(SDK.parent/'goalchemy/.toolchains/jdk-21.0.12.1+1'),
               CARGO_TARGET_DIR=str(base/'rust-package-build-cache'))
    outputs = []
    for kind in ('relative', 'absolute'):
        caller = destination/(kind+'-caller')
        caller.mkdir(exist_ok=True)
        output = destination/kind
        if output.exists():
            raise RuntimeError('immutable output already exists: '+str(output))
        env['GOALCHEMY_BIN'] = os.path.relpath(compiler, caller) if kind == 'relative' else str(compiler)
        arg = os.path.relpath(output, caller) if kind == 'relative' else str(output)
        print('BUILD', target, kind, flush=True)
        command([SDK/('scripts/build-generated-'+target+'.sh'), arg], caller, env, destination/(kind+'.log'))
        # Cargo's target directory is a cache, so copy only the distributed crate.
        if target == 'rust':
            (output/'target/package').mkdir(parents=True, exist_ok=True)
            shutil.copyfile(base/'rust-package-build-cache/package/opentdf-tdf3-0.1.0.crate', output/'target/package/opentdf-tdf3-0.1.0.crate')
        outputs.append(output)
    finish(target, base, compiler, outputs, compiler_revision)


def finish(target, base, compiler, outputs, compiler_revision=None):
    """Freeze the verified pair, including after a scoped failed-build recovery."""
    destination = base/'packages'/target
    hashes = [members(output, target) for output in outputs]
    repeatable = hashes[0] == hashes[1]
    old = SDK.parent/'goalchemy/out'/(target+'-tdf-library/sdk')
    current_sources = production_sources(outputs[0], target)
    prior_sources = production_sources(old, target) if old.exists() else {}
    delta = {p: {'accepted': prior_sources.get(p), 'final': current_sources.get(p)}
             for p in sorted(set(current_sources)|set(prior_sources)) if current_sources.get(p) != prior_sources.get(p)}
    host_revision = subprocess.check_output(['git', '-C', str(SDK.parent/'goalchemy'), 'rev-parse', 'HEAD'], text=True).strip()
    receipt = {'target': target, 'compiler_sha256': sha(compiler), 'compiler_revision': compiler_revision or host_revision,
               'compiler_revision_provenance': 'explicit source pin; match binary SHA to compiler build receipt' if compiler_revision else 'current Goalchemy checkout at build',
               'host_goalchemy_revision': host_revision,
               'helper_sha256': sha(SDK/('scripts/build-generated-'+target+'.sh')),
               'caller_path_proofs': ['relative destination and relative compiler from independent caller', 'absolute destination and compiler from another independent caller'],
               'deliverable_members': hashes[0], 'repeatable': repeatable,
               'second_deliverable_members': hashes[1], 'production_sources': current_sources,
               'historical_default_artifact_tree': str(old), 'historical_default_artifact_source_delta': delta,
               'native_dependencies_lock_sha256': sha(SDK/('hosts/'+target+'/dependencies.lock.json')) if (SDK/('hosts/'+target+'/dependencies.lock.json')).exists() else None,
               'status': 0 if repeatable else 1, 'time': time.time()}
    (destination/'receipt.json').write_text(json.dumps(receipt, indent=2)+'\n')
    if not repeatable:
        raise RuntimeError('distributed members differ; see '+str(destination/'receipt.json'))
    print('PASS repeatable', target, 'source deltas', len(delta), flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('target', choices=(*TARGETS, 'all'))
    parser.add_argument('--base', type=Path, default=SDK/'.local/phase7')
    parser.add_argument('--compiler', type=Path, default=SDK/'.local/phase7/compiler/goalchemy')
    parser.add_argument('--compiler-revision', help='Recorded build source pin for a previously frozen compiler; binary hash remains authoritative')
    args = parser.parse_args()
    for target in TARGETS if args.target == 'all' else (args.target,):
        build(target, args.base.resolve(), args.compiler.resolve(),args.compiler_revision)


if __name__ == '__main__':
    main()
