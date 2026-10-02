# OpenTDF SDK with Goalchemy

This project will implement OpenTDF SDK behavior in shared Go source accepted by Goalchemy and generate libraries for Go, TypeScript, Java, C#, Python, Rust, and C. TypeScript support includes Node and browsers.

The goal is to ship interoperable TDF3 encryption/decryption SDKs across all seven targets, verified against OpenTDF and real KAS. TypeScript includes Node and browsers. The [delivery checklist](docs/delivery-checklist.md) defines completion; broader Go SDK feature parity is outside this goal.

Phases 0–5 are accepted for the shared implementation and compiler foundation. The native Go SDK has verified shared [formats](docs/format.md), an [encryption/integrity engine](docs/crypto-engine.md), and a [client](docs/client.md) with [RSA/P256 wrapping and response sessions](docs/ec-client.md), Bearer authentication and [enforced DPoP/nonce handling](docs/dpop-client.md). Independent live tests passed both directions against the pinned Go and TypeScript reference formats, with stock Web DPoP authentication limitations recorded separately. The [importable generated Go SDK](docs/generated-go-library.md) has also passed real KAS RSA/P256, Bearer/enforced-DPoP and negative checks against both references. The [generated TypeScript SDK](docs/generated-typescript-library.md) is accepted for Node and actual Chromium, including WebCrypto/fetch, imported ESM packages, RSA/P256, Bearer/enforced DPoP, ownership, cancellation and rejection checks. Java, C#, Python, Rust and C SDK delivery remains pending. The [implementation plan](docs/plan.md) and [progress log](docs/progress.md) record the remaining work; [references.lock.json](references.lock.json) pins the reference revisions.

From this directory, run `make platform-up`, `make platform-ready`, and `make interop-smoke`. See [platform setup](docs/platform.md) for prerequisites, configuration, and lifecycle commands.

The native Go module is `opentdf-local/sdk`. Its current byte API exposes `New`, `Create`, `Decrypt`, `PublicKey`, and `Close`; see [client configuration and limitations](docs/client.md). Run `GOTOOLCHAIN=go1.25.14 go test -race ./...` for native checks (Node is needed for the independent URL oracle). Phase 4 has verified generated Go executable HTTP and an initial real-KAS RSA/Bearer interoperability probe against both reference CLIs, plus [TypeScript host scheduling](../goalchemy/docs/typescript-host-operations.md) in Node and actual Chromium, the [Java host lifecycle](../goalchemy/docs/java-host-operations.md) on the JVM, [C# host scheduling](../goalchemy/docs/csharp-host-operations.md) on .NET, the [Python host lifecycle](../goalchemy/docs/python-host-operations.md) on CPython, the [Rust host lifecycle](../goalchemy/docs/rust-host-operations.md) with native and emitted-program checks, and the [C host lifecycle](../goalchemy/docs/c-host-operations.md) with collector, sanitizer and emitted-program checks. All seven byte backends and generic host prerequisites are accepted. The generated Go library owns per-call initialization, copied inputs/results, cancellation, keys and scoped native token providers. TypeScript adds portable WebCrypto/fetch adapters and an importable ESM SDK for Node and browsers. Production adapters and SDK libraries for Java, C#, Python, Rust and C follow in Phase 6.

The [root API index](docs/reference-api.json) and [public subpackage index](docs/reference-subpackages.json) preserve the pinned Go API for reference and possible future work. Their broader service and helper APIs are outside the current delivery scope.

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
