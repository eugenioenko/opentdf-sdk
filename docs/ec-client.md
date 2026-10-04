# Native EC shared-client evidence

This accepted Phase 3e assignment extends the [shared native client](client.md) with real P256 discovery/creation and independent RSA/P256 response sessions. It retains RSA defaults and byte APIs. Authentication signing (RS256/ES256), KAO wrapping and response sessions are three independent choices. This runner covers Bearer authentication; subsequent [DPoP evidence](dpop-client.md) covers the enforced profile. Generated HTTP, all seven target libraries, an actual browser run and full Go SDK parity remain pending.

Configure both choices explicitly when needed:

```go
c, err := sdk.New(sdk.Config{
    PlatformURL: "http://localhost:8080", KASURL: "http://localhost:8080/kas",
    IssuerURL: "http://localhost:8888/auth/realms/opentdf",
    ClientID: "opentdf-sdk", ClientSecret: "secret", AllowHTTP: true,
    KASAlgorithm: "ec:secp256r1", SessionAlgorithm: "rsa:2048",
})
```

`KASAlgorithm` selects PublicKey discovery and creation; `EncryptConfig.Algorithm`, when supplied, must agree. `SessionAlgorithm` selects a fresh response-session key for each decrypt, independently of manifest wrapping. Both default to `rsa:2048`. Explicit creation keys are validated against their configured algorithm and remain caller-owned. The grouped rewrap includes the original base64 policy, metadata, kid/sid and EC ephemeral public key. Wire public keys require SPKI; private/certificate PEM and mismatched algorithms/curves fail. See [client contract](client.md) and the pinned [crypto/protocol audit](tdf-poc.md).

## Offline verification

The independent standard-library fixture verifies all eight KAO × response-session × SRT-auth-signature combinations. It independently unwraps RSA-OAEP-SHA1 or P256/HKDF/AES-GCM KAOs, checks the exact policy-binding HMAC, verifies RS256/ES256 SRT signatures, parses session SPKI and detects reused sessions. EC response derivation uses ECDH, salt SHA256(`TDF`), HKDF-SHA256 with empty info and 32 output bytes, and a 60-byte nonce12/ciphertext32/tag16 frame without AAD.

Negative cases cover incompatible configured/discovered keys and creation options, unsupported algorithms/curves, malformed/missing/empty/nonstring/private/wrong-type/tampered response SPKI, frame lengths, nonce/tag authentication failures, recovered share length, response groups/IDs/status/oneof, required obligations, payload integrity, pre-cancellation, concurrent operations and caller-owned key lifetimes. RSA permits an absent or empty-string protobuf `sessionPublicKey`, while rejecting nonempty or nonstring values. Failure results contain no plaintext or metadata.

From `sdk/`:

```sh
GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/.local/go-build-cache GOMODCACHE=$PWD/.local/go-mod-cache go test -race ./...
GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/.local/go-build-cache GOMODCACHE=$PWD/.local/go-mod-cache .local/bin/goalchemy check -gate cooperative ./src ./src/tdf/... ./tests/sourcecheck/client
GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/.local/go-build-cache GOMODCACHE=$PWD/.local/go-mod-cache .local/bin/goalchemy compile -gate cooperative -target go -out .local/ecclient-probe/go ./tests/sourcecheck/client
```

Race tests and cooperative checking pass. Emission must fail with `GCE002` naming `lib.http.do`; generated blocking HTTP is not acceptance evidence.

## Real KAS matrix

Use the isolated [runner module](../tests/interop/ecclient/go.mod), preserving the earlier RSA-only runner. New readers use the shared client's own token acquisition, discovery, SRT, rewrap and integrity engine; reference key recovery is never used for them. Reference dependencies stay out of the shared module. The runner verifies pinned revisions and tracked cleanliness, bounds HTTP/context/subprocess work, clears stale success reports and CLI plaintext outputs, and stores results only under ignored `.local/interop-ecclient/`. Set `TDF_ECCLIENT_INTEROP_NAME=interop-ecclient-root` for isolated replay.

From `sdk/`, with the development stack initialized:

```sh
(
    set -e
    sdk_checkout_directory="$PWD"
    trap 'make -C "$sdk_checkout_directory" platform-profile-basic && make -C "$sdk_checkout_directory" platform-ready' EXIT
    make platform-profile-ec
    cd tests/interop/ecclient
    GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/../../../.local/go-build-cache GOMODCACHE=$PWD/../../../.local/go-mod-cache timeout 600 go run .
)
```

The subshell captures the absolute checkout path before changing directories and restores basic RSA/Bearer on success and failure. The [EC profile](secure-profiles.md) serves RSA `profile-r1` and P256 `profile-e1`, with P256 as the platform base/default. Every archive is inspected for its actual KAO algorithm, kid, wrapping frame, segment integrity and 16 KiB segment boundaries.

| Evidence | Scope | Passing count |
| --- | --- | ---: |
| New client → stock Go/Web CLI | 2 KAO algorithms × 7 payload cases × 2 consumers × 2 response sessions | 56 |
| Fresh stock Go/Web writer → new client | 2 KAO algorithms × 7 payload cases × 2 producers × 2 response sessions | 56 |
| Stock Go reader payload/metadata | 2 KAO algorithms × 7 payload cases × 2 response sessions | 28 |
| Real PDP denials, zero output | New/Go/Web denied writers × 2 KAO algorithms × 2 response sessions | 12 |
| Rewritten-CRC payload tamper | 2 KAO algorithms × 2 response sessions; must report integrity failure | 4 |
| Pre-canceled decrypt | 2 KAO algorithms × 2 response sessions; must retain context cancellation | 4 |
| Invalid Bearer on allowed policy | 2 KAO algorithms × 2 response sessions; must report HTTP401 | 4 |

The seven payload cases are small, empty, binary, exact 16 KiB, multiple segments, HS256 and nonempty JSON metadata. Every new-reader success compares exact bytes and metadata; stock Go readers also authenticate and compare metadata. The pinned Go writer retains default GMAC in the HS256 case, so independent HS256 producer evidence comes from Web. New writers emit and reference readers consume actual HS256.

The stock Web high-level facade drops metadata, so [its fixture](../tests/interop/ecclient/web-fixture.mjs) uses the pinned lower-level `tdf3Client.encrypt`. Its discovery also prefers the platform base key even when another algorithm is requested ([access.ts](../../web-sdk/lib/src/access.ts)). The fixture supplies the supported `scope.attributeValues[].kasKeys[].publicKey` cache configuration plus an explicit `splitPlan` to pin RSA or EC public material/kid, retaining stock auth/format/crypto. This is a reference configuration choice; it does not alter reference source or bypass real KAS authorization.

The pinned stock Web reader's `getMetadata()` exposes KAS result metadata, not decrypted manifest `encryptedMetadata` ([unwrap result and stream metadata](../../web-sdk/lib/tdf3/src/tdf.ts), [stream API](../../web-sdk/lib/tdf3/src/client/DecoratedReadableStream.ts)). Stock Web CLI payload consumption succeeds, but does not establish a plaintext encrypted-metadata API. The exact metadata evidence is the new reader consuming both independent writers and stock Go consuming the new writer; no stock Web encrypted-metadata API success is claimed.

The passing live run restored basic `r1` and passed readiness. The matrix is native Go/Bearer evidence only. It does not claim DPoP/nonce retry, generated HTTP, usable libraries on every target, an actual browser run, obligation fulfillment or full Go SDK parity.
