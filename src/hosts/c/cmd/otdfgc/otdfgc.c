/* otdfgc is the native C CLI over the generated library. It implements the
 * xtest CLI contract of docs/full-sdk-plan.md exactly as the Go otdfgc does:
 * one SDK per invocation, created with --host and the client credentials;
 * every flag this build cannot honour exits 2 as unsupported. */
#define _POSIX_C_SOURCE 200809L
#include "opentdf.h"
#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <sys/stat.h>

enum { EXIT_OK = 0, EXIT_FAILED = 1, EXIT_USAGE = 2 };
#define TARGET "c"
#define TIMEOUT_MILLIS 120000
static const char *usage =
    "Usage:\n"
    "  otdfgc encrypt <in> --out <file> --host <url> --with-client-creds <json>\n"
    "      [--attr <fqn,...>] [--mime-type <t>] [--wrapping-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]\n"
    "      [--with-assertions <file|json>] [--target-mode 4.3.0] [--policy-mode encrypted] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]\n"
    "  otdfgc decrypt <in> --out <file> --host <url> --with-client-creds <json>\n"
    "      [--no-verify-assertions] [--with-assertion-verification-keys <file|json>] [--session-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]\n"
    "      [--kas-allowlist <url,...|*>] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]\n"
    "  otdfgc inspect <in>\n"
    "  otdfgc supports <feature>\n"
    "  otdfgc --version [--json]\n";

/* The otdfctl flag subset of the CLI contract; boolean flags take no value. */
typedef struct { const char *name; bool boolean; } flag_spec;
static const flag_spec common_flags[] = {{"out", false}, {"host", false}, {"with-client-creds", false}, {"with-client-creds-file", false}, {"dpop", false}, {"dpop-key", false}, {"tls-no-verify", true}, {"log-level", false}, {NULL, false}};
static const flag_spec encrypt_flags[] = {{"attr", false}, {"mime-type", false}, {"with-assertions", false}, {"wrapping-key-algorithm", false}, {"target-mode", false}, {"policy-mode", false}, {"ecdsa-binding", true}, {NULL, false}};
static const flag_spec decrypt_flags[] = {{"with-assertion-verification-keys", false}, {"no-verify-assertions", true}, {"session-key-algorithm", false}, {"kas-allowlist", false}, {NULL, false}};
/* Rules naming the flags this build cannot honour, optionally only for values other than the allowed ones. */
typedef struct { const char *name; const char *const *allowed; } rule;
static const char *const dpop_algorithms[] = {"ES256", "ES384", "ES512", "RS256", "RS384", "RS512", NULL};
static const char *const target_modes[] = {"4.3.0", NULL}, *const policy_modes[] = {"encrypted", NULL}, *const key_types[] = {"rsa:2048", "ec:secp256r1", "rsa:4096", "ec:secp384r1", "ec:secp521r1", NULL};
static const rule encrypt_unsupported[] = {{"ecdsa-binding", NULL}, {"target-mode", target_modes}, {"policy-mode", policy_modes}, {"wrapping-key-algorithm", key_types}, {"dpop", dpop_algorithms}, {"with-client-creds-file", NULL}, {NULL, NULL}};
static const rule decrypt_unsupported[] = {{"session-key-algorithm", key_types}, {"dpop", dpop_algorithms}, {"with-client-creds-file", NULL}, {NULL, NULL}};

