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
int main(int argc, char **argv) {
    assert(argc >= 5 && argc <= 7);
    const char *run = argv[1], *op = argv[2], *size = argv[3];
    assert(!strcmp(op, "e2e"));
    int samples = atoi(argv[4]);
    int warmups = argc > 5 ? atoi(argv[5]) : 1;
    int bulk_warmups = argc > 6 ? atoi(argv[6]) : 0;
    assert(samples > 0 && warmups > 0 && bulk_warmups >= 0);
    double *warmup_history = calloc((size_t)warmups, sizeof(double));
    double *bulk_history = calloc((size_t)(bulk_warmups ? bulk_warmups : 1), sizeof(double));
    assert(warmup_history && bulk_history);
    char path[4096];
    size_t n, pn, kn, tn;
    snprintf(path, sizeof path, "%s/%s.input", run, size);
    uint8_t *input = readfile(path, &n);
    size_t bulk_n = n;
    uint8_t *bulk_input = input;
    if (bulk_warmups && strcmp(size, "50")) { snprintf(path, sizeof path, "%s/50.input", run); bulk_input = readfile(path, &bulk_n); }
    snprintf(path, sizeof path, "%s/kas.pem", run);
    uint8_t *pem = readfile(path, &pn);
    snprintf(path, sizeof path, "%s/kid", run);
    uint8_t *kid = readfile(path, &kn);
    snprintf(path, sizeof path, "%s/token-response.json", run);
    uint8_t *token = readfile(path, &tn);
    tdf3_kas_route route = {text("http://localhost:8080/kas"), text("http://localhost:8080")};
    tdf3_config cfg = {
        .PlatformURL = text("http://localhost:8080"), .KASURL = text("http://localhost:8080/kas"),
        .AllowHTTP = true, .AllowedKAS = &route, .AllowedKASLength = 1,
        .KASPublicKeyPEM = {pem, pn}, .KID = {kid, kn},
        .KASAlgorithm = text("rsa:2048"), .SessionAlgorithm = text("rsa:2048"),
        .AuthAlgorithm = text("ES256"), .TokenProviderName = text("access-token")
    };
    tdf3_bytes attr = text("https://example.com/attr/attr1/value/value1");
    tdf3_encrypt_options options = {
        .Attributes = &attr, .AttributesLength = 1, .SegmentSize = 2 << 20,
        .HasSegmentSize = true, .SegmentHashAlgorithm = text("GMAC")
    };
    gxc_options call = {.provider = provider, .provider_state = token};
    printf("{\"samples_ms\":[");
    for (int i = -bulk_warmups - warmups; i < samples; i++) {
        uint8_t *pair_input = i < -warmups ? bulk_input : input;
        size_t pair_n = i < -warmups ? bulk_n : n;
        tdf3_result archive = {0}, plaintext = {0};
        tdf3_error err = {0};
        double start = clockms();
        int rc = tdf3_encrypt(&cfg, (tdf3_bytes){pair_input, pair_n}, &options, &call, &archive, &err);
        if (!rc) rc = tdf3_decrypt(&cfg, (tdf3_bytes){archive.Payload.data, archive.Payload.length}, &call, &plaintext, &err);
        double elapsed = clockms() - start;
        if (rc) {
            fprintf(stderr, "C failure kind %d code %.*s\n", rc, (int)err.Code.length, err.Code.data);
            return 1;
        }
        assert(plaintext.Payload.length == pair_n && !memcmp(plaintext.Payload.data, pair_input, pair_n));
        snprintf(path, sizeof path, "%s/c-%s-%d.tdf", run, size, i);
        if (i == -1 || i >= 0) writefile(path, archive.Payload.data, archive.Payload.length);
        tdf3_result_free(&archive);
        tdf3_result_free(&plaintext);
        tdf3_error_free(&err);
        if (i >= 0) printf("%s%.9f", i ? "," : "", elapsed);
        else if (i < -warmups) bulk_history[i + bulk_warmups + warmups] = elapsed;
        else warmup_history[i + warmups] = elapsed;
        fflush(stdout);
    }
    printf("],\"warmup_ms\":[");
    for (int i = 0; i < warmups; i++) printf("%s%.9f", i ? "," : "", warmup_history[i]);
    printf("],\"bulk_warmup_ms\":[");
    for (int i = 0; i < bulk_warmups; i++) printf("%s%.9f", i ? "," : "", bulk_history[i]);
    printf("],\"warmup_count\":%d,\"bulk_warmup_count\":%d,\"correct\":true,\"kas_calls_expected\":%d}\n", warmups, bulk_warmups, samples + warmups + bulk_warmups);
    free(warmup_history); free(bulk_history);
    if (bulk_input != input) free(bulk_input);
    free(input);
    free(pem);
    free(kid);
    free(token);
    return 0;
}
