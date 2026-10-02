# TDF3 and KAS source audit

This is a source audit for the first SDK release, not a record of successful interoperability. No platform was started and no live rewrap was performed during Phase 0. The implementation must use subset Go for shared format/protocol logic and maintained native crypto through Goalchemy capabilities. All seven targets are required; TypeScript includes Node and browsers. Feature scope is tracked in [compatibility.md](compatibility.md).

## Reference identity and evidence

The checked-out HEADs match [references.lock.json](../references.lock.json): platform `f2635158b681fa970aafce7eacf108a453521f63`, web SDK `55a0521b1499b392c75373e11ec5930c6a43f0c7`, and Goalchemy `9ab3baac6ca6b543adedccd29876436d5a5b8907`. Platform and web SDK working trees were clean when inspected. Links below refer to the pinned workspace source files. The declared SDK versions are Go `0.34.0` and web `0.21.0`; both target TDF spec `4.3.0` ([Go version](../../platform/sdk/version.go), [web version](../../web-sdk/lib/src/version.ts)). “TDF3” names the ZIP container family, not a manifest schema value of `3`.

## ZIP container

Both pinned writers emit **`0.payload` and `0.manifest.json`**. Both readers prefer **`manifest.json`** when present, falling back to `0.manifest.json` only when the spec entry is missing. A present but invalid/oversized spec entry must not trigger fallback. The new writer can use the spec entry name; its reader must accept both. ([Go names](../../platform/sdk/internal/zipstream/zip_headers.go), [Go selection](../../platform/sdk/internal/zipstream/tdf3_reader.go), [web selection and writer binding](../../web-sdk/lib/tdf3/src/tdf.ts).)

Entries are stored without compression. `0.payload` is a concatenation of encrypted segments, not an archive entry per segment. Go uses streaming headers/data descriptors and chooses ZIP32 or ZIP64 from known encrypted size/offsets; an unmeasurable input forces ZIP64. Its conservative 32-bit switch threshold is `math.MaxInt32`, not 4 GiB. The web `ZipWriter` enables ZIP64 unconditionally, uses UTF-8 filename/data-descriptor flags, ZIP64 extra fields, 64-bit descriptors, central directory, ZIP64 end record/locator, and ordinary end record. Small web files therefore still require ZIP64 reading. ([Go size/layout choice](../../platform/sdk/tdf.go), [Go ZIP primitives](../../platform/sdk/internal/zipstream/zip_primitives.go), [Go stored entries](../../platform/sdk/internal/zipstream/segment_writer.go), [web ZIP writer](../../web-sdk/lib/tdf3/src/utils/zip-writer.ts).)

The first release may write bounded in-memory ZIP32, but must read the ZIP64/data-descriptor forms above. Bounds, duplicate entries, conflicting central/local headers, truncated descriptors, offsets, declared sizes, and CRC handling need independent fixtures. CRC is ZIP bookkeeping; it does not replace cryptographic checks. Go's default manifest bound is 10 MiB ([reader options](../../platform/sdk/tdf_config.go)). Compression variants and HTML wrappers are outside the initial profile unless separately implemented.

## Manifest and segment framing

The modern manifest uses `schemaVersion: "4.3.0"`, `payload`, `encryptionInformation`, and optional `assertions`. Payload fields are `type: "reference"`, `url: "0.payload"`, `protocol: "zip"`, `isEncrypted: true`, and `mimeType`. Encryption fields are `type: "split"`, base64 JSON `policy`, `keyAccess[]`, `method`, and `integrityInformation`. `method.algorithm` is `AES-256-GCM`, `isStreamable` becomes true, and `iv` is empty in the Go writer but contains a base64 metadata/key-info IV in web output. Segment nonces are inside each frame, so `method.iv` is not the nonce to reuse for payload decryption. ([Go models](../../platform/sdk/manifest.go), [Go manifest construction](../../platform/sdk/chunked_writer.go), [web manifest](../../web-sdk/lib/tdf3/src/tdf.ts), [web method](../../web-sdk/lib/tdf3/src/models/encryption-information.ts).)

