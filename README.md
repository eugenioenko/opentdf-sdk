# OpenTDF SDK with Goalchemy

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
