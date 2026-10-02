# OpenTDF SDK implementation plan

Build an OpenTDF SDK from one Go source implementation using Goalchemy, with usable generated libraries on all seven targets. The first release must create and decrypt TDF3 files using real platform authentication, authorization, and KAS rewrap. TypeScript must work in both Node and browsers.

The project lives in `sdk/`, beside the cloned Goalchemy, platform, and web SDK repositories. Extend the local Goalchemy clone as needed. Use the existing Go and TypeScript SDKs as independent compatibility references and their CLIs for operational smoke tests.

The active objective is: **Ship interoperable TDF3 encryption/decryption SDKs across all seven targets, verified against OpenTDF and real KAS.** The user narrowed the goal on 2026-10-02. Phases 0–7 and the [delivery checklist](delivery-checklist.md) define completion. The pinned reference inventory remains useful for compatibility and explicit rejection of unsupported input; broader service APIs, streaming, advanced schemes and full Go SDK parity are outside this goal.

## Execution and review

The root agent is the orchestrator. Run at most one worker agent at a time. A phase can require several small worker assignments; finish and review one assignment before starting the next. Workers must not spawn additional agents. Workers run in the background. Keep the root thread available to the user; inspect status and delivered results without long blocking waits or worker polling loops. Long verification commands should return a background process handle for later inspection.

Use GPT-6.1 Sol with high reasoning effort for crypto, protocol, compiler, scheduler, and exported API work. Use medium effort for bounded setup, packaging, documentation, and straightforward adapters. Give explicitly configured workers a self-contained task and the relevant files; the first reference audit was already started with inherited settings. Escalate a routine task to high effort when evidence shows a harder dependency.

Each assignment specifies the objective, allowed files, dependencies, acceptance checks, and required handoff. The orchestrator owns this plan and status updates. Workers report changed files, exact commands and outcomes, known gaps, and the next dependency. Shared workspace changes are immediately visible, so ownership must stay explicit.

The user's current cadence for each remaining SDK target is: implement the target
→ focused checks → required real-KAS matrix → one acceptance review → commit
→ next target. Run broader preservation checks when a shared change or concrete
failure requires them. Retain failures and use focused repairs at the original
budgets; repeat passed checks only when a new change or demonstrated gap warrants
it. This cadence supersedes older per-target broad-suite workflows. Java and C# have
been accepted and committed; Python is the next sole worker.

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
- [x] Phase 2: Capability contracts and native Go implementations
- [x] Phase 3: Shared TDF3 implementation and native interop
- [x] Phase 4: Seven-target byte/host foundation and generated Go library
- [x] Phase 5: TypeScript SDK for Node and browsers
- [ ] Phase 6: Java, C#, Python, Rust, and C SDKs in sequence
- [ ] Phase 7: Interop CI and TDF3 delivery readiness

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

Phase 2 accepted native Go HTTP and cooperative source checking; generated HTTP deliberately reports an unavailable capability until Phase 4 implements pending host operations and cancellation. Go-emitted crypto, encoding, and wall time are verified. This boundary preserves the native interop sequence and does not waive generated HTTP or any target requirement. See [capability contract and evidence](capabilities.md).

## Phase 3 Shared TDF3 implementation and native interop

Implement the first release in shared subset Go, in this order: byte/encoding helpers; constrained JSON reader/writer; ZIP writer/reader; manifest and policy; segment encryption and integrity; RSA key access; OAuth token acquisition and request signing; real rewrap; EC key access; DPoP challenges and refresh behavior.

Keep the initial public API based on bytes and explicit configuration, with structured errors and context cancellation. Start with one KAS and known wrapping information, then add the discovery needed for ordinary use. Track multi-KAS/key splitting, streaming, assertions, metadata, obligations, legacy versions, and additional algorithms explicitly in the compatibility inventory. Reference files that contain a mandatory unsupported feature must fail clearly rather than silently dropping verification.