The payload key is 32 bytes. Each frame is:

```text
random nonce (12 bytes) || AES-256-GCM ciphertext (plaintext length) || tag (16 bytes)
```

AAD is absent. Every segment gets a fresh nonce. Ciphertext length equals plaintext length + 28; the nonce is part of the integrity input. Go uses `cipher.NewGCMWithRandomNonce`; web explicitly prepends the IV to Web Crypto ciphertext/tag. ([Go AES](../../platform/lib/ocrypto/aes_gcm.go), [web AES framing](../../web-sdk/lib/tdf3/src/ciphers/aes-gcm-cipher.ts), [web segment encryption](../../web-sdk/lib/tdf3/src/tdf.ts).)

Go defaults to 2 MiB plaintext segments, clamps `WithSegmentSize` to 16 KiB–4 MiB, and emits one encrypted empty segment for an empty payload. Web defaults to 1 MiB and omits each per-segment size independently when it equals its corresponding default. Its stream writer emits no segment for completely empty input. Treat empty input as an explicit interop case rather than assuming equal writer behavior. ([Go defaults](../../platform/sdk/tdf_config.go), [Go empty/exact-boundary handling](../../platform/sdk/tdf.go), [web defaults](../../web-sdk/lib/tdf3/src/client/builders.ts), [web stream termination](../../web-sdk/lib/tdf3/src/tdf.ts), [web size omission](../../web-sdk/lib/tdf3/src/tdf.ts).)

`integrityInformation` contains `rootSignature: {alg,sig}`, `segmentHashAlg`, `segmentSizeDefault`, `encryptedSegmentSizeDefault`, and ordered `segments: [{hash,segmentSize?,encryptedSegmentSize?}]`. Go resolves omitted sizes and rejects values that disagree with the 28-byte framing. Because its JSON struct conflates missing and zero, a zero plaintext size falls back to the default only when resolved ciphertext size equals its default. The new reader should track field presence directly and reject negative/inconsistent sizes, overflow, extra/missing payload bytes, and invalid hash lengths. ([Go wire models and resolution](../../platform/sdk/manifest.go).)

## Modern and legacy integrity encodings

For modern files:

1. A `GMAC` segment hash is the frame's trailing 16-byte AES-GCM tag; `HS256` is HMAC-SHA256(payload key, entire frame).
2. `segment.hash = standard_base64(raw hash bytes)`.
3. Aggregate bytes are all **decoded segment hashes concatenated in manifest order**.
4. `rootSignature.alg = "HS256"` and `sig = standard_base64(HMAC-SHA256(payload key, aggregate bytes))`.

GMAC is valid only for AEAD-produced segment bytes. A GMAC root would merely copy the end of manifest-supplied data and is rejected by both pinned readers. Go allows absent/empty root `alg` as historical HS256 and case-insensitive HS256; its segment parser treats anything other than case-insensitive GMAC as HS256. Web requires explicit case-insensitive HS256 at root and explicitly allows only GMAC/HS256 segments (missing segment alg falls back to root alg). The rewrite should fail on unknown algorithms; do not reproduce Go's permissive coercion silently. ([Go integrity](../../platform/sdk/tdf.go), [Go segment parsing](../../platform/sdk/tdf.go), [web allowlists](../../web-sdk/lib/tdf3/src/tdf.ts), [web root verification](../../web-sdk/lib/tdf3/src/tdf.ts).)

