/* Independent native consumer of the installed package's handle API: one SDK
 * per run, otdf_create_tdf/otdf_decrypt_bytes, otdf_load_tdf readers, 100
 * round trips, the platform service clients as JSON, and the controlled checks of
 * tests/interop/delivery/controlled_platform.py. Host OAuth/DPoP is native
 * libcurl/OpenSSL through the gxc_provider callback. */
#define _POSIX_C_SOURCE 200809L
#include "opentdf.h"
#include "json.h"
#include <gc.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <time.h>
#include <pthread.h>
#include <curl/curl.h>
#include <openssl/pem.h>
#include <openssl/evp.h>
#include <openssl/ec.h>
#include <openssl/ecdsa.h>
#include <openssl/core_names.h>
#include <openssl/rand.h>
#define ATTRIBUTE "https://example.com/attr/attr1/value/value1"
static const char *text(J *j,const char *k){J *v=j_get(j,k);return v&&v->t==J_STR?v->s:"";}
static bool boolean(J *j,const char *k){J *v=j_get(j,k);return v&&v->t==J_BOOL&&v->b;}
static int64_t number(J *j,const char *k){J *v=j_get(j,k);return v&&v->t==J_NUM?strtoll(v->s,NULL,10):0;}
static uint8_t *readfile(const char *path,size_t *n){FILE *f=fopen(path,"rb");if(!f)return NULL;assert(!fseek(f,0,SEEK_END));long size=ftell(f);assert(size>=0);rewind(f);uint8_t *p=malloc((size_t)size+1);assert(p);assert(fread(p,1,(size_t)size,f)==(size_t)size);fclose(f);p[size]=0;*n=(size_t)size;return p;}
static void writefile(const char *path,const void *data,size_t n){FILE *f=fopen(path,"wb");assert(f);assert(!n||fwrite(data,1,n,f)==n);assert(!fclose(f));}
static char *jsontext(J *j){char *out=NULL;size_t n=0,cap=0;j_write(j,&out,&n,&cap);return out;}
static char *base64url(const uint8_t *data,size_t n){size_t len=4*((n+2)/3);unsigned char *out=malloc(len+1);assert(out);assert(EVP_EncodeBlock(out,data,(int)n)>=0);while(len&&out[len-1]=='=')len--;for(size_t i=0;i<len;i++){if(out[i]=='+')out[i]='-';else if(out[i]=='/')out[i]='_';}out[len]=0;return(char *)out;}
static char *bnurl(BIGNUM *b,int width){int n=width?width:BN_num_bytes(b);uint8_t *data=malloc((size_t)n);assert(data&&BN_bn2binpad(b,data,n)==n);char *s=base64url(data,(size_t)n);free(data);return s;}
static void check(bool ok,const char *label){if(!ok){fprintf(stderr,"ASSERT: %s\n",label);exit(1);}}
typedef struct {char *data;size_t length;char nonce[1024];const atomic_bool *cancel;} response;
static size_t collect(char *data,size_t a,size_t b,void *state){response *r=state;size_t n=a*b;if(n>128*1024-r->length)return 0;r->data=realloc(r->data,r->length+n+1);assert(r->data);memcpy(r->data+r->length,data,n);r->length+=n;r->data[r->length]=0;return n;}
static size_t header(char *data,size_t a,size_t b,void *state){response *r=state;size_t n=a*b;if(n>12&&!strncasecmp(data,"DPoP-Nonce:",11)){size_t i=11;while(i<n&&(data[i]==' '||data[i]=='\t'))i++;size_t end=n;while(end>i&&(data[end-1]=='\r'||data[end-1]=='\n'))end--;if(end-i<sizeof r->nonce){memcpy(r->nonce,data+i,end-i);r->nonce[end-i]=0;}}return n;}
static int progress(void *state,curl_off_t a,curl_off_t b,curl_off_t c,curl_off_t d){(void)a;(void)b;(void)c;(void)d;return atomic_load(((response *)state)->cancel)?1:0;}
typedef struct {J *config;const char *case_name;} provider_state;
/* The host token source: native OAuth client credentials with a DPoP proof when the profile asks for one. */
static int provider(void *state,const uint8_t *name,size_t name_len,const uint8_t *payload,size_t payload_len,const atomic_bool *cancel,gxc_value *out){(void)name;(void)name_len;(void)payload;(void)payload_len;struct GC_stack_base stack;assert(GC_get_stack_base(&stack)==GC_SUCCESS);int registered=GC_register_my_thread(&stack);assert(registered==GC_SUCCESS||registered==GC_DUPLICATE);provider_state *s=state;const char *label=s->case_name;bool dpop=boolean(s->config,"DPoP");J *token=j_new(J_OBJ);int rc=0;char *value=NULL,*confirmation=NULL;int64_t expires=(int64_t)time(NULL)+300;
 if(!strcmp(label,"provider-reject")){value=strdup("provider rejected");rc=1;goto finish;}
 if(!strcmp(label,"invalid-token")||!strcmp(label,"expired-token")){value=strdup(label);if(!strcmp(label,"expired-token"))expires-=600;goto token;}
 EVP_PKEY *key=NULL;J *jwk=NULL;char *thumb=NULL;
 if(dpop){if(!strcmp(label,"mismatched-provider")){EVP_PKEY_CTX *ctx=EVP_PKEY_CTX_new_id(EVP_PKEY_EC,NULL);assert(ctx&&EVP_PKEY_keygen_init(ctx)>0&&EVP_PKEY_CTX_set_ec_paramgen_curve_nid(ctx,NID_X9_62_prime256v1)>0&&EVP_PKEY_keygen(ctx,&key)>0);EVP_PKEY_CTX_free(ctx);}else{const char *pem=text(s->config,"AuthPrivateKeyPEM");BIO *bio=BIO_new_mem_buf(pem,(int)strlen(pem));key=PEM_read_bio_PrivateKey(bio,NULL,NULL,NULL);BIO_free(bio);}assert(key);bool rsa=EVP_PKEY_base_id(key)==EVP_PKEY_RSA;jwk=j_new(J_OBJ);BIGNUM *x=NULL,*y=NULL;if(rsa){assert(EVP_PKEY_get_bn_param(key,OSSL_PKEY_PARAM_RSA_N,&x)>0&&EVP_PKEY_get_bn_param(key,OSSL_PKEY_PARAM_RSA_E,&y)>0);char *n=bnurl(x,0),*e=bnurl(y,0);j_set(jwk,"e",j_cstr(e));j_set(jwk,"kty",j_cstr("RSA"));j_set(jwk,"n",j_cstr(n));free(n);free(e);}else{assert(EVP_PKEY_get_bn_param(key,OSSL_PKEY_PARAM_EC_PUB_X,&x)>0&&EVP_PKEY_get_bn_param(key,OSSL_PKEY_PARAM_EC_PUB_Y,&y)>0);char *xx=bnurl(x,32),*yy=bnurl(y,32);j_set(jwk,"crv",j_cstr("P-256"));j_set(jwk,"kty",j_cstr("EC"));j_set(jwk,"x",j_cstr(xx));j_set(jwk,"y",j_cstr(yy));free(xx);free(yy);}BN_free(x);BN_free(y);char *json=jsontext(jwk);uint8_t digest[32];unsigned n;assert(EVP_Digest(json,strlen(json),digest,&n,EVP_sha256(),NULL)>0);thumb=base64url(digest,32);}
 const char *endpoint="http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token";char nonce[1024]="";
 for(int attempt=0;attempt<3;attempt++){response r={.cancel=cancel};CURL *curl=curl_easy_init();assert(curl);struct curl_slist *headers=NULL;headers=curl_slist_append(headers,"Content-Type: application/x-www-form-urlencoded");
  if(key){J *head=j_new(J_OBJ);j_set(head,"typ",j_cstr("dpop+jwt"));j_set(head,"alg",j_cstr(EVP_PKEY_base_id(key)==EVP_PKEY_RSA?"RS256":"ES256"));j_set(head,"jwk",jwk);J *claims=j_new(J_OBJ);j_set(claims,"htu",j_cstr(endpoint));j_set(claims,"htm",j_cstr("POST"));char now[32];snprintf(now,sizeof now,"%lld",(long long)time(NULL));J *iat=j_new(J_NUM);iat->s=GC_strdup(now);iat->n=strlen(now);j_set(claims,"iat",iat);uint8_t rand[16];assert(RAND_bytes(rand,16)>0);char *jti=base64url(rand,16);j_set(claims,"jti",j_cstr(jti));free(jti);if(*nonce)j_set(claims,"nonce",j_cstr(nonce));char *h=jsontext(head),*c=jsontext(claims),*hh=base64url((uint8_t *)h,strlen(h)),*cc=base64url((uint8_t *)c,strlen(c));size_t len=strlen(hh)+strlen(cc)+2;char *unsigned_token=malloc(len);snprintf(unsigned_token,len,"%s.%s",hh,cc);free(hh);free(cc);EVP_MD_CTX *md=EVP_MD_CTX_new();assert(md&&EVP_DigestSignInit(md,NULL,EVP_sha256(),NULL,key)>0);uint8_t sig[256];size_t n=sizeof sig;assert(EVP_DigestSign(md,sig,&n,(uint8_t *)unsigned_token,strlen(unsigned_token))>0);EVP_MD_CTX_free(md);if(EVP_PKEY_base_id(key)!=EVP_PKEY_RSA){const unsigned char *p=sig;ECDSA_SIG *ec=d2i_ECDSA_SIG(NULL,&p,(long)n);assert(ec);const BIGNUM *a,*b;ECDSA_SIG_get0(ec,&a,&b);assert(BN_bn2binpad(a,sig,32)==32&&BN_bn2binpad(b,sig+32,32)==32);ECDSA_SIG_free(ec);n=64;}char *ss=base64url(sig,n);len=strlen(unsigned_token)+strlen(ss)+8;char *line=malloc(len);snprintf(line,len,"DPoP: %s.%s",unsigned_token,ss);headers=curl_slist_append(headers,line);free(line);free(unsigned_token);free(ss);}
  curl_easy_setopt(curl,CURLOPT_URL,endpoint);curl_easy_setopt(curl,CURLOPT_POSTFIELDS,"grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret");curl_easy_setopt(curl,CURLOPT_HTTPHEADER,headers);curl_easy_setopt(curl,CURLOPT_TIMEOUT_MS,15000L);curl_easy_setopt(curl,CURLOPT_NOSIGNAL,1L);curl_easy_setopt(curl,CURLOPT_WRITEFUNCTION,collect);curl_easy_setopt(curl,CURLOPT_WRITEDATA,&r);curl_easy_setopt(curl,CURLOPT_HEADERFUNCTION,header);curl_easy_setopt(curl,CURLOPT_HEADERDATA,&r);curl_easy_setopt(curl,CURLOPT_NOPROGRESS,0L);curl_easy_setopt(curl,CURLOPT_XFERINFOFUNCTION,progress);curl_easy_setopt(curl,CURLOPT_XFERINFODATA,&r);CURLcode code=curl_easy_perform(curl);long status=0;curl_easy_getinfo(curl,CURLINFO_RESPONSE_CODE,&status);curl_easy_cleanup(curl);curl_slist_free_all(headers);
  if(code==CURLE_OK&&(status==400||status==401)&&*r.nonce&&strcmp(nonce,r.nonce)){strcpy(nonce,r.nonce);free(r.data);continue;}
  if(code!=CURLE_OK||status!=200){value=strdup("provider transport/status failure");rc=1;free(r.data);break;}const char *err=NULL;J *response=j_parse(r.data,&err);assert(response&&!err);J *access=j_get(response,"access_token"),*expiry=j_get(response,"expires_in");assert(access&&access->t==J_STR);value=strdup(access->s);expires=(int64_t)time(NULL)+(expiry?strtoll(expiry->s,NULL,10):300);confirmation=thumb?strdup(thumb):NULL;free(r.data);break;
 }EVP_PKEY_free(key);free(thumb);if(!value){value=strdup("provider retry limit");rc=1;}
token:
 if(!rc){j_set(token,"value",j_cstr(value));j_set(token,"scheme",j_cstr(dpop?"DPoP":"Bearer"));char expiry[32];snprintf(expiry,sizeof expiry,"%lld",(long long)expires);j_set(token,"expiresAt",j_cstr(expiry));j_set(token,"confirmationJKT",j_cstr(confirmation?confirmation:""));free(value);value=strdup(jsontext(token));}
finish:
 free(confirmation);out->kind=GXC_BYTES;out->bytes=(uint8_t *)value;out->length=strlen(value);if(registered==GC_SUCCESS)GC_unregister_my_thread();return rc;
}
static atomic_bool held_started;
static int held_provider(void *state,const uint8_t *name,size_t names,const uint8_t *data,size_t n,const atomic_bool *cancel,gxc_value *out){(void)state;(void)name;(void)names;(void)data;(void)n;atomic_store(&held_started,true);while(!atomic_load(cancel)){struct timespec pause={0,2000000};nanosleep(&pause,NULL);}out->kind=GXC_BYTES;out->bytes=(uint8_t *)strdup("stopped");out->length=7;return 1;}

