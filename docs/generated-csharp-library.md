# Generated C# library

The shared source compiles into `OpenTDF.TDF3.dll` (namespace `OpenTDF`),
emitted from the two export roots `src/library` (the deprecated `TDF3`
façade) and `src` (the handle API of [api-spec.md](api-spec.md)). The
hand-written [host adapter](../src/hosts/csharp/Sdk.cs.in) presents that API
in C# idiom: it maps names, idioms and errors and implements no protocol.
Crypto and HTTP use .NET 8 built-ins; IEEE CRC32 uses the official Microsoft
`System.IO.Hashing` 8.0.0 package and the platform service clients
`Google.Protobuf` 3.33.6, both locked in
[dependencies.lock.json](../src/hosts/csharp/dependencies.lock.json). The
production path has no Go process, compiler or reference SDK.

## Build and import

The .NET 8 SDK (the repository pins SDK 8.0.425 under
`goalchemy/.toolchains/dotnet`), Bash and Python 3. From `goalchemy/`, build
`GOTOOLCHAIN=go1.25.14 go build -o out/csharp-tdf-library/goalchemy
./cmd/goalchemy`, then run `scripts/build-generated-csharp.sh [destination]`.
`GOALCHEMY_BIN` selects a compiler binary, `GOALCHEMY_ROOT` the Goalchemy
checkout and `DOTNET_BIN` another .NET 8 SDK. The script compiles both roots,
adds `TDF3.cs` (façade), `Sdk.cs` (handle API), `Services.cs` (service
clients) and the generated protobuf messages of `src/hosts/csharp/protocol`,
restores `Google.Protobuf` in locked mode against its content hash, builds a
deterministic `lib/OpenTDF.TDF3.dll` and ships `lib/System.IO.Hashing.dll`,
`lib/Google.Protobuf.dll`, the .NET, Goalchemy and protobuf licenses and
`package-sha256.json`. A net8.0 project references the three DLLs:

```xml
<ItemGroup>
  <Reference Include="OpenTDF.TDF3"><HintPath>/absolute/package/lib/OpenTDF.TDF3.dll</HintPath></Reference>
  <Reference Include="System.IO.Hashing"><HintPath>/absolute/package/lib/System.IO.Hashing.dll</HintPath></Reference>
  <Reference Include="Google.Protobuf"><HintPath>/absolute/package/lib/Google.Protobuf.dll</HintPath></Reference>
</ItemGroup>
```

`tests/interop/generatedcsharp/Consumer.cs` is the independent consumer: it
drives the real platform (encrypt, decrypt, 100 round trips with a cancelled
host provider, a typed negative, the service clients) and, in `controlled` mode, the offline
checks of `tests/interop/delivery/controlled_platform.py` (unreachable
platform, discovery failures, untrusted TLS, caller and source deadlines, a
401 token endpoint, allowlist, invalid input, unsupported option, invalid
argument, 100 reader handles, closed handle).

## Handle API

```csharp
using OpenTDF;

await using var sdk = await Sdk.CreateAsync("http://localhost:8080",
    new Options { ClientId = "opentdf-sdk", ClientSecret = "secret", InsecurePlaintext = true }, ct);
await using (var obj = await sdk.CreateTdfAsync(plaintext, new TdfOptions {
    Attributes = new[] { "https://example.com/attr/attr1/value/value1" },
    KasInformation = new[] { new KasInfo("http://localhost:8080/kas") },
    MimeType = "text/plain" }, ct))
{
    var manifest = await obj.ManifestAsync(ct);   // keyed by the reference's JSON tags
    ciphertext = obj.Bytes;                       // or: await sdk.EncryptBytesAsync(plaintext, options, ct)
}
var reading = new ReaderOptions { KasAllowlist = new[] { "http://localhost:8080" } };
await using (var reader = await sdk.LoadTdfAsync(ciphertext, reading, ct))
{
    var attributes = await reader.DataAttributesAsync(ct);
    await reader.InitAsync(ct);                   // fetch and unwrap keys now, under this token
    var metadata = await reader.UnencryptedMetadataAsync(ct);
}
try { var plain = await sdk.DecryptBytesAsync(ciphertext, reading, ct); }
catch (OpenTdfException e) { Console.WriteLine($"{e.Kind} {e.HttpStatus} {e.Message}"); }
```

