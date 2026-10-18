#define _POSIX_C_SOURCE 200809L
#include "opentdf.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct otdf_sdk { gxc_handle *h; gxc_provider provider; void *state; };
struct otdf_reader { gxc_handle *h; otdf_sdk *sdk; uint8_t *data; size_t length, position; gxc_stream stream; };
struct otdf_tdf_object { gxc_handle *h; otdf_sdk *sdk; };

/* Input values: strings are UTF-8 text, NULL crosses as the zero value. */
static gxc_value str(const char *s) { return s ? (gxc_value){.kind = GXC_BYTES, .bytes = (uint8_t *)s, .length = strlen(s)} : (gxc_value){.kind = GXC_NIL}; }
static gxc_value raw(otdf_bytes b) { return (gxc_value){.kind = GXC_BYTES, .bytes = (uint8_t *)b.data, .length = b.length}; }
static gxc_value integer(int64_t n) { return (gxc_value){.kind = GXC_INT, .integer = n}; }
static gxc_value boolean(bool b) { return (gxc_value){.kind = GXC_BOOL, .integer = b}; }
static gxc_value record(gxc_value *items, char **names, size_t n) { return (gxc_value){.kind = GXC_RECORD, .items = items, .names = names, .length = n}; }
static gxc_value handle(gxc_handle *h) { return (gxc_value){.kind = GXC_HANDLE, .handle = h}; }

/* Lists and nested records are allocated for the call and released after the
 * invocation copied them; an arena keeps the allocations together. */
typedef struct { void **blocks; size_t count, cap; } arena;
static void *keep(arena *a, void *p) {
    if (!p) return NULL;
    if (a->count == a->cap) { a->cap = a->cap ? a->cap * 2 : 16; a->blocks = realloc(a->blocks, a->cap * sizeof(*a->blocks)); if (!a->blocks) abort(); }
    a->blocks[a->count++] = p;
    return p;
}
static void release(arena *a) { for (size_t i = 0; i < a->count; i++) free(a->blocks[i]); free(a->blocks); memset(a, 0, sizeof(*a)); }
static gxc_value strs(arena *a, const char *const *list, size_t n) {
    if (!list) return (gxc_value){.kind = GXC_NIL};
    gxc_value *items = keep(a, calloc(n ? n : 1, sizeof(*items)));
    for (size_t i = 0; i < n; i++) items[i] = str(list[i]);
    return (gxc_value){.kind = GXC_LIST, .items = items, .length = n};
}
#define RECORD(a, names, ...) record_of(a, names, sizeof(names) / sizeof(*names), (gxc_value[]){__VA_ARGS__})
static gxc_value record_of(arena *a, char **names, size_t n, gxc_value *values) {
    gxc_value *items = keep(a, calloc(n, sizeof(*items)));
    memcpy(items, values, n * sizeof(*items));
    return record(items, names, n);
}

