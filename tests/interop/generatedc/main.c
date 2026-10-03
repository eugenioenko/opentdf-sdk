#define _POSIX_C_SOURCE 200809L
#include "tdf3.h"
#include "json.h"
#include <gc.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <time.h>
#include <curl/curl.h>
#include <openssl/pem.h>
#include <openssl/evp.h>
#include <openssl/ec.h>
#include <openssl/ecdsa.h>
#include <openssl/core_names.h>
#include <openssl/rand.h>
static tdf3_bytes str(const char *s){return(tdf3_bytes){(const uint8_t *)s,strlen(s)};}
static tdf3_bytes text(J *j,const char *k){J *v=j_get(j,k);return v&&v->t==J_STR?(tdf3_bytes){(uint8_t *)v->s,v->n}:str("");}
static bool boolean(J *j,const char *k){J *v=j_get(j,k);return v&&v->t==J_BOOL&&v->b;}
static int64_t number(J *j,const char *k){J *v=j_get(j,k);return v&&v->t==J_NUM?strtoll(v->s,NULL,10):0;}
static uint8_t *readfile(const char *path,size_t *n){FILE *f=fopen(path,"rb");if(!f)return NULL;assert(!fseek(f,0,SEEK_END));long size=ftell(f);assert(size>=0);rewind(f);uint8_t *p=malloc((size_t)size+1);assert(p);assert(fread(p,1,(size_t)size,f)==(size_t)size);fclose(f);p[size]=0;*n=(size_t)size;return p;}
static void writefile(const char *path,const void *data,size_t n){FILE *f=fopen(path,"wb");assert(f);assert(!n||fwrite(data,1,n,f)==n);assert(!fclose(f));}
static char *jsontext(J *j){char *out=NULL;size_t n=0,cap=0;j_write(j,&out,&n,&cap);return out;}
static char *base64url(const uint8_t *data,size_t n){size_t len=4*((n+2)/3);unsigned char *out=malloc(len+1);assert(out);assert(EVP_EncodeBlock(out,data,(int)n)>=0);while(len&&out[len-1]=='=')len--;for(size_t i=0;i<len;i++){if(out[i]=='+')out[i]='-';else if(out[i]=='/')out[i]='_';}out[len]=0;return(char *)out;}
static char *bnurl(BIGNUM *b,int width){int n=width?width:BN_num_bytes(b);uint8_t *data=malloc((size_t)n);assert(data&&BN_bn2binpad(b,data,n)==n);char *s=base64url(data,(size_t)n);free(data);return s;}
typedef struct {char *data;size_t length;char nonce[1024];const atomic_bool *cancel;} response;
static size_t collect(char *data,size_t a,size_t b,void *state){response *r=state;size_t n=a*b;if(n>128*1024-r->length)return 0;r->data=realloc(r->data,r->length+n+1);assert(r->data);memcpy(r->data+r->length,data,n);r->length+=n;r->data[r->length]=0;return n;}
static size_t header(char *data,size_t a,size_t b,void *state){response *r=state;size_t n=a*b;if(n>12&&!strncasecmp(data,"DPoP-Nonce:",11)){size_t i=11;while(i<n&&(data[i]==' '||data[i]=='\t'))i++;size_t end=n;while(end>i&&(data[end-1]=='\r'||data[end-1]=='\n'))end--;if(end-i<sizeof r->nonce){memcpy(r->nonce,data+i,end-i);r->nonce[end-i]=0;}}return n;}
static int progress(void *state,curl_off_t a,curl_off_t b,curl_off_t c,curl_off_t d){(void)a;(void)b;(void)c;(void)d;return atomic_load(((response *)state)->cancel)?1:0;}
typedef struct {tdf3_config config;const char *case_name;} provider_state;
static int provider(void *state,const uint8_t *name,size_t name_len,const uint8_t *payload,size_t payload_len,const atomic_bool *cancel,gxc_value *out){(void)name;(void)name_len;(void)payload;(void)payload_len;struct GC_stack_base stack;assert(GC_get_stack_base(&stack)==GC_SUCCESS);int registered=GC_register_my_thread(&stack);assert(registered==GC_SUCCESS||registered==GC_DUPLICATE);provider_state *s=state;const char *label=s->case_name;J *token=j_new(J_OBJ);int rc=0;char *value=NULL,*confirmation=NULL;int64_t expires=(int64_t)time(NULL)+300;
 if(!strcmp(label,"provider-reject")){value=strdup("provider rejected");rc=1;goto finish;}
 if(!strcmp(label,"invalid-token")||!strcmp(label,"expired-token")){value=strdup(label);if(!strcmp(label,"expired-token"))expires-=600;goto token;}
 EVP_PKEY *key=NULL;J *jwk=NULL;char *thumb=NULL;
 if(s->config.DPoP){if(!strcmp(label,"mismatched-provider")){EVP_PKEY_CTX *ctx=EVP_PKEY_CTX_new_id(EVP_PKEY_EC,NULL);assert(ctx&&EVP_PKEY_keygen_init(ctx)>0&&EVP_PKEY_CTX_set_ec_paramgen_curve_nid(ctx,NID_X9_62_prime256v1)>0&&EVP_PKEY_keygen(ctx,&key)>0);EVP_PKEY_CTX_free(ctx);}else{BIO *bio=BIO_new_mem_buf(s->config.AuthPrivateKeyPEM.data,(int)s->config.AuthPrivateKeyPEM.length);key=PEM_read_bio_PrivateKey(bio,NULL,NULL,NULL);BIO_free(bio);}assert(key);bool rsa=EVP_PKEY_base_id(key)==EVP_PKEY_RSA;jwk=j_new(J_OBJ);BIGNUM *x=NULL,*y=NULL;if(rsa){assert(EVP_PKEY_get_bn_param(key,OSSL_PKEY_PARAM_RSA_N,&x)>0&&EVP_PKEY_get_bn_param(key,OSSL_PKEY_PARAM_RSA_E,&y)>0);char *n=bnurl(x,0),*e=bnurl(y,0);j_set(jwk,"e",j_cstr(e));j_set(jwk,"kty",j_cstr("RSA"));j_set(jwk,"n",j_cstr(n));free(n);free(e);}else{assert(EVP_PKEY_get_bn_param(key,OSSL_PKEY_PARAM_EC_PUB_X,&x)>0&&EVP_PKEY_get_bn_param(key,OSSL_PKEY_PARAM_EC_PUB_Y,&y)>0);char *xx=bnurl(x,32),*yy=bnurl(y,32);j_set(jwk,"crv",j_cstr("P-256"));j_set(jwk,"kty",j_cstr("EC"));j_set(jwk,"x",j_cstr(xx));j_set(jwk,"y",j_cstr(yy));free(xx);free(yy);}BN_free(x);BN_free(y);char *json=jsontext(jwk);uint8_t digest[32];unsigned n;assert(EVP_Digest(json,strlen(json),digest,&n,EVP_sha256(),NULL)>0);thumb=base64url(digest,32);}
 const char *endpoint="http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token";char nonce[1024]="";
 for(int attempt=0;attempt<3;attempt++){response r={.cancel=cancel};CURL *curl=curl_easy_init();assert(curl);struct curl_slist *headers=NULL;headers=curl_slist_append(headers,"Content-Type: application/x-www-form-urlencoded");
  if(key){J *head=j_new(J_OBJ);j_set(head,"typ",j_cstr("dpop+jwt"));j_set(head,"alg",j_cstr(EVP_PKEY_base_id(key)==EVP_PKEY_RSA?"RS256":"ES256"));j_set(head,"jwk",jwk);J *claims=j_new(J_OBJ);j_set(claims,"htu",j_cstr(endpoint));j_set(claims,"htm",j_cstr("POST"));char now[32];snprintf(now,sizeof now,"%lld",(long long)time(NULL));J *iat=j_new(J_NUM);iat->s=GC_strdup(now);iat->n=strlen(now);j_set(claims,"iat",iat);uint8_t rand[16];assert(RAND_bytes(rand,16)>0);char *jti=base64url(rand,16);j_set(claims,"jti",j_cstr(jti));free(jti);if(*nonce)j_set(claims,"nonce",j_cstr(nonce));char *h=jsontext(head),*c=jsontext(claims),*hh=base64url((uint8_t *)h,strlen(h)),*cc=base64url((uint8_t *)c,strlen(c));size_t len=strlen(hh)+strlen(cc)+2;char *unsigned_token=malloc(len);snprintf(unsigned_token,len,"%s.%s",hh,cc);free(hh);free(cc);EVP_MD_CTX *md=EVP_MD_CTX_new();assert(md&&EVP_DigestSignInit(md,NULL,EVP_sha256(),NULL,key)>0);uint8_t sig[256];size_t n=sizeof sig;assert(EVP_DigestSign(md,sig,&n,(uint8_t *)unsigned_token,strlen(unsigned_token))>0);EVP_MD_CTX_free(md);if(EVP_PKEY_base_id(key)!=EVP_PKEY_RSA){const unsigned char *p=sig;ECDSA_SIG *ec=d2i_ECDSA_SIG(NULL,&p,(long)n);assert(ec);const BIGNUM *a,*b;ECDSA_SIG_get0(ec,&a,&b);assert(BN_bn2binpad(a,sig,32)==32&&BN_bn2binpad(b,sig+32,32)==32);ECDSA_SIG_free(ec);n=64;}char *ss=base64url(sig,n);len=strlen(unsigned_token)+strlen(ss)+8;char *line=malloc(len);snprintf(line,len,"DPoP: %s.%s",unsigned_token,ss);headers=curl_slist_append(headers,line);free(line);free(unsigned_token);free(ss);}
  curl_easy_setopt(curl,CURLOPT_URL,endpoint);curl_easy_setopt(curl,CURLOPT_POSTFIELDS,"grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret");curl_easy_setopt(curl,CURLOPT_HTTPHEADER,headers);curl_easy_setopt(curl,CURLOPT_TIMEOUT_MS,15000L);curl_easy_setopt(curl,CURLOPT_NOSIGNAL,1L);curl_easy_setopt(curl,CURLOPT_WRITEFUNCTION,collect);curl_easy_setopt(curl,CURLOPT_WRITEDATA,&r);curl_easy_setopt(curl,CURLOPT_HEADERFUNCTION,header);curl_easy_setopt(curl,CURLOPT_HEADERDATA,&r);curl_easy_setopt(curl,CURLOPT_NOPROGRESS,0L);curl_easy_setopt(curl,CURLOPT_XFERINFOFUNCTION,progress);curl_easy_setopt(curl,CURLOPT_XFERINFODATA,&r);CURLcode code=curl_easy_perform(curl);long status=0;curl_easy_getinfo(curl,CURLINFO_RESPONSE_CODE,&status);curl_easy_cleanup(curl);curl_slist_free_all(headers);
  if(code==CURLE_OK&&(status==400||status==401)&&*r.nonce&&strcmp(nonce,r.nonce)){strcpy(nonce,r.nonce);free(r.data);continue;}
  if(code!=CURLE_OK||status!=200){value=strdup("provider transport/status failure");rc=1;free(r.data);break;}const char *err=NULL;J *response=j_parse(r.data,&err);assert(response&&!err);J *access=j_get(response,"access_token"),*expiry=j_get(response,"expires_in");assert(access&&access->t==J_STR);value=strdup(access->s);expires=(int64_t)time(NULL)+(expiry?strtoll(expiry->s,NULL,10):300);confirmation=thumb?strdup(thumb):NULL;free(r.data);break;
 }EVP_PKEY_free(key);free(thumb);if(!value){value=strdup("provider retry limit");rc=1;}
token:
 if(!rc){j_set(token,"value",j_cstr(value));j_set(token,"scheme",j_cstr(s->config.DPoP?"DPoP":"Bearer"));char expiry[32];snprintf(expiry,sizeof expiry,"%lld",(long long)expires);j_set(token,"expiresAt",j_cstr(expiry));j_set(token,"confirmationJKT",j_cstr(confirmation?confirmation:""));free(value);value=strdup(jsontext(token));}
finish:
 free(confirmation);out->kind=GXC_BYTES;out->bytes=(uint8_t *)value;out->length=strlen(value);if(registered==GC_SUCCESS)GC_unregister_my_thread();return rc;
}
static atomic_bool held_started,held_released;
static int held_provider(void *state,const uint8_t *name,size_t names,const uint8_t *data,size_t n,const atomic_bool *cancel,gxc_value *out){(void)state;(void)name;(void)names;(void)data;(void)n;atomic_store(&held_started,true);while(!atomic_load(cancel)){struct timespec pause={0,2000000};nanosleep(&pause,NULL);}struct timespec cleanup={0,40000000};nanosleep(&cleanup,NULL);atomic_store(&held_released,true);out->kind=GXC_BYTES;out->bytes=(uint8_t *)strdup("stopped");out->length=7;return 1;}
static tdf3_config config(J *j){tdf3_config c={0};
#define FIELD(name) c.name=text(j,#name)
 FIELD(PlatformURL);FIELD(KASURL);FIELD(IssuerURL);FIELD(TokenURL);FIELD(ClientID);FIELD(ClientSecret);FIELD(TokenProviderName);FIELD(KASPublicKeyPEM);FIELD(KID);FIELD(KASAlgorithm);FIELD(SessionAlgorithm);FIELD(AuthPrivateKeyPEM);FIELD(AuthAlgorithm);
#undef FIELD
 c.AllowHTTP=boolean(j,"AllowHTTP");c.DPoP=boolean(j,"DPoP");c.TimeoutMillis=number(j,"TimeoutMillis");J *routes=j_get(j,"AllowedKAS");if(routes&&routes->t==J_ARR){tdf3_kas_route *r=calloc(routes->len,sizeof(*r));assert(r);c.AllowedKAS=r;c.AllowedKASLength=routes->len;for(size_t i=0;i<routes->len;i++){r[i].URL=text(routes->items[i],"URL");r[i].APIBaseURL=text(routes->items[i],"APIBaseURL");}}return c;}
