"""Full-profile coverage must distinguish expected oracle failures from SDK failures."""
import importlib.util
import itertools
import json
from pathlib import Path
import tempfile
import unittest

SDK = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location('coverage', SDK/'scripts/delivery-coverage.py')
coverage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(coverage)


class FullDPoPCoverageTests(unittest.TestCase):
    def fixture(self, root):
        output = root/'dpop'
        (output/'invocations').mkdir(parents=True)
        combinations = list(itertools.product(('rsa:2048','ec:secp256r1'), ('rsa:2048','ec:secp256r1'), ('ES256','RS256')))
        cases = ('empty','binary','exact','multiple','hs256','metadata','empty-metadata')
        pairs = []
        for case, (wrap, session, auth) in itertools.product(cases, combinations):
            for producer, consumer in (('generated-swift','stock-go'), ('go','generated-swift'), ('web','generated-swift')):
                pairs.append({'case':case,'wrapping':wrap,'session':session,'auth':auth,'producer':producer,'consumer':consumer})
        failures = [{'consumer':'stock-web','wrapping':wrap,'auth':auth,'success':False,'exit':1,'category':'authentication401'}
                    for wrap, auth in itertools.product(('rsa:2048','ec:secp256r1'), ('ES256','RS256'))]
        negatives = [{'case':'policy-denied','wrapping':wrap,'session':session,'auth':auth} for wrap, session, auth in combinations]
        negatives += [{'case':'mismatched-native-provider-auth-key','auth':auth} for auth in ('ES256','RS256')]
        (output/'results.json').write_text(json.dumps({'pairs':pairs,'negatives':negatives,'stock_web_limitations':failures}))
        events = [{'sequence':i,'terminal_status':1,'label':'stock-web-enforced-'+str(i),'artifacts':{}} for i in range(4)]
        index = output/'invocations/index.jsonl'
        index.write_text(''.join(json.dumps(event)+'\n' for event in events))
        supplement = root/'dpop-delivery-integrity'
        (supplement/'invocations').mkdir(parents=True)
        integrity = [{'wrapping':wrap,'session':session,'auth':auth,'mutation':mutation,'status':0,'zero_plaintext':True,'error':{'code':'integrity'}}
                     for wrap, session, auth in combinations for mutation in ('ciphertext','root')]
        (supplement/'results.json').write_text(json.dumps({'status':0,'cases':integrity}))
        (supplement/'invocations/index.jsonl').write_text('')
        return output, index

    def test_full_dpop_accepts_attributed_stock_web_nonce_failures(self):
        with tempfile.TemporaryDirectory() as directory:
            output, _ = self.fixture(Path(directory))
            result = coverage.check('swift','dpop','full',output)
            self.assertEqual(result['status'], 0)
            self.assertEqual(result['observed_comparisons'], 168)

    def test_full_dpop_rejects_unexpected_native_failure(self):
        with tempfile.TemporaryDirectory() as directory:
            output, index = self.fixture(Path(directory))
            with index.open('a') as stream:
                stream.write(json.dumps({'sequence':4,'terminal_status':1,'label':'decrypt-native','artifacts':{}})+'\n')
            with self.assertRaisesRegex(RuntimeError, 'unexpected nonzero invocation decrypt-native'):
                coverage.check('swift','dpop','full',output)
