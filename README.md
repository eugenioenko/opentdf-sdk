# OpenTDF SDK with Goalchemy

This project will implement OpenTDF SDK behavior in shared Go source accepted by Goalchemy and generate libraries for Go, TypeScript, Java, C#, Python, Rust, and C. TypeScript support includes Node and browsers.

The first release will create and decrypt TDF3 files through a real OpenTDF Key Access Server (KAS). The final goal is full feature parity with the pinned OpenTDF Go SDK on all seven targets; the compatibility inventory tracks required work beyond this first release.

Phases 0–3 are accepted. The native Go SDK has verified shared [formats](docs/format.md), an [encryption/integrity engine](docs/crypto-engine.md), and a [client](docs/client.md) with [RSA/P256 wrapping and response sessions](docs/ec-client.md), Bearer authentication and [enforced DPoP/nonce handling](docs/dpop-client.md). Independent live tests passed both directions against the pinned Go and TypeScript reference formats, with stock Web DPoP authentication limitations recorded separately. Generated libraries on all seven targets, actual browser execution and full Go SDK parity remain pending. The [implementation plan](docs/plan.md) and [progress log](docs/progress.md) record the remaining work; [references.lock.json](references.lock.json) pins the reference revisions.

From this directory, run `make platform-up`, `make platform-ready`, and `make interop-smoke`. See [platform setup](docs/platform.md) for prerequisites, configuration, and lifecycle commands.

The native Go module is `opentdf-local/sdk`. Its current byte API exposes `New`, `Create`, `Decrypt`, `PublicKey`, and `Close`; see [client configuration and limitations](docs/client.md). Run `GOTOOLCHAIN=go1.25.14 go test -race ./...` for native checks (Node is needed for the independent URL oracle). Generated HTTP is still a compiler dependency, so this verification does not establish generated SDK support on any target.

The [root API index](docs/reference-api.json) and [public subpackage index](docs/reference-subpackages.json) preserve the pinned Go API for full-parity review. They identify required scope; they do not prove implementation coverage.

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