Legacy writers hex-encode the segment digest/tag into lowercase ASCII **before** base64. The aggregate then consists of those decoded ASCII hex strings. The root is base64(lowercase ASCII hex HMAC over that aggregate). Go `WithTargetMode` selects this for any version less than `4.3.0` and omits `schemaVersion`; Go readers select legacy solely when the manifest version is absent/empty. Web writers select legacy only for exactly `4.2.2`, omit `schemaVersion` then, and readers use `schemaVersion || tdf_spec_version || "4.2.2"` and the same exact match. Explicit old version strings therefore do not have identical interpretation in both SDKs. Web's legacy HS256 segment helper also round-trips ciphertext through UTF-8 text before HMAC, whereas Go HMACs original bytes; legacy GMAC avoids that difference. Legacy HS256 requires separate fixtures and is not covered by modern interop. ([Go target mode](../../platform/sdk/tdf_config.go), [Go integrity encoding](../../platform/sdk/tdf.go), [web legacy helpers](../../web-sdk/lib/tdf3/src/tdf.ts), [web legacy switch](../../web-sdk/lib/tdf3/src/tdf.ts).)

Root authentication covers the ordered hash list, not every manifest field. Per-segment authentication, AES-GCM verification, and framing/size consistency are separate requirements. Do not return successful plaintext before the complete required verification has passed. Go verifies root and assertions in `buildKey` and segment hashes during reads; web performs corresponding root/assertion verification before its decrypt stream. ([Go verification](../../platform/sdk/tdf.go), [Go read checks](../../platform/sdk/tdf.go), [web verification](../../web-sdk/lib/tdf3/src/tdf.ts).)

## Policy, binding, metadata, and key roles

Policy is standard base64 of JSON with a UUID and `body.dataAttributes`/`body.dissem`; each data attribute contains its `attribute` FQN. Preserve the **exact base64 policy string** for binding and rewrap, rather than parsing/reserializing it. Binding remains `{"alg":"HS256","hash":base64(lowercase_hex(HMAC-SHA256(key share, ASCII base64 policy)))}` for both modern writers. With one KAS/share the share is the payload key. KAS verifies the binding against the share before authorization/rewrap and also accepts a raw base64 digest; this server tolerance is not an alternate writer default. ([policy model](../../platform/sdk/manifest.go), [Go binding](../../platform/sdk/tdf.go), [web binding](../../web-sdk/lib/tdf3/src/models/key-access.ts), [KAS binding validation](../../platform/service/kas/access/rewrap.go).)

Distinct key roles must remain distinct:

| Key | Purpose and representation |
| --- | --- |
| Payload key/share | Random 32-byte secret; encrypts payload, authenticates policy/root/segments; distinct shares for multi-KAS splits |
| KAS wrapping key | Long-lived KAS public key (PEM, identified by optional `kid`); private half stays at KAS |
| Manifest EC ephemeral key | Encrypt-time ECDH key pair; public PEM stored in KAO `ephemeralPublicKey` |
| Authentication/DPoP signing key | Signs HTTP proof and signed request token; public JWK in DPoP header; IdP binds its thumbprint to token |
| Client encryption/session key | Decrypt-time RSA or ECDH key pair; public PEM in `clientPublicKey`; private half unwraps response |
| KAS EC response ephemeral key | Public PEM in response `sessionPublicKey`; derives response wrap key with client's session private key |

Key separation is explicit in [Go KAS client](../../platform/sdk/kas_client.go) and [web rewrap construction](../../web-sdk/lib/tdf3/src/tdf.ts). The response wrapping algorithm follows the client public key and need not equal the manifest wrapping algorithm. Go defaults the client session to RSA-2048 ([reader config](../../platform/sdk/tdf_config.go)).

KAO fields are `type`, `url`, `protocol: "kas"`, `wrappedKey`, `policyBinding`, optional `kid`, `sid`, `schemaVersion: "1.0"`, `ephemeralPublicKey`, and `encryptedMetadata`. This schema version is distinct from manifest version. A multi-share DEK is XOR of one successful share per distinct `sid`; same `sid` KAS alternatives form OR, distinct shares form AND. Single-KAS implementation must reject unsupported multiple-share inputs rather than decrypt with the first response. ([Go KAO](../../platform/sdk/manifest.go), [Go split construction](../../platform/sdk/tdf.go), [Go reconstruction](../../platform/sdk/tdf.go), [web split construction](../../web-sdk/lib/tdf3/src/models/encryption-information.ts).)

