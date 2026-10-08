#!/usr/bin/env python3
"""Bounded final-package KAS smoke or new explicit-empty-metadata coverage.

Historical accepted full matrices are reused by source correspondence. This is
deliberately recorded as focused execution, never as a full CI matrix replay.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import shutil
import subprocess
import urllib.request
import urllib.parse
import zipfile
from delivery_capture import capture

SDK = Path(__file__).resolve().parents[3]


def digest(value):
    return hashlib.sha256(value).hexdigest()


def metadata_presence(path):
    with zipfile.ZipFile(path) as archive:
        manifest = json.loads(archive.read('manifest.json' if 'manifest.json' in archive.namelist() else '0.manifest.json'))
    return any(bool(key.get('encryptedMetadata')) for key in manifest['encryptionInformation']['keyAccess'])


def run(target, profile, packages_base, output_base):
    output = output_base/target/profile
    output.mkdir(parents=True, exist_ok=False)
    consumer_receipt = packages_base/'consumers'/target/'receipt.json'
    installed = json.loads(consumer_receipt.read_text())
    command = installed['consumer_command']
    environment = os.environ.copy()
    environment.update(installed['environment'])
    reference = SDK/'.local/go-tdf-library/stock-go'
    rows = []

    def invoke(label, args):
        result = subprocess.run([str(a) for a in args], cwd=SDK, env=environment, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=120)
        event = capture(label, args, result, output)
        (output/(label+'.status')).write_text(str(result.returncode)+'\n')
        if result.returncode:
            # Reference diagnostics can include credentials; retain hashes only.
            (output/(label+'.diagnostic-sha256.json')).write_text(json.dumps({'stdout': digest(result.stdout), 'stderr': digest(result.stderr)})+'\n')
            raise RuntimeError(label+' failed '+str(result.returncode)+'; event '+str(event['sequence']))
        return event

    def native(mode, name):
        return invoke(mode+'-'+name, [*command, SDK, output, mode, name])

    config = {'PlatformURL':'http://localhost:8080', 'KASURL':'http://localhost:8080/kas',
              'IssuerURL':'http://localhost:8888/auth/realms/opentdf', 'ClientID':'opentdf-sdk',
              'ClientSecret':'secret', 'AllowHTTP':True, 'DPoP':profile == 'dpop',
              'AllowedKAS':[{'URL':'http://localhost:8080/kas','APIBaseURL':'http://localhost:8080'}]}
    wraps = ('rsa:2048',) if profile == 'basic' else ('rsa:2048', 'ec:secp256r1')
    auths = ('ES256',) if profile == 'basic' else ('ES256', 'RS256')
    sessions = ('rsa:2048',) if profile == 'basic' else ('rsa:2048', 'ec:secp256r1')
    case = 'binary' if profile == 'basic' else 'empty-metadata'
    payload = bytes(range(256))*3+b'\x00\xff' if case == 'binary' else b'empty metadata'
    keys = output_base/'keys'
    keys.mkdir(exist_ok=True)
    if profile != 'basic':
        for auth in auths:
            key = keys/('auth-'+auth+'.pem')
            if not key.exists():
                args = ['openssl','genpkey','-algorithm','EC','-pkeyopt','ec_paramgen_curve:P-256'] if auth == 'ES256' else ['openssl','genpkey','-algorithm','RSA','-pkeyopt','rsa_keygen_bits:2048']
                invoke('auth-key-'+auth, args+['-out', key])
                key.chmod(0o600)
    for wrap in wraps:
        if profile == 'ec':
            form = urllib.parse.urlencode({'grant_type':'client_credentials','client_id':'opentdf-sdk','client_secret':'secret'}).encode()
            with urllib.request.urlopen(urllib.request.Request(config['IssuerURL']+'/protocol/openid-connect/token', data=form, headers={'Content-Type':'application/x-www-form-urlencoded'}), timeout=15) as response:
                token = json.load(response)['access_token']
            request = urllib.request.Request('http://localhost:8080/kas.AccessService/PublicKey', data=json.dumps({'algorithm':wrap,'fmt':'pkcs8','v':'2'}).encode(), headers={'Authorization':'Bearer '+token,'Content-Type':'application/json','Connect-Protocol-Version':'1'})
            with urllib.request.urlopen(request, timeout=15) as response:
                public = json.load(response)
            assert public['kid'] == ('profile-r1' if wrap.startswith('rsa') else 'profile-e1')
            (keys/(wrap[:2]+'-public.pem')).write_text(public['publicKey'])
        producer = wrap[:2]+'-'+case
        (output/(producer+'.input')).write_bytes(payload)
        if profile != 'dpop':
            invoke('stock-go-encrypt-'+producer, [reference, output, 'encrypt', producer, wrap])
            fixture = SDK/'tests/interop'/('client/web-fixture.mjs' if profile == 'basic' else 'generatedlibrary/web-fixture.mjs')
            args = ['node', fixture, SDK, output/(producer+'.input'), output/(producer+'.web.tdf'), 'GMAC', '', 'https://example.com/attr/attr1/value/value1']
            if profile == 'ec':
                args += [wrap, 'profile-r1' if wrap.startswith('rsa') else 'profile-e1', keys/(wrap[:2]+'-public.pem')]
            invoke('stock-web-encrypt-'+producer, args)
        else:
            for source in ('go', 'web'):
                shutil.copyfile(output_base/target/'ec'/(producer+'.'+source+'.tdf'), output/(producer+'.'+source+'.tdf'))
        for auth in auths:
            for session in sessions:
                name = wrap[:2]+'-'+auth.lower()+'-'+session[:2]+'-'+case
                cfg = {**config, 'KASAlgorithm':wrap, 'SessionAlgorithm':session, 'AuthAlgorithm':auth}
                if profile != 'basic':
                    cfg.update(KASPublicKeyPEM=(keys/(wrap[:2]+'-public.pem')).read_text(),
                               KID='profile-r1' if wrap.startswith('rsa') else 'profile-e1',
                               AuthPrivateKeyPEM=(keys/('auth-'+auth+'.pem')).read_text())
                (output/'config.json').write_text(json.dumps(cfg))
                (output/'config.json').chmod(0o600)
                (output/(name+'.input')).write_bytes(payload)
                native('encrypt', name)
                assert metadata_presence(output/(name+'.generated.tdf')) == (case == 'empty-metadata')
                event = invoke('stock-go-decrypt-'+name, [reference, output, 'decrypt', name, session])
                assert (output/(name+'.stock-go.out')).read_bytes() == payload
                assert (output/(name+'.stock-go.metadata')).read_bytes() == b''
                rows.append({'producer':target,'consumer':'stock-go','case':case,'wrap':wrap,'session':session,'auth':auth,'invocation':event['sequence']})
                if profile != 'dpop':
                    path = output/(name+'.stock-web.out')
                    event = invoke('stock-web-decrypt-'+name, ['node', SDK/'.local/web-cli/bin/opentdf.mjs', 'decrypt', output/(name+'.generated.tdf'), '--rewrapKeyType', session, '--allowList', 'http://localhost:8080', '--output', path, '--platformUrl', 'http://localhost:8080', '--kasEndpoint', 'http://localhost:8080/kas', '--oidcEndpoint', config['IssuerURL'], '--clientId', 'opentdf-sdk', '--clientSecret', 'secret', '--logLevel', 'error'])
                    assert path.read_bytes() == payload
                    rows.append({'producer':target,'consumer':'stock-web','case':case,'wrap':wrap,'session':session,'auth':auth,'invocation':event['sequence'],'metadata_reader_limitation':True})
                for source in ('go', 'web', 'generated'):
                    label = name+'.'+source
                    archive = output/((name if source == 'generated' else producer)+'.'+source+'.tdf')
                    if archive != output/(label+'.tdf'):
                        shutil.copyfile(archive, output/(label+'.tdf'))
                    event = native('decrypt', label)
                    assert (output/(label+'.out')).read_bytes() == payload
                    assert (output/(label+'.metadata')).read_bytes() == b''
                    presence = metadata_presence(archive)
                    assert (output/(label+'.presence')).read_text().strip() == str(presence).lower()
                    rows.append({'producer':source+('-format-under-bearer' if profile == 'dpop' and source != 'generated' else ''),'consumer':target,'case':case,'wrap':wrap,'session':session,'auth':auth,'has_metadata':presence,'invocation':event['sequence']})
    if profile == 'basic':
        native('repeat', 'ownership')
        (output/'malformed.tdf').write_bytes(b'bad archive')
        native('negative', 'malformed')
        assert json.loads((output/'malformed.error.json').read_text())['code'] == 'archive'
        if target == 'c':
            no_preinit = packages_base/'consumers/c/no-preinit-consumer'
            invoke('public-no-preinit-real-kas', [no_preinit, output/'rs-binary.go.tdf', 'http://localhost:8080', output/'no-preinit.out', output/'no-preinit.metadata', output/'no-preinit.presence'])
            assert (output/'no-preinit.out').read_bytes() == payload
    receipt = {'target':target,'profile':profile,'scope':'actual focused final installed native package execution; historical full matrices remain separately attributed',
               'pairs':rows,'payload_sha256':digest(payload),'metadata_sha256':digest(b''),'status':0,
               'consumer_receipt_sha256':digest(consumer_receipt.read_bytes()),
               'package_receipt_sha256':digest((packages_base/'packages'/target/'receipt.json').read_bytes()),
               'known_stock_web_dpop_limits':'historical accepted four nonce401 cases; no successful stock-Web enforced-auth pair claimed' if profile == 'dpop' else None}
    (output/'results.json').write_text(json.dumps(receipt, indent=2)+'\n')
    print('PASS focused final package',target,profile,len(rows),'comparisons',flush=True)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('target', choices=('go','typescript','java','csharp','python','rust','c','swift'))
    parser.add_argument('profile', choices=('basic','ec','dpop'))
    parser.add_argument('--packages-base', type=Path, required=True)
    parser.add_argument('--output-base', type=Path, required=True)
    args = parser.parse_args()
    run(args.target,args.profile,args.packages_base.resolve(),args.output_base.resolve())
