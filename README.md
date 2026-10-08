# OpenTDF SDK with Goalchemy

**Proof of concept (POC). Work in progress. The implementation may change.**

One shared Go implementation provides TDF3 encryption/decryption libraries for
**Go, TypeScript, Java, C#, Python, Rust, C and Swift**. TypeScript supports Node and
browsers. Swift is experimental, with a Linux baseline. The shared source,
native adapters and build helpers live in this repository. Formatted generated
source packages live in root `dist/`; native build products stay ignored.

The original seven libraries have passed interoperability checks against the pinned
OpenTDF Go and Web SDKs through real KAS. The supported byte API includes
RSA-2048/P-256 wrapping and response sessions, RS256/ES256 signing, Bearer and
enforced DPoP, encrypted metadata, owned results and cancellation. See
[verified delivery and build instructions](docs/final-delivery.md) and
[profile limits](docs/compatibility.md). Full OpenTDF API parity is outside scope.

Swift also passed a fresh full local real-KAS matrix: 541 comparisons across
BASIC, EC and enforced DPoP, plus integrity and transport rejection checks.
Its SwiftPM source package built reproducibly and was imported independently.
macOS/iOS integration remains unverified. Bounded Linux Swift performance
measurements are reported below.

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

Median encrypt → decrypt time through **real KAS**, in milliseconds.
The original and first seven generated rows are historical Goalchemy
**v0.2.1** results.\* Swift uses separate bounded diagnostic measurements.†

| SDK | 1 MiB | 10 MiB | 50 MiB |
| --- | ---: | ---: | ---: |
| Original OpenTDF Go | 36.28 ms | 88.27 ms | 217.77 ms |
| Original OpenTDF Web (Node) | 216.69 ms | 932.65 ms | 4444.67 ms |
| Generated Go | 72.83 ms | 83.18 ms | 266.32 ms |
| TypeScript (Node) | 96.57 ms | 199.28 ms | 750.11 ms |
| Java | 87.70 ms | 142.18 ms | 401.28 ms |
| C# | 190.50 ms | 211.22 ms | 537.33 ms |
| Python | 141.06 ms | 186.54 ms | 691.28 ms |
| Rust | 61.66 ms | 127.14 ms | 614.88 ms |
| C | 148.24 ms | 254.38 ms | 555.77 ms |
| Swift† | 754.02 ms | 706.78 ms | 1401.92 ms |

\* Each historical cell pools 15 measurements: five per process across three fresh
processes, with normal runtime settings. Before timing, each process runs
40 warmup pairs at 50 MiB and 20 at the measured size; generated Go 1 MiB,
Java 1 MiB and C# 10 MiB use 100 size-specific warmups after checking their initial
histories. Every pair checks the full plaintext; all 486 retained archives
also passed independent OpenTDF Go/KAS decryption and ZIP CRC checks.

† Swift uses a release build on Linux x86_64 with one size-specific warmup and
no bulk warmup. The 1/10 MiB cells each use two measured pairs from compiler
commit `5de8b6b`; 50 MiB uses three from `4d94248f`. These are small diagnostic
samples from one process per size, not a matched repeat of the historical
15-sample campaign. Every pair checked full plaintext; all retained archives
passed independent stock-Go/KAS decryption and ZIP CRC checks, and Swift also
decrypted stock-Go archives at all three sizes. The public Foundation `Data`
input/output conversions and SDK-internal setup remain timed.

Original Go reuses a client initialized before timing; generated facades'
internal setup and session-key generation remain timed. Original Web reuses
its client and ES256 signer; its per-decrypt RSA key generation remains timed.
File I/O, OAuth, public-key discovery and correctness checks are untimed. Inputs use 2 MiB
segments. Browser execution is excluded. See the
[methodology](docs/benchmarks.md).

## Build and run

The shared Go implementation and its unit tests live in `src/`, including
`src/tdf/` and `src/tdf/json/`. The module identity remains
`opentdf-local/sdk`; shared-source imports are now `opentdf-local/sdk/src`,
`opentdf-local/sdk/src/tdf` and `opentdf-local/sdk/src/tdf/json`. Direct Go
consumers of the shared source must update these imports. The export façade
and its unit test also live in `src/library/`, imported as
`opentdf-local/sdk/src/library`; every build helper compiles `./src/library`.
Native adapters, dependency locks and delivery tooling live in `src/hosts/`.
Build scripts and top-level `tests/` remain at root. Generated native SDK public
APIs retain their existing package names and signatures.

The build configuration pins unreleased Goalchemy commit
[`7863d12`](https://github.com/eugenioenko/goalchemy/commit/7863d12abd27b2e2e1d655d5ca64295842485d82)
for package-based output across all eight targets, based on v0.5.1.
Readable source-derived identifiers remain the default.
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

## Generated source distributions

`dist/` contains buildable source for all eight targets, grouped by the original
Go packages with shared runtime and initialization files. Public package names
and APIs remain the same. Each target's `README.md` explains its native build;
using a checked-in source distribution does not require the Goalchemy compiler.

Prepare the pinned native toolchains and dependencies using
[the delivery setup](docs/final-delivery.md). To regenerate from a clean
Goalchemy checkout at the compiler revision in `references.lock.json`, use a
fresh ignored work directory:

```sh
python3 scripts/dist-distributions.py regenerate --base .local/dist-work/refresh-1
```

The command reuses the delivery builders, formats the source with pinned tools,
and checks formatter idempotence. Pass `--environment` with the delivery
bootstrap's `environment.json` when using its prepared dependency paths.
Dependency notices and native build metadata
are included. Binaries, caches and diagnostic maps for pre-format line positions
are excluded. Existing CI continues to build fresh packages and check real-KAS
interoperability; it does not verify the checked-in distributions.

The [delivery checklist](docs/delivery-checklist.md) and
[progress log](docs/progress.md) record acceptance. The
[reference API index](docs/reference-api.json) preserves broader APIs for future
work. Package names remain development identities; package-registry publication
and the repository's top-level license decision are separate release work.