`encryptedMetadata` is base64(JSON `{ciphertext,iv}`), where `ciphertext` is base64 of the complete nonce+ciphertext+tag frame encrypted under the KAO share and `iv` separately records base64 nonce. Go emits it only when metadata is nonempty; web creates an encrypted metadata object even for its default empty string. KAS passes metadata without enforcing it. Reading ordinary web files therefore requires understanding metadata, even if the first public API does not expose all metadata options. ([Go metadata](../../platform/sdk/tdf.go), [Go metadata read](../../platform/sdk/tdf.go), [web metadata](../../web-sdk/lib/tdf3/src/models/encryption-information.ts), [KAS proto](../../platform/service/kas/kas.proto).)

## Native cryptographic parameters

| Operation | Verified parameters |
| --- | --- |
| RSA KAO wrap and RSA response unwrap | RSA-OAEP with **SHA-1**, MGF1 using SHA-1, empty label; initial profile RSA-2048. Explicitly select these instead of host defaults. |
| EC KAO and EC response wrap | ECDH; initial profile P-256 (`ec:secp256r1`); HKDF-SHA256 with salt `SHA-256(UTF-8("TDF"))`, empty info, 32-byte output; AES-256-GCM frame above. |
| Request/DPoP RS256 | RSASSA-PKCS1-v1_5 with SHA-256; this is independent of OAEP's hash. |
| Request/DPoP ES256 | ECDSA P-256/SHA-256; JWS signature is fixed-width **R || S**, 32 bytes each (64 total), not DER. |
| Encoding | Standard padded base64 for manifest/protobuf bytes; unpadded base64url for compact JWT parts, token hash and JWK members. |

RSA parameters are in [Go encrypt](../../platform/lib/ocrypto/asym_encryption.go), [Go decrypt](../../platform/lib/ocrypto/asym_decryption.go), and [web RSA](../../web-sdk/lib/tdf3/src/crypto/core/rsa.ts). EC parameters are in [Go EC wrap/salt](../../platform/sdk/tdf.go), [Go HKDF](../../platform/lib/ocrypto/ec_key_pair.go), [Go response derivation](../../platform/sdk/kas_client.go), [web EC derivation](../../web-sdk/lib/tdf3/src/crypto/core/ec.ts), and [web salt](../../web-sdk/lib/tdf3/src/crypto/salt.ts). JWS ECDSA representation is explicit in [web native signing](../../web-sdk/lib/tdf3/src/crypto/core/signing.ts). Native adapters whose signing API emits DER must convert for JOSE.

Public PEM generated by both references is DER **SubjectPublicKeyInfo (SPKI)** with `BEGIN PUBLIC KEY`; private PEM generated by Go is DER **PKCS#8** with `BEGIN PRIVATE KEY`. The proto's comment calling an EC public key “PKCS#8” is imprecise: the server actually parses PKIX/SPKI. KAS wrapping public-key import also supports certificate PEM in Go, and RSA private import tolerates PKCS#1. These are import compatibility choices, not the first writer's required export forms. ([Go RSA export](../../platform/lib/ocrypto/rsa_key_pair.go), [Go EC export](../../platform/lib/ocrypto/ec_key_pair.go), [Go public import](../../platform/lib/ocrypto/asym_encryption.go), [Go private import](../../platform/lib/ocrypto/asym_decryption.go), [web PEM](../../web-sdk/lib/tdf3/src/crypto/core/key-format.ts), [server client-public parsing](../../platform/service/kas/access/rewrap.go).)

## Actual KAS transport and JSON