static gxc_value options_value(arena *a, const otdf_options *o) {
    static char *names[] = {"ClientID", "ClientSecret", "Scopes", "TokenEndpoint", "TokenExchange", "CertExchange", "TokenProviderName", "PlatformConfiguration", "ValidateConnection", "InsecurePlaintext", "InsecureSkipVerify", "TLS", "UnixSocket", "TimeoutMillis", "CoreEndpoint", "PolicyEndpoint", "AuthorizationEndpoint", "EntityResolutionEndpoint", "WellknownEndpoint", "SessionEncryptionRSAPEM", "SessionSignerRSAPEM", "DPoPAlgorithm", "DPoPKeyPEM", "NoKIDInKAO", "FulfillableObligationFQNs", "LogLevel"};
    static char *exchange[] = {"SubjectToken", "Audience"}, *cert[] = {"ClientCertPEM", "ClientKeyPEM", "Audience"}, *tls[] = {"ClientCertPEM", "ClientKeyPEM", "RootCAsPEM", "Audience"}, *entry[] = {"Key", "Value"}, *configuration[] = {"Entries"};
    gxc_value *entries = keep(a, calloc(o->platform_configuration_count ? o->platform_configuration_count : 1, sizeof(*entries)));
    for (size_t i = 0; i < o->platform_configuration_count; i++) entries[i] = RECORD(a, entry, str(o->platform_configuration[i].key), str(o->platform_configuration[i].value));
    gxc_value platform = o->platform_configuration ? RECORD(a, configuration, ((gxc_value){.kind = GXC_LIST, .items = entries, .length = o->platform_configuration_count})) : (gxc_value){.kind = GXC_NIL};
    return RECORD(a, names,
        str(o->client_id), str(o->client_secret), strs(a, o->scopes, o->scope_count), str(o->token_endpoint),
        RECORD(a, exchange, str(o->token_exchange_subject_token), strs(a, o->token_exchange_audience, o->token_exchange_audience_count)),
        RECORD(a, cert, str(o->cert_exchange_client_cert_pem), str(o->cert_exchange_client_key_pem), strs(a, o->cert_exchange_audience, o->cert_exchange_audience_count)),
        str(o->token_provider ? "access-token" : NULL), platform,
        boolean(o->validate_connection), boolean(o->insecure_plaintext), boolean(o->insecure_skip_verify),
        RECORD(a, tls, str(o->tls_client_cert_pem), str(o->tls_client_key_pem), str(o->tls_root_cas_pem), strs(a, o->tls_audience, o->tls_audience_count)),
        str(o->unix_socket), integer(o->timeout_millis), str(o->core_endpoint), str(o->policy_endpoint), str(o->authorization_endpoint), str(o->entity_resolution_endpoint), str(o->wellknown_endpoint),
        str(o->session_encryption_rsa_pem), str(o->session_signer_rsa_pem), str(o->dpop_algorithm), str(o->dpop_key_pem), boolean(o->no_kid_in_kao),
        strs(a, o->fulfillable_obligation_fqns, o->fulfillable_obligation_fqn_count), integer(o->log_level));
}
static gxc_value tdf_options_value(arena *a, const otdf_tdf_options *o) {
    static char *names[] = {"Attributes", "AttributeValues", "DisableAutoconfigure", "KasInformation", "WrappingKeyAlgorithm", "SegmentSize", "InputSize", "MimeType", "Metadata", "Assertions", "SystemMetadataAssertion", "TargetMode"};
    static char *kas[] = {"URL", "PublicKey", "KID", "Algorithm", "Default"}, *assertion[] = {"ID", "Type", "Scope", "AppliesToState", "Statement", "SigningKey"}, *statement[] = {"Format", "Schema", "Value"}, *key[] = {"Alg", "Key"};
    static const otdf_tdf_options zero = OTDF_TDF_OPTIONS_INIT;
    if (!o) o = &zero;
    gxc_value *kases = keep(a, calloc(o->kas_information_count ? o->kas_information_count : 1, sizeof(*kases)));
    for (size_t i = 0; i < o->kas_information_count; i++) { const otdf_kas_info *k = &o->kas_information[i]; kases[i] = RECORD(a, kas, str(k->url), str(k->public_key), str(k->kid), str(k->algorithm), boolean(k->is_default)); }
    gxc_value *assertions = keep(a, calloc(o->assertion_count ? o->assertion_count : 1, sizeof(*assertions)));
    for (size_t i = 0; i < o->assertion_count; i++) { const otdf_assertion_config *c = &o->assertions[i]; assertions[i] = RECORD(a, assertion, str(c->id), str(c->type), str(c->scope), str(c->applies_to_state), RECORD(a, statement, str(c->format), str(c->schema), str(c->value)), RECORD(a, key, str(c->signing_key.alg), str(c->signing_key.key))); }
    return RECORD(a, names, strs(a, o->attributes, o->attribute_count), strs(a, o->attribute_values, o->attribute_value_count), boolean(o->disable_autoconfigure),
        o->kas_information ? (gxc_value){.kind = GXC_LIST, .items = kases, .length = o->kas_information_count} : (gxc_value){.kind = GXC_NIL},
        str(o->wrapping_key_algorithm), integer(o->segment_size), integer(o->input_size), str(o->mime_type), str(o->metadata),
        o->assertions ? (gxc_value){.kind = GXC_LIST, .items = assertions, .length = o->assertion_count} : (gxc_value){.kind = GXC_NIL},
        boolean(o->system_metadata_assertion), str(o->target_mode));
}
static gxc_value reader_options_value(arena *a, const otdf_reader_options *o) {
    static char *names[] = {"DisableAssertionVerification", "AssertionVerificationKeys", "KasAllowlist", "IgnoreAllowlist", "MaxManifestSize", "SchemaValidation", "SessionKeyType", "FulfillableObligationFQNs"};
    static char *verification[] = {"DefaultKey", "Keys"}, *key[] = {"Alg", "Key"}, *entry[] = {"AssertionID", "Alg", "Key"};
    static const otdf_reader_options zero = OTDF_READER_OPTIONS_INIT;
    if (!o) o = &zero;
    gxc_value *keys = keep(a, calloc(o->assertion_key_count ? o->assertion_key_count : 1, sizeof(*keys)));
    for (size_t i = 0; i < o->assertion_key_count; i++) { const otdf_assertion_key_entry *k = &o->assertion_keys[i]; keys[i] = RECORD(a, entry, str(k->assertion_id), str(k->alg), str(k->key)); }
    gxc_value verification_keys = RECORD(a, verification, RECORD(a, key, str(o->assertion_default_key.alg), str(o->assertion_default_key.key)),
        o->assertion_keys ? (gxc_value){.kind = GXC_LIST, .items = keys, .length = o->assertion_key_count} : (gxc_value){.kind = GXC_NIL});
    return RECORD(a, names, boolean(o->disable_assertion_verification), verification_keys, strs(a, o->kas_allowlist, o->kas_allowlist_count), boolean(o->ignore_allowlist), integer(o->max_manifest_size), integer(o->schema_validation), str(o->session_key_type), strs(a, o->fulfillable_obligation_fqns, o->fulfillable_obligation_fqn_count));
}

