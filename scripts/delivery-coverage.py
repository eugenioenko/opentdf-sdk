#!/usr/bin/env python3
"""Fail on missing exact required cases; focused evidence never counts as full."""
from pathlib import Path
import argparse
import hashlib
import itertools
import json
import tarfile

SDK = Path(__file__).resolve().parents[1]
TARGETS = ('go','typescript','java','csharp','python','rust','c')
CASES = ('empty','binary','exact','multiple','hs256','metadata','empty-metadata')


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def direction(row):
    producer, consumer = row['producer'], row['consumer']
    if consumer in ('stock-go','stock-web'):
        return 'target->'+consumer
    if producer == 'generated' or producer.startswith('generated-'):
        return 'self'
    if producer in ('go','web') or producer.startswith(('stock-go','stock-web','go-format','web-format')):
        return ('stock-go' if 'go' in producer else 'stock-web')+'->target'
    return 'self'


def validate_events(output, events, allow_reference_401=False):
    for sequence, event in enumerate(events):
        assert event['sequence'] == sequence, 'invocation ordering drift'
        if event['terminal_status'] and not (allow_reference_401 and event['label'].startswith('stock-web-enforced-')):
            raise RuntimeError('unexpected nonzero invocation '+event['label'])
        for artifact in event['artifacts'].values():
            blob = output/'invocations/objects'/artifact['sha256']
            if not blob.is_file() or sha(blob) != artifact['sha256']:
                raise RuntimeError('immutable captured artifact missing/drifted '+str(blob))