The pinned platform registers Connect handlers at `/kas.AccessService/PublicKey`, `/kas.AccessService/LegacyPublicKey`, and `/kas.AccessService/Rewrap`, relative to the configured platform API base. PublicKey takes `{algorithm,fmt,v}` and returns `{publicKey,kid}`. Rewrap takes `{"signedRequestToken":"<compact JWT>"}`. Connect unary JSON is a suitable new client profile; generated Go clients do not require JSON by default (Connect client options select codecs/protocols). Web's Connect transport is provided by `@connectrpc/connect-web`. Use `POST`, `Content-Type: application/json` and the unary Connect protocol headers for the JSON implementation; validate the actual headers/status serialization against the running service. ([procedures](../../platform/protocol/go/kas/kasconnect/kas.connect.go), [registration](../../platform/service/kas/kas.go), [proto](../../platform/service/kas/kas.proto), [web transport](../../web-sdk/lib/src/platform.ts).)

**No REST HTTP annotation/gateway binding for `/kas/v2/rewrap` or `/kas/v2/kas_public_key` exists in the pinned KAS proto/registration.** The web SDK constructs these legacy URLs, tries Connect first, and falls back to REST only when an `AuthProvider` is present. Its URL helper removes `/v2/rewrap` and `/kas` to derive the platform base; Go's `parseBaseURL` drops all KAS path components and uses the origin. Reverse-proxy path deployments thus need explicit routing tests. The legacy client paths are verified; their availability on this platform revision is not established. ([web routing/fallback](../../web-sdk/lib/src/access.ts), [web RPC](../../web-sdk/lib/src/access/access-rpc.ts), [web REST](../../web-sdk/lib/src/access/access-fetch.ts), [web URL helper](../../web-sdk/lib/src/utils.ts), [Go base URL](../../platform/sdk/kas_client.go).)

The JWT's `requestBody` is a **JSON string**, not a nested object. Modern body shape (placeholder values):

```json
{
  "clientPublicKey": "-----BEGIN PUBLIC KEY-----\n...",
  "requests": [{
    "policy": {"id": "policy", "body": "<base64 original policy JSON>"},
    "keyAccessObjects": [{
      "keyAccessObjectId": "kao-0",
      "keyAccessObject": {
        "type": "wrapped", "url": "https://platform.example/kas",
        "protocol": "kas", "kid": "r1", "wrappedKey": "<base64 ciphertext>",
        "policyBinding": {"alg": "HS256", "hash": "<base64 hex HMAC>"}
      }
    }]
  }]
}
```

Optional per-policy `algorithm` exists, but these TDF clients normally leave it empty. Go includes deprecated top-level `keyAccess`, `policy`, and `algorithm` as well when one policy/KAO is sent. Web also includes them, with top-level `algorithm: "RS256"`; that string must not be confused with actual client encryption key type. KAO `schemaVersion` belongs to the manifest and is absent from the proto model. Protobuf bytes fields use base64 in JSON; JSON names include `type`, `url`, `sid`, and binding `alg` rather than field-name guesses. ([Go SRT construction](../../platform/sdk/kas_client.go), [web SRT construction](../../web-sdk/lib/tdf3/src/tdf.ts), [wire definitions](../../platform/service/kas/kas.proto).)

Go signs `{requestBody,iat:now,exp:now+60s}` using `AccessTokenSource.MakeToken`, i.e. the auth signing key. Web signs `requestBody` using its DPoP signing key, with `iat:now-3600s` and `exp:now+3600s`. KAS validates temporal claims using configured clock skew; when authenticated DPoP supplies a JWK, it verifies SRT against that key and an asymmetric algorithm allowlist. Without that JWK, the server deliberately skips SRT signature verification, although it parses/validates the token and enforces policy/binding. Do not use a Bearer-only success to claim DPoP/SRT validation. ([Go SRT](../../platform/sdk/kas_client.go), [web claims](../../web-sdk/lib/src/auth/auth.ts), [KAS SRT validation](../../platform/service/kas/access/rewrap.go), [KAS verification decision](../../platform/service/kas/access/rewrap.go).)

