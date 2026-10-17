# SDK API specification

This is the contract every phase of [full-sdk-plan.md](full-sdk-plan.md)
implements. The shared source defines it once in the Goalchemy subset of Go;
each target exposes it in its own idiom following the rules in
[Target idioms](#target-idioms). The reference is `../platform/sdk` at the
pinned revision; every type and method below names the reference symbol it
corresponds to.

Design rules, in priority order:

1. **Composable.** The SDK is a small set of handle types with plain data in
   and out. No global state, no hidden singletons, no ambient configuration.
   A wrapper in any language can present the whole surface by delegating to
   the handle methods, and a different implementation can satisfy the same
   interface. The shared source declares `Client` and `TDFReader` interfaces
   that `*SDK` and `*Reader` implement.
2. **Zero value is the reference default.** Every option field's zero value
   means "what the reference does with no option". Flags that the reference
   enables by default are named as their negation (`DisableAutoconfigure`),
   so an empty options value never needs a "has" flag.
3. **Reference names, target casing.** Types, methods, option fields and
   manifest fields keep the reference's names; each target applies its own
   casing. Manifest and policy JSON tags are byte-identical to the reference.
4. **Errors are data.** One error type with a stable `Kind`, the reference's
   message text, and the service details hosts need. Sentinels exist for Go
   `errors.Is`; every other target switches on `Kind`.
5. **Streams at the boundary, bytes for convenience.** Every operation that
   moves a payload exists in a stream form; the byte form is a thin wrapper.
6. **Nothing Go-specific crosses the boundary.** `context.Context` becomes
   the target's cancellation idiom; `io.Reader` becomes the target's native
   stream; functional options exist only in the Go host adapter.

## Package layout

Shared source, `src/` (module `opentdf-local/sdk`, imported as
`opentdf-local/sdk/src`):

| Package | Contents | Reference file(s) |
| --- | --- | --- |
| `sdk` | `New`, `Options`, `SDK`, `Client`, errors, discovery, auth, KAS client | `sdk.go`, `options.go`, `kas_client.go`, `discovery.go`, `basekey.go`, `platformconfig.go`, `*errors.go` |
| `sdk/tdf` | manifest types, writer, reader, integrity, policy binding, assertions, key splitting, granter, resource locator, zip | `tdf.go`, `tdf_config.go`, `manifest.go`, `assertion.go`, `key_splitter.go`, `granter.go`, `resource_locator.go`, `internal/zipstream` |
| `sdk/tdf/chunked` | out-of-order segment writer | `chunked_writer.go`, `chunked_options.go` |
| `sdk/bulk` | bulk decryption | `bulk.go` |
| `sdk/auth` | token sources, DPoP proofs, signed request tokens | `auth/`, `auth/oauth/`, `dpop_key.go`, `idp_*.go` |
| `sdk/protocol/...` | generated messages and Connect clients from the `../platform/service` protos | `sdkconnect/`, `protocol/go` |
| `sdk/experimental/tdf` | experimental writer and reader | `experimental/tdf/` |

The export root for Goalchemy is `src/library`, which re-exports the public
handles and functions below and nothing else. Hand-written host adapters live
in `src/hosts/<target>`.

## Core types

### Options (reference: `Option` and `options.go`)

```go
type Options struct {
	// Authentication. Exactly one of ClientCredentials, TokenExchange,
	// CertExchange or TokenProviderName is set; none means anonymous.
	ClientID          string
	ClientSecret      string
	Scopes            []string
	TokenEndpoint     string   // WithTokenEndpoint
	TokenExchange     *TokenExchange
	CertExchange      *CertExchange
	TokenProviderName string   // host callback; see Token sources

	// Platform configuration. Empty means fetch /.well-known.
	PlatformConfiguration map[string]string // WithPlatformConfiguration
	ValidateConnection    bool               // WithConnectionValidation

	// Transport.
	InsecurePlaintext   bool     // WithInsecurePlaintextConn
	InsecureSkipVerify  bool     // WithInsecureSkipVerifyConn
	TLS                 *TLSOptions // WithTLSCredentials: ClientCertPEM, ClientKeyPEM, RootCAsPEM, Audience
	UnixSocket          string   // WithIPC equivalent on targets with Unix sockets
	TimeoutMillis       int64    // per request; 0 = reference default
	// Service endpoints through the SDK's transport: CoreEndpoint for every
	// service, the others for their own services (see Service clients).
	CoreEndpoint             string // WithCustomCoreConnection
	PolicyEndpoint           string // WithCustomPolicyConnection
	AuthorizationEndpoint    string // WithCustomAuthorizationConnection
	EntityResolutionEndpoint string // WithCustomEntityResolutionConnection
	WellknownEndpoint        string // WithCustomWellknownConnection

	// Keys.
	SessionEncryptionRSAPEM string           // WithSessionEncryptionRSA
	SessionSignerRSAPEM     string           // WithSessionSignerRSA
	DPoPAlgorithm           SigningAlgorithm // WithDPoPAlgorithm; "" = ES256 with an ephemeral P-256 key (reference dpop_key.go)
	DPoPKeyPEM              string           // WithDPoPKeyPEM, WithDPoPJWK (JWK converted by the host adapter)
	NoKIDInKAO              bool             // WithNoKIDInKAO

	// Policy.
	FulfillableObligationFQNs []string // WithFulfillableObligationFQNs

	// Logging. 0 = host sink default.
	LogLevel int // WithLogger: hosts install a slog sink; level selects verbosity
}

type TokenExchange struct { SubjectToken string; Audience []string }
type CertExchange struct { ClientCertPEM, ClientKeyPEM string; Audience []string }
type TLSOptions struct { ClientCertPEM, ClientKeyPEM, RootCAsPEM string; Audience []string }
type SigningAlgorithm string // ES256, ES384, ES512, RS256, RS384, RS512
```

`DPoPAlgorithm` default follows the reference: with no key configured the
SDK generates an ephemeral P-256 key and signs with ES256; with
`DPoPKeyPEM` the algorithm follows the key type.

### SDK (reference: `SDK`)

```go
func New(platformEndpoint string, opts Options) (*SDK, error)

type SDK struct{ /* unexported */ }

func (s *SDK) Close() error
func (s *SDK) IsHealthy(ctx context.Context) (bool, error)
func (s *SDK) PlatformConfiguration() PlatformConfiguration
func (s *SDK) Logger() *slog.Logger

// TDF creation.
func (s *SDK) CreateTDF(ctx context.Context, w io.Writer, r io.Reader, opts TDFOptions) (*TDFObject, error)
func (s *SDK) EncryptBytes(ctx context.Context, plaintext []byte, opts TDFOptions) ([]byte, *TDFObject, error)

// TDF reading.
func (s *SDK) LoadTDF(ctx context.Context, r io.ReadSeeker, opts ReaderOptions) (*Reader, error)
func (s *SDK) DecryptBytes(ctx context.Context, ciphertext []byte, opts ReaderOptions) ([]byte, error)
func (s *SDK) DecryptTo(ctx context.Context, out io.Writer, ciphertext []byte, opts ReaderOptions) error
func (s *SDK) DecryptFile(ctx context.Context, inputPath, outputPath string, opts ReaderOptions) error

// Bulk.
func (s *SDK) PrepareBulkDecrypt(ctx context.Context, opts BulkOptions) (*BulkDecryptPrepared, error)
func (s *SDK) BulkDecrypt(ctx context.Context, opts BulkOptions) error

// Chunked writer.
func (s *SDK) NewChunkedWriter(ctx context.Context, opts ChunkedWriterOptions) (*ChunkedWriter, error)

// Keys and policy helpers.
func (s *SDK) GetBaseKey(ctx context.Context) (*SimpleKasKey, error)
func (s *SDK) StoreKASKeys(url string, keys *KasPublicKeySet) error
func (s *SDK) ListAttributes(ctx context.Context, namespace ...string) ([]*policy.Attribute, error)
func (s *SDK) GetEntityAttributes(ctx context.Context, entity *authorization.Entity) ([]string, error)
func (s *SDK) ValidateAttributes(ctx context.Context, fqns []string) error
func (s *SDK) AttributeExists(ctx context.Context, fqn string) (bool, error)
func (s *SDK) AttributeValueExists(ctx context.Context, fqn string) (bool, error)
func (s *SDK) Auth() *AuthClient // AccessToken(ctx) (AccessToken, error)
func (s *SDK) Conn() ConnectRPCConnection // {Endpoint}: the core services' endpoint

// The typed helpers across the Goalchemy boundary (see Service clients).
func (s *SDK) ListAttributesJSON(ctx context.Context, namespace []string) ([]string, error)
func (s *SDK) GetEntityAttributesJSON(ctx context.Context, entity string) ([]string, error)

// Service clients: one generic JSON call in the shared source; the hosts'
// typed clients are built on it (see Service clients).
func (s *SDK) Call(ctx context.Context, service, method, request string) (string, error)
```

`Client` is the interface with exactly this method set; `*SDK` implements
it. Wrappers and test doubles implement `Client`.

Package-level helpers, as in the reference: `IsValidTdf(io.ReadSeeker)`,
`GetTdfType(io.ReadSeeker) TdfType`, `IsPlatformEndpointMalformed(string)`,
`SupportedFeatures() []string`, `Version`, `TDFSpecVersion`,
`KeyTypeToPolicyAlgorithm`, `PolicyAlgorithmToKeyType`,
`NewAttributeNameFQN`, `NewAttributeValueFQN`, `NewResourceLocator`,
`NewResourceLocatorFromReader`, `DefaultKeySplitter`,
`GetSystemMetadataAssertionConfig`, `FromBulkErrors`.

### TDFOptions (reference: `TDFOption`, `TDFConfig`)

```go
type TDFOptions struct {
	Attributes            []string        // WithDataAttributes
	AttributeValues       []string        // WithDataAttributeValues: policy.Value in protobuf JSON
	DisableAutoconfigure  bool            // WithAutoconfigure(false)
	KasInformation        []KASInfo       // WithKasInformation
	WrappingKeyAlgorithm  KeyType         // WithWrappingKeyAlg; "" = rsa:2048
	SegmentSize           int64           // WithSegmentSize; 0 = 2 MiB, clamped 16 KiB..4 MiB
	InputSize             int64           // WithInputSize; 0 = unknown
	MimeType              string          // WithMimeType; "" = application/octet-stream
	Metadata              string          // WithMetaData
	Assertions            []AssertionConfig // WithAssertions
	SystemMetadataAssertion bool          // WithSystemMetadataAssertion
	TargetMode            string          // WithTargetMode; "" = 4.3.0
	PolicyFrom            *Reader         // WithPolicyFrom
	KeySplitter           KeySplitter     // nil = DefaultKeySplitter; shared-source implementations only
}

type KASInfo struct { URL, PublicKey, KID string; Algorithm KeyType; Default bool }
type KeyType string // rsa:2048, rsa:4096, ec:secp256r1, ec:secp384r1, ec:secp521r1 (the reference ocrypto set)
```

### ReaderOptions (reference: `TDFReaderOption`, `TDFReaderConfig`)

```go
type ReaderOptions struct {
	AssertionVerificationKeys    AssertionVerificationKeys // WithAssertionVerificationKeys
	DisableAssertionVerification bool                      // WithDisableAssertionVerification
	KasAllowlist                 []string                  // WithKasAllowlist; nil = fetch from registry
	IgnoreAllowlist              bool                      // WithIgnoreAllowlist
	MaxManifestSize              int64                     // WithMaxManifestSize; 0 = 10 MiB
	SchemaValidation             SchemaValidationIntensity // WithSchemaValidation; 0 = Skip
	SessionKeyType               KeyType                   // WithSessionKeyType; "" = rsa:2048
	FulfillableObligationFQNs    []string                  // WithTDFFulfillableObligationFQNs
}
```

### Reader (reference: `Reader`)

```go
type Reader struct{ /* unexported */ }

func (r *Reader) Init(ctx context.Context) error          // fetch and unwrap keys now
func (r *Reader) Read(p []byte) (int, error)               // io.Reader
func (r *Reader) ReadAt(p []byte, off int64) (int, error)  // io.ReaderAt
func (r *Reader) Seek(off int64, whence int) (int64, error)
func (r *Reader) WriteTo(w io.Writer) (int64, error)
func (r *Reader) Manifest() Manifest
func (r *Reader) Policy() (PolicyObject, error)
func (r *Reader) DataAttributes() ([]string, error)
func (r *Reader) UnencryptedMetadata() ([]byte, error)
func (r *Reader) Obligations(ctx context.Context) (RequiredObligations, error)
func (r *Reader) UnsafePayloadKeyRetrieval() ([]byte, error)
func (r *Reader) Close() error
```

`TDFReader` is the interface with this method set. `Read`, `ReadAt`, `Seek`
and `WriteTo` run `Init` lazily with the context given to `LoadTDF`; hosts
that need a different context call `Init` first.

Across the boundary the reader is a handle that is also a source stream
(Goalchemy phase 4 of #22): targets receive it as their native readable
stream plus the methods above.

### TDFObject, ChunkedWriter, Bulk (reference names kept)

```go
type TDFObject struct{ /* unexported */ }
func (t *TDFObject) Manifest() Manifest
func (t *TDFObject) Size() int64

type ChunkedWriter struct{ /* unexported */ }
func (w *ChunkedWriter) WriteSegment(ctx context.Context, index int, data []byte) (*ChunkedSegmentResult, error)
func (w *ChunkedWriter) GetManifest(ctx context.Context, opts ChunkedFinalizeOptions) (*Manifest, error)
func (w *ChunkedWriter) Finalize(ctx context.Context, opts ChunkedFinalizeOptions) (*ChunkedFinalizeResult, error)
func (w *ChunkedWriter) Close() error

type BulkOptions struct {
	TDFs            []*BulkTDF      // WithTDFs
	TDFType         TdfType         // WithTDFType
	Reader          ReaderOptions   // WithTDF3DecryptOptions
	KasAllowlist    []string        // WithBulkKasAllowlist
	IgnoreAllowlist bool            // WithBulkIgnoreAllowlist
}
type BulkTDF struct { Reader io.ReadSeeker; Writer io.Writer; Error error }
type BulkDecryptPrepared struct{ /* unexported */ }
func (b *BulkDecryptPrepared) BulkDecrypt(ctx context.Context) error
```

### Manifest and policy types (reference: `manifest.go`, `assertion.go`)

Identical field names and JSON tags to the reference: `Manifest`,
`EncryptionInformation`, `KeyAccess`, `PolicyBinding`, `Method`,
`IntegrityInformation`, `RootSignature`, `Segment`, `Payload`,
`PolicyObject`, `EncryptedMetadata`, `Assertion`, `Statement`, `Binding`,
`AssertionConfig`, `AssertionKey`, `AssertionVerificationKeys`,
`RequiredObligations`, `ResourceLocator`, `AttributeNameFQN`,
`AttributeValueFQN`, `SimpleKasKey`, `KasPublicKeySet`, `KASPublicKey`,
`Split`, `SplitResult`. `KeyAccess.PolicyBinding` is `jsonvalue.Value`
in the shared source (string or object, as in the wire format) with a
`Binding() (PolicyBinding, error)` accessor. Enumerations
(`AssertionType`, `Scope`, `AppliesToState`, `BindingMethod`,
`AssertionKeyAlg`, `TdfType`, `SchemaValidationIntensity`,
`RootIntegrityAlg`, `SegmentIntegrityAlg`) keep the reference's values.

### Errors (reference: `tdferrors.go`, `decrypterrors.go`, `basekeyerrors.go`, `BulkErrors`, `ErrAssertionFailure`)

```go
type Error struct {
	Kind                ErrorKind
	Op                  string   // "CreateTDF", "LoadTDF", "Rewrap", ...
	Message             string   // the reference's wording, e.g. "tdf: rewrap request 403"
	HTTPStatus          int
	ConnectCode         string   // connect.Code name when the failure came from a service
	ServiceMessage      string   // bounded, never logged by the SDK
	KASURL              string
	RequiredObligations []string
	AssertionID         string   // ErrAssertionFailure
	Cause               error
}
func (e *Error) Error() string // Message, with the reference's "[tamper detected] " prefix where it has one
func (e *Error) Unwrap() error

type ErrorKind string
const (
	KindTampered, KindRootSignature, KindSegmentSize, KindSegmentSignature,
	KindPayloadRead, KindInvalidOffset, KindRewrapBadRequest,   // all unwrap to ErrTampered
	KindUnsupportedRootIntegrity, KindUnsupportedSegmentIntegrity,
	KindNotDecryptable, KindDecryptFailed, KindAESGCMExhausted,
	KindKASRequest, KindRewrapForbidden, KindRewrapUnauthorized,
	KindAssertionFailure, KindObligationsRequired,
	KindBaseKeyNotFound, KindBaseKeyInvalid,
	KindAllowlistRejected, KindUnsupportedKeyAccess, KindUnsupportedFeature,
	KindInvalidInput, KindAuth, KindTransport, KindCanceled, KindDeadlineExceeded,
	KindBulk // Cause unwraps to []error
)
```

Go sentinels `ErrTampered`, `ErrRootSigValidation` and the rest of the
reference's list exist as package variables; `errors.Is(err, ErrTampered)`
holds for every tamper kind. Other targets expose `Kind` as an enum and
`Message` verbatim.

### Token sources (reference: `auth.AccessTokenSource`, `WithOAuthAccessTokenSource`, `WithCustom*`)

```go
type AccessToken struct { Value, Scheme string; ExpiresAt int64; ConfirmationJKT string }
type TokenSource interface { Token(ctx context.Context) (AccessToken, error) }
```

Built-in sources: client credentials, token exchange, certificate exchange,
all implemented in shared source over `lib/http.DoWith`. A host-provided
source is registered as a Goalchemy host callback and named in
`Options.TokenProviderName`; the host adapter in each target wraps this in
the idiomatic form (Go: `oauth2.TokenSource`; TypeScript: async function;
Python: callable; Java: `Supplier`; C#: `Func<CancellationToken, Task<AccessToken>>`;
Rust: trait object; C: function pointer; Swift: closure). DPoP keys for host
sources are supplied as PEM.

## Service clients

Messages cross the Goalchemy boundary as protobuf JSON. The shared source
types only what it reads itself; each host types the rest with its own
ecosystem's protobuf messages and JSON codec, generated from the same pinned
protos.

Shared source (`src/connect.go`, `src/protocol/`):

- `scripts/generate-protocol.sh` runs `protoc-gen-goalchemy` with `include=`
  on the `../platform/service` protos for `kas.AccessService` `Rewrap` and
  `PublicKey` (and `UnsignedRewrapRequest`), `GetWellKnownConfiguration`,
  `ListKeyAccessServers`, and the messages the Phase 3 helpers read
  (`ListAttributes`, `GetAttribute`, `GetAttributeValuesByFqns`,
  `GetEntitlements`). The KAS client, discovery and the registry allowlist
  call these generated clients, as the reference calls `kasconnect` and
  `sdkconnect`.
- Every generated client is a `connect.Client` whose `HTTPClient` is the
  SDK's authenticated transport (`httpClient.Do`): the reference's token
  interceptor and DPoP transport (Bearer or DPoP header, proof with `ath`,
  nonce retry), the per-request timeout and the response bound. Nothing is
  package-level; each call builds its `connect.Client` from the SDK
  instance, as the reference builds its KAS clients per request.
- The generic boundary call reaches every other method:

  ```go
  func (s *SDK) Call(ctx context.Context, service, method, request string) (string, error)
  ```

  `service` is one of the 16 services behind the reference's `sdkconnect`
  clients, by protobuf name (`policy.attributes.AttributesService`, ...,
  `policy.obligations.Service`, `wellknownconfiguration.WellKnownService`);
  `method` is the RPC name; `request` and the result are protobuf JSON
  objects. It POSTs Connect unary JSON through the same transport, to the
  service's connection (below). `Call` is part of `Client`.
- A failure the service reports is an `Error` with `ConnectCode` (the Connect
  code name), `ServiceMessage`, `HTTPStatus`, and `Message` worded as
  connect-go's `Error()` (`not_found: attribute not found`). `Kind` is
  `KindAuth` for `unauthenticated` and `permission_denied`, `KindInvalidInput`
  for `invalid_argument`, `not_found`, `already_exists`,
  `failed_precondition` and `out_of_range`, `KindCanceled` and
  `KindDeadlineExceeded` for those codes and `KindTransport` otherwise; the
  KAS calls keep `KindRewrapForbidden` and `KindKASRequest`. A reply the
  client cannot read (wrong content type, malformed JSON) is
  `KindTransport`; a failure of the SDK's own transport (token source, DPoP,
  connection) keeps its `Error`.
- Streaming RPCs are not callable (none of the 16 services has one).

Per-target type sources:

| Target | Message types | JSON codec | Service clients | Errors |
| --- | --- | --- | --- | --- |
| Go | the reference's `github.com/opentdf/platform/protocol/go` v0.41.0 (identical to the pinned platform revision) | `protojson`, `DiscardUnknown` | `s.Attributes()` etc. returning the reference's `sdkconnect` interfaces (`AttributesServiceClient`, ...), generated from `sdkconnect` by `scripts/generate-service-clients.py`; each method `(ctx, *Req) (*Resp, error)` | `*Error` with `ConnectCode` (no connect-go dependency) |
| TypeScript | protobuf-es v2 (`protoc-gen-es` 2.2.5, as the web SDK; runtime `@bufbuild/protobuf` 2.15.0) under `@opentdf-local/tdf3/protocol/*` | `toJsonString`/`fromJsonString` with `ignoreUnknownFields` | `sdk.transport` is a Connect-ES (`@connectrpc/connect` 2.0.2) `Transport`, so `createClient(AnyService, sdk.transport)` works; `sdk.attributes` etc. are pre-made clients | `ConnectError` from the Transport (code from `connectCode`, or from the kind; the `OpenTDFError` is its `cause`); `sdk.call` throws `OpenTDFError` |
| Python | protobuf Python (`protoc` 33.6) under `opentdf_tdf3.protocol`, runtime `protobuf` 6.33.6 | `json_format`, `ignore_unknown_fields=True` | `sdk.attributes.list_attributes(ListAttributesRequest(...))`, snake_case methods with `*_async` twins, built from the service descriptors | `OpenTDFError` with `connect_code` |
| Java | protobuf-java (`protoc` 33.6, `optimize_for = CODE_SIZE`) under `io.opentdf.tdf3.protocol`, runtime `protobuf-java` and `protobuf-java-util` 4.33.6 with `gson` 2.8.9 | `JsonFormat` printer and parser (`ignoringUnknownFields`) with a type registry of every service file | `SDK extends PlatformServices`: `sdk.attributes()` etc. return the reference's `sdkconnect` interfaces (`AttributesServiceClient`, ...); each method blocking with an optional `Duration`, plus an `*Async` twin; `sdk.call` / `callAsync` for raw JSON | `OpenTDFException` with `connectCode()` |
| C# | Google.Protobuf (`protoc` 33.6) under `OpenTDF.Protocol`, runtime `Google.Protobuf` 3.33.6 | `JsonFormatter`, `JsonParser` (`WithIgnoreUnknownFields`) with a type registry of every service file | `Sdk : PlatformServices`: `sdk.Attributes` etc. are `IAttributesServiceClient`, ... with `Task<Resp> XAsync(Req, CancellationToken)`; `sdk.CallAsync` for raw JSON | `OpenTdfException` with `ConnectCode` |
| Rust | prost 0.14.4 (`protoc-gen-prost` 0.5.0) under `opentdf_tdf3::protocol`, serde impls by `protoc-gen-prost-serde` 0.4.0 (pbjson 0.8.0, well-known types from `pbjson-types`) | `serde_json` over the pbjson impls, unknown fields ignored | `sdk.attributes()` etc. return `AttributesServiceClient`, ... with `fn x(&self, &Req) -> Result<Resp>` and `x_async`; `sdk.r#unsafe()`; `sdk.call` / `call_async` for raw JSON | `Error` with `connect_code` |
| Swift | SwiftProtobuf (`protoc-gen-swift` 1.38.1) in the `OpenTDFTDF3` module (`Policy_Attributes_ListAttributesRequest`), runtime `swift-protobuf` 1.38.1 | `jsonString()` and `init(jsonString:options:)` with `ignoreUnknownFields` | `sdk.attributes` etc. return `AttributesServiceClient`, ... with `async throws` methods whose request defaults to the empty message; `sdk.call` for raw JSON | `OpenTDFError` with `connectCode` |
| C | none: protobuf JSON text (typed C is #29) | | `opentdf_services.h`: `otdf_<service>_<method>(sdk, request_json, call, &response_json, &error)` for the 131 methods, over `otdf_sdk_call(sdk, service, method, ...)` | `otdf_error.connect_code` |

Generated host code is committed and `scripts/generate-protocol.sh --check`
proves it current in CI; it pins buf, `protoc`, `protoc-gen-es`,
`protoc-gen-prost`/`protoc-gen-prost-serde` and `protoc-gen-swift` (built from
its tagged commit). Per-call headers and deadlines come from the caller's
context (Go), `AbortSignal`/`timeoutMs` (TypeScript), `timeout=` (Python), a
`Duration` or `CompletableFuture.cancel` (Java), a `CancellationToken` (C#),
dropping the `*_async` future (Rust), `Task` cancellation (Swift) or the
`otdf_call` flag and deadline (C); Connect-ES request headers are not
forwarded.

Go:

```go
attrs, err := s.Attributes().ListAttributes(ctx, &attributes.ListAttributesRequest{Namespace: "example.com"})
var e *opentdf.Error
if errors.As(err, &e) && e.ConnectCode == "not_found" { ... }
```

TypeScript:

```ts
const { attributes } = await sdk.attributes.listAttributes({ namespace: 'example.com' });
const kas = createClient(KeyAccessServerRegistryService, sdk.transport);
```

Python:

```python
attrs = sdk.attributes.list_attributes(ListAttributesRequest(namespace='example.com'))
```

Java:

```java
var attrs = sdk.attributes().listAttributes(ListAttributesRequest.newBuilder().setNamespace("example.com").build());
```

C#:

```csharp
var attrs = await sdk.Attributes.ListAttributesAsync(new ListAttributesRequest { Namespace = "example.com" }, ct);
```

Rust:

```rust
let attrs = sdk.attributes().list_attributes(&ListAttributesRequest { namespace: "example.com".into(), ..Default::default() })?;
```

Swift:

```swift
var request = Policy_Attributes_ListAttributesRequest()
request.namespace = "example.com"
let attrs = try await sdk.attributes.listAttributes(request)
```

C:

```c
char *attrs;
OTDF_CHECK(otdf_attributes_list_attributes(sdk, "{\"namespace\":\"example.com\"}", NULL, &attrs, &e));
```

### Helpers, Auth and endpoint overrides (Phase 3 part 3)

The shared source ports the reference's `discovery.go` and `auth_client.go`:
`ListAttributes` (every page until `next_offset` is 0, at most 1000 pages,
at most one namespace filter), `AttributeExists` and `AttributeValueExists`
(`not_found` is `false`, not an error), `ValidateAttributes` (at most 250
value FQNs, each checked locally, missing ones reported as
`attribute not found: <fqn>, ...`), `GetEntityAttributes` (the entitlement
whose entity id matches) and `SDK.Auth().AccessToken`. They call the
generated `policy.attributes` and `authorization` clients (`include=` in
`scripts/generate-protocol.sh`) over the SDK's authenticated transport,
through the SDK's `attributesService` and `authorizationService`
collaborators. The FQN checks are the reference's regular expressions as
parsers (`granter.go`, differentially tested against Go's `regexp`).
`ListAttributesJSON` and `GetEntityAttributesJSON` carry the typed results
across the boundary as protobuf JSON; every host decodes them with its own
`policy.Attribute` and encodes its own `authorization.Entity`.

Connections, as the reference's `platformConn` and `ersConn`, all over the
SDK's transport (token source, DPoP, timeouts):

| Services | Endpoint |
| --- | --- |
| `policy.*` (attributes, namespaces, KAS registry, ...), including the registry allowlist and the helpers | `PolicyEndpoint`, else `CoreEndpoint`, else the platform endpoint |
| `authorization`, `authorization.v2`, including `GetEntityAttributes` | `AuthorizationEndpoint`, else core, else platform |
| `entityresolution`, `entityresolution.v2` | `EntityResolutionEndpoint`, else core, else platform |
| `wellknownconfiguration`, including the configuration `New` fetches | `WellknownEndpoint`, else core, else platform |
| health (`IsHealthy`, `ValidateConnection`), `SDK.Conn` | `CoreEndpoint`, else platform |

The platform endpoint stays the configuration's `platform_endpoint` and the
allowlist's platform entry; KAS calls go to the KAS URL. A malformed override
fails `New` with `platform endpoint is malformed [<endpoint>]`.

Per target:

| Target | Helpers | Auth | Conn and overrides |
| --- | --- | --- | --- |
| Go | `s.ListAttributes(ctx, []string{ns})` (`[]*policy.Attribute` of `protocol/go`), `s.GetEntityAttributes(ctx, *authorization.Entity)`, `s.ValidateAttributes(ctx, fqns)`, `s.AttributeExists(ctx, fqn)`, `s.AttributeValueExists(ctx, fqn)`, each with trailing `callbacks...` | `a, _ := s.Auth(ctx); a.AccessToken(ctx)`; `a.Close()` | `s.Conn(ctx)`; `WithCustomCoreConnection(&ConnectRPCConnection{Endpoint: ...})` and the policy, authorization, entity resolution and well-known variants |
| TypeScript | `sdk.listAttributes(namespace?)` (`Attribute[]`), `getEntityAttributes(EntityInit)`, `validateAttributes(fqns)`, `attributeExists(fqn)`, `attributeValueExists(fqn)`, each `Promise` with optional `{signal}` | `sdk.auth().accessToken()` | `sdk.conn()`; `coreEndpoint`, `policyEndpoint`, `authorizationEndpoint`, `entityResolutionEndpoint`, `wellknownEndpoint` options |
| Python | `sdk.list_attributes(namespace=None)`, `get_entity_attributes(Entity)`, `validate_attributes(fqns)`, `attribute_exists(fqn)`, `attribute_value_exists(fqn)`, with `timeout=` and `*_async` twins | `sdk.auth().access_token()` / `access_token_async()` | `sdk.conn()`; `core_endpoint=` and the other keyword options |
| Java | `sdk.listAttributes(ns)` (`List<Attribute>`), `getEntityAttributes(Entity)`, `validateAttributes(List)`, `attributeExists(fqn)`, `attributeValueExists(fqn)`, with `Duration` overloads and `*Async` twins | `sdk.auth().accessToken()` (`TDF3.AccessToken`) / `accessTokenAsync()` | `sdk.conn()` (`SDK.ConnectRPCConnection`); `Builder.coreEndpoint(...)` and the others |
| C# | `ListAttributesAsync(ns)`, `GetEntityAttributesAsync(Entity)`, `ValidateAttributesAsync(fqns)`, `AttributeExistsAsync(fqn)`, `AttributeValueExistsAsync(fqn)`, each with a `CancellationToken` | `sdk.Auth().AccessTokenAsync()` | `ConnAsync()`; `Options.CoreEndpoint` and the others |
| Rust | `list_attributes(Option<&str>)` (`Vec<protocol::policy::Attribute>`), `get_entity_attributes(Option<&Entity>)`, `validate_attributes(&[String])`, `attribute_exists(&str)`, `attribute_value_exists(&str)`, with `*_async` twins | `sdk.auth().access_token()` / `access_token_async()` | `sdk.conn()` (`ConnectRpcConnection`); `Options { core_endpoint, .. }` and the others |
| Swift | `listAttributes(_:)` (`[Policy_Attribute]`), `getEntityAttributes(_: Authorization_Entity?)`, `validateAttributes(_:)`, `attributeExists(_:)`, `attributeValueExists(_:)`, `async throws` | `sdk.auth().accessToken()` | `sdk.conn()`; `Options.coreEndpoint` and the others |
| C | `otdf_list_attributes(sdk, namespace_name, call, &attributes_json, &e)` (a JSON array of `policy.Attribute`), `otdf_get_entity_attributes(sdk, entity_json, call, &fqns, &count, &e)`, `otdf_validate_attributes`, `otdf_attribute_exists`, `otdf_attribute_value_exists` | `otdf_auth_access_token(sdk, call, &token, &e)` (`otdf_access_token`, `otdf_access_token_free`) | `otdf_sdk_conn(sdk, call, &endpoint, &e)`; `otdf_options.core_endpoint` and the others |

Go:

```go
attrs, err := s.ListAttributes(ctx, []string{"example.com"})
err = s.ValidateAttributes(ctx, []string{"https://example.com/attr/level/value/high"})
fqns, err := s.GetEntityAttributes(ctx, &authorization.Entity{Id: "e1", EntityType: &authorization.Entity_ClientId{ClientId: "my-service"}})
s, err := sdk.NewWithOptions(ctx, platform, sdk.WithCustomCoreConnection(&sdk.ConnectRPCConnection{Endpoint: "https://core.example.com"}))
```

TypeScript:

```ts
const exists = await sdk.attributeValueExists('https://example.com/attr/level/value/high');
const fqns = await sdk.getEntityAttributes({ id: 'e1', entityType: { case: 'clientId', value: 'my-service' } });
const token = await sdk.auth().accessToken();
```

### Autoconfigure and key splitting (Phase 4 part 1)

`granter.go`, `key_splitter.go` and `resource_locator.go` are ported file by
file. With autoconfigure on (the default), `CreateTDF`/`EncryptBytes` resolve
the key access template as the reference does:

1. `TDFOptions.AttributeValues` (offline, `WithDataAttributeValues`) or
   `TDFOptions.Attributes` looked up with `GetKeyMappingsByFqns`, falling
   back to `GetAttributeValuesByFqns` for values without mapped keys and for
   platforms without the method (`unimplemented`);
2. mapped KAS keys (value, else definition, else namespace) become the
   template directly; legacy grants become a split plan whose keys come
   from `KasInformation` or the KAS;
3. with neither, the platform's base key; without one, the default KAS of
   `KasInformation` (`Default`, else every entry).

The plan is the reference's boolean reduction: an `anyOf` clause is one
share wrapped to each of its KAS (one `sid`), `allOf` and `hierarchy` give one
share per KAS; a single share has no `sid`. `tests/splitplan` checks that the
reference and the shared source produce the same key access objects (KAS,
kid, wrapping and split grouping, in order) for the xtest fixtures and for
random policies. `DisableAutoconfigure` keeps `KasInformation` as the
template (one share per entry, `s-<i>` split ids when there are several).
`Options.NoKIDInKAO` leaves `kid` out of the KAOs.

Exported with it: `NewAttributeNameFQN` and `NewAttributeValueFQN` (values
`{URL, Key}` with the reference's methods in the shared source and the
generated Go package), `NewResourceLocator` and `NewResourceLocatorFromReader`
(a `ResourceLocator` handle), and the `KeySplitter` contract types
(`Split`, `SplitResult` with `Validate` and `VerifyReconstruction`,
`KASPublicKey`, `ErrSplitterRequiresDefaultKAS`,
`ErrSplitterUnsupportedAlgorithm`), which the chunked writer of Phase 10
takes.

Per target:

| Target | Offline attribute values | FQN constructors |
| --- | --- | --- |
| Go | `WithDataAttributeValues(...*policy.Value)` (`protocol/go`); `WithDataAttributes` after it keeps the values' FQNs, as the reference does | `NewAttributeNameFQN(ctx, fqn)`, `NewAttributeValueFQN(ctx, fqn)` with `String`, `Authority`, `Name`, `Value`, `Prefix`, `Select` |
| TypeScript | `attributeValues: ValueInit[]` | `newAttributeNameFqn(fqn)`, `newAttributeValueFqn(fqn)` → `{url, key}` |
| Python | `attribute_values=[policy Value]` | `new_attribute_name_fqn(fqn)`, `new_attribute_value_fqn(fqn)` → `{'url', 'key'}` |
| Java | `Builder.attributeValues(List<Value>)` | `SDK.newAttributeNameFQN(fqn)`, `SDK.newAttributeValueFQN(fqn)` → `SDK.AttributeFQN(url, key)` |
| C# | `TdfOptions.AttributeValues` (`IReadOnlyList<Policy.Value>`) | `Sdk.NewAttributeNameFqnAsync`, `Sdk.NewAttributeValueFqnAsync` → `AttributeFqn(Url, Key)` |
| Rust | `TdfOptions { attribute_values: Vec<protocol::policy::Value>, .. }` | `new_attribute_name_fqn`, `new_attribute_value_fqn` → `AttributeFqn { url, key }` |
| Swift | `TDFOptions(attributeValues: [Policy_Value])` | `SDK.newAttributeNameFQN(_:)`, `SDK.newAttributeValueFQN(_:)` → `AttributeFQN` |
| C | `otdf_tdf_options.attribute_values` (protobuf JSON strings) | `otdf_new_attribute_name_fqn(fqn, &url, &key, &e)`, `otdf_new_attribute_value_fqn` |

```go
tdf, err := s.CreateTDF(ctx, out, in, sdk.WithDataAttributes("https://example.com/attr/level/value/high"))
v, err := sdk.NewAttributeValueFQN(ctx, "https://example.com/attr/level/value/high")
```

## Target idioms

Each generated package exposes the same surface. The table is the binding
rule; the examples show `New`, one encrypt, one decrypt and one service call.

| Concern | Go | TypeScript | Python | Java | C# | Rust | C | Swift |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Package | module `opentdf` (path chosen at publication) | `@<scope>/opentdf-sdk` ESM, Node and browser builds | `opentdf_sdk` wheel | `io.opentdf.sdk` JAR | `OpenTDF.Sdk` | `opentdf_sdk` crate | `libopentdf` + `opentdf.h` | `OpenTDF` SwiftPM |
| `New` | `opentdf.New(endpoint, opts...)` with `With*` adapters | `await SDK.create(endpoint, options)` | `SDK(endpoint, **options)` and `await SDK.create_async(...)` | `SDK.builder(endpoint)....build()` | `await Sdk.CreateAsync(endpoint, options)` | `Sdk::new(endpoint, Options::default())` | `otdf_sdk_new(endpoint, &opts, &sdk)` | `try await SDK(endpoint, options)` |
| Cancellation | `context.Context` | `AbortSignal` option | `timeout=`/`asyncio` cancellation | `CompletableFuture.cancel` on `*Async`; blocking forms take a `Duration` | `CancellationToken` | `Future` drop | `otdf_cancel(handle)` | `Task` cancellation |
| Streams | `io.Reader`/`io.Writer` | `ReadableStream`, Node `Readable`, `Uint8Array`, `Blob` | file-like, `bytes` | `InputStream`/`OutputStream`, `byte[]` | `Stream`, `byte[]` | `Read`/`Write`, `&[u8]` | `gxc_stream` callbacks, buffers | `GoalchemyStream`, `Data` |
| Options | struct plus `With*` | object literal | keyword arguments | builder | record with `init` setters | struct with `Default` | struct, zero-initialised | struct with defaults |
| Errors | `*Error`, sentinels | `OpenTDFError` with `kind`, `message`, `httpStatus`, `requiredObligations` | `OpenTDFError` exception | `OpenTDFException` | `OpenTdfException` | `Error` enum carrying the fields | status code + `otdf_error_get(sdk)` | `OpenTDFError` enum |
| Reader | `*Reader` (`io.Reader` etc.) | `Reader` with `stream()`, `readAt`, `seek`, async iteration | file-like `Reader` | `Reader` with `asInputStream()` | `Reader : Stream` | `Reader: Read + Seek` | `otdf_reader_*` | `Reader: AsyncSequence` |
| Service clients | `s.Attributes().GetAttribute(ctx, req)` over `protocol/go` types | `sdk.attributes.getAttribute(req)` (Connect-ES client) | `sdk.attributes.get_attribute(req)` (protobuf messages) | `sdk.attributes().getAttribute(req)` | `sdk.Attributes.GetAttributeAsync(req, ct)` | `sdk.attributes().get_attribute(&req)` | `otdf_attributes_get_attribute(sdk, req_json, call, &resp_json, &error)` | `sdk.attributes.getAttribute(req)` |
| Release | `Close()` | `await sdk.close()`, `await using` | `close()`, `with` | `close()`, try-with-resources | `DisposeAsync` | `close()`, drop warns | `otdf_sdk_close` | `close()`, deinit warns |

Go:

```go
s, err := opentdf.New("https://platform.example", opentdf.WithClientCredentials(id, secret, nil))
tdf, err := s.CreateTDF(ctx, out, in, opentdf.WithDataAttributes("https://ex.com/attr/a/value/b"))
r, err := s.LoadTDF(ctx, file)
_, err = io.Copy(plain, r)
attr, err := s.Attributes().GetAttribute(ctx, &attributes.GetAttributeRequest{Fqn: fqn})
```

TypeScript:

```ts
const sdk = await SDK.create("https://platform.example", { clientId, clientSecret });
const { manifest } = await sdk.createTdf(outStream, inStream, { attributes: [fqn] });
const reader = await sdk.loadTdf(file);
for await (const chunk of reader) { ... }
const attr = await sdk.attributes.getAttribute({ fqn });
```

Python:

```python
with SDK("https://platform.example", client_id=cid, client_secret=secret) as sdk:
    obj = sdk.create_tdf(out, src, attributes=[fqn])
    with sdk.load_tdf(open("a.tdf", "rb")) as r:
        plain = r.read()
    attr = sdk.attributes.get_attribute(GetAttributeRequest(fqn=fqn))
```

Java:

```java
try (SDK sdk = SDK.builder("https://platform.example").clientCredentials(id, secret).build()) {
    TDFObject obj = sdk.createTDF(out, in, TDFOptions.builder().attributes(List.of(fqn)).build());
    try (Reader r = sdk.loadTDF(channel)) { r.asInputStream().transferTo(plain); }
    Attribute a = sdk.attributes().getAttribute(GetAttributeRequest.newBuilder().setFqn(fqn).build()).getAttribute();
}
```

C#:

```csharp
await using var sdk = await Sdk.CreateAsync("https://platform.example", new Options { ClientId = id, ClientSecret = secret });
var obj = await sdk.CreateTdfAsync(output, input, new TdfOptions { Attributes = [fqn] }, ct);
await using var reader = await sdk.LoadTdfAsync(file, ct: ct);
await reader.CopyToAsync(plain, ct);
var attr = await sdk.Attributes.GetAttributeAsync(new GetAttributeRequest { Fqn = fqn }, ct);
```

Rust:

```rust
let sdk = Sdk::new("https://platform.example", Options { client_id: id, client_secret: secret, ..Default::default() })?;
let obj = sdk.create_tdf(&mut out, &mut input, TdfOptions { attributes: vec![fqn], ..Default::default() })?;
let mut r = sdk.load_tdf(file, ReaderOptions::default())?;
std::io::copy(&mut r, &mut plain)?;
let attr = sdk.attributes().get_attribute(&GetAttributeRequest { identifier: Some(Identifier::Fqn(fqn)), ..Default::default() })?;
```

C:

```c
otdf_sdk *sdk; otdf_options o = OTDF_OPTIONS_INIT; o.client_id = id; o.client_secret = secret;
OTDF_CHECK(otdf_sdk_new("https://platform.example", &o, &sdk));
otdf_tdf_options t = OTDF_TDF_OPTIONS_INIT; t.attributes = &fqn; t.attribute_count = 1;
OTDF_CHECK(otdf_create_tdf(sdk, &out_stream, &in_stream, &t, &obj));
OTDF_CHECK(otdf_load_tdf(sdk, &in_stream, NULL, &reader));
while ((n = otdf_reader_read(reader, buf, sizeof buf)) > 0) { ... }
OTDF_CHECK(otdf_attributes_get_attribute(sdk, "{\"fqn\":\"...\"}", NULL, &resp_json, &e));
```

Swift:

```swift
let sdk = try await SDK("https://platform.example", Options(clientId: id, clientSecret: secret))
let obj = try await sdk.createTDF(to: out, from: input, options: TDFOptions(attributes: [fqn]))
let reader = try await sdk.loadTDF(file)
for try await chunk in reader { ... }
let attr = try await sdk.attributes.getAttribute(.with { $0.fqn = fqn })
```

## Versioning and features

`Version` tracks this SDK; `ReferenceVersion` records the reference SDK
version it matches (`0.34.0`). `TDFSpecVersion` is `4.3.0`.
`SupportedFeatures()` returns the xtest feature names that pass; the CLI's
`supports` subcommand and `--version --json` read it.

## Change control

Changes to this document go in their own PR, reviewed before the
implementing phase starts. A phase that discovers the spec cannot be met
records the deviation here, with the reason, in the same PR as the code.

### Deviations recorded by Phase 2 part 1

Found while implementing the shared source on Goalchemy v0.6.0; each keeps
the reference behaviour unless noted.

- `New` takes a leading `context.Context`: the reference's `New` has none,
  but discovery is network I/O and every generated call already carries the
  host's context, so the generated signatures are unchanged and the caller's
  deadline reaches the well-known and OIDC requests.
- `Options.PlatformConfiguration` is a struct of `Entries []{Key, Value}`
  rather than `map[string]string`: maps do not cross the Goalchemy value
  boundary. String members keep their value, other members their compact
  JSON; the reference's accessors (`Issuer`, `AuthzEndpoint`,
  `TokenEndpoint`) read them, and the Go adapter's
  `WithPlatformConfiguration(map[string]any)` converts.
- `Options.TokenExchange`, `CertExchange` and `TLS` are values whose zero
  value means unset, not pointers, for the same reason.
- `Error` has no `Cause` field (an interface field would make the record
  uncrossable); `Error.Is` matches a sentinel whose text appears in the
  chain, so `errors.Is` holds for every reference sentinel natively and in
  the generated Go package. `Op` and the service fields come from the
  innermost `Error` in the chain; the most specific kind wins over a generic
  `KindInvalidInput`.
- `KeyAccess.PolicyBinding` is the `PolicyBinding` struct, whose JSON
  methods accept the object and the legacy string forms, instead of
  `jsonvalue.Value` (which has unexported fields and cannot cross); the
  `Binding()` accessor stays.
- Manifest types use named fields where the reference embeds
  (`Manifest.EncryptionInformation`, `Manifest.Payload`,
  `EncryptionInformation.IntegrityInformation`,
  `IntegrityInformation.RootSignature`), the anonymous `PolicyObject.Body`
  is `PolicyBody`, and `attributeObject` is exported as `AttributeObject`;
  JSON tags are unchanged.
- `TDFOptions` has no `KeySplitter` (an interface field) or `PolicyFrom`
  (a handle field) member: the splitter is an `SDK` collaborator and
  `WithPolicyFrom` is a Go-adapter option that copies the reader's
  attributes. `AttributeValues` and `ReaderOptions.AssertionVerificationKeys`
  are added with their phases.
- `GetBaseKey` returns `SimpleKasKey` by value and `StoreKASKeys` takes
  `KasPublicKeySet` by value: a pointer to an exported root struct would be
  read as a handle.
- The Goalchemy export roots are `./src/library` and `./src` together: a
  handle must be an exported struct of a root package and only root value
  types keep their names, so re-exporting through `src/library` alone would
  have meant duplicating every type.
- The KAS allowlist is fetched in `LoadTDF`, as the reference does, rather
  than on the first decrypt.

### Deviations recorded by Phase 2 part 2

Found while exposing the handle API through the generated TypeScript and
Python packages; each keeps the reference behaviour unless noted.

- A reader's lazy `Init` (run by `UnencryptedMetadata` and, in Phase 7, the
  stream methods) uses the `LoadTDF` context while it lives; a generated
  host's call context ends with its call, so a later lazy operation falls
  back to the background context. Hosts that need cancellation call `Init`
  first, which every target's generated `Init` takes a context for.
- Every exported function and method returns `*Error`: a plain `errors.New`
  or `fmt.Errorf` value does not cross the Goalchemy boundary with its text
  (Go: `unsupported_error`; other targets: a bare `source` failure), so the
  boundary wraps it with the chain's text and the operation's kind.
- A host token provider without a configured DPoP key gets the reference's
  ephemeral ES256 key for the rewrap request signature, as client
  credentials do; the api-spec's `TokenProviderName` option alone was
  unsigned.
- TypeScript's `createTdf` takes bytes and returns a `TDFObject` holding
  `bytes` plus the `manifest()`/`size()` handle; `encryptBytes` returns the
  bytes alone. Python's `create_tdf`/`encrypt_bytes` do the same. Streams
  arrive in Phase 7.
- Manifests and policies cross to TypeScript and Python keyed by the
  reference's JSON tags (`encryptionInformation.keyAccess[0].url`), not by
  the Go field names, so the "JSON tags are byte-identical" rule holds for
  the host-visible shape; the generated value interfaces keep the Go names.
- TypeScript `platformConfiguration()` and Python `platform_configuration()`
  return the members as a string map, the inverse of the entries the Go
  adapter's `WithPlatformConfiguration` builds.
- `IsHealthy` accepts the Connect JSON enum name `SERVING_STATUS_SERVING`
  as well as `SERVING`; the platform sends the former.
- The Python `timeout=` of the sync methods cancels the call on expiry and
  raises `OpenTDFError(kind='deadline_exceeded')`; the reference has no such
  argument (its `context.Context` carries the deadline).

### Deviations recorded by Phase 2 part 3

Found while exposing the handle API through the generated Java, C#, Rust, C
and Swift packages; part 2's deviations (bytes in and out of `createTdf`,
manifests keyed by the reference's JSON tags, the platform configuration as
a string map) apply to these targets too.

- Java: the error type is `OpenTDFException` (this document's table; the
  plan's mapping says `SDKException`). Manifests, policies and the base key
  are unmodifiable `Map<String, Object>` trees keyed by the JSON tags rather
  than typed classes. Blocking methods take an optional `Duration` and
  every method has an `*Async` twin returning a `CompletableFuture` whose
  `cancel` cancels the call. Token providers reuse the façade's
  `TDF3.TokenProvider`.
- C#: the handle is `Sdk` and the error `OpenTdfException`, as this
  document's examples write them. `Options.Timeout` is a `TimeSpan?` for the
  shared `TimeoutMillis`. Manifests are `IReadOnlyDictionary<string, object>`
  trees keyed by the JSON tags. Methods take `byte[]` until Phase 7's
  streams.
- Rust: the error is a struct `Error` whose `kind` is a `Kind` enum
  (`Kind::Boundary(name)` for the runtime's boundary kinds) carrying the
  shared fields, rather than an enum carrying them. Methods return
  `Result` and block; `*_async` twins return futures, and dropping one
  cancels the call. Manifest types are typed structs whose fields are the
  JSON tags in snake_case.
- C: the header is `opentdf.h` beside the façade's `tdf3.h`. Every call
  fills an optional `otdf_error *` out parameter instead of
  `otdf_error_get(sdk)`, and cancellation and deadlines travel in an
  `otdf_call` (an atomic flag and a millisecond deadline) instead of
  `otdf_cancel(handle)`. Manifests, policies, the platform configuration and
  the base key are returned as JSON text. `*_close` runs Close and releases
  the instance; `*_free` disposes of the handle, so a call on a closed handle
  reports kind 7 as the generated runtime does. The C runtime reports an
  expired libcurl request timeout as a transport failure, so the
  per-request timeout surfaces as `transport` or `deadline_exceeded` on C,
  whichever fires first (a Goalchemy runtime gap, not worked around).
- Swift: `Reader.Init` is `initialize()` (`init` is reserved for
  initializers). The error is a struct `OpenTDFError` whose `kind` is an
  `ErrorKind` enum, not an enum carrying the fields. The shared source's
  `Error` record is emitted as `Error_` by the Swift emitter's public-name
  rule (goalchemy#79); the adapter never exposes it. `SDK.create` is a
  synchronous twin of `try await SDK(...)`.
- An interrupted caller deadline reports the target's cancellation idiom:
  `deadline_exceeded` for a Java `Duration`, `canceled` for a C#
  `CancellationToken`, a dropped Rust future, a C `otdf_call` flag and a
  cancelled Swift `Task`.

### Deviations recorded by Phase 3 part 1

Decided with the user: messages cross the boundary as protobuf JSON, not as
generated value types (generated messages hold pointers, maps and oneof
interfaces the value boundary does not carry, and generating every message
on every target would dominate Swift and Rust build times).

- The shared source exposes one `SDK.Call(ctx, service, method, request)`
  instead of a handle per service client; the Go, TypeScript and Python
  accessors are built over it in the host adapters with each ecosystem's
  protobuf types (table above). `Client` gains `Call`.
- Go accessors are methods, `s.Attributes(callbacks...)`, rather than the
  reference's struct fields (a generated handle cannot gain fields); they
  return the reference's `sdkconnect` interfaces. `EntityResoution` (the
  reference's spelling) and `EntityResolution` both exist; the reference's
  unexported `wellknownConfiguration` is `WellKnown`.
- Go service errors are `*Error` with `ConnectCode`, not `*connect.Error`:
  the generated package does not depend on connect-go.
- The TypeScript Transport rejects with `ConnectError` (Connect-ES clients
  and interceptors expect it) whose `cause` is the `OpenTDFError`; `sdk.call`
  and every other method throw `OpenTDFError`.
- The health check (`grpc.health.v1.Health/Check`, not in the platform
  protos) is sent as JSON through the same Connect client as `Call`.

### Deviations recorded by Phase 3 part 2

- Java and C# accessors live on a generated abstract base class,
  `PlatformServices`, that the hand-written `SDK`/`Sdk` extends, as the
  TypeScript `SDK` merges the generated `PlatformServices` interface: the
  generated client code stays out of the hand-written adapter. The Java
  interfaces keep the reference names (`AttributesServiceClient`); C# adds the
  `I` prefix (`IAttributesServiceClient`).
- Java messages are generated with `optimize_for = CODE_SIZE` (buf managed
  mode): the JSON codec works through descriptors and reflection either way,
  and the generated sources shrink by a fifth. Java and C# carry
  `buf/validate` under the SDK's own package and namespace, as Python and
  TypeScript do; the Rust and Swift messages do not reference it and omit it.
- Rust and Swift clients are concrete types named after the reference's
  interfaces (`AttributesServiceClient`), not traits or protocols. Rust spells
  the unsafe service's accessor `r#unsafe()`; Swift `` `unsafe` ``.
- `google.protobuf.Any` (entity claims, entity resolution error details,
  `authorization` request extensions): Java and C# resolve it through the
  type registry of every service file; Swift resolves only types registered
  with SwiftProtobuf; Rust's `pbjson-types` reads and writes it as
  `{"typeUrl","value"}` rather than protobuf JSON's embedded form, so a Rust
  caller passes such messages through `sdk.call` with JSON.
- C has no typed messages (#29): `otdf_sdk_call` and the 131 generated
  `otdf_<service>_<method>` functions take and return protobuf JSON text.

### Deviations recorded by Phase 3 part 3

- `ListAttributes` and `GetEntityAttributes` keep the reference's typed
  signatures in the shared source; the hosts reach them through
  `ListAttributesJSON` and `GetEntityAttributesJSON` (protobuf JSON, as
  `SDK.Call`), so every host returns its own `policy.Attribute`. C returns
  the JSON array.
- `ValidateAttributes` takes a slice: a variadic parameter does not cross the
  boundary, and in Go the generated method's trailing variadic is the host
  callbacks. The Go adapter's `ListAttributes` takes `[]string` for the same
  reason; more than one namespace still fails with the reference's
  `ListAttributes accepts at most one namespace filter, got N`.
- `AuthClient.AccessToken` returns the SDK's `AccessToken` (value, scheme,
  expiry, DPoP thumbprint) rather than the reference's string. In Go,
  `SDK.Auth`, `SDK.Conn` and `AuthClient.AccessToken` take `ctx` and return
  an error, as every generated method; the `AuthClient` handle has `Close`.
  The other hosts' `auth()` is local and each `accessToken()` opens and
  closes the shared source's `AuthClient` handle.
- `ConnectRPCConnection` is the endpoint only: every connection uses the
  SDK's authenticated transport, so the reference's `Client` and connect
  `Options` have no equivalent (`WithExtraClientOptions` is not planned).
  `SDK.Conn` returns the core endpoint.
- The reference's deprecated `WithCustomPolicyConnection`,
  `WithCustomAuthorizationConnection` and `WithCustomWellknownConnection`
  set its single core connection; here they are per-service overrides with
  the core endpoint as their fallback, as the plan specifies. With only
  `WithCustomCoreConnection` the routing is the reference's. An override is
  validated as the platform endpoint is.
- The reference's `ErrAttributeNotFound` is reported only when the service
  answers without a requested FQN; the platform at the pinned revision
  answers `not_found` for any missing value, so `ValidateAttributes` there
  fails with `validating attributes: not_found: resource not found`, as the
  reference does.
- `NewAttributeNameFQN` and `NewAttributeValueFQN` stay unexported
  (`newAttributeNameFQN`) until Phase 4 exports the FQN types with the
  granter; `ErrInvalid` is exported now.

### Deviations recorded by Phase 4 part 1

- `AttributeNameFQN` and `AttributeValueFQN` export their fields as `URL` and
  `Key` (the reference's unexported `url` and `key`): a struct with
  unexported fields does not cross the Goalchemy boundary (GCE007).
- `DefaultKeySplitter` is unexported (`defaultKeySplitter`) until the chunked
  writer of Phase 10 takes it: a function returning the `KeySplitter`
  interface cannot be a library export. `CreateTDF` does not use a
  `KeySplitter`, as in the reference; its shares come from `splitDEK`.
- `TDFOptions.AttributeValues` is `[]string`, each a `policy.Value` in
  protobuf JSON, like `SDK.Call`; setting it together with `Attributes` is an
  error rather than the reference's last-option-wins (the Go adapter keeps
  the reference's ordering).
- `TDFOptions.Attributes` are parsed with `NewAttributeValueFQN` when the TDF
  is created, as the reference's `WithDataAttributes` does when the option is
  applied, so an invalid FQN fails with `invalid type: ...`.
- `KeyTypeToPolicyAlgorithm` and `PolicyAlgorithmToKeyType` keep the
  protojson names of `policy.Algorithm` at the boundary (the generated enum
  type does not cross); the shared source uses the typed enum. They know the
  post-quantum key types; a KAO for one fails with `unsupported in this
  build: wrapping key algorithm <type>`.
- The reference's `disjunction.Less` (unused, and indexing one past the end)
  is replaced by `sort.Interface` methods; `sortedNoDupes` sorts with
  Goalchemy's `sort.Sort`, which runs the same insertion sort as Go's
  `sort.Slice` for the short lists a plan has.
- The exported FQN and resource locator constructors return the SDK's
  `Error` (kind `invalid_input`) with the reference's text.