/* config.json carries the façade Config fields the delivery scripts write; they map onto the handle API's options as the Go consumer maps them. */
static otdf_options options_of(J *cfg,provider_state *ps,bool credentials){otdf_options o=OTDF_OPTIONS_INIT;o.insecure_plaintext=boolean(cfg,"AllowHTTP");if(*text(cfg,"TokenURL"))o.token_endpoint=text(cfg,"TokenURL");if(credentials&&*text(cfg,"ClientID")){o.client_id=text(cfg,"ClientID");o.client_secret=text(cfg,"ClientSecret");}if(*text(cfg,"TokenProviderName")){o.token_provider=provider;o.token_provider_state=ps;}o.timeout_millis=number(cfg,"TimeoutMillis");if(boolean(cfg,"DPoP"))o.dpop_algorithm=*text(cfg,"AuthAlgorithm")?text(cfg,"AuthAlgorithm"):"ES256";if(*text(cfg,"AuthPrivateKeyPEM"))o.dpop_key_pem=text(cfg,"AuthPrivateKeyPEM");return o;}
static char kas_url[2100];
static otdf_kas_info kas_information(J *cfg){if(*text(cfg,"KASURL"))snprintf(kas_url,sizeof kas_url,"%s",text(cfg,"KASURL"));else{snprintf(kas_url,sizeof kas_url,"%s",text(cfg,"PlatformURL"));size_t n=strlen(kas_url);while(n&&kas_url[n-1]=='/')kas_url[--n]=0;snprintf(kas_url+n,sizeof kas_url-n,"/kas");}return(otdf_kas_info){.url=kas_url,.public_key=text(cfg,"KASPublicKeyPEM"),.kid=text(cfg,"KID"),.algorithm=text(cfg,"KASAlgorithm")};}
static const char *attribute=ATTRIBUTE;
static otdf_tdf_options tdf_options(J *cfg,otdf_kas_info *kas){*kas=kas_information(cfg);otdf_tdf_options o=OTDF_TDF_OPTIONS_INIT;o.disable_autoconfigure=true;o.kas_information=kas;o.kas_information_count=1;o.attributes=&attribute;o.attribute_count=1;o.segment_size=16384;if(*text(cfg,"KASAlgorithm"))o.wrapping_key_algorithm=text(cfg,"KASAlgorithm");return o;}
static const char *allow[16];
static otdf_reader_options reader_options(J *cfg){size_t n=0;allow[n++]=text(cfg,"PlatformURL");if(*text(cfg,"KASURL"))allow[n++]=text(cfg,"KASURL");J *routes=j_get(cfg,"AllowedKAS");if(routes&&routes->t==J_ARR)for(size_t i=0;i<routes->len&&n<16;i++)allow[n++]=text(routes->items[i],"URL");otdf_reader_options o=OTDF_READER_OPTIONS_INIT;o.kas_allowlist=allow;o.kas_allowlist_count=n;if(*text(cfg,"SessionAlgorithm"))o.session_key_type=text(cfg,"SessionAlgorithm");return o;}
static J *report(const otdf_error *e){J *j=j_new(J_OBJ);j_set(j,"kind",j_cstr(e->kind?e->kind:""));j_set(j,"op",j_cstr(e->op?e->op:""));char n[32];snprintf(n,sizeof n,"%lld",(long long)e->http_status);J *status=j_new(J_NUM);status->s=GC_strdup(n);status->n=strlen(n);j_set(j,"httpStatus",status);j_set(j,"connectCode",j_cstr(e->connect_code?e->connect_code:""));j_set(j,"message",j_cstr(e->message?e->message:""));return j;}
static void fail(const char *label,otdf_error *e){fprintf(stderr,"consumer failure %s: %s\n",label,jsontext(report(e)));exit(1);}
#define TRY(label,call) do{otdf_error e_={0};if(call)fail(label,&e_);}while(0)
/* expect_failure runs a call that must fail and returns its report. */
#define EXPECT(label,call) ({otdf_error e_={0};int rc_=(call);if(!rc_){fprintf(stderr,"%s returned successful output\n",label);exit(1);}J *r_=report(&e_);otdf_error_free(&e_);r_;})
static bool same_kind(J *r,const char *kind){return !strcmp(text(r,"kind"),kind);}
static void write_json(const char *run,const char *file,J *j){char path[4096];snprintf(path,sizeof path,"%s/%s",run,file);char *out=jsontext(j);writefile(path,out,strlen(out));}

