# OpenTDF SDK implementation plan

Build an OpenTDF SDK from one Go source implementation using Goalchemy, with usable generated libraries on all seven targets. The first release must create and decrypt TDF3 files using real platform authentication, authorization, and KAS rewrap. TypeScript must work in both Node and browsers.

The project lives in `sdk/`, beside the cloned Goalchemy, platform, and web SDK repositories. Extend the local Goalchemy clone as needed. Use the existing Go and TypeScript SDKs as independent compatibility references and their CLIs for operational smoke tests.

Full feature parity with the pinned OpenTDF Go SDK on every target is the final objective. It is broader than TDF3 and KAS: the reference exposes platform service clients, discovery, streaming, assertions, obligations, and additional encryption schemes. Phase 0 inventories these features, and Phase 8 implements the remaining required scope. Do not claim full parity when only the first release has passed.

## Execution and review

The root agent is the orchestrator. Run at most one worker agent at a time. A phase can require several small worker assignments; finish and review one assignment before starting the next. Workers must not spawn additional agents.

Use GPT-6.1 Sol with high reasoning effort for crypto, protocol, compiler, scheduler, and exported API work. Use medium effort for bounded setup, packaging, documentation, and straightforward adapters. Give explicitly configured workers a self-contained task and the relevant files; the first reference audit was already started with inherited settings. Escalate a routine task to high effort when evidence shows a harder dependency.

Each assignment specifies the objective, allowed files, dependencies, acceptance checks, and required handoff. The orchestrator owns this plan and status updates. Workers report changed files, exact commands and outcomes, known gaps, and the next dependency. Shared workspace changes are immediately visible, so ownership must stay explicit.

After each handoff, the orchestrator reads the changes, runs the appropriate checks, and either accepts the task or assigns a focused repair. Required repository instructions apply to changes inside each clone. Do not count skipped tests, a mock KAS, an executable wrapper, or a successful self-round-trip as evidence of generated SDK compatibility.

Continue through approved scope without requesting permission for routine reversible implementation or local testing. Bring material changes in scope or dependencies to the user. Public publishing, pushing, and release actions require separate authorization.

The user authorized Git initialization in `sdk/` and a commit after each accepted phase. The orchestrator commits reviewed changes with a Conventional Commit message using the configured Git identity and signing settings. If a phase changes Goalchemy, commit its relevant changes in that repository too and record both repositories' commit IDs in the SDK progress log. Preserve unrelated work and never commit generated credentials, keys, or test outputs. A commit marks verified phase completion; a partial or blocked phase must not be mislabeled complete.

Keep status in this file and detailed verification in `docs/progress.md`. Update both from observed results. A phase is complete only when its acceptance criteria pass or its narrower accepted scope is explicitly recorded.

## Design boundaries

| Area | Owner | Boundary |
| --- | --- | --- |
| TDF ZIP, JSON, manifest, segments, policy, JWT construction, and KAS protocol | `sdk/` shared source | Go code within Goalchemy's accepted subset |
| Cryptographic primitives, key handles, encoding, HTTP, and clocks | `goalchemy/lib/` and target implementations | Declared capability contracts with native Go wrappers and native target implementations |
| Bytes, suspension, host completion, and exported library calls | Goalchemy compiler and runtimes | Reusable language support rather than OpenTDF-specific behavior |
| Public target APIs and packaging | Generated libraries and minimal adapters | Natural byte, error, cancellation, and asynchronous representations for each host |
| Development services and interop tooling | `sdk/` | Pinned platform configuration, real KAS tests, and reference runners |

Use maintained native crypto. Expected external dependencies are Python `cryptography`, Rust crypto and HTTP crates through Cargo, and OpenSSL plus libcurl for C, in addition to Goalchemy's existing C collector. Exact packages, versions, licenses, and supported environments are recorded before adoption. Prefer Web Crypto and `fetch` for portable TypeScript primitives; async crypto must use the same host completion contract as async HTTP.

Do not transpile the reference SDK and its dependency tree wholesale. Goalchemy accepts project source and its declared library packages; the rewrite must preserve supported observable behavior through that boundary.

## Phase checklist

- [x] Phase 0: Reference audit and compatibility inventory
- [x] Phase 1: Basic local platform and reference smoke tests
- [ ] Phase 2: Capability contracts and native Go implementations
- [ ] Phase 3: Shared TDF3 implementation and native interop
- [ ] Phase 4: Compiler support for bytes, host operations, and library exports
- [ ] Phase 5: TypeScript SDK for Node and browsers
- [ ] Phase 6: Java, C#, Python, Rust, and C SDKs in sequence
- [ ] Phase 7: Interop CI and first release readiness
- [ ] Phase 8: Remaining SDK parity

