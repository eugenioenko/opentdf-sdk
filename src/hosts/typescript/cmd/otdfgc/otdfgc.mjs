#!/usr/bin/env node
// otdfgc is the native Node CLI over the generated TypeScript package. It
// implements the xtest CLI contract of docs/full-sdk-plan.md exactly as the
// Go otdfgc does: one SDK per invocation, created with --host and the client
// credentials; every flag this build cannot honour exits 2 as unsupported.
import {readFile,writeFile,rename,unlink} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {inflateRawSync} from 'node:zlib';

const EXIT_SUCCESS=0,EXIT_ERROR=1,EXIT_USAGE=2;
const TARGET='typescript';
const TIMEOUT_MILLIS=120000;
const USAGE=`Usage:
  otdfgc encrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--attr <fqn,...>] [--mime-type <t>] [--wrapping-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]
      [--target-mode 4.3.0] [--policy-mode encrypted] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc decrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--no-verify-assertions] [--session-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]
      [--kas-allowlist <url,...|*>] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc inspect <in>
  otdfgc supports <feature>
  otdfgc --version [--json]
`;

const packagePath=process.env.OTDFGC_TS_PACKAGE??resolve(import.meta.dirname,'package/dist/node-index.js');
const g=await import(pathToFileURL(packagePath).href);

const commonFlags=[['out'],['host'],['with-client-creds'],['with-client-creds-file'],['dpop'],['dpop-key'],['tls-no-verify',true],['log-level']];
const encryptFlags=[['attr'],['mime-type'],['with-assertions'],['wrapping-key-algorithm'],['target-mode'],['policy-mode'],['ecdsa-binding',true],...commonFlags];
const decryptFlags=[['with-assertion-verification-keys'],['no-verify-assertions',true],['session-key-algorithm'],['kas-allowlist'],...commonFlags];
const encryptUnsupported=[['with-assertions'],['ecdsa-binding'],['target-mode',['4.3.0']],['policy-mode',['encrypted']],['wrapping-key-algorithm',['rsa:2048','ec:secp256r1','rsa:4096','ec:secp384r1','ec:secp521r1']],['dpop',['ES256','ES384','ES512','RS256','RS384','RS512']],['with-client-creds-file']];
const decryptUnsupported=[['with-assertion-verification-keys'],['session-key-algorithm',['rsa:2048','ec:secp256r1','rsa:4096','ec:secp384r1','ec:secp521r1']],['dpop',['ES256','ES384','ES512','RS256','RS384','RS512']],['with-client-creds-file']];

