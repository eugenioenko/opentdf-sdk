# Generated Go TDF3 library

The shared façade is [`library`](../library/library.go). Goalchemy lowers its
Encrypt and Decrypt exports and all reachable SDK code into an importable Go
package. The emitted package imports its bundled standard-library capabilities;
it never imports the original SDK implementation. Other target libraries and
non-Go production HTTP/crypto adapters remain unavailable.

## Build and import

From `goalchemy/`, use `GOTOOLCHAIN=go1.25.14 go build -o
out/go-tdf-library/goalchemy ./cmd/goalchemy`, then run
`../sdk/scripts/build-generated-go.sh`. An optional absolute destination argument
selects another output directory; `GOALCHEMY_BIN` selects a compiler binary.
The script runs compilation from the SDK module, copies the small native token
wire adapter, formats it and builds the emitted package. It emits module
`goalchemyout`, package `generated`, requiring Go 1.25 on a 64-bit host.

A separate native consumer can require `goalchemyout v0.0.0` and replace
`goalchemyout` with the generated directory. Import it as `tdf "goalchemyout"`.
For distribution, choose a module path and update `go.mod` and the generated
`goalchemyout/rt` and `goalchemyout/cap/...` imports together. The current build
script creates a reviewable local package; it does not publish an artifact.

```go
config := tdf.Config{
    PlatformURL: "https://platform.example",
    IssuerURL: "https://issuer.example/realm",
    ClientID: clientID, ClientSecret: clientSecret,
}
archive, err := tdf.Encrypt(ctx, config, payload, tdf.EncryptOptions{
    Metadata: metadata, IncludeMetadata: true,
})
result, err := tdf.Decrypt(ctx, config, archive)
// result.Payload, result.Metadata, result.HasMetadata, result.ManifestJSON
```

Config preserves trusted KAS routes, KAS URL/kid, RSA/P-256 wrapping and session
algorithms, RS256/ES256 auth, DPoP, timeout, token endpoint and server credentials.
Optional KASPublicKeyPEM and AuthPrivateKeyPEM are imported for each operation;
the façade owns and closes those imports. Session and authentication keys stay
distinct. A DPoP token must match the imported authentication key. No native key
handles or persistent clients are exposed.

EncryptOptions preserves policy base64, attributes/dissemination, MIME type,
GMAC/HS256 segmentation, int64 SegmentSize/HasSegmentSize and metadata presence.
The 64-bit source int conversion keeps the shared clamping behavior. Timeout
and token expiry retain int64 ranges. Raw payload and metadata bytes never pass
through text/JSON conversion. Go strings retain their byte representation;
protocol JSON enforces its existing string rules. HasMetadata distinguishes
absent from explicitly encrypted empty metadata. ManifestJSON represents the
validated supported manifest, with shared canonical serialization.

## Native token providers

Server hosts may use credentials. Hosts with an independent token provider set
Config.TokenProviderName to `tdf.TokenProviderName` and pass
`tdf.TokenCallbacks(provider)` as the optional final argument to Encrypt/Decrypt.
TokenProvider is `func(context.Context) (AccessToken, error)`; AccessToken retains
Value, Scheme, int64 ExpiresAt and ConfirmationJKT. The provider runs on the
owned host worker, can block on cancellation-aware native I/O, and must observe
its context. Rejections are declared provider errors; panics are host faults.
The shared SDK validates token expiry, scheme and DPoP binding and owns protocol
behavior. The provider owns refresh; no token cache survives an operation.

The generic asynchronous Callback/Callbacks boundary is also available. A
Callback receives native context, copied request bytes and a settlement function,
and returns a nonblocking cancellation hook. The scoped callback name resolves
only in the current operation's copied registry. For token acquisition, request
is `{}` and response is bounded strict JSON:

```json
{"value":"opaque-token","scheme":"Bearer","expiresAt":"9223372036854775807","confirmationJKT":""}
```

