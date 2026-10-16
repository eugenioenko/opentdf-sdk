/* The handle API of api-spec.md in C idiom: opaque otdf_sdk, otdf_reader and
 * otdf_tdf_object handles, zero-initialised option structs with explicit
 * lengths, a status code from every call and an otdf_error out parameter.
 * Strings are NUL-terminated UTF-8 (NULL means unset); payloads carry lengths.
 * Manifests, policies and keys are returned as JSON text keyed by the
 * reference's JSON tags. The platform service clients take and return
 * protobuf JSON text (opentdf_services.h). The adapter maps names and errors
 * only; the TDF protocol executes in the generated library. */
#ifndef OPENTDF_H
#define OPENTDF_H
#include "library.h"
#include <stdatomic.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct otdf_sdk otdf_sdk;
typedef struct otdf_reader otdf_reader;
typedef struct otdf_tdf_object otdf_tdf_object;

typedef struct { const uint8_t *data; size_t length; } otdf_bytes;
/* Output bytes; release with otdf_bytes_free. */
typedef struct { uint8_t *data; size_t length; } otdf_owned_bytes;
typedef struct { const char *key, *value; } otdf_entry;

/* The reference's Options (shared-source Options); every zero member is the
 * reference default. Initialise with OTDF_OPTIONS_INIT. */
typedef struct {
    const char *client_id, *client_secret;
    const char *const *scopes; size_t scope_count;
    const char *token_endpoint;
    const char *token_exchange_subject_token; const char *const *token_exchange_audience; size_t token_exchange_audience_count;
    const char *cert_exchange_client_cert_pem, *cert_exchange_client_key_pem; const char *const *cert_exchange_audience; size_t cert_exchange_audience_count;
    /* A host token source replacing client credentials: the gxc_provider
     * receives the request and answers the JSON {"value","scheme","expiresAt","confirmationJKT"}. */
    gxc_provider token_provider; void *token_provider_state;
    /* WithPlatformConfiguration; a non-string member is carried as its JSON text. */
    const otdf_entry *platform_configuration; size_t platform_configuration_count;
    bool validate_connection, insecure_plaintext, insecure_skip_verify;
    const char *tls_client_cert_pem, *tls_client_key_pem, *tls_root_cas_pem; const char *const *tls_audience; size_t tls_audience_count;
    const char *unix_socket;
    int64_t timeout_millis;
    /* WithCustom*Connection: core_endpoint for every service, the others for
     * their own services, all through the SDK's transport. */
    const char *core_endpoint, *policy_endpoint, *authorization_endpoint, *entity_resolution_endpoint, *wellknown_endpoint;
    const char *session_encryption_rsa_pem, *session_signer_rsa_pem;
    /* WithDPoPAlgorithm: ES256, ES384, ES512, RS256, RS384 or RS512. */
    const char *dpop_algorithm, *dpop_key_pem;
    bool no_kid_in_kao;
    const char *const *fulfillable_obligation_fqns; size_t fulfillable_obligation_fqn_count;
    int64_t log_level;
} otdf_options;
#define OTDF_OPTIONS_INIT {0}

/* The reference's KASInfo: one key access server for the template. */
typedef struct { const char *url, *public_key, *kid, *algorithm; bool is_default; } otdf_kas_info;
typedef struct { const char *id, *type, *scope, *applies_to_state, *format, *schema, *value; } otdf_assertion_config;
/* The reference's TDFConfig options (shared-source TDFOptions). */
typedef struct {
    const char *const *attributes; size_t attribute_count;
    bool disable_autoconfigure;
    const otdf_kas_info *kas_information; size_t kas_information_count;
    /* rsa:2048 (default) or ec:secp256r1 in this build. */
    const char *wrapping_key_algorithm;
    int64_t segment_size, input_size;
    const char *mime_type, *metadata;
    const otdf_assertion_config *assertions; size_t assertion_count;
    bool system_metadata_assertion;
    const char *target_mode;
} otdf_tdf_options;
#define OTDF_TDF_OPTIONS_INIT {0}
/* The reference's TDFReaderConfig options (shared-source ReaderOptions). */
typedef struct {
    bool disable_assertion_verification;
    /* NULL fetches the registry's list. */
    const char *const *kas_allowlist; size_t kas_allowlist_count;
    bool ignore_allowlist;
    int64_t max_manifest_size, schema_validation;
    /* rsa:2048 (default) or ec:secp256r1. */
    const char *session_key_type;
    const char *const *fulfillable_obligation_fqns; size_t fulfillable_obligation_fqn_count;
} otdf_reader_options;
#define OTDF_READER_OPTIONS_INIT {0}
typedef struct { const char *pem, *kid, *alg; } otdf_kas_public_key;

