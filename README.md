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
materializes the complete plaintext. All eight SDKs were measured again using
fresh packages with native CRC32. The 10 MiB files use five 2 MiB segments;
these results are not sums of the earlier encryption/decryption timings.

Original Go reuses a client initialized before the timing loop. Configuration,
token providers and encryption options are prepared outside timing for every
SDK. Generated APIs are stateless facades, so their internal per-operation
initialization and decryption session-key generation remain timed. File I/O,
OAuth acquisition, public-key discovery and correctness checks are untimed.

| SDK | 10 KiB | 100 KiB | 1 MiB | 10 MiB |
| --- | ---: | ---: | ---: | ---: |
| Original OpenTDF Go | 29.27 ms | 21.69 ms | 47.97 ms | 100.33 ms |
| Generated Go | 74.32 ms | 104.07 ms | 59.86 ms | 83.57 ms |
| TypeScript (Node) | 92.13 ms | 102.60 ms | 116.57 ms | 224.53 ms |
| Java | 129.48 ms | 113.06 ms | 204.55 ms | 392.39 ms |
| C# | 179.21 ms | 134.51 ms | 161.10 ms | 258.92 ms |
| Python | 147.00 ms | 133.20 ms | 159.67 ms | 231.37 ms |
| Rust | 68.61 ms | 76.47 ms | 85.91 ms | 136.43 ms |
| C | 144.61 ms | 172.36 ms | 193.15 ms | 199.56 ms |

All 192 warmup/measured archives passed independent original-Go decryption
through KAS, and every end-to-end plaintext matched its input exactly.
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
