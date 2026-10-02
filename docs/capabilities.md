# Native capability boundary

Phase 2 supplies ordinary Go implementations in `goalchemy/lib/{crypto,encoding,http,clock}`. Shared SDK source imports those packages; JSON, JWT, TDF, policy and KAS wire construction stay in the SDK. Contracts are versioned declaration-backed `lib.*` entries under `goalchemy/specs/runtime/capabilities/`. `spec validate` compares signatures to actual Go declarations. `native_tests` points at separately run package tests, including independent vectors and host interoperability; it is traceability metadata, **not** a claim that `goalchemy test` runs these files. Empty catalog cases require a nonempty, existing native test reference, allowing nondeterministic key generation and wall time without fabricated expected values.

## Bytes, errors and ownership

Inputs are read-only for the operation's lifetime. Calls return fresh output bytes/strings; outputs never alias input storage. Nil bytes mean empty bytes. Shared callers must keep inputs unchanged until a possibly suspending call completes. Host async adapters must copy mutable host inputs before submitting work and retain inputs and key snapshots through completion/cancellation cleanup. No output plaintext is returned on authentication or unwrap failure. Invalid parameters/unsupported key types return an error and zero result. Correctly sized invalid signatures return `false,nil`; malformed signature sizes return an error. Error messages are diagnostic strings, not a portable dispatch API. Native context errors remain compatible with Go `errors.Is`.

Crypto inputs are at most 64 MiB per byte argument. Secure randomness takes 0..64 MiB. HKDF output takes 0..8160 bytes. PEM takes at most 64 KiB. Encoding accepts at most 64 MiB decoded bytes and `4*ceil(64MiB/3)` encoded bytes. These are host resource limits, not TDF segment defaults. The limits are checked before cryptographic output allocation. AES ciphertext has an additional 16-byte tag allowance, so maximum accepted plaintext remains decryptable.

## Algorithms and keys

| Capability | Exact profile |
| --- | --- |
| Random | Native cryptographically secure system randomness; no deterministic substitute |
| SHA256, HMACSHA256 | SHA-256 digest, HMAC-SHA256 over exact input bytes |
| HMACSHA256Verify | Added during Phase 3: verifies an exact 32-byte MAC using native constant-time comparison; empty/nil key and data are valid, malformed MAC/bounds return `false,error`, well-formed mismatch returns `false,nil` |
| HKDFSHA256 | RFC 5869 extract and expand; explicit salt/info, including empty; 0..255×32 output bytes |
| AES256GCMEncrypt/Decrypt | 32-byte key; 12-byte nonce; explicit AAD; output ciphertext followed by 16-byte tag; nonce is separate; no implicit framing; caller must ensure nonce uniqueness per key |
| RSAOAEPEncrypt/Decrypt | RSA-2048; OAEP SHA-1, MGF1 SHA-1, empty label; plaintext <=214 bytes, ciphertext exactly 256 bytes |
| RS256Sign/Verify | RSA-2048 PKCS1-v1.5 signature of SHA-256 message digest; 256-byte signature |
| ES256Sign/Verify | P-256 ECDSA over SHA-256 message digest; JOSE raw 32-byte R followed by 32-byte S; 64-byte signature; no DER signature |
| GenerateRSA2048, GenerateP256 | Independent native private key pairs |
| ECDH | P-256, returning raw 32-byte shared x-coordinate before a separate HKDF |
| ImportPEM | Exactly one unencrypted SPKI PUBLIC KEY, PKCS8 PRIVATE KEY, RSA PKCS1 public/private block, or CERTIFICATE; RSA-2048/P-256 only; certificate imports its public key without making a trust assertion; malformed/extra blocks or surrounding non-whitespace fail |
| Key.PublicPEM/PrivatePEM | SPKI PUBLIC KEY / unencrypted PKCS8 PRIVATE KEY; private export requires a private key |
| Key.PublicJWK | Six strings `[kty,crv,n,e,x,y]`; RSA integer fields use minimal unsigned big-endian; P-256 coordinates use fixed 32 bytes; unpadded base64url; no private data; shared code constructs JSON/thumbprints |
| Base64Encode/Decode | Canonical padded RFC 4648 standard alphabet |
| Base64URLEncode/Decode | Canonical unpadded RFC 4648 URL alphabet |
| clock.Unix | Real UTC Unix seconds, independent of scheduler virtual time; host adjustments can move it backward |

`*crypto.Key` is an opaque native handle. Shared code must retain/pass pointers and must **never copy the dereferenced Key value**. The current compiler does not enforce that value-copy restriction. It must never access native key fields. Pointer aliases share lifetime. `Close` accepts nil and is idempotent, drops retained native references and rejects subsequent key acquisitions. Host-managed storage cannot promise secure erasure. Native operations use a read lock; ECDH converts and retains the private native snapshot before acquiring the public key, allowing aliased inputs without a double-lock deadlock. Closing prevents new acquisitions but does not revoke snapshots already retained by an operation. Future adapters must preserve that rule and release retained operation resources before reporting cancellation completion. Public serialized key bytes already returned remain independently owned after Close.

