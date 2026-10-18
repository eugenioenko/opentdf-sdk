"""Independent installed-package consumer of the handle API: one SDK per run,
create_tdf/decrypt_bytes, load_tdf readers, 100 round trips, the platform
service clients, and the offline controlled checks the Go consumer runs. Stock
references are test runners only."""
import json,sys,time,urllib.request,urllib.parse,hashlib,base64,secrets,threading,asyncio,http.server,ssl,subprocess,tempfile
from pathlib import Path
from cryptography.hazmat.primitives import serialization,hashes
from cryptography.hazmat.primitives.asymmetric import rsa,ec,padding,utils
from opentdf_tdf3 import SDK,AccessToken,OpenTDFError,ProviderRejected,is_valid_tdf,version,supported_features,new_attribute_value_fqn
from opentdf_tdf3.protocol.policy import objects_pb2
from opentdf_tdf3.protocol.policy.attributes import attributes_pb2
from opentdf_tdf3.protocol.policy.namespaces import namespaces_pb2
from opentdf_tdf3.protocol.authorization import authorization_pb2
run=Path(sys.argv[2]);mode,name=sys.argv[3:5]
ATTRIBUTE='https://example.com/attr/attr1/value/value1'
def b64(v):return base64.urlsafe_b64encode(v).rstrip(b'=').decode()
def jsonbytes(v):return json.dumps(v,separators=(',',':'),ensure_ascii=False).encode()