typedef struct { char *names[64]; char *values[64]; size_t count; char *positional[64]; size_t positional_count; } parsed;
static const char *value_of(const parsed *p, const char *name) { for (size_t i = 0; i < p->count; i++) if (!strcmp(p->names[i], name)) return p->values[i] ? p->values[i] : ""; return ""; }
static bool flag_of(const parsed *p, const char *name) { for (size_t i = 0; i < p->count; i++) if (!strcmp(p->names[i], name)) return true; return false; }
static const flag_spec *spec_of(const char *name, const flag_spec *extra) {
    for (const flag_spec *s = common_flags; s->name; s++) if (!strcmp(s->name, name)) return s;
    for (const flag_spec *s = extra; s->name; s++) if (!strcmp(s->name, name)) return s;
    return NULL;
}
static int parse(int argc, char **argv, const flag_spec *extra, parsed *p, char *error, size_t n) {
    memset(p, 0, sizeof(*p));
    for (int i = 0; i < argc; i++) {
        char *a = argv[i];
        if (a[0] != '-' || !strcmp(a, "-")) { if (p->positional_count < 64) p->positional[p->positional_count++] = a; continue; }
        char *name = a; while (*name == '-') name++;
        char *value = NULL; bool inline_value = false;
        char *eq = strchr(name, '=');
        if (eq) { *eq = 0; value = eq + 1; inline_value = true; }
        if (!strcmp(name, "o")) name = "out";
        const flag_spec *s = spec_of(name, extra);
        if (!s) { snprintf(error, n, "unknown flag: %s%s", inline_value ? "--" : "", inline_value ? name : a); return -1; }
        if (p->count >= 64) { snprintf(error, n, "too many flags"); return -1; }
        if (s->boolean) {
            if (inline_value && strcmp(value, "true") && strcmp(value, "false")) { snprintf(error, n, "flag --%s takes no value", name); return -1; }
            if (!inline_value || !strcmp(value, "true")) { p->names[p->count] = name; p->values[p->count] = NULL; p->count++; }
            continue;
        }
        if (!inline_value) {
            if (i + 1 >= argc) { snprintf(error, n, "flag --%s needs a value", name); return -1; }
            value = argv[++i];
        }
        p->names[p->count] = name; p->values[p->count] = value; p->count++;
    }
    return 0;
}
static const char *unsupported(const parsed *p, const rule *rules, char *out, size_t n) {
    for (const rule *r = rules; r->name; r++) {
        if (!flag_of(p, r->name)) continue;
        if (!r->allowed) { snprintf(out, n, "--%s", r->name); return out; }
        bool ok = false;
        for (const char *const *a = r->allowed; *a; a++) if (!strcmp(*a, value_of(p, r->name))) ok = true;
        if (!ok) { snprintf(out, n, "--%s %s", r->name, value_of(p, r->name)); return out; }
    }
    return NULL;
}
/* Splits a comma list into owned, trimmed, non-empty items. */
static size_t split_list(const char *s, char ***out) {
    size_t count = 0, cap = 0;
    *out = NULL;
    for (const char *start = s; ; ) {
        const char *end = strchr(start, ',');
        size_t len = end ? (size_t)(end - start) : strlen(start);
        while (len && isspace((unsigned char)*start)) { start++; len--; }
        while (len && isspace((unsigned char)start[len - 1])) len--;
        if (len) {
            if (count == cap) { cap = cap ? cap * 2 : 4; *out = realloc(*out, cap * sizeof(**out)); if (!*out) abort(); }
            (*out)[count] = strndup(start, len);
            count++;
        }
        if (!end) break;
        start = end + 1;
    }
    return count;
}

static int read_file(const char *path, uint8_t **data, size_t *n) {
    FILE *f = fopen(path, "rb");
    if (!f) return -1;
    size_t cap = 65536, len = 0;
    uint8_t *buf = malloc(cap);
    if (!buf) abort();
    for (;;) {
        if (len == cap) { cap *= 2; buf = realloc(buf, cap); if (!buf) abort(); }
        size_t k = fread(buf + len, 1, cap - len, f);
        len += k;
        if (k == 0) break;
    }
    bool failed = ferror(f);
    fclose(f);
    if (failed) { free(buf); return -1; }
    *data = buf; *n = len;
    return 0;
}

/* describe renders an SDK failure: the reference SDK's wording plus otdfctl's "required obligations" suffix. */
static void describe(const otdf_error *e, FILE *out) {
    fputs(e->message ? e->message : "", out);
    if (e->required_obligation_count) {
        fputs("\nrequired obligations: [", out);
        for (size_t i = 0; i < e->required_obligation_count; i++) fprintf(out, "%s%s", i ? " " : "", e->required_obligations[i]);
        fputs("]", out);
    }
    fputc('\n', out);
}
static void describe_new(const otdf_error *e, const char *host, FILE *out) {
    const char *kind = e->kind ? e->kind : "", *message = e->message ? e->message : "";
    if (!strcmp(kind, "auth")) fprintf(out, "Failed to authenticate with flag-provided client credentials. (%s)\n", message);
    else if (strstr(message, "platform unreachable or not responding") || !strcmp(kind, "transport") || !strcmp(kind, "deadline_exceeded"))
        fprintf(out, "Failed to connect to the platform. Is the platform accepting connections at '%s'? (%s)\n", host, message);
    else fprintf(out, "%s\n", message);
}

/* A bounded JSON string field reader for the credentials: {"clientId":"...","clientSecret":"..."}. */
static char *json_field(const char *json, const char *name) {
    char key[128];
    snprintf(key, sizeof key, "\"%s\"", name);
    const char *p = strstr(json, key);
    if (!p) return NULL;
    p += strlen(key);
    while (*p && isspace((unsigned char)*p)) p++;
    if (*p++ != ':') return NULL;
    while (*p && isspace((unsigned char)*p)) p++;
    if (*p++ != '"') return NULL;
    char *out = malloc(strlen(p) + 1), *w = out;
    if (!out) abort();
    for (; *p && *p != '"'; p++) {
        if (*p == '\\' && p[1]) { p++; *w++ = *p == 'n' ? '\n' : *p == 't' ? '\t' : *p; }
        else *w++ = *p;
    }
    *w = 0;
    if (*p != '"') { free(out); return NULL; }
    return out;
}

/* A small JSON tree for --with-assertions and --with-assertion-verification-keys,
 * which take a file path or JSON text as otdfctl does: an object statement
 * value is kept as compact JSON with sorted keys; HS256 keys are the string's
 * bytes, RS256 keys PEM. */