/* xtest's test_self.py through the service clients: list namespaces, create a
 * namespace and an attribute, read the attribute back by id, through the
 * namespace's list and through the raw call, and check a missing attribute's error. */
static J *reply(const char *label,int rc,otdf_error *e,char **json){if(rc)fail(label,e);const char *err=NULL;J *j=j_parse(*json,&err);check(j&&!err,label);otdf_string_free(*json);*json=NULL;return j;}
static size_t count(J *j,const char *k){J *v=j_get(j,k);return v&&v->t==J_ARR?v->len:0;}
static void services(otdf_sdk *sdk,const char *run){
 char ns[64],request[512];otdf_error e={0};char *out=NULL;uint8_t rand[6];assert(RAND_bytes(rand,6)>0);snprintf(ns,sizeof ns,"gcc%02x%02x%02x%02x%02x%02x.com",rand[0],rand[1],rand[2],rand[3],rand[4],rand[5]);
 J *before=reply("list namespaces",otdf_namespaces_list_namespaces(sdk,NULL,NULL,&out,&e),&e,&out);
 snprintf(request,sizeof request,"{\"name\":\"%s\"}",ns);
 J *namespace=j_get(reply("create namespace",otdf_namespaces_create_namespace(sdk,request,NULL,&out,&e),&e,&out),"namespace");
 check(namespace&&*text(namespace,"id")&&!strcmp(text(namespace,"name"),ns),"created namespace");
 J *after=reply("list namespaces again",otdf_namespaces_list_namespaces(sdk,"{}",NULL,&out,&e),&e,&out);bool listed=false;
 for(size_t i=0;i<count(after,"namespaces");i++)listed=listed||!strcmp(text(j_get(after,"namespaces")->items[i],"id"),text(namespace,"id"));
 check(listed&&count(after,"namespaces")==count(before,"namespaces")+1,"namespace listed");
 snprintf(request,sizeof request,"{\"namespaceId\":\"%s\",\"name\":\"free\",\"rule\":\"ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF\",\"values\":[\"1\",\"2\",\"3\"]}",text(namespace,"id"));
 J *created=j_get(reply("create attribute",otdf_attributes_create_attribute(sdk,request,NULL,&out,&e),&e,&out),"attribute");check(created!=NULL,"created attribute");
 snprintf(request,sizeof request,"{\"attributeId\":\"%s\"}",text(created,"id"));
 J *attribute=j_get(reply("get attribute",otdf_attributes_get_attribute(sdk,request,NULL,&out,&e),&e,&out),"attribute");check(attribute!=NULL,"attribute");
 char fqn[256];snprintf(fqn,sizeof fqn,"https://%s/attr/free",ns);J *values=j_new(J_ARR),*listed_values=j_get(attribute,"values");
 for(size_t i=0;listed_values&&i<listed_values->len;i++)j_push(values,j_cstr(text(listed_values->items[i],"value")));
 check(!strcmp(text(attribute,"fqn"),fqn)&&!strcmp(text(attribute,"rule"),"ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF")&&!strcmp(jsontext(values),"[\"1\",\"2\",\"3\"]")&&!strcmp(text(j_get(attribute,"namespace"),"name"),ns),"attribute read back");
 snprintf(request,sizeof request,"{\"namespace\":\"%s\"}",ns);
 J *in_namespace=reply("list attributes",otdf_attributes_list_attributes(sdk,request,NULL,&out,&e),&e,&out);
 check(count(in_namespace,"attributes")==1&&!strcmp(text(j_get(in_namespace,"attributes")->items[0],"id"),text(attribute,"id")),"attribute listed");
 snprintf(request,sizeof request,"{\"attributeId\":\"%s\"}",text(attribute,"id"));
 J *raw=reply("raw call",otdf_sdk_call(sdk,"policy.attributes.AttributesService","GetAttribute",request,NULL,&out,&e),&e,&out);check(!strcmp(text(j_get(raw,"attribute"),"fqn"),fqn),"raw call");
 J *missing=EXPECT("missing attribute",otdf_attributes_get_attribute(sdk,"{\"attributeId\":\"00000000-0000-0000-0000-000000000000\"}",NULL,&out,&e_));
 check(!strcmp(text(missing,"connectCode"),"not_found")&&number(missing,"httpStatus")==404,"missing attribute error");
 J *report_=j_new(J_OBJ);char n[32];j_set(report_,"namespace",j_cstr(ns));
 J *before_n=j_new(J_NUM);snprintf(n,sizeof n,"%zu",count(before,"namespaces"));before_n->s=GC_strdup(n);before_n->n=strlen(n);j_set(report_,"namespaces_before",before_n);
 J *after_n=j_new(J_NUM);snprintf(n,sizeof n,"%zu",count(after,"namespaces"));after_n->s=GC_strdup(n);after_n->n=strlen(n);j_set(report_,"namespaces_after",after_n);
 j_set(report_,"attribute_fqn",j_cstr(fqn));j_set(report_,"values",values);j_set(report_,"not_found",missing);write_json(run,"services.json",report_);
}