/* Output values. */
static char *dup_text(const gxc_value *v) {
    size_t n = v && v->kind == GXC_BYTES ? v->length : 0;
    char *s = malloc(n + 1);
    if (!s) abort();
    if (n) memcpy(s, v->bytes, n);
    s[n] = 0;
    return s;
}
static otdf_owned_bytes dup_bytes(const gxc_value *v) {
    otdf_owned_bytes b = {0};
    if (v && v->kind == GXC_BYTES) { b.data = malloc(v->length ? v->length : 1); if (!b.data) abort(); memcpy(b.data, v->bytes, v->length); b.length = v->length; }
    else { b.data = malloc(1); if (!b.data) abort(); }
    return b;
}
static char **dup_texts(const gxc_value *v, size_t *count) {
    size_t n = v && v->kind == GXC_LIST ? v->length : 0;
    char **out = calloc(n ? n : 1, sizeof(*out));
    if (!out) abort();
    for (size_t i = 0; i < n; i++) out[i] = dup_text(&v->items[i]);
    *count = n;
    return out;
}
void otdf_bytes_free(otdf_owned_bytes *b) { if (!b) return; free(b->data); memset(b, 0, sizeof(*b)); }
void otdf_string_free(char *s) { free(s); }
void otdf_strings_free(char **s, size_t n) { if (!s) return; for (size_t i = 0; i < n; i++) free(s[i]); free(s); }
void otdf_error_free(otdf_error *e) {
    if (!e) return;
    free(e->kind); free(e->op); free(e->message); free(e->connect_code); free(e->service_message); free(e->kas_url); free(e->assertion_id);
    otdf_strings_free(e->required_obligations, e->required_obligation_count);
    memset(e, 0, sizeof(*e));
}

/* The SDK's Error record crosses as the fields of a kind-1 failure; every
 * other kind is a boundary failure that keeps the runtime's name. */
static int take_error(otdf_error *e, gxc_error *source, int rc) {
    static const char *kinds[] = {"", "source", "source_panic", "host_fault", "source_fatal", "invalid_argument", "canceled", "closed", "poisoned", "instance_mismatch", "reentrant", "unsupported"};
    int kind = source->kind ? source->kind : rc;
    if (e) {
        otdf_error_free(e);
        const gxc_value *fields = &source->fields;
        if (kind == 1 && gxc_field(fields, "Kind")->kind == GXC_BYTES) {
            e->kind = dup_text(gxc_field(fields, "Kind")); e->op = dup_text(gxc_field(fields, "Op")); e->message = dup_text(gxc_field(fields, "Message"));
            e->http_status = gxc_field(fields, "HTTPStatus")->integer; e->connect_code = dup_text(gxc_field(fields, "ConnectCode")); e->service_message = dup_text(gxc_field(fields, "ServiceMessage"));
            e->kas_url = dup_text(gxc_field(fields, "KASURL")); e->required_obligations = dup_texts(gxc_field(fields, "RequiredObligations"), &e->required_obligation_count); e->assertion_id = dup_text(gxc_field(fields, "AssertionID"));
        } else {
            const char *name = kind > 0 && kind < 12 ? kinds[kind] : "host_fault";
            e->kind = strdup(name); e->op = strdup(""); e->connect_code = strdup(""); e->service_message = strdup(""); e->kas_url = strdup(""); e->assertion_id = strdup("");
            e->required_obligations = calloc(1, sizeof(char *));
            size_t n = source->message.kind == GXC_BYTES ? source->message.length : 0;
            e->message = malloc(n + 32);
            if (!e->kind || !e->message || !e->required_obligations) abort();
            snprintf(e->message, n + 32, "goalchemy library: %s%s%.*s", name, n ? ": " : "", (int)n, n ? (const char *)source->message.bytes : "");
        }
    }
    gxc_error_free(source);
    return kind ? kind : 1;
}
static int invoke(const char *name, gxc_value *args, size_t n, const otdf_call *call, otdf_sdk *sdk, gxc_value *result, otdf_error *error) {
    gxc_options o = {0};
    if (call) { o.canceled = call->canceled; o.timeout_nanoseconds = call->timeout_millis > 0 ? call->timeout_millis * 1000000 : 0; }
    if (sdk) { o.provider = sdk->provider; o.provider_state = sdk->state; }
    gxc_value value = {0}; gxc_error failure = {0};
    int rc = goalchemy_invoke(name, args, n, &o, &value, &failure);
    if (rc) return take_error(error, &failure, rc);
    if (result) *result = value; else gxc_value_free(&value);
    return 0;
}
static int invalid(otdf_error *e) { gxc_error f = {.kind = 5}; return take_error(e, &f, 5); }