/* Per-call cancellation and deadline; NULL means neither. */
typedef struct { const atomic_bool *canceled; int64_t timeout_millis; } otdf_call;

/* The shared-source Error record: kind is the stable ErrorKind string
 * ("tampered", "rewrap_forbidden", "closed", ...), message the reference's
 * wording. Zero-initialise and release with otdf_error_free. */
typedef struct {
    char *kind, *op, *message;
    int64_t http_status;
    char *connect_code, *service_message, *kas_url;
    char **required_obligations; size_t required_obligation_count;
    char *assertion_id;
} otdf_error;
/* Every call returns 0 on success, else the generated library's error kind:
 * 1 an SDK error (error->kind names it), 2 source panic, 3 host fault,
 * 4 fatal, 5 invalid argument, 6 canceled, 7 closed handle, 8 poisoned,
 * 9 handles of different instances. error may be NULL. */

void otdf_error_free(otdf_error *error);
void otdf_bytes_free(otdf_owned_bytes *bytes);
void otdf_string_free(char *string);
void otdf_strings_free(char **strings, size_t count);

/* The reference's New: discovers the platform configuration and the token endpoint. */
int otdf_sdk_new(const char *platform_endpoint, const otdf_options *options, const otdf_call *call, otdf_sdk **sdk, otdf_error *error);
/* Runs the source Close and releases the instance; later calls on the handle
 * fail with kind 7 ("closed"). The handle stays valid until otdf_sdk_free. */
int otdf_sdk_close(otdf_sdk *sdk, otdf_error *error);
/* Frees the handle, releasing it without Close when it is still open. */
void otdf_sdk_free(otdf_sdk *sdk);

/* The reference's CreateTDF over bytes: the TDF bytes with their TDFObject handle. */
int otdf_create_tdf(otdf_sdk *sdk, otdf_bytes plaintext, const otdf_tdf_options *options, const otdf_call *call, otdf_owned_bytes *tdf, otdf_tdf_object **object, otdf_error *error);
/* otdf_create_tdf without keeping the handle: the TDF bytes only. */
int otdf_encrypt_bytes(otdf_sdk *sdk, otdf_bytes plaintext, const otdf_tdf_options *options, const otdf_call *call, otdf_owned_bytes *tdf, otdf_error *error);
int otdf_tdf_object_manifest_json(otdf_tdf_object *object, const otdf_call *call, char **json, otdf_error *error);
int otdf_tdf_object_size(otdf_tdf_object *object, const otdf_call *call, int64_t *size, otdf_error *error);
/* Close and free as for otdf_sdk: close releases, free disposes of the handle. */
int otdf_tdf_object_close(otdf_tdf_object *object, otdf_error *error);
void otdf_tdf_object_free(otdf_tdf_object *object);

/* The reference's LoadTDF over bytes; the reader keeps a copy of the bytes until it is closed. */
int otdf_load_tdf(otdf_sdk *sdk, otdf_bytes ciphertext, const otdf_reader_options *options, const otdf_call *call, otdf_reader **reader, otdf_error *error);
/* Fetches and unwraps the payload key now. */
int otdf_reader_init(otdf_reader *reader, const otdf_call *call, otdf_error *error);
/* The manifest keyed by the reference's JSON tags (encryptionInformation.keyAccess[0].url, ...). */
int otdf_reader_manifest_json(otdf_reader *reader, const otdf_call *call, char **json, otdf_error *error);
/* The policy keyed by the reference's JSON tags (uuid, body.dataAttributes, body.dissem). */
int otdf_reader_policy_json(otdf_reader *reader, const otdf_call *call, char **json, otdf_error *error);
int otdf_reader_data_attributes(otdf_reader *reader, const otdf_call *call, char ***attributes, size_t *count, otdf_error *error);
/* The decrypted metadata; runs init lazily. */
int otdf_reader_unencrypted_metadata(otdf_reader *reader, const otdf_call *call, otdf_owned_bytes *metadata, otdf_error *error);
/* Close and free as for otdf_sdk: close releases, free disposes of the handle and its bytes. */
int otdf_reader_close(otdf_reader *reader, otdf_error *error);
void otdf_reader_free(otdf_reader *reader);
int otdf_decrypt_bytes(otdf_sdk *sdk, otdf_bytes ciphertext, const otdf_reader_options *options, const otdf_call *call, otdf_owned_bytes *plaintext, otdf_error *error);