Implement a bounded ZIP writer for in-memory inputs and a reader that accepts ZIP32 and the ZIP64/data-descriptor forms emitted by the pinned reference writers. Preserve TDF entry naming. Verify the root signature and every segment's integrity and declared size before returning successful plaintext. Define how unsupported manifest variants and assertions fail. Validate KAS destinations before sending credentials, following an explicit caller configuration or discovered allowlist.

Run both directions against both pinned SDKs through real KAS: new SDK encryption followed by reference decryption, and reference encryption followed by new SDK decryption. Compare bytes, not ciphertext, because encryption is randomized.

Acceptance: native Go interop passes for both references; cases include empty/binary payloads, exact segment boundaries, multiple segments, allowed and denied policy, RSA and EC, token expiry/cancellation, DPoP nonce retry, tampered ciphertext, root/segment metadata tampering, malformed archives, and unsupported features. Maintain deterministic format fixtures separately from live network tests.

## Phase 4 Compiler foundation and generated Go library

Phase 4 is accepted for the seven-target byte/host foundation and the first
importable generated Go SDK. The remaining target library exports are implemented
and accepted together with their production adapters in Phases 5–6. This sequencing
lets each export boundary execute the actual SDK; all seven importable libraries
remain delivery requirements. Earlier progress entries marking Phase 4 open refer
to the previous all-target-export grouping.

Assign and review the compiler tasks separately.

Start from the current [library-target gate](../../goalchemy/internal/driver/emit.go), [TypeScript slice representation](../../goalchemy/targets/typescript/types/slice.ts), [slice contracts](../../goalchemy/specs/types/slice.yaml), and [runtime contracts](../../goalchemy/specs/runtime/core/). Existing [slice-aliasing](../../goalchemy/tests/language/testdata/slices_alias/main.go) and [growth](../../goalchemy/tests/language/testdata/slice_growth/main.go) fixtures are regression baselines. Native byte storage has passed its bounded acceptance checks on all seven targets: Go retains its standard byte slices, and TypeScript, Java, C#, Python, Rust and C now use specialized native backing. Generated Go scheduler/HTTP, the [TypeScript portable host lifecycle](../../goalchemy/docs/typescript-host-operations.md), the [Java host lifecycle](../../goalchemy/docs/java-host-operations.md), the [C# host lifecycle](../../goalchemy/docs/csharp-host-operations.md), the [Python host lifecycle](../../goalchemy/docs/python-host-operations.md), and the [Rust host lifecycle](../../goalchemy/docs/rust-host-operations.md) have passed bounded acceptance; the [C host lifecycle](../../goalchemy/docs/c-host-operations.md) has also passed collector/sanitizer/native/emitted acceptance. All seven generic host prerequisites and the [generated Go SDK](generated-go-library.md) are accepted; Phase 5 also accepts the TypeScript Node/browser SDK. Phase 4 host evidence covers actual Node and Chromium executable frames; Phase 5 adds production HTTP/crypto adapters and SDK libraries. Every specialization must preserve arbitrary Go string bytes rather than substitute a UTF-8 text decoder. See [TypeScript design and evidence](../../goalchemy/docs/typescript-byte-storage.md), [Java design and evidence](../../goalchemy/docs/java-byte-storage.md), [C# design and evidence](../../goalchemy/docs/csharp-byte-storage.md), [Python design and evidence](../../goalchemy/docs/python-byte-storage.md), [Rust design and evidence](../../goalchemy/docs/rust-byte-storage.md) and [C design and evidence](../../goalchemy/docs/c-byte-storage.md). These prerequisites establish generic host behavior; production adapters and SDK libraries for Java, C#, Python, Rust and C follow in Phase 6.