/* 100 encrypt and decrypt calls on one SDK, independent inputs and results, every handle closed; then the SDK is closed and later calls fail. */
static void repeat(otdf_sdk *sdk,J *cfg,const char *name){
 uint8_t original[]={0,255,128,1};otdf_owned_bytes archive={0};otdf_kas_info kas;
 for(int i=0;i<100;i++){
  uint8_t input[5];memcpy(input,original,4);input[4]=(uint8_t)i;
  otdf_tdf_options o=tdf_options(cfg,&kas);char metadata[32];snprintf(metadata,sizeof metadata,"round %d",i);o.metadata=metadata;o.mime_type="application/日本語";
  otdf_owned_bytes tdf={0};otdf_tdf_object *object=NULL;otdf_error e={0};
  if(otdf_create_tdf(sdk,(otdf_bytes){input,5},&o,NULL,&tdf,&object,&e))fail("create",&e);
  int64_t size=0;if(otdf_tdf_object_size(object,NULL,&size,&e))fail("size",&e);check(size==(int64_t)tdf.length,"size");
  char *manifest=NULL;if(otdf_tdf_object_manifest_json(object,NULL,&manifest,&e))fail("manifest",&e);check(strstr(manifest,"application/日本語")!=NULL,"UTF-8 manifest");free(manifest);
  otdf_tdf_object_close(object,NULL);otdf_tdf_object_free(object);
  input[0]=9;
  otdf_reader *reader=NULL;otdf_reader_options ro=reader_options(cfg);
  if(otdf_load_tdf(sdk,(otdf_bytes){tdf.data,tdf.length},&ro,NULL,&reader,&e))fail("load",&e);
  char **attributes=NULL;size_t count=0;if(otdf_reader_data_attributes(reader,NULL,&attributes,&count,&e))fail("attributes",&e);check(count==1,"attributes");otdf_strings_free(attributes,count);
  otdf_reader_close(reader,NULL);otdf_reader_free(reader);
  otdf_owned_bytes plain={0};ro=reader_options(cfg);if(otdf_decrypt_bytes(sdk,(otdf_bytes){tdf.data,tdf.length},&ro,NULL,&plain,&e))fail("decrypt",&e);
  uint8_t expected[5];memcpy(expected,original,4);expected[4]=(uint8_t)i;check(plain.length==5&&!memcmp(plain.data,expected,5),"round trip");
  otdf_bytes_free(&plain);otdf_bytes_free(&archive);archive=tdf;
 }
 provider_state ps={cfg,name};otdf_options wrong=options_of(cfg,&ps,true);wrong.client_secret="wrong";otdf_sdk *bad=NULL;otdf_owned_bytes out={0};otdf_reader_options ro=reader_options(cfg);
 J *rejected=EXPECT("wrong credentials",otdf_sdk_new(text(cfg,"PlatformURL"),&wrong,NULL,&bad,&e_)||otdf_decrypt_bytes(bad,(otdf_bytes){archive.data,archive.length},&ro,NULL,&out,&e_));if(bad)otdf_sdk_close(bad,NULL);otdf_sdk_free(bad);
 check(same_kind(rejected,"auth")&&number(rejected,"httpStatus")==401,"wrong credentials rejected");
 otdf_options held=options_of(cfg,&ps,false);held.token_provider=held_provider;held.token_provider_state=NULL;otdf_sdk *holder=NULL;TRY("holder",otdf_sdk_new(text(cfg,"PlatformURL"),&held,NULL,&holder,&e_));
 atomic_bool cancel=false;otdf_call call={.canceled=&cancel};
 pthread_t canceller;struct timespec pause={0,2000000};
 void *cancel_after(void *flag){for(int i=0;i<10000&&!atomic_load(&held_started);i++)nanosleep(&pause,NULL);atomic_store((atomic_bool *)flag,true);return NULL;}
 assert(!pthread_create(&canceller,NULL,cancel_after,&cancel));
 ro=reader_options(cfg);J *canceled=EXPECT("active cancel",otdf_decrypt_bytes(holder,(otdf_bytes){archive.data,archive.length},&ro,&call,&out,&e_));
 pthread_join(canceller,NULL);check(atomic_load(&held_started),"provider started");check(same_kind(canceled,"canceled"),"active provider cancel is typed");
 otdf_sdk_close(holder,NULL);otdf_sdk_free(holder);
 otdf_tdf_object *object=NULL;otdf_owned_bytes async={0};otdf_tdf_options o=tdf_options(cfg,&kas);TRY("async encrypt",otdf_create_tdf(sdk,(otdf_bytes){(const uint8_t *)"a\0\xff",3},&o,NULL,&async,&object,&e_));otdf_tdf_object_close(object,NULL);otdf_tdf_object_free(object);
 ro=reader_options(cfg);TRY("async decrypt",otdf_decrypt_bytes(sdk,(otdf_bytes){async.data,async.length},&ro,NULL,&out,&e_));check(out.length==3&&!memcmp(out.data,"a\0\xff",3),"second round trip");otdf_bytes_free(&out);otdf_bytes_free(&async);
 otdf_sdk_close(sdk,NULL);
 check(same_kind(EXPECT("closed",otdf_decrypt_bytes(sdk,(otdf_bytes){archive.data,archive.length},NULL,NULL,&out,&e_)),"closed"),"closed handle");otdf_sdk_free(sdk);
 otdf_bytes_free(&archive);
}

