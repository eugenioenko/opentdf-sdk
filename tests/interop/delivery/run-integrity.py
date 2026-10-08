#!/usr/bin/env python3
"""Full-mode profile integrity supplement using retained real-KAS archives."""
from pathlib import Path
import argparse
import hashlib
import io
import json
import os
import subprocess
import zipfile
from delivery_capture import capture

SDK = Path(__file__).resolve().parents[3]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('target',choices=('go','typescript','java','csharp','python','rust','c','swift'))
    parser.add_argument('profile',choices=('ec','dpop'))
    parser.add_argument('--packages-base',type=Path,required=True)
    args = parser.parse_args()
    base = SDK/'.local'/(args.target+'-tdf-library')
    run = base/(args.profile+'-delivery-integrity')
    run.mkdir(exist_ok=False)
    installed = json.loads((args.packages_base/'consumers'/args.target/'receipt.json').read_text())
    environment = os.environ.copy()
    environment.update(installed['environment'])
    rows = []
    for wrap in ('rsa:2048','ec:secp256r1'):
        for auth in ('ES256','RS256'):
            source = base/args.profile/(wrap[:2]+'-'+auth.lower()+'-binary.generated.tdf')
            raw = source.read_bytes()
            with zipfile.ZipFile(io.BytesIO(raw)) as archive:
                original = {name:archive.read(name) for name in archive.namelist()}
            manifest_name = 'manifest.json' if 'manifest.json' in original else '0.manifest.json'
            for session in ('rsa:2048','ec:secp256r1'):
                config = {'PlatformURL':'http://localhost:8080','KASURL':'http://localhost:8080/kas','IssuerURL':'http://localhost:8888/auth/realms/opentdf','ClientID':'opentdf-sdk','ClientSecret':'secret','AllowHTTP':True,'KASAlgorithm':wrap,'SessionAlgorithm':session,'AuthAlgorithm':auth,'AuthPrivateKeyPEM':(base/('auth-'+auth+'.pem')).read_text(),'DPoP':args.profile=='dpop','AllowedKAS':[{'URL':'http://localhost:8080/kas','APIBaseURL':'http://localhost:8080'}],'KASPublicKeyPEM':(base/'ec'/(wrap[:2]+'-public.pem')).read_text(),'KID':'profile-r1' if wrap.startswith('rsa') else 'profile-e1'}
                (run/'config.json').write_text(json.dumps(config));(run/'config.json').chmod(0o600)
                for mutation in ('ciphertext','root'):
                    name = wrap[:2]+'-'+auth.lower()+'-'+session[:2]+'-'+mutation
                    entries = dict(original)
                    if mutation == 'ciphertext':
                        value = bytearray(entries['0.payload']);value[12] ^= 1;entries['0.payload'] = value
                    else:
                        manifest = json.loads(entries[manifest_name]);signature = manifest['encryptionInformation']['integrityInformation']['rootSignature']['sig'];manifest['encryptionInformation']['integrityInformation']['rootSignature']['sig'] = ('A' if signature[0]!='A' else 'B')+signature[1:];entries[manifest_name] = json.dumps(manifest).encode()
                    destination = run/(name+'.tdf')
                    with zipfile.ZipFile(destination,'w',compression=zipfile.ZIP_STORED) as archive:
                        for key,value in entries.items():archive.writestr(key,value)
                    command = [*installed['consumer_command'],SDK,run,'negative',name]
                    result = subprocess.run([str(value) for value in command],env=environment,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=120)
                    event = capture(name,command,result,run)
                    assert result.returncode == 0
                    error = json.loads((run/(name+'.error.json')).read_text());assert error['code'] == 'integrity'
                    assert not (run/(name+'.out')).exists()
                    rows.append({'profile':args.profile,'wrapping':wrap,'session':session,'auth':auth,'mutation':mutation,'source_archive_sha256':hashlib.sha256(raw).hexdigest(),'mutated_sha256':hashlib.sha256(destination.read_bytes()).hexdigest(),'error':error,'status':0,'zero_plaintext':True,'invocation':event['sequence']})
    (run/'results.json').write_text(json.dumps({'scope':'retained archive integrity supplement; no producer replay','cases':rows,'status':0},indent=2)+'\n')


if __name__ == '__main__':
    main()
