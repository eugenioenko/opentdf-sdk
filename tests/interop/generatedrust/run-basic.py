#!/usr/bin/env python3
"""Real basic KAS matrix for a separately built importing Rust consumer."""
from pathlib import Path
import hashlib,json,subprocess,sys,zipfile,io,copy
sdk=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(sdk/'tests/interop/delivery'))
from delivery_capture import capture,assert_metadata_presence
run=sdk/(sys.argv[1] if len(sys.argv)>1 else '.local/rust-tdf-library/basic')
run.mkdir(parents=True,exist_ok=True)
consumer=sdk/'tests/interop/generatedrust/consumer.sh'
reference=sdk/'.local/go-tdf-library/stock-go'
config={'PlatformURL':'http://localhost:8080','KASURL':'http://localhost:8080/kas','IssuerURL':'http://localhost:8888/auth/realms/opentdf','ClientID':'opentdf-sdk','ClientSecret':'secret','AllowHTTP':True}
(run/'config.json').write_text(json.dumps(config));(run/'config.json').chmod(0o600)
# Verify the pinned, unmodified independent references before using their artifacts.
lock=json.loads((sdk/'references.lock.json').read_text())
for name in ('platform','web-sdk'):
    pin=lock['repositories'][name];repo=sdk/pin['path']
    revision=subprocess.check_output(['git','-C',str(repo),'rev-parse','HEAD'],text=True).strip()
    assert revision==pin['revision']
    assert not subprocess.check_output(['git','-C',str(repo),'status','--porcelain','--untracked-files=no'])
rows=[]
def invoke(label,args):
    # Do not persist unsanitized reference diagnostics containing possible tokens.
    result=subprocess.run([str(a) for a in args],cwd=sdk,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=120)
    capture(label,args,result,run)
    (run/(label+'.status')).write_text(str(result.returncode)+'\n')
    if result.returncode:
        if args[0]==consumer:(run/(label+'.log')).write_bytes(result.stdout+result.stderr)
        raise RuntimeError(label+' failed; status='+str(result.returncode))
def new(mode,name):
    invoke(mode+'-'+name,[consumer,sdk,run,mode,name])
    if mode=='decrypt':assert_metadata_presence(run,name)
def compare(name,producer,expected,metadata):
    new('decrypt',name+'.'+producer)
    assert (run/(name+'.'+producer+'.out')).read_bytes()==expected
    assert (run/(name+'.'+producer+'.metadata')).read_bytes()==metadata
    with zipfile.ZipFile(run/(name+'.'+producer+'.tdf')) as z:
        manifest=json.loads(z.read('manifest.json' if 'manifest.json' in z.namelist() else '0.manifest.json'))
    presence=any(bool(ka.get('encryptedMetadata')) for ka in manifest['encryptionInformation']['keyAccess'])
    assert (run/(name+'.'+producer+'.presence')).read_text()==str(presence).lower()
    rows.append({'producer':producer,'consumer':'generated-rust','case':name,'has_metadata':presence,'bytes':len(expected),'payload_sha256':hashlib.sha256(expected).hexdigest(),'metadata_sha256':hashlib.sha256(metadata).hexdigest()})
cases={'empty':b'','binary':bytes(range(256))*3+b'\x00\xff','exact':bytes([0,255,128,7])*4096,'multiple':bytes([0,255,128,7])*8193,'hs256':bytes([0,255,128,7])*8193,'metadata':b'metadata case','empty-metadata':b'empty metadata'}
for name,data in cases.items():
    (run/(name+'.input')).write_bytes(data)
    new('encrypt',name)
    metadata=b'{"source":"independent metadata","count":7}' if name=='metadata' else b''
    invoke('stock-go-decrypt-'+name,[reference,run,'decrypt',name,'rsa:2048'])
    assert (run/(name+'.stock-go.out')).read_bytes()==data
    assert (run/(name+'.stock-go.metadata')).read_bytes()==metadata
    rows.append({'producer':'generated-rust','consumer':'stock-go','case':name,'bytes':len(data),'payload_sha256':hashlib.sha256(data).hexdigest(),'metadata_sha256':hashlib.sha256(metadata).hexdigest()})
    invoke('stock-web-decrypt-'+name,['node',sdk/'.local/web-cli/bin/opentdf.mjs','decrypt',run/(name+'.generated.tdf'),'--rewrapKeyType','rsa:2048','--allowList','http://localhost:8080','--output',run/(name+'.stock-web.out'),'--platformUrl','http://localhost:8080','--kasEndpoint','http://localhost:8080/kas','--oidcEndpoint','http://localhost:8888/auth/realms/opentdf','--clientId','opentdf-sdk','--clientSecret','secret','--logLevel','error'])
    assert (run/(name+'.stock-web.out')).read_bytes()==data
    rows.append({'producer':'generated-rust','consumer':'stock-web','case':name,'bytes':len(data),'payload_sha256':hashlib.sha256(data).hexdigest(),'metadata_check':'stock Web reader does not expose decrypted metadata'})
    compare(name,'generated',data,metadata)
    assert (run/(name+'.generated.presence')).read_text()==('true' if name in ('metadata','empty-metadata') else 'false')
    invoke('stock-go-encrypt-'+name,[reference,run,'encrypt',name,'rsa:2048'])
    compare(name,'go',data,metadata)
    invoke('stock-web-encrypt-'+name,['node',sdk/'tests/interop/client/web-fixture.mjs',sdk,run/(name+'.input'),run/(name+'.web.tdf'),'HS256' if name=='hs256' else 'GMAC',metadata.decode() if metadata else '', 'https://example.com/attr/attr1/value/value1'])
    compare(name,'web',data,metadata)
    print('PASS real basic-KAS importing library and both stock references',name,flush=True)
