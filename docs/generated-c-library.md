# Native C library

`opentdf.h` exposes the handle API of [api-spec.md](api-spec.md) in C idiom
(`otdf_sdk_new`, `otdf_create_tdf`, `otdf_load_tdf`, `otdf_decrypt_bytes`,
...), over the generated library compiled from the export root `src`.
The hand-written [adapter](../src/hosts/c/opentdf.c) maps names, values and
errors onto `goalchemy_invoke` and implements no protocol. Production calls
need no compiler, Go process or source SDK process. This delivery targets
Linux x86_64 with threaded Boehm GC 8.2.8, OpenSSL 3 EVP and libcurl's
OpenSSL backend. Exact tested package/runtime/source/license identities are
in `src/hosts/c/dependencies.lock.json`; external native dependencies are not
bundled.

## Handle API

```c
#include "opentdf.h"
otdf_options o = OTDF_OPTIONS_INIT;
o.client_id = "opentdf-sdk"; o.client_secret = "secret"; o.insecure_plaintext = true;
otdf_sdk *sdk; otdf_error e = {0};
if (otdf_sdk_new("http://localhost:8080", &o, NULL, &sdk, &e)) { fprintf(stderr, "%s: %s\n", e.kind, e.message); otdf_error_free(&e); }

const char *attr = "https://example.com/attr/attr1/value/value1";
otdf_kas_info kas = {.url = "http://localhost:8080/kas"};
otdf_tdf_options t = OTDF_TDF_OPTIONS_INIT;
t.attributes = &attr; t.attribute_count = 1; t.kas_information = &kas; t.kas_information_count = 1;
otdf_owned_bytes tdf; otdf_tdf_object *object; char *manifest;
otdf_create_tdf(sdk, (otdf_bytes){plaintext, n}, &t, NULL, &tdf, &object, &e);
otdf_tdf_object_manifest_json(object, NULL, &manifest, &e);   /* keyed by the reference's JSON tags */
otdf_tdf_object_close(object, &e); otdf_tdf_object_free(object);

const char *allow = "http://localhost:8080";
otdf_reader_options r = OTDF_READER_OPTIONS_INIT; r.kas_allowlist = &allow; r.kas_allowlist_count = 1;
otdf_reader *reader;
otdf_load_tdf(sdk, (otdf_bytes){tdf.data, tdf.length}, &r, NULL, &reader, &e);
otdf_reader_init(reader, NULL, &e);                           /* fetch and unwrap keys now */
otdf_reader_close(reader, &e); otdf_reader_free(reader);

atomic_bool cancel = false; otdf_call call = {.canceled = &cancel, .timeout_millis = 30000};
otdf_owned_bytes plain;
otdf_decrypt_bytes(sdk, (otdf_bytes){tdf.data, tdf.length}, &r, &call, &plain, &e);
otdf_sdk_close(sdk, &e); otdf_sdk_free(sdk);
```

