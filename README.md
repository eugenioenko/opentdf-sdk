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

## Performance

Median of **five timed runs**, following one warmup, on a local workstation.
Columns are plaintext sizes. Each cell shows **milliseconds (duration ratio)**;
the original OpenTDF Go SDK is **1.00×** for each size and operation, and lower
ratios are faster.

These measurements include a fresh public SDK operation and complete in-memory
output; decryption includes **real KAS rewrap**. The original Go lifecycle also
includes client construction, which generates an RSA session key even for
encryption. File I/O, OAuth acquisition and correctness validation are untimed.
All 144 encryption outputs were decrypted and verified through original Go and
real KAS. See [methodology](docs/benchmarks.md) and [samples and runtime versions](docs/benchmark-results.json).

### Encryption

| SDK | 10 KiB | 100 KiB | 1 MiB |
| --- | ---: | ---: | ---: |
| Original OpenTDF Go | 53.49 ms (1.00×) | 62.68 ms (1.00×) | 34.65 ms (1.00×) |
| Generated Go | 0.28 ms (0.01×) | 0.75 ms (0.01×) | 4.68 ms (0.13×) |
| TypeScript (Node) | 21.31 ms (0.40×) | 30.99 ms (0.49×) | 100.69 ms (2.91×) |
| Java | 10.63 ms (0.20×) | 16.40 ms (0.26×) | 34.47 ms (0.99×) |
| C# | 6.86 ms (0.13×) | 9.00 ms (0.14×) | 17.45 ms (0.50×) |
| Python | 32.82 ms (0.61×) | 167.10 ms (2.67×) | 1249.27 ms (36.05×) |
| Rust | 4.47 ms (0.08×) | 19.57 ms (0.31×) | 149.32 ms (4.31×) |
| C | 5.64 ms (0.11×) | 13.64 ms (0.22×) | 72.26 ms (2.09×) |

### Decryption

| SDK | 10 KiB | 100 KiB | 1 MiB |
| --- | ---: | ---: | ---: |
| Original OpenTDF Go | 100.59 ms (1.00×) | 91.60 ms (1.00×) | 214.73 ms (1.00×) |
| Generated Go | 66.12 ms (0.66×) | 76.26 ms (0.83×) | 83.24 ms (0.39×) |
| TypeScript (Node) | 92.06 ms (0.92×) | 99.86 ms (1.09×) | 260.47 ms (1.21×) |
| Java | 133.05 ms (1.32×) | 140.52 ms (1.53×) | 119.09 ms (0.55×) |
| C# | 171.51 ms (1.71×) | 250.17 ms (2.73×) | 200.61 ms (0.93×) |
| Python | 169.43 ms (1.68×) | 455.23 ms (4.97×) | 2472.00 ms (11.51×) |
| Rust | 81.69 ms (0.81×) | 112.34 ms (1.23×) | 350.54 ms (1.63×) |
| C | 137.86 ms (1.37×) | 228.82 ms (2.50×) | 273.05 ms (1.27×) |

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
