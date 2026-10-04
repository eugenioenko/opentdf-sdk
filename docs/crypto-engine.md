# Bounded native TDF3 encryption and integrity engine

[engine.go](../src/tdf/engine.go) implements the modern single-share encryption and integrity milestone of Phase 3. It uses shared subset Go and declared Goalchemy crypto/encoding capabilities. [manifest and ZIP validation](format.md) remains the format boundary. The engine passes the compiler's cooperative gate and runs as ordinary Go and emitted Go. This milestone supplies native cryptographic interoperability; the shared SDK authentication, discovery and KAS rewrap client remain subsequent work.

`Encrypt(data, EncryptConfig)` returns a complete in-memory TDF3 archive. Callers supply a trusted KAS key handle, URL and optional kid. Wrapping algorithms are `rsa:2048` (default) and `ec:secp256r1`. The URL is bounded, nonempty manifest configuration; the engine performs no HTTP request. The later SDK client must validate configured/discovered KAS destinations before sending credentials. Opaque `*crypto.Key` handles remain pointers and remain caller-owned. Nil, closed and wrong-type keys fail through the native capabilities.

`DecryptWithPayloadKey(archive, recoveredKey)` accepts an independently recovered 32-byte payload key and returns payload, metadata and the validated manifest. The result is completely zero on any failure, including a failure in the last segment or encrypted metadata. Key recovery and policy authorization belong to the KAS client. Possession of a key is sufficient for this engine operation and must not be presented as a new authorization check.

`NewPolicy(attributes, dissem)` creates a secure random UUIDv4 and independently owned slices. Encryption uses that policy unless `PolicyBase64` supplies an explicit, validated modern policy. Supplied policy base64 is preserved exactly, including JSON whitespace encoded inside it; combining that policy with generated attributes/dissemination is an error.

## Protocol and resource profile

The implementation uses these pinned parameters:

- Payload, metadata and EC wrap frames are `nonce12 || AES-256-GCM ciphertext || tag16`, with no AAD. Keys and nonces use native secure randomness. RSA wrapping uses OAEP SHA-1, MGF1 SHA-1 and an empty label. EC wrapping uses a new P-256 ephemeral key, ECDH, HKDF-SHA256 with salt `SHA256("TDF")`, empty info and 32 output bytes. The ephemeral public key is SPKI PEM in the KAO; the private handle is closed on every exit. See [Go wrapping and metadata](../../platform/sdk/tdf.go), [Go AES](../../platform/lib/ocrypto/aes_gcm.go) and [Web crypto](../../web-sdk/lib/tdf3/src/crypto/core/ec.ts).
- Segment integrity defaults to GMAC: the trailing authenticated AES-GCM tag. Optional HS256 uses HMAC-SHA256 over the entire frame, including nonce. Root HS256 authenticates the ordered concatenation of decoded segment hashes. These values use standard padded base64. Policy binding is base64 of lowercase ASCII hex of HMAC-SHA256 over the **exact policy base64 bytes**. See [Go integrity helpers](../../platform/sdk/tdf.go) and [Web key access](../../web-sdk/lib/tdf3/src/models/key-access.ts).
- The writer follows Go's 2 MiB plaintext default. An explicit size clamps to 16 KiB–4 MiB, including explicit zero/negative values. `HasSegmentSize` distinguishes zero explicitly supplied from an omitted zero; a nonzero `SegmentSize` also selects an explicit size. The writer emits one authenticated empty frame for empty input. The reader also accepts Web's zero-segment empty form and resolves independently omitted segment sizes. See [Go configuration](../../platform/sdk/tdf_config.go), [Go empty handling](../../platform/sdk/tdf.go) and [Web writer](../../web-sdk/lib/tdf3/src/tdf.ts).
- Metadata is optional. `IncludeMetadata` permits an explicitly encrypted empty value; nonempty metadata enables it automatically. The KAO records base64(JSON `{ciphertext,iv}`); `ciphertext` contains the whole frame and `iv` repeats its nonce. The reader validates matching framing/IV and authenticates the frame. See [Go metadata](../../platform/sdk/tdf.go) and [Web metadata](../../web-sdk/lib/tdf3/src/models/encryption-information.ts).
- Encrypted payload is limited to 64 MiB, manifest to 10 MiB, archive to 128 MiB, segments to 10,000 and metadata plaintext to 512 KiB. Encryption checks frame overhead before encrypting, so a maximum-size plaintext request can fail the encrypted-payload limit. Other JSON/policy/archive bounds apply as documented in [format.md](format.md). Inputs stay read-only through completion; returned bytes are independently owned.

Decryption validates the complete modern profile and exact segment/payload sizes before slicing; verifies the policy binding and root; verifies each segment hash and AES-GCM authentication; and authenticates metadata before returning a result. GMAC comparison examines public manifest/tag bytes after root verification, followed by native AES-GCM authentication. Secret HMAC verification uses the declared [`HMACSHA256Verify`](../../goalchemy/lib/crypto/crypto.go) capability, implemented with native `crypto/hmac.Equal`. Its contract accepts nil/empty byte key/data, requires a 32-byte MAC, returns `false,nil` for a well-formed mismatch and `false,error` for invalid bounds/size, and retains read-only inputs across possible suspension. See [contract](../../goalchemy/specs/runtime/capabilities/crypto_hmac_sha256_verify.yaml) and [native vectors](../../goalchemy/lib/crypto/hmac_verify_test.go).