| Function | Reference | Notes |
| --- | --- | --- |
| `otdf_sdk_new(endpoint, &options, call, &sdk, &error)` | `New` | Fetches `/.well-known/opentdf-configuration` and the issuer's OpenID configuration unless `platform_configuration` is given; `validate_connection` runs the health check first. |
| `otdf_create_tdf` → bytes and `otdf_tdf_object` (`otdf_tdf_object_manifest_json`, `_size`, `_close`, `_free`), `otdf_encrypt_bytes` | `CreateTDFContext` over bytes, `TDFObject` | Key access templates from autoconfigure or explicit KAS information, split across KAS; RSA-2048/4096 and P-256/384/521 wrapping. |
| `otdf_encrypt_to(sdk, const gxc_stream *output, const gxc_stream *plaintext, options, call, &object, &error)`, `otdf_stream_file(FILE *)` | `CreateTDFContext` over streams | `gxc_stream` carries `read`, `write`, `seek` and `close` callbacks (`library.h`); `otdf_stream_file` wraps an open stdio `FILE`. The payload is encrypted one segment at a time; a `seek` callback or `options->input_size` selects ZIP32, an unknown size writes ZIP64. |
| `otdf_load_tdf`, `otdf_load_tdf_stream(sdk, const gxc_stream *, ...)` → `otdf_reader` (`otdf_reader_init`, `_manifest_json`, `_policy_json`, `_data_attributes`, `_unencrypted_metadata`, `_read(buffer, length, &count)`, `_seek(offset, whence, call, &position)`, `_read_at(offset, length, call, &bytes)`, `_write_to(const gxc_stream *, call, &written)`, `_close`, `_free`), `otdf_decrypt_bytes`, `otdf_decrypt_to(sdk, output, ciphertext, ...)`, `otdf_decrypt_file(sdk, input_path, output_path, ...)` | `LoadTDF`, `Reader`, `DecryptBytes`, `DecryptTo`, `DecryptFile` | A stream with `read` and `seek` is read segment by segment and must outlive the reader; `otdf_reader_read` reads plaintext from the reader's position (at most 1 MiB per call, 0 at the end). Call `otdf_reader_init` first when the SDK uses a token provider (`_read` carries no callbacks). `otdf_decrypt_file` writes beside the output and renames into place. One rewrap request per KAS; the DEK is the XOR of one share per split id, any KAO of a split will do, duplicates are skipped; post-quantum KAOs fail as `unsupported_key_access`. RSA and EC sessions of every non-PQ key type. The reader keeps a copy of the bytes. The KAS allowlist comes from the registry unless `kas_allowlist` or `ignore_allowlist` is set. |
| `otdf_tdf_options.assertions` (`otdf_assertion_config` with an optional `signing_key {alg, key}`), `system_metadata_assertion`; `otdf_reader_options.assertion_default_key`, `assertion_keys`/`assertion_key_count` (`otdf_assertion_key_entry {assertion_id, alg, key}`), `disable_assertion_verification` | `WithAssertions`, `WithSystemMetadataAssertion`, `WithAssertionVerificationKeys`, `WithDisableAssertionVerification` | Keys are text: the HS256 secret or an RS256 PEM block. Assertions are signed into the manifest with HS256 over the payload key or the config's signing key and verified on read by default, by assertion id with a default key, or with the payload key; failures carry kind `assertion_failure` and `assertion_id`. |
| `otdf_is_healthy`, `otdf_platform_configuration_json`, `otdf_get_base_key_json`, `otdf_store_kas_keys`, `otdf_sdk_close`, `otdf_sdk_free` | same | Configuration and keys are returned as JSON text. |
| `otdf_is_valid_tdf`, `otdf_get_tdf_type`, `otdf_is_platform_endpoint_malformed`, `otdf_key_type_to_policy_algorithm`, `otdf_policy_algorithm_to_key_type`, `otdf_version`, `otdf_tdf_spec_version`, `otdf_supported_features` | same | Free functions. |
| `otdf_sdk_call(sdk, service, method, request_json, call, &response_json, &error)`, `otdf_<service>_<method>` (`opentdf_services.h`) | `SDK.Attributes` and the other `sdkconnect` clients | Protobuf JSON text in and out; see Service clients. |

### Service clients

The platform service clients are JSON functions: `opentdf_services.h`
(included by `opentdf.h`) declares one function per method of the
reference's `sdkconnect` clients, named after the accessor and method in
snake_case (`otdf_attributes_get_attribute`,
`otdf_namespaces_list_namespaces`, ...; 131 in all), each a call of
`otdf_sdk_call` with the service and method. The request is a protobuf JSON
object (NULL is `{}`) and the reply, protobuf JSON, is malloc-owned text
released with `otdf_string_free`. A failure the service reports fills
`connect_code`, `service_message` and `http_status`. Typed C messages are
[#29](https://github.com/eugenioenko/opentdf-sdk/issues/29).

```c
char *reply;
if (otdf_attributes_list_attributes(sdk, "{\"namespace\":\"example.com\"}", NULL, &reply, &e) == 0) { puts(reply); otdf_string_free(reply); }
else if (!strcmp(e.connect_code, "not_found")) { ... }
```

The helpers are `otdf_list_attributes(sdk, namespace_name, call, &json, &e)`
(a JSON array of `policy.Attribute`; NULL lists every namespace),
`otdf_get_entity_attributes(sdk, entity_json, call, &fqns, &count, &e)`,
`otdf_validate_attributes(sdk, fqns, count, call, &e)`,
`otdf_attribute_exists` and `otdf_attribute_value_exists` (a `bool` out);
`otdf_auth_access_token` fills an `otdf_access_token` (release with
`otdf_access_token_free`) and `otdf_sdk_conn` returns the core endpoint.
`otdf_options.core_endpoint`, `policy_endpoint`, `authorization_endpoint`,
`entity_resolution_endpoint` and `wellknown_endpoint` route those services
through the SDK's transport.

```c
bool exists;
if (otdf_attribute_value_exists(sdk, "https://example.com/attr/level/value/high", NULL, &exists, &e) == 0 && exists) { ... }
```

With autoconfigure on (the default), `otdf_tdf_options.attributes` drive the
reference's key split (grants and mapped keys, else the base key, else the
default KAS); `attribute_values` (each a `policy.Value` in protobuf JSON with
its definition) plan offline. `otdf_new_attribute_value_fqn(fqn, &url, &key,
&e)` and `otdf_new_attribute_name_fqn` return the FQN and its lower-case key
(release both with `otdf_string_free`).