/* One controlled case against the harness fixture (tests/interop/delivery/controlled_platform.py). */
static void controlled(const char *run,J *cfg,const char *name){
 const char *url=text(cfg,"PlatformURL");
 otdf_kas_info malicious={.url="http://localhost:9999/malicious/kas",.public_key=text(cfg,"KASPublicKeyPEM"),.kid="k1",.algorithm="rsa:2048"};const char *attr="https://example.com/attr/a/value/b";
 otdf_tdf_options offline=OTDF_TDF_OPTIONS_INIT;offline.disable_autoconfigure=true;offline.kas_information=&malicious;offline.kas_information_count=1;offline.attributes=&attr;offline.attribute_count=1;
 otdf_options credentials=OTDF_OPTIONS_INIT;credentials.client_id="opentdf";credentials.client_secret="secret";
 otdf_sdk *sdk=NULL;J *error;
 if(!strcmp(name,"unreachable")){otdf_options o=OTDF_OPTIONS_INIT;o.validate_connection=true;error=EXPECT(name,otdf_sdk_new(url,&o,NULL,&sdk,&e_));}
 else if(!strcmp(name,"caller-deadline")){
  atomic_bool cancel=false;otdf_call call={.canceled=&cancel};pthread_t timer;
  void *flip(void *flag){struct timespec pause={1,0};nanosleep(&pause,NULL);atomic_store((atomic_bool *)flag,true);return NULL;}
  assert(!pthread_create(&timer,NULL,flip,&cancel));error=EXPECT(name,otdf_sdk_new(url,&credentials,&call,&sdk,&e_));pthread_join(timer,NULL);
 }
 else if(!strcmp(name,"source-deadline")){otdf_options o=credentials;o.timeout_millis=100;error=EXPECT(name,otdf_sdk_new(url,&o,NULL,&sdk,&e_));}
 else if(!strcmp(name,"token-401")){
  TRY(name,otdf_sdk_new(url,&credentials,NULL,&sdk,&e_));otdf_owned_bytes archive={0};TRY("offline encrypt",otdf_encrypt_bytes(sdk,(otdf_bytes){(const uint8_t *)"offline payload",15},&offline,NULL,&archive,&e_));
  otdf_reader_options ro=OTDF_READER_OPTIONS_INIT;ro.ignore_allowlist=true;otdf_owned_bytes out={0};
  error=EXPECT(name,otdf_decrypt_bytes(sdk,(otdf_bytes){archive.data,archive.length},&ro,NULL,&out,&e_));otdf_bytes_free(&archive);otdf_sdk_close(sdk,NULL);otdf_sdk_free(sdk);sdk=NULL;
 }
 else if(!strcmp(name,"offline")){
  J *rows=j_new(J_OBJ);TRY(name,otdf_sdk_new(url,&credentials,NULL,&sdk,&e_));otdf_owned_bytes archive={0};TRY("offline encrypt",otdf_encrypt_bytes(sdk,(otdf_bytes){(const uint8_t *)"offline payload",15},&offline,NULL,&archive,&e_));
  otdf_reader_options ro=OTDF_READER_OPTIONS_INIT;ro.kas_allowlist=&url;ro.kas_allowlist_count=1;otdf_owned_bytes out={0};
  j_set(rows,"allowlist",EXPECT("allowlist",otdf_decrypt_bytes(sdk,(otdf_bytes){archive.data,archive.length},&ro,NULL,&out,&e_)));
  j_set(rows,"invalid-tdf",EXPECT("invalid-tdf",otdf_decrypt_bytes(sdk,(otdf_bytes){(const uint8_t *)"not a tdf",9},NULL,NULL,&out,&e_)));
  otdf_tdf_options legacy=OTDF_TDF_OPTIONS_INIT;legacy.target_mode="4.2.2";otdf_owned_bytes tdf={0};
  j_set(rows,"unsupported-option",EXPECT("unsupported-option",otdf_encrypt_bytes(sdk,(otdf_bytes){(const uint8_t *)"x",1},&legacy,NULL,&tdf,&e_)));
  j_set(rows,"invalid-argument",EXPECT("invalid-argument",otdf_encrypt_bytes(NULL,(otdf_bytes){(const uint8_t *)"x",1},NULL,NULL,&tdf,&e_)));
  for(int i=0;i<100;i++){otdf_reader *reader=NULL;otdf_reader_options ignore=OTDF_READER_OPTIONS_INIT;ignore.ignore_allowlist=true;TRY("load",otdf_load_tdf(sdk,(otdf_bytes){archive.data,archive.length},&ignore,NULL,&reader,&e_));char *manifest=NULL;TRY("manifest",otdf_reader_manifest_json(reader,NULL,&manifest,&e_));check(strstr(manifest,"\"kid\":\"k1\"")!=NULL,"manifest");free(manifest);otdf_reader_close(reader,NULL);otdf_reader_free(reader);}
  otdf_sdk_close(sdk,NULL);
  j_set(rows,"closed-handle",EXPECT("closed-handle",otdf_decrypt_bytes(sdk,(otdf_bytes){archive.data,archive.length},NULL,NULL,&out,&e_)));otdf_sdk_free(sdk);sdk=NULL;
  J *handles=j_new(J_OBJ);J *readers=j_new(J_NUM);readers->s=GC_strdup("100");readers->n=3;j_set(handles,"readers",readers);J *t=j_new(J_BOOL);t->b=true;j_set(handles,"closed",t);j_set(rows,"handles",handles);
  bool valid=false;char *version=NULL;char **features=NULL;size_t count=0;TRY("valid",otdf_is_valid_tdf((otdf_bytes){archive.data,archive.length},NULL,&valid,&e_));TRY("version",otdf_version(&version,&e_));TRY("features",otdf_supported_features(&features,&count,&e_));bool hexless=false;for(size_t i=0;i<count;i++)if(!strcmp(features[i],"hexless"))hexless=true;check(valid&&*version&&hexless,"free functions");free(version);otdf_strings_free(features,count);
  otdf_bytes_free(&archive);write_json(run,"offline.json",rows);return;
 }
 else error=EXPECT(name,otdf_sdk_new(url,&credentials,NULL,&sdk,&e_));
 if(sdk){otdf_sdk_close(sdk,NULL);otdf_sdk_free(sdk);}
 char file[256];snprintf(file,sizeof file,"%s.error.json",name);write_json(run,file,error);
}