| Adapter symbol | Reference | Notes |
| --- | --- | --- |
| `await Sdk.CreateAsync(endpoint, Options, ct)` | `New` | Fetches `/.well-known/opentdf-configuration` and the issuer's OpenID configuration unless `PlatformConfiguration` is given; `ValidateConnection` runs the health check first. |
| `sdk.CreateTdfAsync(byte[], TdfOptions, ct)` → `TdfObject` (`Bytes`, `ManifestAsync`, `SizeAsync`, `CloseAsync`), `sdk.EncryptBytesAsync` | `CreateTDFContext` over bytes, `TDFObject` | Single-KAS templates; RSA-2048 and P-256 wrapping. Streams arrive in Phase 7. |
| `sdk.LoadTdfAsync(byte[], ReaderOptions, ct)` → `Reader` (`InitAsync`, `ManifestAsync`, `PolicyAsync`, `DataAttributesAsync`, `UnencryptedMetadataAsync`, `CloseAsync`), `sdk.DecryptBytesAsync` | `LoadTDF`, `Reader`, `DecryptBytes` | The KAS allowlist comes from the registry unless `KasAllowlist` or `IgnoreAllowlist` is set, as in the reference. |
| `sdk.IsHealthyAsync`, `PlatformConfigurationAsync`, `GetBaseKeyAsync`, `StoreKasKeysAsync`, `CloseAsync` | same | `PlatformConfigurationAsync` returns the members as strings (non-string members as compact JSON); `GetBaseKeyAsync` a dictionary keyed `kasUri`, `publicKey{algorithm,kid,pem}`, `kasId`. |
| `Sdk.IsValidTdfAsync`, `GetTdfTypeAsync`, `IsPlatformEndpointMalformedAsync`, `KeyTypeToPolicyAlgorithmAsync`, `PolicyAlgorithmToKeyTypeAsync`, `VersionAsync`, `TdfSpecVersionAsync`, `SupportedFeaturesAsync` | same | Static methods; each is a library call. |

Every method returns a `Task` and takes an optional `CancellationToken`;
cancelling it cancels the library call (kind `canceled`). `Sdk`,
`TdfObject` and `Reader` are `IAsyncDisposable` (and `IDisposable`).
Options are records with `init` setters (`Options`, `TdfOptions`,
`ReaderOptions`, `KasInfo`, `TokenExchange`, `CertExchange`, `TlsOptions`)
using the shared field names in PascalCase; every unset member is the
reference default. `Timeout` is a `TimeSpan?` (the shared `TimeoutMillis`),
sizes are `long`, strings UTF-8 `string`s, payloads `byte[]`.
`TokenProvider` takes the façade's `TDF3.TokenProvider` (or
`TDF3.FromTask`) and replaces client credentials; the SDK then signs rewrap
requests with the reference's ephemeral ES256 key unless `DpopKeyPem` is
set. Options this build does not implement yet (`TokenExchange`,
`CertExchange`, `Tls`, `UnixSocket`, service endpoint overrides, assertions,
target mode `4.2.2`, schema validation, RSA-4096 and P-384/P-521 keys) fail
with kind `unsupported_feature` and the message `unsupported in this build:
<option>`.

Manifests and policies are returned as read-only
`IReadOnlyDictionary<string, object>` trees keyed by the reference's JSON
tags (`encryptionInformation.keyAccess[0].url`, `payload.mimeType`,
`body.dataAttributes`), with `string`, `long`, `bool` and list leaves.

## Service clients