## Phase 0 Reference audit and compatibility inventory

Pin the checked-out revisions in `references.lock.json`; preserve existing working tree edits. Read the Go and TypeScript writers, readers, crypto implementations, auth transports, KAS clients, protobuf definitions, and CLI behavior. Online docs provide context; pinned source and executable interop settle behavior.

Write `docs/tdf-poc.md` with exact ZIP entry names, manifest versions, segment framing and defaults, signature encodings, policy binding, key wrapping, KAS endpoints, request and response JSON, JWT claims, authentication keys, and DPoP behavior. Record RSA-OAEP parameters, EC curve/HKDF parameters, AES-GCM framing, and JOSE ECDSA signature representation. Distinguish the auth signing key from the session encryption key and the KAS wrapping key.

Write `docs/compatibility.md` mapping reference features to first-release requirements, later work, or an explicit unsupported status. Include Node versus browser auth and native API differences. Source inspection alone does not establish a running platform's supported algorithms or enforcement settings.

Acceptance: every protocol claim has a pinned source reference; reference differences and unknowns are explicit; the inventory prevents unsupported features from passing silently.

## Phase 1 Basic local platform and reference smoke tests

Create a minimal workspace-local setup with the monolithic OpenTDF service, Keycloak, and Postgres. Use the official quickstart as a template, adapting it to the pinned platform clone. The clone's default Compose file starts infrastructure only; the official all-container quickstart currently downloads from `main` and uses a `nightly` platform image, so it is not a reproducibility lock by itself.

Provide commands for initialization, readiness, provisioning, start, stop, logs, and status. Keep generated keys and credentials under ignored local storage. The setup must preserve other Docker projects and avoid global certificate or host-file changes by default. Choose and document reachable issuer/KAS URLs for both container services and host clients; a token's issuer must match the platform configuration.

Start with ordinary client-credentials auth and RSA. Keep authentication enabled. Add a separate DPoP-enforced test configuration and EC key access when the later protocol tasks require them. Browser tests need CORS support, including readable DPoP nonce headers, and a public-client/token-provider flow without shipping a client secret to the browser.

Use `platform/otdfctl` and `web-sdk/cli` to create and decrypt a small TDF. Pin builds of both CLIs to the reference revisions. Put each CLI's local profile and output in isolated test storage. Check the actual CLI help before scripting flags; online examples can differ from the clone.

Acceptance: an operator can start a fresh stack using documented commands; health and token acquisition succeed; a real KAS is used; Go and TypeScript reference files decrypt in both directions with plaintext bytes compared. Record actual KAS algorithms and DPoP settings. DPoP enforcement is a later test requirement, not a claim made from the basic setup.

## Phase 2 Capability contracts and native Go implementations

Define only the capabilities needed for the first release, with byte ownership, error behavior, key lifetime, suspension effects, cancellation, deadlines, and input limits specified. Use opaque native key handles where supported instead of exposing native types in shared source.

Implement native Go wrappers for secure randomness, AES-256-GCM, SHA-256, HMAC-SHA256, RSA-OAEP encryption/decryption, RSA and ECDSA signing/verification, RSA and EC key generation, ECDH, HKDF-SHA256, required PEM/JWK conversion, base64/base64url, and wall-clock time. RSA-OAEP's digest must follow the pinned protocol; it must not silently inherit a host default. PEM formats required by fixtures, including certificate public keys if needed, must be explicitly covered.

HTTP needs a method, URL, headers, request bytes, cancellation/deadline, bounded response bytes, status, and response headers. DPoP nonce retries require response headers. Avoid describing the capability as POST-only if issuer or KAS discovery requires GET. HTTP and Web Crypto suspension must be declared in contracts even while the native Go wrappers run synchronously from the caller's perspective.

Add independent known-answer and cross-library checks for algorithm parameters, encodings, key import/export, and rejection of invalid inputs. Native wrappers can use the Go standard library; the shared source can import only Goalchemy's accepted library boundary.

Acceptance: wrappers and contracts agree; operations pass primitive and encoding tests; unsupported target capabilities fail with useful diagnostics; the shared SDK can call the wrappers using ordinary Go and still pass the relevant Goalchemy source checks.

## Phase 3 Shared TDF3 implementation and native interop

