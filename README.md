# OpenTDF SDK with Goalchemy

This project will implement OpenTDF SDK behavior in shared Go source accepted by Goalchemy and generate libraries for Go, TypeScript, Java, C#, Python, Rust, and C. TypeScript support includes Node and browsers.

The first release will create and decrypt TDF3 files through a real OpenTDF Key Access Server (KAS). The final goal is full feature parity with the pinned OpenTDF Go SDK on all seven targets; the compatibility inventory tracks required work beyond this first release.

SDK implementation has not started. The [reference audit](docs/tdf-poc.md) and [compatibility inventory](docs/compatibility.md) complete Phase 0 of the [implementation plan](docs/plan.md). Phase 1 establishes a running platform and reference smoke tests. The checked-out reference revisions are recorded in [references.lock.json](references.lock.json).

## Workspace

| Directory | Role |
| --- | --- |
| `../goalchemy/` | Compiler, runtime contracts, native capabilities, and generated library support |
| `../platform/sdk/` | Go SDK reference |
| `../platform/otdfctl/` | Go CLI for setup and reference encryption/decryption |
| `../web-sdk/lib/` | TypeScript SDK reference for Node and browser behavior |
| `../web-sdk/cli/` | Node CLI for reference encryption/decryption |
| `./` | Shared SDK source, development platform setup, interop harness, and generated package configuration |

## Progress

The orchestrator coordinates one worker agent at a time, reviews each result, and records the evidence before advancing. See the phase checklist in the plan for accepted work and outstanding tasks. Generated artifacts, local credentials, keys, and test outputs must stay out of source control.
