#define _POSIX_C_SOURCE 200809L
#include "tdf3.h"
#include <stdlib.h>
#include <string.h>
#include <pthread.h>
#include <time.h>
#include <errno.h>
#include <limits.h>
static gxc_value bytes(tdf3_bytes b){return (gxc_value){.kind=GXC_BYTES,.bytes=(uint8_t *)b.data,.length=b.length};}
static gxc_value integer(int64_t n){return(gxc_value){.kind=GXC_INT,.integer=n};}
static gxc_value boolean(bool b){return(gxc_value){.kind=GXC_BOOL,.integer=b};}
static gxc_value record(gxc_value *v,char **names,size_t n){return(gxc_value){.kind=GXC_RECORD,.items=v,.names=names,.length=n};}
static tdf3_owned_bytes take_bytes(gxc_value *v){if(v->kind!=GXC_BYTES)return (tdf3_owned_bytes){0};tdf3_owned_bytes b={v->bytes,v->length};v->bytes=NULL;v->length=0;return b;}
static tdf3_owned_bytes take_field(gxc_value *v,const char *name){return take_bytes((gxc_value *)gxc_field(v,name));}
void tdf3_result_free(tdf3_result *r){if(!r)return;free(r->Payload.data);free(r->Metadata.data);free(r->ManifestJSON.data);memset(r,0,sizeof(*r));}
void tdf3_error_free(tdf3_error *e){if(!e)return;free(e->Message.data);free(e->Code.data);free(e->Operation.data);free(e->ServerCode.data);free(e->ServerMessage.data);free(e->CauseCategory.data);for(size_t i=0;i<e->RequiredObligationsLength;i++)free(e->RequiredObligations[i].data);free(e->RequiredObligations);memset(e,0,sizeof(*e));}
static int error_take(tdf3_error *e,gxc_error *source){e->Kind=source->kind;e->Message=take_bytes(&source->message);e->Code=take_field(&source->fields,"Code");e->Operation=take_field(&source->fields,"Operation");e->ServerCode=take_field(&source->fields,"ServerCode");e->ServerMessage=take_field(&source->fields,"ServerMessage");e->CauseCategory=take_field(&source->fields,"CauseCategory");e->HTTPStatus=gxc_field(&source->fields,"HTTPStatus")->integer;
 gxc_value *list=(gxc_value *)gxc_field(&source->fields,"RequiredObligations");if(list->kind==GXC_LIST&&list->length){e->RequiredObligations=calloc(list->length,sizeof(*e->RequiredObligations));if(!e->RequiredObligations){e->Kind=3;}else{e->RequiredObligationsLength=list->length;for(size_t i=0;i<list->length;i++)e->RequiredObligations[i]=take_bytes(&list->items[i]);}}
 gxc_error_free(source);return e->Kind;}
