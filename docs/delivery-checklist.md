# Seven-target TDF3 delivery

The active goal, narrowed by the user on 2026-10-02, is: **Ship interoperable
TDF3 encryption/decryption SDKs across all seven targets, verified against
OpenTDF and real KAS.** This checklist and Phases 0–7 of [the plan](plan.md)
define completion. Historical progress entries describing full SDK parity are
superseded by this scope.

## Required product

One shared Go implementation must provide TDF3 encryption/decryption through
usable Go, TypeScript, Java, C#, Python, Rust and C libraries. TypeScript must
work in Node and an actual browser. Native consumers must import/link the built
SDK; executable wrappers and output scraping do not satisfy delivery.

A minimal public encryption/decryption façade is sufficient. It must execute
the shared implementation and expose supported configuration, payload/metadata
results and useful errors. It may construct and close a shared client per call.
Persistent clients, public key handles and extra methods are optional exported
surfaces; any exposed surface must preserve its documented behavior. See
[library requirements](library-requirements.md) for ownership, callbacks,
suspension, cancellation, initialization and repeated-call acceptance.

## Supported profile and rejection behavior

The existing [compatibility inventory](compatibility.md) marks delivery features
as Required and deferred features as Later. Required profiles remain:

- Modern encrypted TDF3/manifest 4.3, stored ZIP32/ZIP64 and reference data
  descriptors, both manifest entry names with defined precedence.
- AES-256-GCM payloads, exact segmentation/integrity/policy binding, both
  reference empty-payload forms, arbitrary binary bytes and encrypted metadata.
- Single-KAS policy authorization, trusted destination routing, ordinary key
  discovery and explicit-key configuration, modern Connect JSON rewrap and
  validation of grouped result identifiers/status.
- RSA-2048 and P-256 key access and response-session combinations; RS256/ES256
  request signing with distinct authentication and session keys.
- Server client credentials and browser-safe access-token providers; Bearer and
  enforced DPoP, expiry/cancellation, nonce handling and key binding.
- Bounded HTTP/input handling, real deadlines, TLS verification, cleanup and
  declared errors without successful partial plaintext after rejection.

Unsupported assertions, multi-share/advanced algorithms, legacy formats and
other mandatory unsupported features must fail explicitly. Full assertion APIs,
multi-KAS planning, streaming/seek, platform service clients, obligation
fulfillment, additional OAuth grants and complete Go SDK/API parity are outside
this goal. Rejecting unsupported enforcement requirements remains in scope.

## Evidence required for each target

Build each package from documented prerequisites and import it from an
independent native consumer. Verify repeated calls, input/output ownership,
documented overlapping-call behavior, cancellation/error propagation and release
of native resources. Retain meaningful compiler/runtime/byte conformance tests.
Record dependency versions/licenses and reproducible build/package instructions;
Rust uses a Cargo package and lockfile, and C has explicit headers/buffer release.

Against the pinned platform and references in [references.lock.json](../references.lock.json):

1. New SDK encrypts; stock Go and Web SDK consumers decrypt through real KAS.
2. Go and Web SDK producers encrypt; the new SDK decrypts through real KAS.
3. Compare exact payload bytes for empty, binary, segment-boundary and
   multisegment cases. Verify supported metadata separately where the reference
   exposes it; the Web reader's metadata limitation is documented in the inventory.
4. Expand both directions by the required RSA/P-256, Bearer/DPoP and negative
   cases: denied policy/authentication, expiry/cancellation, tampering,
   malformed input, untrusted destinations/TLS and unsupported mandatory features.
5. Exercise TypeScript from an actual browser against the platform, including
   CORS and a token-provider flow with no shipped client secret.

The baseline is 7 targets × 2 directions × 2 references = 28 pairs, plus the
browser run and profile/negative expansions. The pinned Web SDK's stock DPoP
failures must remain visible: any explicit auth-adapted reference runner is
labeled separately and cannot be claimed as stock Web behavior. A skipped
required case, mock KAS or self-round-trip cannot satisfy these checks.

## Current acceptance state

| Target | Accepted work | Delivery evidence still required |
| --- | --- | --- |
| Go | Importable generated SDK/native consumer; RSA/P256, RS256/ES256, Bearer/enforced-DPoP, metadata, ownership/cancellation and rejection matrices | Final clean package/CI delivery checks in Phase 7 |
| TypeScript | Importable Node/browser ESM SDK, native WebCrypto/fetch, RSA/P256, RS256/ES256, Bearer/enforced-DPoP, metadata, ownership/cancellation and rejection matrices | Final clean package/CI delivery checks in Phase 7 |
| Java | Importable JAR/native consumer; JCA/BC crypto, bounded HTTP, RSA/P256, Bearer/enforced-DPoP, metadata, ownership/cancellation and rejection matrices | Final clean package/CI delivery checks in Phase 7 |
| C# | Importable .NET 8 DLL/native consumer; built-in crypto/HTTP, RSA/P256, Bearer/enforced-DPoP, metadata, ownership/cancellation and rejection matrices | Final clean package/CI delivery checks in Phase 7 |
| Python | Installable wheel/native consumer; cryptography and bounded HTTP, RSA/P256, Bearer/enforced-DPoP, metadata, ownership/cancellation and rejection matrices | Final clean package/CI delivery checks in Phase 7 |
| Rust | Importable locked Cargo library/native consumer; maintained crypto/HTTP, RSA/P256, Bearer/enforced-DPoP, metadata, ownership/cancellation and rejection matrices | Final clean package/CI delivery checks in Phase 7 |
| C | Importable native archive/headers; OpenSSL/libcurl, owned values, collector/sanitizer lifecycle, RSA/P256, Bearer/enforced-DPoP and SDK/KAS matrices | Final clean package/CI delivery checks in Phase 7 |

Update this ledger from terminal evidence, not implementation intent. All seven
target SDK phases are accepted; Phase 7 CI/package checks remain required for
all seven targets. Commit after each verified phase in SDK and, when changed, Goalchemy.
Shipping here means reviewable, reproducible packages and passing delivery
evidence; public publishing requires a separate release instruction.
