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
| `SDK.EncryptBytes`, `SDK.CreateTDF(ctx, io.Writer, io.Reader, TDFOptions)` | `CreateTDFContext` | `CreateTDF` reads the payload whole in this phase; Phase 7 streams it. Single-KAS templates; RSA-2048 and P-256 wrapping. |
| `SDK.LoadTDF(ctx, io.ReadSeeker, ReaderOptions)`, `SDK.DecryptBytes` | `LoadTDF`, `DecryptBytes` | The KAS allowlist comes from the registry unless `KasAllowlist` or `IgnoreAllowlist` is set, as in the reference. |
| `Reader.Init`, `Manifest`, `Policy`, `DataAttributes`, `UnencryptedMetadata`, `Close` | `Reader` | Stream methods arrive in Phase 7. |
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
`UnixSocket`, service endpoint overrides, assertions, target mode `4.2.2`,
schema validation, RSA-4096 and P-384/P-521 keys) fail with
`Kind: unsupported_feature` and the message `unsupported in this build:
<option>`.

Authentication follows the reference: client credentials with HTTP Basic
authentication, a DPoP proof on every token request (ephemeral ES256 key by
default, `WithDPoPAlgorithm` or `WithDPoPKeyPEM` otherwise), and
`Authorization: DPoP` with an `ath`-bound proof on resource requests when the
IdP issued a DPoP token, `Bearer` otherwise. A host token provider is
registered with `TokenCallbacks` and selected with `WithTokenProvider()` (or
`Options.TokenProviderName = TokenProviderName`).

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
