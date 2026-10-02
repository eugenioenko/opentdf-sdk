# Shared native SDK client

This accepted Phase 3 implementation provides a shared SDK client for Bearer or sender-constrained DPoP authentication, discovery, RSA/P256 TDF creation and real KAS rewrap. Shared source imports only declared Goalchemy libraries and the shared TDF packages. It passes the cooperative source gate. Generated HTTP remains unavailable (`GCE002: lib.http.do`) until Phase 4; this acceptance covers native Go. Generated SDKs on all seven targets and full Go SDK parity remain pending.

## API and lifecycle

`New(Config)` validates configuration and generates a distinct P-256 authentication signing key by default. `AuthAlgorithm: "RS256"` selects RSA-2048 instead; supplied `AuthKey` handles must match the algorithm. `Close` cancels in-flight HTTP and token-provider contexts, clears token and nonce caches, and closes only the generated authentication key. It is idempotent and safe on nil or zero clients. Operations reject uninitialized/closed clients. Supplied auth and KAS keys remain caller-owned pointers, and are never copied by dereferencing their opaque handle. A returned `PublicKey` handle belongs to the caller.

`Create(ctx, plaintext, tdf.EncryptConfig)` creates a bounded in-memory archive. Options supply policy/attributes, segmentation, MIME type and metadata; the client supplies its configured KAS destination, discovered/explicit wrapping key and kid. Set `KASPublicKey` and `KID` on `Config` to create files without authentication network calls or discovery. `New` still validates an authentication configuration for later client operations; callers needing a purely offline writer can use `tdf.Encrypt` directly. `Decrypt(ctx, archive)` validates the archive and supported manifest profile, authorizes the manifest KAS destination, obtains a token, creates a fresh RSA-2048 or P256 session key, signs and sends the actual rewrap request, unwraps the returned share, then authenticates all payload and metadata integrity through the engine. Failures return no plaintext or metadata. Session private handles close on every normal error/success path, and recovered key buffers receive best-effort zeroing after use. Native key generation/CPU encryption are synchronous; context cancellation is checked before and after these operations. HTTP/provider work honors cancellation while pending.

Example using the local development stack:

```go
c, err := sdk.New(sdk.Config{
    PlatformURL: "http://localhost:8080",
    KASURL: "http://localhost:8080/kas",
    IssuerURL: "http://localhost:8888/auth/realms/opentdf",
    ClientID: "opentdf-sdk", ClientSecret: "secret", AllowHTTP: true,
})
if err != nil { return err }
defer c.Close()
archive, err := c.Create(ctx, []byte("example"), tdf.EncryptConfig{
    Attributes: []string{"https://example.com/attr/attr1/value/value1"},
})
if err != nil { return err }
result, err := c.Decrypt(ctx, archive)
```

`Error` carries `Code`, `Operation`, `HTTPStatus`, `ServerCode`, optional `ServerMessage`, `RequiredObligations` and an unwrap-able `Cause`. `Error()` omits server text and secrets. Hosts may inspect bounded server diagnostics; logging them requires host review. Transport errors, HTTP authentication/denial, per-KAO failure, unsupported profiles, protocol mismatch, obligations and integrity failures remain distinguishable. Every per-KAO `fail` retains its error string as `rewrap_failed`; it is not automatically classified as policy denial.

## Authentication and discovery

Client credentials use `application/x-www-form-urlencoded` with correctly percent-encoded UTF-8 `client_id` and `client_secret`, plus `grant_type=client_credentials`. Credentials stay in server-side configuration. Responses require a nonempty printable token, a scheme matching `Config.DPoP` (`Bearer` or `DPoP`), and integer `expires_in` between 1 and 86,400 seconds. Cache expiry uses declared real UTC wall time, with a five-second refresh margin. A cancellation-aware gate serializes acquisition, preventing refresh races or redundant parallel acquisition. A final HTTP 401 invalidates the cache. Bearer requests are never replayed. DPoP requests may replay once on a distinct valid nonce challenge before final status handling; this is proof refresh, not token refresh. Provider acquisition and each HTTP request use `TimeoutMillis` (default 15,000; range 1–300,000), caller cancellation/deadline, and client lifetime cancellation. Providers must honor the supplied context; the SDK cannot forcibly stop a host callback that ignores cancellation.

Browser hosts supply `TokenProvider func(context.Context) (AccessToken, error)`, with `Value`, `Scheme: "Bearer"` and a future UTC Unix `ExpiresAt`. This path requires no client secret. The provider handles host authentication and refresh; the SDK caches only until its reported expiration margin. With `DPoP: true`, supply `Scheme: "DPoP"` and a matching caller-owned `AuthKey`. JWT tokens must contain matching `cnf.jkt`; an opaque token requires the provider's explicit `ConfirmationJKT` attestation. JWT decoding checks key-binding consistency and does not verify token signatures/issuer authority locally. The trusted provider and resource server own that authority. [DPoP details and evidence](dpop-client.md) specify the exact opaque-token contract, proof fields, nonce retry and trust boundary. A scheme mismatch fails without downgrade. Unsupported auth signing algorithms fail explicitly.