```c
const char *attrs[] = {"https://example.com/attr/level/value/high"};
otdf_tdf_options o = OTDF_TDF_OPTIONS_INIT; o.attributes = attrs; o.attribute_count = 1;
```

Every call returns 0 or the generated library's error kind (1 an SDK error,
2 source panic, 3 host fault, 4 fatal, 5 invalid argument, 6 canceled, 7
closed handle, 8 poisoned, 9 handles of different instances) and fills the
optional `otdf_error` with `kind` (the `ErrorKind` string of api-spec.md, or
the boundary kind's name), `op`, `message` (the reference's wording, see
[error-wording.md](error-wording.md)), `http_status`, `connect_code`,
`service_message`, `kas_url`, `required_obligations` and `assertion_id`;
release it with `otdf_error_free`. Option structs are zero-initialised
(`OTDF_OPTIONS_INIT`, `OTDF_TDF_OPTIONS_INIT`, `OTDF_READER_OPTIONS_INIT`)
with the shared field names in snake_case: strings are NUL-terminated UTF-8
(NULL is unset), lists carry explicit counts, payloads are `otdf_bytes` with
lengths. Outputs (`otdf_owned_bytes`, strings, string lists) are
malloc-owned and released with `otdf_bytes_free`, `otdf_string_free` and
`otdf_strings_free`. An `otdf_call` carries an atomic cancellation flag and a
per-call deadline (NULL means neither). A host token source is a
`gxc_provider` in `token_provider` answering the JSON
`{"value","scheme","expiresAt","confirmationJKT"}`. `*_close` runs the
source Close and releases the instance (later calls fail with kind 7);
`*_free` disposes of the handle. Options this build does not implement yet
fail with kind `unsupported_feature` and `unsupported in this build: <option>`.

`src/hosts/c/cmd/otdfgc/otdfgc.c` is the native C CLI over this library,
built by `scripts/build-c-cli.sh <package> <launcher>` (linking `libtdf3.a`
with the dependencies of `native-link.env`) and driven by the xtest harness
as the `gcc` SDK type (see [xtest.md](xtest.md)); it implements the Go
`otdfgc` contract. The consumer in `tests/interop/generatedc/main.c` drives
the real platform (including the service clients) and, in `controlled` mode, the offline checks of
`tests/interop/delivery/controlled_platform.py`. The C runtime reports an
expired request timeout as a transport failure, so the per-request timeout
(`timeout_millis`) surfaces as kind `transport` or `deadline_exceeded`,
whichever of libcurl's timeout and the call deadline fires first; a cancelled `otdf_call` flag is reported as `canceled`.

The sections below describe the package build and the library's value and
ownership rules.

## Build and isolated import

Install the zlib development package (`zlib1g-dev` on Ubuntu) alongside OpenSSL
and libcurl headers. Goalchemy 0.6.0 uses zlib directly for HTTP gzip decoding,
so consumers must include `-lz` when linking the static SDK archive.

