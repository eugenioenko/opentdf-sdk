# Generated C# TDF3 library

The generated `OpenTDF.TDF3.dll` exposes `OpenTDF.TDF3` from the shared
[library facade](../src/library/library.go). It performs TDF3 encryption/decryption
inside the imported assembly; the production path has no Go process, compiler
internals or stock SDK dependency. Root acceptance and the seven-target delivery
ledger remain in [the plan](plan.md) and [delivery checklist](delivery-checklist.md).
All seven targets have accepted native libraries. [Final delivery](final-delivery.md)
records clean packages and the scope of final-package and CI validation.

Build from any working directory with Bash, Python3, the .NET8 SDK and a built
Goalchemy compiler. The checkout pins SDK8.0.425/runtime8.0.31 under
`goalchemy/.toolchains/dotnet`. On Linux the native runtime uses the system
OpenSSL/libc prerequisites of .NET8. Crypto and HTTP use .NET built-ins.
IEEE CRC32 uses the official Microsoft `System.IO.Hashing` 8.0.0 NuGet package,
with an exact version/content-hash lock; it is separate from the shared runtime.
Exact dependency versions/licenses are in
[dependencies.lock.json](../src/hosts/csharp/dependencies.lock.json); output includes
Goalchemy Apache2, .NET MIT/third-party notices, and the hashing package's MIT
license/notices. Native APIs and pinned runtime
imports provide HKDF and omitted-public-point P256 support.

```bash
cd goalchemy
GOTOOLCHAIN=go1.25.14 go build -o out/csharp-tdf-library/goalchemy ./cmd/goalchemy
GOALCHEMY_BIN=out/csharp-tdf-library/goalchemy \
  ../sdk/scripts/build-generated-csharp.sh out/csharp-tdf-library/sdk
```

`GOALCHEMY_BIN` and destination paths resolve against the invocation directory;
`DOTNET_BIN` optionally supplies another .NET8 SDK executable. The helper compiles
the shared library, copies the facade, builds a deterministic named DLL, records
package hashes and ships licenses. Independently reference both
`lib/OpenTDF.TDF3.dll` and `lib/System.IO.Hashing.dll` from a net8.0 project;
the helper packages both DLLs. A project reference to generated `main.csproj`
instead carries the NuGet dependency transitively. The runtime-only consumer
needs the .NET8 runtime and both deployed DLLs,
not Bash, Python3, Go, the SDK source or stock SDKs.

```xml
<ItemGroup>
  <Reference Include="OpenTDF.TDF3">
    <HintPath>/absolute/package/lib/OpenTDF.TDF3.dll</HintPath>
  </Reference>
  <Reference Include="System.IO.Hashing">
    <HintPath>/absolute/package/lib/System.IO.Hashing.dll</HintPath>
  </Reference>
</ItemGroup>
```

```csharp
using OpenTDF;

var config = new TDF3.Config {
    PlatformURL = "https://platform.example",
    KASURL = "https://platform.example/kas",
    IssuerURL = "https://issuer.example/realms/opentdf",
    ClientID = "server-client", ClientSecret = "server-secret"
};
var options = new TDF3.EncryptOptions {
    Attributes = new[] { "https://example.com/attr/department/value/engineering" }
};
byte[] archive = await TDF3.EncryptAsync(config, payload, options,
    cancellation: cancellationToken);
TDF3.Decrypted result = await TDF3.DecryptAsync(config, archive,
    cancellation: cancellationToken);
byte[] plaintext = result.Payload;
```

`Config` exposes routes, discovery or explicit KAS PEM/kid, RSA/P256 wrapping and
session algorithms, RS256/ES256 auth PEM, DPoP, allowHTTP and timeout. `EncryptOptions`
retains segment-size presence, metadata presence (including explicitly empty),
policy, attributes/dissemination, segment integrity and MIME. Host text uses strict
UTF8; malformed UTF16 rejects, BOM is preserved and rejected as part of an invalid
destination. Payload/metadata are arbitrary bytes. Long values remain exact.
`Decrypted` returns defensively copied Payload/Metadata and authenticated manifest
text, with HasMetadata independent of byte length.

`EncryptAsync`/`DecryptAsync` accept CancellationToken and an optional TokenProvider.
`Encrypt`/`Decrypt` additionally return an Operation with Completion Task and Cancel.
Queued cancellation does no source work; active cancellation completes only after
actual native/provider resource release. Cancellation is a typed TDFError with
Kind/Code `canceled`, rather than Task.IsCanceled. TDFError also exposes source
code/operation/status/server fields, obligations and cause category; source failure,
source panic and adapter host_fault remain distinct. Returned arrays and nested
errors stay owned across subsequent calls. Calls serialize within one assembly;
each acquired call constructs and closes its shared client and PEM imports.
There is no advertised persistent client or public native key handle API.

A native token provider can operate without source client credentials. Supply a
matching AuthPrivateKeyPEM for DPoP and return AccessToken(Value, Scheme, ExpiresAt,
ConfirmationJKT). `TDF3.FromTask` bridges a Task that completes only after all
provider-owned resources release. Explicit ProviderRequest Retain/OnStop/Resolve/
Reject/Fault supports resource settlement separately from cancellation requests.
Terminal settlement seals late Retain/OnStop. Stop hooks execute outside the
request monitor, and completion waits acquired stop invocations and their faults.
Duplicate/late replies are ignored; synchronous provider submission faults remain
host_fault even after settlement. A provider cannot synchronously wait for another
operation queued behind the call it is servicing. Never-settled retained work stays
pending to preserve honest cleanup acknowledgement.

Native .NET8 cryptography and dedicated HttpClient implement the tested TDF3 profile.
Requests use trusted default TLS, HTTP1.1, no redirects, strict trusted URLs,
bounded input/response/header bodies, deadlines and active cancellation. Native
Latin1 header selectors preserve source header bytes; invalid framing/controls
reject before contact. .NET system proxy configuration remains native behavior.
See [the reusable C# boundary](../../goalchemy/docs/csharp-library-boundary.md)
for algorithm parameters, key/driver/resource ownership and bounded value limits.

The independent [consumer](../tests/interop/generatedcsharp/Consumer.cs) imports
only the built DLL, obtains native OAuth/DPoP tokens and drives real KAS. Reusable
compiler tests cover actual generated imports and lifecycle; native crypto tests
use published vectors and independently generated native Go fixtures. Required
real-KAS evidence covers both directions against pinned stock Go/Web for all seven
empty/binary/exact/multisegment/HS256/metadata/empty-metadata cases; RSA/P256 wrapping
and response sessions, RS256/ES256 signing, discovery/explicit keys, matching native
provider, policy/auth/binding/integrity/route/unsupported-format and actual transport
negatives. Stock Web's enforced-nonce401 behavior and missing reader metadata remain
explicit reference limitations, separately labeled in matrix results. Stock-Go
session outputs are retained separately before its helper overwrites base names.
No mock positive KAS or self-round-trip alone constitutes acceptance.