/* JSON for manifests, policies and keys keyed by the reference's JSON tags. */
typedef struct tag { const char *go, *json; const struct tag *child; } tag;
static const tag policy_binding[] = {{"Alg", "alg", NULL}, {"Hash", "hash", NULL}, {NULL, NULL, NULL}};
static const tag key_access[] = {{"KeyType", "type", NULL}, {"KasURL", "url", NULL}, {"Protocol", "protocol", NULL}, {"WrappedKey", "wrappedKey", NULL}, {"PolicyBinding", "policyBinding", policy_binding}, {"EncryptedMetadata", "encryptedMetadata", NULL}, {"KID", "kid", NULL}, {"SplitID", "sid", NULL}, {"SchemaVersion", "schemaVersion", NULL}, {"EphemeralPublicKey", "ephemeralPublicKey", NULL}, {NULL, NULL, NULL}};
static const tag method[] = {{"Algorithm", "algorithm", NULL}, {"IV", "iv", NULL}, {"IsStreamable", "isStreamable", NULL}, {NULL, NULL, NULL}};
static const tag root_signature[] = {{"Algorithm", "alg", NULL}, {"Signature", "sig", NULL}, {NULL, NULL, NULL}};
static const tag segment[] = {{"Hash", "hash", NULL}, {"Size", "segmentSize", NULL}, {"EncryptedSize", "encryptedSegmentSize", NULL}, {NULL, NULL, NULL}};
static const tag integrity[] = {{"RootSignature", "rootSignature", root_signature}, {"SegmentHashAlgorithm", "segmentHashAlg", NULL}, {"DefaultSegmentSize", "segmentSizeDefault", NULL}, {"DefaultEncryptedSegSize", "encryptedSegmentSizeDefault", NULL}, {"Segments", "segments", segment}, {NULL, NULL, NULL}};
static const tag encryption[] = {{"KeyAccessType", "type", NULL}, {"Policy", "policy", NULL}, {"KeyAccessObjs", "keyAccess", key_access}, {"Method", "method", method}, {"IntegrityInformation", "integrityInformation", integrity}, {NULL, NULL, NULL}};
static const tag payload[] = {{"Type", "type", NULL}, {"URL", "url", NULL}, {"Protocol", "protocol", NULL}, {"MimeType", "mimeType", NULL}, {"IsEncrypted", "isEncrypted", NULL}, {NULL, NULL, NULL}};
static const tag statement[] = {{"Format", "format", NULL}, {"Schema", "schema", NULL}, {"Value", "value", NULL}, {NULL, NULL, NULL}};
static const tag binding[] = {{"Method", "method", NULL}, {"Signature", "signature", NULL}, {NULL, NULL, NULL}};
static const tag assertion[] = {{"ID", "id", NULL}, {"Type", "type", NULL}, {"Scope", "scope", NULL}, {"AppliesToState", "appliesToState", NULL}, {"Statement", "statement", statement}, {"Binding", "binding", binding}, {NULL, NULL, NULL}};
static const tag manifest[] = {{"EncryptionInformation", "encryptionInformation", encryption}, {"Payload", "payload", payload}, {"Assertions", "assertions", assertion}, {"TDFVersion", "schemaVersion", NULL}, {NULL, NULL, NULL}};
static const tag attribute[] = {{"Attribute", "attribute", NULL}, {"DisplayName", "displayName", NULL}, {"IsDefault", "isDefault", NULL}, {"PubKey", "pubKey", NULL}, {"KasURL", "kasURL", NULL}, {NULL, NULL, NULL}};
static const tag body[] = {{"DataAttributes", "dataAttributes", attribute}, {"Dissem", "dissem", NULL}, {NULL, NULL, NULL}};
static const tag policy[] = {{"UUID", "uuid", NULL}, {"Body", "body", body}, {NULL, NULL, NULL}};
static const tag simple_public[] = {{"Algorithm", "algorithm", NULL}, {"Kid", "kid", NULL}, {"Pem", "pem", NULL}, {NULL, NULL, NULL}};
static const tag simple_key[] = {{"KasURI", "kasUri", NULL}, {"PublicKey", "publicKey", simple_public}, {"KasID", "kasId", NULL}, {NULL, NULL, NULL}};
typedef struct { char *data; size_t length, cap; } buffer;
static void put(buffer *b, const char *s, size_t n) {
    if (b->length + n + 1 > b->cap) { b->cap = (b->length + n + 1) * 2; b->data = realloc(b->data, b->cap); if (!b->data) abort(); }
    memcpy(b->data + b->length, s, n);
    b->length += n;
    b->data[b->length] = 0;
}
static void puts_(buffer *b, const char *s) { put(b, s, strlen(s)); }
static void put_string(buffer *b, const uint8_t *s, size_t n) {
    put(b, "\"", 1);
    for (size_t i = 0; i < n; i++) {
        unsigned char c = s[i];
        char escaped[8];
        if (c == '"' || c == '\\') { escaped[0] = '\\'; escaped[1] = (char)c; put(b, escaped, 2); }
        else if (c < 0x20) { snprintf(escaped, sizeof escaped, "\\u%04x", c); put(b, escaped, 6); }
        else put(b, (const char *)&s[i], 1);
    }
    put(b, "\"", 1);
}
static void put_value(buffer *b, const gxc_value *v, const tag *tags) {
    char number[32];
    switch (v->kind) {
    case GXC_BOOL: puts_(b, v->integer ? "true" : "false"); break;
    case GXC_INT: snprintf(number, sizeof number, "%lld", (long long)v->integer); puts_(b, number); break;
    case GXC_BYTES: put_string(b, v->bytes, v->length); break;
    case GXC_LIST:
        put(b, "[", 1);
        for (size_t i = 0; i < v->length; i++) { if (i) put(b, ",", 1); put_value(b, &v->items[i], tags); }
        put(b, "]", 1);
        break;
    case GXC_RECORD:
        put(b, "{", 1);
        for (size_t i = 0; i < v->length; i++) {
            const tag *t = tags;
            while (t && t->go && strcmp(t->go, v->names[i])) t++;
            if (i) put(b, ",", 1);
            put_string(b, (const uint8_t *)(t && t->go ? t->json : v->names[i]), strlen(t && t->go ? t->json : v->names[i]));
            put(b, ":", 1);
            put_value(b, &v->items[i], t && t->go ? t->child : NULL);
        }
        put(b, "}", 1);
        break;
    default: puts_(b, "null");
    }
}
static char *json_of(gxc_value *v, const tag *tags) {
    buffer b = {0};
    put_value(&b, v, tags);
    gxc_value_free(v);
    return b.data;
}
static int json_call(const char *name, gxc_value *args, size_t n, const otdf_call *call, otdf_sdk *sdk, const tag *tags, char **json, otdf_error *error) {
    if (!json) return invalid(error);
    gxc_value result = {0};
    int rc = invoke(name, args, n, call, sdk, &result, error);
    if (!rc) *json = json_of(&result, tags);
    return rc;
}

