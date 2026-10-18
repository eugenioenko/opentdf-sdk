// Independent native importing consumer of the installed package; no shared
// SDK, compiler or reference import. It drives the handle API: one SDK per
// run, createTdf/decryptBytes, loadTdf readers, the platform service clients,
// and offline controlled checks.
import {readFile,writeFile} from 'node:fs/promises';
import {dirname,resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {acquireToken} from './token-provider.mjs';
import {controlledNegatives} from './controlled-negatives.mjs';

const [sdkRoot,run,mode,name]=process.argv.slice(2);
if(!name)throw new Error('consumer sdk run mode case');
const entry=resolve(process.env.TDF_TS_NODE_PACKAGE??process.env.TDF_TS_PACKAGE??resolve(sdkRoot,'../goalchemy/out/typescript-tdf-library/sdk/dist/node-index.js'));
const g=await import(pathToFileURL(entry).href);
const read=n=>readFile(resolve(run,n));
const write=(n,b)=>writeFile(resolve(run,n),b??new Uint8Array(),{mode:0o600});
function assert(ok,label){if(!ok)throw new Error(label);}
const ATTRIBUTE='https://example.com/attr/attr1/value/value1';

// config.json carries the façade Config fields the delivery scripts write;
// they map onto the handle API's options as the Go consumer maps them.
function optionsOf(cfg){
 const options={insecurePlaintext:Boolean(cfg.AllowHTTP)};
 if(cfg.TokenURL)options.tokenEndpoint=cfg.TokenURL;
 if(cfg.ClientID){options.clientId=cfg.ClientID;options.clientSecret=cfg.ClientSecret??'';}
 if(cfg.TokenProviderName)options.tokenProvider=signal=>acquireToken(cfg,signal,name);
 if(cfg.TimeoutMillis)options.timeoutMillis=Number(cfg.TimeoutMillis);
 if(cfg.DPoP)options.dpopAlgorithm=cfg.AuthAlgorithm||'ES256';
 if(cfg.AuthPrivateKeyPEM)options.dpopKeyPem=cfg.AuthPrivateKeyPEM;
 return options;
}
function kasInformation(cfg){
 return {url:cfg.KASURL||cfg.PlatformURL.replace(/\/$/,'')+'/kas',publicKey:cfg.KASPublicKeyPEM,kid:cfg.KID,algorithm:cfg.KASAlgorithm};
}
function tdfOptions(cfg,extra={}){
 const options={disableAutoconfigure:true,kasInformation:[kasInformation(cfg)],attributes:[ATTRIBUTE],segmentSize:16384,...extra};
 if(cfg.KASAlgorithm)options.wrappingKeyAlgorithm=cfg.KASAlgorithm;
 return options;
}
function readerOptions(cfg){
 const allow=[cfg.PlatformURL];
 if(cfg.KASURL)allow.push(cfg.KASURL);
 for(const r of cfg.AllowedKAS??[])allow.push(r.URL);
 const options={kasAllowlist:allow};
 if(cfg.SessionAlgorithm)options.sessionKeyType=cfg.SessionAlgorithm;
 return options;
}
function report(e){
 if(e instanceof g.OpenTDFError)return {kind:e.kind,op:e.op,httpStatus:e.httpStatus,connectCode:e.connectCode,assertionId:e.assertionId,message:e.message};
 return {untyped:e?.name+': '+e?.message};
}

try{
 if(mode==='controlled'){
  await controlledNegatives(g,run);
  console.log('PASS generated native library',mode,name);
 }else{
  const cfg=JSON.parse(await readFile(resolve(run,'config.json'),'utf8'));
  await using sdk=await g.SDK.create(cfg.PlatformURL,optionsOf(cfg));
  if(mode==='encrypt'){
   const extra={};
   if((name==='metadata'||name.endsWith('-metadata'))&&name!=='empty-metadata'&&!name.endsWith('-empty-metadata'))extra.metadata='{"source":"independent metadata","count":7}';
   if(name==='denied'||name.endsWith('-denied'))extra.attributes=['https://example.com/attr/attr1/value/value2'];
   const split=name==='split'||name.endsWith('-split');
   if(split)extra.kasInformation=[kasInformation(cfg),kasInformation(cfg)];
   const object=await sdk.createTdf(new Uint8Array(await read(name+'.input')),tdfOptions(cfg,extra));
   const manifest=await object.manifest();
   assert((await object.size())===object.bytes.length&&manifest.encryptionInformation.keyAccess.length===(split?2:1),'object');
   await object.close();
   await write(name+'.generated.tdf',object.bytes);
  }else if(mode==='decrypt'){
   const input=new Uint8Array(await read(name+'.tdf'));
   const reader=await sdk.loadTdf(input,readerOptions(cfg));
   const manifest=await reader.manifest();
   const hasMetadata=manifest.encryptionInformation.keyAccess.length>0&&manifest.encryptionInformation.keyAccess[0].encryptedMetadata!=='';
   let metadata=new Uint8Array();
   if(hasMetadata){await reader.init();metadata=await reader.unencryptedMetadata();}
   await reader.close();
   const payload=await sdk.decryptBytes(input,readerOptions(cfg));
   await write(name+'.out',payload);await write(name+'.metadata',metadata);await write(name+'.manifest',new TextEncoder().encode(JSON.stringify(manifest)));await write(name+'.presence',String(hasMetadata));
  }else if(mode==='negative'){
   let rejected=false;
   try{await sdk.decryptBytes(new Uint8Array(await read(name+'.tdf')),readerOptions(cfg));}
   catch(e){assert(e instanceof g.OpenTDFError,'typed SDK error');rejected=true;await write(name+'.error.json',JSON.stringify(report(e)));}
   assert(rejected,'negative returned output');
  }else if(mode==='assertions'){
   await assertions(sdk,cfg);
  }else if(mode==='repeat'){
   await repeat(sdk,cfg);
  }else if(mode==='services'){
   await services(sdk,cfg);
  }else throw new Error('unknown mode');
  console.log('PASS generated native library',mode,name);
 }
}catch(e){console.error('consumer failure',mode,name,report(e));process.exitCode=1;}

// assertions writes a TDF with an unsigned assertion (HS256 over the payload
// key) and one signed with a caller HS256 secret, then reads it with the
// right key by id, with a wrong key and with no key (both assertion_failure)
// and with verification disabled.
async function assertions(sdk,cfg){
 const secret='assertion-secret-'+Date.now().toString(36);
 const configs=[
  {id:'unsigned',type:'handling',scope:'tdo',appliesToState:'encrypted',statement:{format:'json+stanag5636',schema:'urn:nato:stanag:5636:A:1:elements:json',value:'{"ocl":{"pol":"2ccf11cb-6c9a-4e49-9746-a7f0a295945d"}}'}},
  {id:'signed',type:'other',scope:'payload',appliesToState:'unencrypted',statement:{format:'json',schema:'urn:example:assertion:v1',value:'{"note":"independent consumer"}'},signingKey:{alg:'HS256',key:secret}},
 ];
 const input=new TextEncoder().encode('assertion payload');
 const object=await sdk.createTdf(input,tdfOptions(cfg,{assertions:configs}));
 const manifest=await object.manifest();
 await object.close();
 assert(manifest.assertions.length===2&&manifest.assertions[0].id==='unsigned'&&manifest.assertions[1].id==='signed'&&manifest.assertions[1].binding.method==='jws'&&manifest.assertions[1].binding.signature!=='','assertions in manifest');
 assert(manifest.assertions[0].statement.value===configs[0].statement.value,'statement value kept');
 const withKeys=keys=>({...readerOptions(cfg),assertionVerificationKeys:keys});
 const plain=await sdk.decryptBytes(object.bytes,withKeys({keys:{signed:{alg:'HS256',key:secret}}}));
 assert(plain.length===input.length&&plain.every((b,i)=>b===input[i]),'decrypt with the right key');
 const fails=async(label,options,id)=>{let error;try{await sdk.decryptBytes(object.bytes,options);}catch(e){error=e;}
  assert(error instanceof g.OpenTDFError&&error.kind==='assertion_failure'&&error.assertionId===id,label+': '+JSON.stringify(report(error)));return error;};
 const wrong=await fails('wrong key',withKeys({keys:{signed:{alg:'HS256',key:'not the secret'}}}),'signed');
 const missing=await fails('missing key',readerOptions(cfg),'signed');
 await fails('wrong default key',withKeys({defaultKey:{alg:'HS256',key:'not the secret'}}),'unsigned');
 const unverified=await sdk.decryptBytes(object.bytes,{...readerOptions(cfg),disableAssertionVerification:true});
 assert(unverified.length===input.length,'decrypt with verification disabled');
 await write('assertions.generated.tdf',object.bytes);
 await write('assertions.json',new TextEncoder().encode(JSON.stringify({assertions:manifest.assertions.map(a=>({id:a.id,method:a.binding.method})),wrong_key:report(wrong),missing_key:report(missing)})));
}

// repeat runs 100 encrypt and decrypt calls on one SDK, keeps inputs and
// results independent, closes every reader handle, then closes the SDK and
// checks that later calls fail; an AbortSignal cancels an active provider.
async function repeat(sdk,cfg){
 const original=new Uint8Array([0,255,128,1]);
 for(let i=0;i<100;i++){
  const input=new Uint8Array([...original,i]);
  const object=await sdk.createTdf(input,tdfOptions(cfg,{metadata:'round '+i,mimeType:'application/日本語'}));
  assert((await object.size())===object.bytes.length,'size');
  assert((await object.manifest()).payload.mimeType==='application/日本語','UTF-8 manifest');
  await object.close();
  const reader=await sdk.loadTdf(object.bytes,readerOptions(cfg));
  assert((await reader.dataAttributes()).length===1,'attributes');
  await reader.close();
  const plain=await sdk.decryptBytes(object.bytes,readerOptions(cfg));
  assert(plain.length===input.length&&plain.every((b,j)=>b===input[j]),'round trip');
  plain[0]=1;assert(input[0]===0,'result alias');
 }
 let rejected=false;
 try{const s=await g.SDK.create(cfg.PlatformURL,{...optionsOf(cfg),clientSecret:'wrong'});await s.decryptBytes(original,readerOptions(cfg));}catch(e){assert(e instanceof g.OpenTDFError,'typed rejection');rejected=true;}
 assert(rejected,'wrong credentials rejected');
 const started=Promise.withResolvers(),abort=new AbortController();
 const provider=async signal=>{started.resolve();await new Promise(r=>signal.addEventListener('abort',r,{once:true}));throw new Error('canceled');};
 const holder=await g.SDK.create(cfg.PlatformURL,{...optionsOf(cfg),clientId:undefined,clientSecret:undefined,tokenProvider:provider});
 const canceled=holder.decryptBytes(original,readerOptions(cfg),{signal:abort.signal});
 await Promise.race([started.promise,canceled.catch(()=>{})]);
 abort.abort('test cancel');
 let failed=false;
 try{await canceled;}catch(e){failed=e instanceof g.OpenTDFError;}
 assert(failed,'active provider cancel is typed');
 await holder.close();
 await sdk.close();
 let closed=false;
 try{await sdk.decryptBytes(original,readerOptions(cfg));}catch(e){closed=e instanceof g.OpenTDFError&&e.kind==='closed';}
 assert(closed,'closed handle accepted a call');
}

// services is xtest's test_self.py through the Connect-ES service clients:
// list namespaces, create a namespace and an attribute, read the attribute
// back by id, through the namespace's list and through the raw JSON call, and
// check a missing attribute's ConnectError.
async function services(sdk,cfg){
 const {AttributeRuleTypeEnum}=await import(pathToFileURL(resolve(dirname(entry),'protocol/policy/objects_pb.js')).href);
 const name='gcts'+Date.now().toString(36)+'.com';
 const before=await sdk.namespaces.listNamespaces({});
 const {namespace}=await sdk.namespaces.createNamespace({name});
 assert(namespace?.id&&namespace.name===name,'created namespace');
 const after=await sdk.namespaces.listNamespaces({});
 assert(after.namespaces.some(n=>n.id===namespace.id)&&after.namespaces.length===before.namespaces.length+1,'namespace listed');
 const created=await sdk.attributes.createAttribute({namespaceId:namespace.id,name:'free',rule:AttributeRuleTypeEnum.ANY_OF,values:['1','2','3']});
 const {attribute}=await sdk.attributes.getAttribute({identifier:{case:'attributeId',value:created.attribute.id}});
 const values=attribute.values.map(v=>v.value);
 assert(attribute.fqn==='https://'+name+'/attr/free'&&attribute.rule===AttributeRuleTypeEnum.ANY_OF&&values.join()==='1,2,3'&&attribute.namespace?.name===name,'attribute read back');
 const listed=await sdk.attributes.listAttributes({namespace:name});
 assert(listed.attributes.length===1&&listed.attributes[0].id===attribute.id,'attribute listed');
 const raw=JSON.parse(await sdk.call('policy.attributes.AttributesService','GetAttribute',JSON.stringify({attributeId:attribute.id})));
 assert(raw.attribute.fqn===attribute.fqn,'raw JSON call');
 let missing;
 try{await sdk.attributes.getAttribute({identifier:{case:'attributeId',value:'00000000-0000-0000-0000-000000000000'}});}catch(e){missing=e;}
 const cause=missing?.cause;
 assert(missing?.name==='ConnectError'&&missing.code===5&&cause instanceof g.OpenTDFError&&cause.connectCode==='not_found'&&cause.httpStatus===404,'missing attribute ConnectError');
 await write('services.json',JSON.stringify({namespace:name,namespaces_before:before.namespaces.length,namespaces_after:after.namespaces.length,attribute_fqn:attribute.fqn,values,
  not_found:{connectError:missing.code,rawMessage:missing.rawMessage,kind:cause.kind,connectCode:cause.connectCode,httpStatus:cause.httpStatus,message:cause.message},
  helpers:await helpers(sdk,cfg,name,attribute.fqn),autoconfigure:await autoconfigure(sdk,cfg,attribute)}));
}

// autoconfigure encrypts under a value of the attribute created, looked up
// (attributes) and offline (attributeValues): the attribute has no grants, so
// the plan is the base key or the default KAS.
async function autoconfigure(sdk,cfg,attribute){
 const value=attribute.fqn+'/value/1';
 const fqn=await g.newAttributeValueFqn(value);
 const offline={fqn:value,value:'1',attribute:{fqn:attribute.fqn,rule:attribute.rule,namespace:attribute.namespace}};
 const report={key:fqn.key};
 for(const [label,extra] of [['looked_up',{attributes:[value]}],['offline',{attributeValues:[offline]}]]){
  const object=await sdk.createTdf(new TextEncoder().encode(label),{kasInformation:[kasInformation(cfg)],...extra});
  const kaos=(await object.manifest()).encryptionInformation.keyAccess;
  await object.close();
  report[label]={kaos:kaos.length,kas:kaos[0].url,sid:kaos[0].sid??''};
 }
 return report;
}

// The reference's attribute helpers on the attribute services created,
// SDK.auth, and an SDK whose platform endpoint is unreachable but whose core
// connection is the platform.
async function helpers(sdk,cfg,namespace,fqn){
 const report={};
 report.attribute_exists=await sdk.attributeExists(fqn);
 report.attribute_missing=await sdk.attributeExists(fqn+'-missing');
 report.value_exists=await sdk.attributeValueExists(fqn+'/value/1');
 report.value_missing=await sdk.attributeValueExists(fqn+'/value/9');
 report.listed=(await sdk.listAttributes(namespace)).map(a=>a.fqn);
 await sdk.validateAttributes([fqn+'/value/1',fqn+'/value/2']);
 report.validate_ok=true;
 try{await sdk.validateAttributes([fqn+'/value/1',fqn+'/value/9']);}catch(e){assert(e instanceof g.OpenTDFError,'typed validate error');report.validate_missing={kind:e.kind,connectCode:e.connectCode,message:e.message};}
 assert(report.validate_missing,'validate missing');
 report.entity_attributes=await sdk.getEntityAttributes({id:'e1',entityType:{case:'clientId',value:'opentdf-sdk'}});
 const token=await sdk.auth().accessToken();
 report.access_token={present:token.value!=='',scheme:token.scheme};
 await using override=await g.SDK.create('http://127.0.0.1:9',{...optionsOf(cfg),coreEndpoint:cfg.PlatformURL});
 report.override={endpoint:(await override.conn()).endpoint,attribute_exists:await override.attributeExists(fqn)};
 return report;
}