Implement the first release in shared subset Go, in this order: byte/encoding helpers; constrained JSON reader/writer; ZIP writer/reader; manifest and policy; segment encryption and integrity; RSA key access; OAuth token acquisition and request signing; real rewrap; EC key access; DPoP challenges and refresh behavior.

Keep the initial public API based on bytes and explicit configuration, with structured errors and context cancellation. Start with one KAS and known wrapping information, then add the discovery needed for ordinary use. Track multi-KAS/key splitting, streaming, assertions, metadata, obligations, legacy versions, and additional algorithms explicitly in the compatibility inventory. Reference files that contain a mandatory unsupported feature must fail clearly rather than silently dropping verification.

Implement a bounded ZIP writer for in-memory inputs and a reader that accepts ZIP32 and the ZIP64/data-descriptor forms emitted by the pinned reference writers. Preserve TDF entry naming. Verify the root signature and every segment's integrity and declared size before returning successful plaintext. Define how unsupported manifest variants and assertions fail. Validate KAS destinations before sending credentials, following an explicit caller configuration or discovered allowlist.

Run both directions against both pinned SDKs through real KAS: new SDK encryption followed by reference decryption, and reference encryption followed by new SDK decryption. Compare bytes, not ciphertext, because encryption is randomized.

Acceptance: native Go interop passes for both references; cases include empty/binary payloads, exact segment boundaries, multiple segments, allowed and denied policy, RSA and EC, token expiry/cancellation, DPoP nonce retry, tampered ciphertext, root/segment metadata tampering, malformed archives, and unsupported features. Maintain deterministic format fixtures separately from live network tests.

## Phase 4 Compiler support for bytes host operations and libraries

Assign and review these compiler tasks separately:

1. Specialize `[]byte` storage on non-Go targets. Preserve slice identity, offset/length/capacity, nil versus empty, append growth, aliasing, `copy` overlap, strings containing arbitrary bytes, and conversion semantics. Test language behavior and measure memory on representative SDK payloads.
2. Add pending host operations to the scheduler. Register requests and completions, retain buffers/keys for the declared lifetime, resume only through the scheduler, and implement cancellation and shutdown. No runnable tasks with pending I/O must not be reported as deadlock. Host deadlines and wall-clock time must work alongside the existing virtual-clock conformance fixtures.
3. Add exported library mode for all targets. Initialize once per instance, convert host bytes/strings/configuration, expose structured errors, and support calls that suspend. Test overlapping calls, instance isolation, cleanup, background tasks, and failure propagation. C needs explicit output ownership and release rules; host exceptions must not turn into source panics accidentally.

Acceptance: relevant compiler/runtime conformance passes on every affected target; exported APIs can be called from small native consumers; asynchronous completion and cancellation tests pass; byte semantics hold under aliasing; existing runnable program behavior still passes.

## Phase 5 TypeScript SDK for Node and browsers

Use TypeScript as the first generated SDK to exercise the combined byte, library, and async changes. Replace direct Node dependencies in shared runtime conversion, output, failure, and scheduler configuration with portable behavior or explicit host adapters. Emit importable JavaScript and accurate TypeScript declarations rather than relying on Node's direct `.ts` execution.

Implement crypto with Web Crypto where it covers the pinned profile, HTTP with `fetch`, and byte APIs with `Uint8Array`. Export promise-based SDK operations. Support a token-provider/public-client path for browsers and client credentials for appropriate non-browser consumers. Any required browser-specific PEM/JWK conversion must preserve the shared protocol's exact representation.

Acceptance: an independent Node application and a real browser test import the generated package; both interop directions pass against both references using real KAS; DPoP challenges, cancellation, invalid inputs, and byte conversions behave correctly. No Node-only import is present in the browser dependency graph.

## Phase 6 Remaining target SDKs

Implement and accept each target before starting the next. Each target needs capability implementations, library adapters, package/dependency generation, primitive conformance, a native consumer smoke test, and the same real KAS interop cases.

| Order | Target | Proposed primitives and delivery |
| --- | --- | --- |
| 1 | Java | JCA crypto, `java.net.http`, importable artifact with byte arrays and appropriate async/error APIs |
| 2 | C# | .NET crypto and `HttpClient`, class library with byte APIs, cancellation, and async calls |
| 3 | Python | `cryptography` plus a bounded HTTP adapter, installable package and documented sync/async behavior |
| 4 | Rust | Maintained crypto/HTTP crates, generated Cargo package and lockfile, idiomatic `Result` and clear key/resource ownership |
| 5 | C | OpenSSL and libcurl, headers/library, explicit handles/buffer release, documented async driving model, existing collector integration |