Modern response shape is `{sessionPublicKey?,responses:[{policyId,results:[{keyAccessObjectId,status,kasWrappedKey?|error?,metadata?}]}]}`; status is `permit` or `fail`. `kasWrappedKey` is base64 in JSON, decoded bytes in generated models. EC responses include `sessionPublicKey`; RSA responses do not need it. Match policy/KAO IDs and validate key lengths; HTTP success does not mean each KAO succeeded. Legacy `entityWrappedKey`, `schemaVersion` and top-level `metadata` are upgraded by the reference clients. ([response proto](../../platform/service/kas/kas.proto), [Go upgrade](../../platform/sdk/kas_client.go), [Go result handling](../../platform/sdk/kas_client.go), [web upgrade](../../web-sdk/lib/src/utils.ts).)

RPC failures distinguish `invalid_argument`, `unauthenticated`, `permission_denied`, and `internal` (and transport/deadline failures). KAS uses generic `bad request` for secret-dependent unwrap/binding failures. Per-KAO errors are serialized strings rather than typed Connect codes; preserve the distinction and avoid treating all failures as denial or all HTTP 200 responses as success. Additional context is header `X-Rewrap-Additional-Context: base64(JSON {obligations:{fulfillableFQNs:[]}})`; required obligation FQNs appear in result metadata under `X-Required-Obligations`. ([KAS error helpers](../../platform/service/kas/access/rewrap.go), [Go context/results](../../platform/sdk/kas_client.go), [web errors](../../web-sdk/lib/src/access/access-rpc.ts).)

## OAuth, DPoP, and browser implications

Go client-credentials auth POSTs a URL-encoded body with `grant_type=client_credentials` and optional space-delimited `scope`, using HTTP Basic for a client secret (or private-key JWT client assertion). It sends a DPoP proof to the token endpoint, without `ath`, and retries once on HTTP 400 with `DPoP-Nonce`. Token JSON includes `access_token`, `token_type`, `expires_in`; caching/expiry follows the token response. Web's client-credentials provider instead puts `client_id` and `client_secret` in the form. Both mechanisms must be verified against the local Keycloak configuration. ([Go OAuth](../../platform/sdk/auth/oauth/oauth.go), [Go nonce handling](../../platform/sdk/auth/oauth/oauth.go), [Go token scheme](../../platform/sdk/auth/access_token_source.go), [web form](../../web-sdk/lib/src/auth/oidc.ts), [web token providers](../../web-sdk/lib/src/auth/token-providers.ts).)

Go's default resolved signing key is ES256/P-256; explicit RS256 and other RSA/EC signing options exist. Its nonce-aware transport uses `Authorization: DPoP <token>` for DPoP tokens and `Bearer <token>` for Bearer tokens, with no proof in the Bearer case. DPoP protected header is `{typ:"dpop+jwt",alg,jwk:<public JWK>}`; proof claims are `jti`, `iat`, `htm`, `htu`, optional `nonce`, and resource-only `ath = base64url(SHA-256(access-token bytes))`. URI normalization removes query/fragment, lowercases scheme/host, removes default ports, preserves escaped path, and uses `/` for empty path. Nonces are cached per origin; 401 or 400 nonce challenges with a different nonce get one replay with a fresh proof; successful responses can update cached nonce. Request bytes must be replayable. ([Go key default](../../platform/sdk/sdk.go), [Go DPoP transport](../../platform/sdk/auth/dpop_transport.go), [Go proof/normalization](../../platform/sdk/auth/dpop_transport.go).)