From the SDK checkout, run `bash scripts/build-generated-c.sh /absolute/output`. `GOALCHEMY_BIN` selects a freshly built Goalchemy compiler; `GOALCHEMY_BDWGC` selects the threaded collector installation. `TDF3_CURL_PREFIX` optionally selects locally extracted matching libcurl development/runtime packages. The default development prefix is ignored `.local/root-c-development-prerequisites/prefix`. `CC`, `CFLAGS`, `CPPFLAGS`, and `AR` select native build tools/flags. The build emits `libtdf3.a`, public headers, `native-link.env`, content hashes, and a reproducibly ordered archive `opentdf-tdf3-c-0.1.0-linux-x86_64.tar.gz`.

Extract that archive in a separate directory. A consumer includes only `opentdf.h` from its `include/` directory and links the SDK archive plus native dependencies:

```sh
cc -std=c17 -I package/include consumer.c package/lib/libtdf3.a \
  -L "$CURL_PREFIX/usr/lib/x86_64-linux-gnu" -lcurl -lssl -lcrypto -lz \
  "$GOALCHEMY_BDWGC/lib/libgc.a" -lpthread -ldl -o consumer
LD_LIBRARY_PATH="$CURL_PREFIX/usr/lib/x86_64-linux-gnu" ./consumer
```

Production consumers do not initialize Boehm or register their own threads; the SDK initializes the collector on its first owned source thread, retires that primordial registration after cleanup, and joins each owner before completion. First calls may originate synchronously or asynchronously from ordinary native threads that subsequently exit. Existing consumer collector registrations remain consumer-owned. A consumer using Boehm independently remains responsible for its own allocations. `tests/interop/generatedc/install-consumer.sh` builds an actual extracted-package importer. For a compiler-free native rebuild, enter `package/src` and run `GOALCHEMY_BDWGC=... CPPFLAGS=... sh build.sh`; this emits `libtdf3.a` and `libgoalchemy.a` from the supplied C source/runtime. Supply matching curl include flags through `CPPFLAGS`. No stale object glob enters either archive.

## Values, ownership, and completion

All bytes carry explicit `size_t` lengths, including configuration strings; no terminator is required. Integer fields are exact `int64_t`. Recursive input configuration/options/bytes are copied before blocking invocation waits and before asynchronous submission returns. Source execution is serialized across callers on a fresh registered collector owner, with source initialization per call. Successful payload/metadata/manifest and failure message/fields are detached into malloc-owned buffers while that owner remains alive. They remain valid after later calls and must be released with `tdf3_result_free` or `tdf3_error_free`; releases accept nil and zero their fields. Initialize result/error structures to zero, release prior owned content before reusing them, and never free borrowed input through these functions.

`IncludeMetadata` distinguishes absent metadata from explicitly included empty metadata. `HasMetadata` on decryption reflects the archive's encrypted metadata presence independently of decrypted byte length. The stock Go producer omits explicit-empty metadata; the stock Web producer encrypts empty metadata and its reader does not expose decrypted metadata. Interoperability tests derive expected presence from each immutable producer manifest.

`tdf3_encrypt_submit` and `tdf3_decrypt_submit` return an opaque operation. `tdf3_operation_drive(op,0)` polls, `-1` waits, and positive milliseconds bound one wait. `wake` signals the actual active runtime driver without executing source on the caller. `cancel` requests cancellation while queued or active. Drive reports completion only after native transport/callback acquisitions are released, actual cleanup ACKs are received, and workers/collector owner join. `take` transfers the owned result/error once after completion; misuse returns invalid-input5. `destroy(&op)` requests cancellation, joins, releases untaken content, and sets the pointer to nil; repeated destruction is harmless. Do not concurrently destroy/take the same handle. Submission samples any initial `gxc_options.canceled` flag; use operation cancellation for subsequent asynchronous requests. Blocking calls observe the supplied atomic flag until returning.

Callbacks run on native workers outside runtime locks with owned name/payload bytes and an atomic cancellation flag. Return malloc-owned `GXC_BYTES` responses:0 success,1 declared rejection,3 host fault. Release actual callback resources before returning. Callback state remains caller-owned until the blocking call returns or operation destruction completes. Callback cancellation is cooperative: a callback that ignores cancellation delays completion, with no fabricated cleanup. Synchronous reentry into source from a callback is rejected as host_fault3. Native workers retain only malloc data and numeric mailbox identities; source frames/collector values stay on the owner.