int main(int argc,char **argv){assert(argc==5);GC_INIT();GC_allow_register_threads();const char *run=argv[2],*mode=argv[3],*name=argv[4];char path[4096];snprintf(path,sizeof path,"%s/config.json",run);size_t n;uint8_t *raw=readfile(path,&n);assert(raw);const char *parse_error=NULL;J *cfg=j_parse((char *)raw,&parse_error);assert(cfg&&!parse_error);
 if(!strcmp(mode,"controlled")){controlled(run,cfg,name);printf("PASS generated native library %s %s\n",mode,name);return 0;}
 provider_state ps={cfg,name};otdf_options o=options_of(cfg,&ps,true);otdf_sdk *sdk=NULL;otdf_error e={0};
 if(otdf_sdk_new(text(cfg,"PlatformURL"),&o,NULL,&sdk,&e))fail("new",&e);
 if(!strcmp(mode,"encrypt")){
  otdf_kas_info kas;otdf_tdf_options options=tdf_options(cfg,&kas);const char *denied="https://example.com/attr/attr1/value/value2";
  if(strstr(name,"metadata")&&!strstr(name,"empty-metadata"))options.metadata="{\"source\":\"independent metadata\",\"count\":7}";
  if(strstr(name,"denied")){options.attributes=&denied;}
  snprintf(path,sizeof path,"%s/%s.input",run,name);uint8_t *input=readfile(path,&n);assert(input);
  otdf_owned_bytes tdf={0};otdf_tdf_object *object=NULL;if(otdf_create_tdf(sdk,(otdf_bytes){input,n},&options,NULL,&tdf,&object,&e))fail("encrypt",&e);
  int64_t size=0;char *manifest=NULL;if(otdf_tdf_object_size(object,NULL,&size,&e)||otdf_tdf_object_manifest_json(object,NULL,&manifest,&e))fail("object",&e);check(size==(int64_t)tdf.length&&strstr(manifest,"\"keyAccess\":[{")!=NULL,"object");free(manifest);otdf_tdf_object_close(object,NULL);otdf_tdf_object_free(object);
  snprintf(path,sizeof path,"%s/%s.generated.tdf",run,name);writefile(path,tdf.data,tdf.length);otdf_bytes_free(&tdf);free(input);
 }else if(!strcmp(mode,"decrypt")){
  snprintf(path,sizeof path,"%s/%s.tdf",run,name);uint8_t *input=readfile(path,&n);assert(input);
  otdf_reader *reader=NULL;otdf_reader_options ro=reader_options(cfg);if(otdf_load_tdf(sdk,(otdf_bytes){input,n},&ro,NULL,&reader,&e))fail("load",&e);
  char *manifest=NULL;if(otdf_reader_manifest_json(reader,NULL,&manifest,&e))fail("manifest",&e);
  const char *em=strstr(manifest,"\"encryptedMetadata\":\"");bool has_metadata=em&&em[21]!='"';
  otdf_owned_bytes metadata={0};if(has_metadata){if(otdf_reader_init(reader,NULL,&e)||otdf_reader_unencrypted_metadata(reader,NULL,&metadata,&e))fail("metadata",&e);}
  otdf_reader_close(reader,NULL);otdf_reader_free(reader);
  otdf_owned_bytes payload={0};ro=reader_options(cfg);if(otdf_decrypt_bytes(sdk,(otdf_bytes){input,n},&ro,NULL,&payload,&e))fail("decrypt",&e);
  snprintf(path,sizeof path,"%s/%s.out",run,name);writefile(path,payload.data,payload.length);snprintf(path,sizeof path,"%s/%s.metadata",run,name);writefile(path,metadata.data,metadata.length);snprintf(path,sizeof path,"%s/%s.manifest",run,name);writefile(path,manifest,strlen(manifest));snprintf(path,sizeof path,"%s/%s.presence",run,name);writefile(path,has_metadata?"true":"false",has_metadata?4:5);
  otdf_bytes_free(&payload);otdf_bytes_free(&metadata);free(manifest);free(input);
 }else if(!strcmp(mode,"negative")){
  snprintf(path,sizeof path,"%s/%s.tdf",run,name);uint8_t *input=readfile(path,&n);assert(input);otdf_owned_bytes out={0};otdf_reader_options ro=reader_options(cfg);
  J *error=EXPECT("negative",otdf_decrypt_bytes(sdk,(otdf_bytes){input,n},&ro,NULL,&out,&e_));char file[256];snprintf(file,sizeof file,"%s.error.json",name);write_json(run,file,error);free(input);
 }else if(!strcmp(mode,"repeat")){repeat(sdk,cfg,name);sdk=NULL;}
 else if(!strcmp(mode,"services"))services(sdk,run);
 else{fprintf(stderr,"unknown mode\n");return 1;}
 if(sdk){otdf_sdk_close(sdk,NULL);otdf_sdk_free(sdk);}
 printf("PASS generated native library %s %s\n",mode,name);return 0;
}