1. Specialize `[]byte` storage on non-Go targets. Preserve slice identity, offset/length/capacity, nil versus empty, append growth, aliasing, `copy` overlap, strings containing arbitrary bytes, and conversion semantics. Test language behavior and measure memory on representative SDK payloads.
2. Add pending host operations to the scheduler. Register requests and completions, retain buffers/keys for the declared lifetime, resume only through the scheduler, and implement cancellation and shutdown. No runnable tasks with pending I/O must not be reported as deadlock. Host deadlines and wall-clock time must work alongside the existing virtual-clock conformance fixtures.
   Generated Go scheduler/HTTP has passed bounded acceptance, including an emitted SDK real-KAS RSA/Bearer probe in both directions against both references. TypeScript's portable Promise lifecycle has also passed bounded acceptance in Node and actual Chromium, Java's generic worker/mailbox lifecycle has passed actual JVM and emitted-program checks, and C#'s Task/mailbox lifecycle has passed actual CLR and emitted-program checks, and Python's calling-owner lifecycle has passed actual CPython and emitted-program checks. Rust's owned-wire/native-thread host lifecycle and C's native-wire/collector lifecycle have also passed native/emitted acceptance. All seven generic ports are accepted; importable SDK libraries and production non-Go capability adapters follow. Review registration/completion races, cancellation cleanup, stale callbacks, scheduler ownership and real versus virtual deadlines before ports depend on the design. These bounded host acceptances do not complete library, production-adapter or SDK/KAS delivery requirements.
3. Establish the importable Go TDF3 library boundary, then port it with each target SDK in Phases 5–6. Convert host bytes/configuration/results, expose structured errors and support calls that suspend. A minimal encryption/decryption façade may construct and close shared clients internally; exporting every source type/method is not required. Define initialization and overlapping-call behavior, preserve independent caller configuration, and test cancellation, cleanup and failure propagation. Persistent clients, if exposed, must preserve their documented lifetime. C needs explicit output ownership and release rules; host exceptions must not turn into source panics or process exits. The [library acceptance requirements](library-requirements.md) define this TDF3 boundary; toy exports alone cannot satisfy it.

Acceptance: relevant compiler/runtime conformance passes on every affected target; exported APIs can be called from small native consumers; asynchronous completion and cancellation tests pass; byte semantics hold under aliasing; existing runnable program behavior still passes. Phase 4 acceptance covers all-seven byte/host conformance and the generated Go export/SDK matrix. Remaining target exports retain the same acceptance criteria in their SDK phases.

## Phase 5 TypeScript SDK for Node and browsers

Use TypeScript as the first non-Go generated SDK to exercise the combined byte, library, and async changes. Replace direct Node dependencies in shared runtime conversion, output, failure, and scheduler configuration with portable behavior or explicit host adapters. Emit importable JavaScript and accurate TypeScript declarations rather than relying on Node's direct `.ts` execution.

Implement crypto with Web Crypto where it covers the pinned profile, HTTP with `fetch`, and byte APIs with `Uint8Array`. Export promise-based SDK operations. Support a token-provider/public-client path for browsers and client credentials for appropriate non-browser consumers. Any required browser-specific PEM/JWK conversion must preserve the shared protocol's exact representation.

Acceptance: an independent Node application and a real browser test import the generated package; both interop directions pass against both references using real KAS; DPoP challenges, cancellation, invalid inputs, and byte conversions behave correctly. No Node-only import is present in the browser dependency graph.

Phase 5 is accepted. Independent installed ESM consumers work in Node and actual
Chromium, with an 88-input browser SDK graph free of Node dependencies. Real KAS
BASIC/EC/enforced-DPoP matrices pass, with stock Web enforced-nonce401 limitations
recorded separately. Ownership, queued/active cancellation, source failures,
crypto lifetimes, UTF-8 and transport rejection checks pass. Original compiler
campaign failures and its cache-processing termination remain retained alongside
terminal current supplements. Java and C# are accepted in Phase 6; three remaining
SDKs and Phase 7 stay open. See [TypeScript delivery](generated-typescript-library.md)
and [the progress log](progress.md) for precise evidence and limits.

## Phase 6 Remaining target SDKs

Implement and accept each target before starting the next. Each target needs capability implementations, library adapters, package/dependency generation, primitive conformance, a native consumer smoke test, and the same real KAS interop cases.

