# SDK compatibility inventory

This records the intended scope against the revisions in [references.lock.json](../references.lock.json). **The new SDK is not implemented yet; no feature below is currently claimed supported.** Full feature parity with `platform/sdk` on all seven targets is required before the overall goal is complete. The TDF3/KAS first release is an intermediate milestone. “Required” in the tables means a first-release acceptance requirement on Go, TypeScript, Python, Java, C#, Rust and C. TypeScript requires both Node and browser library consumers. “Later” means deferred from that milestone, but reference Go SDK functionality still must be implemented and tested before full-parity completion. Web-only conveniences can be separately scoped. “Reject” means the first release must fail clearly on an unsupported input or option; it does not permanently exclude a Go-reference feature from the overall goal. Protocol details and differences are in [tdf-poc.md](tdf-poc.md).

## Format and encryption

| Area | Pinned reference behavior | First release / subsequent scope |
| --- | --- | --- |
| Modern ZTDF/TDF3 | Both write manifest `4.3.0`; ZIP contains `0.payload`, `0.manifest.json` | Required create/decrypt; writer may emit spec `manifest.json`; read both |
| Manifest entry precedence | Both prefer `manifest.json` when both names exist | Required; invalid preferred entry must not fall back |
| ZIP layouts | Go ZIP32/ZIP64 based on size/unknown input; web always ZIP64; stored entries and descriptors | Required reader for both; bounded byte API writer initially |
| ZIP compression, multipart/archive variants | Neither inspected writer uses compression or multiple payload/manifest pairs | Reject until separately supported; do not interpret arbitrary ZIP as TDF |
| Payload encryption | AES-256-GCM with 12-byte nonce, 16-byte tag, no AAD; 32-byte DEK | Required native crypto on every target |
| Segmentation | Go 2 MiB default/16 KiB–4 MiB option clamp; web 1 MiB, omitted default sizes | Required mixed reference sizes and independent size omission; validate framing |
| Empty payload | Go one empty encrypted segment; web no segments | Required fixtures for both behaviors and new writer interop; runtime still unverified |
| Segment integrity | GMAC default; HS256 accepted; web has strict algorithm allowlist | Required GMAC/HS256 verification; reject unknown algs |
| Root integrity | HS256 only; Go accepts absent root alg, web does not | Required HS256; reject GMAC/unknown; document absent-alg legacy choice |
| Policy binding | Base64 of ASCII hex HMAC of original base64 policy, keyed by share | Required exact binding bytes and tamper rejection |
| Policy contents | UUID, attribute FQNs, dissemination list | Required basic attributes and real allow/deny policy; advanced policy construction later |
| MIME type | Go supplies octet-stream default; web writes when configured | Required preserve/inspect basic field; content-type is not cryptographic authority |
| Encrypted metadata | Go opt-in; web emits encrypted empty metadata by default | Required parse/decrypt ordinary reference metadata, expose supported bytes explicitly; richer metadata APIs later |
| RSA wrapping | OAEP-SHA1, empty label; Go/web support RSA-2048 and additional key sizes | Required RSA-2048; other sizes explicitly unsupported until tested |
| EC wrapping | Go supports P-256/P-384/P-521 helpers; web EC KAO constructor generates P-256; service preview gated | Required P-256 KAO and session rewrap; reject unsupported curves/types; broader curves later |
| Client session key | Separate from auth key and KAS key; Go defaults RSA-2048 | Required RSA-2048 and P-256 sessions; test combinations independently of KAO wrapping type |
| KAO `kid`/version | Optional kid identifies KAS key; KAO version `1.0` | Required preserve/send kid, parse supported schema; unknown mandatory profile rejects |
| Legacy 4.2.2 | Hex-before-base64 signatures, omitted version; readers/writers have different version-selection rules | Later compatibility; first reader must either implement verified legacy profile or reject before successful decryption |
| Legacy HS256 segments | Web legacy helper UTF-8 transforms ciphertext, Go uses raw bytes | Later dedicated cross-reference fixtures; no implied compatibility |
| Assertions | Go `WithSystemMetadataAssertion` opt-in; web `systemMetadataAssertion` opt-in; readers verify by default | First files can omit assertions. Any incoming assertion must be verified or explicitly rejected; never silently disable verification. Full custom/assertion APIs later |
| HTML wrapper | Web has `unwrapHtml` utility; high-level OpenTDF reader detects ZIP prefix | Later wrapper support; reject unsupported format |
| Unencrypted TDF | Experimental Go writer has different plaintext/integrity path | Later; first release encrypted-only |
| Hybrid and post-quantum | Go supports `hybrid-wrapped`/X-Wing and `mlkem-wrapped`; web has pure ML-KEM-768/1024 paths; service preview gated | Later; reject these KAOs and unrelated key algs explicitly |

