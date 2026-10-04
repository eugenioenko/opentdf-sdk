"""Protect pooled benchmark reporting, warmup evidence, and normal runtime settings."""
import copy
import importlib.util
import json
import statistics
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('benchmark_sdk', ROOT / 'scripts/benchmark-sdk.py')
BENCH = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BENCH)


def native_result(samples=None):
    return {'correct': True, 'samples_ms': samples or [5, 6, 7, 8, 9],
            'warmup_ms': [100] * 20, 'bulk_warmup_ms': [500] * 40,
            'warmup_count': 20, 'bulk_warmup_count': 40, 'kas_calls_expected': 65}


def batch(index, samples):
    result = native_result(samples)
    return {**result, 'record_type': 'batch', 'batch_index': index, 'batches_requested': 3,
            'target': 'java', 'operation': 'e2e', 'size_bytes': 1024 * 1024,
            'size_label': '1MiB', 'fixture_id': '1', 'payload_sha256': 'fixture',
            'samples_requested': 5, 'status': 'ok', 'warmup_statistics': BENCH.warmup_statistics(result),
            'encrypted_samples_validated': [{'batch': index, 'sample': i} for i in range(-1, 5)]}


class BenchmarkPolicyTests(unittest.TestCase):
    def test_pool_all_samples_in_batch_order_without_fastest_selection(self):
        rows = [batch(2, [4, 5, 6, 7, 8]), batch(0, [1, 2, 3, 100, 101]),
                batch(1, [10, 11, 12, 13, 14])]
        aggregate = BENCH.aggregate_batches(rows)
        expected = rows[1]['samples_ms'] + rows[2]['samples_ms'] + rows[0]['samples_ms']
        self.assertEqual(aggregate['samples_ms'], expected)
        self.assertEqual(aggregate['median_ms'], statistics.median(expected))
        self.assertNotEqual(aggregate['median_ms'], statistics.median([statistics.median(r['samples_ms']) for r in rows]))
        self.assertEqual(aggregate['maximum_ms'], 101)
        self.assertEqual(len(aggregate['encrypted_samples_validated']), 18)

    def test_missing_batch_is_pending_and_failed_batch_is_failed(self):
        first = batch(0, [1, 2, 3, 4, 5])
        self.assertEqual(BENCH.aggregate_batches([first])['status'], 'pending')
        first['status'] = 'failed'
        self.assertEqual(BENCH.aggregate_batches([first])['status'], 'failed')

    def test_mixed_fixture_batches_are_rejected(self):
        rows = [batch(i, [1, 2, 3, 4, 5]) for i in range(3)]
        rows[2]['payload_sha256'] = 'different-input'
        with self.assertRaisesRegex(ValueError, 'inconsistent'):
            BENCH.aggregate_batches(rows)

    def test_full_finite_warmup_and_correctness_history_required(self):
        BENCH.validate_native_result(native_result(), 5, 20, 40)
        for field, value in [('warmup_ms', [1] * 19), ('bulk_warmup_ms', [1] * 39),
                             ('samples_ms', [1, 2, float('nan'), 4, 5]),
                             ('samples_ms', [1, 2, float('inf'), 4, 5]),
                             ('correct', False), ('kas_calls_expected', 64), ('warmup_count', 19)]:
            with self.subTest(field=field, value=value):
                result = native_result()
                result[field] = value
                with self.assertRaises(ValueError):
                    BENCH.validate_native_result(result, 5, 20, 40)

    def test_sustained_downward_trend_flags_investigation_preserving_samples(self):
        result = native_result([70, 71, 72, 73, 74])
        result['warmup_ms'] = [100] * 10 + [70] * 10
        before = copy.deepcopy(result)
        self.assertTrue(BENCH.warmup_statistics(result)['investigation_required'])
        self.assertEqual(result, before)
        result['samples_ms'] = [100] * 5
        self.assertFalse(BENCH.warmup_statistics(result)['investigation_required'])

    def test_remove_runtime_tuning_preserve_ordinary_paths(self):
        environment = {'PATH': '/tools', 'JAVA_HOME': '/jdk', 'DOTNET_ROOT': '/dotnet',
                       'DOTNET_CLI_HOME': '/cache', 'HTTP_PROXY': 'proxy',
                       **{key: 'tuned' for key in BENCH.RUNTIME_OPTIONS},
                       'COMPlus_TieredCompilation': '0', 'DOTNET_gcServer': '1'}
        normalized = BENCH.normal_environment(environment)
        self.assertEqual(normalized, {key: environment[key] for key in ['PATH', 'JAVA_HOME', 'DOTNET_ROOT', 'DOTNET_CLI_HOME', 'HTTP_PROXY']})
        self.assertEqual(environment['NODE_OPTIONS'], 'tuned')

    def test_normalization_retains_all_batches_and_public_table_uses_pooled_median(self):
        rows = [batch(0, [1, 2, 3, 100, 101]), batch(1, [10, 11, 12, 13, 14]), batch(2, [4, 5, 6, 7, 8])]
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            (base / 'raw.jsonl').write_text(''.join(json.dumps(row) + '\n' for row in rows))
            BENCH.tables(base)
            summary = json.loads((base / 'summary.json').read_text())
            self.assertEqual(len(summary), 1)
            self.assertEqual(len(summary[0]['batches']), 3)
            self.assertEqual(len(summary[0]['samples_ms']), 15)
            self.assertIn('| Java | 8.00 ms |', (base / 'tables.md').read_text())

    def test_invalid_cli_policy_rejected_before_building_or_contacting_services(self):
        with tempfile.TemporaryDirectory() as directory:
            result = subprocess.run([sys.executable, str(ROOT / 'scripts/benchmark-sdk.py'), '--output', directory, '--samples', '0'], capture_output=True, text=True, timeout=10)
            self.assertEqual(result.returncode, 2)
            self.assertIn('positive samples', result.stderr)
            self.assertFalse((Path(directory) / 'environment.json').exists())


if __name__ == '__main__':
    unittest.main()
