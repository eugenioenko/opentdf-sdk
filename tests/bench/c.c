// C handle-API benchmark: one otdf_sdk per process, as the reference consumer.
#define _POSIX_C_SOURCE 200809L
#include "opentdf.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <assert.h>
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
// The pre-acquired token-response.json already carries {"value","scheme","expiresAt","confirmationJKT"}.
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
static void fail(const char *label,otdf_error *e){
    fprintf(stderr,"C failure %s: kind %s op %s: %s\n",label,e->kind?e->kind:"",e->op?e->op:"",e->message?e->message:"");
    exit(1);
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
    if (bulk_warmups) { snprintf(path, sizeof path, "%s/bulk.input", run); bulk_input = readfile(path, &bulk_n); }
    snprintf(path, sizeof path, "%s/kas.pem", run);
    uint8_t *pem = readfile(path, &pn);
    snprintf(path, sizeof path, "%s/kid", run);
    uint8_t *kid = readfile(path, &kn);
    while (kn && (kid[kn - 1] == '\n' || kid[kn - 1] == '\r')) kid[--kn] = 0;
    snprintf(path, sizeof path, "%s/token-response.json", run);
    uint8_t *token = readfile(path, &tn);
    const char *platform = "http://localhost:8080", *kas_url = "http://localhost:8080/kas";
    otdf_options sdk_options = OTDF_OPTIONS_INIT;
    sdk_options.insecure_plaintext = true;
    sdk_options.token_endpoint = "http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token";
    sdk_options.token_provider = provider;
    sdk_options.token_provider_state = token;
    otdf_error err = {0};
    otdf_sdk *sdk = NULL;
    if (otdf_sdk_new(platform, &sdk_options, NULL, &sdk, &err)) fail("new", &err);
    const char *attribute = "https://example.com/attr/attr1/value/value1";
    otdf_kas_info kas = {.url = kas_url, .public_key = (const char *)pem, .kid = (const char *)kid, .algorithm = "rsa:2048", .is_default = false};
    otdf_tdf_options options = OTDF_TDF_OPTIONS_INIT;
    options.disable_autoconfigure = true;
    options.kas_information = &kas;
    options.kas_information_count = 1;
    options.wrapping_key_algorithm = "rsa:2048";
    options.segment_size = 2 << 20;
    options.attributes = &attribute;
    options.attribute_count = 1;
    // load_tdf reuses the SDK's default RSA2048 session. Do not generate a replacement.
    otdf_reader_options reader_options = OTDF_READER_OPTIONS_INIT;
    reader_options.kas_allowlist = &kas_url;
    reader_options.kas_allowlist_count = 1;
    printf("{\"samples_ms\":[");
    for (int i = -bulk_warmups - warmups; i < samples; i++) {
        uint8_t *pair_input = i < -warmups ? bulk_input : input;
        size_t pair_n = i < -warmups ? bulk_n : n;
        otdf_owned_bytes archive = {0}, plaintext = {0};
        double start = clockms();
        if (otdf_encrypt_bytes(sdk, (otdf_bytes){pair_input, pair_n}, &options, NULL, &archive, &err)) fail("encrypt", &err);
        if (otdf_decrypt_bytes(sdk, (otdf_bytes){archive.data, archive.length}, &reader_options, NULL, &plaintext, &err)) fail("decrypt", &err);
        double elapsed = clockms() - start;
        assert(plaintext.length == pair_n && !memcmp(plaintext.data, pair_input, pair_n));
        snprintf(path, sizeof path, "%s/c-%s-%d.tdf", run, size, i);
        if (i == -1 || i >= 0) writefile(path, archive.data, archive.length);
        otdf_bytes_free(&archive);
        otdf_bytes_free(&plaintext);
        if (i >= 0) printf("%s%.9f", i ? "," : "", elapsed);
        else if (i < -warmups) bulk_history[i + bulk_warmups + warmups] = elapsed;
        else warmup_history[i + warmups] = elapsed;
        fflush(stdout);
    }
    if (otdf_sdk_close(sdk, &err)) fail("close", &err);
    otdf_sdk_free(sdk);
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
