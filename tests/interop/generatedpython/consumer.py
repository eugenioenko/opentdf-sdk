"""Independent installed-package consumer; stock references are test runners only."""
import json,sys,time,urllib.request,urllib.parse,hashlib,base64,secrets,threading,asyncio
from pathlib import Path
from cryptography.hazmat.primitives import serialization,hashes
from cryptography.hazmat.primitives.asymmetric import rsa,ec,padding,utils
from opentdf_tdf3 import Config,KASRoute,EncryptOptions,AccessToken,TDFError,ProviderRejected,encrypt,decrypt,encrypt_async,decrypt_async
run=Path(sys.argv[2]);mode,name=sys.argv[3:5]
raw=json.loads((run/'config.json').read_text())
NAMES={'PlatformURL':'platform_url','KASURL':'kas_url','AllowedKAS':'allowed_kas','IssuerURL':'issuer_url','TokenURL':'token_url','ClientID':'client_id','ClientSecret':'client_secret','TokenProviderName':'token_provider_name','AllowHTTP':'allow_http','TimeoutMillis':'timeout_millis','KASPublicKeyPEM':'kas_public_key_pem','KID':'kid','KASAlgorithm':'kas_algorithm','SessionAlgorithm':'session_algorithm','AuthPrivateKeyPEM':'auth_private_key_pem','AuthAlgorithm':'auth_algorithm','DPoP':'dpop'}
def config_from(raw):
 values={NAMES[k]:v for k,v in raw.items()}
 if 'allowed_kas' in values:values['allowed_kas']=[KASRoute(r['URL'],r['APIBaseURL']) for r in values['allowed_kas']]
 return Config(**values)
