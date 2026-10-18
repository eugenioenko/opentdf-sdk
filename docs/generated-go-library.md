# Generated Go library

The shared source compiles into an importable Go package: module
`goalchemyout`, package `generated`, emitted from the two export roots
`src/library` (the deprecated `Encrypt`/`Decrypt` façade) and `src` (the
handle API of [api-spec.md](api-spec.md)). The emitted package imports its
bundled standard-library capabilities; it never imports the original SDK
implementation.

## Build and import

From `goalchemy/`, use `GOTOOLCHAIN=go1.25.14 go build -o
out/go-tdf-library/goalchemy ./cmd/goalchemy`, then run
`scripts/build-generated-go.sh`. An optional absolute destination argument
selects another output directory; `GOALCHEMY_BIN` selects a compiler binary.
The script compiles both roots from the SDK module, copies the hand-written
host adapters from `src/hosts/go` (`token.go.in`, `options.go.in`,
`errors.go.in`), formats them and builds the package. It requires Go 1.25 on
a 64-bit host.

A separate native consumer can require `goalchemyout v0.0.0` and replace
`goalchemyout` with the generated directory. Import it as `sdk "goalchemyout"`.
`tests/interop/generatedlibrary` is the independent consumer; its `controlled`
mode runs offline checks against an httptest platform (discovery failures,
deadlines, allowlist rejection, invalid input, unsupported options, handle
release) and the other modes drive the real platform.

## Handle API

Every generated call takes a `context.Context` first and returns `error`
last; a method that fails after the handle was closed returns a
`*LibraryError` with kind `closed`.

```go
s, err := sdk.NewWithOptions(ctx, "http://localhost:8080",
    sdk.WithClientCredentials("opentdf", "secret", nil))
defer s.Close()

opts, _ := sdk.TDFOptionsOf(
    sdk.WithKasInformation(sdk.KASInfo{URL: "http://localhost:8080/kas"}),
    sdk.WithDataAttributes("https://example.com/attr/attr1/value/value1"),
    sdk.WithMimeType("text/plain"))
ciphertext, obj, err := s.EncryptBytes(ctx, plaintext, opts)
manifest, _ := obj.Manifest(ctx)
obj.Close()

ropts, _ := sdk.ReaderOptionsOf(sdk.WithSessionKeyType("ec:secp256r1"))
plaintext, err = s.DecryptBytes(ctx, ciphertext, ropts)

r, err := s.LoadTDF(ctx, file, sdk.ReaderOptions{})
attrs, _ := r.DataAttributes(ctx)
if err := r.Init(ctx); err == nil { meta, _ := r.UnencryptedMetadata(ctx) }
r.Close()
```

| Generated symbol | Reference | Notes |
| --- | --- | --- |
| `New(ctx, endpoint, Options)`, `NewWithOptions(ctx, endpoint, ...Option)` | `New` | Fetches `/.well-known/opentdf-configuration` (Connect reply or the bare document) and the issuer's OpenID configuration unless `PlatformConfiguration` is given; `ValidateConnection` runs the health check first. |
| `SDK.EncryptBytes`, `SDK.CreateTDF(ctx, io.Writer, io.Reader, TDFOptions)` | `CreateTDFContext` | `CreateTDF` currently buffers the complete payload and archive. Key access templates from autoconfigure or explicit KAS information, split across KAS; RSA-2048/4096 and P-256/384/521 wrapping. |
| `SDK.LoadTDF(ctx, io.ReadSeeker, ReaderOptions)`, `SDK.DecryptBytes` | `LoadTDF`, `DecryptBytes` | One rewrap request per KAS; the DEK is the XOR of one share per split id, any KAO of a split will do, duplicates are skipped; post-quantum KAOs fail as `unsupported_key_access`. RSA and EC sessions of every non-PQ key type. The KAS allowlist comes from the registry unless `KasAllowlist` or `IgnoreAllowlist` is set, as in the reference. |
| `Reader.Init`, `Manifest`, `Policy`, `DataAttributes`, `UnencryptedMetadata`, `Close` | `Reader` | Payload streaming is not implemented. |
| `WithAssertions`, `WithSystemMetadataAssertion`; `WithAssertionVerificationKeys`, `WithDisableAssertionVerification` | same | Assertions are signed into the manifest with HS256 over the payload key or the config's `SigningKey` (HS256 secret or RS256 private PEM) and verified on read by default, by assertion ID with a default key, or with the payload key. `GetSystemMetadataAssertionConfig` adds the `system-metadata` assertion. |
| `TDFObject.Manifest`, `Size`, `Close` | `TDFObject` | |
| `SDK.IsHealthy`, `PlatformConfiguration`, `GetBaseKey`, `StoreKASKeys`, `Close` | same | `GetBaseKey` returns `SimpleKasKey` by value. |
| `IsValidTdf`, `GetTdfType`, `IsPlatformEndpointMalformed`, `KeyTypeToPolicyAlgorithm`, `PolicyAlgorithmToKeyType`, `SupportedFeatures`, `Version`, `TDFSpecVersion` | same | Free functions. |

Options are the shared-source structs `Options`, `TDFOptions` and
`ReaderOptions`; every zero value is the reference default. The host adapter
provides the reference's `With*` functions over them (`OptionsOf`,
`TDFOptionsOf`, `ReaderOptionsOf`), including `WithPlatformConfiguration`
over a `map[string]any` and `WithPolicyFrom(ctx, reader)`. `WithLogger`
installs the process-wide host log sink (`goalchemyout/cap/log`) for the
given `*slog.Logger`; `Options.LogLevel` selects the level. Options this
build does not implement yet (`TokenExchange`, `CertExchange`, `TLS`,
`UnixSocket`, target mode `4.2.2`,
schema validation, post-quantum key types) fail with
`Kind: unsupported_feature` and the message `unsupported in this build: <option>`.

