# OpenTDF SDK implementation progress

## Current status

Phases 0–2 are accepted. SDK commits for Phases 0 and 1 are `0e45fba` and `cc2b087`. Phase 2 capabilities are committed in Goalchemy as `7df8103`; its baseline audit revision is retained in the lockfile. Git is initialized in `sdk/` on branch `main`. Shared SDK format implementation is the next task; seven-target SDK interoperability is not yet implemented.

All seven targets are required. Future workers use GPT-6.1 Sol High for complex work and Medium for routine work, one at a time. The first audit worker inherited the orchestrator's settings. The orchestrator will commit after each verified phase, including a separate Goalchemy commit when that repository changes.

The active harness goal requires full feature parity with the pinned OpenTDF Go SDK on all seven targets and passing real-platform interop tests. The TDF3/KAS first release is an intermediate milestone, not a reduced definition of completion.

## Verification

The initial inspection found clean working trees in Goalchemy, platform, and web SDK. Go 1.25.1, Node 24.15.0, Docker 29.2.1, and Docker Compose 5.0.2 are installed. These are discovery results, not evidence that the clones build with those toolchains.

During Phase 0, the existing platform Compose and official quickstart were read without starting services. Phase 1 subsequently started the isolated `tdf-sdk` stack and verified real reference interoperability; no global host, certificate, or CLI profile configuration was changed.

## Next task

Implement shared subset-Go JSON, ZIP, manifest, policy, segment encryption and integrity, then real auth/rewrap and native interoperability. Assign bounded Sol High tasks sequentially, with workers running in the background. Root checks completed work without long blocking waits.
## Phase 0 acceptance

The worker wrote `tdf-poc.md` and `compatibility.md` from both pinned SDKs, platform KAS/auth/protobuf code, and Goalchemy source. The orchestrator reviewed the documents and independently verified 169 local links, document whitespace, and all three reference revisions. Review corrected the opt-in Go assertion option to `WithSystemMetadataAssertion` and replaced imprecise line anchors with links to the authoritative files.

Direct source checks confirmed RSA-OAEP SHA-1, the EC HKDF parameters, Connect procedure names, distinct authentication/session keys, and the web DPoP interceptor's differences. No live test is claimed. Runtime KAS algorithms, DPoP enforcement, empty payload behavior, routing, and CORS remain Phase 1 or subsequent integration evidence.

The first phase commit is `0e45fba` (`docs(sdk): complete phase 0 reference audit and implementation plan`). It contains an SSH signature using the existing Git signing configuration. None of the reference repositories changed during Phase 0.

## Phase 1 acceptance

The Sol Medium platform worker built the pinned native platform, Go CLI, and TypeScript SDK/CLI, then verified fresh persistent-state startup and fixture provisioning. The orchestrator reviewed the scripts and configuration and independently reran `make platform-ready` and `make interop-smoke`; both exited 0.

All six cross-reference byte comparisons passed: Go -> TypeScript and TypeScript -> Go for 35-byte text, 4,110-byte binary, and empty input. Both consumers rejected a real denied policy without plaintext. A protected policy RPC returned HTTP 401 without authentication. OAuth issuer/audience checks and RSA public-key discovery passed. The running profile uses Bearer authentication and RSA-2048 key `r1`; enforced DPoP, EC, and browser operation remain later requirements.

The setup preserves other Docker projects and tracked reference sources. Reviewed adaptations are workspace-local Keycloak ownership initialization, replacement of external fixture KAS routing while preserving authorization mappings, and refresh of only the locally rebuilt SDK tarball identity in an isolated CLI copy. See [platform instructions](platform.md) and [detailed results](phase1-results.md).

## Phase 2 acceptance

The Sol High native capability worker implemented crypto, canonical encoding, bounded HTTP, and real Unix wall time. Twenty-six declaration-backed contracts validate against native signatures. Twenty-five generated Go mappings bundle standard-library native code; generated HTTP reports its Phase 4 dependency. All six non-Go targets currently report unavailable new capabilities explicitly, including type-only opaque key references.

Goalchemy commit: `7df8103bf0d4985ffaf89235ba204b513deacc34` (`feat(capabilities): add native crypto encoding HTTP and wall clock`). Reference SDK trees remain unchanged. The compiler lock now records this revision and preserves the initial compiler revision as `baseline_revision`.

The orchestrator independently verified native crypto/encoding/HTTP/clock tests under the race detector, catalog validation (13 types, 93 functions, seven targets), all 501 generated specification files, and 179/179 existing Go runtime contract cases. Cooperative source checks accepted primitive, HTTP, and type-only key probes. The expanded generated Go primitive probe ran successfully. Missing TypeScript capabilities and emitted Go HTTP returned the expected `GCE002` diagnostics. Driver regressions cover all six non-Go targets.

Independent live native capability testing obtained a real Bearer token, imported RSA key `r1`, wrapped a fresh 32-byte share, formed the exact HMAC policy binding and signed request, called Connect Rewrap, validated response IDs/status, and recovered identical bytes through an independent session key. This is native primitive evidence, not shared TDF interoperability or enforced-DPoP verification.

Review corrected AES ciphertext bounds, strict single-block PEM parsing, portable GET body rules, key snapshot lifetime documentation, and schema enforcement requiring native-test references for empty deterministic case lists. Package tests cover published vectors, cross-standard-library signing/wrapping/key formats, rejection, HTTP limits/cancellation/deadlines and TLS verification. See [capabilities](capabilities.md) for commands and contracts.

Both Go 1.25.14 and the pinned loader Go 1.27.1 are available. An intermediate loader override excluded the existing Go 1.26 diagnostic fixture; that override was removed. The orchestrator reran the failed fixture on the unchanged default loader and it passed. The final worker regression suite also passed with the default loader.

Remaining boundaries: generated HTTP host scheduling is required in Phase 4; shared code must use pointer key handles without copying their dereferenced values; non-Go implementations and all generated SDK libraries remain required. Phase 3 can use ordinary native Go capabilities immediately.