All crypto operations, including Close, declare `suspension: may` to support asynchronous native adapters. Native Go retains its synchronous implementation; its runtime mappings complete the current scheduler task by assigning its result vector. An asynchronous Close rejects new key acquisitions and waits for retained operation snapshots to finish before reporting cleanup completion. Encoding does not suspend. HTTP may suspend. Wall-clock observation never suspends and is nondeterministic; JWT iat/exp must use this clock. Phase 5 verifies generated Go and TypeScript call/defer/go lowering after the Close declaration change, including successful deferred native calls and affected Go SDK/KAS preservation.

The shared TDF engine uses `HMACSHA256Verify` for policy, root and HS256 segment checks. Its native implementation calls `crypto/hmac.Equal`; shared code does not implement secret MAC comparison. The operation returns no buffers and preserves all input aliases. Native tests include RFC 4231, mismatch, malformed lengths, size bounds and mutation checks. The current catalog validates 13 types and 95 functions, with 588 generated spec files after Go, TypeScript, Java and C# capability/library mappings. The dedicated SDK `tests/sourcecheck/macverify` probe confirms explicit `GCE002` rejection on Python, Rust and C until each target implements the operation.

## HTTP, cancellation and deadlines

`http.Do(ctx, method, url, alternatingHeaders, requestBytes, maxResponseBytes, timeoutMillis)` returns `(status, alternatingResponseHeaders, responseBytes, error)`. Methods are GET/POST; GET requires an empty body for browser fetch portability. Header names/values alternate; duplicate values are preserved. Input order is retained per header name; response names are canonicalized and sorted, preserving duplicate value order rather than original inter-name wire order. Invalid header names, odd lengths, control characters except HTAB in values, framing/hop/proxy headers, URL credentials and URL fragments fail. HTTP(S) URLs are at most 8192 bytes. Request bytes and the caller's reply limit are 0..64 MiB. Headers are bounded to 64 KiB. `timeoutMillis` is mandatory, 1..300000, and measures real host elapsed time; an earlier native context deadline shortens it. Cancellation is propagated to the transport and reply reader.

TLS certificate/hostname verification stays enabled. Native requests use a dedicated bounded transport, host-configured proxy settings, bounded dial/TLS establishment and no persistent idle connection pool. Redirects are returned as ordinary responses and never followed, so origin credentials are not forwarded. Non-2xx statuses are ordinary responses; transport, timeout, cancellation and body-limit errors return status zero and no partial headers/body. Native Go follows transport automatic gzip decoding; the limit applies to decoded bytes. Automatically decoded content-encoding/length headers are removed by the transport. Explicit caller Accept-Encoding can select a different wire representation; shared protocol code should leave it unset when it expects decoded data.

Native `lib/context.Context` aliases Go's standard context and has real cancellation/deadlines. Phase 4 now implements generated Go HTTP using pending host operations, copied inputs, completion ownership and cancellation cleanup. HTTP-linked executable entries select a real monotonic scheduler clock; default entries and conformance fixtures retain virtual time. Source contexts bridge their earliest absolute deadline into native HTTP, and the mandatory request timeout starts before snapshot/submission. Context errors preserve standard Go sentinel identity and `errors.Is`. Only the owning driver resumes source tasks; shutdown cancels and drains workers. See [host-operation design and verification](../../goalchemy/docs/host-operations.md). The [TypeScript host lifecycle](../../goalchemy/docs/typescript-host-operations.md) and [production library/adapters](generated-typescript-library.md) pass Node and actual browser acceptance. Native fetch has declared exposed-header, decoded-body, URL and redirect observations described in the [transport design](../../goalchemy/docs/typescript-library-boundary.md). The [Java worker/mailbox lifecycle](../../goalchemy/docs/java-host-operations.md) and [production JAR/adapters](generated-java-library.md) are accepted with JDK 21 native HTTP/JCA plus pinned BC 1.86 for HKDF and omitted-Q P-256 imports. Actual native consumer and real KAS evidence verify cancellation, bounds, TLS and cleanup. The [C# Task/mailbox lifecycle](../../goalchemy/docs/csharp-host-operations.md) and [production DLL/adapters](generated-csharp-library.md) are accepted with .NET 8 built-in crypto/HTTP, native token providers, cancellable owned calls and real-KAS evidence. The [Python generic owner/Condition lifecycle](../../goalchemy/docs/python-host-operations.md) has also passed CPython/emitted acceptance; production Python adapters remain unavailable. The [Rust generic owned-wire/mailbox lifecycle](../../goalchemy/docs/rust-host-operations.md) has passed native/emitted acceptance; production Rust adapters remain unavailable. The [C generic native-wire/mailbox lifecycle](../../goalchemy/docs/c-host-operations.md) has passed collector, sanitizer and emitted-program acceptance. Production C adapters and all-target TDF3 library APIs remain pending.

