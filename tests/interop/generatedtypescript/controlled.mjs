// Rejecting HTTP/TLS fixtures only; positive KAS evidence is the real platform.
import{createServer as httpServer}from'node:http';import{createServer as httpsServer}from'node:https';import{readFile,writeFile,mkdir}from'node:fs/promises';import{resolve}from'node:path';import{execFileSync}from'node:child_process';import{pathToFileURL}from'node:url';
const sdk=resolve(import.meta.dirname,'../../..'),run=resolve(sdk,'.local/typescript-tdf-library/controlled');await mkdir(run,{recursive:true});const g=await import(pathToFileURL(resolve(sdk,'../goalchemy/out/typescript-tdf-library/sdk/dist/index.js')).href),archive=new Uint8Array(await readFile(resolve(sdk,'.local/typescript-tdf-library/basic/binary.generated.tdf'))),rows=[];
function assert(ok,label){if(!ok)throw new Error(label);}
const key=resolve(run,'tls-key.pem'),cert=resolve(run,'tls-cert.pem');execFileSync('openssl',['req','-x509','-newkey','rsa:2048','-nodes','-keyout',key,'-out',cert,'-days','1','-subj','/CN=localhost','-addext','subjectAltName=DNS:localhost'],{stdio:'ignore'});
for(const mode of ['http401','http403','redirect','content-type','malformed-response','oversized-response','untrusted-tls','caller-deadline','source-deadline','active-http-cancel']){
 let requests=0,redirected=0;const entered=Promise.withResolvers(),released=Promise.withResolvers();
 const handler=async(req,res)=>{requests++;for await(const _ of req){}if(req.url==='/leak')redirected++;res.on('close',()=>released.resolve());res.setHeader('Content-Type','application/json');
  if(mode==='http401'||mode==='http403'){res.writeHead(mode==='http401'?401:403);res.end(JSON.stringify({code:'fixture-rejected',message:'診断 café'}));}
  else if(mode==='redirect'){res.writeHead(307,{Location:'/leak'});res.end();}
  else if(mode==='content-type'){res.setHeader('Content-Type','text/html');res.end('{}');}
  else if(mode==='malformed-response')res.end('{bad');
  else if(mode==='oversized-response')res.end(' '.repeat((1<<20)+1));
  else if(['caller-deadline','source-deadline','active-http-cancel'].includes(mode))entered.resolve();else throw new Error('TLS should reject before request');
 };
 const server=mode==='untrusted-tls'?httpsServer({key:await readFile(key),cert:await readFile(cert)},handler):httpServer(handler);await new Promise(r=>server.listen(0,'localhost',r));const endpoint=(mode==='untrusted-tls'?'https':'http')+'://localhost:'+server.address().port;
 const cfg={PlatformURL:endpoint,KASURL:'http://localhost:8080/kas',AllowedKAS:[{URL:'http://localhost:8080/kas',APIBaseURL:endpoint}],AllowHTTP:true,...(mode==='source-deadline'?{TimeoutMillis:100n}:{})},abort=new AbortController();let alarm;
 if(mode==='caller-deadline')alarm=setTimeout(()=>abort.abort('caller deadline'),100);
 if(mode==='active-http-cancel')entered.promise.then(()=>abort.abort('active cancel'));
 const start=performance.now();let error;
 try{await g.decrypt(cfg,archive,{signal:abort.signal,tokenProvider:async()=>({value:'negative-fixture-token',scheme:'Bearer',expiresAt:BigInt(Math.floor(Date.now()/1000)+300)})});}catch(e){error=e;}
 clearTimeout(alarm);assert(error instanceof g.TDFError,'typed rejection '+mode);const wanted={'http401':'unauthenticated','http403':'denied',redirect:'http_status','content-type':'invalid_content_type','malformed-response':'invalid_json','oversized-response':'transport','untrusted-tls':'transport','source-deadline':'transport','caller-deadline':'canceled','active-http-cancel':'canceled'}[mode];assert(error.code===wanted,'category '+mode+': '+error.code);
 if(mode==='http401'||mode==='http403')assert(error.serverCode==='fixture-rejected'&&error.serverMessage==='診断 café','UTF8 error adapter');
 if(mode==='redirect')assert(error.httpStatus===307n&&redirected===0,'manual redirect/no credential follow');if(mode==='untrusted-tls')assert(requests===0,'native TLS verification');
 if(['caller-deadline','source-deadline','active-http-cancel'].includes(mode)){await released.promise;assert(performance.now()-start<1500,'real deadline/release');}
 rows.push({case:mode,code:error.code,kind:error.kind,httpStatus:Number(error.httpStatus),causeCategory:error.causeCategory,requests,redirected,zeroOutput:true});server.closeAllConnections();await new Promise(r=>server.close(r));console.log('PASS controlled generated SDK',mode);
}
await writeFile(resolve(run,'results.json'),JSON.stringify({scope:'controlled rejecting endpoints through actual compiled SDK, no mock positive KAS',cases:rows},null,2));
