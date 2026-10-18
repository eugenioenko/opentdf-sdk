---
title: Build and install
description: Choose a native package and build it from the pinned shared source.
---

Packages are built from this repository and are not published to registries yet.
Use the package names and installation steps in your language's guide.

| Language | Package | Guide |
| --- | --- | --- |
| Go | Importable Go module | [Go](../languages/go/) |
| TypeScript | ESM for Node and browsers | [TypeScript](../languages/typescript/) |
| Python | Installable wheel | [Python](../languages/python/) |
| Java | JAR | [Java](../languages/java/) |
| C# | .NET class library | [C#](../languages/csharp/) |
| Rust | Cargo crate | [Rust](../languages/rust/) |
| C | Headers, static library, and source | [C](../languages/c/) |
| Swift | SwiftPM source package | [Swift](../languages/swift/) |

## Build from source

Use the compiler revision in `references.lock.json` and the language's build
helper under `scripts/`. The [build reference](../reference/builds/) lists the
checkout layout and prerequisites. Change shared source or host adapters, then
regenerate packages instead of editing generated code.

## Connect to a platform

You need a reachable OpenTDF platform, an authorized identity, and the KAS
endpoints referenced by your files. Use an existing deployment or follow the
[local platform setup](../reference/platform/).

Continue with [encrypting and decrypting](../guides/encrypt-and-decrypt/).