static void result_take(tdf3_result *r,gxc_value *value,bool decrypt){if(decrypt){r->Payload=take_field(value,"Payload");r->Metadata=take_field(value,"Metadata");r->ManifestJSON=take_field(value,"ManifestJSON");r->HasMetadata=gxc_field(value,"HasMetadata")->integer!=0;}else r->Payload=take_bytes(value);gxc_value_free(value);}
static int snapshot(const tdf3_config *c,tdf3_bytes input,const tdf3_encrypt_options *o,gxc_value args[3]){
 if(!c || c->AllowedKASLength>1000000 || (c->AllowedKASLength&&!c->AllowedKAS) || (o && (o->AttributesLength>1000000||o->DissemLength>1000000||(o->AttributesLength&&!o->Attributes)||(o->DissemLength&&!o->Dissem))))return 5;
 gxc_value *routes=calloc(c->AllowedKASLength?c->AllowedKASLength:1,sizeof(*routes)),*pairs=calloc(c->AllowedKASLength?c->AllowedKASLength*2:1,sizeof(*pairs));if(!routes||!pairs){free(routes);free(pairs);return 3;}
 char *route_names[]={"URL","APIBaseURL"};for(size_t i=0;i<c->AllowedKASLength;i++){pairs[2*i]=bytes(c->AllowedKAS[i].URL);pairs[2*i+1]=bytes(c->AllowedKAS[i].APIBaseURL);routes[i]=record(&pairs[2*i],route_names,2);}
 char *names[]={"PlatformURL","KASURL","AllowedKAS","IssuerURL","TokenURL","ClientID","ClientSecret","TokenProviderName","AllowHTTP","TimeoutMillis","KASPublicKeyPEM","KID","KASAlgorithm","SessionAlgorithm","AuthPrivateKeyPEM","AuthAlgorithm","DPoP"};
 gxc_value values[]={bytes(c->PlatformURL),bytes(c->KASURL),{.kind=GXC_LIST,.items=routes,.length=c->AllowedKASLength},bytes(c->IssuerURL),bytes(c->TokenURL),bytes(c->ClientID),bytes(c->ClientSecret),bytes(c->TokenProviderName),boolean(c->AllowHTTP),integer(c->TimeoutMillis),bytes(c->KASPublicKeyPEM),bytes(c->KID),bytes(c->KASAlgorithm),bytes(c->SessionAlgorithm),bytes(c->AuthPrivateKeyPEM),bytes(c->AuthAlgorithm),boolean(c->DPoP)};
 gxc_value config=record(values,names,17),payload=bytes(input);int rc=gxc_value_copy(&args[0],&config);free(routes);free(pairs);if(rc)return rc==3?3:5;
 rc=gxc_value_copy(&args[1],&payload);if(rc)return rc==3?3:5;
 if(o){gxc_value *attr=calloc(o->AttributesLength?o->AttributesLength:1,sizeof(*attr)),*dissem=calloc(o->DissemLength?o->DissemLength:1,sizeof(*dissem));if(!attr||!dissem){free(attr);free(dissem);return 3;}for(size_t i=0;i<o->AttributesLength;i++)attr[i]=bytes(o->Attributes[i]);for(size_t i=0;i<o->DissemLength;i++)dissem[i]=bytes(o->Dissem[i]);
 char *onames[]={"PolicyBase64","Attributes","Dissem","SegmentSize","HasSegmentSize","SegmentHashAlgorithm","MimeType","Metadata","IncludeMetadata"};
 gxc_value options[]={bytes(o->PolicyBase64),{.kind=GXC_LIST,.items=attr,.length=o->AttributesLength},{.kind=GXC_LIST,.items=dissem,.length=o->DissemLength},integer(o->SegmentSize),boolean(o->HasSegmentSize),bytes(o->SegmentHashAlgorithm),bytes(o->MimeType),bytes(o->Metadata),boolean(o->IncludeMetadata)};gxc_value v=record(options,onames,9);rc=gxc_value_copy(&args[2],&v);free(attr);free(dissem);if(rc)return rc==3?3:5;
 }return 0;
}
static void args_free(gxc_value args[3]){for(int i=0;i<3;i++)gxc_value_free(&args[i]);}
static int invoke(bool decrypt,gxc_value args[3],const gxc_options *opts,tdf3_result *r,tdf3_error *e){gxc_value value={0};gxc_error failure={0};int rc=goalchemy_invoke(decrypt?"Decrypt":"Encrypt",args,decrypt?2:3,opts,&value,&failure);if(rc)return error_take(e,&failure);result_take(r,&value,decrypt);return 0;}
int tdf3_encrypt(const tdf3_config *c,tdf3_bytes input,const tdf3_encrypt_options *o,const gxc_options *opts,tdf3_result *r,tdf3_error *e){if(!r||!e)return 5;memset(r,0,sizeof(*r));memset(e,0,sizeof(*e));gxc_value args[3]={0};tdf3_encrypt_options zero={0};int rc=snapshot(c,input,o?o:&zero,args);if(!rc)rc=invoke(false,args,opts,r,e);else e->Kind=rc;args_free(args);return rc;}
int tdf3_decrypt(const tdf3_config *c,tdf3_bytes input,const gxc_options *opts,tdf3_result *r,tdf3_error *e){if(!r||!e)return 5;memset(r,0,sizeof(*r));memset(e,0,sizeof(*e));gxc_value args[3]={0};int rc=snapshot(c,input,NULL,args);if(!rc)rc=invoke(true,args,opts,r,e);else e->Kind=rc;args_free(args);return rc;}
struct tdf3_operation{pthread_t thread;pthread_mutex_t mutex;pthread_cond_t wake;atomic_bool canceled;gxc_value args[3];gxc_options options;bool decrypt,done,taken;tdf3_result result;tdf3_error error;};
static void *operation_thread(void *arg){tdf3_operation *op=arg;invoke(op->decrypt,op->args,&op->options,&op->result,&op->error);args_free(op->args);pthread_mutex_lock(&op->mutex);op->done=true;pthread_cond_broadcast(&op->wake);pthread_mutex_unlock(&op->mutex);return NULL;}
static tdf3_operation *submit(bool decrypt,const tdf3_config *c,tdf3_bytes input,const tdf3_encrypt_options *o,const gxc_options *opts,tdf3_error *e){if(!e)return NULL;memset(e,0,sizeof(*e));tdf3_operation *op=calloc(1,sizeof(*op));if(!op){e->Kind=3;return NULL;}int rc=snapshot(c,input,o,op->args);if(rc){args_free(op->args);free(op);e->Kind=rc;return NULL;}op->decrypt=decrypt;if(opts)op->options=*opts;atomic_init(&op->canceled,opts&&opts->canceled&&atomic_load(opts->canceled));op->options.canceled=&op->canceled;
 rc=pthread_mutex_init(&op->mutex,NULL);if(rc){args_free(op->args);free(op);e->Kind=3;return NULL;}rc=pthread_cond_init(&op->wake,NULL);if(rc){pthread_mutex_destroy(&op->mutex);args_free(op->args);free(op);e->Kind=3;return NULL;}
 rc=pthread_create(&op->thread,NULL,operation_thread,op);if(rc){pthread_cond_destroy(&op->wake);pthread_mutex_destroy(&op->mutex);args_free(op->args);free(op);e->Kind=3;return NULL;}return op;}