/* The reader's byte source: a seekable gxc_stream over the copied ciphertext. */
static int64_t stream_read(void *state, uint8_t *buffer_, size_t length, char *message) {
    (void)message;
    otdf_reader *r = state;
    size_t n = r->position >= r->length ? 0 : (length < r->length - r->position ? length : r->length - r->position);
    if (n) memcpy(buffer_, r->data + r->position, n);
    r->position += n;
    return (int64_t)n;
}
static int64_t stream_seek(void *state, int64_t offset, int whence, char *message) {
    otdf_reader *r = state;
    int64_t base = whence == 0 ? 0 : whence == 1 ? (int64_t)r->position : (int64_t)r->length;
    if (base + offset < 0) { snprintf(message, 255, "seek: negative position"); return -1; }
    r->position = (size_t)(base + offset);
    return base + offset;
}

int otdf_sdk_new(const char *endpoint, const otdf_options *options, const otdf_call *call, otdf_sdk **sdk, otdf_error *error) {
    static const otdf_options zero = OTDF_OPTIONS_INIT;
    if (!endpoint || !sdk) return invalid(error);
    if (!options) options = &zero;
    *sdk = NULL;
    otdf_sdk shell = {.provider = options->token_provider, .state = options->token_provider_state};
    arena a = {0};
    gxc_value args[] = {str(endpoint), options_value(&a, options)}, result = {0};
    int rc = invoke("New", args, 2, call, &shell, &result, error);
    release(&a);
    if (rc) return rc;
    otdf_sdk *s = calloc(1, sizeof(*s));
    if (!s) abort();
    *s = shell;
    s->h = result.handle;
    *sdk = s;
    return 0;
}
int otdf_sdk_close(otdf_sdk *sdk, otdf_error *error) {
    if (!sdk) return invalid(error);
    gxc_error failure = {0};
    int rc = gxc_handle_close(sdk->h, &failure);
    if (rc) rc = take_error(error, &failure, rc);
    return rc;
}
void otdf_sdk_free(otdf_sdk *sdk) { if (!sdk) return; gxc_handle_free(sdk->h); free(sdk); }

int otdf_create_tdf(otdf_sdk *sdk, otdf_bytes plaintext, const otdf_tdf_options *options, const otdf_call *call, otdf_owned_bytes *tdf, otdf_tdf_object **object, otdf_error *error) {
    if (!sdk || !tdf) return invalid(error);
    arena a = {0};
    gxc_value args[] = {handle(sdk->h), raw(plaintext), tdf_options_value(&a, options)}, result = {0};
    int rc = invoke("SDK.EncryptBytes", args, 3, call, sdk, &result, error);
    release(&a);
    if (rc) return rc;
    *tdf = dup_bytes(&result.items[0]);
    if (object) {
        otdf_tdf_object *o = calloc(1, sizeof(*o));
        if (!o) abort();
        o->h = result.items[1].handle;
        o->sdk = sdk;
        *object = o;
    } else { gxc_error ignored = {0}; gxc_handle_close(result.items[1].handle, &ignored); gxc_error_free(&ignored); gxc_handle_free(result.items[1].handle); }
    gxc_value_free(&result);
    return 0;
}
int otdf_encrypt_bytes(otdf_sdk *sdk, otdf_bytes plaintext, const otdf_tdf_options *options, const otdf_call *call, otdf_owned_bytes *tdf, otdf_error *error) {
    return otdf_create_tdf(sdk, plaintext, options, call, tdf, NULL, error);
}
int otdf_tdf_object_manifest_json(otdf_tdf_object *o, const otdf_call *call, char **json, otdf_error *error) {
    if (!o) return invalid(error);
    gxc_value args[] = {handle(o->h)};
    return json_call("TDFObject.Manifest", args, 1, call, o->sdk, manifest, json, error);
}
int otdf_tdf_object_size(otdf_tdf_object *o, const otdf_call *call, int64_t *size, otdf_error *error) {
    if (!o || !size) return invalid(error);
    gxc_value args[] = {handle(o->h)}, result = {0};
    int rc = invoke("TDFObject.Size", args, 1, call, o->sdk, &result, error);
    if (!rc) { *size = result.integer; gxc_value_free(&result); }
    return rc;
}
int otdf_tdf_object_close(otdf_tdf_object *o, otdf_error *error) {
    if (!o) return invalid(error);
    gxc_error failure = {0};
    int rc = gxc_handle_close(o->h, &failure);
    if (rc) rc = take_error(error, &failure, rc);
    return rc;
}
void otdf_tdf_object_free(otdf_tdf_object *o) { if (!o) return; gxc_handle_free(o->h); free(o); }

