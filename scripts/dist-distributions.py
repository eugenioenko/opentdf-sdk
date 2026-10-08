#!/usr/bin/env python3
"""Regenerate, format, or check the committed eight-target source distributions."""
from pathlib import Path
import argparse
import hashlib
import importlib.util
import json
import os
import re
import shutil
import subprocess
import urllib.request

SDK = Path(__file__).resolve().parents[1]
TARGETS = ('go', 'typescript', 'java', 'csharp', 'python', 'rust', 'c', 'swift')
PIN = json.loads((SDK / 'references.lock.json').read_text())['repositories']['goalchemy']['revision']
TOOLS = SDK / '.local/dist-formatters'
EXCLUDE = {'classes', 'lib', 'obj', 'bin', 'target', 'dist', 'package', 'build', '__pycache__',
           'opentdf_tdf3.egg-info', 'node_modules', '.build', '.swiftpm'}
EXTENSIONS = {'.go', '.ts', '.java', '.cs', '.py', '.rs', '.c', '.h', '.json', '.toml', '.lock', '.csproj', '.sh', '.mod', '.sum', '.typed', '.swift', '.modulemap'}


def run(command, **kwargs):
    subprocess.run([str(p) for p in command], check=True, **kwargs)


def source_member(relative):
    """Only shipped sources, metadata and notices; never local build products."""
    if EXCLUDE.intersection(relative.parts):
        return False
    if relative.name in {'package-sha256.json', 'native-link.env', 'line-map.json', 'source-map.json'}:
        return False
    return ('licenses' in relative.parts or relative.suffix in EXTENSIONS
            or relative.name in {'README.md', 'LICENSE', '.editorconfig'})


def hashes(root):
    return {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted(root.rglob('*'))
            if p.is_file() and source_member(p.relative_to(root))}


def formatter_tools():
    TOOLS.mkdir(parents=True, exist_ok=True)
    if not (TOOLS / 'venv').exists():
        run(['python3', '-m', 'venv', TOOLS / 'venv'])
        run([TOOLS / 'venv/bin/pip', 'install', 'black==25.1.0', 'clang-format==18.1.8'])
    if not (TOOLS / 'node_modules/prettier').exists():
        shutil.copyfile(SDK / 'scripts/dist-tooling/package.json', TOOLS / 'package.json')
        shutil.copyfile(SDK / 'scripts/dist-tooling/package-lock.json', TOOLS / 'package-lock.json')
        run(['npm', 'ci', '--prefix', TOOLS, '--ignore-scripts', '--no-audit', '--no-fund'])
    jar = TOOLS / 'google-java-format-1.28.0-all-deps.jar'
    if not jar.exists():
        urllib.request.urlretrieve('https://repo.maven.apache.org/maven2/com/google/googlejavaformat/google-java-format/1.28.0/google-java-format-1.28.0-all-deps.jar', jar)
    if hashlib.sha256(jar.read_bytes()).hexdigest() != '32342e7c1b4600f80df3471da46aee8012d3e1445d5ea1be1fb71289b07cc735':
        raise RuntimeError('google-java-format checksum mismatch')
    run(['rustup', 'component', 'add', 'rustfmt', '--toolchain', '1.98.0'])
    return jar