function parseFlags(args,spec){
 const specs=new Map(spec.map(([name,bool])=>[name,Boolean(bool)]));
 const values={},set=new Set(),positional=[];
 for(let i=0;i<args.length;i++){
  const a=args[i];
  if(!a.startsWith('-')||a==='-'){positional.push(a);continue;}
  let name=a.replace(/^-+/,''),value='',inline=false;
  const eq=name.indexOf('=');
  if(eq>=0){value=name.slice(eq+1);name=name.slice(0,eq);inline=true;}
  if(name==='o')name='out';
  if(!specs.has(name))throw new Error('unknown flag: '+a);
  if(specs.get(name)){
   if(inline&&value!=='true'&&value!=='false')throw new Error('flag --'+name+' takes no value');
   if(!inline||value==='true')set.add(name);
   continue;
  }
  if(!inline){if(i+1>=args.length)throw new Error('flag --'+name+' needs a value');value=args[++i];}
  values[name]=value;set.add(name);
 }
 return {values,set,positional,value:n=>values[n]??'',flag:n=>set.has(n)};
}
function unsupported(p,rules){
 for(const [name,allowed] of rules){
  if(!p.set.has(name))continue;
  if(!allowed)return '--'+name;
  const v=p.values[name]??'';
  if(!allowed.includes(v))return '--'+name+' '+v;
 }
 return '';
}
function splitList(s){return s.split(',').map(x=>x.trim()).filter(Boolean);}
function describe(e){
 if(!(e instanceof g.OpenTDFError))return String(e?.message??e);
 let msg=e.message;
 if(e.requiredObligations.length>0)msg+='\nrequired obligations: ['+e.requiredObligations.join(' ')+']';
 return msg;
}
function describeNew(e,host){
 if(!(e instanceof g.OpenTDFError))return String(e?.message??e);
 if(e.kind==='auth')return 'Failed to authenticate with flag-provided client credentials. ('+e.message+')';
 if(e.message.includes('platform unreachable or not responding')||e.kind==='transport'||e.kind==='deadline_exceeded')return "Failed to connect to the platform. Is the platform accepting connections at '"+host+"'? ("+e.message+')';
 return e.message;
}
async function options(p){
 const host=p.value('host').replace(/\/+$/,'');
 if(!host)return {code:EXIT_USAGE,error:'--host is required\n'+USAGE};
 const creds=p.value('with-client-creds');
 if(!creds)return {code:EXIT_USAGE,error:'--with-client-creds is required\n'+USAGE};
 let c;
 try{c=JSON.parse(creds);}catch{c=null;}
 if(!c||typeof c.clientId!=='string'||!c.clientId||typeof c.clientSecret!=='string'||!c.clientSecret)return {code:EXIT_USAGE,error:'--with-client-creds must be JSON with clientId and clientSecret'};
 const opts={clientId:c.clientId,clientSecret:c.clientSecret};
 if(host.toLowerCase().startsWith('http://'))opts.insecurePlaintext=true;
 if(p.flag('tls-no-verify'))opts.insecureSkipVerify=true;
 if(p.value('dpop'))opts.dpopAlgorithm=p.value('dpop');
 if(p.value('dpop-key')){
  if(!p.value('dpop'))return {code:EXIT_USAGE,error:'--dpop-key needs --dpop'};
  try{opts.dpopKeyPem=await readFile(p.value('dpop-key'),'utf8');}catch(e){return {code:EXIT_ERROR,error:'Failed to read DPoP key '+p.value('dpop-key')+': '+e.message};}
 }
 return {host,opts};
}
async function newSDK(p,signal){
 const o=await options(p);
 if(o.error){process.stderr.write(o.error+'\n');return {code:o.code};}
 try{return {sdk:await g.SDK.create(o.host,o.opts,{signal}),host:o.host};}
 catch(e){process.stderr.write(describeNew(e,o.host)+'\n');return {code:EXIT_ERROR};}
}
async function writeOut(path,data){
 const tmp=path+'.tmp';
 await writeFile(tmp,data);
 try{await rename(tmp,path);}catch(e){await unlink(tmp).catch(()=>{});throw e;}
}

async function versionInfo(){
 return {sdk_version:await g.version(),schema_version:await g.tdfSpecVersion(),supported_features:await g.supportedFeatures(),target:TARGET};
}
async function version(args){
 let asJSON=false;
 for(const a of args){if(a==='--json')asJSON=true;else{process.stderr.write('unknown flag: '+a+'\n');return EXIT_USAGE;}}
 let info;
 try{info=await versionInfo();}catch(e){process.stderr.write('Failed to read version: '+describe(e)+'\n');return EXIT_ERROR;}
 if(asJSON){process.stdout.write(JSON.stringify(info)+'\n');return EXIT_SUCCESS;}
 process.stdout.write(`otdfgc (${TARGET}) sdk ${info.sdk_version} tdf-spec ${info.schema_version} features ${info.supported_features.join(',')}\n`);
 return EXIT_SUCCESS;
}
async function supports(args){
 if(args.length!==1||args[0].startsWith('-')){process.stderr.write('usage: otdfgc supports <feature>\n');return EXIT_USAGE;}
 let features;
 try{features=await g.supportedFeatures();}catch(e){process.stderr.write('Failed to read supported features: '+describe(e)+'\n');return EXIT_ERROR;}
 if(features.includes(args[0]))return EXIT_SUCCESS;
 process.stdout.write(args[0]+' unsupported\n');
 return EXIT_ERROR;
}