int otdf_is_healthy(otdf_sdk *sdk, const otdf_call *call, bool *healthy, otdf_error *error);
/* The platform configuration members as a JSON object of strings. */
int otdf_platform_configuration_json(otdf_sdk *sdk, const otdf_call *call, char **json, otdf_error *error);
/* The reference's GetBaseKey as JSON (kasUri, publicKey{algorithm,kid,pem}, kasId). */
int otdf_get_base_key_json(otdf_sdk *sdk, const otdf_call *call, char **json, otdf_error *error);
int otdf_store_kas_keys(otdf_sdk *sdk, const char *url, const otdf_kas_public_key *keys, size_t count, const otdf_call *call, otdf_error *error);
/* Sends one unary request to a platform service method, such as
 * ("policy.attributes.AttributesService", "ListAttributes"), with the request
 * and *response_json in protobuf JSON (NULL request_json is {}): the shared
 * source's SDK.Call. A failure the service reports sets error->connect_code,
 * service_message and http_status. Release *response_json with otdf_string_free. */
int otdf_sdk_call(otdf_sdk *sdk, const char *service, const char *method, const char *request_json, const otdf_call *call, char **response_json, otdf_error *error);

/* The reference's attribute helpers. otdf_list_attributes sets
 * *attributes_json to a JSON array of policy.Attribute in protobuf JSON;
 * namespace_name NULL lists every namespace. entity_json is an
 * authorization.Entity in protobuf JSON (NULL is the reference's nil). */
int otdf_list_attributes(otdf_sdk *sdk, const char *namespace_name, const otdf_call *call, char **attributes_json, otdf_error *error);
int otdf_get_entity_attributes(otdf_sdk *sdk, const char *entity_json, const otdf_call *call, char ***fqns, size_t *count, otdf_error *error);
int otdf_validate_attributes(otdf_sdk *sdk, const char *const *fqns, size_t count, const otdf_call *call, otdf_error *error);
int otdf_attribute_exists(otdf_sdk *sdk, const char *attribute_fqn, const otdf_call *call, bool *exists, otdf_error *error);
int otdf_attribute_value_exists(otdf_sdk *sdk, const char *value_fqn, const otdf_call *call, bool *exists, otdf_error *error);

/* SDK.Auth().AccessToken: the SDK's access token with its scheme ("Bearer"
 * or "DPoP"), expiry in UTC Unix seconds and DPoP thumbprint. Zero-initialise
 * and release with otdf_access_token_free. */
typedef struct { char *value, *scheme; int64_t expires_at; char *confirmation_jkt; } otdf_access_token;
void otdf_access_token_free(otdf_access_token *token);
int otdf_auth_access_token(otdf_sdk *sdk, const otdf_call *call, otdf_access_token *token, otdf_error *error);
/* SDK.Conn: the endpoint of the core services (core_endpoint, else the
 * platform endpoint). Release *endpoint with otdf_string_free. */
int otdf_sdk_conn(otdf_sdk *sdk, const otdf_call *call, char **endpoint, otdf_error *error);

/* Free functions of the reference, each a library call. */
int otdf_is_valid_tdf(otdf_bytes data, const otdf_call *call, bool *valid, otdf_error *error);
int otdf_get_tdf_type(otdf_bytes data, const otdf_call *call, char **type, otdf_error *error);
int otdf_is_platform_endpoint_malformed(const char *endpoint, bool *malformed, otdf_error *error);
int otdf_key_type_to_policy_algorithm(const char *key_type, char **algorithm, otdf_error *error);
int otdf_policy_algorithm_to_key_type(const char *algorithm, char **key_type, otdf_error *error);
int otdf_version(char **version, otdf_error *error);
int otdf_tdf_spec_version(char **version, otdf_error *error);
int otdf_supported_features(char ***features, size_t *count, otdf_error *error);
#include "opentdf_services.h"
#endif