Source map: [Go models](../../platform/sdk/manifest.go), [Go TDF options](../../platform/sdk/tdf_config.go), [Go stable/chunked implementation](../../platform/sdk/tdf.go), [chunked manifest](../../platform/sdk/chunked_writer.go), [experimental SDK](../../platform/sdk/experimental/tdf/doc.go), [web TDF implementation](../../web-sdk/lib/tdf3/src/tdf.ts), [web builders](../../web-sdk/lib/tdf3/src/client/builders.ts), [web KAO algorithms](../../web-sdk/lib/tdf3/src/models/key-access.ts), [web wrapper](../../web-sdk/lib/tdf3/src/utils/unwrap.ts). The RSA/P-256 requirement above narrows first-release algorithms, not target languages.

## KAS, authentication, and authorization

| Area | Pinned reference behavior | First release / subsequent scope |
| --- | --- | --- |
| Real KAS | Authenticated rewrap verifies binding and consults authorization | Required on every target; offline unwrap/mock is not acceptance |
| Connect RPC | `/kas.AccessService/Rewrap` and `/kas.AccessService/PublicKey`; protobuf models/bytes | Required unary JSON implementation with exact field names and structured transport errors |
| Older REST paths | Web retains `/kas/v2/rewrap` and `/kas/v2/kas_public_key` fallbacks; pinned server has no corresponding registered binding | Later if requested; do not presume these paths exist |
| KAS/platform URL mapping | Go strips KAS path to origin; web removes trailing `/kas`/`/v2/rewrap` | Required explicit base/KAS settings; reverse-proxy behavior must be tested/documented |
| Signed request token | Signed by auth key, carries requestBody JSON string; Go 60-second expiry, web ±1-hour timestamps | Required RS256/ES256 signing and key separation; wall-clock expiry; use bounded lifetime |
| Response grouping | Permit/fail per KAO under policy IDs; modern grouped response; legacy response upgrade | Required modern IDs/status validation; legacy response support only if tested |
| Key discovery | PublicKey, well-known base key, key registry/default KAS; base key may override requested wrapping algorithm | Required enough discovery for ordinary single-KAS use plus explicit-key mode; reject unsupported discovered algorithm |
| Allowed destinations | Both SDKs have KAS allowlist support; web compares origins; Go normalizes host/port | Required configured/discovered allowlist before sending credentials; precise semantics documented |
| Single KAS/share | One bound wrapped share | Required initial profile |
| Multi-KAS/key splitting | Distinct sid shares XOR to DEK; alternatives for same sid are OR; policy grants/autoconfiguration build split plans | Later; reject mandatory multiple-share inputs until implemented, including autoconfiguration that selects them |
| Bulk rewrap/decrypt | Go groups policies/KAOs and exposes bulk operations; web uses one KAO per attempt and concurrent pools | Later public bulk API; first implementation must still understand modern grouped response |
| Obligations | Client sends fulfillable FQNs; KAS returns required FQNs per result, may deny without fulfillment | Later fulfillment APIs; first release must surface/reject required obligations, never discard enforcement information |
| Client credentials | Go secret via Basic, web secret in form; both have other credential options | Required server-side client credentials with expiration-aware cache, cancellation and useful errors |
| External access-token provider | Go custom/oauth token sources; web token-provider/interceptor hooks | Required browser-safe token-provider path; preserve token scheme and matching DPoP key |
| DPoP proof and SRT binding | Go nonce-aware transport handles token scheme; server requires cnf.jkt match | Required ES256/RS256, resource ath, JWK thumbprint, nonce retry/cache, fresh jti and signed request token |
| Web DPoP reference gaps | Legacy/new interceptors send Bearer; new interceptor omits ath; no observed nonce retry | Track stock failures separately; use explicit reference-runner auth adapter for enforced cases if needed; do not attribute adapted behavior to stock SDK |
| Other signing algorithms | Go allows RS384/512 and ES384/512; KAS additionally permits RSA-PSS family | Later; fail clearly on unsupported selection |
| Token refresh/exchange | Web supports refresh token and external-JWT exchange; Go supports token exchange, cert/private-key credentials, custom OAuth source | Later native grants beyond first requirements; public token provider may refresh externally |
| Interactive login | Web app implements authorization-code/PKCE; Go CLI has login/auth commands | Browser host integration required, shared redirect UI optional/later; no client secret in browser |
| CORS/TLS | Browser fetch/Web Crypto and platform transport deployment requirements | Required real browser validation; trusted TLS/allowed origin and readable nonce headers; test actual runtime settings |

