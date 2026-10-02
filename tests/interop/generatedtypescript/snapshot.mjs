// Actual SDK ownership supplement: native Buffer is intentionally a borrowed-
// slice Uint8Array subclass. Production must copy its elements at invocation.
import{readFile,writeFile}from'node:fs/promises';import{resolve}from'node:path';import{pathToFileURL}from'node:url';import{acquireToken}from'./token-provider.mjs';
const config=JSON.parse(await readFile(process.argv[2],'utf8')),destination=process.argv[3];
const sdk=await import(pathToFileURL(resolve(import.meta.dirname,'../../../../goalchemy/out/typescript-tdf-library/sdk/dist/index.js')).href);
const provider=signal=>acquireToken(config,signal,'snapshot');
const payload=Buffer.from([0,255,1,7]),metadata=Buffer.from([128,255,0]);const work=sdk.encrypt(config,payload,{Metadata:metadata,IncludeMetadata:true},{tokenProvider:provider});payload.fill(44);metadata.fill(44);const archive=await work;
const result=await sdk.decrypt(config,archive,{tokenProvider:provider});if(String([...result.Payload])!=='0,255,1,7'||String([...result.Metadata])!=='128,255,0'||!result.HasMetadata)throw new Error('Buffer payload/metadata snapshot');
const gate=Promise.withResolvers(),entered=Promise.withResolvers();const blocking=sdk.decrypt(config,archive,{tokenProvider:async signal=>{entered.resolve();await gate.promise;return provider(signal);}});await entered.promise;
const input=Buffer.from(archive),queued=sdk.decrypt(config,input,{tokenProvider:provider});input.fill(44);gate.resolve();await blocking;const copied=await queued;if(String([...copied.Payload])!=='0,255,1,7'||String([...copied.Metadata])!=='128,255,0')throw new Error('queued Buffer archive snapshot');
await writeFile(destination,JSON.stringify({payload:[...result.Payload],metadata:[...result.Metadata],hasMetadata:result.HasMetadata,queuedArchive:true,nativeProvider:true,scope:'actual generated SDK Buffer invocation/queued ownership through real KAS'},null,2));console.log('PASS actual SDK Buffer payload/metadata/queued archive snapshots');