Without an explicit `TokenURL`, the client fetches `PlatformURL + "/.well-known/opentdf-configuration"`, checks `idp.issuer` against the caller's expected `IssuerURL`, and accepts only the conventional token route `IssuerURL + "/protocol/openid-connect/token"`. This profile targets the tested Keycloak/platform metadata. Alternative issuer/token route layouts require an explicit caller-trusted `TokenURL`. Discovered metadata never extends the credential allowlist. With a provider, OAuth discovery is unnecessary.

Wrapping-key discovery uses Connect unary JSON `POST /kas.AccessService/PublicKey`, `{algorithm:Config.KASAlgorithm,fmt:"pkcs8",v:"2"}`. `KASAlgorithm` accepts `rsa:2048` (default) or `ec:secp256r1`. The pinned server's `pkcs8` wire label returns SPKI PEM. Wire keys require `BEGIN PUBLIC KEY`; private PEM, certificates, unsupported curves and mismatched types fail. `PublicJWK` validates RSA-2048 modulus size or P256 curve/coordinates before use. `PublicKey` always discovers the configured algorithm and returns a caller-owned handle, even when configuration supplies an explicit creation key. The shared client does not autoconfigure policy-service attributes or register/discover arbitrary KAS destinations.

`Config.SessionAlgorithm` independently selects `rsa:2048` (default) or `ec:secp256r1` for each decrypt response. It is independent of `KASAlgorithm`, the archive's KAO type and `AuthAlgorithm` (RS256/ES256). A configured EC writer can read RSA KAOs and vice versa; reader routing and the manifest determine the KAS request. Each decrypt creates and closes its own session private handle. RSA responses use OAEP-SHA1 and require absent or empty-string `sessionPublicKey`; other types or nonempty values fail. EC responses require a top-level P256 SPKI `sessionPublicKey`. ECDH with the private session, HKDF-SHA256 with salt SHA256(`TDF`), empty info and 32 output bytes derives an AES-256-GCM key. The returned share frame is `nonce12 || ciphertext32 || tag16` without AAD. Invalid session material, framing, authentication or recovered share length returns zero plaintext and metadata.

`Create` uses `Config.KASAlgorithm` consistently for discovery and encryption. An explicit `EncryptConfig.Algorithm` must match it; unsupported or conflicting values fail. Explicit `Config.KASPublicKey` must match that algorithm. `Config.KID` requires an explicit key. Creation options cannot replace the client's KAS key, destination or kid: a supplied pointer must be the same configured handle and supplied URL/kid must match configuration. Set wrapping configuration on `Config`; standalone engine writers may configure keys directly through `tdf.Encrypt`. Closed caller-owned key handles fail on use without changing ownership.

## Credential destinations and routing

`PlatformURL` is the platform metadata base. `KASURL` is the primary manifest destination, defaulting to platform base + `/kas`. `AllowedKAS` contains explicit `KASRoute{URL, APIBaseURL}` entries; when omitted it contains only the primary KAS mapped to the platform base. `APIBaseURL` is the trusted Connect base, including any reverse-proxy mount path. For example a manifest destination `https://example.com/kas-west` may map to `https://example.com/proxy/platform`, sending rewrap to `/proxy/platform/kas.AccessService/Rewrap`. The client never derives credential routing by stripping an attacker-provided path. Multiple configured routes are permitted, but manifests still require one supported KAO.

Destination comparison parses a bounded ASCII DNS/canonical IPv4 profile, lowercases scheme/host, removes default ports and one trailing path slash. It rejects userinfo, queries, fragments, percent escapes, backslashes, non-ASCII/IDNA, IPv6, trailing-dot hosts, empty/double authority fields, invalid ports, dot path segments and duplicate slash segments. DNS labels are bounded; decimal IPv4 requires four octets of at most three digits and values 0–255 without leading zeros. WHATWG numeric final-label, hexadecimal, octal and shortened IPv4 forms are rejected rather than silently canonicalized differently by browser hosts. Paths contain only unescaped RFC3986 unreserved ASCII and slash. This explicit supported profile is narrower than full Go URL parity; unsupported canonicalization fails closed. Independent native `net/url` and Node WHATWG checks cover accepted canonicalization and ambiguous numeric negatives. These are URL-oracle tests, not an actual browser SDK run.

HTTPS is required unless the caller explicitly enables development `AllowHTTP`. Manifest validation and route authorization precede token/provider acquisition. Goalchemy's HTTP capability preserves TLS verification and returns redirects without following them, preventing redirect credential forwarding. Requests/responses are bounded at 1 MiB and JSON depth/node/string limits. Successful responses require one `application/json` content type and strict bounded JSON; duplicate names fail parsing.

## Rewrap protocol

