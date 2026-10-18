# OpenTDF SDK with Goalchemy

**Proof of concept (POC). Work in progress. The implementation may change.**

One shared Go implementation provides TDF3 encryption/decryption libraries for
**Go, TypeScript, Java, C#, Python, Rust, C and Swift**. TypeScript supports Node and
browsers. Swift is experimental, with a Linux baseline. The shared source,
native adapters and build helpers live
in this repository; generated packages are ignored build outputs.

The original seven libraries have passed interoperability checks against the pinned
OpenTDF Go and Web SDKs through real KAS. The supported byte API includes
RSA-2048/P-256 wrapping and response sessions, RS256/ES256 signing, Bearer and
enforced DPoP, encrypted metadata, owned results and cancellation. See
[verified delivery and build instructions](docs/final-delivery.md) and
[profile limits](docs/compatibility.md). Full OpenTDF API parity is outside scope.

Swift also passed a fresh full local real-KAS matrix: 541 comparisons across
BASIC, EC and enforced DPoP, plus integrity and transport rejection checks.
Its SwiftPM source package built reproducibly and was imported independently.
macOS/iOS integration remains unverified. Swift is omitted from the current
performance campaign.

Browse the [SDK documentation](https://eugenioenko.github.io/opentdf-sdk/) for
getting-started guides, [authentication examples](https://eugenioenko.github.io/opentdf-sdk/languages/authentication/),
the [SDK API Reference](https://eugenioenko.github.io/opentdf-sdk/api/), and the
[RPC API Reference](https://eugenioenko.github.io/opentdf-sdk/api/rpc/).
The [documentation maintenance guide](docs-web/README.md) explains generation
and automatic GitHub Pages publishing.

| SDK | Package | API and prerequisites |
| --- | --- | --- |
| Go | Importable Go module | [Go](docs/generated-go-library.md) |
| TypeScript | Portable ESM and declarations for Node/browser | [TypeScript](docs/generated-typescript-library.md) |
| Java | JAR with locked provider | [Java](docs/generated-java-library.md) |
| C# | .NET 8 class library | [C#](docs/generated-csharp-library.md) |
| Python | Installable wheel | [Python](docs/generated-python-library.md) |
| Rust | Locked Cargo crate | [Rust](docs/generated-rust-library.md) |
| C | C17 headers, static library and source archive | [C](docs/generated-c-library.md) |
| Swift | SwiftPM source package with Foundation Data API | [Swift](docs/generated-swift-library.md) |

## End-to-end performance

Median encrypt → decrypt latency through **real local KAS**, in milliseconds,
measured on 2026-10-18 after the bulk buffer wiping fix at SDK `948cf50`.
The generated SDKs use Goalchemy `468ec24`, with native pausing except C
(frames). Both upstream Go and Web SDK references were remeasured in the same
campaign.
Swift is omitted from these timings.

| SDK | 1 KiB | 10 KiB | 100 KiB | 1 MiB | 10 MiB |
| --- | ---: | ---: | ---: | ---: | ---: |
| Original OpenTDF Go | 15.26 ms | 14.81 ms | 20.40 ms | 43.25 ms | 109.29 ms |
| Original OpenTDF Web (Node) | 55.24 ms | 70.33 ms | 47.93 ms | 61.88 ms | 135.38 ms |
| Generated Go | 16.04 ms | 15.44 ms | 19.77 ms | 18.20 ms | 39.43 ms |
| TypeScript (Node) | 38.26 ms | 37.38 ms | 39.43 ms | 45.69 ms | 134.35 ms |
| Java | 24.17 ms | 25.45 ms | 26.57 ms | 29.63 ms | 74.62 ms |
| C# | 21.37 ms | 21.76 ms | 23.70 ms | 37.96 ms | 105.08 ms |
| Python | 52.85 ms | 56.09 ms | 59.04 ms | 62.94 ms | 167.77 ms |
| Rust | 22.99 ms | 24.90 ms | 24.61 ms | 28.69 ms | 73.66 ms |
| C | 22.74 ms | 24.11 ms | 22.05 ms | 30.13 ms | 83.06 ms |

Each cell pools fifteen timed pairs across three fresh processes after
forty 10 MiB warmup pairs and twenty size-specific warmups per process.
SDK handles are initialized once and reused. The upstream Web API generates
a fresh RSA response key inside each decrypt; the other SDKs reuse theirs.
File I/O, OAuth, discovery, and correctness checks are untimed. All retained
archives pass independent Go/KAS decryption, plaintext, ZIP CRC, and segment
checks. Sizes use 1,024-based units. See the [methodology and source revisions](docs/benchmarks.md)
and [timing evidence](docs/benchmark-results/2026-10-18.json).

## Build and run

The shared Go implementation and its unit tests live in `src/`, including
`src/tdf/` and `src/tdf/json/`. The module identity remains
`opentdf-local/sdk`; shared-source imports are now `opentdf-local/sdk/src`,
`opentdf-local/sdk/src/tdf` and `opentdf-local/sdk/src/tdf/json`. Direct Go
consumers of the shared source must update these imports. Every build helper
compiles the export root `./src`. Native adapters, dependency locks and delivery tooling live in `src/hosts/`.
Build scripts and top-level `tests/` remain at root. Generated native SDK public
APIs retain their existing package names and signatures.

The build configuration pins released
[Goalchemy v0.7.2](https://github.com/eugenioenko/goalchemy/releases/tag/v0.7.2)
for package-based output across all eight targets. The exact release commit is
recorded in [references.lock.json](references.lock.json).
Build helpers explicitly select `--pause-model=native` for Go, TypeScript,
Python, Java, C#, Rust and Swift; C uses `--pause-model=frames`. Native pausing
uses each target's async/await or blocking calls under Goalchemy's scheduler.
Generated identifiers use source names by default.
The performance table identifies the actual compiler commits used for its
measurements.

Use adjacent `sdk`, `goalchemy`, `platform` and `web-sdk` checkouts at the revisions
in [references.lock.json](references.lock.json). Build the pinned Goalchemy binary
and run the corresponding `scripts/build-generated-<target>.sh` helper; both
relative and absolute output/compiler paths are supported. Exact toolchains,
installation commands and native dependencies are documented in
[final delivery](docs/final-delivery.md).

From this directory, run `make platform-up`, `make platform-ready` and
`make interop-smoke` for the authenticated BASIC Docker setup. See
[platform setup](docs/platform.md) and [EC/DPoP profiles](docs/secure-profiles.md).

## Verification

[GitHub Actions](.github/workflows/tdf3-delivery.yml) builds all eight native SDKs
and runs focused real-KAS interoperability against the pinned OpenTDF Go and Web
SDKs. TypeScript includes Node and actual Chromium coverage; EC, DPoP, metadata
and rejection cases are included. The complete matrices remain available as a
manual option.

Package-output checks also cover independent native imports, initialization,
shared type identity, retained results, overlapping calls and cancellation.
See [evidence and limitations](docs/final-delivery.md) and the
[progress log](docs/progress.md) for the scope and revisions of completed runs.
Native build products, local credentials, keys and test outputs stay out of Git.

The [delivery checklist](docs/delivery-checklist.md) and
[progress log](docs/progress.md) record acceptance. The
[reference API index](docs/reference-api.json) preserves broader APIs for future
work. Package names remain development identities; package-registry publication
and the repository's top-level license decision are separate release work.