def format_tree(root, targets):
    jar = formatter_tools()
    java = Path(os.environ.get('JAVA_HOME', SDK.parent / 'goalchemy/.toolchains/jdk-21.0.12.1+1')) / 'bin/java'
    dotnet = os.environ.get('DOTNET_BIN', str(SDK.parent / 'goalchemy/.toolchains/dotnet/dotnet'))
    if subprocess.check_output([dotnet, '--version'], text=True).strip() != '8.0.425':
        raise RuntimeError('dotnet formatter requires SDK 8.0.425')
    for target in targets:
        tree = root / target
        files = lambda suffix: sorted(p for p in tree.rglob('*' + suffix)
                                      if 'licenses' not in p.relative_to(tree).parts
                                      and source_member(p.relative_to(tree)))
        if target == 'go':
            run(['go', 'fmt', './...'], cwd=tree, env={**os.environ, 'GOTOOLCHAIN': 'go1.25.14'})
        elif target == 'typescript':
            run([TOOLS / 'node_modules/.bin/prettier', '--config', SDK / 'scripts/dist-prettier.json', '--ignore-path', SDK / 'scripts/dist-prettier-ignore', '--write', *files('.ts')])
        elif target == 'java':
            # Generated dispatch methods can require a larger parser stack.
            run([java, '-Xss16m', '-jar', jar, '--replace', *files('.java')])
        elif target == 'csharp':
            shutil.copyfile(SDK / 'scripts/dist-editorconfig', tree / '.editorconfig')
            run([dotnet, 'format', 'whitespace', tree, '--folder', '--include', *files('.cs'), '--verbosity', 'quiet'])
        elif target == 'python':
            run([TOOLS / 'venv/bin/black', '--config', SDK / 'scripts/dist-black.toml', *files('.py')])
        elif target == 'rust':
            run(['rustfmt', '+1.98.0', '--config-path', SDK / 'scripts/dist-rustfmt.toml', *files('.rs')])
        elif target == 'swift':
            formatter = SDK.parent / 'goalchemy/.toolchains/swift-6.4.0/usr/bin/swift-format'
            run([formatter, 'format', '--configuration', SDK / 'scripts/dist-swift-format.json', '--in-place', *files('.swift')])
            # Swift packages also ship their native crypto/HTTP/CRC shim.
            run([TOOLS / 'venv/bin/clang-format', '--style=file:' + str(SDK / 'scripts/dist-clang-format.yaml'), '-i', *files('.c'), *files('.h')])
        elif target == 'c':
            run([TOOLS / 'venv/bin/clang-format', '--style=file:' + str(SDK / 'scripts/dist-clang-format.yaml'), '-i', *files('.c'), *files('.h')])
        configs = files('.json')
        if configs:
            run([TOOLS / 'node_modules/.bin/prettier', '--config', SDK / 'scripts/dist-prettier.json', '--ignore-path', SDK / 'scripts/dist-prettier-ignore', '--write', *configs])


def build(targets, base, compiler):
    # Reuse the accepted delivery builders for native manifests, locks and notices.
    spec = importlib.util.spec_from_file_location('packages', SDK / 'scripts/delivery-packages.py')
    packages = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(packages)
    for target in targets:
        packages.build(target, base, compiler, PIN)


def regenerate(targets, base, destination):
    compiler = base / 'compiler/goalchemy'
    compiler.parent.mkdir(parents=True, exist_ok=True)
    goalchemy = SDK.parent / 'goalchemy'
    revision = subprocess.check_output(['git', '-C', goalchemy, 'rev-parse', 'HEAD'], text=True).strip()
    if revision != PIN:
        raise RuntimeError('Goalchemy checkout must match references.lock.json: ' + PIN)
    if subprocess.check_output(['git', '-C', goalchemy, 'status', '--porcelain']):
        raise RuntimeError('Goalchemy checkout must be clean, including untracked source files')
    # The frozen trimpath compiler is relocated under --base. Verify and pass
    # its standard-library root exactly as the existing delivery bootstrap does.
    spec = importlib.util.spec_from_file_location('bootstrap', SDK / 'scripts/delivery-bootstrap.py')
    bootstrap = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(bootstrap)
    environment = {**os.environ, 'GOTOOLCHAIN': 'go1.25.14'}
    os.environ['GOROOT'] = bootstrap.pinned_go_root(environment)
    run(['go', 'build', '-trimpath', '-o', compiler, './cmd/goalchemy'], cwd=goalchemy,
        env={**os.environ, 'GOTOOLCHAIN': 'go1.25.14'})
    build(targets, base, compiler=compiler)
    assemble(targets, base, destination)


