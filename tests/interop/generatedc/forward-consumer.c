// SPDX-License-Identifier: Apache-2.0
#define _POSIX_C_SOURCE 200809L
#include "opentdf.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
static uint8_t *readfile(const char *p,size_t *n){FILE *f=fopen(p,"rb");assert(f);assert(!fseek(f,0,SEEK_END));long size=ftell(f);assert(size>=0);*n=(size_t)size;assert(!fseek(f,0,SEEK_SET));uint8_t *b=malloc(*n?*n:1);assert(b);assert(fread(b,1,*n,f)==*n);assert(!fclose(f));return b;}
static void writefile(const char *p,const void *data,size_t n){FILE *f=fopen(p,"wb");assert(f);assert(!n||fwrite(data,1,n,f)==n);assert(!fclose(f));}
static void check(int rc,otdf_error *e){if(rc)fprintf(stderr,"rc%d kind %s op %s message %s\n",rc,e->kind?e->kind:"",e->op?e->op:"",e->message?e->message:"");assert(!rc);otdf_error_free(e);}
int main(int argc,char **argv){
 assert(argc==6);setbuf(stdout,NULL);size_t n;uint8_t *archive=readfile(argv[1],&n);
 otdf_options o=OTDF_OPTIONS_INIT;o.client_id="opentdf-sdk";o.client_secret="secret";o.insecure_plaintext=true;
 const char *allowlist[]={"http://localhost:8080/kas",argv[2]};
 otdf_reader_options r=OTDF_READER_OPTIONS_INIT;r.kas_allowlist=allowlist;r.kas_allowlist_count=2;
 otdf_error e={0};otdf_sdk *sdk=NULL;otdf_reader *reader=NULL;char *manifest=NULL;otdf_owned_bytes payload={0},metadata={0};
 puts("first native entry is the public handle API; no consumer collector initialization/registration");
 check(otdf_sdk_new("http://localhost:8080",&o,NULL,&sdk,&e),&e);
 check(otdf_load_tdf(sdk,(otdf_bytes){archive,n},&r,NULL,&reader,&e),&e);
 check(otdf_reader_manifest_json(reader,NULL,&manifest,&e),&e);
 bool has_metadata=strstr(manifest,"\"encryptedMetadata\"")!=NULL;
 if(has_metadata)check(otdf_reader_unencrypted_metadata(reader,NULL,&metadata,&e),&e);
 check(otdf_decrypt_bytes(sdk,(otdf_bytes){archive,n},&r,NULL,&payload,&e),&e);
 writefile(argv[3],payload.data,payload.length);writefile(argv[4],metadata.data,metadata.length);
 FILE *f=fopen(argv[5],"w");assert(f);fprintf(f,"%d\n",has_metadata);assert(!fclose(f));
 otdf_bytes_free(&payload);otdf_bytes_free(&metadata);otdf_string_free(manifest);
 check(otdf_reader_close(reader,&e),&e);otdf_reader_free(reader);
 check(otdf_decrypt_bytes(sdk,(otdf_bytes){archive,n},&r,NULL,&payload,&e),&e);otdf_bytes_free(&payload);
 check(otdf_sdk_close(sdk,&e),&e);otdf_sdk_free(sdk);free(archive);
 puts("PASS real KAS decrypt through the public handle API, first owner and a subsequent call on the same handle");return 0;}