int otdf_load_tdf(otdf_sdk *sdk, otdf_bytes ciphertext, const otdf_reader_options *options, const otdf_call *call, otdf_reader **reader, otdf_error *error) {
    if (!sdk || !reader) return invalid(error);
    *reader = NULL;
    otdf_reader *r = calloc(1, sizeof(*r));
    if (!r) abort();
    r->sdk = sdk;
    r->data = malloc(ciphertext.length ? ciphertext.length : 1);
    if (!r->data) abort();
    memcpy(r->data, ciphertext.data, ciphertext.length);
    r->length = ciphertext.length;
    r->stream = (gxc_stream){.state = r, .read = stream_read, .seek = stream_seek};
    arena a = {0};
    gxc_value args[] = {handle(sdk->h), {.kind = GXC_STREAM, .stream = &r->stream}, reader_options_value(&a, options)}, result = {0};
    int rc = invoke("SDK.LoadTDF", args, 3, call, sdk, &result, error);
    release(&a);
    if (rc) { free(r->data); free(r); return rc; }
    r->h = result.handle;
    *reader = r;
    return 0;
}
int otdf_reader_init(otdf_reader *r, const otdf_call *call, otdf_error *error) {
    if (!r) return invalid(error);
    gxc_value args[] = {handle(r->h)};
    return invoke("Reader.Init", args, 1, call, r->sdk, NULL, error);
}
int otdf_reader_manifest_json(otdf_reader *r, const otdf_call *call, char **json, otdf_error *error) {
    if (!r) return invalid(error);
    gxc_value args[] = {handle(r->h)};
    return json_call("Reader.Manifest", args, 1, call, r->sdk, manifest, json, error);
}
int otdf_reader_policy_json(otdf_reader *r, const otdf_call *call, char **json, otdf_error *error) {
    if (!r) return invalid(error);
    gxc_value args[] = {handle(r->h)};
    return json_call("Reader.Policy", args, 1, call, r->sdk, policy, json, error);
}
int otdf_reader_data_attributes(otdf_reader *r, const otdf_call *call, char ***attributes, size_t *count, otdf_error *error) {
    if (!r || !attributes || !count) return invalid(error);
    gxc_value args[] = {handle(r->h)}, result = {0};
    int rc = invoke("Reader.DataAttributes", args, 1, call, r->sdk, &result, error);
    if (!rc) { *attributes = dup_texts(&result, count); gxc_value_free(&result); }
    return rc;
}
int otdf_reader_unencrypted_metadata(otdf_reader *r, const otdf_call *call, otdf_owned_bytes *metadata, otdf_error *error) {
    if (!r || !metadata) return invalid(error);
    gxc_value args[] = {handle(r->h)}, result = {0};
    int rc = invoke("Reader.UnencryptedMetadata", args, 1, call, r->sdk, &result, error);
    if (!rc) { *metadata = dup_bytes(&result); gxc_value_free(&result); }
    return rc;
}
int otdf_reader_close(otdf_reader *r, otdf_error *error) {
    if (!r) return invalid(error);
    gxc_error failure = {0};
    int rc = gxc_handle_close(r->h, &failure);
    if (rc) rc = take_error(error, &failure, rc);
    return rc;
}
void otdf_reader_free(otdf_reader *r) { if (!r) return; gxc_handle_free(r->h); free(r->data); free(r); }
int otdf_decrypt_bytes(otdf_sdk *sdk, otdf_bytes ciphertext, const otdf_reader_options *options, const otdf_call *call, otdf_owned_bytes *plaintext, otdf_error *error) {
    if (!sdk || !plaintext) return invalid(error);
    arena a = {0};
    gxc_value args[] = {handle(sdk->h), raw(ciphertext), reader_options_value(&a, options)}, result = {0};
    int rc = invoke("SDK.DecryptBytes", args, 3, call, sdk, &result, error);
    release(&a);
    if (!rc) { *plaintext = dup_bytes(&result); gxc_value_free(&result); }
    return rc;
}

int otdf_is_healthy(otdf_sdk *sdk, const otdf_call *call, bool *healthy, otdf_error *error) {
    if (!sdk || !healthy) return invalid(error);
    gxc_value args[] = {handle(sdk->h)}, result = {0};
    int rc = invoke("SDK.IsHealthy", args, 1, call, sdk, &result, error);
    if (!rc) { *healthy = result.integer != 0; gxc_value_free(&result); }
    return rc;
}
int otdf_platform_configuration_json(otdf_sdk *sdk, const otdf_call *call, char **json, otdf_error *error) {
    if (!sdk || !json) return invalid(error);
    gxc_value args[] = {handle(sdk->h)}, result = {0};
    int rc = invoke("SDK.PlatformConfiguration", args, 1, call, sdk, &result, error);
    if (rc) return rc;
    const gxc_value *entries = gxc_field(&result, "Entries");
    buffer b = {0};
    put(&b, "{", 1);
    for (size_t i = 0; entries->kind == GXC_LIST && i < entries->length; i++) {
        const gxc_value *key = gxc_field(&entries->items[i], "Key"), *value = gxc_field(&entries->items[i], "Value");
        if (i) put(&b, ",", 1);
        put_string(&b, key->bytes, key->length);
        put(&b, ":", 1);
        put_string(&b, value->bytes, value->length);
    }
    put(&b, "}", 1);
    gxc_value_free(&result);
    *json = b.data;
    return 0;
}
int otdf_get_base_key_json(otdf_sdk *sdk, const otdf_call *call, char **json, otdf_error *error) {
    if (!sdk) return invalid(error);
    gxc_value args[] = {handle(sdk->h)};
    return json_call("SDK.GetBaseKey", args, 1, call, sdk, simple_key, json, error);
}
int otdf_store_kas_keys(otdf_sdk *sdk, const char *url, const otdf_kas_public_key *keys, size_t count, const otdf_call *call, otdf_error *error) {
    static char *set[] = {"Keys"}, *key[] = {"Pem", "Kid", "Alg"};
    if (!sdk || !url || (count && !keys)) return invalid(error);
    arena a = {0};
    gxc_value *items = keep(&a, calloc(count ? count : 1, sizeof(*items)));
    for (size_t i = 0; i < count; i++) items[i] = RECORD(&a, key, str(keys[i].pem), str(keys[i].kid), str(keys[i].alg));
    gxc_value args[] = {handle(sdk->h), str(url), RECORD(&a, set, ((gxc_value){.kind = GXC_LIST, .items = items, .length = count}))};
    int rc = invoke("SDK.StoreKASKeys", args, 3, call, sdk, NULL, error);
    release(&a);
    return rc;
}

