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
helper under `scripts/`. For example, build the Go package in adjacent checkouts:

```sh
git clone https://github.com/eugenioenko/opentdf-sdk.git sdk
git clone https://github.com/eugenioenko/goalchemy.git goalchemy
cd sdk
python3 - <<'PY'
import json, subprocess
revision = json.load(open('references.lock.json'))['repositories']['goalchemy']['revision']
subprocess.run(['git', '-C', '../goalchemy', 'checkout', revision], check=True)
PY
(cd ../goalchemy && go build -o bin/goalchemy ./cmd/goalchemy)
GOALCHEMY_BIN="$PWD/../goalchemy/bin/goalchemy" \
  bash scripts/build-generated-go.sh "$PWD/out/go"
```

Install Go with automatic toolchain selection enabled so the compiler can use
the toolchain declared in its module. Other language helpers need their native
tools and dependencies, listed in each language guide. Build once, then install
the resulting package in your application.

Change shared source or host adapters and regenerate packages instead of editing
generated code.

## Connect to a platform

You need a reachable OpenTDF platform, an authorized identity, and the KAS
endpoints referenced by your files. Your platform administrator supplies the
endpoint, authentication settings, and policy attributes.

The [local test platform](../contributing/platform/) is an optional contributor
setup for running this repository's tests.

Continue with [encrypting and decrypting](../guides/encrypt-and-decrypt/).