# Host provider uses the generated typed API bridge and real OAuth endpoint.
config['TokenProviderName']='access-token';config['ClientID']='';config['ClientSecret']=''
(run/'config.json').write_text(json.dumps(config));new('decrypt','binary.go')
assert (run/'binary.go.out').read_bytes()==cases['binary'];print('PASS real native token provider',flush=True)
config['TokenProviderName']='';config['ClientID']='opentdf-sdk';config['ClientSecret']='secret'
(run/'config.json').write_text(json.dumps(config))
new('repeat','ownership')
# SDK failures must have useful typed fields and no plaintext.
(run/'denied.input').write_bytes(b'denied');new('encrypt','denied');new('negative','denied.generated')
error=json.loads((run/'denied.generated.error.json').read_text());assert error['code']=='rewrap_failed' and error['operation']=='rewrap' and error['httpStatus']==0
archive=(run/'binary.generated.tdf').read_bytes()
with zipfile.ZipFile(io.BytesIO(archive)) as z: entries={n:z.read(n) for n in z.namelist()}
payload=bytearray(entries['0.payload']);payload[12]^=1;entries['0.payload']=payload
with zipfile.ZipFile(run/'tampered.tdf','w',compression=zipfile.ZIP_STORED) as z:
    for n,b in entries.items():z.writestr(n,b)
new('negative','tampered');assert json.loads((run/'tampered.error.json').read_text())['code']=='integrity'
(run/'malformed.tdf').write_bytes(b'bad archive');new('negative','malformed');assert json.loads((run/'malformed.error.json').read_text())['code']=='archive'
# Authenticated root signature tampering and unsupported mandatory features fail.
with zipfile.ZipFile(io.BytesIO(archive)) as z: original={n:z.read(n) for n in z.namelist()}
manifestName='manifest.json' if 'manifest.json' in original else '0.manifest.json'
for label,kind in [('root-tamper','root'),('unsupported-root','algorithm'),('unsupported-assertion','assertion'),('policy-binding-tamper','binding')]:
    entries=dict(original);manifest=json.loads(entries[manifestName])
    integrity=manifest['encryptionInformation']['integrityInformation']
    if kind=='root':
        sig=integrity['rootSignature']['sig'];integrity['rootSignature']['sig']=('A' if sig[0]!='A' else 'B')+sig[1:]
    elif kind=='algorithm':integrity['rootSignature']['alg']='GMAC'
    elif kind=='binding':
        value=manifest['encryptionInformation']['keyAccess'][0]['policyBinding']['hash'];decoded=bytearray(__import__('base64').b64decode(value));decoded[0]=ord('1') if decoded[0]!=ord('1') else ord('2');manifest['encryptionInformation']['keyAccess'][0]['policyBinding']['hash']=__import__('base64').b64encode(decoded).decode()
    else:manifest['assertions']=[{'id':'mandatory'}]
    entries[manifestName]=json.dumps(manifest).encode()
    with zipfile.ZipFile(run/(label+'.tdf'),'w',compression=zipfile.ZIP_STORED) as z:
        for n,b in entries.items():z.writestr(n,b)
    new('negative',label);error=json.loads((run/(label+'.error.json')).read_text())
    assert error['code'] in ('rewrap_failed','integrity') if kind=='binding' else error['code']==('integrity' if kind=='root' else 'unsupported_manifest')
# Invalid provider tokens exercise native typed fields without exposing tokens.
config['TokenProviderName']='access-token';config['ClientID']='';config['ClientSecret']=''
(run/'config.json').write_text(json.dumps(config))
for label,code,status in [('invalid-token','unauthenticated',401),('expired-token','invalid_token',0),('provider-reject','token_acquisition',0)]:
    (run/(label+'.tdf')).write_bytes(archive);new('negative',label)
    error=json.loads((run/(label+'.error.json')).read_text());assert error['code']==code and error['httpStatus']==status
config['TokenProviderName']='';config['ClientID']='opentdf-sdk';config['ClientSecret']='secret';config['AllowedKAS']=[{'URL':'http://localhost:8080/kas','APIBaseURL':'http://localhost:8080'}]
(run/'config.json').write_text(json.dumps(config))
# Trusted routing failure precedes token acquisition.
with zipfile.ZipFile(io.BytesIO(archive)) as z: entries={n:z.read(n) for n in z.namelist()}
manifestName='manifest.json' if 'manifest.json' in entries else '0.manifest.json'
manifest=json.loads(entries[manifestName]);manifest['encryptionInformation']['keyAccess'][0]['url']='http://localhost:8081/kas';entries[manifestName]=json.dumps(manifest).encode()
with zipfile.ZipFile(run/'untrusted.tdf','w',compression=zipfile.ZIP_STORED) as z:
    for n,b in entries.items():z.writestr(n,b)
new('negative','untrusted');assert json.loads((run/'untrusted.error.json').read_text())['code']=='kas_not_allowed'
(run/'results.json').write_text(json.dumps({'profile':'basic','scope':'actual importable generated Rust library; profile/negative expansions still required','pairs':rows,'native_provider':True,'typed_negatives':['grouped-denial','integrity','archive','unauthenticated401','expired-token','provider-rejection','untrusted-route','root-tampering','unsupported-root','unsupported-assertion','policy-binding-tamper']},indent=2)+'\n')
print('PASS basic generated Rust library matrix:',len(rows),'comparisons',flush=True)
