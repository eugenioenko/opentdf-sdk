# Generated Swift TDF3 library

Swift is the eighth native target. The baseline is **Linux x86_64, Swift 6.4.0,
Swift 5 language mode**. macOS/iOS integration is unverified. The package is
experimental, like the rest of this POC, and its API may change.

## Build

Use adjacent `sdk`, `goalchemy`, `platform` and `web-sdk` checkouts. The
Goalchemy revision for all eight targets is pinned in
[references.lock.json](../references.lock.json): v0.7.4. The build helper
selects `--pause-model=native`, so suspending source functions emit Swift
`async throws` functions driven by Goalchemy's scheduler.

Ubuntu 22.04 prerequisites:

```sh
sudo apt-get install build-essential pkg-config libssl-dev zlib1g-dev \
    libcurl4-openssl-dev libxml2 libsqlite3-0 libncurses6 libpython3.10
```

From the workspace containing the adjacent repositories:

```sh
cd goalchemy
git checkout "$(python3 -c 'import json;print(json.load(open("../sdk/references.lock.json"))["repositories"]["goalchemy"]["revision"])')"
bash scripts/fetch-toolchains.sh swift
export PATH="$PWD/.toolchains/swift-6.4.0/usr/bin:$PATH"
go build -o out/swift-tdf-library/goalchemy ./cmd/goalchemy
cd ../sdk
GOALCHEMY_BIN=../goalchemy/out/swift-tdf-library/goalchemy \
    bash scripts/build-generated-swift.sh out/swift-tdf-library/sdk
```