typedef enum { JV_NULL, JV_BOOL, JV_NUMBER, JV_STRING, JV_ARRAY, JV_OBJECT } jv_kind;
typedef struct jv { jv_kind kind; char *text; char **keys; struct jv **items; size_t count; } jv;
typedef struct { const char *s; const char *error; } jv_parser;
static void jv_free(jv *v) {
    if (!v) return;
    for (size_t i = 0; i < v->count; i++) { jv_free(v->items[i]); if (v->keys) free(v->keys[i]); }
    free(v->items); free(v->keys); free(v->text); free(v);
}
static void jv_ws(jv_parser *p) { while (isspace((unsigned char)*p->s)) p->s++; }
static jv *jv_node(jv_kind kind) { jv *v = calloc(1, sizeof *v); if (!v) abort(); v->kind = kind; return v; }
static void jv_append(char **buf, size_t *n, size_t *cap, const char *data, size_t len) {
    if (*n + len + 1 > *cap) { *cap = (*n + len + 1) * 2; *buf = realloc(*buf, *cap); if (!*buf) abort(); }
    memcpy(*buf + *n, data, len); *n += len; (*buf)[*n] = 0;
}
static void jv_utf8(char **buf, size_t *n, size_t *cap, unsigned cp) {
    char out[4]; size_t len;
    if (cp < 0x80) { out[0] = (char)cp; len = 1; }
    else if (cp < 0x800) { out[0] = (char)(0xC0 | cp >> 6); out[1] = (char)(0x80 | (cp & 0x3F)); len = 2; }
    else if (cp < 0x10000) { out[0] = (char)(0xE0 | cp >> 12); out[1] = (char)(0x80 | ((cp >> 6) & 0x3F)); out[2] = (char)(0x80 | (cp & 0x3F)); len = 3; }
    else { out[0] = (char)(0xF0 | cp >> 18); out[1] = (char)(0x80 | ((cp >> 12) & 0x3F)); out[2] = (char)(0x80 | ((cp >> 6) & 0x3F)); out[3] = (char)(0x80 | (cp & 0x3F)); len = 4; }
    jv_append(buf, n, cap, out, len);
}
static unsigned jv_hex4(jv_parser *p) {
    unsigned cp = 0;
    for (int i = 0; i < 4; i++) {
        char c = *p->s++;
        if (!isxdigit((unsigned char)c)) { p->error = "bad \\u escape"; return 0; }
        cp = cp * 16 + (unsigned)(isdigit((unsigned char)c) ? c - '0' : tolower((unsigned char)c) - 'a' + 10);
    }
    return cp;
}
static char *jv_string(jv_parser *p) {
    if (*p->s != '"') { p->error = "expected string"; return NULL; }
    p->s++;
    char *buf = NULL; size_t n = 0, cap = 0;
    jv_append(&buf, &n, &cap, "", 0);
    while (*p->s && *p->s != '"' && !p->error) {
        if (*p->s == '\\') {
            p->s++;
            char c = *p->s++;
            switch (c) {
                case '"': case '\\': case '/': jv_append(&buf, &n, &cap, &c, 1); break;
                case 'b': jv_append(&buf, &n, &cap, "\b", 1); break;
                case 'f': jv_append(&buf, &n, &cap, "\f", 1); break;
                case 'n': jv_append(&buf, &n, &cap, "\n", 1); break;
                case 'r': jv_append(&buf, &n, &cap, "\r", 1); break;
                case 't': jv_append(&buf, &n, &cap, "\t", 1); break;
                case 'u': {
                    unsigned cp = jv_hex4(p);
                    if (!p->error && cp >= 0xD800 && cp < 0xDC00 && p->s[0] == '\\' && p->s[1] == 'u') { p->s += 2; unsigned low = jv_hex4(p); cp = 0x10000 + ((cp - 0xD800) << 10) + (low - 0xDC00); }
                    if (!p->error) jv_utf8(&buf, &n, &cap, cp);
                    break;
                }
                default: p->error = "bad escape";
            }
        } else jv_append(&buf, &n, &cap, p->s++, 1);
    }
    if (p->error || *p->s != '"') { free(buf); if (!p->error) p->error = "unterminated string"; return NULL; }
    p->s++;
    return buf;
}
static jv *jv_value(jv_parser *p, int depth) {
    jv_ws(p);
    if (depth > 64) { p->error = "too deep"; return NULL; }
    if (*p->s == '{' || *p->s == '[') {
        bool object = *p->s == '{';
        jv *v = jv_node(object ? JV_OBJECT : JV_ARRAY);
        p->s++; jv_ws(p);
        if (*p->s == (object ? '}' : ']')) { p->s++; return v; }
        for (;;) {
            char *key = NULL;
            if (object) { jv_ws(p); key = jv_string(p); if (!key) { jv_free(v); return NULL; } jv_ws(p); if (*p->s++ != ':') { free(key); jv_free(v); p->error = "expected ':'"; return NULL; } }
            jv *item = jv_value(p, depth + 1);
            if (!item) { free(key); jv_free(v); return NULL; }
            v->items = realloc(v->items, (v->count + 1) * sizeof *v->items); if (!v->items) abort();
            if (object) { v->keys = realloc(v->keys, (v->count + 1) * sizeof *v->keys); if (!v->keys) abort(); v->keys[v->count] = key; }
            v->items[v->count++] = item;
            jv_ws(p);
            if (*p->s == ',') { p->s++; continue; }
            if (*p->s == (object ? '}' : ']')) { p->s++; return v; }
            p->error = object ? "expected ',' or '}'" : "expected ',' or ']'"; jv_free(v); return NULL;
        }
    }
    if (*p->s == '"') { char *text = jv_string(p); if (!text) return NULL; jv *v = jv_node(JV_STRING); v->text = text; return v; }
    if (!strncmp(p->s, "true", 4) || !strncmp(p->s, "false", 5)) { jv *v = jv_node(JV_BOOL); v->text = strdup(*p->s == 't' ? "true" : "false"); p->s += strlen(v->text); return v; }
    if (!strncmp(p->s, "null", 4)) { p->s += 4; return jv_node(JV_NULL); }
    const char *start = p->s;
    if (*p->s == '-') p->s++;
    while (*p->s && (isdigit((unsigned char)*p->s) || strchr(".eE+-", *p->s))) p->s++;
    if (p->s == start) { p->error = "unexpected character"; return NULL; }
    jv *v = jv_node(JV_NUMBER); v->text = strndup(start, (size_t)(p->s - start)); return v;
}
static jv *jv_parse(const char *text, const char **error) {
    jv_parser p = {text, NULL};
    jv *v = jv_value(&p, 0);
    if (v) { jv_ws(&p); if (*p.s) { p.error = "trailing data"; jv_free(v); v = NULL; } }
    *error = p.error;
    return v;
}
static jv *jv_field(const jv *v, const char *name) {
    if (!v || v->kind != JV_OBJECT) return NULL;
    for (size_t i = 0; i < v->count; i++) if (!strcmp(v->keys[i], name)) return v->items[i];
    return NULL;
}
static const char *jv_text(const jv *v, const char *name) { jv *f = jv_field(v, name); return f && f->kind == JV_STRING ? f->text : ""; }
static void jv_write_string(char **buf, size_t *n, size_t *cap, const char *s) {
    jv_append(buf, n, cap, "\"", 1);
    for (; *s; s++) {
        unsigned char c = (unsigned char)*s;
        if (c == '"' || c == '\\') { char esc[2] = {'\\', (char)c}; jv_append(buf, n, cap, esc, 2); }
        else if (c < 0x20) { char esc[8]; snprintf(esc, sizeof esc, "\\u%04x", c); jv_append(buf, n, cap, esc, 6); }
        else jv_append(buf, n, cap, s, 1);
    }
    jv_append(buf, n, cap, "\"", 1);
}
static int jv_key_compare(const void *a, const void *b) { return strcmp(*(char *const *)a, *(char *const *)b); }
/* Compact JSON with object keys sorted, as Go's json.Marshal writes a map. */
static void jv_write(const jv *v, char **buf, size_t *n, size_t *cap) {
    switch (v->kind) {
        case JV_NULL: jv_append(buf, n, cap, "null", 4); break;
        case JV_BOOL: case JV_NUMBER: jv_append(buf, n, cap, v->text, strlen(v->text)); break;
        case JV_STRING: jv_write_string(buf, n, cap, v->text); break;
        case JV_ARRAY:
            jv_append(buf, n, cap, "[", 1);
            for (size_t i = 0; i < v->count; i++) { if (i) jv_append(buf, n, cap, ",", 1); jv_write(v->items[i], buf, n, cap); }
            jv_append(buf, n, cap, "]", 1);
            break;
        case JV_OBJECT: {
            char **order = malloc((v->count ? v->count : 1) * sizeof *order); if (!order) abort();
            for (size_t i = 0; i < v->count; i++) order[i] = v->keys[i];
            qsort(order, v->count, sizeof *order, jv_key_compare);
            jv_append(buf, n, cap, "{", 1);
            for (size_t i = 0; i < v->count; i++) {
                if (i) jv_append(buf, n, cap, ",", 1);
                jv_write_string(buf, n, cap, order[i]); jv_append(buf, n, cap, ":", 1);
                jv_write(jv_field(v, order[i]), buf, n, cap);
            }
            jv_append(buf, n, cap, "}", 1);
            free(order);
        }
    }
}
static jv *file_or_json(const char *arg, char *error, size_t error_size) {
    const char *why = NULL;
    jv *v = jv_parse(arg, &why);
    if (v) return v;
    uint8_t *data; size_t n;
    if (read_file(arg, &data, &n)) { snprintf(error, error_size, "%s is neither JSON nor a readable file: %s", arg, strerror(errno)); return NULL; }
    char *text = malloc(n + 1); if (!text) abort();
    memcpy(text, data, n); text[n] = 0; free(data);
    v = jv_parse(text, &why);
    free(text);
    if (!v) snprintf(error, error_size, "%s", why ? why : "invalid JSON");
    return v;
}
typedef struct { otdf_assertion_config *configs; size_t count; char **owned; size_t owned_count; } assertion_list;
static void assertion_list_free(assertion_list *l) { for (size_t i = 0; i < l->owned_count; i++) free(l->owned[i]); free(l->owned); free(l->configs); }
static const char *own(assertion_list *l, const char *text) {
    char *copy = strdup(text ? text : ""); if (!copy) abort();
    l->owned = realloc(l->owned, (l->owned_count + 1) * sizeof *l->owned); if (!l->owned) abort();
    l->owned[l->owned_count++] = copy;
    return copy;
}
static int parse_assertion_key(assertion_list *l, const jv *k, const char *label, otdf_assertion_key *out, char *error, size_t error_size) {
    const char *alg = jv_text(k, "alg");
    if (strcmp(alg, "HS256") && strcmp(alg, "RS256")) { snprintf(error, error_size, "%.120s: unsupported signing key algorithm %.64s", label, alg); return -1; }
    out->alg = own(l, alg); out->key = own(l, jv_text(k, "key"));
    return 0;
}
static int parse_assertions(const char *arg, assertion_list *l, char *error, size_t error_size) {
    char why[256];
    jv *list = file_or_json(arg, why, sizeof why);
    if (!list) { snprintf(error, error_size, "assertions must be a JSON list: %.200s", why); return -1; }
    if (list->kind != JV_ARRAY) { snprintf(error, error_size, "assertions must be a JSON list"); jv_free(list); return -1; }
    l->configs = calloc(list->count ? list->count : 1, sizeof *l->configs); if (!l->configs) abort();
    for (size_t i = 0; i < list->count; i++) {
        const jv *a = list->items[i], *statement = jv_field(a, "statement"), *value = jv_field(statement, "value"), *key = jv_field(a, "signingKey");
        otdf_assertion_config *c = &l->configs[l->count];
        c->id = own(l, jv_text(a, "id")); c->type = own(l, jv_text(a, "type")); c->scope = own(l, jv_text(a, "scope")); c->applies_to_state = own(l, jv_text(a, "appliesToState"));
        c->format = own(l, jv_text(statement, "format")); c->schema = own(l, jv_text(statement, "schema"));
        if (!value || value->kind == JV_NULL) c->value = own(l, "");
        else if (value->kind == JV_STRING) c->value = own(l, value->text);
        else if (value->kind == JV_OBJECT) { char *buf = NULL; size_t n = 0, cap = 0; jv_write(value, &buf, &n, &cap); c->value = own(l, buf); free(buf); }
        else { snprintf(error, error_size, "assertion %.120s: statement value is neither a string nor an object", c->id); jv_free(list); return -1; }
        if (key && key->kind != JV_NULL) {
            char label[300]; snprintf(label, sizeof label, "assertion %s", c->id);
            if (parse_assertion_key(l, key, label, &c->signing_key, error, error_size)) { jv_free(list); return -1; }
        }
        l->count++;
    }
    jv_free(list);
    return 0;
}
typedef struct { otdf_assertion_key default_key; otdf_assertion_key_entry *entries; size_t count; char **owned; size_t owned_count; } verification_keys;
static int parse_verification_keys(const char *arg, verification_keys *v, char *error, size_t error_size) {
    char why[256];
    jv *file = file_or_json(arg, why, sizeof why);
    if (!file) { snprintf(error, error_size, "assertion verification keys must be a JSON object: %.180s", why); return -1; }
    if (file->kind != JV_OBJECT) { snprintf(error, error_size, "assertion verification keys must be a JSON object"); jv_free(file); return -1; }
    assertion_list owner = {0};
    const jv *d = jv_field(file, "defaultKey"), *keys = jv_field(file, "keys");
    if (d && d->kind == JV_OBJECT) { v->default_key.alg = own(&owner, jv_text(d, "alg")); v->default_key.key = own(&owner, jv_text(d, "key")); }
    if (keys && keys->kind == JV_OBJECT) {
        v->entries = calloc(keys->count ? keys->count : 1, sizeof *v->entries); if (!v->entries) abort();
        for (size_t i = 0; i < keys->count; i++) { v->entries[i].assertion_id = own(&owner, keys->keys[i]); v->entries[i].alg = own(&owner, jv_text(keys->items[i], "alg")); v->entries[i].key = own(&owner, jv_text(keys->items[i], "key")); }
        v->count = keys->count;
    }
    v->owned = owner.owned; v->owned_count = owner.owned_count;
    jv_free(file);
    return 0;
}
static void verification_keys_free(verification_keys *v) { for (size_t i = 0; i < v->owned_count; i++) free(v->owned[i]); free(v->owned); free(v->entries); }