int otdf_sdk_call(otdf_sdk *sdk, const char *service, const char *method, const char *request_json, const otdf_call *call, char **response_json, otdf_error *error) {
    if (!sdk || !service || !method || !response_json) return invalid(error);
    gxc_value args[] = {handle(sdk->h), str(service), str(method), str(request_json)}, result = {0};
    int rc = invoke("SDK.Call", args, 4, call, sdk, &result, error);
    if (!rc) { *response_json = dup_text(&result); gxc_value_free(&result); }
    return rc;
}

static int fqn_check(otdf_sdk *sdk, const char *name, const char *fqn, const otdf_call *call, bool *exists, otdf_error *error) {
    if (!sdk || !fqn || !exists) return invalid(error);
    gxc_value args[] = {handle(sdk->h), str(fqn)}, result = {0};
    int rc = invoke(name, args, 2, call, sdk, &result, error);
    if (!rc) { *exists = result.integer != 0; gxc_value_free(&result); }
    return rc;
}
int otdf_attribute_exists(otdf_sdk *sdk, const char *attribute_fqn, const otdf_call *call, bool *exists, otdf_error *error) {
    return fqn_check(sdk, "SDK.AttributeExists", attribute_fqn, call, exists, error);
}
int otdf_attribute_value_exists(otdf_sdk *sdk, const char *value_fqn, const otdf_call *call, bool *exists, otdf_error *error) {
    return fqn_check(sdk, "SDK.AttributeValueExists", value_fqn, call, exists, error);
}
int otdf_validate_attributes(otdf_sdk *sdk, const char *const *fqns, size_t count, const otdf_call *call, otdf_error *error) {
    if (!sdk || (count && !fqns)) return invalid(error);
    arena a = {0};
    static const char *none[1];
    gxc_value args[] = {handle(sdk->h), strs(&a, fqns ? fqns : none, count)};
    int rc = invoke("SDK.ValidateAttributes", args, 2, call, sdk, NULL, error);
    release(&a);
    return rc;
}
int otdf_list_attributes(otdf_sdk *sdk, const char *namespace_name, const otdf_call *call, char **attributes_json, otdf_error *error) {
    if (!sdk || !attributes_json) return invalid(error);
    arena a = {0};
    const char *filter[] = {namespace_name};
    gxc_value args[] = {handle(sdk->h), strs(&a, filter, namespace_name ? 1 : 0)}, result = {0};
    int rc = invoke("SDK.ListAttributesJSON", args, 2, call, sdk, &result, error);
    release(&a);
    if (rc) return rc;
    buffer b = {0};
    put(&b, "[", 1);
    for (size_t i = 0; result.kind == GXC_LIST && i < result.length; i++) {
        if (i) put(&b, ",", 1);
        put(&b, (const char *)result.items[i].bytes, result.items[i].length);
    }
    put(&b, "]", 1);
    gxc_value_free(&result);
    *attributes_json = b.data;
    return 0;
}
int otdf_get_entity_attributes(otdf_sdk *sdk, const char *entity_json, const otdf_call *call, char ***fqns, size_t *count, otdf_error *error) {
    if (!sdk || !fqns || !count) return invalid(error);
    gxc_value args[] = {handle(sdk->h), str(entity_json)}, result = {0};
    int rc = invoke("SDK.GetEntityAttributesJSON", args, 2, call, sdk, &result, error);
    if (!rc) { *fqns = dup_texts(&result, count); gxc_value_free(&result); }
    return rc;
}
void otdf_access_token_free(otdf_access_token *t) {
    if (!t) return;
    free(t->value); free(t->scheme); free(t->confirmation_jkt);
    memset(t, 0, sizeof(*t));
}
int otdf_auth_access_token(otdf_sdk *sdk, const otdf_call *call, otdf_access_token *token, otdf_error *error) {
    if (!sdk || !token) return invalid(error);
    gxc_value args[] = {handle(sdk->h)}, client = {0}, result = {0};
    int rc = invoke("SDK.Auth", args, 1, call, sdk, &client, error);
    if (rc) return rc;
    gxc_handle *h = client.handle;
    gxc_value auth[] = {handle(h)};
    rc = invoke("AuthClient.AccessToken", auth, 1, call, sdk, &result, error);
    gxc_error ignored = {0};
    gxc_handle_close(h, &ignored);
    gxc_error_free(&ignored);
    gxc_handle_free(h);
    if (rc) return rc;
    otdf_access_token_free(token);
    token->value = dup_text(gxc_field(&result, "Value"));
    token->scheme = dup_text(gxc_field(&result, "Scheme"));
    token->expires_at = gxc_field(&result, "ExpiresAt")->integer;
    token->confirmation_jkt = dup_text(gxc_field(&result, "ConfirmationJKT"));
    gxc_value_free(&result);
    return 0;
}
int otdf_sdk_conn(otdf_sdk *sdk, const otdf_call *call, char **endpoint, otdf_error *error) {
    if (!sdk || !endpoint) return invalid(error);
    gxc_value args[] = {handle(sdk->h)}, result = {0};
    int rc = invoke("SDK.Conn", args, 1, call, sdk, &result, error);
    if (!rc) { *endpoint = dup_text(gxc_field(&result, "Endpoint")); gxc_value_free(&result); }
    return rc;
}

