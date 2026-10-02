# OpenTDF SDK implementation progress

## Current status

Phase 0 is accepted: the workspace structure, reference revisions, implementation plan, protocol audit, and compatibility inventory are recorded and reviewed. Git is initialized in `sdk/` on branch `main`. SDK and compiler implementation has not started. Phase 1 is next.

All seven targets are required. Future workers use GPT-6.1 Sol High for complex work and Medium for routine work, one at a time. The first audit worker inherited the orchestrator's settings. The orchestrator will commit after each verified phase, including a separate Goalchemy commit when that repository changes.

The active harness goal requires full feature parity with the pinned OpenTDF Go SDK on all seven targets and passing real-platform interop tests. The TDF3/KAS first release is an intermediate milestone, not a reduced definition of completion.

## Verification

The initial inspection found clean working trees in Goalchemy, platform, and web SDK. Go 1.25.1, Node 24.15.0, Docker 29.2.1, and Docker Compose 5.0.2 are installed. These are discovery results, not evidence that the clones build with those toolchains.

The existing platform Compose and official quickstart were read. No OpenTDF containers have been started, no system configuration has been changed, and no interop test has run.

## Next task

Bring up an isolated basic local platform from the pinned clone and verify health, token acquisition, KAS discovery, and Go/TypeScript reference interoperability. Use one Sol Medium worker; the orchestrator reviews the results before the Phase 1 commit.

## Phase 0 acceptance

The worker wrote `tdf-poc.md` and `compatibility.md` from both pinned SDKs, platform KAS/auth/protobuf code, and Goalchemy source. The orchestrator reviewed the documents and independently verified 169 local links, document whitespace, and all three reference revisions. Review corrected the opt-in Go assertion option to `WithSystemMetadataAssertion` and replaced imprecise line anchors with links to the authoritative files.

Direct source checks confirmed RSA-OAEP SHA-1, the EC HKDF parameters, Connect procedure names, distinct authentication/session keys, and the web DPoP interceptor's differences. No live test is claimed. Runtime KAS algorithms, DPoP enforcement, empty payload behavior, routing, and CORS remain Phase 1 or subsequent integration evidence.

The first phase commit contains the plan and source audit; its hash will be recorded with the next phase's results. None of the reference repositories changed during Phase 0.