typedef struct { char *id, *secret, *dpop_key; } credentials;
/* Builds the SDK from --host, --with-client-creds and the DPoP and TLS flags; the SDK discovers everything else. */
static int new_sdk(const parsed *p, otdf_sdk **sdk, char *host, size_t host_size, credentials *c) {
    snprintf(host, host_size, "%s", value_of(p, "host"));
    size_t n = strlen(host);
    while (n && host[n - 1] == '/') host[--n] = 0;
    if (!*host) { fprintf(stderr, "--host is required\n%s", usage); return EXIT_USAGE; }
    const char *creds = value_of(p, "with-client-creds");
    if (!*creds) { fprintf(stderr, "--with-client-creds is required\n%s", usage); return EXIT_USAGE; }
    c->id = json_field(creds, "clientId"); c->secret = json_field(creds, "clientSecret");
    if (!c->id || !*c->id || !c->secret || !*c->secret) { fprintf(stderr, "--with-client-creds must be JSON with clientId and clientSecret\n"); return EXIT_USAGE; }
    otdf_options o = OTDF_OPTIONS_INIT;
    o.client_id = c->id; o.client_secret = c->secret;
    o.insecure_plaintext = !strncasecmp(host, "http://", 7);
    o.insecure_skip_verify = flag_of(p, "tls-no-verify");
    o.timeout_millis = TIMEOUT_MILLIS;
    if (*value_of(p, "dpop")) o.dpop_algorithm = value_of(p, "dpop");
    if (*value_of(p, "dpop-key")) {
        if (!*value_of(p, "dpop")) { fprintf(stderr, "--dpop-key needs --dpop\n"); return EXIT_USAGE; }
        uint8_t *pem; size_t len;
        if (read_file(value_of(p, "dpop-key"), &pem, &len)) { fprintf(stderr, "Failed to read DPoP key %s: %s\n", value_of(p, "dpop-key"), strerror(errno)); return EXIT_FAILED; }
        c->dpop_key = strndup((const char *)pem, len);
        free(pem);
        o.dpop_key_pem = c->dpop_key;
    }
    otdf_call call = {.timeout_millis = TIMEOUT_MILLIS};
    otdf_error e = {0};
    if (otdf_sdk_new(host, &o, &call, sdk, &e)) { describe_new(&e, host, stderr); otdf_error_free(&e); return EXIT_FAILED; }
    return EXIT_OK;
}
static void credentials_free(credentials *c) { free(c->id); free(c->secret); free(c->dpop_key); }