| Order | Target | Proposed primitives and delivery |
| --- | --- | --- |
| 1 | Java (accepted) | JDK 21 JCA/HTTP plus pinned BC 1.86 for HKDF/omitted-Q P256, named-package JAR, owned bytes and cancellable async/error API |
| 2 | C# (accepted) | .NET crypto and `HttpClient`, class library with byte APIs, cancellation, and async calls |
| 3 | Python | `cryptography` plus a bounded HTTP adapter, installable package and documented sync/async behavior |
| 4 | Rust | Maintained crypto/HTTP crates, generated Cargo package and lockfile, idiomatic `Result` and clear key/resource ownership |
| 5 | C | OpenSSL and libcurl, headers/library, explicit handles/buffer release, documented async driving model, existing collector integration |

Avoid blocking the cooperative scheduler during host network I/O. Java, C#, Python, Rust, and C adapters must define their completion behavior as carefully as TypeScript. Validate TLS verification, cancellation, platform errors, and actual RSA/EC/JOSE parameters rather than assuming host defaults align.

Acceptance per target: install/build from a clean environment; all required primitive conformance checks pass; the native consumer calls the SDK as a library; both interop directions pass against both references; no skipped required tests; dependency versions and licenses are recorded.

Java is accepted with an independently importing native consumer, reproducible
JAR, 170 boundary/lifecycle checks, 59 native crypto checks and artifact-scoped
BASIC/EC/enforced-DPoP matrices. The final library dispatch repair has a fresh
real-KAS supplement and independent native launch-failure recovery proof.
Original broad failures and passing scoped repairs remain visible in
[the progress log](progress.md). C# is accepted with reproducible .NET 8 DLLs,
93 boundary/lifecycle checks, 39 native crypto checks and artifact-scoped
BASIC/EC/enforced-DPoP matrices. Focused final checks cover cancellation,
callback fault handling and detached source-panic ownership; current emitted
runtime files match the repository. See [C# delivery](generated-csharp-library.md).
Python is the next sole worker, followed by Rust and C; Phase 6 and final
Phase 7 delivery remain open.

## Phase 7 Interop CI and TDF3 delivery readiness

Run a pinned platform stack in CI with separate bounded jobs for targets. Test seven targets in both directions against both reference SDKs: 28 baseline producer/consumer pairs. The TypeScript browser environment is an additional run of the TypeScript target. RSA/EC, Bearer/DPoP, payload sizes, and negative cases expand each applicable pair rather than being implied by the count.

Provide fast offline format/conformance tests and a clearly named integration suite requiring real services. Cache dependencies without hiding version drift. Record compiler revision, reference revisions, platform configuration, target dependencies, and test outcomes. CI must fail when a required integration case is skipped or when the target only succeeds as an executable and has no usable library export.

Document APIs, setup, dependencies, limits, supported TDF profiles, unsupported features, and package installation. Check clean builds and ownership/error behavior at host boundaries. Keep release artifacts reproducible; do not publish packages without an explicit release instruction.

Acceptance: the complete baseline matrix and required negative cases pass in CI; generated packages work from native consumers; documentation matches the supported TDF3 profile and explicit limitations; Phase 0 through Phase 6 criteria and the delivery checklist are satisfied. This completes the current seven-target TDF3/KAS goal. Public package publication is a separate action requiring an explicit release instruction.

## Deferred reference features outside this goal

The Phase 0 inventory preserves broader reference functionality for possible future work: general platform service clients, policy autoconfiguration, multi-KAS/key splitting, streaming/seekable APIs, assertion creation/verification beyond the declared profile, obligation fulfillment, additional auth grants, legacy formats and advanced encryption schemes. These do not gate the current delivery. An input requiring an unsupported feature must still fail clearly before successful plaintext; enforcing that rejection remains in scope.

Complete SDK/API parity would need a separate feature-by-feature plan and acceptance audit. The current implementation must advertise only its tested TDF3 profiles and byte API.

## Historical initial findings and uncertainties

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