Go-emitted crypto, encoding, wall clock and HTTP are implemented and bundled into the generated module with only standard-library imports. The remaining Python, Rust and C production capabilities fail before emission with `GCE002: target … does not implement runtime function lib.…`. Type-only opaque references on those remaining targets (the `./handles` probe) also fail before target lowering, even without a capability call. Later target work must implement the contract before accepting programs; no mock cryptography or fake successful I/O is emitted.

## Generated Go callbacks and libraries

The accepted [generated Go SDK](generated-go-library.md) uses the bounded
[callback capability](../../goalchemy/lib/callback/callback.go) for host token
providers. Its request explicitly suspends source execution. Registries belong
to one call; terminal settlement acknowledges provider-resource release, and
cancellation waits for that acknowledgment. Typed token fields retain exact
int64 expiry, scheme and DPoP binding. The library copies results/errors before
resetting source globals, retires all owned key imports/generations and returns
source-panic/host-fault categories without exiting the native importing process.
The [TypeScript SDK](generated-typescript-library.md) adds a per-call async token
provider and the same owned callback/key lifecycle in Node and browsers. The
[Java SDK](generated-java-library.md) provides owned native token callbacks,
serialized cancellable operations and copied typed results/errors after actual
resource retirement. The [C# SDK](generated-csharp-library.md) provides native
token callbacks and cancellable Tasks, publishing copied results after source
and acquired-resource retirement. Python, Rust and C callback mappings and
production capabilities remain gated until their SDK implementations pass the same requirements.

## Verification

Native/compiler commands use the cached Go1.25.14 toolchain explicitly. The existing frontend loader keeps its pinned Go1.27.1 reference toolchain and Go1.25 source profile; `GOTOOLCHAIN=go1.27.1 go version` succeeds in this environment. The declaration resolver reads standard package source, avoiding stale export-data/toolchain mismatches when an independently built compiler runs under a different ambient GOROOT.

From `goalchemy/`:

```sh
GOTOOLCHAIN=go1.25.14 go test -race ./lib/crypto ./lib/encoding ./lib/http ./lib/clock
GOTOOLCHAIN=go1.25.14 go test ./internal/catalog ./internal/contracts ./internal/driver ./internal/frontend ./internal/link ./internal/subset ./internal/lower ./internal/emit/golang ./internal/specgen ./targets/go/runtime
GOTOOLCHAIN=go1.25.14 go build -o out/goalchemy ./cmd/goalchemy
GOTOOLCHAIN=go1.25.14 out/goalchemy spec validate
GOTOOLCHAIN=go1.25.14 out/goalchemy spec generate -check
GOTOOLCHAIN=go1.25.14 out/goalchemy test -target go
```

From `sdk/tests/capabilities/`:

```sh
GOTOOLCHAIN=go1.25.14 go run ./primitives
GOTOOLCHAIN=go1.25.14 go run ./http
GOTOOLCHAIN=go1.25.14 ../../../goalchemy/out/goalchemy check -gate cooperative ./primitives ./http ./handles
GOTOOLCHAIN=go1.25.14 ../../../goalchemy/out/goalchemy compile -gate cooperative -target go -out ../../.local/capabilities/go ./primitives
```

Run `GOTOOLCHAIN=go1.25.14 go run .` inside `sdk/.local/capabilities/go/`. `./http` requires the local platform for GET health. The test-only native `go run ./live` reads `.local/interop/small.go.tdf`'s allowed policy, obtains the real issuer Bearer token, imports RSA key r1, wraps a fresh share using OAEP and exact HMAC binding, signs an SRT with a separate P-256 auth key and submits real KAS Rewrap. It validates response IDs/status and decrypts with an independent RSA session key to compare 32 bytes. The optional argument selects another reference policy fixture. Local development credentials match the Phase 1 stack; no tokens/keys are printed or persisted. This validates the native primitive boundary against real KAS, **not** shared SDK/TDF compatibility, EC rewrap, or enforced DPoP/SRT verification. Bearer KAS skips SRT signature verification; independent primitive signature tests cover its encoding.

The Go runtime conformance suite now passes 180/180 cases, including the new invalid HTTP boundary case; capabilities also require the separate package and native/emitted probes above. Package tests include published SHA/HMAC/HKDF/NIST AES vectors; fixed P-256 ECDH scalars; bidirectional standard-library RSA/OAEP/signature/PEM checks; certificate/PKCS1 imports; strict base64 cases; size/parameter/tamper/lifetime rejection; and real local HTTP server status/headers, redirect, decoded body/header bounds, cancellation, real deadlines and untrusted TLS rejection. Root additionally verified an emitted Go SDK executable through real KAS: empty/binary files consumed by both pinned CLIs, both pinned producers decrypted by the emitted SDK, and two own payload/metadata round trips. This RSA/Bearer prerequisite does not establish the full profile matrix, library exports, browser support or all-target parity.
