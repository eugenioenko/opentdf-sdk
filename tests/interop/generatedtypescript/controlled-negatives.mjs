// Offline controlled checks through the installed package, the TypeScript
// twin of the Go consumer's negative.go: a platform that only discovers and
// rejects (no KAS), then allowlist, option, input and handle checks on a TDF
// written against an explicit KAS key.
import {createServer as httpServer} from 'node:http';
import {createServer as httpsServer} from 'node:https';
import {writeFile,readFile,mkdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {execFileSync} from 'node:child_process';
import {generateKeyPairSync} from 'node:crypto';

function assert(ok,label){if(!ok)throw new Error(label);}

function fakePlatform(mode,state){
 return async(req,res)=>{
  state.requests++;
  for await(const _ of req){}
  res.setHeader('Content-Type','application/json');
  const issuer='http://'+req.headers.host+'/issuer';
  const wellKnown=req.url==='/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration';
  if(mode==='wellknown-500'&&wellKnown){res.writeHead(500);res.end('{"code":"internal","message":"harmless fixture diagnostic"}');}
  else if(mode==='redirect'&&wellKnown){res.writeHead(307,{Location:'/leak'});res.end();}
  else if(mode==='content-type'&&wellKnown){res.setHeader('Content-Type','text/html');res.end('{}');}
  else if(mode==='malformed-response'&&wellKnown)res.end('{bad');
  else if((mode==='caller-deadline'||mode==='source-deadline')&&wellKnown){state.entered.resolve();res.on('close',()=>state.released.resolve());}
  else if(wellKnown)res.end(JSON.stringify({configuration:{platform_issuer:issuer,idp:{issuer,token_endpoint:issuer+'/protocol/openid-connect/token'}}}));
  else if(req.url==='/issuer/.well-known/openid-configuration')res.end(JSON.stringify({token_endpoint:issuer+'/protocol/openid-connect/token'}));
  else if(req.url==='/issuer/protocol/openid-connect/token'){res.writeHead(401);res.end('{"error":"invalid_client","error_description":"harmless fixture diagnostic"}');}
  else if(req.url==='/grpc.health.v1.Health/Check')res.end('{"status":"SERVING"}');
  else if(req.url==='/policy.kasregistry.KeyAccessServerRegistryService/ListKeyAccessServers')res.end('{"keyAccessServers":[]}');
  else{res.writeHead(404);res.end('{"code":"unimplemented","message":"'+req.url+'"}');}
 };
}
function kind(e){
 if(e?.name==='OpenTDFError')return [e.kind,{kind:e.kind,op:e.op,httpStatus:e.httpStatus,connectCode:e.connectCode,message:e.message}];
 return ['untyped',{error:String(e)}];
}
async function listen(server){await new Promise(r=>server.listen(0,'127.0.0.1',r));server.unref();return 'http://127.0.0.1:'+server.address().port;}
async function stop(server){server.closeAllConnections?.();await new Promise(r=>server.close(r));}

export async function controlledNegatives(g,run){
 await mkdir(run,{recursive:true});
 const rows=[];
 const kasPEM=generateKeyPairSync('rsa',{modulusLength:2048}).publicKey.export({type:'spki',format:'pem'});
 const closed=httpServer(()=>{});const closedURL=await listen(closed);await stop(closed);

 const plain=httpServer(fakePlatform('',{requests:0}));const plainURL=await listen(plain);
 const sdk=await g.SDK.create(plainURL,{clientId:'opentdf',clientSecret:'secret'});
 const object=await sdk.createTdf(new TextEncoder().encode('offline payload'),{disableAutoconfigure:true,kasInformation:[{url:'http://localhost:9999/malicious/kas',publicKey:kasPEM,kid:'k1',algorithm:'rsa:2048'}],attributes:['https://example.com/attr/a/value/b']});
 const archive=object.bytes;await object.close();

 const check=(name,e,want,extra={})=>{const [got,row]=kind(e);if(got!==want)throw new Error('controlled wrong kind: '+name+': '+got+' ('+JSON.stringify(row)+')');rows.push({case:name,...row,...extra});};

 let error;try{await g.SDK.create(closedURL,{validateConnection:true});}catch(e){error=e;}
 assert(error&&error.message.includes('platform unreachable or not responding'),'unreachable platform sentinel');
 check('unreachable',error,'transport');

 const key=resolve(run,'tls-key.pem'),cert=resolve(run,'tls-cert.pem');
 execFileSync('openssl',['req','-x509','-newkey','rsa:2048','-nodes','-keyout',key,'-out',cert,'-days','1','-subj','/CN=localhost','-addext','subjectAltName=IP:127.0.0.1'],{stdio:'ignore'});
 for(const name of ['wellknown-500','redirect','content-type','malformed-response','untrusted-tls','caller-deadline','source-deadline','token-401']){
  const state={requests:0,entered:Promise.withResolvers(),released:Promise.withResolvers()};
  const handler=fakePlatform(name,state);
  const server=name==='untrusted-tls'?httpsServer({key:await readFile(key),cert:await readFile(cert)},handler):httpServer(handler);
  const url=(name==='untrusted-tls'?'https':'http')+'://127.0.0.1:'+(await listen(server)).split(':')[2];
  const options={clientId:'opentdf',clientSecret:'secret'};
  const call={};
  if(name==='caller-deadline')call.signal=AbortSignal.timeout(100);
  if(name==='source-deadline')options.timeoutMillis=100;
  const start=performance.now();
  let failure;
  try{
   const s=await g.SDK.create(url,options,call);
   if(name==='token-401'){try{await s.decryptBytes(archive,{ignoreAllowlist:true});}finally{await s.close();}}
   else await s.close();
  }catch(e){failure=e;}
  const elapsed=performance.now()-start;
  assert(failure,'controlled negative succeeded: '+name);
  const want={'wellknown-500':'transport',redirect:'transport','content-type':'transport','malformed-response':'transport','untrusted-tls':'transport','caller-deadline':'canceled','source-deadline':'deadline_exceeded','token-401':'auth'}[name];
  check(name,failure,want,{requests:state.requests,elapsed_ms:Math.round(elapsed)});
  if(name!=='caller-deadline'&&name!=='token-401'&&name!=='untrusted-tls')assert(failure.message.includes('failed to retrieve platform configuration'),'discovery failures carry ErrPlatformConfigFailed: '+name);
  if(name==='token-401')assert(failure.httpStatus===401&&failure.connectCode==='invalid_client'&&failure.message.includes('tdf: decrypt failed'),'token failure fields');
  if(name==='caller-deadline'||name==='source-deadline'){assert(elapsed<2000,'deadline delayed');await Promise.race([state.released.promise,new Promise((_,reject)=>setTimeout(()=>reject(new Error('HTTP request cleanup not observed: '+name)),1000))]);}
  if(name==='untrusted-tls')assert(state.requests===0,'untrusted TLS reached handler');
  await stop(server);
 }

 error=undefined;try{await sdk.decryptBytes(archive,{kasAllowlist:[plainURL]});}catch(e){error=e;}
 assert(error&&error.message.includes('tdf: decrypt failed'),'allowlist rejection is a decrypt failure');check('allowlist',error,'allowlist_rejected');
 error=undefined;try{await sdk.decryptBytes(new TextEncoder().encode('not a tdf'));}catch(e){error=e;}
 assert(error&&error.message.includes('tdf: input is not decryptable'),'invalid input sentinel');check('invalid-tdf',error,'not_decryptable');
 error=undefined;try{await sdk.createTdf(new Uint8Array([120]),{targetMode:'4.2.2'});}catch(e){error=e;}
 check('unsupported-option',error,'unsupported_feature');
 error=undefined;try{await sdk.createTdf(new Uint8Array([120]),{segmentSize:'big'});}catch(e){error=e;}
 check('invalid-argument',error,'invalid_argument');

 for(let i=0;i<100;i++){
  const reader=await sdk.loadTdf(archive,{ignoreAllowlist:true});
  const manifest=await reader.manifest();
  assert(manifest.encryptionInformation.keyAccess[0].kid==='k1','manifest');
  await reader.close();
 }
 await sdk.close();
 error=undefined;try{await sdk.decryptBytes(archive);}catch(e){error=e;}
 check('closed-handle',error,'closed');
 rows.push({case:'handles',readers:100,closed:true});
 await stop(plain);
 await writeFile(resolve(run,'controlled-negatives.json'),JSON.stringify({scope:'controlled discovery, transport, allowlist, option and handle checks through actual generated exports; no mock positive KAS',negatives:rows},null,2)+'\n');
}
