# Shared native DPoP client

`Config.DPoP: true` enables the shared client's own OAuth and resource proofs. `AuthAlgorithm` selects ES256/P256 by default or RS256/RSA-2048. `New` generates an authentication signing key unless `AuthKey` supplies a matching caller-owned handle. This key signs both DPoP and the modern grouped rewrap SRT. KAS wrapping and fresh response-session keys remain independent. Native Go is the accepted execution scope; generated HTTP still reports `GCE002: lib.http.do`, and actual browser/all-target acceptance remains subsequent work.

```go
c, err := sdk.New(sdk.Config{
    PlatformURL: "http://localhost:8080", KASURL: "http://localhost:8080/kas",
    IssuerURL: "http://localhost:8888/auth/realms/opentdf",
    ClientID: "opentdf-sdk", ClientSecret: "secret", AllowHTTP: true,
    DPoP: true, AuthAlgorithm: "ES256",
    KASAlgorithm: "ec:secp256r1", SessionAlgorithm: "ec:secp256r1",
})
```

The development HTTP option is explicit. Production destinations require HTTPS and keep the [client's bounded trusted routing rules](client.md). Platform discovery uses the caller's expected issuer and conventional token destination, or an explicitly configured trusted `TokenURL`.

## Proofs and token trust

Every token-endpoint/resource DPoP attempt gets a new 32-byte secure-random base64url `jti`, real UTC `iat`, actual uppercase `htm`, and full normalized `htu` including the request path. Scheme/host are lowercased, default ports removed, an empty path becomes `/`, and request trailing slashes are retained. The supported URL profile rejects query/fragment/escaped paths instead of silently changing request routing. Proof headers contain `typ: dpop+jwt`, `alg: ES256` or `RS256`, and only the required public JWK members. ES256 uses JOSE raw 64-byte R||S. The JWK thumbprint hashes RFC 7638 required members in lexical order.

Token-endpoint proofs have no `ath` and no token Authorization header. Resources use `Authorization: DPoP <access_token>` and `ath = base64url(SHA256(access_token))`. `DPoP: true` requires a DPoP token scheme; `DPoP: false` retains Bearer behavior. Neither path silently changes to the other scheme.

`AccessToken` keeps `Value`, `Scheme` and future UTC Unix `ExpiresAt`. Tokens refresh at the existing five-second margin under a cancellation-aware gate. No provider callback, signing operation or HTTP call runs with the client mutex held.

Recognizable compact JWTs (base64url JSON JOSE header with `alg`, or `typ: JWT`) require a strict bounded claims object containing matching `cnf.jkt`. Missing, nonobject, mismatched, duplicate or malformed claims fail with `token_binding`, even if `ConfirmationJKT` also matches. The SDK decodes these claims only to check consistency; it does **not** verify token signatures, issuer, audience or issuer keys locally. The authenticated caller-trusted OAuth endpoint/provider and the resource server establish token authority. The resource server verifies the actual issuer signature and sender binding.

Opaque tokens have a separate explicit contract. An authenticated caller-trusted OAuth endpoint returning `token_type: DPoP` attests that an opaque response is bound to the submitted proof's key. An opaque provider token requires `ConfirmationJKT` to match the client's RFC 7638 thumbprint. Dots alone do not classify an opaque value as JWT; attested dotted opaque values are accepted. A token recognizable as a compact JWT is checked as JWT and cannot bypass `cnf.jkt` through opaque metadata. Unattested opaque tokens fail. This prevents silent unbound-token use without requiring all authorization servers to issue JWTs.

Secretless hosts supply `TokenProvider func(context.Context) (AccessToken, error)` and their matching caller-owned `AuthKey`. The host performs authorization and refresh without shipping a client secret. For JWTs return `Scheme: "DPoP"`, valid expiry and the issuer's bound token. For opaque tokens also set the verified provider `ConfirmationJKT`. The SDK cannot forcibly interrupt a callback ignoring its context. Borrowed key handles remain caller-owned through `Close` and are never copied by dereferencing the handle.

## Nonces and retry bounds

A response nonce is inspected before HTTP error parsing and 401 token eviction. Exactly one `DPoP-Nonce` header is permitted, with 1–1024 bytes of RFC 9449 nonce visible ASCII excluding quote/backslash; duplicate, empty, whitespace/control and oversized values produce `invalid_nonce`. Successful responses update the nonce too. Caches are race-safe and bounded by the trusted origin set, normalized independently of paths. Keycloak and platform origins cannot exchange nonce state.

HTTP 400 or 401 with a valid newly received nonce different from the attempted nonce retries once, with a fresh proof/jti and identical request bytes. The rewrap SRT and session key remain coherent across that retry. A second challenge is returned directly; plain 401 and same-nonce challenges are not replayed. Final 401 evicts the token. The cache can retain the second challenge for a later distinct operation without looping. Redirects are returned without following them, preventing credential/proof forwarding. Caller cancellation, configured deadlines and client lifetime bound both attempts. `Close` clears caches and late results cannot repopulate them.

The pinned server accepts its current/previous nonce with a fresh jti. Reusing an identical proof jti fails. It currently accepts a bound token with Bearer plus a valid proof with a warning; the shared client nevertheless sends the required DPoP scheme. These are [observed server boundaries](secure-profiles.md), not a claim that the server enforces the scheme alone.

## Verification and evidence boundaries

Native tests independently verify standard-library JWK thumbprints and RS256/ES256 signatures, fresh proof identifiers/time/claims, token/resource header distinctions and hash binding. Eight wrapping × response-session × auth combinations independently unwrap actual grouped requests with policy HMAC and metadata checks. Nonce cases cover initial token/resource challenges, rotation, successful updates, origin isolation, immutable bodies, plain/same/second challenge bounds and malformed/duplicate headers. Provider tests cover secretless resource/SRT round trips for both auth algorithms with a matching borrowed key, JWT/opaque binding, scheme mismatch, concurrent acquisition, expiry refresh, cancellation/deadlines, borrowed-key lifetime, in-flight HTTP shutdown and cache clearing.

From `sdk/`:

```sh
GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/.local/go-build-cache GOMODCACHE=$PWD/.local/go-mod-cache go test -race ./... -count=1
GOALCHEMY_ROOT=$PWD/../goalchemy GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/.local/go-build-cache GOMODCACHE=$PWD/.local/go-mod-cache .local/bin/goalchemy check -gate cooperative ./src ./src/tdf/... ./tests/sourcecheck/client
GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/.local/go-build-cache GOMODCACHE=$PWD/.local/go-mod-cache go run ./tests/sourcecheck/client
```

The [isolated live runner](../tests/interop/dpopclient/main.go) validates reference pins/tracked cleanliness, removes stale successes/plaintext, bounds HTTP/subprocesses, and supports an independently replayable safe directory name beginning `interop-dpopclient` via `TDF_DPOPCLIENT_INTEROP_NAME` (default ignored `.local/interop-dpopclient`). It switches only this project's platform container through existing controlled helpers and restores basic plus readiness before writing the final success report. Use this bounded subshell with an outer restoration trap so termination is also covered:

```sh
(
  set -e
  tdf_sdk_path=$PWD
  trap 'tdf_status=$?; if test "$(cat "$tdf_sdk_path/.local/profiles/active" 2>/dev/null)" != basic; then cd "$tdf_sdk_path"; make platform-profile-basic platform-ready || exit 1; fi; exit "$tdf_status"' EXIT
  cd tests/interop/dpopclient
  GOTOOLCHAIN=go1.25.14 GOCACHE="$tdf_sdk_path/.local/go-build-cache" GOMODCACHE="$tdf_sdk_path/.local/go-mod-cache" timeout 900s go run .
)
```

All claimed shared-client successes go directly to the real platform under enforced DPoP, required native nonces and strict full URI checking. The runner checks RSA/P256 wrapping × RSA/P256 response sessions × ES256/RS256 auth over small, empty, binary, exact/multiple segments, HS256 and exact metadata. The client uses its own OAuth, discovery, proof, SRT, unwrap and integrity paths; no reference unsafe payload-key retrieval is used.

Stock Go CLI/SDK consumes new files under enforced DPoP and compares payload/metadata bytes. Stock Web CLI with `--dpop` actually fails HTTP401/unauthenticated on this profile and returns no plaintext. Its outcome is recorded explicitly. Web format interoperability is proved separately: pinned stock Web producer files are created under the EC/Bearer profile then consumed through the shared client's own enforced DPoP; new DPoP-authenticated files are consumed by stock Web after a controlled switch to EC/Bearer. Persistent KAS registry keys/kids remain identical. These prove format compatibility, not stock Web DPoP authentication. The pinned Go writer's nominal HS256 case retains GMAC; independent stock Web supplies actual HS256. The pinned Web facade drops encryption metadata, so its supported lower-level stock client creates metadata; stock Web `getMetadata` is not claimed to decrypt manifest encryptedMetadata.

Zero-output negatives include real denied policies from all three producers, payload/root-signature/segment-hash/encryptedMetadata tampering with fresh ZIP CRCs, cancellation, invalid issuer tokens and local key-binding mismatch. A **labeled native mutation proxy** is used only for protocol negatives: it independently verifies the shared client's original proof/SRT, preserves the actual grouped request/session body, and re-signs the destination proof with the same bound fixture key because the test route changed. Before each mutation it proves real destination token/proof/nonce acceptance with protected policy ListAttributes HTTP200. Unmodified grouped SRT passes real rewrap; wrong auth key, invalid signature, expired/future claims and body tampering return actual KAS401 without an auth-middleware challenge. Missing/nonstring requestBody returns actual KAS400. Proof key/signature/method/full-URI/hash/time negatives return actual auth-middleware401 with `invalid_dpop_proof`. The proxy never supplies plaintext or recovered keys and is excluded from direct shared-client positive counts. Per-combination evidence records only status/case names, not raw proofs/tokens/secrets.

Native fixtures observe exact retry counts. Live acceptance proves server nonce enforcement; the dedicated `lib/http` transport does not expose an observer hook, so the runner does not fabricate direct live shared-client attempt counts. Generated HTTP, all seven targets, actual browsers, additional authentication methods and full Go SDK parity remain required later work.

Observed full native live run on 2026-10-02:

| Evidence | Count | Authentication / scope |
| --- | ---: | --- |
| New client → stock Go CLI | 56 | Enforced DPoP, direct real platform |
| Go/Web producer formats → new client | 112 | Own enforced DPoP, direct real platform |
| Stock Go payload + exact metadata | 56 | Enforced DPoP, direct real platform |
| New client → stock Web format consumers | 56 | Separate EC/Bearer profile |
| Typed zero-output failures | 200 | Direct denial/integrity/cancel/token cases plus labeled protocol mutations |
| Labeled protocol evidence files | 8 | Protected destination preflight, grouped SRT baseline and mutations |

The eight protocol files contain eight unmodified grouped-SRT HTTP200 baselines, 104 HTTP401 mutations and 16 HTTP400 body-claim mutations, each with protected destination preflight HTTP200. Proxy baselines/mutations are separate from direct interoperability counts. Stock Web enforced-DPoP authentication failed actual HTTP401/unauthenticated and produced no plaintext. Evidence is ignored `.local/interop-dpopclient/results.json` and `protocol-*.json`; the successful run restored basic and verified readiness before saving its report.