// readManifest extracts 0.manifest.json from the ZIP container natively, as
// the Go CLI does with archive/zip: central directory first, then the entry.
function readManifest(data){
 const dv=new DataView(data.buffer,data.byteOffset,data.byteLength);
 let eocd=-1;
 for(let i=data.length-22;i>=Math.max(0,data.length-22-65535);i--){if(dv.getUint32(i,true)===0x06054b50){eocd=i;break;}}
 if(eocd<0)throw new Error('zip: not a valid zip file');
 let entries=dv.getUint16(eocd+10,true),offset=dv.getUint32(eocd+16,true);
 if(entries===0xffff||offset===0xffffffff){
  const locator=eocd-20;
  if(locator<0||dv.getUint32(locator,true)!==0x07064b50)throw new Error('zip: not a valid zip file');
  const zip64=Number(dv.getBigUint64(locator+8,true));
  if(dv.getUint32(zip64,true)!==0x06064b50)throw new Error('zip: not a valid zip file');
  entries=Number(dv.getBigUint64(zip64+32,true));offset=Number(dv.getBigUint64(zip64+48,true));
 }
 for(let n=0;n<entries;n++){
  if(dv.getUint32(offset,true)!==0x02014b50)throw new Error('zip: not a valid zip file');
  const method=dv.getUint16(offset+10,true),nameLength=dv.getUint16(offset+28,true),extraLength=dv.getUint16(offset+30,true),commentLength=dv.getUint16(offset+32,true);
  let compressed=dv.getUint32(offset+20,true),local=dv.getUint32(offset+42,true);
  const name=new TextDecoder().decode(data.subarray(offset+46,offset+46+nameLength));
  if(compressed===0xffffffff||local===0xffffffff){
   let p=offset+46+nameLength;const end=p+extraLength;
   while(p+4<=end){const id=dv.getUint16(p,true),size=dv.getUint16(p+2,true);if(id===1){let q=p+4;if(dv.getUint32(offset+24,true)===0xffffffff)q+=8;if(compressed===0xffffffff){compressed=Number(dv.getBigUint64(q,true));q+=8;}if(local===0xffffffff)local=Number(dv.getBigUint64(q,true));break;}p+=4+size;}
  }
  if(name==='0.manifest.json'){
   if(dv.getUint32(local,true)!==0x04034b50)throw new Error('zip: not a valid zip file');
   const start=local+30+dv.getUint16(local+26,true)+dv.getUint16(local+28,true);
   const raw=data.subarray(start,start+compressed);
   if(method===0)return raw;
   if(method===8)return inflateRawSync(raw);
   throw new Error('zip: unsupported compression method '+method);
  }
  offset+=46+nameLength+extraLength+commentLength;
 }
 throw new Error('manifest entry not found');
}
async function inspect(args){
 if(args.length!==1||args[0].startsWith('-')){process.stderr.write('usage: otdfgc inspect <in>\n');return EXIT_USAGE;}
 let manifest;
 try{manifest=readManifest(new Uint8Array(await readFile(args[0])));}catch(e){process.stderr.write('not a valid TDF: '+e.message+'\n');return EXIT_ERROR;}
 try{process.stdout.write(JSON.stringify(JSON.parse(new TextDecoder('utf-8',{fatal:true}).decode(manifest)),null,2)+'\n');}
 catch(e){process.stderr.write('failed to inspect TDF: '+e.message+'\n');return EXIT_ERROR;}
 return EXIT_SUCCESS;
}

