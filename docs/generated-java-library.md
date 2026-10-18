# Generated Java library

The shared source compiles into `tdf3-java.jar` (package `io.opentdf.tdf3`),
emitted from the export root `src` (the handle API of
[api-spec.md](api-spec.md)). The hand-written
[host adapter](../src/hosts/java) (`SDK`, `Reader`, `TDFObject`,
`TDFOptions`, `ReaderOptions`, `OpenTDFException`, `Values`, and the token
provider types in `TDF3`) presents that API
in Java idiom: it maps names, idioms and errors and implements no protocol.
Production calls use the generated Java, JDK 21 JCA, `java.net.http`, the
pinned Bouncy Castle provider and, for the platform service clients,
protobuf-java; no compiler, Go process or external executable.

## Build and import

JDK 21 (the repository pins Temurin 21.0.12.1+1 under
`goalchemy/.toolchains`), Bash and Python 3. From `goalchemy/`, build
`GOTOOLCHAIN=go1.25.14 go build -o out/java-tdf-library/goalchemy
./cmd/goalchemy`, then run `scripts/build-generated-java.sh [destination]`.
`GOALCHEMY_BIN` selects a compiler binary, `GOALCHEMY_ROOT` the Goalchemy
checkout (toolchains, the Java target's dependency lock) and `JAVA_HOME` a
JDK (default: the pinned one under the checkout). The script compiles both
roots, copies every `src/hosts/java/*.java.in` adapter file into
`src/io/opentdf/tdf3/` and the generated protobuf messages of
`src/hosts/java/protocol` into `src/io/opentdf/tdf3/protocol/`, compiles the
generated and adapter sources with `javac`, and writes `tdf3-java.jar`,
`lib/bcprov-jdk18on-1.86.jar`, the protobuf runtime of
`protocol-dependencies.lock.json` (`protobuf-java` and `protobuf-java-util`
4.33.6, `gson` 2.8.9, each checked against its SHA-256), both dependency
locks, licenses and `package-sha256.json`. JAR entry timestamps are fixed for
repeatable builds.

An independent application compiles and runs using only the JARs:

```sh
javac -cp "$PACKAGE/tdf3-java.jar:$PACKAGE/lib/*" -d classes Consumer.java
java -cp "classes:$PACKAGE/tdf3-java.jar:$PACKAGE/lib/*" example.Consumer
```

`tests/interop/generatedjava/Consumer.java` is the independent consumer: it
drives the real platform (encrypt, decrypt, 100 round trips with a cancelled
host provider, a typed negative, the service clients) and, in `controlled` mode, the offline
checks of `tests/interop/delivery/controlled_platform.py` (unreachable
platform, discovery failures, untrusted TLS, caller and source deadlines, a
401 token endpoint, allowlist, invalid input, unsupported option, invalid
argument, 100 reader handles, closed handle).

## Handle API

```java
import io.opentdf.tdf3.*;

try (SDK sdk = SDK.builder("http://localhost:8080")
        .clientCredentials("opentdf-sdk", "secret").insecurePlaintext(true).build()) {
    TDFOptions options = TDFOptions.builder()
        .attributes("https://example.com/attr/attr1/value/value1")
        .kasInformation(new TDFOptions.KASInfo("http://localhost:8080/kas"))
        .mimeType("text/plain").build();
    byte[] ciphertext;
    try (TDFObject object = sdk.createTDF(plaintext, options)) {
        Map<String, Object> manifest = object.manifest();   // keyed by the reference's JSON tags
        ciphertext = object.bytes();                        // or: sdk.encryptBytes(plaintext, options)
    }
    ReaderOptions reading = ReaderOptions.builder().kasAllowlist("http://localhost:8080").build();
    try (Reader reader = sdk.loadTDF(ciphertext, reading)) {
        List<String> attributes = reader.dataAttributes();
        reader.init(Duration.ofSeconds(30));               // fetch and unwrap keys now, within the Duration
        byte[] metadata = reader.unencryptedMetadata();
    }
    try {
        byte[] plain = sdk.decryptBytes(ciphertext, reading);
    } catch (OpenTDFException e) {
        System.out.println(e.kind() + " " + e.httpStatus() + " " + e.getMessage());
    }
    CompletableFuture<byte[]> pending = sdk.decryptBytesAsync(ciphertext, reading);
    pending.cancel(true);                                   // cancels the library call
}
```

| Adapter symbol | Reference | Notes |
| --- | --- | --- |
| `SDK.builder(endpoint)....build()`, `build(Duration)`, `buildAsync()` | `New` | Fetches `/.well-known/opentdf-configuration` and the issuer's OpenID configuration unless `platformConfiguration` is given; `validateConnection(true)` runs the health check first. |
| `sdk.createTDF(byte[], TDFOptions)` → `TDFObject` (`bytes()`, `manifest()`, `size()`, `close()`), `sdk.encryptBytes` | `CreateTDFContext` over bytes, `TDFObject` | Key access templates from autoconfigure or explicit KAS information, split across KAS; RSA-2048/4096 and P-256/384/521 wrapping. Streams arrive in Phase 7. |
| `sdk.loadTDF(byte[], ReaderOptions)` → `Reader` (`init`, `manifest`, `policy`, `dataAttributes`, `unencryptedMetadata`, `close`), `sdk.decryptBytes` | `LoadTDF`, `Reader`, `DecryptBytes` | One rewrap request per KAS; the DEK is the XOR of one share per split id, any KAO of a split will do, duplicates are skipped; post-quantum KAOs fail as `unsupported_key_access`. RSA and EC sessions of every non-PQ key type. The KAS allowlist comes from the registry unless `kasAllowlist` or `ignoreAllowlist` is set, as in the reference. |
| `TDFOptions.Builder.assertions(List<AssertionConfig>)` (`AssertionConfig` with an optional `AssertionKey signingKey`), `systemMetadataAssertion(true)`; `ReaderOptions.Builder.assertionVerificationKeys(AssertionKey defaultKey, Map<String, AssertionKey> keys)`, `disableAssertionVerification(true)` | `WithAssertions`, `WithSystemMetadataAssertion`, `WithAssertionVerificationKeys`, `WithDisableAssertionVerification` | `AssertionKey.hs256(secret)` / `rs256(pem)`: assertions are signed into the manifest with HS256 over the payload key or the config's signing key and verified on read by default, by assertion id with a default key, or with the payload key; failures carry kind `assertion_failure` and `assertionId()`. |
| `sdk.isHealthy`, `platformConfiguration`, `getBaseKey`, `storeKASKeys`, `close` | same | `platformConfiguration()` returns the members as `Map<String, String>` (non-string members as compact JSON); `getBaseKey()` a map keyed `kasUri`, `publicKey{algorithm,kid,pem}`, `kasId`. |
| `SDK.isValidTdf`, `getTdfType`, `isPlatformEndpointMalformed`, `keyTypeToPolicyAlgorithm`, `policyAlgorithmToKeyType`, `version`, `tdfSpecVersion`, `supportedFeatures` | same | Static methods; each is a library call. |

Every blocking method has an overload taking a `Duration`: when it expires
the call is cancelled and `OpenTDFException` with kind `deadline_exceeded`
is thrown. Every method has an `*Async` twin returning a
`CompletableFuture` whose `cancel` cancels the library call. `SDK`,
`TDFObject` and `Reader` are `AutoCloseable`. Options are builders over the
shared-source structs (`SDK.Builder` for `Options`, `TDFOptions.Builder`,
`ReaderOptions.Builder`) with the shared field names in camelCase
(`clientCredentials`, `insecurePlaintext`, `dpopAlgorithm`,
`kasInformation`, `wrappingKeyAlgorithm`, `kasAllowlist`, `sessionKeyType`,
...); every unset field is the reference default. Sizes are `long`, strings
UTF-8 `String`s, payloads `byte[]`. `tokenProvider` takes a
`TDF3.TokenProvider` (a `ProviderRequest` resolved with an `AccessToken`)
and replaces client credentials; the SDK then signs rewrap requests with the
reference's ephemeral ES256 key unless `dpopKeyPem` is set. Options this
build does not implement yet (`tokenExchange`, `certExchange`, `tls`,
`unixSocket`, service endpoint overrides, target mode `4.2.2`, schema
validation, post-quantum key types) fail with kind
`unsupported_feature` and the message `unsupported in this build: <option>`.

Manifests and policies are returned as unmodifiable `Map<String, Object>`
trees keyed by the reference's JSON tags
(`encryptionInformation.keyAccess[0].url`, `payload.mimeType`,
`policy.body.dataAttributes`), with `String`, `Long`, `Boolean` and `List`
leaves, so they read like `manifest.json`.

## Service clients

`SDK` extends `PlatformServices`: `sdk.attributes()`, `sdk.namespaces()` and
the other accessors return the reference's `sdkconnect` interfaces
(`AttributesServiceClient`, `NamespaceServiceClient`, ...) over the
protobuf-java messages of `io.opentdf.tdf3.protocol` (generated by `protoc`
33.6 from the pinned platform protos; `buf/validate` is
`io.opentdf.tdf3.protocol.buf.validate`). Each RPC is a blocking method with
an optional `Duration` and an `*Async` twin. Messages cross as protobuf JSON
(`JsonFormat`, unknown fields ignored) through `sdk.call` and the SDK's
authenticated transport; a failure the service reports is an
`OpenTDFException` with `connectCode()`.

```java
import io.opentdf.tdf3.protocol.policy.attributes.ListAttributesRequest;

var attrs = sdk.attributes().listAttributes(ListAttributesRequest.newBuilder().setNamespace("example.com").build());
String json = sdk.call("policy.namespaces.NamespaceService", "ListNamespaces", "{}");
```

The helpers are `sdk.listAttributes(namespace)` (`List<Attribute>`),
`sdk.getEntityAttributes(Entity)`, `sdk.validateAttributes(List<String>)`,
`sdk.attributeExists(fqn)` and `sdk.attributeValueExists(fqn)`, blocking with
an optional `Duration` and with `*Async` twins; `sdk.auth().accessToken()`
returns a `TDF3.AccessToken` and `sdk.conn()` the core endpoint.
`Builder.coreEndpoint`, `policyEndpoint`, `authorizationEndpoint`,
`entityResolutionEndpoint` and `wellknownEndpoint` route those services
through the SDK's transport.

```java
List<String> fqns = sdk.getEntityAttributes(Entity.newBuilder().setId("e1").setClientId("my-service").build());
boolean exists = sdk.attributeValueExists("https://example.com/attr/level/value/high");
```

With autoconfigure on (the default), `Builder.attributes` drive the
reference's key split (grants and mapped keys, else the base key, else the
default KAS); `Builder.attributeValues(List<Value>)` plans offline from
values with their definitions. `SDK.newAttributeValueFQN(fqn)` and
`SDK.newAttributeNameFQN(fqn)` return `SDK.AttributeFQN(url, key)`.

