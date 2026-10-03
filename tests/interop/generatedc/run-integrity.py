#!/usr/bin/env python3
"""Artifact-scoped integrity negatives under the live profile; no producer replay."""
from pathlib import Path
import hashlib,io,json,subprocess,sys,zipfile
sdk=Path(__file__).resolve().parents[3]
stage=sys.argv[1];assert stage in ('ec','dpop')
base=sdk/'.local/c-tdf-library';run=base/(stage+'-integrity');run.mkdir(exist_ok=False)
rows=[]
for wrapping in ('rsa:2048','ec:secp256r1'):
 for auth in ('ES256','RS256'):
  source=base/stage/(wrapping[:2]+'-'+auth.lower()+'-binary.generated.tdf')
  archive=source.read_bytes()
  with zipfile.ZipFile(io.BytesIO(archive)) as z:original={n:z.read(n) for n in z.namelist()}
  manifest_name='manifest.json' if 'manifest.json' in original else '0.manifest.json'
  for session in ('rsa:2048','ec:secp256r1'):
   config={'PlatformURL':'http://localhost:8080','KASURL':'http://localhost:8080/kas','IssuerURL':'http://localhost:8888/auth/realms/opentdf','ClientID':'opentdf-sdk','ClientSecret':'secret','AllowHTTP':True,'KASAlgorithm':wrapping,'SessionAlgorithm':session,'AuthAlgorithm':auth,'AuthPrivateKeyPEM':(base/('auth-'+auth+'.pem')).read_text(),'DPoP':stage=='dpop','AllowedKAS':[{'URL':'http://localhost:8080/kas','APIBaseURL':'http://localhost:8080'}],'KASPublicKeyPEM':(base/'ec'/(wrapping[:2]+'-public.pem')).read_text(),'KID':'profile-r1' if wrapping.startswith('rsa') else 'profile-e1'}
   (run/'config.json').write_text(json.dumps(config));(run/'config.json').chmod(0o600)
   for mutation in ('ciphertext','root'):
    name=wrapping[:2]+'-'+auth.lower()+'-'+session[:2]+'-'+mutation
    entries=dict(original)
    if mutation=='ciphertext':
     payload=bytearray(entries['0.payload']);payload[12]^=1;entries['0.payload']=payload
    else:
     manifest=json.loads(entries[manifest_name]);sig=manifest['encryptionInformation']['integrityInformation']['rootSignature']['sig'];manifest['encryptionInformation']['integrityInformation']['rootSignature']['sig']=('A' if sig[0]!='A' else 'B')+sig[1:];entries[manifest_name]=json.dumps(manifest).encode()
    destination=run/(name+'.tdf')
    with zipfile.ZipFile(destination,'w',compression=zipfile.ZIP_STORED) as z:
     for n,b in entries.items():z.writestr(n,b)
    result=subprocess.run([str(sdk/'tests/interop/generatedc/consumer.sh'),str(sdk),str(run),'negative',name],stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=120)
    (run/(name+'.status')).write_text(str(result.returncode)+'\n');(run/(name+'.log')).write_bytes(result.stdout+result.stderr)
    assert not result.returncode,name
    error=json.loads((run/(name+'.error.json')).read_text());assert error['kind']==1 and error['code']=='integrity',error
    rows.append({'profile':stage,'wrapping':wrapping,'session':session,'auth':auth,'mutation':mutation,'source_archive':str(source.relative_to(sdk)),'source_sha256':hashlib.sha256(archive).hexdigest(),'mutated_sha256':hashlib.sha256(destination.read_bytes()).hexdigest(),'error':error,'status':0,'zero_plaintext':True})
    print('PASS native importing integrity',stage,name,flush=True)
(run/'results.json').write_text(json.dumps({'scope':'narrow retained-artifact integrity supplement, no producer/matrix replay','cases':rows},indent=2)+'\n')