The accepted schema is 4.3.0, one RSA/EC wrapped KAO, AES-256-GCM, HS256 root and GMAC/HS256 segments. Unsupported versions, algorithms, assertions, unknown fields and multiple KAS/shares fail explicitly. Streaming, splitting, legacy encodings, other curves/RSA sizes, KEM/hybrid schemes, assertions, obligations, OAuth, discovery, request signing, real rewrap and DPoP remain mandatory later parity work. Non-Go capability implementations, host scheduling, exported library APIs and all generated target SDKs remain separate requirements. EC wrapping has independent native cryptographic evidence here; real EC KAS interoperability remains subsequent work.

## Verification and reproducible commands

[Native engine tests](../src/tdf/engine_test.go) independently unwrap RSA with standard-library OAEP SHA-1 and EC with standard-library ECDH/HKDF/AES, decrypt frames with native AES-GCM, and recompute policy, segment and root MACs with native HMAC. They include the published NIST AES-256-GCM empty-message vector, Go/Web empty forms, binary data, exact/multiple boundaries, independently omitted defaults/algorithm fallback, metadata, explicit/omitted/clamped sizes, UUIDv4, exact policy preservation, nil/closed/wrong-type keys and malformed sizes/archives. Tampering tests rewrite ZIP CRCs so cryptographic rejection, including last-segment/metadata failures with zero output, is exercised. The bounded fuzz target includes a deterministic valid archive/key seed.

From `sdk/`:

```sh
GOTOOLCHAIN=go1.25.14 go test -race ./src/tdf/...
GOTOOLCHAIN=go1.25.14 go test ./src/tdf -run '^$' -fuzz '^FuzzEngineDecrypt$' -fuzztime=5s -parallel=2
GOTOOLCHAIN=go1.25.14 ../goalchemy/out/goalchemy check -gate cooperative ./src/tdf ./src/tdf/json ./tests/sourcecheck/engine ./tests/sourcecheck/macverify
GOTOOLCHAIN=go1.25.14 ../goalchemy/out/goalchemy compile -gate cooperative -target go -out .local/engine/go ./tests/sourcecheck/engine
(cd .local/engine/go && GOTOOLCHAIN=go1.25.14 go run .)
make platform-ready
GOTOOLCHAIN=go1.25.14 GOCACHE="$PWD/.local/go-build-cache" GOMODCACHE="$PWD/.local/go-mod-cache" go -C tests/interop/engine run .
```

The [native-only interop module](../tests/interop/engine/main.go) keeps pinned reference SDK imports out of the shared module. It verifies tracked-clean pinned reference revisions, fetches public RSA key r1, creates six engine files (small, empty, exact 16 KiB, multiple segments, HS256 and explicit zero size), and compares bytes after both stock CLI consumers decrypt through real KAS. It removes prior consumer outputs and bounds subprocesses to 45 seconds. The pinned Go reader independently recovers keys in memory and compares actual metadata bytes; keys/tokens are never printed or persisted. Seven existing Go/Web fixtures and a freshly created Go multi-segment/metadata file decrypt in the engine. Generated archives, plaintext comparison files and a nonsecret results summary stay under ignored `.local/interop-engine/`. Web CLI payload success is not a claim about a Web metadata accessor.

Observed outcomes: all six engine files decrypted with both reference CLIs; all eight independent reference reads passed, along with engine reads using reference-recovered keys. Native race, cooperative gate and emitted-Go RSA/P256/negative probe passed. The bounded 5-second fuzz run completed 6,629 executions without a failure. The real platform profile is RSA r1, ordinary client-credentials Bearer, EC disabled and DPoP enforcement disabled. The reference SDK owns authentication/rewrap in this runner, so these results establish the engine milestone rather than a working shared SDK KAS client.

From `goalchemy/` after changing the capability:

```sh
GOTOOLCHAIN=go1.25.14 go test -race ./lib/crypto ./lib/encoding ./lib/http ./lib/clock
GOTOOLCHAIN=go1.25.14 go test ./internal/catalog ./internal/contracts ./internal/driver ./internal/frontend ./internal/link ./internal/subset ./internal/lower ./internal/emit/golang ./internal/specgen ./targets/go/runtime
GOTOOLCHAIN=go1.25.14 go build -o out/goalchemy ./cmd/goalchemy
GOTOOLCHAIN=go1.25.14 out/goalchemy spec validate
GOTOOLCHAIN=go1.25.14 out/goalchemy spec generate -check
GOTOOLCHAIN=go1.25.14 out/goalchemy test -target go
```

The observed catalog validates 13 type contracts, 94 function contracts and seven targets; all 502 generated files are current; the Go runtime suite passes 179/179 existing contract cases. Capability native tests and the emitted probe run separately. Rebuild the compiler after catalog changes: a binary embeds its build-time catalog. The local SDK binary was refreshed from the rebuilt compiler; `GOALCHEMY_ROOT` can also select the authoritative checkout. Compiling `tests/sourcecheck/macverify` for `typescript` correctly fails with `GCE002` naming `lib.crypto.hmac_sha256_verify`; non-Go implementations are unavailable. An initial probe used the unsupported target shorthand `ts` and returned `GCE001`; the documented target name is `typescript`.