def token_provider(cfg):
 def token(request):
  now=int(time.time())
  if name=='invalid-token':return AccessToken('invalid-token','Bearer',now+300)
  if name=='expired-token':return AccessToken('expired-token','Bearer',now-1)
  if name=='provider-reject':raise ProviderRejected('provider rejected')
  endpoint='http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token'
  nonce='';thumb='';key=None;jwk=None
  if cfg.get('DPoP'):
   key=ec.generate_private_key(ec.SECP256R1()) if name=='mismatched-provider' else serialization.load_pem_private_key(cfg['AuthPrivateKeyPEM'].encode(),password=None)
   p=key.public_key().public_numbers()
   if isinstance(key,rsa.RSAPrivateKey):jwk={'e':b64(p.e.to_bytes((p.e.bit_length()+7)//8,'big')),'kty':'RSA','n':b64(p.n.to_bytes(256,'big'))}
   else:jwk={'crv':'P-256','kty':'EC','x':b64(p.x.to_bytes(32,'big')),'y':b64(p.y.to_bytes(32,'big'))}
   thumb=b64(hashlib.sha256(jsonbytes(jwk)).digest())
  for attempt in range(3):
   if request.canceled:raise ProviderRejected('provider stopped')
   headers={'Content-Type':'application/x-www-form-urlencoded'}
   if cfg.get('DPoP'):
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
   value=json.loads(body);return AccessToken(value['access_token'],'DPoP' if cfg.get('DPoP') else 'Bearer',int(time.time())+int(value['expires_in']),thumb)
  raise ProviderRejected('provider nonce retries')
 return token

# config.json carries the façade Config fields the delivery scripts write;
# they map onto the handle API's options as the Go consumer maps them.
def options_of(cfg):
 options={'insecure_plaintext':bool(cfg.get('AllowHTTP'))}
 if cfg.get('TokenURL'):options['token_endpoint']=cfg['TokenURL']
 if cfg.get('ClientID'):options['client_id']=cfg['ClientID'];options['client_secret']=cfg.get('ClientSecret','')
 if cfg.get('TokenProviderName'):options['token_provider']=token_provider(cfg)
 if cfg.get('TimeoutMillis'):options['timeout_millis']=int(cfg['TimeoutMillis'])
 if cfg.get('DPoP'):options['dpop_algorithm']=cfg.get('AuthAlgorithm') or 'ES256'
 if cfg.get('AuthPrivateKeyPEM'):options['dpop_key_pem']=cfg['AuthPrivateKeyPEM']
 return options
def kas_information(cfg):
 info={'url':cfg.get('KASURL') or cfg['PlatformURL'].rstrip('/')+'/kas'}
 for host,gen in (('public_key','KASPublicKeyPEM'),('kid','KID'),('algorithm','KASAlgorithm')):
  if cfg.get(gen):info[host]=cfg[gen]
 return info
def tdf_options(cfg,**extra):
 options={'disable_autoconfigure':True,'kas_information':[kas_information(cfg)],'attributes':[ATTRIBUTE],'segment_size':16384}
 if cfg.get('KASAlgorithm'):options['wrapping_key_algorithm']=cfg['KASAlgorithm']
 options.update(extra);return options
def reader_options(cfg):
 allow=[cfg['PlatformURL']]
 if cfg.get('KASURL'):allow.append(cfg['KASURL'])
 allow+=[r['URL'] for r in cfg.get('AllowedKAS') or []]
 options={'kas_allowlist':allow}
 if cfg.get('SessionAlgorithm'):options['session_key_type']=cfg['SessionAlgorithm']
 return options
def report(e):
 if isinstance(e,OpenTDFError):return {'kind':e.kind,'op':e.op,'httpStatus':e.http_status,'connectCode':e.connect_code,'assertionId':e.assertion_id,'message':e.message}
 return {'untyped':type(e).__name__+': '+str(e)}

def assertions(sdk,cfg):
 """A TDF with an unsigned assertion (HS256 over the payload key) and one
 signed with a caller HS256 secret, read with the right key by id, with a
 wrong key and with no key (assertion_failure), and with verification off."""
 secret='assertion-secret-'+secrets.token_hex(4)
 configs=[
  {'id':'unsigned','type':'handling','scope':'tdo','applies_to_state':'encrypted','statement':{'format':'json+stanag5636','schema':'urn:nato:stanag:5636:A:1:elements:json','value':'{"ocl":{"pol":"2ccf11cb-6c9a-4e49-9746-a7f0a295945d"}}'}},
  {'id':'signed','type':'other','scope':'payload','applies_to_state':'unencrypted','statement':{'format':'json','schema':'urn:example:assertion:v1','value':'{"note":"independent consumer"}'},'signing_key':{'alg':'HS256','key':secret}},
 ]
 payload=b'assertion payload'
 with sdk.create_tdf(payload,**tdf_options(cfg,assertions=configs)) as obj:
  manifest=obj.manifest();archive=obj.bytes
 found=manifest['assertions']
 assert [a['id'] for a in found]==['unsigned','signed'] and found[1]['binding']['method']=='jws' and found[1]['binding']['signature'],'assertions in manifest'
 assert found[0]['statement']['value']==configs[0]['statement']['value'],'statement value kept'
 def with_keys(**keys):return dict(reader_options(cfg),assertion_verification_keys=keys)
 assert sdk.decrypt_bytes(archive,**with_keys(keys={'signed':{'alg':'HS256','key':secret}}))==payload,'decrypt with the right key'
 def fails(label,options,assertion_id):
  try:sdk.decrypt_bytes(archive,**options)
  except OpenTDFError as e:
   assert e.kind=='assertion_failure' and e.assertion_id==assertion_id,label+': '+json.dumps(report(e));return report(e)
  raise AssertionError(label+' returned plaintext')
 wrong=fails('wrong key',with_keys(keys={'signed':{'alg':'HS256','key':'not the secret'}}),'signed')
 missing=fails('missing key',reader_options(cfg),'signed')
 fails('wrong default key',with_keys(default_key={'alg':'HS256','key':'not the secret'}),'unsigned')
 assert sdk.decrypt_bytes(archive,**dict(reader_options(cfg),disable_assertion_verification=True))==payload,'decrypt with verification disabled'
 (run/'assertions.generated.tdf').write_bytes(archive)
 (run/'assertions.json').write_text(json.dumps({'assertions':[{'id':a['id'],'method':a['binding']['method']} for a in found],'wrong_key':wrong,'missing_key':missing}))

def streams(sdk,cfg):
 """Phase 7: encrypt_to from a file into a file (ZIP32 for a seekable
 input), load_tdf over the file with as_file/seek/read_at/write_to, then
 decrypt_to and decrypt_file."""
 payload=bytes(range(256))*300
 plain=run/'stream.plain';plain.write_bytes(payload);encrypted=run/'stream.generated.tdf'
 with open(plain,'rb') as src,open(encrypted,'wb') as dst,sdk.encrypt_to(dst,src,**tdf_options(cfg,segment_size=16384)) as obj:
  assert obj.bytes==b'' and obj.manifest()['encryptionInformation']['integrityInformation']['segments'][0]['segmentSize']==16384,'streamed object'
 assert is_valid_tdf(encrypted.read_bytes()),'streamed archive is a TDF'
 with open(encrypted,'rb') as src,sdk.load_tdf(src,**reader_options(cfg)) as reader:
  reader.init();f=reader.as_file()
  assert f.read(5)==payload[:5] and reader.seek(-10,2)==len(payload)-10 and f.read()==payload[-10:],'read and seek'
  assert reader.read_at(16380,8)==payload[16380:16388] and reader.seek(0)==0,'read_at'
  out=bytearray()
  class Sink:
   def write(self,b):out.extend(b);return len(b)
  assert reader.write_to(Sink())==len(payload) and bytes(out)==payload,'write_to'
 with open(encrypted,'rb') as src:
  sink=Sink();out=bytearray();sdk.decrypt_to(sink,src,**reader_options(cfg));assert bytes(out)==payload,'decrypt_to'
 decrypted=run/'stream.decrypted';sdk.decrypt_file(str(encrypted),str(decrypted),**reader_options(cfg))
 assert decrypted.read_bytes()==payload,'decrypt_file'
 try:sdk.decrypt_file(str(plain),str(decrypted),**reader_options(cfg));raise AssertionError('plain file decrypted')
 except OpenTDFError as e:assert e.kind=='not_decryptable' and decrypted.read_bytes()==payload,report(e)

def repeat(sdk,cfg):
 """100 encrypt and decrypt calls on one SDK, independent inputs and results,
 every reader closed; then the SDK is closed and later calls fail."""
 original=bytes([0,255,128,1])
 for i in range(100):
  payload=bytearray(original+bytes([i]))
  with sdk.create_tdf(payload,**tdf_options(cfg,metadata='round %d'%i,mime_type='application/日本語')) as obj:
   assert obj.size()==len(obj.bytes) and obj.manifest()['payload']['mimeType']=='application/日本語'
   archive=obj.bytes
  payload[0]=9
  with sdk.load_tdf(archive,**reader_options(cfg)) as reader:assert len(reader.data_attributes())==1
  plain=sdk.decrypt_bytes(archive,**reader_options(cfg));assert plain==original+bytes([i]),'round trip'
 streams(sdk,cfg)
 try:
  with SDK(cfg['PlatformURL'],**{**options_of(cfg),'client_secret':'wrong'}) as s:s.decrypt_bytes(archive,**reader_options(cfg))
  raise AssertionError('wrong credentials accepted')
 except OpenTDFError as e:assert e.kind=='auth' and e.http_status==401,report(e)
 started=threading.Event()
 def held(request):
  started.set();request.cancellation.wait(20);raise ProviderRejected('stopped')
 holder=SDK(cfg['PlatformURL'],**{k:v for k,v in options_of(cfg).items() if k not in ('client_id','client_secret')},token_provider=held)
 try:holder.decrypt_bytes(archive,timeout=1,**reader_options(cfg));raise AssertionError('held provider returned plaintext')
 except OpenTDFError as e:assert e.kind=='deadline_exceeded',report(e)
 assert started.is_set();holder.close()
 async def native_async():
  async with await SDK.create_async(cfg['PlatformURL'],**options_of(cfg)) as s:
   data=await s.encrypt_bytes_async(b'async\0\xff',**tdf_options(cfg))
   assert (await s.decrypt_bytes_async(data,**reader_options(cfg)))==b'async\0\xff'
 asyncio.run(native_async())
 sdk.close()
 try:sdk.decrypt_bytes(archive,**reader_options(cfg));raise AssertionError('closed handle accepted a call')
 except OpenTDFError as e:assert e.kind=='closed',e.kind

class Fixture(http.server.BaseHTTPRequestHandler):
 """A platform that only discovers and rejects: no KAS."""
 mode='';state=None
 def log_message(self,*args):pass
 def do_POST(self):
  self.state['requests']+=1;self.rfile.read(int(self.headers.get('Content-Length','0')))
  issuer='http://'+self.headers['Host']+'/issuer';mode=self.mode;well_known=self.path=='/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration'
  def reply(status,body,content_type='application/json',extra=()):
   self.send_response(status);self.send_header('Content-Type',content_type);self.send_header('Content-Length',str(len(body)))
   for k,v in extra:self.send_header(k,v)
   self.end_headers();self.wfile.write(body)
  if mode=='wellknown-500' and well_known:reply(500,b'{"code":"internal","message":"harmless fixture diagnostic"}')
  elif mode=='redirect' and well_known:reply(307,b'',extra=[('Location','/leak')])
  elif mode=='content-type' and well_known:reply(200,b'{}','text/html')
  elif mode=='malformed-response' and well_known:reply(200,b'{bad')
  elif mode in ('caller-deadline','source-deadline') and well_known:
   self.state['entered'].set()
   try:
    self.connection.settimeout(15);self.rfile.read(1)
   except OSError:pass
   self.state['released'].set();self.close_connection=True
  elif well_known:reply(200,jsonbytes({'configuration':{'platform_issuer':issuer,'idp':{'issuer':issuer,'token_endpoint':issuer+'/protocol/openid-connect/token'}}}))
  elif self.path=='/issuer/protocol/openid-connect/token':reply(401,b'{"error":"invalid_client","error_description":"harmless fixture diagnostic"}')
  elif self.path=='/grpc.health.v1.Health/Check':reply(200,b'{"status":"SERVING"}')
  elif self.path=='/policy.kasregistry.KeyAccessServerRegistryService/ListKeyAccessServers':reply(200,b'{"keyAccessServers":[]}')
  else:reply(404,jsonbytes({'code':'unimplemented','message':self.path}))
 def do_GET(self):
  self.state['requests']+=1;issuer='http://'+self.headers['Host']+'/issuer'
  if self.path=='/issuer/.well-known/openid-configuration':
   body=jsonbytes({'token_endpoint':issuer+'/protocol/openid-connect/token'});self.send_response(200);self.send_header('Content-Type','application/json');self.send_header('Content-Length',str(len(body)));self.end_headers();self.wfile.write(body)
  else:self.send_response(404);self.send_header('Content-Length','0');self.end_headers()

def serve(mode,tls=None):
 state={'requests':0,'entered':threading.Event(),'released':threading.Event()}
 handler=type('Handler',(Fixture,),{'mode':mode,'state':state})
 server=http.server.ThreadingHTTPServer(('127.0.0.1',0),handler)
 if tls:server.socket=tls.wrap_socket(server.socket,server_side=True)
 threading.Thread(target=server.serve_forever,daemon=True).start()
 return server,state,('https' if tls else 'http')+'://127.0.0.1:'+str(server.server_port)

def controlled():
 rows=[]
 kas_pem=rsa.generate_private_key(public_exponent=65537,key_size=2048).public_key().public_bytes(serialization.Encoding.PEM,serialization.PublicFormat.SubjectPublicKeyInfo).decode()
 closed=http.server.ThreadingHTTPServer(('127.0.0.1',0),Fixture);closed_url='http://127.0.0.1:'+str(closed.server_port);closed.server_close()
 plain,_,plain_url=serve('')
 sdk=SDK(plain_url,client_id='opentdf',client_secret='secret')
 with sdk.create_tdf(b'offline payload',disable_autoconfigure=True,kas_information=[{'url':'http://localhost:9999/malicious/kas','public_key':kas_pem,'kid':'k1','algorithm':'rsa:2048'}],attributes=['https://example.com/attr/a/value/b']) as obj:archive=obj.bytes
 def check(label,e,want,**extra):
  got=e.kind if isinstance(e,OpenTDFError) else 'untyped'
  assert got==want,'controlled wrong kind: %s: %s (%s)'%(label,got,report(e))
  rows.append({'case':label,**report(e),**extra})
 try:SDK(closed_url,validate_connection=True);raise AssertionError('unreachable platform accepted')
 except OpenTDFError as e:assert 'platform unreachable or not responding' in e.message;check('unreachable',e,'transport')
 with tempfile.TemporaryDirectory() as tmp:
  key,cert=Path(tmp)/'key.pem',Path(tmp)/'cert.pem'
  subprocess.run(['openssl','req','-x509','-newkey','rsa:2048','-nodes','-keyout',str(key),'-out',str(cert),'-days','1','-subj','/CN=localhost','-addext','subjectAltName=IP:127.0.0.1'],check=True,stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
  tls=ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER);tls.load_cert_chain(str(cert),str(key))
  want={'wellknown-500':'transport','redirect':'transport','content-type':'transport','malformed-response':'transport','untrusted-tls':'transport','caller-deadline':'deadline_exceeded','source-deadline':'deadline_exceeded','token-401':'auth'}
  for label in want:
   server,state,url=serve(label,tls if label=='untrusted-tls' else None)
   options={'client_id':'opentdf','client_secret':'secret'};timeout=None
   if label=='caller-deadline':timeout=0.1
   if label=='source-deadline':options['timeout_millis']=100
   start=time.monotonic();failure=None
   try:
    s=SDK(url,timeout=timeout,**options)
    try:
     if label=='token-401':s.decrypt_bytes(archive,ignore_allowlist=True)
    finally:s.close()
   except OpenTDFError as e:failure=e
   elapsed=time.monotonic()-start
   assert failure is not None,'controlled negative succeeded: '+label
   check(label,failure,want[label],requests=state['requests'],elapsed_ms=int(elapsed*1000))
   if label not in ('caller-deadline','token-401','untrusted-tls'):assert 'failed to retrieve platform configuration' in failure.message,label
   if label=='token-401':assert failure.http_status==401 and failure.connect_code=='invalid_client' and 'tdf: decrypt failed' in failure.message,'token failure fields'
   if label in ('caller-deadline','source-deadline'):assert elapsed<3,'deadline delayed';assert state['released'].wait(3),'HTTP request cleanup not observed: '+label
   if label=='untrusted-tls':assert state['requests']==0,'untrusted TLS reached handler'
   server.shutdown();server.server_close()
 try:sdk.decrypt_bytes(archive,kas_allowlist=[plain_url]);raise AssertionError('allowlist')
 except OpenTDFError as e:assert 'tdf: decrypt failed' in e.message;check('allowlist',e,'allowlist_rejected')
 try:sdk.decrypt_bytes(b'not a tdf');raise AssertionError('invalid tdf')
 except OpenTDFError as e:assert 'tdf: input is not decryptable' in e.message;check('invalid-tdf',e,'not_decryptable')
 try:sdk.create_tdf(b'x',target_mode='4.2.2');raise AssertionError('unsupported option')
 except OpenTDFError as e:check('unsupported-option',e,'unsupported_feature')
 try:sdk.create_tdf(b'x',segment_size='big');raise AssertionError('invalid argument')
 except OpenTDFError as e:check('invalid-argument',e,'invalid_argument')
 for i in range(100):
  with sdk.load_tdf(archive,ignore_allowlist=True) as reader:assert reader.manifest()['encryptionInformation']['keyAccess'][0]['kid']=='k1'
 sdk.close()
 try:sdk.decrypt_bytes(archive);raise AssertionError('closed')
 except OpenTDFError as e:check('closed-handle',e,'closed')
 rows.append({'case':'handles','readers':100,'closed':True})
 plain.shutdown();plain.server_close()
 assert is_valid_tdf(archive) and version() and 'hexless' in supported_features()
 (run/'controlled-negatives.json').write_text(json.dumps({'scope':'controlled discovery, transport, allowlist, option and handle checks through actual generated exports; no mock positive KAS','negatives':rows},indent=2)+'\n')

# services is xtest's test_self.py through the service clients: list
# namespaces, create a namespace and an attribute, read the attribute back by
# id, through the namespace's list and through the raw JSON call, and check a
# missing attribute's error.
def services(sdk,cfg):
 ns_name='gcpy'+secrets.token_hex(6)+'.com'
 before=sdk.namespaces.list_namespaces(namespaces_pb2.ListNamespacesRequest())
 namespace=sdk.namespaces.create_namespace(namespaces_pb2.CreateNamespaceRequest(name=ns_name)).namespace
 assert namespace.id and namespace.name==ns_name,'created namespace'
 after=sdk.namespaces.list_namespaces()
 assert any(n.id==namespace.id for n in after.namespaces) and len(after.namespaces)==len(before.namespaces)+1,'namespace listed'
 created=sdk.attributes.create_attribute(attributes_pb2.CreateAttributeRequest(namespace_id=namespace.id,name='free',rule=objects_pb2.ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF,values=['1','2','3']))
 attribute=sdk.attributes.get_attribute(attributes_pb2.GetAttributeRequest(attribute_id=created.attribute.id)).attribute
 values=[v.value for v in attribute.values]
 assert attribute.fqn=='https://'+ns_name+'/attr/free' and attribute.rule==objects_pb2.ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF and values==['1','2','3'] and attribute.namespace.name==ns_name,'attribute read back'
 listed=sdk.attributes.list_attributes(attributes_pb2.ListAttributesRequest(namespace=ns_name))
 assert len(listed.attributes)==1 and listed.attributes[0].id==attribute.id,'attribute listed'
 raw=json.loads(sdk.call('policy.attributes.AttributesService','GetAttribute',json.dumps({'attributeId':attribute.id})))
 assert raw['attribute']['fqn']==attribute.fqn,'raw JSON call'
 assert asyncio.run(sdk.attributes.get_attribute_async(attributes_pb2.GetAttributeRequest(attribute_id=attribute.id))).attribute.fqn==attribute.fqn,'async client'
 try:sdk.attributes.get_attribute(attributes_pb2.GetAttributeRequest(attribute_id='00000000-0000-0000-0000-000000000000'));raise AssertionError('missing attribute')
 except OpenTDFError as e:missing=e
 assert missing.connect_code=='not_found' and missing.http_status==404,'missing attribute error'
 try:sdk.attributes.get_attribute(namespaces_pb2.ListNamespacesRequest());raise AssertionError('wrong request type')
 except TypeError:pass
 (run/'services.json').write_text(json.dumps({'namespace':ns_name,'namespaces_before':len(before.namespaces),'namespaces_after':len(after.namespaces),'attribute_fqn':attribute.fqn,'values':values,
  'not_found':report(missing),'helpers':helpers(sdk,cfg,ns_name,attribute.fqn),'autoconfigure':autoconfigure(sdk,cfg,attribute)}))

# autoconfigure encrypts under a value of the attribute created, looked up
# (attributes) and offline (attribute_values): the attribute has no grants, so
# the plan is the base key or the default KAS.
def autoconfigure(sdk,cfg,attribute):
 value=attribute.fqn+'/value/1'
 offline=objects_pb2.Value(fqn=value,value='1',attribute=objects_pb2.Attribute(fqn=attribute.fqn,rule=attribute.rule,namespace=attribute.namespace))
 out={'key':new_attribute_value_fqn(value)['key']}
 for label,extra in (('looked_up',{'attributes':[value]}),('offline',{'attribute_values':[offline]})):
  with sdk.create_tdf(label.encode(),kas_information=[kas_information(cfg)],**extra) as obj:
   kaos=obj.manifest()['encryptionInformation']['keyAccess']
  out[label]={'kaos':len(kaos),'kas':kaos[0]['url'],'sid':kaos[0].get('sid','')}
 return out

# The reference's attribute helpers on the attribute services created,
# SDK.auth, and an SDK whose platform endpoint is unreachable but whose core
# connection is the platform.
def helpers(sdk,cfg,namespace,fqn):
 out={'attribute_exists':sdk.attribute_exists(fqn),'attribute_missing':sdk.attribute_exists(fqn+'-missing'),
      'value_exists':sdk.attribute_value_exists(fqn+'/value/1'),'value_missing':asyncio.run(sdk.attribute_value_exists_async(fqn+'/value/9'))}
 out['listed']=[a.fqn for a in sdk.list_attributes(namespace)]
 sdk.validate_attributes([fqn+'/value/1',fqn+'/value/2']);out['validate_ok']=True
 try:sdk.validate_attributes([fqn+'/value/1',fqn+'/value/9']);raise AssertionError('validate missing')
 except OpenTDFError as e:out['validate_missing']=report(e)
 out['entity_attributes']=sdk.get_entity_attributes(authorization_pb2.Entity(id='e1',client_id='opentdf-sdk'))
 token=sdk.auth().access_token()
 out['access_token']={'present':token.value!='','scheme':token.scheme}
 with SDK('http://127.0.0.1:9',**options_of(cfg),core_endpoint=cfg['PlatformURL']) as override:
  out['override']={'endpoint':override.conn().endpoint,'attribute_exists':override.attribute_exists(fqn)}
 return out

try:
 if mode=='controlled':
  run.mkdir(parents=True,exist_ok=True);controlled()
 else:
  cfg=json.loads((run/'config.json').read_text())
  with SDK(cfg['PlatformURL'],**options_of(cfg)) as sdk:
   if mode=='encrypt':
    extra={}
    if (name=='metadata' or name.endswith('-metadata')) and name!='empty-metadata' and not name.endswith('-empty-metadata'):extra['metadata']='{"source":"independent metadata","count":7}'
    if name=='denied' or name.endswith('-denied'):extra['attributes']=['https://example.com/attr/attr1/value/value2']
    split=name=='split' or name.endswith('-split')
    if split:extra['kas_information']=[kas_information(cfg),kas_information(cfg)]
    with sdk.create_tdf((run/(name+'.input')).read_bytes(),**tdf_options(cfg,**extra)) as obj:
     assert obj.size()==len(obj.bytes) and len(obj.manifest()['encryptionInformation']['keyAccess'])==(2 if split else 1)
     (run/(name+'.generated.tdf')).write_bytes(obj.bytes)
   elif mode=='decrypt':
    data=(run/(name+'.tdf')).read_bytes()
    with sdk.load_tdf(data,**reader_options(cfg)) as reader:
     manifest=reader.manifest();kao=manifest['encryptionInformation']['keyAccess']
     has_metadata=bool(kao) and kao[0]['encryptedMetadata']!=''
     metadata=b''
     if has_metadata:reader.init();metadata=reader.unencrypted_metadata()
    payload=sdk.decrypt_bytes(data,**reader_options(cfg))
    (run/(name+'.out')).write_bytes(payload);(run/(name+'.metadata')).write_bytes(metadata);(run/(name+'.manifest')).write_text(json.dumps(manifest));(run/(name+'.presence')).write_text('true' if has_metadata else 'false')
   elif mode=='negative':
    try:sdk.decrypt_bytes((run/(name+'.tdf')).read_bytes(),**reader_options(cfg));raise AssertionError('negative plaintext')
    except OpenTDFError as e:(run/(name+'.error.json')).write_text(json.dumps(report(e)))
   elif mode=='assertions':assertions(sdk,cfg)
   elif mode=='repeat':repeat(sdk,cfg)
   elif mode=='services':services(sdk,cfg)
   else:raise AssertionError('unknown mode')
 print('PASS generated native library',mode,name)
except OpenTDFError as e:
 raise RuntimeError('consumer failure '+mode+' '+name+' '+json.dumps(report(e))) from None