async function encrypt(args,signal){
 let p;
 try{p=parseFlags(args,encryptFlags);}catch(e){process.stderr.write(e.message+'\n'+USAGE);return EXIT_USAGE;}
 const reason=unsupported(p,encryptUnsupported);
 if(reason){process.stderr.write('unsupported in this build: '+reason+'\n');return EXIT_USAGE;}
 if(p.positional.length!==1||!p.value('out')){process.stderr.write('encrypt needs exactly one input file and --out\n'+USAGE);return EXIT_USAGE;}
 let input;
 try{input=new Uint8Array(await readFile(p.positional[0]));}catch(e){process.stderr.write('Failed to read '+p.positional[0]+': '+e.message+'\n');return EXIT_ERROR;}
 const created=await newSDK(p,signal);
 if(!created.sdk)return created.code;
 try{
  // As otdfctl: the platform's KAS is the default key access server and
  // attributes drive autoconfigure.
  const options={kasInformation:[{url:created.host+'/kas'}],mimeType:p.value('mime-type')};
  if(p.value('attr'))options.attributes=splitList(p.value('attr'));
  if(p.value('wrapping-key-algorithm'))options.wrappingKeyAlgorithm=p.value('wrapping-key-algorithm');
  if(p.value('target-mode'))options.targetMode=p.value('target-mode');
  let data;
  try{data=await created.sdk.encryptBytes(input,options,{signal});}catch(e){process.stderr.write('Failed to encrypt: '+describe(e)+'\n');return EXIT_ERROR;}
  try{await writeOut(p.value('out'),data);}catch(e){process.stderr.write('Failed to write encrypted file '+p.value('out')+': '+e.message+'\n');return EXIT_ERROR;}
  return EXIT_SUCCESS;
 }finally{await created.sdk.close().catch(()=>{});}
}
async function decrypt(args,signal){
 let p;
 try{p=parseFlags(args,decryptFlags);}catch(e){process.stderr.write(e.message+'\n'+USAGE);return EXIT_USAGE;}
 const reason=unsupported(p,decryptUnsupported);
 if(reason){process.stderr.write('unsupported in this build: '+reason+'\n');return EXIT_USAGE;}
 if(p.positional.length!==1||!p.value('out')){process.stderr.write('decrypt needs exactly one input file and --out\n'+USAGE);return EXIT_USAGE;}
 let input;
 try{input=new Uint8Array(await readFile(p.positional[0]));}catch(e){process.stderr.write('Failed to read file: '+e.message+'\n');return EXIT_ERROR;}
 const created=await newSDK(p,signal);
 if(!created.sdk)return created.code;
 try{
  const options={disableAssertionVerification:p.flag('no-verify-assertions')};
  if(p.value('session-key-algorithm'))options.sessionKeyType=p.value('session-key-algorithm');
  // otdfctl: "*" alone ignores the allowlist; any other list is explicit.
  const allowlist=splitList(p.value('kas-allowlist'));
  if(allowlist.length===1&&allowlist[0]==='*')options.ignoreAllowlist=true;
  else if(allowlist.length>0)options.kasAllowlist=allowlist;
  let plaintext;
  try{plaintext=await created.sdk.decryptBytes(input,options,{signal});}catch(e){process.stderr.write('Failed to decrypt file: '+describe(e)+'\n');return EXIT_ERROR;}
  try{await writeOut(p.value('out'),plaintext);}catch(e){process.stderr.write('Failed to write decrypted data to file: '+e.message+'\n');return EXIT_ERROR;}
  return EXIT_SUCCESS;
 }finally{await created.sdk.close().catch(()=>{});}
}

async function run(args){
 if(args.length===0){process.stderr.write(USAGE);return EXIT_USAGE;}
 const signal=AbortSignal.timeout(TIMEOUT_MILLIS);
 switch(args[0]){
  case '--version':case 'version':return version(args.slice(1));
  case 'supports':return supports(args.slice(1));
  case 'inspect':return inspect(args.slice(1));
  case 'encrypt':return encrypt(args.slice(1),signal);
  case 'decrypt':return decrypt(args.slice(1),signal);
  case 'help':case '--help':case '-h':process.stdout.write(USAGE);return EXIT_SUCCESS;
 }
 process.stderr.write('unknown command: '+args[0]+'\n'+USAGE);
 return EXIT_USAGE;
}
process.exitCode=await run(process.argv.slice(2));