def check(target, profile, mode, output):
    path = output/'results.json'
    if not path.is_file():
        raise RuntimeError('required result missing: '+str(path))
    result = json.loads(path.read_text())
    if result.get('status',0) != 0:
        raise RuntimeError('required result nonzero: '+str(path))
    browser = target == 'typescript-browser'
    cases = CASES if mode == 'full' else ('binary' if profile == 'basic' else 'empty-metadata',)
    combinations = [('rsa:2048','rsa:2048','ES256')] if profile == 'basic' else list(itertools.product(('rsa:2048','ec:secp256r1'),('rsa:2048','ec:secp256r1'),('ES256','RS256')))
    directions = ['target->stock-go','stock-go->target','stock-web->target']
    if profile != 'dpop':
        directions.append('target->stock-web')
    expected = {(case,wrap,session,auth,d) for case in cases for wrap,session,auth in combinations for d in directions}
    observed = set()
    events = []
    if not browser:
        index = output/'invocations/index.jsonl'
        if not index.exists():
            raise RuntimeError('required per-invocation archive missing: '+str(index))
        events = [json.loads(line) for line in index.read_text().splitlines()]
        validate_events(output,events,mode == 'full' and profile == 'dpop')
    for row in result['pairs']:
        d = direction(row)
        if d == 'self':
            continue
        key = (row['case'],row.get('wrap',row.get('wrapping','rsa:2048')),row.get('session','rsa:2048'),row.get('auth','ES256'),d)
        if key in observed:
            raise RuntimeError('duplicate claimed coverage '+str(key))
        observed.add(key)
        if mode == 'focused' and not browser:
            event = events[row['invocation']]
            assert event['terminal_status'] == 0
            config = event['configuration']['sanitized']
            assert config['KASAlgorithm'] == key[1] and config['SessionAlgorithm'] == key[2] and config['AuthAlgorithm'] == key[3]
            assert config['DPoP'] == (profile == 'dpop')
            if 'has_metadata' in row:
                assert event['label'].startswith('decrypt-')
                presence_name = event['label'][len('decrypt-'):]+'.presence'
                artifact = event['artifacts'][presence_name]
                observed_presence = (output/'invocations/objects'/artifact['sha256']).read_text().strip()
                assert observed_presence == str(row['has_metadata']).lower()
    missing, unexpected = expected-observed, observed-expected
    if missing or unexpected:
        raise RuntimeError(str(path)+' missing '+str(sorted(missing))+' unexpected '+str(sorted(unexpected)))
    if mode == 'full' and profile == 'dpop' and not browser:
        limitations = result.get('stock_web_limitations',[])
        keys = {(row['wrapping'],row['auth']) for row in limitations}
        assert keys == set(itertools.product(('rsa:2048','ec:secp256r1'),('ES256','RS256')))
        assert all(not row['success'] and row['exit'] != 0 and row['category'] == 'authentication401' for row in limitations)
    if mode == 'focused' and profile == 'basic' and not browser:
        labels = {event['label'] for event in events}
        assert {'repeat-ownership','negative-malformed'} <= labels
        assert json.loads((output/'malformed.error.json').read_text())['code'] == 'archive'
        if target == 'c':
            assert 'public-no-preinit-real-kas' in labels
    if mode == 'full' and not browser:
        if profile == 'basic':
            assert {'grouped-denial','integrity','archive','unauthenticated401','expired-token','provider-rejection','untrusted-route','root-tampering','unsupported-root','unsupported-assertion'} <= set(result['typed_negatives'])
            if target == 'c':
                proof = json.loads((output/'delivery-no-preinit/receipt.json').read_text())
                assert proof['status'] == 0 and proof['archive_sha256'] == sha(output/'binary.go.tdf')
            controlled_path = output/'controlled-negatives.json' if target == 'go' else output.parent/'controlled/results.json'
            controlled = json.loads(controlled_path.read_text())
            controls = controlled.get('cases',controlled.get('negatives'))
            required_controls = {'http401','http403','redirect','content-type','malformed-response','oversized-response','untrusted-tls','caller-deadline','source-deadline','active-http-cancel'} if target in ('go','typescript') else {'http401','http403','redirect','content-type','malformed','oversize','truncated','tls','deadline','active-cancel','bom'}
            if target == 'c':
                required_controls.add('header-oversize')
            assert {row['case'] for row in controls} == required_controls
            assert all(row.get('zeroOutput',row.get('zero_output',False)) for row in controls)
        else:
            validate_events(output,[json.loads(line) for line in (output/'invocations/index.jsonl').read_text().splitlines()])
            denied = {(row['wrapping'],row['session'],row['auth']) for row in result['negatives'] if row['case'] == 'policy-denied'}
            assert denied == set(combinations), 'missing profile policy denial'
            supplement = output.parent/(profile+'-delivery-integrity')
            evidence = json.loads((supplement/'results.json').read_text())
            expected_integrity = {(w,s,a,m) for w,s,a in combinations for m in ('ciphertext','root')}
            actual_integrity = {(row['wrapping'],row['session'],row['auth'],row['mutation']) for row in evidence['cases']}
            assert evidence['status'] == 0 and expected_integrity == actual_integrity
            assert all(row['status'] == 0 and row['zero_plaintext'] and row['error']['code'] == 'integrity' for row in evidence['cases'])
            validate_events(supplement,[json.loads(line) for line in (supplement/'invocations/index.jsonl').read_text().splitlines()])
            if profile == 'dpop':
                binding = {row['auth'] for row in result['negatives'] if row['case'] == 'mismatched-native-provider-auth-key'}
                assert binding == {'ES256','RS256'}, 'missing native provider binding denial'
    if browser:
        assert result['graphInputs'] > 0 and result['brokerRequests'] > 0
        if mode == 'focused':
            assert len(result['events']) == len(combinations)
            assert all(event['terminal_status'] == 0 for event in result['events'])
            for event in result['events']:
                assert not event['configuration'].get('ClientSecret') and not event['configuration'].get('ClientID')
                for suffix, artifact in event['artifacts'].items():
                    assert sha(output/(event['name']+'.'+suffix)) == artifact['sha256']
            if profile == 'dpop':
                assert any(row['status'] == 401 and row['nonceReadable'] and '8080' in row['url'] for row in result['observations'])
        else:
            labels = {row['case'] for row in result['negatives']}
            assert {'invalid-token','expired-token','provider-reject','malformed','active-cancel','tampered','root-tamper','unsupported-root','unsupported-assertion','untrusted','policy-denied'} <= labels
            assert {(row['wrap'],row['session'],row['auth']) for row in result['negatives'] if row['case']=='policy-denied'} == set(combinations)
            assert result['snapshot']['presence'] and result['snapshot']['payload'] == [0,255,1,7] and result['snapshot']['metadata'] == [128,255,0]
            if profile == 'dpop':
                assert {row['auth'] for row in result['negatives'] if row['case']=='mismatched-provider-auth-key'} == {'ES256','RS256'}
                assert any(row['case']=='actual-accepted-proof-replay' and row['status']==401 for row in result['negatives'])
                assert any(row['status']==401 and row['nonceReadable'] and '8080' in row['url'] for row in result['transport'])
    return {'target':target,'profile':profile,'mode':mode,'result_sha256':sha(path),
            'required_comparisons':[list(key) for key in sorted(expected)],'observed_comparisons':len(observed),
            'invocation_count':len(events),'missing':[],'status':0}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode',choices=('focused','full'))
    parser.add_argument('--target',choices=(*TARGETS,'all'),required=True)
    parser.add_argument('--profiles',nargs='+',choices=('basic','ec','dpop'),required=True)
    parser.add_argument('--packages-base',type=Path,required=True)
    parser.add_argument('--output-base',type=Path,required=True)
    parser.add_argument('--receipt',type=Path,required=True)
    args = parser.parse_args()
    targets = TARGETS if args.target == 'all' else (args.target,)
    checks = []
    for target in targets:
        for kind in ('packages','consumers'):
            path = args.packages_base/kind/target/'receipt.json'
            data = json.loads(path.read_text())
            assert data['status'] == 0, str(path)
            if kind == 'packages':
                assert data['repeatable'] and data['deliverable_members'] == data['second_deliverable_members']
                assert all(sha(path.parent/'relative'/name) == expected for name,expected in data['deliverable_members'].items()), 'distributed member drift'
            else:
                installed_members = data['installed_package_members']
                if target == 'rust':
                    # Retained first receipts inventoried an unused package/
                    # directory. The immutable crate is the source oracle for
                    # that historical receipt and for every future installation.
                    crate = args.packages_base/'packages/rust/relative/target/package/opentdf-tdf3-0.1.0.crate'
                    with tarfile.open(crate) as archive:
                        expected_members = {'installed/'+member.name:hashlib.sha256(archive.extractfile(member).read()).hexdigest() for member in archive.getmembers() if member.isfile()}
                    assert expected_members
                    if installed_members:
                        assert installed_members == expected_members, 'Rust installed inventory differs from immutable crate'
                    installed_members = expected_members
                assert installed_members, 'empty installed package inventory'
                assert all(sha(path.parent/name) == expected for name,expected in installed_members.items()), 'installed package member drift'
        for profile in args.profiles:
            if args.mode == 'focused' and profile != 'basic' and target not in ('go','typescript'):
                continue
            output = args.output_base/target/profile if args.mode == 'focused' else SDK/'.local'/(target+'-tdf-library')/profile
            checks.append(check(target,profile,args.mode,output))
    if 'typescript' in targets:
        for profile in args.profiles:
            output = args.output_base/'typescript-browser'/profile if args.mode == 'focused' else SDK/'.local/typescript-tdf-library'/('browser-'+profile)
            checks.append(check('typescript-browser',profile,args.mode,output))
    receipt = {'status':0,'mode':args.mode,'scope':'exact actual '+args.mode+' executable coverage; accepted historical full evidence is separate',
               'checks':checks,'missing_required_cases':[],'full_matrix_executed':args.mode=='full'}
    args.receipt.parent.mkdir(parents=True,exist_ok=True)
    args.receipt.write_text(json.dumps(receipt,indent=2)+'\n')
    print('PASS exact',args.mode,'coverage',len(checks),'target/profile environments')


if __name__ == '__main__':
    main()