def assemble(targets, base, destination, kind="relative"):
    for target in targets:
        output = destination / target
        shutil.rmtree(output, ignore_errors=True)
        output.mkdir(parents=True)
        source = base / 'packages' / target / kind
        for p in sorted(source.rglob('*')):
            rel = p.relative_to(source)
            if not p.is_file() or not source_member(rel):
                continue
            dest = output / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(p, dest)
            if target == 'typescript' and p.suffix == '.ts':
                # These maps describe the unformatted compiler output and are
                # excluded below. Do not leave dangling debugger references.
                dest.write_text(''.join(line for line in dest.read_text().splitlines(keepends=True)
                                        if not line.startswith('//# sourceMappingURL=')))
        if target == 'c':
            shutil.copyfile(SDK / 'src/hosts/c/dependencies.lock.json', output / 'dependencies.lock.json')
            shutil.copytree(SDK / 'src/hosts/c/licenses', output / 'licenses', dirs_exist_ok=True)
        if target in ('java', 'c'):
            shutil.copyfile(SDK / 'scripts/dist-build' / (target + '.sh'), output / 'build-sdk.sh')
        if target == 'typescript':
            manifest = output / 'package.json'
            package = json.loads(manifest.read_text())
            package['devDependencies'] = {'typescript': '6.0.3'}
            manifest.write_text(json.dumps(package, indent=2) + '\n')
            run(['npm', 'install', '--package-lock-only', '--ignore-scripts', '--no-audit', '--no-fund'], cwd=output)
        if target == 'csharp':
            project = output / 'main.csproj'
            project.write_text(project.read_text().replace(str(source), '$(MSBuildProjectDirectory)'))
        if target == 'swift':
            # SwiftPM warns about excludes for sidecars deliberately omitted
            # after formatting. Keep its source list, but remove those entries.
            project = output / 'Package.swift'
            project.write_text(re.sub(r'"[^"\n]+\.(?:lines|map)",\s*', '', project.read_text()))
        # Sidecars describe pre-format positions; require every native member
        # rather than concealing a missing module with a filtered inventory.
        for manifest in output.rglob('goalchemy.manifest.json'):
            metadata = json.loads(manifest.read_text())
            for inventory in ('generated_files', 'runtime_files'):
                shipped = []
                for name in metadata.get(inventory, []):
                    if Path(name).suffix in ('.map', '.lines'):
                        continue
                    if not (manifest.parent / name).is_file():
                        raise RuntimeError('distribution member missing: ' + str(manifest.parent / name))
                    shipped.append(name)
                metadata[inventory] = sorted(set(shipped))
            if target == 'rust':
                if not (manifest.parent / 'src/generated.rs').is_file():
                    raise RuntimeError('Rust distribution is missing its generated entry')
                metadata['generated_files'].append('src/generated.rs')
                metadata['generated_files'] = sorted(set(metadata['generated_files']))
            for package in metadata.get('source_packages', []):
                for name in package.get('files', []):
                    if not (manifest.parent / name).is_file():
                        raise RuntimeError('source-package member missing: ' + str(manifest.parent / name))
            metadata['distribution'] = {'compiler_revision': PIN, 'formatted': True,
                                        'diagnostic_line_maps': 'omitted after formatting'}
            manifest.write_text(json.dumps(metadata, indent=2) + '\n')
        usage = (SDK / 'docs/dist-usage' / (target + '.md')).read_text()
        (output / 'README.md').write_text(usage)
    format_tree(destination, targets)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('regenerate', 'format', 'check'))
    parser.add_argument('--target', choices=(*TARGETS, 'all'), default='all')
    parser.add_argument('--base', type=Path, default=SDK / '.local/dist-work/run')
    parser.add_argument('--destination', type=Path, default=SDK / 'dist')
    parser.add_argument('--environment', type=Path, default=SDK / '.local/dist-bootstrap/environment.json')
    args = parser.parse_args()
    if args.environment.exists():
        os.environ.update(json.loads(args.environment.read_text()))
    targets = TARGETS if args.target == 'all' else (args.target,)
    base, destination = args.base.resolve(), args.destination.resolve()
    if args.action == 'format':
        format_tree(destination, targets)
    else:
        if base.exists():
            raise RuntimeError('use a fresh --base for immutable package evidence: ' + str(base))
        expected = {name: digest for name, digest in hashes(destination).items()
                    if name.split('/')[0] in targets} if args.action == 'check' else None
        output = base / 'dist' if expected is not None else destination
        regenerate(targets, base, output)
        first = hashes(output)
        format_tree(output, targets)
        if hashes(output) != first:
            raise RuntimeError('formatting is not idempotent')
        if expected is not None and first != expected:
            changed = sorted(p for p in set(first) | set(expected) if first.get(p) != expected.get(p))
            raise RuntimeError('committed distributions drift: ' + ', '.join(changed))
        print('PASS distributions ' + args.action + '; formatting idempotent')


if __name__ == '__main__':
    main()
