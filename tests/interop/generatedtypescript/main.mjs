// Independent native importing consumer; no shared SDK/compiler/reference import.
import{readFile,writeFile}from'node:fs/promises';import{resolve}from'node:path';import{pathToFileURL}from'node:url';import{acquireToken}from'./token-provider.mjs';
const[sdk,run,mode,name]=process.argv.slice(2);if(!name)throw new Error('consumer sdk run mode case');
const g=await import(pathToFileURL(resolve(process.env.TDF_TS_NODE_PACKAGE??process.env.TDF_TS_PACKAGE??resolve(sdk,'../goalchemy/out/typescript-tdf-library/sdk/dist/node-index.js'))).href);
const cfg=JSON.parse(await readFile(resolve(run,'config.json'),'utf8'));if(cfg.TimeoutMillis!==undefined)cfg.TimeoutMillis=BigInt(cfg.TimeoutMillis);
const read=n=>readFile(resolve(run,n));const write=(n,b)=>writeFile(resolve(run,n),b??new Uint8Array(),{mode:0o600});
function assert(ok,label){if(!ok)throw new Error(label);}
const provider=signal=>acquireToken(cfg,signal,name);
const call=cfg.TokenProviderName?{tokenProvider:provider}:{};
try{
 if(mode==='encrypt'){
  const options={Attributes:['https://example.com/attr/attr1/value/value1'],SegmentSize:16384n,HasSegmentSize:true};
  if(name==='metadata'||name.endsWith('-metadata')){options.Metadata=new TextEncoder().encode('{"source":"independent metadata","count":7}');options.IncludeMetadata=true;}
  if(name==='empty-metadata'||name.endsWith('-empty-metadata')){options.Metadata=new Uint8Array();options.IncludeMetadata=true;}
  if(name==='hs256'||name.endsWith('-hs256'))options.SegmentHashAlgorithm='HS256';
  if(name==='denied'||name.endsWith('-denied'))options.Attributes=['https://example.com/attr/attr1/value/value2'];
  const output=await g.encrypt(cfg,new Uint8Array(await read(name+'.input')),options,call);await write(name+'.generated.tdf',output);
 }else if(mode==='decrypt'){
  const output=await g.decrypt(cfg,new Uint8Array(await read(name+'.tdf')),call);await write(name+'.out',output.Payload);await write(name+'.metadata',output.Metadata??new Uint8Array());await write(name+'.manifest',output.ManifestJSON);await write(name+'.presence',String(output.HasMetadata));
 }else if(mode==='negative'){
  let rejected=false;try{await g.decrypt(cfg,new Uint8Array(await read(name+'.tdf')),call);}catch(e){assert(e instanceof g.TDFError,'typed SDK error');rejected=true;await write(name+'.error.json',JSON.stringify({kind:e.kind,code:e.code,operation:e.operation,httpStatus:Number(e.httpStatus),causeCategory:e.causeCategory}));}assert(rejected,'negative returned output');
 }else if(mode==='repeat'){
  const input=new Uint8Array([0,255,128,1]),metadata=new Uint8Array([0,255]),opts={Attributes:['https://example.com/attr/attr1/value/value1'],Metadata:metadata,IncludeMetadata:true,MimeType:'application/日本語'};
  let rejected=false;try{await g.encrypt({...cfg,ClientSecret:''},input,opts);}catch(e){assert(e instanceof g.TDFError,'constructor typed');rejected=true;}assert(rejected,'constructor rejected');
  const archive=await g.encrypt(cfg,input,opts,call),first=await g.decrypt(cfg,archive,call),second=await g.decrypt(cfg,archive,call);assert(first.Payload.every((b,i)=>b===input[i])&&first.Metadata[1]===255,'bytes retained');assert(first.ManifestJSON.includes('日本語'),'UTF8 ergonomic manifest');metadata[0]=1;second.Payload[0]=1;assert(first.Payload[0]===0&&first.Metadata[0]===0,'outputs independent');
  const started=Promise.withResolvers(),abort=new AbortController();const canceled=g.encrypt({...cfg,ClientID:'',ClientSecret:''},input,{}, {signal:abort.signal,tokenProvider:async signal=>{started.resolve();await new Promise(r=>signal.addEventListener('abort',r,{once:true}));throw new Error('canceled');}});await started.promise;abort.abort('test cancel');let failed=false;try{await canceled;}catch(e){failed=e instanceof g.TDFError&&e.kind==='canceled';}assert(failed,'real active provider cancel');await g.decrypt(cfg,archive,call);
 }else throw new Error('unknown mode');
}catch(e){console.error('consumer failure',mode,name,e instanceof g.TDFError?{kind:e.kind,code:e.code,operation:e.operation,cause:e.causeCategory}:e.name+': '+e.message);process.exitCode=1;}
