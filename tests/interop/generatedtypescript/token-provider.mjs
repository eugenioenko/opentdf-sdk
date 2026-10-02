// Independent native OAuth/DPoP provider. Server credentials stay host-side.
import{createPrivateKey,createPublicKey,generateKeyPairSync,createHash,randomBytes,sign}from'node:crypto';
const b64=b=>Buffer.from(b).toString('base64url');
export async function acquireToken(cfg,signal,name){
 if(name==='invalid-token')return {value:'invalid-token',scheme:'Bearer',expiresAt:BigInt(Math.floor(Date.now()/1000)+300)};
 if(name==='expired-token')return {value:'expired-token',scheme:'Bearer',expiresAt:BigInt(Math.floor(Date.now()/1000)-1)};
 if(name==='provider-reject')throw new Error('provider rejected');
 const endpoint='http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token';let nonce='';let key,jwk,alg,confirmationJKT;
 if(cfg.DPoP){key=name==='mismatched-provider'?generateKeyPairSync('ec',{namedCurve:'prime256v1'}).privateKey:createPrivateKey(cfg.AuthPrivateKeyPEM);const full=createPublicKey(key).export({format:'jwk'});jwk=full.kty==='RSA'?{e:full.e,kty:'RSA',n:full.n}:{crv:'P-256',kty:'EC',x:full.x,y:full.y};alg=full.kty==='RSA'?'RS256':'ES256';confirmationJKT=b64(createHash('sha256').update(JSON.stringify(jwk)).digest());}
 for(let attempt=0;attempt<3;attempt++){
  const headers={'Content-Type':'application/x-www-form-urlencoded'};
  if(cfg.DPoP){const header={typ:'dpop+jwt',alg,jwk},claims={htu:endpoint,htm:'POST',iat:Math.floor(Date.now()/1000),jti:b64(randomBytes(16)),...(nonce?{nonce}: {})};const unsigned=b64(JSON.stringify(header))+'.'+b64(JSON.stringify(claims));headers.DPoP=unsigned+'.'+b64(sign('sha256',Buffer.from(unsigned),{key,dsaEncoding:'ieee-p1363'}));}
  const response=await fetch(endpoint,{method:'POST',headers,body:new URLSearchParams({grant_type:'client_credentials',client_id:'opentdf-sdk',client_secret:'secret'}),signal,redirect:'manual'});const token=await response.json();const next=response.headers.get('DPoP-Nonce');if((response.status===400||response.status===401)&&next&&next!==nonce){nonce=next;continue;}if(!response.ok)throw new Error('provider endpoint');return {value:token.access_token,scheme:cfg.DPoP?'DPoP':'Bearer',expiresAt:BigInt(Math.floor(Date.now()/1000)+token.expires_in),confirmationJKT};
 }throw new Error('provider nonce retries');
}