Modern Connect requests send `Content-Type: application/json`, `Connect-Protocol-Version: 1` and `Authorization: Bearer` or `Authorization: DPoP` according to configuration. DPoP adds a fresh proof signed with the same authentication key as the SRT. SRT claims are exact JSON-string `requestBody`, real current `iat` and `exp = iat + 60`. Signing is RS256 or ES256 with the declared raw 64-byte JOSE ECDSA capability. The grouped body uses one policy id `policy`, one KAO id `kao-0`, the exact bound base64 policy bytes, full proto-compatible KAO fields including kid/sid/protocol/encrypted metadata and EC ephemeralPublicKey, and the fresh session's public SPKI PEM. Manifest-only `schemaVersion` is absent from the protobuf KAO.

Responses require exactly one matching policy group and KAO result, rejecting unexpected, duplicate or missing groups/ids and unsupported fields. Status/result oneof, base64 ciphertext and wrapping lengths are checked: RSA replies are 256 bytes; EC replies are exactly 60 bytes. Metadata containing required obligations produces a structured `required_obligations` error with the FQNs; the client advertises no fulfillment capability and never ignores required obligations. Unknown metadata is rejected. OAEP and MGF1 use SHA-1 with an empty label through the crypto capability. A recovered share must be exactly 32 bytes before integrity decryption. Legacy response upgrades, multiple shares/KAS alternatives, assertions and obligation fulfillment remain later work.

## Verification

From `sdk/`:

```sh
GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/.local/go-build-cache GOMODCACHE=$PWD/.local/go-mod-cache go test -race ./...
GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/.local/go-build-cache GOMODCACHE=$PWD/.local/go-mod-cache .local/bin/goalchemy check -gate cooperative . ./tdf/... ./tests/sourcecheck/client
GOTOOLCHAIN=go1.25.14 .local/bin/goalchemy compile -gate cooperative -target go -out .local/client-probe/go ./tests/sourcecheck/client
```

The last command must fail with `GCE002` naming `lib.http.do`; a generated blocking HTTP mapping is not acceptance evidence. The native probe can run with `go run ./tests/sourcecheck/client`.

Native tests use independent standard-library JWT signature verification, RSA-OAEP share/session wrapping, P256 ECDH/HKDF/AES-GCM wrapping and policy HMAC binding checks. Eight combinations cover two KAO algorithms, two response sessions and RS256/ES256 SRT signing. EC session negatives reject malformed/private/wrong-type/wrong-curve/tampered keys, missing/nonstring keys, frame-size and nonce/tag authentication failures, and wrong share lengths. Explicit/discovered key and creation-option conflicts fail without output. They cover actual Connect paths/headers/PublicKey fields, exact JSON-string SRT claims, bound policy/SID preservation, correct OAuth form encoding, cache expiration/concurrent refresh, invalid/stale/DPoP tokens, discovery issuer/token-route rejection, provider cancellation, Close/key ownership/races, context deadlines, response limits/content type/JSON, strict response groups/ids/status/oneof, required obligations, destination rejection before credentials, redirects and tampered payload zero output. Native acceptance requires Node for its independent WHATWG URL oracle.

Run the [local platform](platform.md) readiness checks and reference smoke first, then from `sdk/tests/interop/client/`:

```sh
GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/../../../.local/go-build-cache GOMODCACHE=$PWD/../../../.local/go-mod-cache go run .
```

The isolated runner imports the pinned reference SDK only in its own module, verifies reference revisions and tracked cleanliness, deletes fresh output paths before invoking stock CLIs, and uses subprocess/context deadlines. It tests 16 client-produced Go/Web consumer pairs across small, empty, binary, exact 16 KiB boundaries, multiple segments, HS256, nonempty metadata and no-attribute policy. It also tests 22 reference-produced reads: six earlier smoke fixtures plus fresh Go/Web fixtures for all eight cases. Generated manifests are checked for actual segment algorithms, sizes and counts. The pinned stable Go writer retains GMAC for the `client-hs256.go-default-gmac` case; actual reference HS256 evidence comes from Web. The stock Web high-level facade drops the metadata option, so the Node fixture uses its pinned lower-level client to create nonempty metadata while retaining stock auth/format/crypto. Three real denied policies, payload tamper, invalid Bearer HTTP 401 on an allowed policy, and pre-canceled allowed-policy decryption return zero output. Results live under ignored `.local/interop-client/results.json`; set `TDF_CLIENT_INTEROP_NAME=interop-client-root` for an isolated replay directory beneath `.local`.

The separate [EC client evidence](ec-client.md) covers native Bearer RSA/P256 KAO and both independent response sessions against the EC profile, restoring ordinary Bearer/RSA r1 afterward. The separate [DPoP client evidence](dpop-client.md) covers own OAuth/proofs, nonce retries, token binding and real modern grouped SRT enforcement. No reference source, engine/format source or Goalchemy capability was changed by the EC client assignment. Scheduler/HTTP emission, usable generated libraries on every target, an actual browser run and full Go SDK parity remain mandatory subsequent work.
