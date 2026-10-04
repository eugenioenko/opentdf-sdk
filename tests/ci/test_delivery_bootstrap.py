"""Exercise the real bootstrap CLI with isolated native tool processes."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest


SDK = Path(__file__).resolve().parents[2]


class BootstrapVersionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.sdk = self.root / 'sdk'
        scripts = self.sdk / 'scripts'
        scripts.mkdir(parents=True)
        shutil.copyfile(SDK / 'scripts/delivery-bootstrap.py', scripts / 'delivery-bootstrap.py')
        (self.sdk / 'references.lock.json').write_text('{"repositories": {}}\n')
        self.compiler = self.root / 'goalchemy'
        self.compiler.mkdir()
        (self.compiler / 'toolchains.lock').write_text('')
        self.tools = self.root / 'tools'
        self.tools.mkdir()
        self.goroot = self.root / 'pinned-go'
        for relative in ('src/runtime/runtime2.go','src/unsafe/unsafe.go'):
            path = self.goroot / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text('// isolated standard library fixture\n')
        (self.goroot / 'VERSION').write_text('go1.25.14\n')
        self.environment = os.environ.copy()
        self.environment.pop('GOROOT', None)
        self.environment['PATH'] = str(self.tools) + os.pathsep + self.environment['PATH']
        self.tool('go', 'go version go1.25.14 linux/amd64\n')
        self.tool('go', 'go version go1.25.14 linux/amd64\n', path=self.goroot / 'bin/go')
        self.tool('node', 'v24.15.0\n')
        self.tool('python3', 'Python 3.10.21\n')
        self.output = self.sdk / 'output'

    def tool(self, name, stdout='', stderr='', status=0, path=None):
        path = path or self.tools / name
        path.parent.mkdir(parents=True, exist_ok=True)
        source = ('#!' + sys.executable + '\nimport os,sys\n'
                  + ('assert os.environ["GOTOOLCHAIN"] in ("go1.25.14","local")\n' if name == 'go' else '')
                  + ('if sys.argv[1:] == ["env","GOROOT"]:print(' + repr(str(self.goroot)) + ');sys.exit(0)\n' if name == 'go' else '')
                  + 'sys.stdout.write(' + repr(stdout) + ')\n'
                  + 'sys.stderr.write(' + repr(stderr) + ')\n'
                  + 'sys.exit(' + str(status) + ')\n')
        path.write_text(source)
        path.chmod(0o755)

    def run_bootstrap(self, target='go'):
        return subprocess.run([sys.executable, self.sdk / 'scripts/delivery-bootstrap.py',
                               target, '--base', self.output], env=self.environment,
                              capture_output=True, text=True, timeout=15)

    def test_cold_download_notice_preserved_without_corrupting_version(self):
        notice = 'go: downloading go1.25.14 (linux/amd64)\n'
        self.tool('go', 'go version go1.25.14 linux/amd64\n', notice)
        result = self.run_bootstrap()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(notice, result.stderr)
        receipt = json.loads((self.output / 'bootstrap-receipt.json').read_text())
        self.assertEqual(receipt['versions']['go'], 'go version go1.25.14 linux/amd64')
        self.assertNotIn('downloading', receipt['versions']['go'])
        self.assertEqual(receipt['environment']['GOTOOLCHAIN'], 'go1.25.14')
        self.assertEqual(receipt['environment']['GOROOT'], str(self.goroot))

    def test_goroot_metadata_must_match_pinned_toolchain(self):
        (self.goroot / 'VERSION').write_text('go1.25.13\n')
        result = self.run_bootstrap()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('GOROOT must match Go1.25.14', result.stderr)
        self.assertFalse((self.output / 'environment.json').exists())

    def test_goroot_must_contain_standard_library_sources(self):
        (self.goroot / 'src/runtime/runtime2.go').unlink()
        result = self.run_bootstrap()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('GOROOT must contain its toolchain and standard library', result.stderr)
        self.assertFalse((self.output / 'environment.json').exists())

    def test_goroot_executable_must_match_pinned_toolchain(self):
        self.tool('go', 'go version go1.25.13 linux/amd64\n', path=self.goroot / 'bin/go')
        result = self.run_bootstrap()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('GOROOT executable must match Go1.25.14', result.stderr)
        self.assertFalse((self.output / 'environment.json').exists())

    def test_wrong_go_version_rejected(self):
        self.tool('go', 'go version go1.25.13 linux/amd64\n')
        result = self.run_bootstrap()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Go1.25.14 required', result.stderr)
        self.assertFalse((self.output / 'bootstrap-receipt.json').exists())

    def test_stderr_cannot_override_wrong_stdout_version(self):
        self.tool('go', 'go version go1.25.13 linux/amd64\n', 'go version go1.25.14 linux/amd64\n')
        result = self.run_bootstrap()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Go1.25.14 required', result.stderr)

    def test_stderr_version_alone_cannot_satisfy_go_version(self):
        self.tool('go', '', 'go version go1.25.14 linux/amd64\n')
        result = self.run_bootstrap()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Go1.25.14 required', result.stderr)

    def test_stderr_warning_does_not_override_correct_stdout_version(self):
        self.tool('go', 'go version go1.25.14 linux/amd64\n', 'warning: cached go1.25.13 is unused\n')
        result = self.run_bootstrap()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('cached go1.25.13', result.stderr)

    def test_failed_command_preserves_diagnostics_and_rejects_valid_output(self):
        self.tool('go', 'go version go1.25.14 linux/amd64\n', 'go: toolchain download failed\n', 7)
        result = self.run_bootstrap()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('go: toolchain download failed', result.stderr)
        self.assertIn('go version go1.25.14 linux/amd64', result.stderr)
        self.assertIn('exit status 7', result.stderr)
        self.assertFalse((self.output / 'bootstrap-receipt.json').exists())

    def test_wrong_node_version_still_rejected(self):
        self.tool('node', 'v22.0.0\n')
        result = self.run_bootstrap()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Node24.15.0 required', result.stderr)

    def test_java_version_remains_explicitly_stderr_based(self):
        archive = self.compiler / '.toolchains/downloads/OpenJDK21U-jdk_x64_linux_hotspot_21.0.12.1_1.tar.gz'
        archive.parent.mkdir(parents=True)
        archive.write_bytes(b'isolated test toolchain archive')
        (self.compiler / 'toolchains.lock').write_text("JDK_SHA256='" + hashlib.sha256(archive.read_bytes()).hexdigest() + "'\n")
        fetch = self.compiler / 'scripts/fetch-toolchains.sh'
        self.tool('fetch', path=fetch)
        version = 'openjdk version "21.0.12.1" 2026-07-21\nOpenJDK Runtime Environment\n'
        self.tool('java', stderr=version, path=self.compiler / '.toolchains/jdk-21.0.12.1+1/bin/java')
        result = self.run_bootstrap('java')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn(version, result.stderr)
        receipt = json.loads((self.output / 'bootstrap-receipt.json').read_text())
        self.assertEqual(receipt['versions']['java'], version.strip())
        self.assertEqual(receipt['artifacts'][archive.name], hashlib.sha256(archive.read_bytes()).hexdigest())


    def test_all_targets_keeps_version_probe_callable_after_wheel_download(self):
        archives = {'JDK':'OpenJDK21U-jdk_x64_linux_hotspot_21.0.12.1_1.tar.gz',
                    'DOTNET':'dotnet-sdk-8.0.425-linux-x64.tar.gz','BDWGC':'gc-8.2.8.tar.gz'}
        lock = []
        for name, filename in archives.items():
            path = self.compiler / '.toolchains/downloads' / filename
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(name.encode())
            lock.append(name + "_SHA256='" + hashlib.sha256(path.read_bytes()).hexdigest() + "'")
        (self.compiler / 'toolchains.lock').write_text('\n'.join(lock) + '\n')
        self.tool('fetch', path=self.compiler / 'scripts/fetch-toolchains.sh')
        self.tool('java', stderr='openjdk version "21.0.12.1"\n',
                  path=self.compiler / '.toolchains/jdk-21.0.12.1+1/bin/java')
        self.tool('dotnet', '8.0.425\n', path=self.compiler / '.toolchains/dotnet/dotnet')
        gc = self.compiler / '.toolchains/bdwgc/lib/libgc.a'
        gc.parent.mkdir(parents=True)
        gc.write_bytes(b'isolated collector')
        delivery = self.sdk / 'src/hosts/delivery'
        delivery.mkdir(parents=True)
        for name in ('package.json', 'package-lock.json'):
            (delivery / name).write_text('{}\n')
        self.tool('npm')
        self.tool('playwright', path=self.output / 'tooling/node_modules/.bin/playwright')
        python_host = self.sdk / 'src/hosts/python'
        python_host.mkdir(parents=True)
        wheel = 'sample-1.0-py3-none-any.whl'
        content = b'isolated downloaded wheel'
        (python_host / 'dependencies.lock.json').write_text(json.dumps([{
            'artifact':wheel,'sha256':hashlib.sha256(content).hexdigest(),
            'metadata':['Name: sample','Version: 1.0']}]))
        python = self.tools / 'python3'
        python.write_text('#!' + sys.executable + '\nimport pathlib,sys\n'
                          'if "download" in sys.argv:\n'
                          ' path=pathlib.Path(sys.argv[sys.argv.index("-d")+1])\n'
                          ' path.mkdir(parents=True,exist_ok=True)\n'
                          ' (path/' + repr(wheel) + ').write_bytes(' + repr(content) + ')\n'
                          'elif "--version" in sys.argv:print("Python 3.10.21")\n')
        python.chmod(0o755)
        self.tool('venv-python', path=self.output / 'python-build-venv/bin/python')
        self.tool('rustc', 'rustc 1.98.0 (isolated fixture)\n')
        c_host = self.sdk / 'src/hosts/c'
        c_host.mkdir(parents=True)
        (c_host / 'dependencies.lock.json').write_text(json.dumps({'dependencies':[
            {'name':'libcurl','artifacts':[]},{'name':'OpenSSL','runtime_sha256':{}}]}))
        result = self.run_bootstrap('all')
        self.assertEqual(result.returncode, 0, result.stderr)
        receipt = json.loads((self.output / 'bootstrap-receipt.json').read_text())
        self.assertEqual(receipt['targets'], ['go','typescript','java','csharp','python','rust','c'])
        self.assertEqual(receipt['versions']['rustc'], 'rustc 1.98.0 (isolated fixture)')
        self.assertEqual(receipt['artifacts'][wheel], hashlib.sha256(content).hexdigest())
        self.assertTrue(any('sample==1.0' in command['command'] for command in receipt['commands']))

    def test_bootstrap_job_reports_safe_version_summary_without_raw_log(self):
        source = ('#!' + sys.executable + '\nimport json,pathlib,sys\n'
                  'base=pathlib.Path(sys.argv[sys.argv.index("--base")+1])\n'
                  'base.mkdir(parents=True,exist_ok=True)\n'
                  '(base/"bootstrap-version-failure.json").write_text(json.dumps({"tool":"go","reason":"command failed","exit_status":7}))\n'
                  'print("PRIVATE_SECRET_CANARY")\n'
                  'sys.exit(7)\n')
        fake = self.tools / 'python3'
        fake.write_text(source)
        fake.chmod(0o755)
        self.environment['TDF_COMPOSE_PROJECT'] = 'phase7-bootstrap-regression'
        result = subprocess.run([sys.executable, SDK / 'scripts/delivery-job.py', 'go',
                                 '--base', self.output], env=self.environment,
                                capture_output=True, text=True, timeout=15)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Bootstrap version check failed: go command failed', result.stdout)
        self.assertNotIn('PRIVATE_SECRET_CANARY', result.stdout + result.stderr)
        logs = list((self.output / 'jobs').glob('*/bootstrap.log'))
        self.assertEqual(len(logs), 1)
        self.assertIn('PRIVATE_SECRET_CANARY', logs[0].read_text())
        receipts = list((self.output / 'public').glob('*/failed-job-receipt.json'))
        self.assertEqual(len(receipts), 1)
        self.assertEqual(json.loads(receipts[0].read_text())['status'], 7)

    def test_package_failure_artifact_preserves_generic_child_error_and_excludes_private_logs(self):
        failure = 'shell/import/typecheck failure: unexpected token at native build\n'
        source = ('#!' + sys.executable + '\nimport pathlib,sys\n'
                  'base=pathlib.Path(sys.argv[sys.argv.index("--base")+1])\n'
                  'if "delivery-bootstrap.py" in sys.argv[1]:\n'
                  ' (base/"environment.json").write_text("{}")\n'
                  'else:\n'
                  ' log=base/"packages/go/relative.log"\n'
                  ' log.parent.mkdir(parents=True,exist_ok=True)\n'
                  ' log.write_text(' + repr(failure) + ')\n'
                  ' log.with_suffix(".log.status").write_text("7\\n")\n'
                  ' job=next((base/"jobs").iterdir())\n'
                  ' (job/"private-service-up.log").write_text("PRIVATE_AUTH_SECRET_CANARY")\n'
                  ' (log.parent/"private-token.json").write_text("PRIVATE_AUTH_SECRET_CANARY")\n'
                  ' print("RuntimeError: native package helper failed")\n'
                  ' sys.exit(7)\n')
        fake = self.tools / 'python3'
        fake.write_text(source)
        fake.chmod(0o755)
        self.environment['TDF_COMPOSE_PROJECT'] = 'phase7-package-diagnostic-regression'
        result = subprocess.run([sys.executable, SDK / 'scripts/delivery-job.py', 'go',
                                 '--base', self.output], env=self.environment,
                                capture_output=True, text=True, timeout=15)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Build failure diagnostics:', result.stdout)
        receipts = list((self.output / 'public').glob('*/failed-job-receipt.json'))
        self.assertEqual(len(receipts), 1)
        receipt = json.loads(receipts[0].read_text())
        self.assertEqual([item['status'] for item in receipt['commands']], [0,0,7])
        self.assertEqual({item['artifact'] for item in receipt['build_failure_diagnostics']}, {
            'build-failure-diagnostics/packages.log',
            'build-failure-diagnostics/packages/go/relative.log',
            'build-failure-diagnostics/packages/go/relative.log.status'})
        for item in receipt['build_failure_diagnostics']:
            artifact = receipts[0].parent / item['artifact']
            self.assertEqual(hashlib.sha256(artifact.read_bytes()).hexdigest(), item['sha256'])
        child = receipts[0].parent / 'build-failure-diagnostics/packages/go/relative.log'
        self.assertEqual(child.read_text(), failure)
        self.assertEqual(child.with_suffix('.log.status').read_text(), '7\n')
        for path in (self.output / 'public').rglob('*'):
            if path.is_file():self.assertNotIn('PRIVATE_AUTH_SECRET_CANARY', path.read_text())
        self.assertNotIn('PRIVATE_AUTH_SECRET_CANARY', result.stdout + result.stderr)

    def test_private_service_failure_never_reports_raw_logs_or_stale_version_summary(self):
        self.tool('bash', stderr='PRIVATE_SERVICE_SECRET_CANARY\n', status=9)
        self.output.mkdir()
        (self.output / 'bootstrap-version-failure.json').write_text(json.dumps({'tool':'go','reason':'command failed','exit_status':7}))
        self.environment['TDF_COMPOSE_PROJECT'] = 'phase7-private-service-regression'
        result = subprocess.run([sys.executable, SDK / 'scripts/delivery-job.py', 'go',
                                 '--base', self.output, '--packages-base', self.sdk / 'unused-packages'],
                                env=self.environment, capture_output=True, text=True, timeout=15)
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('PRIVATE_SERVICE_SECRET_CANARY', result.stdout + result.stderr)
        self.assertNotIn('Bootstrap version check failed', result.stdout + result.stderr)
        logs = list((self.output / 'jobs').glob('*/private-service-up.log'))
        self.assertEqual(len(logs), 1)
        self.assertIn('PRIVATE_SERVICE_SECRET_CANARY', logs[0].read_text())
        receipts = list((self.output / 'public').glob('*/failed-job-receipt.json'))
        self.assertEqual(len(receipts), 1)
        self.assertEqual(json.loads(receipts[0].read_text())['build_failure_diagnostics'], [])
        self.assertFalse(list((self.output / 'public').glob('*/build-failure-diagnostics')))


if __name__ == '__main__':
    unittest.main()
