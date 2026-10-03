#define _POSIX_C_SOURCE 200809L
#include "tdf3.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <assert.h>
static tdf3_bytes text(const char *s){
    return(tdf3_bytes){
        (const uint8_t *)s,strlen(s)
    }
    ;
}
static uint8_t *readfile(const char *p,size_t *n){
    FILE *f=fopen(p,"rb");
    assert(f);
    assert(!fseek(f,0,SEEK_END));
    long size=ftell(f);
    assert(size>=0);
    *n=(size_t)size;
    rewind(f);
    uint8_t *b=malloc(*n+1);
    assert(b);
    assert(fread(b,1,*n,f)==*n);
    b[*n]=0;
    assert(!fclose(f));
    return b;
}
static void writefile(const char *p,const void *data,size_t n){
    FILE *f=fopen(p,"wb");
    assert(f);
    assert(!n||fwrite(data,1,n,f)==n);
    assert(!fclose(f));
}
static int provider(void *state,const uint8_t *name,size_t names,const uint8_t *data,size_t n,const atomic_bool *cancel,gxc_value *out){
    (void)name;
    (void)names;
    (void)data;
    (void)n;
    (void)cancel;
    out->kind=GXC_BYTES;
    out->bytes=(uint8_t *)strdup((char *)state);
    out->length=strlen((char *)state);
    return 0;
}
static double clockms(void){
    struct timespec t;
    assert(!clock_gettime(CLOCK_MONOTONIC,&t));
    return (double)t.tv_sec*1000.+(double)t.tv_nsec/1e6;
}
int main(int argc,char **argv){
    assert(argc==5);
    const char *run=argv[1],*op=argv[2],*size=argv[3];
    int samples=atoi(argv[4]);
    char path[4096];
    size_t n,an=0,pn,kn,tn;
    snprintf(path,sizeof path,"%s/%s.input",run,size);
    uint8_t *input=readfile(path,&n),*archive=NULL;
    if(!strcmp(op,"decrypt")){
        snprintf(path,sizeof path,"%s/%s.reference.tdf",run,size);
        archive=readfile(path,&an);
    }
    snprintf(path,sizeof path,"%s/kas.pem",run);
    uint8_t *pem=readfile(path,&pn);
    snprintf(path,sizeof path,"%s/kid",run);
    uint8_t *kid=readfile(path,&kn);
    snprintf(path,sizeof path,"%s/token-response.json",run);
    uint8_t *token=readfile(path,&tn);
    printf("{\"samples_ms\":[");
    for(int i=-1;i<samples;i++){
        double start=clockms();
        tdf3_kas_route route={
            text("http://localhost:8080/kas"),text("http://localhost:8080")
        }
        ;
        tdf3_config c={
            .PlatformURL=text("http://localhost:8080"),.KASURL=text("http://localhost:8080/kas"),.AllowHTTP=true,.AllowedKAS=&route,.AllowedKASLength=1,.KASPublicKeyPEM={
                pem,pn
            }
            ,.KID={
                kid,kn
            }
            ,.KASAlgorithm=text("rsa:2048"),.SessionAlgorithm=text("rsa:2048"),.AuthAlgorithm=text("ES256"),.TokenProviderName=text("access-token")
        }
        ;
        tdf3_bytes attr=text("https://example.com/attr/attr1/value/value1");
        tdf3_encrypt_options opts={
            .Attributes=&attr,.AttributesLength=1,.SegmentSize=2<<20,.HasSegmentSize=true,.SegmentHashAlgorithm=text("GMAC")
        }
        ;
        gxc_options call={
            .provider=provider,.provider_state=token
        }
        ;
        tdf3_result result={
            0
        }
        ;
        tdf3_error err={
            0
        }
        ;
        int rc=!strcmp(op,"encrypt")?tdf3_encrypt(&c,(tdf3_bytes){
            input,n
        }
        ,&opts,&call,&result,&err):tdf3_decrypt(&c,(tdf3_bytes){
            archive,an
        }
        ,&call,&result,&err);
        double elapsed=clockms()-start;
        if(rc){
            fprintf(stderr,"C failure kind %d code %.*s\n",rc,(int)err.Code.length,err.Code.data);
            return 1;
        }
        if(!strcmp(op,"decrypt")){
            assert(result.Payload.length==n&&!memcmp(result.Payload.data,input,n));
        }
        else{
            snprintf(path,sizeof path,"%s/c-%s-%d.tdf",run,size,i);
            writefile(path,result.Payload.data,result.Payload.length);
        }
        tdf3_result_free(&result);
        tdf3_error_free(&err);
        if(i>=0)printf("%s%.9f",i?",":"",elapsed);
        fflush(stdout);
    }
    printf("],\"correct\":true}\n");
    free(input);
    free(archive);
    free(pem);
    free(kid);
    free(token);
    return 0;
}