config=config_from(raw)
def b64(v):return base64.urlsafe_b64encode(v).rstrip(b'=').decode()
def jsonbytes(v):return json.dumps(v,separators=(',',':'),ensure_ascii=False).encode()
def token(request):
 now=int(time.time())
 if name=='invalid-token':return AccessToken('invalid-token','Bearer',now+300)
 if name=='expired-token':return AccessToken('expired-token','Bearer',now-1)
 if name=='provider-reject':raise ProviderRejected('provider rejected')
 endpoint='http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token'
 nonce='';thumb='';key=None;jwk=None
 if config.dpop:
  key=ec.generate_private_key(ec.SECP256R1()) if name=='mismatched-provider' else serialization.load_pem_private_key(config.auth_private_key_pem.encode(),password=None)
  p=key.public_key().public_numbers()
  if isinstance(key,rsa.RSAPrivateKey):jwk={'e':b64(p.e.to_bytes((p.e.bit_length()+7)//8,'big')),'kty':'RSA','n':b64(p.n.to_bytes(256,'big'))}
  else:jwk={'crv':'P-256','kty':'EC','x':b64(p.x.to_bytes(32,'big')),'y':b64(p.y.to_bytes(32,'big'))}
  thumb=b64(hashlib.sha256(jsonbytes(jwk)).digest())
 for attempt in range(3):
  if request.canceled:raise ProviderRejected('provider stopped')
  headers={'Content-Type':'application/x-www-form-urlencoded'}
  if config.dpop:
   header={'typ':'dpop+jwt','alg':'RS256' if isinstance(key,rsa.RSAPrivateKey) else 'ES256','jwk':jwk}
   claims={'htu':endpoint,'htm':'POST','iat':int(time.time()),'jti':b64(secrets.token_bytes(16))}
   if nonce:claims['nonce']=nonce
   data=b64(jsonbytes(header))+'.'+b64(jsonbytes(claims));message=data.encode()
   if isinstance(key,rsa.RSAPrivateKey):sig=key.sign(message,padding.PKCS1v15(),hashes.SHA256())
   else:
    r,s=utils.decode_dss_signature(key.sign(message,ec.ECDSA(hashes.SHA256())));sig=r.to_bytes(32,'big')+s.to_bytes(32,'big')
   headers['DPoP']=data+'.'+b64(sig)
  req=urllib.request.Request(endpoint,data=urllib.parse.urlencode({'grant_type':'client_credentials','client_id':'opentdf-sdk','client_secret':'secret'}).encode(),headers=headers)
  try:response=urllib.request.urlopen(req,timeout=15)
  except urllib.error.HTTPError as error:response=error
  with response:
   body=response.read(128*1024+1);status=response.status;next_nonce=response.headers.get('DPoP-Nonce','')
  if status in (400,401) and next_nonce and next_nonce!=nonce:nonce=next_nonce;continue
  if status!=200 or len(body)>128*1024:raise ProviderRejected('provider status')
  value=json.loads(body);return AccessToken(value['access_token'],'DPoP' if config.dpop else 'Bearer',int(time.time())+int(value['expires_in']),thumb)
 raise ProviderRejected('provider nonce retries')
def provider():return token if config.token_provider_name else None
def opts(label):
 o=EncryptOptions(attributes=['https://example.com/attr/attr1/value/'+('value2' if label=='denied' or label.endswith('-denied') else 'value1')],segment_size=16384,has_segment_size=True)
 if label=='metadata' or label.endswith('-metadata'):o.metadata=b'{"source":"independent metadata","count":7}';o.include_metadata=True
 if label=='empty-metadata' or label.endswith('-empty-metadata'):o.metadata=b'';o.include_metadata=True
 if label=='hs256' or label.endswith('-hs256'):o.segment_hash_algorithm='HS256'
 return o

def write_error(label,e):
 (run/(label+'.error.json')).write_text(json.dumps({'kind':e.kind,'code':e.code,'operation':e.operation,'httpStatus':e.http_status,'causeCategory':e.cause_category,'serverCode':e.server_code,'serverMessage':e.server_message}))

def repeat():
 payload=bytearray([0,255,128,1]);metadata=bytearray([0,255]);o=opts('metadata');o.metadata=metadata;o.mime_type='application/日本語'
 work=encrypt(config,payload,o,provider=provider());payload[:]=b'xxxx';metadata[:]=b'xx';o.mime_type='changed'
 archive=work.result(90);a=decrypt(config,archive,provider=provider()).result(90);b=decrypt(config,archive,provider=provider()).result(90)
 assert a.payload==b'\0\xff\x80\x01' and a.metadata==b'\0\xff' and '日本語' in a.manifest_json and a==b
 started=threading.Event();released=threading.Event()
 def held(req):
  started.set();req.cancellation.wait(20);released.set();raise ProviderRejected('stopped')
 active=decrypt(config,archive,provider=held);assert started.wait(20)
 queued=decrypt(config,archive,provider=provider());assert queued.cancel()
 try:queued.result(5);raise AssertionError('queued plaintext')
 except TDFError as e:assert e.kind=='canceled'
 assert not released.is_set();assert active.cancel()
 try:active.result(20);raise AssertionError('active plaintext')
 except TDFError as e:assert e.kind=='canceled'
 assert released.is_set();assert decrypt(config,archive,provider=provider()).result(90).payload==a.payload
 async def native_async():
  data=await encrypt_async(config,b'async\0\xff',provider=provider())
  assert (await decrypt_async(config,data,provider=provider())).payload==b'async\0\xff'
 asyncio.run(native_async())

def controlled():
 def fixed(req):return AccessToken('negative-fixture-token','Bearer',int(time.time())+300)
 work=decrypt(config,(run/'archive.tdf').read_bytes(),provider=fixed)
 if name=='active-cancel':
  def stop():
   end=time.monotonic()+15
   while not (run/'entered').exists():
    assert time.monotonic()<end;time.sleep(.002)
   work.cancel()
  thread=threading.Thread(target=stop);thread.start()
 try:work.result(25);raise AssertionError('controlled plaintext')
 except TDFError as e:
  expected={'http401':'unauthenticated','http403':'denied','redirect':'http_status','content-type':'invalid_content_type','malformed':'invalid_json','active-cancel':'canceled','bom':'invalid_destination'}.get(name,'transport')
  if name=='active-cancel':assert e.kind=='canceled' and e.cause_category=='canceled',(name,e.kind,e.code)
  else:assert e.code==expected,(name,e.kind,e.code)
  if name in ('http401','http403'):assert e.server_code=='fixture-rejected' and e.server_message=='診断 café'
  write_error(name,e)
 if name=='active-cancel':thread.join()
 try:decrypt(Config(),b'\0').result(5);raise AssertionError('recovery')
 except TDFError as e:assert e.kind=='source'

try:
 if mode=='encrypt':(run/(name+'.generated.tdf')).write_bytes(encrypt(config,(run/(name+'.input')).read_bytes(),opts(name),provider=provider()).result(90))
 elif mode=='decrypt':
  v=decrypt(config,(run/(name+'.tdf')).read_bytes(),provider=provider()).result(90)
  (run/(name+'.out')).write_bytes(v.payload);(run/(name+'.metadata')).write_bytes(v.metadata);(run/(name+'.manifest')).write_text(v.manifest_json);(run/(name+'.presence')).write_text('true' if v.has_metadata else 'false')
 elif mode=='negative':
  try:decrypt(config,(run/(name+'.tdf')).read_bytes(),provider=provider()).result(90);raise AssertionError('negative plaintext')
  except TDFError as e:write_error(name,e)
 elif mode=='repeat':repeat()
 elif mode=='controlled':controlled()
 else:raise AssertionError('unknown mode')
except TDFError as e:
 raise RuntimeError('consumer failure '+mode+' '+name+' kind='+e.kind+' code='+e.code+' operation='+e.operation+' cause='+e.cause_category) from None
