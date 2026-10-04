# OpenTDF SDK with Goalchemy

**Proof of concept (POC). Work in progress. The implementation may change.**

One shared Go implementation provides TDF3 encryption/decryption libraries for
**Go, TypeScript, Java, C#, Python, Rust and C**. TypeScript supports Node and
browsers. The shared source, native adapters and all seven build helpers live
in this repository; generated packages are ignored build outputs.

All seven libraries have passed interoperability checks against the pinned
OpenTDF Go and Web SDKs through real KAS. The supported byte API includes
RSA-2048/P-256 wrapping and response sessions, RS256/ES256 signing, Bearer and
enforced DPoP, encrypted metadata, owned results and cancellation. See
[verified delivery and build instructions](docs/final-delivery.md) and
[profile limits](docs/compatibility.md). Full OpenTDF API parity is outside scope.

| SDK | Package | API and prerequisites |
| --- | --- | --- |
| Go | Importable Go module | [Go](docs/generated-go-library.md) |
| TypeScript | Portable ESM and declarations for Node/browser | [TypeScript](docs/generated-typescript-library.md) |
| Java | JAR with locked provider | [Java](docs/generated-java-library.md) |
| C# | .NET 8 class library | [C#](docs/generated-csharp-library.md) |
| Python | Installable wheel | [Python](docs/generated-python-library.md) |
| Rust | Locked Cargo crate | [Rust](docs/generated-rust-library.md) |
| C | C17 headers, static library and source archive | [C](docs/generated-c-library.md) |

## End-to-end performance

Median **milliseconds** from five timed runs after one warmup. Each run encrypts
the input, then decrypts the freshly produced archive through **real KAS** and
materializes the complete plaintext. These are fresh measurements of the original
Go SDK and all seven generated native SDKs. The generated packages were built with
released Goalchemy **v0.2.0**.
The current build pins **v0.2.1**; the subsequent
[buffer optimization comparison](docs/all-target-buffer-benchmarks.md) records
fresh before/after measurements for the changes it includes.
The 1, 10 and 50 MiB inputs use one, five and 25 segments of up to 2 MiB.

Original Go reuses a client initialized before the timing loop. Configuration,
token providers and encryption options are prepared outside timing for every
SDK. Generated APIs are stateless facades, so their internal per-operation
initialization and decryption session-key generation remain timed. File I/O,
OAuth acquisition, public-key discovery and correctness checks are untimed.

| SDK | 1 MiB | 10 MiB | 50 MiB |
| --- | ---: | ---: | ---: |
| Original OpenTDF Go | 54.60 ms | 103.71 ms | 224.47 ms |
| Generated Go | 86.00 ms | 119.58 ms | 221.57 ms |
| TypeScript (Node) | 123.43 ms | 228.59 ms | 820.11 ms |
| Java | 178.42 ms | 436.74 ms | 1615.78 ms |
| C# | 181.29 ms | 281.72 ms | 651.87 ms |
| Python | 165.12 ms | 245.59 ms | 1166.82 ms |
| Rust | 80.87 ms | 126.55 ms | 668.85 ms |
| C | 166.93 ms | 300.19 ms | 572.36 ms |

All 144 warmup/measured archives passed independent original-Go decryption
through KAS and ZIP CRC verification, and every end-to-end plaintext matched its
input exactly. No earlier timing cells are reused. Browser execution is excluded.
See [methodology and lifecycle details](docs/benchmarks.md) and
[individual samples and runtime versions](docs/benchmark-results.json).

## Build and run

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

Each package was built reproducibly and imported by an independent native
consumer. Final local focused real-KAS checks cover all seven targets plus
actual Chromium, with additional EC/DPoP metadata cases and a readable browser
nonce challenge. Existing full interoperability and rejection matrices were
preserved and matched to final production sources. The only generated Go source
difference is diagnostic line comments. See [evidence and limitations](docs/final-delivery.md).

[GitHub Actions](.github/workflows/tdf3-delivery.yml) defines focused checks for
pull requests and manual runs, with full matrices as a manual option. Remote CI
and a repeated full matrix are not claimed as executed final-delivery results.
Generated packages, local credentials, keys and test outputs stay out of Git.

The [delivery checklist](docs/delivery-checklist.md) and
[progress log](docs/progress.md) record acceptance. The
[reference API index](docs/reference-api.json) preserves broader APIs for future
work. Package names remain development identities; package-registry publication
and the repository's top-level license decision are separate release work.