Platform requires access-token `cnf.jkt` to match SHA-256 JWK thumbprint; a bound token presented as Bearer is rejected even with global enforcement disabled. DPoP verification checks asymmetric signature, `typ`, public-only JWK, timestamp, HTTP method/URI, token hash, configured nonce, and replayed `jti`. Nonce challenges expose `DPoP-Nonce` and `WWW-Authenticate: DPoP error="use_dpop_nonce"`; other proof failures use `invalid_dpop_proof`. Strict absolute-URI matching and nonce enforcement are configurable. ([platform validation](../../platform/service/internal/auth/authn.go), [platform proof checks](../../platform/service/internal/auth/authn.go), [challenge metadata](../../platform/service/internal/auth/authn.go).)

**The pinned web auth is not equivalent to that Go path.** Legacy `AccessToken.withCreds` sends `Bearer` even when adding a DPoP proof, its token response type does not track `token_type`, and it has no DPoP nonce retry. The newer `authTokenDPoPInterceptor` also sends Bearer and creates the proof without an access token argument (`ath` absent). Its legacy provider interceptor signs a path-only `htu`. These are source-observed limitations, not a recommendation for the rewrite. Enforced-DPoP web-reference interop may require a narrowly scoped reference runner auth adapter, with the stock behavior recorded separately. ([web legacy auth](../../web-sdk/lib/src/auth/oidc.ts), [web interceptor](../../web-sdk/lib/src/auth/interceptors.ts), [web bridge](../../web-sdk/lib/src/auth/interceptors.ts).)

Browser consumers must use a public-client/token-provider flow. Client credentials are explicitly server-side in the web source; the sample app supplies authorization-code/PKCE redirect flow outside the library. DPoP-bound tokens require the same signing key during issuance, proofs and SRT signing; generating an unrelated interceptor key after receiving a bound token cannot work. Browser operation also requires Web Crypto's permitted execution context and CORS allowing the RPC/auth headers and exposing nonce/challenge response headers. Browser redirect UX need not live in shared SDK code; token and matching key provision can be a host contract. ([web token-provider contract](../../web-sdk/lib/src/auth/interceptors.ts), [secret restriction](../../web-sdk/lib/src/auth/token-providers.ts), [sample PKCE](../../web-sdk/web-app/src/session.ts).)

## Runtime questions and next validation

The checked-in development configuration enables auth, disables DPoP enforcement and EC/hybrid TDF preview, and specifies localhost issuer/audience. This only describes a file: it does not establish which key types a running KAS serves or whether its deployment uses this config. EC request/response paths have preview gates; enabling them and provisioning keys are required for EC tests. ([dev configuration](../../platform/opentdf-dev.yaml), [preview flags](../../platform/service/kas/access/provider.go), [EC response gate](../../platform/service/kas/access/rewrap.go).)

Phase 1 should build both pinned CLIs, inspect their actual help, provision one allowed and one denied attribute policy, obtain a token, fetch a KAS key, and decrypt both reference writers' files in both directions. Go operational sources are [tdf commands](../../platform/otdfctl/cmd/tdf/encrypt.go), [decrypt command](../../platform/otdfctl/cmd/tdf/decrypt.go), and [handlers](../../platform/otdfctl/pkg/handlers/tdf.go). Web uses [bin/opentdf.mjs](../../web-sdk/cli/bin/opentdf.mjs) with [cli.ts](../../web-sdk/cli/src/cli.ts); its [README](../../web-sdk/cli/README.md) provides examples whose flags must be checked against this revision. No CLI commands are claimed tested by this audit.

Subsequent interop must cover modern GMAC and HS256 segments, both entry names, both ZIP layouts, omitted sizes, empty/binary/exact-boundary/multisegment data, metadata, RSA and P-256 wrap/session combinations, Bearer and enforced DPoP, token expiry, nonce retries, cancellation, denied policies, and tamper/malformed/unsupported feature rejection. Legacy encodings, multi-KAS, assertions, obligations and additional wrapping schemes require explicitly scoped cases before support is claimed. A 7-target × 2-direction × 2-reference baseline is 28 producer/consumer pairs; browser is an additional TypeScript host run. No pair has passed yet.
