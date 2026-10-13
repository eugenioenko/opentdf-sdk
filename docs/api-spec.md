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
| `sdk/protocol/...` | generated messages and Connect clients from `../platform/protocol` | `sdkconnect/`, `protocol/go` |
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
func (s *SDK) ListAttributes(ctx context.Context, namespaces []string) ([]*policy.Attribute, error)
func (s *SDK) GetEntityAttributes(ctx context.Context, entity *authorization.Entity) ([]string, error)
func (s *SDK) ValidateAttributes(ctx context.Context, fqns []string) error
func (s *SDK) AttributeExists(ctx context.Context, fqn string) (bool, error)
func (s *SDK) AttributeValueExists(ctx context.Context, fqn string) (bool, error)
func (s *SDK) Auth() *AuthClient // AccessToken(ctx) (AccessToken, error)

// Service clients (generated; see Service clients).
func (s *SDK) Actions() *actions.ActionServiceClient
func (s *SDK) Attributes() *attributes.AttributesServiceClient
func (s *SDK) Authorization() *authorization.AuthorizationServiceClient
func (s *SDK) AuthorizationV2() *authorizationv2.AuthorizationServiceClient
func (s *SDK) EntityResolution() *entityresolution.EntityResolutionServiceClient
func (s *SDK) EntityResolutionV2() *entityresolutionv2.EntityResolutionServiceClient
func (s *SDK) KeyAccessServerRegistry() *kasregistry.KeyAccessServerRegistryServiceClient
func (s *SDK) KeyManagement() *keymanagement.KeyManagementServiceClient
func (s *SDK) Namespaces() *namespaces.NamespaceServiceClient
func (s *SDK) Obligations() *obligations.ObligationsServiceClient
func (s *SDK) RegisteredResources() *registeredresources.RegisteredResourcesServiceClient
func (s *SDK) ResourceMapping() *resourcemapping.ResourceMappingServiceClient
func (s *SDK) SubjectMapping() *subjectmapping.SubjectMappingServiceClient
func (s *SDK) DynamicValueMapping() *dynamicvaluemapping.DynamicValueMappingServiceClient
func (s *SDK) Unsafe() *unsafe.UnsafeServiceClient
func (s *SDK) WellKnown() *wellknown.WellKnownServiceClient
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
	AttributeValues       []*policy.Value // WithDataAttributeValues
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

Generated from `../platform/protocol` by `protoc-gen-goalchemy` into
`src/protocol/<service>` and wired to the SDK's authenticated transport:
Bearer or DPoP header, nonce retry, timeout, endpoint overrides.

Shared source shape (reference `sdkconnect` plus `protocol/go`):

```go
resp, err := s.Attributes().GetAttributeValuesByFqns(ctx, &attributes.GetAttributeValuesByFqnsRequest{
	Fqns:      fqns,
	WithValue: &attributes.AttributeValueSelector{WithKeyAccessGrants: true},
})
if connect.CodeOf(err) == connect.CodeNotFound { ... }
```

Rules:

- Message types are the generated structs with protojson field names; they
  cross the boundary as generated value types per target (the same
  machinery as `protoc-gen-goalchemy` uses for the `proto_platform`
  fixture).
- Each client is a handle on the SDK instance; it has one method per unary
  RPC taking the request message and returning the response message or an
  error. Streaming RPCs are not generated (none of the policy services need
  them).
- Service errors are `*connect.Error` in shared source and surface as
  `Error{Kind: KindTransport | KindAuth | ..., ConnectCode: "not_found"}`
  with `Message` equal to the Connect message.
- Per-call headers and deadlines come from `ctx`; there is no per-client
  configuration beyond the endpoint overrides in `Options`.

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
| Service clients | `s.Attributes()` returning generated client | `sdk.attributes.getAttribute(req)` | `sdk.attributes.get_attribute(req)` | `sdk.attributes().getAttribute(req)` | `sdk.Attributes.GetAttributeAsync(req)` | `sdk.attributes().get_attribute(req)` | `otdf_attributes_get_attribute(sdk, req_json, &resp_json)` | `sdk.attributes.getAttribute(req)` |
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
    Attribute a = sdk.attributes().getAttribute(GetAttributeRequest.newBuilder().setFqn(fqn).build());
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
let attr = sdk.attributes().get_attribute(&GetAttributeRequest { fqn, ..Default::default() })?;
```

C:

```c
otdf_sdk *sdk; otdf_options o = OTDF_OPTIONS_INIT; o.client_id = id; o.client_secret = secret;
OTDF_CHECK(otdf_sdk_new("https://platform.example", &o, &sdk));
otdf_tdf_options t = OTDF_TDF_OPTIONS_INIT; t.attributes = &fqn; t.attribute_count = 1;
OTDF_CHECK(otdf_create_tdf(sdk, &out_stream, &in_stream, &t, &obj));
OTDF_CHECK(otdf_load_tdf(sdk, &in_stream, NULL, &reader));
while ((n = otdf_reader_read(reader, buf, sizeof buf)) > 0) { ... }
OTDF_CHECK(otdf_attributes_get_attribute(sdk, "{\"fqn\":\"...\"}", &resp_json));
```

Swift:

```swift
let sdk = try await SDK("https://platform.example", Options(clientId: id, clientSecret: secret))
let obj = try await sdk.createTDF(to: out, from: input, options: TDFOptions(attributes: [fqn]))
let reader = try await sdk.loadTDF(file)
for try await chunk in reader { ... }
let attr = try await sdk.attributes.getAttribute(GetAttributeRequest(fqn: fqn))
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
