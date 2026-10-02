# OpenTDF SDK with Goalchemy

This project will implement OpenTDF SDK behavior in shared Go source accepted by Goalchemy and generate libraries for Go, TypeScript, Java, C#, Python, Rust, and C. TypeScript support includes Node and browsers.

The first release will create and decrypt TDF3 files through a real OpenTDF Key Access Server (KAS). The final goal is full feature parity with the pinned OpenTDF Go SDK on all seven targets; the compatibility inventory tracks required work beyond this first release.

Phases 0, 1, and 2 are complete: the [reference audit](docs/tdf-poc.md), [compatibility inventory](docs/compatibility.md), and [local platform with passing reference interoperability](docs/phase1-results.md) are verified. [Native capability contracts and Go implementations](docs/capabilities.md) are also complete. Shared TDF implementation begins in Phase 3 of the [implementation plan](docs/plan.md). The checked-out reference revisions are recorded in [references.lock.json](references.lock.json).

From this directory, run `make platform-up`, `make platform-ready`, and `make interop-smoke`. See [platform setup](docs/platform.md) for prerequisites, configuration, and lifecycle commands.

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