```java
TDFObject object = sdk.createTDF(data, TDFOptions.builder().attributes("https://example.com/attr/level/value/high").build());
```

## Errors

Every failure is an `OpenTDFException` with `kind()`, `op()`,
`getMessage()` (the reference's wording, see
[error-wording.md](error-wording.md)), `httpStatus()`, `connectCode()`,
`serviceMessage()`, `kasUrl()`, `requiredObligations()` and
`assertionId()`. Boundary failures keep the runtime's kind: `canceled`,
`deadline_exceeded` (a Duration expired), `closed` (a call after `close()`),
`invalid_argument` (a null payload), `poisoned`, `host_fault`. Switch on
`kind()`; the kinds are the `ErrorKind` list of [api-spec.md](api-spec.md).

## CLI

`src/hosts/java/cmd/otdfgc/Main.java` is the native Java CLI over this
package, built by `scripts/build-java-cli.sh <package> <launcher>` (a
launcher over the pinned JDK with the package's JARs on the classpath) and
driven by the xtest harness as the `gcjava` SDK type (see
[xtest.md](xtest.md)). It implements the same contract as the Go `otdfgc`:
`encrypt`, `decrypt`, `inspect` (the manifest read from the ZIP), `supports
<feature>` and `--version --json` with the same flags, exit codes (0, 1
operation failed, 2 usage or `unsupported in this build: <flag>`) and
otdfctl's wording.
