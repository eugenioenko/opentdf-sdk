#!/usr/bin/env python3
"""TypeScript Node generated library real-KAS profile expansion; root owns profile switches."""
from pathlib import Path
import json,subprocess,hashlib,sys,io,zipfile,urllib.request,urllib.parse,shutil
sdk=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(sdk/'tests/interop/delivery'))
from delivery_capture import capture,assert_metadata_presence
stage=sys.argv[1]
assert stage in ('ec','dpop')
base=sdk/'.local/typescript-tdf-library'
run=base/stage;run.mkdir(parents=True,exist_ok=True)
consumer=sdk/'tests/interop/generatedtypescript/consumer.sh';reference=sdk/'.local/go-tdf-library/stock-go'
rows=[];limitations=[];negative=[]
cases={'empty':b'','binary':bytes(range(256))*3+b'\x00\xff','exact':bytes([0,255,128,7])*4096,'multiple':bytes([0,255,128,7])*8193,'metadata':b'metadata bytes','hs256':bytes([0,255,128,7])*8193,'empty-metadata':b'empty metadata'}
def invoke(label,args,required=True):
    result=subprocess.run([str(a) for a in args],cwd=sdk,stdout=subprocess.PIPE,stderr=subprocess.PIPE,timeout=120)
    capture(label,args,result,run)
    (run/(label+'.status')).write_text(str(result.returncode)+'\n')
    if result.returncode and required:raise RuntimeError(label+' failed; status='+str(result.returncode))
    return result
def new(mode,name):
    invoke(mode+'-'+name,[consumer,sdk,run,mode,name])
    if mode=='decrypt':assert_metadata_presence(run,name)
def configure(wrapping,session='rsa:2048',auth='ES256',explicit=True):
    prefix=wrapping[:2]
    cfg={'PlatformURL':'http://localhost:8080','KASURL':'http://localhost:8080/kas','IssuerURL':'http://localhost:8888/auth/realms/opentdf','ClientID':'opentdf-sdk','ClientSecret':'secret','AllowHTTP':True,'KASAlgorithm':wrapping,'SessionAlgorithm':session,'AuthAlgorithm':auth,'DPoP':stage=='dpop','AllowedKAS':[{'URL':'http://localhost:8080/kas','APIBaseURL':'http://localhost:8080'}]}
    if explicit:cfg.update(KASPublicKeyPEM=(base/'ec'/(prefix+'-public.pem')).read_text(),KID='profile-r1' if prefix=='rs' else 'profile-e1')
    # Explicit imported auth keys prove per-operation ownership and algorithms.
    cfg['AuthPrivateKeyPEM']=(base/('auth-'+auth+'.pem')).read_text()
    (run/'config.json').write_text(json.dumps(cfg));(run/'config.json').chmod(0o600)
    return cfg
if stage=='ec':
    # Public discovery uses the real trusted endpoint; private keys never leave ignored files.
    form=urllib.parse.urlencode({'grant_type':'client_credentials','client_id':'opentdf-sdk','client_secret':'secret'}).encode()
    with urllib.request.urlopen(urllib.request.Request('http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token',data=form,headers={'Content-Type':'application/x-www-form-urlencoded'}),timeout=15) as r:token=json.load(r)['access_token']
    for wrapping in ('rsa:2048','ec:secp256r1'):
        request=urllib.request.Request('http://localhost:8080/kas.AccessService/PublicKey',data=json.dumps({'algorithm':wrapping,'fmt':'pkcs8','v':'2'}).encode(),headers={'Authorization':'Bearer '+token,'Content-Type':'application/json','Connect-Protocol-Version':'1'})
        with urllib.request.urlopen(request,timeout=15) as r:key=json.load(r)
        assert key['kid']==('profile-r1' if wrapping.startswith('rsa') else 'profile-e1')
        (run/(wrapping[:2]+'-public.pem')).write_text(key['publicKey'])
    for auth in ('ES256','RS256'):
        key=base/('auth-'+auth+'.pem')
        if not key.exists():
            args=['openssl','genpkey','-algorithm','EC','-pkeyopt','ec_paramgen_curve:P-256'] if auth=='ES256' else ['openssl','genpkey','-algorithm','RSA','-pkeyopt','rsa_keygen_bits:2048']
            invoke('auth-key-'+auth,args+['-out',key]);key.chmod(0o600)