/* Free functions: a bytes stream for the reader parameters, text results. */
typedef struct { otdf_bytes data; size_t position; } cursor;
static int64_t cursor_read(void *state, uint8_t *buffer_, size_t length, char *message) {
    (void)message;
    cursor *c = state;
    size_t n = c->position >= c->data.length ? 0 : (length < c->data.length - c->position ? length : c->data.length - c->position);
    if (n) memcpy(buffer_, c->data.data + c->position, n);
    c->position += n;
    return (int64_t)n;
}
static int64_t cursor_seek(void *state, int64_t offset, int whence, char *message) {
    cursor *c = state;
    int64_t base = whence == 0 ? 0 : whence == 1 ? (int64_t)c->position : (int64_t)c->data.length;
    if (base + offset < 0) { snprintf(message, 255, "seek: negative position"); return -1; }
    c->position = (size_t)(base + offset);
    return base + offset;
}
static int bool_call(const char *name, gxc_value *args, size_t n, const otdf_call *call, bool *out, otdf_error *error) {
    if (!out) return invalid(error);
    gxc_value result = {0};
    int rc = invoke(name, args, n, call, NULL, &result, error);
    if (!rc) { *out = result.integer != 0; gxc_value_free(&result); }
    return rc;
}
static int text_call(const char *name, gxc_value *args, size_t n, const otdf_call *call, char **out, otdf_error *error) {
    if (!out) return invalid(error);
    gxc_value result = {0};
    int rc = invoke(name, args, n, call, NULL, &result, error);
    if (!rc) { *out = dup_text(&result); gxc_value_free(&result); }
    return rc;
}
int otdf_is_valid_tdf(otdf_bytes data, const otdf_call *call, bool *valid, otdf_error *error) {
    cursor c = {data, 0};
    gxc_stream s = {.state = &c, .read = cursor_read, .seek = cursor_seek};
    gxc_value args[] = {{.kind = GXC_STREAM, .stream = &s}};
    return bool_call("IsValidTdf", args, 1, call, valid, error);
}
int otdf_get_tdf_type(otdf_bytes data, const otdf_call *call, char **type, otdf_error *error) {
    cursor c = {data, 0};
    gxc_stream s = {.state = &c, .read = cursor_read, .seek = cursor_seek};
    gxc_value args[] = {{.kind = GXC_STREAM, .stream = &s}};
    return text_call("GetTdfType", args, 1, call, type, error);
}
int otdf_is_platform_endpoint_malformed(const char *endpoint, bool *malformed, otdf_error *error) {
    if (!endpoint) return invalid(error);
    gxc_value args[] = {str(endpoint)};
    return bool_call("IsPlatformEndpointMalformed", args, 1, NULL, malformed, error);
}
static int fqn_call(const char *name, const char *fqn, char **url, char **key, otdf_error *error) {
    if (!fqn || !url || !key) return invalid(error);
    gxc_value args[] = {str(fqn)}, result = {0};
    int rc = invoke(name, args, 1, NULL, NULL, &result, error);
    if (!rc) { *url = dup_text(gxc_field(&result, "URL")); *key = dup_text(gxc_field(&result, "Key")); gxc_value_free(&result); }
    return rc;
}
int otdf_new_attribute_name_fqn(const char *fqn, char **url, char **key, otdf_error *error) { return fqn_call("NewAttributeNameFQN", fqn, url, key, error); }
int otdf_new_attribute_value_fqn(const char *fqn, char **url, char **key, otdf_error *error) { return fqn_call("NewAttributeValueFQN", fqn, url, key, error); }
int otdf_key_type_to_policy_algorithm(const char *key_type, char **algorithm, otdf_error *error) {
    if (!key_type) return invalid(error);
    gxc_value args[] = {str(key_type)};
    return text_call("KeyTypeToPolicyAlgorithm", args, 1, NULL, algorithm, error);
}
int otdf_policy_algorithm_to_key_type(const char *algorithm, char **key_type, otdf_error *error) {
    if (!algorithm) return invalid(error);
    gxc_value args[] = {str(algorithm)};
    return text_call("PolicyAlgorithmToKeyType", args, 1, NULL, key_type, error);
}
int otdf_version(char **version, otdf_error *error) { return text_call("Version", NULL, 0, NULL, version, error); }
int otdf_tdf_spec_version(char **version, otdf_error *error) { return text_call("TDFSpecVersion", NULL, 0, NULL, version, error); }
int otdf_supported_features(char ***features, size_t *count, otdf_error *error) {
    if (!features || !count) return invalid(error);
    gxc_value result = {0};
    int rc = invoke("SupportedFeatures", NULL, 0, NULL, NULL, &result, error);
    if (!rc) { *features = dup_texts(&result, count); gxc_value_free(&result); }
    return rc;
}