Sources: [Go KAS client](../../platform/sdk/kas_client.go), [Go SDK initialization/default signing](../../platform/sdk/sdk.go), [Go auth transport](../../platform/sdk/auth/dpop_transport.go), [Go auth options](../../platform/sdk/options.go), [Go OAuth](../../platform/sdk/auth/oauth/oauth.go), [KAS wire](../../platform/service/kas/kas.proto), [KAS enforcement](../../platform/service/kas/access/rewrap.go), [platform DPoP](../../platform/service/internal/auth/authn.go), [web access/discovery](../../web-sdk/lib/src/access.ts), [web auth](../../web-sdk/lib/src/auth/oidc.ts), [web interceptors](../../web-sdk/lib/src/auth/interceptors.ts), [browser example](../../web-sdk/web-app/src/session.ts).

The basic stack must retain authentication. The checked-in dev config disables DPoP enforcement and EC preview; successful Bearer/RSA smoke tests do not establish EC or DPoP support. Separate configs must provision EC keys and enable strict/enforced DPoP/nonce tests. Discover and record issuer, audience, available algorithms, key IDs, routing and CORS at runtime ([dev config](../../platform/opentdf-dev.yaml), [KAS preview](../../platform/service/kas/access/provider.go)).

## Public SDK APIs and required later parity

The first public API uses bytes plus explicit configuration and structured errors. It must support cancellation/deadlines and concurrent independent calls as defined by each generated host adapter. API compatibility is distinct from feature compatibility: target APIs do not need literal translations of Go functional options or TypeScript builders.

| Reference area | Initial decision |
| --- | --- |
| Go `CreateTDF`/`LoadTDF`/Reader | Required equivalent encrypted byte operations; io.Reader/io.Writer, ReadAt/Seek/WriteTo APIs later |
| Go `DecryptBytes`/`DecryptTo`/`DecryptFile` | Required byte equivalent; stream/file helpers later with no partial successful output on failure |
| Go chunked writer (parallel/out-of-order segments, manifest finalization, partial segment selection) | Later; do not claim streaming from an in-memory API |
| Go experimental writer/reader/keysplit | Later; experimental profiles stay separate from stable supported profile |
| Web `OpenTDF`/TDF3Client streams, File/URL/chunker sources | Required byte/Uint8Array equivalent in Node/browser; stream/range URL APIs later |
| Manifest/attributes/metadata inspection | Basic safe inspection useful for first API; complete reference inspection methods later |
| Assertion key lookup, signing, canonical hashes and binding | Later unless needed to accept test files; unsupported assertions fail explicitly |
| Base key/policy autoconfiguration, attribute grants/mappings, offline split planning | Later complete parity; first explicit config/discovery must expose unsupported outcome |
| Policy service clients | Later: namespaces, attributes/values, actions, resource/subject/dynamic-value mappings, registered resources, obligations, KAS registry/key management and unsafe APIs |
| Authorization/entity resolution clients | Later: v1/v2 where reference exposes them, entitlement/access evaluation |
| Platform well-known/config, connection customization and validation, logging/cache options | Minimal discovery/config first; full options/custom-client parity later |
| CLI commands/profiles/policy administration | Operational references/test runners; replacement CLI not a first-release SDK deliverable |
| Supported-feature advertisement | Advertise only tested new-SDK features; skipped interop tests must fail required acceptance |