static void errorfile(const char *path,const tdf3_error *e){J *j=j_new(J_OBJ);
#define FIELD(name,key) j_set(j,key,j_strn((const char *)e->name.data,e->name.length))
 FIELD(Message,"message");FIELD(Code,"code");FIELD(Operation,"operation");FIELD(CauseCategory,"causeCategory");FIELD(ServerCode,"serverCode");FIELD(ServerMessage,"serverMessage");
#undef FIELD
 char n[32];snprintf(n,sizeof n,"%lld",(long long)e->HTTPStatus);J *status=j_new(J_NUM);status->s=GC_strdup(n);status->n=strlen(n);j_set(j,"httpStatus",status);snprintf(n,sizeof n,"%d",e->Kind);J *kind=j_new(J_NUM);kind->s=GC_strdup(n);kind->n=strlen(n);j_set(j,"kind",kind);char *out=jsontext(j);writefile(path,out,strlen(out));}
int main(int argc,char **argv){assert(argc==5);GC_INIT();GC_allow_register_threads();const char *run=argv[2],*mode=argv[3],*name=argv[4];char path[4096];snprintf(path,sizeof path,"%s/config.json",run);size_t n;uint8_t *raw=readfile(path,&n);assert(raw);const char *parse_error=NULL;J *j=j_parse((char *)raw,&parse_error);assert(j&&!parse_error);free(raw);tdf3_config c=config(j);provider_state ps={c,name};gxc_options calls={.provider=c.TokenProviderName.length?provider:NULL,.provider_state=&ps};tdf3_result result={0};tdf3_error e={0};bool encrypt=!strcmp(mode,"encrypt");
 if(!strcmp(mode,"repeat")){
 uint8_t input[]={0,255,128,1},meta[]={0,255};tdf3_bytes attr=str("https://example.com/attr/attr1/value/value1");tdf3_encrypt_options o={.Attributes=&attr,.AttributesLength=1,.Metadata={meta,2},.IncludeMetadata=true,.MimeType={ (const uint8_t *)"application/日本語",strlen("application/日本語") }};
 tdf3_operation *op=tdf3_encrypt_submit(&c,(tdf3_bytes){input,4},&o,&calls,&e);assert(op);memset(input,7,4);memset(meta,7,2);tdf3_operation_wake(op);assert(tdf3_operation_drive(op,-1));assert(!tdf3_operation_take(op,&result,&e));tdf3_operation_destroy(&op);tdf3_operation_destroy(&op);
 tdf3_result decrypted={0};assert(!tdf3_decrypt(&c,(tdf3_bytes){result.Payload.data,result.Payload.length},&calls,&decrypted,&e));uint8_t expected[]={0,255,128,1},metadata[]={0,255};assert(decrypted.Payload.length==4&&!memcmp(decrypted.Payload.data,expected,4));assert(decrypted.Metadata.length==2&&!memcmp(decrypted.Metadata.data,metadata,2)&&decrypted.HasMetadata);bool mime=false;for(size_t i=0;i+strlen("日本語")<=decrypted.ManifestJSON.length;i++)if(!memcmp(decrypted.ManifestJSON.data+i,"日本語",strlen("日本語")))mime=true;assert(mime);
 tdf3_config held=c;held.TokenProviderName=str("held");gxc_options held_options={.provider=held_provider};tdf3_operation *active=tdf3_decrypt_submit(&held,(tdf3_bytes){result.Payload.data,result.Payload.length},&held_options,&e);assert(active);for(int i=0;i<10000&&!atomic_load(&held_started);i++){struct timespec pause={0,2000000};nanosleep(&pause,NULL);}assert(atomic_load(&held_started));
 tdf3_operation *queued=tdf3_decrypt_submit(&c,(tdf3_bytes){result.Payload.data,result.Payload.length},&calls,&e);assert(queued);tdf3_operation_cancel(queued);assert(tdf3_operation_drive(queued,-1));tdf3_result empty={0};assert(tdf3_operation_take(queued,&empty,&e)==6);assert(!atomic_load(&held_released)&&!empty.Payload.data);tdf3_error_free(&e);tdf3_operation_destroy(&queued);
 tdf3_operation_cancel(active);tdf3_operation_wake(active);assert(tdf3_operation_drive(active,-1));assert(tdf3_operation_take(active,&empty,&e)==1);assert(atomic_load(&held_released)&&!empty.Payload.data);tdf3_error_free(&e);tdf3_operation_destroy(&active);
 for(int i=0;i<12;i++){tdf3_result next={0};assert(!tdf3_decrypt(&c,(tdf3_bytes){result.Payload.data,result.Payload.length},&calls,&next,&e));assert(next.Payload.length==4&&!memcmp(next.Payload.data,expected,4));tdf3_result_free(&next);tdf3_error_free(&e);}
 // Parsed JSON owns strings referenced by the malloc-backed route array.
 // Keep that caller input alive across every async/repeated submission.
 tdf3_result_free(&result);assert(!memcmp(decrypted.Payload.data,expected,4));tdf3_result_free(&decrypted);GC_reachable_here(j);free((void *)c.AllowedKAS);puts("PASS async snapshots/owned metadata + queued/active cancel actual provider release + drive/wake/take/destroy/recovery");return 0;}
 if(!strcmp(mode,"controlled")){
 c.TokenProviderName=str("controlled");gxc_options controlled_calls={.provider=provider,.provider_state=&ps};ps.config=c;ps.case_name="invalid-token";
 snprintf(path,sizeof path,"%s/archive.tdf",run);raw=readfile(path,&n);assert(raw);tdf3_operation *op=tdf3_decrypt_submit(&c,(tdf3_bytes){raw,n},&controlled_calls,&e);assert(op);free(raw);
 if(!strcmp(name,"active-cancel")){snprintf(path,sizeof path,"%s/entered",run);bool entered=false;for(int i=0;i<10000;i++){FILE *f=fopen(path,"rb");if(f){fclose(f);entered=true;break;}struct timespec pause={0,2000000};nanosleep(&pause,NULL);}assert(entered);tdf3_operation_cancel(op);}
 assert(tdf3_operation_drive(op,-1));struct timespec settled;assert(!clock_gettime(CLOCK_MONOTONIC,&settled));snprintf(path,sizeof path,"%s/settled.nanoseconds",run);FILE *settlement=fopen(path,"w");assert(settlement);fprintf(settlement,"%lld\n",(long long)settled.tv_sec*1000000000+settled.tv_nsec);assert(!fclose(settlement));int rc=tdf3_operation_take(op,&result,&e);tdf3_operation_destroy(&op);assert(rc&&!result.Payload.data&&!result.Payload.length);snprintf(path,sizeof path,"%s/%s.error.json",run,name);errorfile(path,&e);tdf3_error_free(&e);free((void *)c.AllowedKAS);return 0;}

 snprintf(path,sizeof path,"%s/%s.%s",run,name,encrypt?"input":"tdf");raw=readfile(path,&n);assert(raw);int rc;
 if(encrypt){tdf3_bytes attr=str(strstr(name,"denied")?"https://example.com/attr/attr1/value/value2":"https://example.com/attr/attr1/value/value1");bool metadata=strstr(name,"metadata")!=NULL,empty=strstr(name,"empty-metadata")!=NULL;tdf3_encrypt_options o={.Attributes=&attr,.AttributesLength=1,.SegmentSize=16384,.HasSegmentSize=true,.SegmentHashAlgorithm=str(strstr(name,"hs256")?"HS256":""),.Metadata=str(metadata&&!empty?"{\"source\":\"independent metadata\",\"count\":7}":""),.IncludeMetadata=metadata};rc=tdf3_encrypt(&c,(tdf3_bytes){raw,n},&o,&calls,&result,&e);}
 else rc=tdf3_decrypt(&c,(tdf3_bytes){raw,n},&calls,&result,&e);
 free(raw);
 if(rc){snprintf(path,sizeof path,"%s/%s.error.json",run,name);errorfile(path,&e);assert(!result.Payload.length&&!result.Payload.data);if(strcmp(mode,"negative")){fprintf(stderr,"native C error kind=%d code=%.*s operation=%.*s message=%.*s\n",e.Kind,(int)e.Code.length,e.Code.data,(int)e.Operation.length,e.Operation.data,(int)e.Message.length,e.Message.data);tdf3_error_free(&e);return 1;}tdf3_error_free(&e);free((void *)c.AllowedKAS);return 0;}
 assert(strcmp(mode,"negative"));snprintf(path,sizeof path,"%s/%s.%s",run,name,encrypt?"generated.tdf":"out");writefile(path,result.Payload.data,result.Payload.length);
 if(!encrypt){snprintf(path,sizeof path,"%s/%s.metadata",run,name);writefile(path,result.Metadata.data,result.Metadata.length);snprintf(path,sizeof path,"%s/%s.presence",run,name);writefile(path,result.HasMetadata?"true":"false",result.HasMetadata?4:5);snprintf(path,sizeof path,"%s/%s.manifest",run,name);writefile(path,result.ManifestJSON.data,result.ManifestJSON.length);}
 tdf3_result_free(&result);tdf3_result_free(&result);tdf3_error_free(&e);free((void *)c.AllowedKAS);return 0;
}