static int version(int argc, char **argv) {
    bool as_json = false;
    for (int i = 0; i < argc; i++) {
        if (!strcmp(argv[i], "--json")) as_json = true;
        else { fprintf(stderr, "unknown flag: %s\n", argv[i]); return EXIT_USAGE; }
    }
    char *v = NULL, *schema = NULL, **features = NULL; size_t count = 0;
    otdf_error e = {0};
    if (otdf_version(&v, &e) || otdf_tdf_spec_version(&schema, &e) || otdf_supported_features(&features, &count, &e)) {
        fputs("Failed to read version: ", stderr); describe(&e, stderr); otdf_error_free(&e); free(v); free(schema); return EXIT_FAILED;
    }
    if (as_json) {
        printf("{\"sdk_version\":\"%s\",\"schema_version\":\"%s\",\"supported_features\":[", v, schema);
        for (size_t i = 0; i < count; i++) printf("%s\"%s\"", i ? "," : "", features[i]);
        printf("],\"target\":\"%s\"}\n", TARGET);
    } else {
        printf("otdfgc (%s) sdk %s tdf-spec %s features ", TARGET, v, schema);
        for (size_t i = 0; i < count; i++) printf("%s%s", i ? "," : "", features[i]);
        printf("\n");
    }
    free(v); free(schema); otdf_strings_free(features, count);
    return EXIT_OK;
}
static int supports(int argc, char **argv) {
    if (argc != 1 || argv[0][0] == '-') { fprintf(stderr, "usage: otdfgc supports <feature>\n"); return EXIT_USAGE; }
    char **features = NULL; size_t count = 0;
    otdf_error e = {0};
    if (otdf_supported_features(&features, &count, &e)) { fputs("Failed to read supported features: ", stderr); describe(&e, stderr); otdf_error_free(&e); return EXIT_FAILED; }
    bool found = false;
    for (size_t i = 0; i < count; i++) if (!strcmp(features[i], argv[0])) found = true;
    otdf_strings_free(features, count);
    if (found) return EXIT_OK;
    printf("%s unsupported\n", argv[0]);
    return EXIT_FAILED;
}
/* inspect reads the manifest through the handle API: an SDK with a provided
 * platform configuration needs no platform, and the loaded reader's manifest
 * is printed keyed by the reference's JSON tags. */