## Errors and native capabilities

Kinds are0 success,1 declared source error,2 source panic,3 native host fault,4 fatal source error,5 invalid native input,6 cancellation before source entry while queued. Declared SDK errors retain code/operation/server diagnostics, HTTP status, required obligations and cause category. Rejection paths return zero plaintext. Ordinary errors/panics/faults retire the invocation and allow subsequent calls. Native crashes, failed worker joins or missing cleanup ACK cannot be converted into safe successful cleanup; process survival after those faults is not promised.

The maintained OpenSSL adapter provides RSA2048 OAEP SHA1/MGF1SHA1, RS256, P256 ECDH32 and ES256 raw64-byte signatures, AES256-GCM, HKDF/HMAC/SHA256, constant-time MAC comparison, and cryptographic randomness. Crypto executes on the collector owner; cancellation is observed at source operation boundaries and cannot interrupt an in-progress OpenSSL call. Opaque native key IDs are owner-scoped; aliases close idempotently and retirement releases remaining keys. Strict bounded PEM uses OpenSSL3 [OSSL_DECODER](https://docs.openssl.org/3.0/man3/OSSL_DECODER_CTX_new_for_pkey/) and [EVP key validation](https://docs.openssl.org/3.0/man3/EVP_PKEY_check/), including effective private/public pair consistency. Public SPKI/PKCS1, private PKCS8/PKCS1 and certificates are accepted. Go1.25 normalizes encoded EC Q from D; this adapter rejects inconsistent encoded Q, accepts omitted Q, and documents that validation difference in native fixture evidence. No private-material erasure guarantee is made.

The libcurl adapter verifies TLS peer/hostname, permits bounded HTTP/HTTPS GET/POST, returns redirects without following them, and enforces request/response/header/timeout limits. Automatic gzip decoding applies when the first Accept-Encoding and Range values are absent or empty, bounds the decoded bytes, and removes Content-Encoding/Content-Length only for automatically decoded gzip. Explicit encoding or Range requests preserve the wire body and encoding headers. Response names are canonical and sorted; duplicate values retain server order. Invalid header names or control values are rejected before network contact. [curl_multi_poll](https://curl.se/libcurl/c/curl_multi_poll.html) supports cooperative cancellation; easy/multi/socket/header/body/input acquisitions are released before ACK. TLS verification uses [CURLOPT_SSL_VERIFYPEER](https://curl.se/libcurl/c/CURLOPT_SSL_VERIFYPEER.html) and hostname verification without global policy changes. Native input and detached output trees retain depth64/node1million structural budgets. Individual buffers and aggregate output allocations have no fixed byte-size cap; native allocation capacity, source descriptor ranges and overflow checks apply. Public crypto inputs are bounded64MiB, PEM64KiB, HKDF8160; SDK archive/protocol limits remain the shared source limits.

## Validation evidence

The delivery job's `no-preinit-consumer` (`tests/interop/generatedc/forward-consumer.c`) calls the public handle API against real BASIC KAS before any collector fixture setup. `http-contract.sh` exercises bounded actual local HTTP exchanges, gzip/explicit encoding/Range/header rejection and sorted duplicate headers. Named receipts are retained under ignored `.local/c-tdf-library` and `goalchemy/out/c-tdf-library`: actual independent crypto-free aggregate and accepted scalar importers; exact-int/binary values, cyclic-output rejection/recovery, forced GC, owned failure detachment; maintained native vectors and independent Go crypto proof; snapshot/queued-active cancellation/drive-wake-take-destroy; controlled HTTP/TLS/bounds/deadline/socket-release negatives; native archive/importer ASan+UBSan; C language/generic host/byte preservation; strict C conformance and generated-spec freshness. The target freeze records exact commands, statuses, hashes and original failed receipts. The real-KAS BASIC/EC/enforced-DPoP matrices cover all seven cases in both directions against pinned stock Go/Web references, with separately labeled stock Web nonce limitations and Bearer-format producer archives. Final seven-target clean-package/CI acceptance is a separate Phase7 gate.