for wrapping in ('rsa:2048','ec:secp256r1'):
    prefix=wrapping[:2]
    # Fresh independent stock producers are created under Bearer. DPoP stage
    # reads those exact retained formats with generated enforced DPoP auth.
    if stage=='ec':
        for name,data in cases.items():
            label=prefix+'-'+name;(run/(label+'.input')).write_bytes(data)
            metadata='{"source":"independent metadata","count":7}' if name=='metadata' else ''
            invoke('stock-go-encrypt-'+label,[reference,run,'encrypt',label,wrapping])
            invoke('stock-web-encrypt-'+label,['node',sdk/'tests/interop/generatedlibrary/web-fixture.mjs',sdk,run/(label+'.input'),run/(label+'.web.tdf'),'HS256' if name=='hs256' else 'GMAC',metadata,'https://example.com/attr/attr1/value/value1',wrapping,'profile-r1' if prefix=='rs' else 'profile-e1',run/(prefix+'-public.pem')])
    else:
        for name,data in cases.items():
            label=prefix+'-'+name;(run/(label+'.input')).write_bytes(data)
            for producer in ('go','web'):shutil.copyfile(base/'ec'/(label+'.'+producer+'.tdf'),run/(label+'.'+producer+'.tdf'))
    for auth in ('ES256','RS256'):
        for name,data in cases.items():
            label=prefix+'-'+auth.lower()+'-'+name;(run/(label+'.input')).write_bytes(data)
            metadata=b'{"source":"independent metadata","count":7}' if name=='metadata' else b''
            configure(wrapping,auth=auth,explicit=name!='binary')
            new('encrypt',label)
            # Encrypted format algorithm/kid checks are independent of plaintext.
            with zipfile.ZipFile(run/(label+'.generated.tdf')) as z:
                m=json.loads(z.read('manifest.json' if 'manifest.json' in z.namelist() else '0.manifest.json'))
            ka=m['encryptionInformation']['keyAccess'][0]
            assert ka['type']==('wrapped' if prefix=='rs' else 'ec-wrapped') and ka['kid']==('profile-r1' if prefix=='rs' else 'profile-e1')
            for session in ('rsa:2048','ec:secp256r1'):
                configure(wrapping,session,auth)
                invoke('stock-go-decrypt-'+label+'-'+session[:2],[reference,run,'decrypt',label,session])
                assert (run/(label+'.stock-go.out')).read_bytes()==data and (run/(label+'.stock-go.metadata')).read_bytes()==metadata
                rows.append({'producer':'generated-typescript-node','consumer':'stock-go','case':name,'wrapping':wrapping,'session':session,'auth':auth,'profile':stage,'payload_sha256':hashlib.sha256(data).hexdigest(),'metadata_sha256':hashlib.sha256(metadata).hexdigest()})
                if stage=='ec':
                    out=run/(label+'.stock-web-'+session[:2]+'.out')
                    invoke('stock-web-decrypt-'+label+'-'+session[:2],['node',sdk/'.local/web-cli/bin/opentdf.mjs','decrypt',run/(label+'.generated.tdf'),'--rewrapKeyType',session,'--allowList','http://localhost:8080','--output',out,'--platformUrl','http://localhost:8080','--kasEndpoint','http://localhost:8080/kas','--oidcEndpoint','http://localhost:8888/auth/realms/opentdf','--clientId','opentdf-sdk','--clientSecret','secret','--logLevel','error'])
                    assert out.read_bytes()==data
                    rows.append({'producer':'generated-typescript-node','consumer':'stock-web','case':name,'wrapping':wrapping,'session':session,'auth':auth,'profile':stage,'metadata_check':'stock reader limitation'})
                # Independent producers + generated own artifact all decrypt
                # through the real KAS with each requested response session.
                for producer in ('go','web','generated'):
                    source=prefix+'-'+name+'.'+producer+'.tdf' if producer!='generated' else label+'.generated.tdf'
                    target=label+'-'+session[:2]+'.'+producer
                    shutil.copyfile(run/source,run/(target+'.tdf'));new('decrypt',target)
                    assert (run/(target+'.out')).read_bytes()==data and (run/(target+'.metadata')).read_bytes()==metadata
                    rows.append({'producer':('stock-'+producer+'-format-under-bearer' if stage=='dpop' and producer!='generated' else producer),'consumer':'generated-typescript-node','case':name,'wrapping':wrapping,'session':session,'auth':auth,'profile':stage,'payload_sha256':hashlib.sha256(data).hexdigest(),'metadata_sha256':hashlib.sha256(metadata).hexdigest()})
            print('PASS real generated',stage,wrapping,auth,name,'both sessions',flush=True)
        # Per profile/auth/wrapping: real denied policy and integrity rejection.
        name=prefix+'-'+auth.lower()+'-denied';(run/(name+'.input')).write_bytes(b'denied');configure(wrapping,auth=auth);new('encrypt',name)
        for session in ('rsa:2048','ec:secp256r1'):
            configure(wrapping,session,auth);new('negative',name+'.generated')
            error=json.loads((run/(name+'.generated.error.json')).read_text());assert error['code']=='rewrap_failed'
            negative.append({'case':'policy-denied','wrapping':wrapping,'session':session,'auth':auth})
        if stage=='dpop':
            # Retain stock Web's enforced-nonce authentication limitation as
            # an observed failure; never count it as successful stock interop.
            label=prefix+'-'+auth.lower()+'-binary';out=run/(label+'.stock-web.out')
            result=invoke('stock-web-enforced-'+label,['node',sdk/'.local/web-cli/bin/opentdf.mjs','decrypt',run/(label+'.generated.tdf'),'--rewrapKeyType','rsa:2048','--allowList','http://localhost:8080','--output',out,'--platformUrl','http://localhost:8080','--kasEndpoint','http://localhost:8080/kas','--oidcEndpoint','http://localhost:8888/auth/realms/opentdf','--clientId','opentdf-sdk','--clientSecret','secret','--logLevel','error'],required=False)
            assert result.returncode!=0
            # Inspect diagnostics in memory; persist the category, not token-bearing output.
            diagnostic=result.stderr+result.stdout;assert b'401' in diagnostic or b'Unauthenticated' in diagnostic or b'unauthenticated' in diagnostic
            limitations.append({'consumer':'stock-web','profile':'nonce-enforced-dpop','wrapping':wrapping,'auth':auth,'exit':result.returncode,'category':'authentication401','success':False})