Expiry is an unsigned decimal string bounded by signed int64; JSON numbers are
rejected. Scheme is preserved and must agree with Config.DPoP. ConfirmationJKT
attests opaque DPoP tokens; JWTs still need matching cnf.jkt claims. Unknown and
duplicate response fields are rejected. Token JSON is bounded to 128 KiB and
individual strings to 64 KiB. The generic callback capability bounds names to
128 bytes and request/reply buffers to 1 MiB.

Exactly the first settlement is accepted; reply bytes are copied before settle
returns. Settlement acknowledges that provider-owned resources have been
released. Cancellation requests the hook and waits for that acknowledgment.
If the hook panics, the bridge retains ownership until settlement, then reports
a host fault. Providers must settle after cancellation. If submission itself
panics before returning a hook, the provider must already have cleaned any
unreported asynchronous acquisitions; the bridge cannot retire external
resources that were never reported. Callback threads never run source frames.
Duplicate/late completions cannot mutate source or a subsequent operation.

## Ownership, cancellation and errors

All exports in one generated package serialize through a cancellation-aware
reservation. Native byte/config/option trees and callback registries are copied
before queue acquisition. Canceling a queued caller leaves the active operation
untouched. Each acquired call creates a fresh scheduler and initializes source
packages exactly once in that owner; it resets source globals on retirement.
It does not expose persistent mutable source instances. Retirement cancels
pending native work, waits for worker/resource acknowledgments, closes all
façade-generated/imported keys and clears source/runtime registrations before
releasing the reservation. Results are independently copied and remain valid
after later calls. Nil and explicit empty slices are preserved at the public
value boundary. Callers must avoid concurrent mutation while entry snapshots
are being taken, as with any Go slice-reading API.

The earliest native caller deadline and inherited source timeout reach native
HTTP/provider contexts. Cancellation is observed at cooperative driver
boundaries; a non-yielding source computation is not preempted. A native crypto
acquisition already running finishes before cleanup/release. Nested invocation
of this package from its own token provider is unsupported because the provider
holds the active reservation.

Use `errors.As` with `*tdf.Failure` for declared SDK errors: Code, Operation,
HTTPStatus, ServerCode, ServerMessage, RequiredObligations and CauseCategory.
CauseCategory is `canceled`, `deadline_exceeded`, or `source`; source errors are
bounded value records and do not promise native cause identity. ServerMessage
is available for reviewed inspection and omitted from Error(). Native caller
cancellation uses `*tdf.LibraryError`, Kind `canceled`, with the native context
sentinel available through errors.Is. LibraryError separately categorizes
source_panic, host_fault, deadlock and invalid native boundary arguments. Panic
values are omitted to avoid exposing credentials/key material. Unrecovered
source panic and host faults return after cleanup; they never exit the importing
process. Decrypt failures return no partial successful payload/metadata.

## Verification scope

Compiler integration tests create a separate native importing module and prove
actual generated exports, value ownership/init, real HTTP, queued/active
cancellation, provider success/rejection/duplicate/late completion, gated cleanup
ACK after stop-hook panic, and source panic versus host fault. Runtime tests
check imported/generated key retirement after success, source/host fault and
constructor/init failure. SDK native tests check exact token wire ranges and
stable error fields. The independent live consumer/reference sources are under
`tests/interop/generatedlibrary` and `tests/interop/generatedreference`.

Generated Go profile/negative/KAS acceptance and all-seven SDK delivery must be
reported from terminal evidence separately. This boundary does not complete
Phase 4 or the seven-target delivery goal.

Successful value/error ownership conversion runs inside the active owner before
source-global reset, runtime retirement or reservation release. This includes
slices backed by source-global fixed arrays. Public struct value-tree errors
retain their concrete type with copied slice fields; immutable native
errors.New/context sentinel identity is preserved. Arbitrary error layouts
containing private fields or opaque/pointer/interface state are outside the
bounded export ABI and become LibraryError with Kind `unsupported_error`.
The shared SDK façade always converts declared failures to its supported public
Failure record before reaching this boundary.