`Sdk` derives from `PlatformServices`: `sdk.Attributes`, `sdk.Namespaces` and
the other properties return the reference's `sdkconnect` interfaces with the
C# `I` prefix (`IAttributesServiceClient`, `INamespaceServiceClient`, ...)
over the Google.Protobuf messages of `OpenTDF.Protocol` (generated by `protoc`
33.6 from the pinned platform protos; `buf/validate` is
`OpenTDF.Protocol.Buf.Validate`). Each RPC is a `Task` method taking a
`CancellationToken`. Messages cross as protobuf JSON (`JsonFormatter`,
`JsonParser` ignoring unknown fields) through `sdk.CallAsync` and the SDK's
authenticated transport; a failure the service reports is an
`OpenTdfException` with `ConnectCode`.

```csharp
using OpenTDF.Protocol.Policy.Attributes;

var attrs = await sdk.Attributes.ListAttributesAsync(new ListAttributesRequest { Namespace = "example.com" }, ct);
string json = await sdk.CallAsync("policy.namespaces.NamespaceService", "ListNamespaces", "{}", ct);
```

The helpers are `ListAttributesAsync(ns)`, `GetEntityAttributesAsync(Entity)`,
`ValidateAttributesAsync(fqns)`, `AttributeExistsAsync(fqn)` and
`AttributeValueExistsAsync(fqn)`, each with a `CancellationToken`;
`sdk.Auth().AccessTokenAsync()` returns a `TDF3.AccessToken` and
`ConnAsync()` the core endpoint. `Options.CoreEndpoint`, `PolicyEndpoint`,
`AuthorizationEndpoint`, `EntityResolutionEndpoint` and `WellknownEndpoint`
route those services through the SDK's transport.

```csharp
var fqns = await sdk.GetEntityAttributesAsync(new OpenTDF.Protocol.Authorization.Entity { Id = "e1", ClientId = "my-service" }, ct);
await sdk.ValidateAttributesAsync(new[] { "https://example.com/attr/level/value/high" }, ct);
```

With autoconfigure on (the default), `TdfOptions.Attributes` drive the
reference's key split (grants and mapped keys, else the base key, else the
default KAS); `TdfOptions.AttributeValues` (`Policy.Value` messages with
their definitions) plan offline. `Sdk.NewAttributeValueFqnAsync(fqn)` and
`Sdk.NewAttributeNameFqnAsync(fqn)` return `AttributeFqn(Url, Key)`.

```csharp
await using var tdf = await sdk.CreateTdfAsync(data, new TdfOptions { Attributes = new[] { "https://example.com/attr/level/value/high" } });
```

## Errors

Every failure is an `OpenTdfException` with `Kind`, `Op`, `Message` (the
reference's wording, see [error-wording.md](error-wording.md)),
`HttpStatus`, `ConnectCode`, `ServiceMessage`, `KasUrl`,
`RequiredObligations` and `AssertionId`. Boundary failures keep the
runtime's kind: `canceled`, `deadline_exceeded`, `closed` (a call after
close), `invalid_argument` (a null payload), `poisoned`, `host_fault`.
Switch on `Kind`; the kinds are the `ErrorKind` list of
[api-spec.md](api-spec.md).

## Deprecated façade

`TDF3.Encrypt`, `TDF3.Decrypt`, `TDF3.EncryptAsync`, `TDF3.DecryptAsync`,
`TDF3.Config`, `TDF3.EncryptOptions`, `TDF3.Decrypted`, `TDF3.Operation` and
`TDF3.TDFError` remain with their previous shapes for one release,
implemented in the shared source as thin wrappers that construct an SDK per
call. `TDFError.Code` is now the `Error.Kind` string; `PolicyBase64`,
`Dissem` and `SegmentHashAlgorithm` other than `GMAC` are no longer
supported, and empty metadata writes no `encryptedMetadata` entry, as the
reference does.

## CLI

`src/hosts/csharp/cmd/otdfgc/Program.cs` is the native .NET CLI over this
package, built by `scripts/build-csharp-cli.sh <package> <launcher>` (a
console project referencing the package's DLLs and a launcher over the
pinned runtime) and driven by the xtest harness as the `gccs` SDK type (see
[xtest.md](xtest.md)). It implements the same contract as the Go `otdfgc`:
`encrypt`, `decrypt`, `inspect` (the manifest read from the ZIP), `supports
<feature>` and `--version --json` with the same flags, exit codes and
otdfctl's wording.