if stage=='dpop':
    # Browser-style configuration carries an external provider and matching
    # explicit auth PEM with no source client credentials, for both signing algs.
    for auth in ('ES256','RS256'):
        cfg=configure('ec:secp256r1','ec:secp256r1',auth)
        cfg.update(TokenProviderName='access-token',ClientID='',ClientSecret='')
        (run/'config.json').write_text(json.dumps(cfg))
        source=run/('ec-'+auth.lower()+'-binary.generated.tdf')
        label='provider-'+auth.lower();shutil.copyfile(source,run/(label+'.tdf'));new('decrypt',label)
        assert (run/(label+'.out')).read_bytes()==cases['binary']
        rows.append({'producer':'generated-typescript-node','consumer':'generated-typescript-node-native-DPoP-provider','auth':auth,'wrapping':'ec:secp256r1','session':'ec:secp256r1','source_credentials':False})
        shutil.copyfile(source,run/'mismatched-provider.tdf');new('negative','mismatched-provider')
        error=json.loads((run/'mismatched-provider.error.json').read_text());assert error['code']=='token_binding'
        negative.append({'case':'mismatched-native-provider-auth-key','auth':auth})
(run/'results.json').write_text(json.dumps({'profile':stage,'scope':'actual importing generated library expanded against real KAS; stock Web enforced-auth limitation is separate','pairs':rows,'negatives':negative,'stock_web_limitations':limitations},indent=2)+'\n')
print('PASS generated profile',stage,'comparisons',len(rows),'negatives',len(negative),'stock-Web-limitations',len(limitations),flush=True)
