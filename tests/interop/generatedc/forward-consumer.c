// SPDX-License-Identifier: Apache-2.0
#define _POSIX_C_SOURCE 200809L
#include "tdf3.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static tdf3_bytes text(const char *s){return(tdf3_bytes){(const uint8_t *)s,strlen(s)};}
static uint8_t *readfile(const char *p,size_t *n){FILE *f=fopen(p,"rb");assert(f);assert(!fseek(f,0,SEEK_END));long size=ftell(f);assert(size>=0);*n=(size_t)size;assert(!fseek(f,0,SEEK_SET));uint8_t *b=malloc(*n?*n:1);assert(b);assert(fread(b,1,*n,f)==*n);assert(!fclose(f));return b;}
static void writefile(const char *p,const void *data,size_t n){FILE *f=fopen(p,"wb");assert(f);assert(!n||fwrite(data,1,n,f)==n);assert(!fclose(f));}
int main(int argc,char **argv){assert(argc==6);setbuf(stdout,NULL);size_t n;uint8_t *archive=readfile(argv[1],&n);tdf3_kas_route route={text("http://localhost:8080/kas"),text(argv[2])};tdf3_config c={.PlatformURL=text("http://localhost:8080"),.KASURL=text("http://localhost:8080/kas"),.IssuerURL=text("http://localhost:8888/auth/realms/opentdf"),.ClientID=text("opentdf-sdk"),.ClientSecret=text("secret"),.AllowHTTP=true,.AllowedKAS=&route,.AllowedKASLength=1};tdf3_error e={0};tdf3_result r={0};puts("first native entry is async decrypt; no consumer collector initialization/registration");tdf3_operation *op=tdf3_decrypt_submit(&c,(tdf3_bytes){archive,n},NULL,&e);assert(op);assert(tdf3_operation_drive(op,-1));int rc=tdf3_operation_take(op,&r,&e);tdf3_operation_destroy(&op);if(rc)fprintf(stderr,"kind%d code%.*s message%.*s\n",rc,(int)e.Code.length,e.Code.data,(int)e.Message.length,e.Message.data);assert(!rc);writefile(argv[3],r.Payload.data,r.Payload.length);writefile(argv[4],r.Metadata.data,r.Metadata.length);FILE *f=fopen(argv[5],"w");assert(f);fprintf(f,"%d\n",r.HasMetadata);assert(!fclose(f));tdf3_result_free(&r);tdf3_error_free(&e);assert(!tdf3_decrypt(&c,(tdf3_bytes){archive,n},NULL,&r,&e));tdf3_result_free(&r);tdf3_error_free(&e);free(archive);puts("PASS real KAS compressed forwarding, first async owner and subsequent synchronous call");return 0;}