Avoid blocking the cooperative scheduler during host network I/O. Java, C#, Python, Rust, and C adapters must define their completion behavior as carefully as TypeScript. Validate TLS verification, cancellation, platform errors, and actual RSA/EC/JOSE parameters rather than assuming host defaults align.

Acceptance per target: install/build from a clean environment; all required primitive conformance checks pass; the native consumer calls the SDK as a library; both interop directions pass against both references; no skipped required tests; dependency versions and licenses are recorded.

## Phase 7 Interop CI and first release readiness

Run a pinned platform stack in CI with separate bounded jobs for targets. Test seven targets in both directions against both reference SDKs: 28 baseline producer/consumer pairs. The TypeScript browser environment is an additional run of the TypeScript target. RSA/EC, Bearer/DPoP, payload sizes, and negative cases expand each applicable pair rather than being implied by the count.

Provide fast offline format/conformance tests and a clearly named integration suite requiring real services. Cache dependencies without hiding version drift. Record compiler revision, reference revisions, platform configuration, target dependencies, and test outcomes. CI must fail when a required integration case is skipped or when the target only succeeds as an executable and has no usable library export.

Document APIs, setup, dependencies, limits, supported TDF profiles, unsupported features, and package installation. Check clean builds and ownership/error behavior at host boundaries. Keep release artifacts reproducible; do not publish packages without an explicit release instruction.

Acceptance: the complete baseline matrix and required negative cases pass in CI; generated packages work from native consumers; documentation matches the compatibility inventory; Phase 0 through Phase 6 criteria are satisfied. This establishes the first TDF3/KAS release, not full SDK parity.

## Phase 8 Remaining SDK parity

Use the Phase 0 inventory to schedule the remaining features in small tasks, rechecking the pinned references before each implementation. Expected areas include platform service clients, policy discovery/autoconfiguration, multi-KAS grants and key splitting, streaming/seekable I/O, assertions and metadata, obligations, auth methods, legacy compatibility, and additional TDF/key-wrapping profiles. Some of these may become prerequisites earlier when reference interop requires them; update the plan when that happens.

Generate or implement shared protocol models/clients within the accepted language subset. Keep format and protocol decisions in shared source; keep native boundary adapters small. Add independent interop tests for every newly claimed feature across relevant targets.

Acceptance: every supported feature of the pinned Go SDK has implemented behavior, documentation, and passing tests on all seven targets. Deferral may describe an intermediate release; it cannot establish completion of the full goal. API compatibility and feature compatibility are separate claims; exact Go option APIs need not map literally to every language. The final completion audit must check the inventory feature by feature and require real-platform interop evidence for the relevant behavior.

## Initial findings and uncertainties

The pinned Goalchemy supports seven executable targets, but exported library builds are currently restricted to C and simple non-suspending parameters/results. Non-Go byte slices are boxed. The TypeScript runtime currently includes `Buffer`, `node:fs`, `process.exit`, and scheduler environment reads. These are implementation gaps to resolve, not fundamental browser restrictions.

The pinned development platform configuration disables DPoP enforcement and EC TDF preview. This does not prove what a running KAS serves. The reference rewrap request is signed with the authentication key and supplies a distinct client encryption public key. Exact protocol details belong in the Phase 0 audit.

Phase 1 verified a fresh local stack, pinned Go/Node reference builds, both reference interoperability directions, denied policies, and unauthenticated rejection. See [observed results](phase1-results.md). The basic profile exercises Bearer authentication and RSA-2048; enforced DPoP, EC, browsers, and compiler prerequisites remain subsequent work.

No wall-clock estimate is committed yet. The compiler host/library work and per-target native boundaries are the largest uncertainties. Estimate subsequent tasks from the native interop and first generated TypeScript results.

## Sources

- [Pinned repositories](../references.lock.json)
- [Goalchemy usage](../../goalchemy/docs/usage.md)
- [Goalchemy known gaps](../../goalchemy/docs/followups.md)
- [Go SDK](../../platform/sdk/README.md)
- [Platform consumer setup](../../platform/docs/Consuming.md)
- [TypeScript SDK](../../web-sdk/README.md)
- [OpenTDF documentation index](https://opentdf.io/llms.txt)
- [Official quickstart](https://opentdf.io/quickstart)
- [Official quickstart Compose](https://opentdf.io/quickstart/docker-compose.yaml)
