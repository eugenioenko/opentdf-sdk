// Actual Chromium and installed portable SDK. Test-only OAuth broker stays host-side.
import {createServer} from 'node:http';
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {resolve} from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';
import {spawnSync} from 'node:child_process';
import {acquireToken} from '../generatedtypescript/token-provider.mjs';

const [profile,packagesBase,outputBase]=process.argv.slice(2);
if(!['basic','ec','dpop'].includes(profile)||!packagesBase||!outputBase)throw new Error('profile packages-base output-base required');
const sdk=resolve(import.meta.dirname,'../../..'),base=resolve(outputBase),run=resolve(base,'typescript-browser',profile);
await mkdir(run,{recursive:true});
const tooling=process.env.TDF_BROWSER_TOOLING;
if(!tooling)throw new Error('pinned TDF_BROWSER_TOOLING required');
const {build}=await import(pathToFileURL(resolve(tooling,'esbuild/lib/main.js')).href);
const {chromium}=await import(pathToFileURL(resolve(tooling,'playwright/index.mjs')).href);
const installed=resolve(packagesBase,'consumers/typescript/package/dist/index.js');
const bundle=await build({entryPoints:[installed],bundle:true,platform:'browser',format:'esm',write:false,metafile:true});
for(const input of Object.keys(bundle.metafile.inputs)){if(/node:|\bBuffer\b|\bprocess\b/.test(await readFile(input,'utf8')))throw new Error('nonportable production member '+input);}
await writeFile(resolve(run,'browser-graph.json'),JSON.stringify(bundle.metafile,null,2));
const hash=b=>createHash('sha256').update(b).digest('hex');
let brokerRequests=0;
const server=createServer(async(req,res)=>{try{
 if(req.url==='/sdk.js'){res.setHeader('Content-Type','text/javascript');res.end(bundle.outputFiles[0].text);return;}
 if(req.url==='/token'&&req.method==='POST'){
  brokerRequests++;const chunks=[];for await(const chunk of req)chunks.push(chunk);
  const config=JSON.parse(Buffer.concat(chunks));if(config.ClientSecret||config.ClientID)throw new Error('browser carried credentials');
  const controller=new AbortController();res.on('close',()=>controller.abort());
  const token=await acquireToken(config,controller.signal,'browser');res.setHeader('Content-Type','application/json');res.end(JSON.stringify({...token,expiresAt:String(token.expiresAt)}));return;
 }
 res.setHeader('Content-Type','text/html');res.end('<script type="module">import * as sdk from "/sdk.js";window.sdk=sdk;window.ready=true;</script>');
}catch{res.writeHead(500);res.end('fixture provider failed');}});
await new Promise((ready,reject)=>{server.once('error',reject);server.listen(5173,'localhost',ready);});
let browser;
const rows=[],events=[];
function invoke(label,args){const result=spawnSync(args[0],args.slice(1),{cwd:sdk,timeout:120000,encoding:'buffer'});const status=result.status??-1;if(status!==0)throw new Error(label+' failed '+status);return writeFile(resolve(run,label+'.status'),String(status)+'\n');}
try{
 browser=await chromium.launch({headless:true,args:['--no-sandbox']});const version=await browser.version();
 const page=await browser.newPage();await page.goto('http://localhost:5173');await page.waitForFunction(()=>window.ready);
 await page.evaluate(()=>{window.observations=[];const original=window.fetch;window.fetch=async(...args)=>{const response=await original(...args);window.observations.push({url:String(args[0]).split('?')[0],status:response.status,type:response.type,nonceReadable:response.headers.get('DPoP-Nonce')!==null});return response;};window.options=config=>({tokenProvider:async signal=>{const response=await fetch('/token',{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(config),signal});if(!response.ok)throw new Error('provider failed');const token=await response.json();return {...token,expiresAt:BigInt(token.expiresAt)};}});});
 const caseName=profile==='basic'?'binary':'empty-metadata';
 const payload=caseName==='binary'?Uint8Array.from([...Array.from({length:768},(_,i)=>i%256),0,255]):new TextEncoder().encode('empty metadata');
 const combos=profile==='basic'?[['rsa:2048','rsa:2048','ES256']]:['rsa:2048','ec:secp256r1'].flatMap(w=>['rsa:2048','ec:secp256r1'].flatMap(s=>['ES256','RS256'].map(a=>[w,s,a])));
 for(const[wrap,session,auth]of combos){
  const name=wrap.slice(0,2)+'-'+auth.toLowerCase()+'-'+session.slice(0,2)+'-'+caseName;
  const config={PlatformURL:'http://localhost:8080',KASURL:'http://localhost:8080/kas',IssuerURL:'http://localhost:8888/auth/realms/opentdf',AllowHTTP:true,KASAlgorithm:wrap,SessionAlgorithm:session,AuthAlgorithm:auth,DPoP:profile==='dpop',AllowedKAS:[{URL:'http://localhost:8080/kas',APIBaseURL:'http://localhost:8080'}]};
  if(profile!=='basic'){config.KASPublicKeyPEM=await readFile(resolve(base,'keys',wrap.slice(0,2)+'-public.pem'),'utf8');config.KID=wrap.startsWith('rsa')?'profile-r1':'profile-e1';config.AuthPrivateKeyPEM=await readFile(resolve(base,'keys','auth-'+auth+'.pem'),'utf8');}
  await writeFile(resolve(run,name+'.input'),payload);
  await writeFile(resolve(run,'config.json'),JSON.stringify({...config,ClientID:'opentdf-sdk',ClientSecret:'secret'}),{mode:0o600});
  const archive=Uint8Array.from(await page.evaluate(async({config,payload,caseName})=>Array.from(await window.sdk.encrypt(config,new Uint8Array(payload),{Attributes:['https://example.com/attr/attr1/value/value1'],SegmentSize:16384n,HasSegmentSize:true,IncludeMetadata:caseName==='empty-metadata',Metadata:new Uint8Array()},window.options(config))),{config,payload:Array.from(payload),caseName}));
  await writeFile(resolve(run,name+'.generated.tdf'),archive);
  const reference=resolve(sdk,'.local/go-tdf-library/stock-go');
  await invoke('stock-go-decrypt-'+name,[reference,run,'decrypt',name,session]);
  if(hash(await readFile(resolve(run,name+'.stock-go.out')))!==hash(payload)||(await readFile(resolve(run,name+'.stock-go.metadata'))).length!==0)throw new Error('stock Go byte mismatch');
  rows.push({producer:'typescript-browser',consumer:'stock-go',case:caseName,wrap,session,auth});
  if(profile!=='dpop'){
   const output=resolve(run,name+'.stock-web.out');
   await invoke('stock-web-decrypt-'+name,['node',resolve(sdk,'.local/web-cli/bin/opentdf.mjs'),'decrypt',resolve(run,name+'.generated.tdf'),'--rewrapKeyType',session,'--allowList','http://localhost:8080','--output',output,'--platformUrl','http://localhost:8080','--kasEndpoint','http://localhost:8080/kas','--oidcEndpoint','http://localhost:8888/auth/realms/opentdf','--clientId','opentdf-sdk','--clientSecret','secret','--logLevel','error']);
   if(hash(await readFile(output))!==hash(payload))throw new Error('stock Web byte mismatch');
   rows.push({producer:'typescript-browser',consumer:'stock-web',case:caseName,wrap,session,auth,metadata_reader_limitation:true});
  }
  const producerRun=resolve(base,'typescript',profile==='dpop'?'ec':profile);
  for(const producer of ['go','web','generated']){
   const source=producer==='generated'?archive:new Uint8Array(await readFile(resolve(producerRun,wrap.slice(0,2)+'-'+caseName+'.'+producer+'.tdf')));
   const output=await page.evaluate(async({config,source})=>{const result=await window.sdk.decrypt(config,new Uint8Array(source),window.options(config));return{payload:Array.from(result.Payload),metadata:Array.from(result.Metadata??[]),presence:result.HasMetadata,manifest:result.ManifestJSON};},{config,source:Array.from(source)});
   const manifest=JSON.parse(output.manifest),presence=manifest.encryptionInformation.keyAccess.some(key=>Boolean(key.encryptedMetadata));
   if(hash(Uint8Array.from(output.payload))!==hash(payload)||output.metadata.length!==0||output.presence!==presence||(producer==='generated'&&presence!==(caseName==='empty-metadata')))throw new Error('browser bytes/metadata presence '+producer);
   await writeFile(resolve(run,name+'.'+producer+'.out'),Uint8Array.from(output.payload));
   await writeFile(resolve(run,name+'.'+producer+'.metadata'),Uint8Array.from(output.metadata));
   await writeFile(resolve(run,name+'.'+producer+'.presence'),String(output.presence));
   rows.push({producer:producer+((profile==='dpop'&&producer!=='generated')?'-format-under-bearer':''),consumer:'typescript-browser',case:caseName,wrap,session,auth,has_metadata:presence});
  }
  const safeConfig={...config};if(safeConfig.AuthPrivateKeyPEM)safeConfig.AuthPrivateKeyPEM='<private-input-sha256:'+hash(safeConfig.AuthPrivateKeyPEM)+'>';
  const artifacts={};for(const suffix of ['input','generated.tdf','stock-go.out','stock-go.metadata','go.out','go.metadata','go.presence','web.out','web.metadata','web.presence','generated.out','generated.metadata','generated.presence']){const bytes=await readFile(resolve(run,name+'.'+suffix));artifacts[suffix]={sha256:hash(bytes),bytes:bytes.length};}
  events.push({name,terminal_status:0,configuration:safeConfig,artifacts});
  console.log('PASS actual installed browser',profile,name);
 }
 const observations=await page.evaluate(()=>window.observations);
 if(profile==='dpop'&&!observations.some(value=>value.url.includes('8080')&&value.status===401&&value.nonceReadable))throw new Error('browser CORS-readable nonce challenge missing');
 await writeFile(resolve(run,'results.json'),JSON.stringify({profile,status:0,scope:'actual focused Chromium final installed portable package; historical full browser matrices remain separate',browser:version,pairs:rows,events,brokerRequests,graphInputs:Object.keys(bundle.metafile.inputs).length,observations},null,2));
 console.log('PASS focused actual browser',profile,version,rows.length,'comparisons');
}finally{await browser?.close();server.closeAllConnections();await new Promise(ready=>server.close(ready));}
