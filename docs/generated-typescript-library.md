# Generated TypeScript TDF3 library

The [shared façade](../library/library.go) and reachable SDK implementation are
compiled into an importable ESM package for Node and browsers. The small
[index adapter](../hosts/typescript/index.ts.in) supplies JS text/token types;
it does not implement the TDF protocol or import an original/reference SDK.

## Build and import

From `goalchemy/`, build `go build -o out/typescript-tdf-library/goalchemy
./cmd/goalchemy`, then run `../sdk/scripts/build-generated-typescript.sh`.
An optional destination argument selects a local package directory;
GOALCHEMY_BIN selects a rebuilt compiler binary. The script compiles from the
SDK module, adds the typed adapter, and emits JavaScript and declarations with
tsc. Verified tooling is TypeScript6.0.3 and Go1.25.14. Node requires22.6 or later;
current Node24.15.0 and Chromium147 evidence use native WebCrypto/fetch.

The output package is `@opentdf-local/tdf3`. A native consumer can run
`npm install /absolute/path/to/generated/package` and import it normally.
`npm pack` makes a reviewable local tarball; neither build nor packing publishes
anything. Browser applications bundle the ESM package with their normal
bundler. Production has no extra dependencies or Node imports/polyfills.

```typescript
import { encrypt, decrypt, TDFError, type Config } from '@opentdf-local/tdf3';
const config: Config = {
  PlatformURL: 'https://platform.example',
  IssuerURL: 'https://issuer.example/realm',
  ClientID: clientID, ClientSecret: clientSecret,
};
const archive = await encrypt(config, new Uint8Array([0, 255]), {
  Metadata: new Uint8Array(), IncludeMetadata: true,
  SegmentSize: 16384n, HasSegmentSize: true,
});
const result = await decrypt(config, archive);
// Uint8Array Payload/Metadata, boolean HasMetadata, UTF-8 string ManifestJSON.
```

Config and EncryptOptions retain exported PascalCase source field names.
Configuration preserves trusted routes, KAS URL/kid, explicit/discovered KAS
keys, RSA/P256 wrapping and response sessions, RS256/ES256 authentication,
DPoP and imported per-operation auth PEM. Options preserve policy/attributes,
dissemination, MIME type, GMAC/HS256 segmentation and metadata presence. int/int64
fields such as TimeoutMillis and SegmentSize are bigint; rounded JS numbers are
rejected. The source's existing segment-size clamping remains in effect.

Payload, archive and metadata remain byte-exact owned Uint8Arrays. Inputs copy
synchronously at invocation, including Buffer and subclasses with view-returning
slice. Config/options arrays copy indexed elements, bypassing caller map/iterator
hooks and retaining the invocation-time policy/routes. Payload/Metadata outputs are nonnullable Uint8Arrays; HasMetadata retains
absent versus explicitly empty metadata. Config/options text is UTF-8 encoded
at the adapter boundary, and ManifestJSON and typed error text are decoded to
native JS strings. The raw generated `dist/main.js` API retains nullable slices,
binary Go strings and byte ManifestJSON; the two APIs have explicit distinct
types. Calls fresh-initialize shared source and serialize safely, with durable
results/errors copied before owner retirement and global reset.

## Token providers and cancellation

Browser hosts use an independent token provider; credentials stay on their
server. Pass `{tokenProvider, signal}` as encrypt's fourth or decrypt's third
argument. TokenProvider is `(signal: AbortSignal) => Promise<AccessToken>`.
AccessToken is `{value:string, scheme:string, expiresAt:bigint,
confirmationJKT?:string}`. Expiry remains signed-int64 exact through decimal
wire JSON. The shared SDK checks expiry, scheme and DPoP binding. A DPoP provider
must obtain tokens matching Config.AuthPrivateKeyPEM; session keys are distinct.
No token cache or native key escapes an operation.

```typescript
const call = {
  signal: controller.signal,
  tokenProvider: async (signal: AbortSignal) => {
    const response = await fetch('/session/token', { signal });
    const token = await response.json();
    return { ...token, expiresAt: BigInt(token.expiresAt) };
  },
};
const result = await decrypt(browserConfig, archive, call);
```

Server credential flow is available in Node. A browser must not ship client
secrets; its broker/provider is application-owned. Real browser evidence uses a
host-side OAuth broker and actual KAS requests. CORS must allow the application's
origin and Authorization/DPoP/Content-Type/Connect-Protocol-Version, and expose
DPoP-Nonce so native source nonce retries can work.

Queued AbortSignal cancellation removes that call without disturbing the active
owner. Active cancellation requests native stop and waits for actual resource
settlement. WebCrypto itself is not abortable; its result is discarded and owned
keys released before cleanup ACK. Providers must eventually settle even after
abort. TDFError carries kind, code, operation, bigint httpStatus, serverCode,
serverMessage, requiredObligations and causeCategory. Source panic/host fault
return safe categories, preserving process survival; cancellation cause retains
the caller's signal reason.

The generic raw generated Callback API also accepts explicit settle and an
optional nonblocking stop hook. Explicit replies copy at publication, with first
settlement winning. Promise replies copy when fulfillment is observed. Stop-hook
faults still wait for resource settlement; late/duplicate publications cannot
resume retired owners.

## Native limits and interoperability

See the [compiler boundary and transport observations](../../goalchemy/docs/typescript-library-boundary.md)
for exact limits, URL/header restrictions, manual redirect/CORS behavior,
exposed merged headers, native compression and true reader cleanup. TLS
verification stays enabled and credentials are never forwarded by redirect.
Production uses native WebCrypto, fetch, secure random and bounded PEM/DER
framing, with no third-party cryptographic dependency. Build/test dependencies
are TypeScript6.0.3 Apache2, esbuild0.25.12 MIT and Playwright1.58.2 Apache2.

Independent [Node/browser interoperability tools](../tests/interop/generatedtypescript)
compare exact payload/metadata against separately labeled stock Go/Web tools
through real BASIC, EC and enforced-DPoP KAS profiles. Stock Web's enforced-DPoP
401 limitation is retained separately and is never counted as successful stock
Web authentication. Browser credentials remain host-side, and the bundle graph
is audited for Node imports/globals. Root owns profile transitions and final
acceptance across all seven targets; TypeScript acceptance alone does not imply
completion of other target SDKs.