static void pretty(const char *json) {
    int depth = 0; bool in_string = false;
    for (const char *p = json; *p; p++) {
        char c = *p;
        if (in_string) { putchar(c); if (c == '\\' && p[1]) putchar(*++p); else if (c == '"') in_string = false; continue; }
        if (c == '"') { in_string = true; putchar(c); }
        else if (c == '{' || c == '[') { putchar(c); if (p[1] == '}' || p[1] == ']') { putchar(*++p); continue; } depth++; printf("\n%*s", depth * 2, ""); }
        else if (c == '}' || c == ']') { depth--; printf("\n%*s%c", depth * 2, "", c); }
        else if (c == ',') printf(",\n%*s", depth * 2, "");
        else if (c == ':') printf(": ");
        else putchar(c);
    }
    putchar('\n');
}
static int inspect(int argc, char **argv) {
    if (argc != 1 || argv[0][0] == '-') { fprintf(stderr, "usage: otdfgc inspect <in>\n"); return EXIT_USAGE; }
    uint8_t *data; size_t n;
    if (read_file(argv[0], &data, &n)) { fprintf(stderr, "not a valid TDF: %s\n", strerror(errno)); return EXIT_FAILED; }
    otdf_entry issuer = {"platform_issuer", "http://localhost"};
    otdf_options o = OTDF_OPTIONS_INIT;
    o.platform_configuration = &issuer; o.platform_configuration_count = 1; o.insecure_plaintext = true;
    otdf_reader_options ro = OTDF_READER_OPTIONS_INIT;
    ro.ignore_allowlist = true;
    otdf_sdk *sdk = NULL; otdf_reader *reader = NULL; char *json = NULL;
    otdf_error e = {0};
    int rc = otdf_sdk_new("http://localhost", &o, NULL, &sdk, &e);
    if (!rc) rc = otdf_load_tdf(sdk, (otdf_bytes){data, n}, &ro, NULL, &reader, &e);
    if (!rc) rc = otdf_reader_manifest_json(reader, NULL, &json, &e);
    free(data);
    if (reader) { otdf_reader_close(reader, NULL); otdf_reader_free(reader); }
    if (sdk) { otdf_sdk_close(sdk, NULL); otdf_sdk_free(sdk); }
    if (rc) { fprintf(stderr, "not a valid TDF: %s\n", e.message ? e.message : ""); otdf_error_free(&e); return EXIT_FAILED; }
    pretty(json);
    free(json);
    return EXIT_OK;
}
static int encrypt(int argc, char **argv) {
    parsed p; char error[256], reason[256];
    if (parse(argc, argv, encrypt_flags, &p, error, sizeof error)) { fprintf(stderr, "%s\n%s", error, usage); return EXIT_USAGE; }
    if (unsupported(&p, encrypt_unsupported, reason, sizeof reason)) { fprintf(stderr, "unsupported in this build: %s\n", reason); return EXIT_USAGE; }
    if (p.positional_count != 1 || !*value_of(&p, "out")) { fprintf(stderr, "encrypt needs exactly one input file and --out\n%s", usage); return EXIT_USAGE; }
    FILE *input = fopen(p.positional[0], "rb");
    if (!input) { fprintf(stderr, "Failed to read %s: %s\n", p.positional[0], strerror(errno)); return EXIT_FAILED; }
    otdf_sdk *sdk = NULL; char host[2048]; credentials c = {0};
    int rc = new_sdk(&p, &sdk, host, sizeof host, &c);
    if (rc) { fclose(input); credentials_free(&c); return rc; }
    /* As otdfctl: the platform's KAS is the default key access server and attributes drive autoconfigure. */
    char kas[2100];
    snprintf(kas, sizeof kas, "%s/kas", host);
    otdf_kas_info info = {.url = kas};
    char **attributes = NULL;
    size_t attribute_count = split_list(value_of(&p, "attr"), &attributes);
    otdf_tdf_options o = OTDF_TDF_OPTIONS_INIT;
    o.kas_information = &info; o.kas_information_count = 1; o.mime_type = value_of(&p, "mime-type");
    o.attributes = (const char *const *)attributes; o.attribute_count = attribute_count;
    if (*value_of(&p, "wrapping-key-algorithm")) o.wrapping_key_algorithm = value_of(&p, "wrapping-key-algorithm");
    if (*value_of(&p, "target-mode")) o.target_mode = value_of(&p, "target-mode");
    assertion_list assertions = {0};
    if (*value_of(&p, "with-assertions")) {
        if (parse_assertions(value_of(&p, "with-assertions"), &assertions, error, sizeof error)) {
            fprintf(stderr, "Failed to read assertions: %s\n", error);
            otdf_sdk_close(sdk, NULL); otdf_sdk_free(sdk); fclose(input); otdf_strings_free(attributes, attribute_count); credentials_free(&c); assertion_list_free(&assertions);
            return EXIT_USAGE;
        }
        o.assertions = assertions.configs; o.assertion_count = assertions.count;
    }
    char tmp[4096];
    snprintf(tmp, sizeof tmp, "%s.tmp", value_of(&p, "out"));
    FILE *output = fopen(tmp, "wb");
    if (!output) {
        fprintf(stderr, "Failed to write encrypted file %s: %s\n", value_of(&p, "out"), strerror(errno));
        otdf_sdk_close(sdk, NULL); otdf_sdk_free(sdk); fclose(input); otdf_strings_free(attributes, attribute_count); credentials_free(&c); assertion_list_free(&assertions);
        return EXIT_FAILED;
    }
    otdf_call call = {.timeout_millis = TIMEOUT_MILLIS};
    otdf_error e = {0};
    gxc_stream out = otdf_stream_file(output), in = otdf_stream_file(input);
    rc = otdf_encrypt_to(sdk, &out, &in, &o, &call, NULL, &e);
    otdf_sdk_close(sdk, NULL);
    otdf_sdk_free(sdk);
    fclose(input); otdf_strings_free(attributes, attribute_count); credentials_free(&c); assertion_list_free(&assertions);
    bool flushed = !fclose(output);
    if (rc) { remove(tmp); fputs("Failed to encrypt: ", stderr); describe(&e, stderr); otdf_error_free(&e); return EXIT_FAILED; }
    if (!flushed || chmod(tmp, 0644) || rename(tmp, value_of(&p, "out"))) { remove(tmp); fprintf(stderr, "Failed to write encrypted file %s: %s\n", value_of(&p, "out"), strerror(errno)); return EXIT_FAILED; }
    return EXIT_OK;
}
static int decrypt(int argc, char **argv) {
    parsed p; char error[256], reason[256];
    if (parse(argc, argv, decrypt_flags, &p, error, sizeof error)) { fprintf(stderr, "%s\n%s", error, usage); return EXIT_USAGE; }
    if (unsupported(&p, decrypt_unsupported, reason, sizeof reason)) { fprintf(stderr, "unsupported in this build: %s\n", reason); return EXIT_USAGE; }
    if (p.positional_count != 1 || !*value_of(&p, "out")) { fprintf(stderr, "decrypt needs exactly one input file and --out\n%s", usage); return EXIT_USAGE; }
    struct stat st;
    if (stat(p.positional[0], &st)) { fprintf(stderr, "Failed to read file: %s\n", strerror(errno)); return EXIT_FAILED; }
    otdf_sdk *sdk = NULL; char host[2048]; credentials c = {0};
    int rc = new_sdk(&p, &sdk, host, sizeof host, &c);
    if (rc) { credentials_free(&c); return rc; }
    otdf_reader_options o = OTDF_READER_OPTIONS_INIT;
    o.disable_assertion_verification = flag_of(&p, "no-verify-assertions");
    verification_keys keys = {0};
    if (*value_of(&p, "with-assertion-verification-keys")) {
        if (parse_verification_keys(value_of(&p, "with-assertion-verification-keys"), &keys, error, sizeof error)) {
            fprintf(stderr, "Failed to read assertion verification keys: %s\n", error);
            otdf_sdk_close(sdk, NULL); otdf_sdk_free(sdk); credentials_free(&c); verification_keys_free(&keys);
            return EXIT_USAGE;
        }
        o.assertion_default_key = keys.default_key; o.assertion_keys = keys.entries; o.assertion_key_count = keys.count;
    }
    if (*value_of(&p, "session-key-algorithm")) o.session_key_type = value_of(&p, "session-key-algorithm");
    /* otdfctl: "*" alone ignores the allowlist; any other list is explicit. */
    char **allowlist = NULL;
    size_t allowlist_count = split_list(value_of(&p, "kas-allowlist"), &allowlist);
    if (allowlist_count == 1 && !strcmp(allowlist[0], "*")) o.ignore_allowlist = true;
    else if (allowlist_count) { o.kas_allowlist = (const char *const *)allowlist; o.kas_allowlist_count = allowlist_count; }
    otdf_call call = {.timeout_millis = TIMEOUT_MILLIS};
    otdf_error e = {0};
    rc = otdf_decrypt_file(sdk, p.positional[0], value_of(&p, "out"), &o, &call, &e);
    otdf_sdk_close(sdk, NULL);
    otdf_sdk_free(sdk);
    otdf_strings_free(allowlist, allowlist_count); credentials_free(&c); verification_keys_free(&keys);
    if (rc) { fputs("Failed to decrypt file: ", stderr); describe(&e, stderr); otdf_error_free(&e); return EXIT_FAILED; }
    return EXIT_OK;
}
int main(int argc, char **argv) {
    if (argc < 2) { fputs(usage, stderr); return EXIT_USAGE; }
    const char *command = argv[1];
    int rest = argc - 2; char **args = argv + 2;
    if (!strcmp(command, "--version") || !strcmp(command, "version")) return version(rest, args);
    if (!strcmp(command, "supports")) return supports(rest, args);
    if (!strcmp(command, "inspect")) return inspect(rest, args);
    if (!strcmp(command, "encrypt")) return encrypt(rest, args);
    if (!strcmp(command, "decrypt")) return decrypt(rest, args);
    if (!strcmp(command, "help") || !strcmp(command, "--help") || !strcmp(command, "-h")) { fputs(usage, stdout); return EXIT_OK; }
    fprintf(stderr, "unknown command: %s\n%s", command, usage);
    return EXIT_USAGE;
}