Authentication follows the reference: client credentials with HTTP Basic
authentication, a DPoP proof on every token request (ephemeral ES256 key by
default, `WithDPoPAlgorithm` or `WithDPoPKeyPEM` otherwise), and
`Authorization: DPoP` with an `ath`-bound proof on resource requests when the
IdP issued a DPoP token, `Bearer` otherwise. A host token provider is
registered with `TokenCallbacks` and selected with `WithTokenProvider()` (or
`Options.TokenProviderName = TokenProviderName`).

## Service clients

`s.Attributes()`, `s.Namespaces()` and the other accessors return the
reference's `sdkconnect` interfaces over the reference's own messages from
`github.com/opentdf/platform/protocol/go` (v0.41.0, required by the
package's `go.mod`). Each call is protobuf JSON through `SDK.Call` and the
SDK's authenticated transport. Pass the token provider's callbacks to the
accessor when the SDK uses one.

```go
import "github.com/opentdf/platform/protocol/go/policy/attributes"

resp, err := s.Attributes().ListAttributes(ctx, &attributes.ListAttributesRequest{Namespace: "example.com"})
var e *opentdf.Error
if errors.As(err, &e) && e.ConnectCode == "not_found" { /* ... */ }
out, err := s.Call(ctx, "policy.namespaces.NamespaceService", "ListNamespaces", "{}")
```

The reference's helpers are methods of the handle: `s.ListAttributes(ctx,
[]string{ns})` returns `[]*policy.Attribute` of `protocol/go`,
`s.GetEntityAttributes(ctx, *authorization.Entity)`, `s.ValidateAttributes`,
`s.AttributeExists` and `s.AttributeValueExists`. `s.Auth(ctx)` returns an
`*AuthClient` whose `AccessToken(ctx)` is the SDK's token (close it after
use), `s.Conn(ctx)` the core endpoint. `WithCustomCoreConnection` and the
policy, authorization, entity resolution and well-known variants take a
`*ConnectRPCConnection{Endpoint}` and route those services there through the
SDK's transport (see [api-spec.md](api-spec.md#helpers-auth-and-endpoint-overrides-phase-3-part-3)).

```go
exists, err := s.AttributeValueExists(ctx, "https://example.com/attr/level/value/high")
s2, err := sdk.NewWithOptions(ctx, platform, sdk.WithCustomCoreConnection(&sdk.ConnectRPCConnection{Endpoint: "https://core.example.com"}))
```

With autoconfigure on (the default), `WithDataAttributes` drives the
reference's key split: grants and mapped keys of the attributes decide the
key access objects, else the base key, else the default KAS of
`WithKasInformation` (see [api-spec.md](api-spec.md#autoconfigure-and-key-splitting-phase-4-part-1)).
`WithDataAttributeValues(...*policy.Value)` plans offline from values that
carry their definitions. `NewAttributeValueFQN(ctx, fqn)` and
`NewAttributeNameFQN` return the FQN values with the reference's methods.

```go
o, err := sdk.TDFOptionsOf(sdk.WithDataAttributes("https://example.com/attr/level/value/high"))
v, err := sdk.NewAttributeValueFQN(ctx, "https://example.com/attr/level/value/high") // v.Value() == "high"
```

## Errors

SDK failures are `*sdk.Error` values with `Kind`, `Op`, `Message`,
`HTTPStatus`, `ConnectCode`, `ServiceMessage`, `KASURL` and
`RequiredObligations`. `Message` is the reference SDK's wording for the whole
chain (see [error-wording.md](error-wording.md)); the native sentinels in
`errors.go.in` (`ErrTampered`, `ErrRootSigValidation`, `ErrRewrapForbidden`,
`ErrTDFNotDecryptable`, `ErrPlatformUnreachable`, ...) and the `Kind*`
constants let callers write `errors.Is(err, sdk.ErrTampered)` or switch on
`Kind`. The record carries no cause: interface fields do not cross the
boundary. Cancellation of the native call itself is a `*LibraryError` with
kind `canceled`, as before.

## Deprecated façade

`Encrypt(ctx, Config, []byte, EncryptOptions)` and `Decrypt(ctx, Config, []byte)`
remain with their previous shapes for one release, implemented as thin
wrappers that construct an SDK per call: `Config.IssuerURL` with
`Config.TokenURL` becomes the provided platform configuration (alone, the
SDK discovers both from the platform), `Config.KASURL` the KAS information
and the allowlist, `Config.DPoP` the DPoP algorithm. Their `Failure.Code` is now the
`Error.Kind` string. `PolicyBase64`, `Dissem` and `SegmentHashAlgorithm` other
than `GMAC` are no longer supported.

## CLI

`src/hosts/go/cmd/otdfgc` is the native CLI over this package (built by
`scripts/build-go-cli.sh`) that the xtest harness drives as the `gcgo` SDK
type; see [xtest.md](xtest.md). It constructs one SDK from `--host` and
`--with-client-creds`, encrypts with the platform's KAS as the key access
server and `--attr` as data attributes, decrypts with `--kas-allowlist`
(`*` ignores the allowlist) and `--session-key-algorithm`, and prints the
SDK's error text after otdfctl's `Failed to encrypt:` / `Failed to decrypt
file:` prefixes.
