# Java TDF3 SDK

Build with Bash, Python 3, JDK 21 (the repository pins Temurin 21.0.12.1+1), and the adjacent
Goalchemy compiler built with Go 1.25.14:

```sh
(cd ../goalchemy && GOTOOLCHAIN=go1.25.14 go build -o out/java-tdf-library/goalchemy ./cmd/goalchemy)
export JAVA_HOME="$PWD/../goalchemy/.toolchains/jdk-21.0.12.1+1"
scripts/build-generated-java.sh ../goalchemy/out/java-tdf-library/sdk
```

The package contains `tdf3-java.jar`, `lib/bcprov-jdk18on-1.86.jar`, dependency
lock, licenses and `package-sha256.json`. JAR entry timestamps are fixed for
repeatable builds. Build helpers resolve relative compiler/destination paths
from the caller's working directory. Both dependency locks are byte-equivalent;
the downloader verifies the pin before atomic publication, with independent
concurrent temporary downloads. Bouncy Castle supplies only native HKDF-SHA256
and absent-Q P-256 derivation; JCA supplies other cryptography. The dependency
is required, including when an imported private key omits its public point.
No global provider registration or crypto-default changes occur.

An independent named-package application compiles and runs using only the JARs:

```sh
javac -cp "$PACKAGE/tdf3-java.jar:$PACKAGE/lib/bcprov-jdk18on-1.86.jar" -d classes Consumer.java
java -cp "classes:$PACKAGE/tdf3-java.jar:$PACKAGE/lib/bcprov-jdk18on-1.86.jar" example.Consumer
```

The exported adapter is `io.opentdf.tdf3.TDF3`; the compiler/native runtime source
layout is not part of a consumer build. For example:

```java
package example;
import io.opentdf.tdf3.TDF3;

public class Consumer {
  public static void main(String[] args) throws Exception {
    var config = new TDF3.Config();
    config.PlatformURL = "https://platform.example";
    config.KASURL = "https://platform.example/kas";
    config.ClientID = System.getenv("TDF_CLIENT_ID");
    config.ClientSecret = System.getenv("TDF_CLIENT_SECRET");
    var options = new TDF3.EncryptOptions();
    options.Attributes = new String[]{"https://example.com/attr/class/value/public"};
    byte[] archive = TDF3.encrypt(config, new byte[]{0, (byte)255}, options, null)
        .completion().toCompletableFuture().get();
    var clear = TDF3.decrypt(config, archive, null)
        .completion().toCompletableFuture().get();
    System.out.println(clear.Payload().length);
  }
}
```

Per call, the generated shared Go source builds/closes its client. Operations
are serialized within one generated graph per JVM classloader; independent
graphs require independent classloaders. Configuration, payload and nested
collections are snapshotted before queueing. Outputs and failures are copied
before owner retirement. `Decrypted.Payload()` and `Metadata()` return fresh
arrays; `HasMetadata()` distinguishes absence from explicit empty metadata.
Text fields use strict UTF-8, while generic generated source exports use binary
byte strings. `long` preserves source integer ranges, including exact full
64-bit values. Unsupported generic public shapes fail honestly before emission.

`Operation.cancel()` requests stop. A queued operation does no source work;
an active operation completes only after real native/provider release.
`completion()` is a CompletionStage view, and cancelling a derived Future is
not SDK cancellation. Completion continuations can submit and wait for another
operation. Native owner launch failures roll back the unacquired call; completion
launch failures use the JDK common-pool after retirement. If both independent
publishers reject, the retired driver releases the queue/gate and faults detached
calls outside the monitor. This synchronous emergency path cannot guarantee
parallel progress for arbitrary blocking host callback graphs when independent
native dispatch is unavailable. Failures are `TDF3.TDFError`, with `kind`, `code`, `operation`,
`httpStatus`, `serverCode`, `serverMessage`, `causeCategory` and copied
`requiredObligations()`. Kinds distinguish declared source failure, cancellation,
source panic, invalid argument and unexpected adapter/cleanup fault. SDK calls
never use executable `System.exit` semantics.

For client credentials held outside shared source, provide
`CallOptions.tokenProvider`. It receives a native `ProviderRequest` and resolves
an `AccessToken(value, scheme, expiresAt, confirmationJKT)`. Expiry is exact Unix
seconds. Matching DPoP tokens must carry the generated authentication key's
canonical JWK thumbprint, and provider acquisition must handle the issuer's
nonce. `retain()`/`onStop()` records native acquisition; stop only requests
cancellation. Resolve/reject/fault means provider resources have actually been
released. A synchronous adapter throw remains `host_fault`, even after a
synchronous resolve/reject. Async stage rejection via `TDF3.fromStage` is declared
token acquisition failure. An attachment throw on a live stage waits for explicit
terminal request settlement, with host-fault precedence. A provider that retains
resources and never settles intentionally prevents completion.

Transport uses trusted JVM TLS and NEVER follows redirects. It supports bounded
GET/POST, native deadlines/context cancellation, automatic gzip, Latin-1 response
header bytes and canonical sorted duplicate headers. Request header values are
ASCII-only and rejected before contact otherwise, because the pinned JDK writer
silently replaces obs-text. JVM ProxySelector/system proxy settings apply;
Go environment proxy behavior and raw header whitespace are not promised.
Exposed response headers are bounded at 64 KiB and decoded bodies at each shared
call's configured limit; native parser limits can reject earlier.

Verification uses the actual importing consumer under
`tests/interop/generatedjava`. BASIC, EC and enforced-DPoP matrices preserve
separate artifact-scoped results, stock Go/Web producers and consumers, native
OAuth providers, RSA/P-256 sessions/authentication/wrapping, metadata presence
and rejection cases. Stock Web enforced nonce failures remain explicitly labeled;
Bearer-produced format fixtures do not establish stock Web DPoP authentication.
Controlled endpoints are negative tests only; positive interoperability uses
real KAS. Generic compiler tests independently verify queue re-entry, structured
error copies, field/key GC, retained native JCA work, Close and cleanup ordering.
See [the Java library design](../../goalchemy/docs/java-library-boundary.md) for
the precise compiler boundary and reviewed JDK/dependency observations.
