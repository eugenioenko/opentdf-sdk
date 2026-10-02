# OpenTDF SDK implementation progress

## Current status

Phases 0 and 1 are accepted. Phase 0 is committed as `0e45fba`; Phase 1 adds the running platform and verified reference interoperability. Git is initialized in `sdk/` on branch `main`. SDK and compiler implementation begins with Phase 2 capability contracts and native Go wrappers.
All seven targets are required. Future workers use GPT-6.1 Sol High for complex work and Medium for routine work, one at a time. The first audit worker inherited the orchestrator's settings. The orchestrator will commit after each verified phase, including a separate Goalchemy commit when that repository changes.

The active harness goal requires full feature parity with the pinned OpenTDF Go SDK on all seven targets and passing real-platform interop tests. The TDF3/KAS first release is an intermediate milestone, not a reduced definition of completion.

## Verification

The initial inspection found clean working trees in Goalchemy, platform, and web SDK. Go 1.25.1, Node 24.15.0, Docker 29.2.1, and Docker Compose 5.0.2 are installed. These are discovery results, not evidence that the clones build with those toolchains.

During Phase 0, the existing platform Compose and official quickstart were read without starting services. Phase 1 subsequently started the isolated `tdf-sdk` stack and verified real reference interoperability; no global host, certificate, or CLI profile configuration was changed.

## Next task

Define capability contracts and native Go implementations for encoding, crypto, wall-clock time, and bounded HTTP. Use one Sol High worker at a time; review primitive checks, source acceptance, and useful unsupported-target diagnostics before accepting Phase 2.
## Phase 0 acceptance

The worker wrote `tdf-poc.md` and `compatibility.md` from both pinned SDKs, platform KAS/auth/protobuf code, and Goalchemy source. The orchestrator reviewed the documents and independently verified 169 local links, document whitespace, and all three reference revisions. Review corrected the opt-in Go assertion option to `WithSystemMetadataAssertion` and replaced imprecise line anchors with links to the authoritative files.

Direct source checks confirmed RSA-OAEP SHA-1, the EC HKDF parameters, Connect procedure names, distinct authentication/session keys, and the web DPoP interceptor's differences. No live test is claimed. Runtime KAS algorithms, DPoP enforcement, empty payload behavior, routing, and CORS remain Phase 1 or subsequent integration evidence.

The first phase commit is `0e45fba` (`docs(sdk): complete phase 0 reference audit and implementation plan`). It contains an SSH signature using the existing Git signing configuration. None of the reference repositories changed during Phase 0.

## Phase 1 acceptance

The Sol Medium platform worker built the pinned native platform, Go CLI, and TypeScript SDK/CLI, then verified fresh persistent-state startup and fixture provisioning. The orchestrator reviewed the scripts and configuration and independently reran `make platform-ready` and `make interop-smoke`; both exited 0.

All six cross-reference byte comparisons passed: Go -> TypeScript and TypeScript -> Go for 35-byte text, 4,110-byte binary, and empty input. Both consumers rejected a real denied policy without plaintext. A protected policy RPC returned HTTP 401 without authentication. OAuth issuer/audience checks and RSA public-key discovery passed. The running profile uses Bearer authentication and RSA-2048 key `r1`; enforced DPoP, EC, and browser operation remain later requirements.

The setup preserves other Docker projects and tracked reference sources. Reviewed adaptations are workspace-local Keycloak ownership initialization, replacement of external fixture KAS routing while preserving authorization mappings, and refresh of only the locally rebuilt SDK tarball identity in an isolated CLI copy. See [platform instructions](platform.md) and [detailed results](phase1-results.md).