Sources: [Go SDK service surface](../../platform/sdk/sdk.go), [Go decrypt APIs](../../platform/sdk/decrypt.go), [Go chunked options](../../platform/sdk/chunked_options.go), [experimental package](../../platform/sdk/experimental/tdf/doc.go), [Go autoconfiguration](../../platform/sdk/granter.go), [web OpenTDF](../../web-sdk/lib/src/opentdf.ts), [web service surface](../../web-sdk/lib/src/platform.ts), [web source handling](../../web-sdk/lib/src/seekable.ts), [Go feature advertisement](../../platform/sdk/version.go).

## Goalchemy work and target delivery

Pinned Goalchemy accepts restricted Go/module source and declared `lib/` capabilities; importing the entire Go SDK/dependency graph is not supported. Current capabilities are errors/context/time/sync/task/runtime, so crypto, encoding, HTTP, and wall-clock support need new contracts and target implementations. Shared ZIP/JSON/TDF/JWT logic stays in subset Go. Native keys should remain opaque handles with explicit lifetimes; the shared layer handles serialized PEM/JWK and protocol bytes. ([Goalchemy imports/capabilities](../../goalchemy/docs/usage.md), [existing library](../../goalchemy/lib/task/task.go).)

Compiler/runtime requirements precede usable generated SDKs:

- Efficient `[]byte` storage across non-Go targets, preserving slice aliasing, length/capacity, nil/empty, overlap, append/copy and arbitrary-byte string conversions. Current non-Go bytes are boxed ([known gaps](../../goalchemy/docs/followups.md)).
- Pending host crypto/network operations, completion/cancellation/shutdown, buffer/key retention and wall-clock deadlines. Current TypeScript scheduler reports deadlock when no runnable work/timers remain; fetch/Web Crypto completion must be represented ([scheduler](../../goalchemy/targets/typescript/runtime/task_spawn.ts)).
- Exported library calls on all targets, including suspension, byte/error/configuration conversion, instance lifetime, concurrent calls and cleanup. Existing exports are C-only, scalar/string only, non-suspending, single-thread collector entry; other library targets are rejected ([library limits](../../goalchemy/docs/followups.md), [usage](../../goalchemy/docs/usage.md)).
- Portable TypeScript runtime and compiled JavaScript/package exports/declarations. Existing runtime uses `node:fs`, `Buffer`, `process.env` and `process.exit`; `.ts` direct execution assumes Node. These are removable host dependencies, not a fundamental browser restriction ([printing](../../goalchemy/targets/typescript/types/print.ts), [UTF-8](../../goalchemy/targets/typescript/types/utf8.ts), [scheduler](../../goalchemy/targets/typescript/runtime/task_spawn.ts), [output packaging](../../goalchemy/docs/usage.md)).

| Target | Required native boundary and delivery (proposed, not implemented) |
| --- | --- |
| Go | Standard crypto/net/http wrappers; importable package; context and errors |
| TypeScript | Web Crypto/fetch, Uint8Array, promise exports; compiled Node/browser ESM package and declarations |
| Java | JCA plus java.net.http; byte[] and usable library artifact; explicit async/cancellation/error mapping |
| C# | .NET crypto/HttpClient; byte arrays, Task/cancellation; class library |
| Python | Maintained cryptography dependency plus HTTP adapter; installable package; documented sync/async behavior |
| Rust | Maintained crypto/HTTP crates; Cargo package/lockfile; Result and explicit resource ownership |
| C | OpenSSL/libcurl plus existing collector; headers/library; buffer/key ownership and completion-driving contract |

Crypto parameters must match [the audit](tdf-poc.md), especially OAEP-SHA1, empty HKDF info, SPKI public PEM and raw JOSE ECDSA. HTTP capability requires GET/POST, status, response headers/body, limits, deadlines and cancellation. Exact dependency packages/versions/licenses are selected and recorded before adoption. Existing Rust output already includes Cargo scaffolding but its run script uses plain rustc/standard library; supporting dependency crates requires adapting that build path ([output description](../../goalchemy/docs/usage.md)). No toolchain availability beyond observed environment checks is assumed.

## Acceptance ledger

Phase 0 evidence is source inspection and link/whitespace validation only. All live and generated results are **pending**. The first release requires 28 baseline target/direction/reference pairs plus a real browser run, expanded by RSA/P-256, Bearer/DPoP and negative cases. A runnable generated executable is not proof of an importable SDK; self-round-trip is not cross-reference compatibility; skipped tests do not pass. Each deferred row must remain visible in release documentation, and an unsupported mandatory feature must produce a useful error before a successful decrypt result.