Relative and absolute compiler/output paths work from independent caller
directories. `GOALCHEMY_ROOT` can select a separate compiler source snapshot.
The helper creates a SwiftPM source package; production calls require no Go
runtime or reference SDK. OpenSSL 3 provides crypto, libcurl provides HTTP, and
zlib provides native CRC32. SwiftPM discovers system dependencies with
pkg-config. The only remote Swift package is
[SwiftProtobuf](https://github.com/apple/swift-protobuf) 1.38.1 (Apache-2.0),
the runtime of the service clients' messages, pinned by exact version in
`Package.swift` and by revision in `Package.resolved`; SwiftPM fetches it on
the first build. Native library versions depend on the host package manager;
the Swift compiler archive is SHA-256 pinned.

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

The product/module is `OpenTDFTDF3`. It exports the handle API of
[api-spec.md](api-spec.md) through the hand-written
[adapter](../src/hosts/swift/OpenTDF.swift.in), with the token provider
types in [Token.swift.in](../src/hosts/swift/Token.swift.in).
The package is compiled from the export root `src`;
the shared source's `Error` record is emitted as `Error_` by the Swift
emitter's public-name rule (goalchemy#79), and the adapter presents it as
`OpenTDFError`.

```swift
import Foundation
import OpenTDFTDF3

let sdk = try await SDK("http://localhost:8080",
                        Options(clientId: "opentdf-sdk", clientSecret: "secret", insecurePlaintext: true))
let object = try await sdk.createTDF(plaintext, TDFOptions(
  attributes: ["https://example.com/attr/attr1/value/value1"],
  kasInformation: [KASInfo(url: "http://localhost:8080/kas")], mimeType: "text/plain"))
let manifest = try await object.manifest()      // typed, the reference's JSON tags
try object.close()
let reading = ReaderOptions(kasAllowlist: ["http://localhost:8080"])
let reader = try await sdk.loadTDF(object.bytes, reading)
let attributes = try await reader.dataAttributes()
try await reader.initialize()                   // fetch and unwrap keys now
let metadata = try await reader.unencryptedMetadata()
try reader.close()
do {
  let plain = try await sdk.decryptBytes(object.bytes, reading)
} catch let e as OpenTDFError {
  print(e.kind.name, e.httpStatus, e.message)
}
try sdk.close()
```

| Adapter symbol | Reference | Notes |
| --- | --- | --- |
| `try await SDK(endpoint, Options)`, `SDK.create` (synchronous) | `New` | Fetches `/.well-known/opentdf-configuration` and the issuer's OpenID configuration unless `platformConfiguration` is given; `validateConnection` runs the health check first. |
| `sdk.createTDF(Data, TDFOptions)` → `TDFObject` (`bytes`, `manifest()`, `size()`, `close()`), `sdk.encryptBytes` | `CreateTDFContext` over bytes, `TDFObject` | Key access templates from autoconfigure or explicit KAS information, split across KAS; RSA-2048/4096 and P-256/384/521 wrapping. |
| `sdk.encryptTo(HostStream output, HostStream plaintext, TDFOptions)` → `TDFObject` (empty `bytes`) | `CreateTDFContext` over streams | `HostStream` (the runtime's `GoalchemyStream`) carries `read`, `write`, `seek` and `close` closures, or is built from bytes, an `InputStream` or an `OutputStream`. The payload is encrypted one segment at a time; a `seek` closure or `TDFOptions.inputSize` selects ZIP32, an unknown size writes ZIP64. |
| `sdk.loadTDF(Data \| HostStream, ReaderOptions)` → `Reader` (`initialize()`, `manifest()`, `policy()`, `dataAttributes()`, `unencryptedMetadata()`, `read(maxLength)`, `seek(offset, whence)`, `readAt(offset, length)`, `writeTo(HostStream)`, `io()`, `close()`), `sdk.decryptBytes`, `sdk.decryptTo(HostStream output, HostStream ciphertext, ReaderOptions)`, `sdk.decryptFile(inputPath, outputPath, ReaderOptions)` | `LoadTDF`, `Reader`, `DecryptBytes`, `DecryptTo`, `DecryptFile` | A stream with `read` and `seek` is read segment by segment; `io()` is the plaintext as a `SourceStream` (`read`, `seek`, `for try await chunk in`) sharing the reader's position. Call `initialize()` first when the SDK uses a token provider (view reads carry no callbacks). `decryptFile` writes beside the output and renames into place. One rewrap request per KAS; the DEK is the XOR of one share per split id, any KAO of a split will do, duplicates are skipped; post-quantum KAOs fail as `unsupported_key_access`. RSA and EC sessions of every non-PQ key type. `Reader.Init` is `initialize()` (`init` is reserved for initializers in Swift). The KAS allowlist comes from the registry unless `kasAllowlist` or `ignoreAllowlist` is set. |
| `TDFOptions.assertions` (`AssertionConfig` with `signingKey: AssertionKey?`), `TDFOptions.systemMetadataAssertion`; `ReaderOptions.assertionVerificationKeys` (`AssertionVerificationKeys(defaultKey:, keys: [assertionId: AssertionKey])`), `ReaderOptions.disableAssertionVerification` | `WithAssertions`, `WithSystemMetadataAssertion`, `WithAssertionVerificationKeys`, `WithDisableAssertionVerification` | `AssertionKey.hs256(secret)` / `.rs256(pem)`: assertions are signed into the manifest with HS256 over the payload key or the config's signing key and verified on read by default, by assertion id with a default key, or with the payload key; failures are `.assertionFailure` with `assertionId`. |
| `sdk.isHealthy()`, `platformConfiguration()`, `getBaseKey()`, `storeKASKeys`, `close()` | same | `platformConfiguration()` returns `[String: String]` (non-string members as compact JSON). |
| `SDK.isValidTdf`, `getTdfType`, `isPlatformEndpointMalformed`, `keyTypeToPolicyAlgorithm`, `policyAlgorithmToKeyType`, `version`, `tdfSpecVersion`, `supportedFeatures` | same | Static functions; each is a library call. |

Methods are `async throws` and honour Swift `Task` cancellation: cancelling
the calling task cancels the library call (kind `canceled`). Options are
structs with defaults (`Options`, `TDFOptions`, `ReaderOptions`, `KASInfo`,
`TokenExchange`, `CertExchange`, `TLSOptions`) using the shared field names
in camelCase; every default is the reference default. Strings are Swift
`String`s, payloads `Data`, sizes `Int64`. `tokenProvider` takes a
`TokenProvider` closure and replaces client credentials; the SDK
then signs rewrap requests with the reference's ephemeral ES256 key unless
`dpopKeyPem` is set. Options this build does not implement yet fail with
`.unsupportedFeature` and `unsupported in this build: <option>`. `Manifest`,
`PolicyObject`, `KeyAccess` and the other manifest types are typed structs
whose fields are the reference's JSON tags.

Every failure is an `OpenTDFError` with `kind` (an `ErrorKind` enum whose
`name` is the api-spec string; `.boundary(name)` carries the runtime's kind
for `closed`, `poisoned`, `invalid_argument`, `host`), `op`, `message` (the
reference's wording, see [error-wording.md](error-wording.md)),
`httpStatus`, `connectCode`, `serviceMessage`, `kasUrl`,
`requiredObligations` and `assertionId`.

### Service clients

`sdk.attributes`, `sdk.namespaces` and the other properties return clients
named after the reference's `sdkconnect` interfaces
(`AttributesServiceClient`, `NamespaceServiceClient`, ...) over the
SwiftProtobuf messages generated by `protoc-gen-swift` 1.38.1 into the
`OpenTDFTDF3` module (`Policy_Attributes_ListAttributesRequest`, ...). Each
RPC is an `async throws` method whose request defaults to the empty message.
Messages cross as protobuf JSON (unknown fields ignored) through `sdk.call`
and the SDK's authenticated transport; a failure the service reports is an
`OpenTDFError` with `connectCode`.

```swift
var request = Policy_Attributes_ListAttributesRequest()
request.namespace = "example.com"
let attrs = try await sdk.attributes.listAttributes(request)
let json = try await sdk.call("policy.namespaces.NamespaceService", "ListNamespaces", "{}")
```

The helpers are `sdk.listAttributes(_:)` (`[Policy_Attribute]`),
`getEntityAttributes(_: Authorization_Entity?)`, `validateAttributes(_:)`,
`attributeExists(_:)` and `attributeValueExists(_:)`, all `async throws`;
`sdk.auth().accessToken()` returns the SDK's `AccessToken` and `sdk.conn()`
the core endpoint. `Options.coreEndpoint`, `policyEndpoint`,
`authorizationEndpoint`, `entityResolutionEndpoint` and `wellknownEndpoint`
route those services through the SDK's transport.

```swift
let exists = try await sdk.attributeValueExists("https://example.com/attr/level/value/high")
let token = try await sdk.auth().accessToken()
```

With autoconfigure on (the default), `TDFOptions.attributes` drive the
reference's key split (grants and mapped keys, else the base key, else the
default KAS); `attributeValues` (`Policy_Value`s with their definitions)
plan offline. `SDK.newAttributeValueFQN(_:)` and `SDK.newAttributeNameFQN(_:)`
return `AttributeFQN`.

```swift
let tdf = try await sdk.createTDF(data, TDFOptions(attributes: ["https://example.com/attr/level/value/high"]))
```

`src/hosts/swift/cmd/otdfgc/main.swift` is the native Swift CLI over this
package, built by `scripts/build-swift-cli.sh <package> <binary>` and
registered with the xtest harness as the `gcswift` SDK type (see
[xtest.md](xtest.md)); it implements the Go `otdfgc` contract. The consumer
in `tests/interop/generatedswift/Consumer.swift` drives the real platform
(including the service clients) and, in `controlled` mode, the offline checks of
`tests/interop/delivery/controlled_platform.py`.

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
