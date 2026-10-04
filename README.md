# OpenTDF SDK with Goalchemy

**Proof of concept (POC). Work in progress. The implementation may change.**

One shared Go implementation provides TDF3 encryption/decryption libraries for
**Go, TypeScript, Java, C#, Python, Rust and C**. TypeScript supports Node and
browsers. The shared source, native adapters and all seven build helpers live
in this repository. Generated source distributions are committed under `dist/`;
compiled packages remain ignored build outputs.

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

Median encrypt → decrypt time through **real KAS**, in milliseconds.
Generated SDKs were built with Goalchemy **v0.2.1**.\*

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

\* Each cell pools 15 measurements: five per process across three fresh
processes, with normal runtime settings. Before timing, each process runs
40 warmup pairs at 50 MiB and 20 at the measured size; generated Go 1 MiB,
Java 1 MiB and C# 10 MiB use 100 size-specific warmups after checking their initial
histories. Every pair checks the full plaintext; all 486 retained archives
also passed independent OpenTDF Go/KAS decryption and ZIP CRC checks.

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
Compiled packages, local credentials, keys and test outputs stay out of Git.

The [delivery checklist](docs/delivery-checklist.md) and
[progress log](docs/progress.md) record acceptance. The
[reference API index](docs/reference-api.json) preserves broader APIs for future
work. Package names remain development identities; package-registry publication
and the repository's top-level license decision are separate release work.

Shared implementation lives under `src/`. The seven committed source distributions
under [`dist/`](dist/) contain generated code, native adapters, dependency manifests
and notices; each target README describes building and importing it without a
Goalchemy executable. Compiled packages, caches and downloaded dependencies stay
ignored. Current distributions use Goalchemy **v0.4.0** with its default readable
source-derived identifiers; the benchmark table above remains the historical
v0.2.1 measurement.

To regenerate and format every distribution, check out the compiler revision in
`references.lock.json`, bootstrap the pinned prerequisites with
`python3 scripts/delivery-bootstrap.py all --base .local/dist-bootstrap`, then run:

```sh
python3 scripts/dist-distributions.py regenerate --base .local/dist-regeneration
```

Use a fresh ignored `--base` for each run. `check` regenerates into ignored storage
and fails on any committed drift, then formats twice to verify idempotence.
Formatter versions are pinned in `scripts/dist-formatters.json`, with explicit
configuration beside it. Pre-format diagnostic line maps are omitted.
Existing CI retains its ephemeral generation/build/KAS checks. Distribution build
and CI coverage are tracked in [issue #7](https://github.com/eugenioenko/opentdf-sdk/issues/7);
no additional CI jobs are introduced here. Native source build commands are in each
distribution README. A root Make entrypoint is tracked in
[issue #8](https://github.com/eugenioenko/opentdf-sdk/issues/8).
