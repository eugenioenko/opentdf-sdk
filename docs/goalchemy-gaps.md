# Goalchemy gaps

The reference SDK's non-OpenTDF imports, from
`go list -f '{{join .Imports "\n"}}' ./...` in `platform/sdk` (every package,
including `audit`, `codegen`, `internal/zipstream` and the six in-scope
subpackages) and in `platform/lib/ocrypto`, at the pinned platform revision
`4d9e555bddf0`. Each import is mapped to a Goalchemy package it already has
(`std/` pure subset Go, `lib/` native capability; see
`../goalchemy/docs/usage.md` and `docs/library.md` at v0.6.0), to a
subset-Go rewrite inside this SDK, or to a not-planned entry, with the phase of
[full-sdk-plan.md](full-sdk-plan.md) that first needs it. Real gaps have a
Goalchemy issue; the issues are linked from [parity-matrix.md](parity-matrix.md)
through this file.

## Issues opened

| Issue | Gap | Needed by |
| --- | --- | --- |
| [goalchemy#71](https://github.com/eugenioenko/goalchemy/issues/71) | `std/net/url`: `Parse`, `ParseRequestURI`, `URL.Hostname/Port/String/JoinPath`, `JoinPath`, `PathEscape`, `QueryEscape`, `Values`, `JoinHostPort` | Phase 2 (KAS origin, allowlist normalisation, discovery URLs, DPoP `htu`); released in v0.6.3 as `std/net/url` and `std/net`, pinned |
| [goalchemy#72](https://github.com/eugenioenko/goalchemy/issues/72) | `std/encoding/json/jcs`: RFC 8785 `Transform` | Phase 5 (assertion hashes) |
| [goalchemy#73](https://github.com/eugenioenko/goalchemy/issues/73) | `std/os` and `lib/os` file handles: `Open`, `Create`, `OpenFile`, `CreateTemp`, `Stat`, `Remove`, `Rename`, `*File` as `io.Reader/ReaderAt/Writer/Seeker/Closer`; `std/path/filepath` `Dir/Base/Join/Clean` | Phase 7 (`DecryptFile`, streaming file CLI, payloads above `MaxFileBytes`) |
| [goalchemy#74](https://github.com/eugenioenko/goalchemy/issues/74) | `lib/runtime`: `OS`, `Arch`, `Version` | Phase 5 (system-metadata assertion) |
| [goalchemy#75](https://github.com/eugenioenko/goalchemy/issues/75) | `std/encoding/json/schema`: bounded draft-07 subset validator (`type`, `properties`, `required`, `items`, `enum`, `oneOf`) | Phase 6 (`WithSchemaValidation`) |
| [goalchemy#76](https://github.com/eugenioenko/goalchemy/issues/76) | `lib/crypto.ImportCertificatePublicKey`: public key of an X.509 certificate given as DER (`x5c[0]`) | Phase 5 (`Assertion.Verify` with an `x5c` header) |

Not opened, with the reason:

- `regexp`: the reference uses eight anchored FQN patterns in `sdk/granter.go`
  (`^(https?://[\w./-]+)/attr/([^/\s]*)$` and the value forms) and one in
  `experimental/tdf/keysplit/boolean_expression.go`. They are one small
  hand-written parser in subset Go, which architecture decision 5 prefers; a
  `std/regexp` issue is opened only if that parser becomes unreadable.
- `crypto/x509`, `encoding/pem` for transport: `WithTLSCredentials` takes a
  `tls.Config`, which maps to PEM client certificate, key and root CAs that
  `lib/http.DoWith` already accepts (`http.ClientCert`, `http.ClientKey`,
  `http.RootCAs`); `DoCertExchange` sends an ordinary token request over that
  mTLS client. PEM blocks for keys are imported by `lib/crypto.ImportPEM`. The
  only certificate the SDK parses itself is an assertion's `x5c` header, which
  is goalchemy#76.
- `math/big`, `encoding/asn1`, `crypto/elliptic`: used inside `ocrypto` for
  EC point compression and the PQ ASN.1 structures; `lib/crypto` owns key
  formats natively and the PQ structures are not planned.

## Import inventory

Phase is the first phase that needs the mapping; "now" means the current
shared source already uses it. Reference packages: root = `platform/sdk`,
and the subpackage or `ocrypto` where noted.

### Go standard library

| Import | Reference use | Mapping | Phase |
| --- | --- | --- | --- |
| `bytes` | buffers, `bytes.Equal` | `std/bytes` | now |
| `context` | cancellation and deadlines on every call | `lib/context` | now |
| `crypto`, `crypto/rand`, `crypto/rsa`, `crypto/ecdsa`, `crypto/ecdh`, `crypto/aes`, `crypto/cipher`, `crypto/hmac`, `crypto/sha1`, `crypto/sha256` (root, `auth`, `keysplit`, `ocrypto`) | DEK and session keys, RSA-OAEP (SHA-1), AES-256-GCM, HMAC-SHA256 integrity, ECDH plus HKDF for `ec-wrapped`, request and DPoP signing | `lib/crypto` (`Random`, `GenerateRSA2048/4096`, `GenerateP256/384/521`, `RSAOAEPEncrypt/Decrypt`, `AES256GCMEncrypt/Decrypt`, `HMACSHA256`, `ECDH`, `RS256..RS512`, `ES256..ES512`, `SHA256`) | now (RSA-2048, P-256); 4 (RSA-4096, P-384, P-521); 8 (RS/ES 384 and 512) |
| `crypto/sha3`, `crypto/mlkem` (`ocrypto`) | ML-KEM and X-Wing KEMs | not planned: post-quantum KAOs (goalchemy #21 closed) | n/a |
| `crypto/subtle` (root) | `XORBytes` to reconstruct the DEK from splits, `ConstantTimeCompare` to verify it | subset-Go byte loops (`sdk/tdf` key splitter); constant-time compare as an OR-accumulated XOR | 4 |
| `crypto/tls` (root, `auth`, `httputil`) | TLS 1.2 minimum, skip-verify, client certificates, custom CAs | `lib/http.DoWith` options (`InsecureSkipVerify`, `ClientCert`, `ClientKey`, `RootCAs`, `UnixSocket`); hosts set the TLS floor | now (skip-verify); 8 (mTLS, custom CA, Unix socket) |
| `crypto/x509`, `encoding/pem`, `encoding/asn1`, `math/big`, `crypto/elliptic` (`ocrypto`) | key encodings, EC point compression, PQ ASN.1 | inside `lib/crypto` natively (`ImportPEM`, `Key.PublicPEM/PrivatePEM/PublicJWK`); PQ not planned | now |
| `embed` (root) | the two manifest schemas | subset-Go string constants in `sdk/tdf` | 6 |
| `encoding/base64` | manifest and protocol fields | `lib/encoding` (`Base64Encode/Decode`, URL forms) | now |
| `encoding/binary` (root, `internal/zipstream`) | ZIP headers | `std/encoding/binary` | now (byte writer); 7 (streaming zip) |
| `encoding/hex` | legacy 4.2.2 integrity values | `std/encoding/hex` | 6 |
| `encoding/json` (root, `auth/oauth`) | manifest, policy, assertions, OAuth responses | `std/encoding/json` (typed) and `std/encoding/jsonvalue` (policy binding string-or-object, JWT claims) | now |
| `errors`, `fmt` | sentinel values, `%w` wrapping, `errors.Is/As/Join` | `std/errors`, `std/fmt` | 2 (error model) |
| `hash/crc32` (root, `internal/zipstream`) | ZIP entry CRCs | `lib/checksum.CRC32IEEE` | now |
| `io` | readers, writers, `ReadSeeker`, `ReaderAt`, `WriterTo` | `std/io` and `std/io/hostio` host streams | 2 (handles); 7 (streams) |
| `log/slog` (root, `auth`, `auth/oauth`, `keysplit`, `experimental/tdf`) | SDK diagnostics, `WithLogger`, `SDK.Logger` | `std/log/slog` with `lib/log` host sink; `Options.LogLevel` | 2 (`WithLogger`); 8 (sink idioms per target) |
| `log` (`codegen`) | generator messages | not planned: build tooling | n/a |
| `math` (root, `internal/zipstream`) | `MaxInt32`/`MaxInt64` bounds | subset-Go constants | 6, 7 |
| `net` (root) | `JoinHostPort` in `parseBaseURL` | `std/net` (goalchemy#71, v0.6.3) | now |
| `net/http` (root, `auth`, `auth/oauth`, `httputil`) | the HTTP client, `RoundTripper` for DPoP, interceptors | `lib/http` (`Do`, `DoWith`, `Open` streamed exchange), `std/http` bodies, `std/connect` for RPC; the DPoP transport becomes the SDK's own request layer with no `http.Client` exposure | now (bounded); 3 (Connect); 7 (streamed bodies) |
| `net/url` (root, `auth`, `auth/oauth`, `keysplit`) | see goalchemy#71 | `std/net/url` (goalchemy#71, v0.6.3) | now |
| `os` (root, `codegen/runner`) | `DecryptFile` temp file and rename; generator output | `std/os.ReadFile/WriteFile` today; goalchemy#73 for handles | 7 |
| `path`, `path/filepath` (root, `codegen`) | `filepath.Dir(outputPath)` for the temp file | `std/path/filepath` subset in goalchemy#73 | 7 |
| `regexp` (root, `keysplit`) | attribute FQN validation and splitting | subset-Go FQN parser in `sdk/tdf` | 4 |
| `runtime` (root, `codegen`) | `GOOS`, `GOARCH`, `Version()` in the system-metadata assertion | goalchemy#74 | 5 |
| `slices` (root) | `slices.Clone` of assertion and attribute lists, `slices.Sort` of segment indices in the chunked writer, `slices.Contains` for the DPoP algorithm allow-list | subset-Go copy loops and `std/sort` | 5, 8, 10 |
| `sort` (root, `keysplit`, `internal/zipstream`) | deterministic KAO and split ordering | `std/sort` | 4 |
| `strconv`, `strings` | parsing, `EqualFold` for algorithm names | `std/strconv`, `std/strings` | now |
| `sync` (root, `auth`, `internal/zipstream`), `sync/atomic` (`experimental/tdf`) | key cache and nonce cache locks, token mutex, `finalized atomic.Bool` | `lib/sync.Mutex` under the cooperative gate; a mutex-guarded bool for `atomic.Bool` | now; 10 |
| `time` (root, `auth`, `auth/oauth`, `httputil`, `internal/zipstream`) | token expiry, JWT `iat`/`exp`, RFC 3339 creation date, ZIP timestamps | `std/time` (`Now`, `Format(RFC3339)`, `Duration`) and `lib/clock` | now |
| `go/ast`, `golang.org/x/text/cases`, `golang.org/x/text/language`, `golang.org/x/tools/go/packages` (`codegen/runner`) | service wrapper generation | not planned: build tooling; `protoc-gen-goalchemy` generates the clients instead | n/a |

### Third-party modules

| Import | Reference use | Mapping | Phase |
| --- | --- | --- | --- |
| `connectrpc.com/connect` (root, `audit`, `auth`, `sdkconnect`) | KAS `Rewrap`/`PublicKey`, policy and authorization clients, error codes | `std/connect` (`CallUnary`, `Code`, `Error`, `CodeOf`) with `protoc-gen-goalchemy` clients | 3 |
| `github.com/Masterminds/semver/v3` (root) | `isLessThanSemver` for `WithTargetMode` (4.2.2 versus 4.3.0) | subset-Go three-part version compare in `sdk/tdf` | 6 |
| `github.com/google/uuid` (root, `audit`, `auth`, `auth/oauth`, `keysplit`) | policy UUID (`uuid.NewUUID`, version 1, `tdf.go:814`), split ids and JWT `jti` (`uuid.New`, version 4) | UUIDv4 from `lib/crypto.Random` (already `src/tdf/engine.go`); the policy UUID is v4 as in the web SDK, which no reader checks | now |
| `github.com/gowebpki/jcs` (root) | RFC 8785 for assertion hashes | goalchemy#72 | 5 |
| `github.com/lestrrat-go/jwx/v2/jwa`, `jwk`, `jws`, `jwt` (root, `auth`, `auth/oauth`) | JWT signing of request tokens and DPoP proofs, JWK thumbprints, caller-supplied JWKs, token parsing | subset Go already in `src/protocol.go` and `src/dpop.go` (`signJWT`, `authJWK`, `authThumbprint`) over `lib/crypto`; extend for RS/ES 384 and 512 and PEM-supplied keys; JWK input converted to PEM by the host adapter (api-spec) | now; 8 |
| `github.com/lestrrat-go/jwx/v2/cert` (root) | `Assertion.Verify` takes the verification key from the JWS `x5c` header when no key is configured (`assertion.go:146-152`) | goalchemy#76; the base64 and PEM decoding stay in subset Go | 5 |
| `github.com/xeipuuv/gojsonschema` (root) | `WithSchemaValidation` | goalchemy#75 | 6 |
| `github.com/cloudflare/circl/kem/xwing` (`ocrypto`) | X-Wing hybrid KEM | not planned: post-quantum KAOs (goalchemy #21 closed) | n/a |
| `golang.org/x/crypto/hkdf` (`ocrypto`) | EC session key derivation (empty info, versioned salt) | `lib/crypto.HKDFSHA256` | now |
| `golang.org/x/oauth2` (root) | `oauth2.TokenSource` for `WithOAuthAccessTokenSource`, `ReuseTokenSource` caching | host callback (`lib/callback`) named by `Options.TokenProviderName`; caching in subset Go | 8 |
| `google.golang.org/grpc/codes` (root) | `codes.InvalidArgument.String()` and `codes.PermissionDenied.String()` as substrings when classifying per-KAO errors | subset-Go constants `InvalidArgument`, `PermissionDenied`; the KAS strings do not change | 2 |
| `google.golang.org/grpc/health/grpc_health_v1` (root) | `IsHealthy` calls `grpc.health.v1.Health/Check` | `std/connect` unary call with a generated `HealthCheckRequest/Response` | 2 |
| `google.golang.org/protobuf/encoding/protojson`, `types/known/structpb` (root) | KAS request bodies, obligation metadata in rewrap responses (`structpb.Value`) | `std/encoding/protojson` (`Struct`, `Value`, `ListValue` codecs) with `protoc-gen-goalchemy` messages | 3 (generated KAS client); 9 (obligations metadata) |

### `platform/lib/ocrypto`

`ocrypto` itself is replaced by `lib/crypto` plus subset-Go wrappers:
`KeyType` strings (`rsa:2048`, `rsa:4096`, `ec:secp256r1`, `ec:secp384r1`,
`ec:secp521r1`; `mlkem:768` and `mlkem:1024` are rejected), PEM and JWK
handling through `lib/crypto.Key`, and the HKDF salt and info the reference
uses for `ec-wrapped` keys (recorded in [tdf-poc.md](tdf-poc.md)). Its
`BENCHMARK_REPORT.md`, hybrid and PQ files are not ported.
