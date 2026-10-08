# Generated Swift TDF3 library

Swift is the eighth native target. The baseline is **Linux x86_64, Swift 6.4.0,
Swift 5 language mode**. macOS/iOS integration is unverified. The package is
experimental, like the rest of this POC, and its API may change.

## Build

Use adjacent `sdk`, `goalchemy`, `platform` and `web-sdk` checkouts. A shared
[Goalchemy **v0.5.1**](https://github.com/eugenioenko/goalchemy/releases/tag/v0.5.1)
is pinned for all eight targets in
[references.lock.json](../references.lock.json).

Ubuntu 22.04 prerequisites:

```sh
sudo apt-get install build-essential pkg-config libssl-dev zlib1g-dev \
    libcurl4-openssl-dev libxml2 libsqlite3-0 libncurses6 libpython3.10
```

From the workspace containing the adjacent repositories:

```sh
cd goalchemy
git checkout v0.5.1
bash scripts/fetch-toolchains.sh swift
export PATH="$PWD/.toolchains/swift-6.4.0/usr/bin:$PATH"
GOTOOLCHAIN=go1.25.14 go build -o out/swift-tdf-library/goalchemy ./cmd/goalchemy
cd ../sdk
GOALCHEMY_BIN=../goalchemy/out/swift-tdf-library/goalchemy \
    bash scripts/build-generated-swift.sh out/swift-tdf-library/sdk
```

Relative and absolute compiler/output paths work from independent caller
directories. `GOALCHEMY_ROOT` can select a separate compiler source snapshot.
The helper creates a SwiftPM source package; production calls require no Go
runtime or reference SDK. OpenSSL 3 provides crypto, libcurl provides HTTP, and
zlib provides native CRC32. SwiftPM discovers system dependencies with
pkg-config; it downloads no remote Swift packages. Native library versions
depend on the host package manager; the Swift compiler archive is SHA-256 pinned.

## Import and use

Add the generated directory as a local SwiftPM dependency:

```swift
// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "Consumer",
    dependencies: [.package(path: "../out/swift-tdf-library/sdk")],
    targets: [
        .executableTarget(name: "Consumer", dependencies: [
            .product(name: "OpenTDFTDF3", package: "sdk")
        ])
    ],
    swiftLanguageModes: [.v5]
)
```

The product/module is `OpenTDFTDF3`. Its wrapper accepts Foundation `Data` for
payloads and returns owned `Data`. Configuration/option structs retain Go field
names and exact integer widths. Text fields use binary-preserving `GoString`:
literals encode UTF-8, dynamic strings use `GoString(text)`, and
`GoString(bytes: ...)` preserves arbitrary bytes. Optional arrays distinguish
nil from explicitly empty source slices.

```swift
import Foundation
import OpenTDFTDF3

var config = Config()
config.PlatformURL = "http://localhost:8080"
config.KASURL = "http://localhost:8080/kas"
config.IssuerURL = "http://localhost:8888/auth/realms/opentdf"
config.ClientID = "opentdf-sdk"
config.ClientSecret = "secret"
config.AllowHTTP = true

var options = EncryptOptions()
options.Attributes = ["https://example.com/attr/attr1/value/value1"]
options.IncludeMetadata = true
options.Metadata = [] // present, explicitly empty metadata

let archive = try encrypt(config, Data([0, 255, 128, 1]), options).wait()
let result = try await decrypt(config, archive).value()
print(result.payload, result.hasMetadata)
```

`Decrypted` owns `payload`, `metadata`, `hasMetadata` and `manifestJSON` bytes.
Calls are lazy: `.wait()` or `.value()` starts execution. Inputs are copied when
the operation is created. Calls serialize through the generated runtime and
results detach from source storage. Caller mutations cannot alter a submitted
call's inputs.

## Authentication, cancellation and failures

The shared implementation handles discovery, client-credentials OAuth,
RSA-2048/P-256 wrapping and response sessions, RS256/ES256 signing and DPoP.
Set `DPoP`, `KASAlgorithm`, `SessionAlgorithm`, `AuthAlgorithm` and any explicit
PEM/KID fields as in the other native packages.

A typed `TokenProvider` receives a `CallbackRequest` and completion callback.
Complete once with an `AccessToken` or an error, including after cancellation;
optionally return a cancellation/cleanup hook. The adapter serializes the token
into the shared callback format. DPoP providers must acquire a token bound to
the configured auth key and supply its `confirmationJKT`. The host supplies its
own OAuth implementation and signer. Do not synchronously wait on another
generated call from a callback or cleanup hook.

`CallOptions` carries a `CancellationToken`, optional `timeoutNanoseconds`, and
raw callbacks. `.cancel()` requests operation cancellation. The runtime waits
for native/provider cleanup before publishing results. `.value()` does not
automatically wire Swift Task cancellation; use the token or `.cancel()`.

Failures throw `TDFError` with `kind`, `code`, `operation`, `httpStatus`,
`causeCategory`, `serverCode`, `serverMessage` and `requiredObligations`.
Cancellation/deadline errors retain their categories. Error descriptions contain
a short code/operation summary; server diagnostics remain separate fields.

## Verification

Delivery tooling accepts `swift` for pinned bootstrap, repeatable package builds,
independent importing consumers and exact coverage checks. CI adds a Swift job
using the shared compiler pin. Focused execution includes **BASIC, EC and
nonce-enforced DPoP**, with both wrapping/session algorithms and both signing
algorithms in secure profiles. Manual full execution includes all seven input
cases, typed rejection tests, host token providers, negative transport fixtures
and profile integrity supplements.

```sh
TDF_COMPOSE_PROJECT=phase7-swift-local \
    python3 scripts/delivery-job.py swift --mode focused
TDF_COMPOSE_PROJECT=phase7-swift-full \
    python3 scripts/delivery-job.py swift --mode full
```

Use fresh ignored outputs for each run. KAS/Keycloak and the pinned Go/Web SDKs
are required; a self-round-trip is insufficient. Generated output, secrets and
execution receipts remain ignored. The README now includes bounded Linux Swift
performance measurements; see [benchmark methodology](benchmarks.md#swift-performance-follow-up)
for sample counts, compiler commits and comparison limits.

The initial full local acceptance used CI-green compiler commit
`e4ce9461f70306cfcf9b04e51fc522df10f179e5` and fresh isolated services
on ports 18080/18888/15432. Default loopback ports were substituted only in the
ignored local test environment. The independently built Swift consumer passed
35 BASIC, 280 EC and 226 enforced-DPoP comparisons (including self checks).
Exact coverage verified 28/224/168 required cross-SDK directions respectively.
Both secure profiles passed all 16 ciphertext/root integrity mutations;
BASIC passed all 11 controlled transport cases and the typed rejection suite.
Typed Bearer/DPoP providers, mismatched-key rejection, cancellation/recovery,
binary ownership, Unicode MIME type and empty metadata were exercised. The
four stock Web nonce-auth failures remain explicitly attributed limitations.
The shared offline gate passed 140 named checks, and 29 CI-tooling regressions
passed. The SDK's hosted CI job has not yet run for these local changes.