tdf3_operation *tdf3_encrypt_submit(const tdf3_config *c,tdf3_bytes input,const tdf3_encrypt_options *o,const gxc_options *opts,tdf3_error *e){tdf3_encrypt_options zero={0};return submit(false,c,input,o?o:&zero,opts,e);}
tdf3_operation *tdf3_decrypt_submit(const tdf3_config *c,tdf3_bytes input,const gxc_options *opts,tdf3_error *e){return submit(true,c,input,NULL,opts,e);}
void tdf3_operation_cancel(tdf3_operation *op){if(op){atomic_store(&op->canceled,true);tdf3_operation_wake(op);}}
void tdf3_operation_wake(tdf3_operation *op){if(op){goalchemy_wake();pthread_mutex_lock(&op->mutex);pthread_cond_broadcast(&op->wake);pthread_mutex_unlock(&op->mutex);}}
bool tdf3_operation_drive(tdf3_operation *op,int64_t millis){if(!op)return true;pthread_mutex_lock(&op->mutex);if(millis<0){while(!op->done)pthread_cond_wait(&op->wake,&op->mutex);}else if(millis>0&&!op->done){if(millis>86400000)millis=86400000;struct timespec until;clock_gettime(CLOCK_REALTIME,&until);until.tv_sec+=(time_t)(millis/1000);until.tv_nsec+=(long)(millis%1000)*1000000;if(until.tv_nsec>=1000000000){until.tv_sec++;until.tv_nsec-=1000000000;}while(!op->done){int rc=pthread_cond_timedwait(&op->wake,&op->mutex,&until);if(rc==ETIMEDOUT)break;}}bool done=op->done;pthread_mutex_unlock(&op->mutex);return done;}
int tdf3_operation_take(tdf3_operation *op,tdf3_result *r,tdf3_error *e){if(!op||!r||!e)return 5;pthread_mutex_lock(&op->mutex);if(!op->done||op->taken){pthread_mutex_unlock(&op->mutex);return 5;}*r=op->result;*e=op->error;memset(&op->result,0,sizeof(op->result));memset(&op->error,0,sizeof(op->error));op->taken=true;pthread_mutex_unlock(&op->mutex);return e->Kind;}
void tdf3_operation_destroy(tdf3_operation **handle){if(!handle||!*handle)return;tdf3_operation *op=*handle;tdf3_operation_cancel(op);if(pthread_join(op->thread,NULL))abort();tdf3_result_free(&op->result);tdf3_error_free(&op->error);args_free(op->args);pthread_cond_destroy(&op->wake);pthread_mutex_destroy(&op->mutex);free(op);*handle=NULL;}
