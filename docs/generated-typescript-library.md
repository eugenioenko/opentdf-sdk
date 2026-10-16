# Generated TypeScript library

The shared source compiles into an importable ESM package for Node and
browsers, emitted from the two export roots `src/library` (the deprecated
`encrypt`/`decrypt` façade) and `src` (the handle API of
[api-spec.md](api-spec.md)). The hand-written
[host adapter](../src/hosts/typescript/index.ts.in) presents that API in
TypeScript idiom: it maps names, idioms and errors and implements no protocol.

## Build and import

From `goalchemy/`, build `GOTOOLCHAIN=go1.25.14 go build -o
out/typescript-tdf-library/goalchemy ./cmd/goalchemy`, then run
`scripts/build-generated-typescript.sh`. An optional destination argument
selects the package directory; `GOALCHEMY_BIN` selects a compiler binary and
`TSC_BIN` a TypeScript compiler (6.0.3 is the verified one). The script
compiles both roots from the SDK module, adds `index.ts` and `node-index.ts`,
and emits JavaScript and declarations with `tsc`. Node 22.6 or later; the
accepted evidence is Node 24.15.0 and Playwright 1.58.2 Chromium.

The package is `@opentdf-local/tdf3`. `npm install /path/to/package` or
`npm pack` for a local tarball; nothing is published. Conditional exports
select `dist/node-index.js` in Node (installing the `node:zlib.crc32`
adapter) and `dist/index.js` for browsers and bundlers; the browser graph has
no Node imports or polyfills. `tests/interop/generatedtypescript` is the
independent consumer: `main.mjs` drives the real platform (encrypt, decrypt,
100 round trips, a typed negative) and, in `controlled` mode, the offline
checks (discovery failures, deadlines, untrusted TLS, token 401, allowlist,
invalid input, unsupported options, 100 reader handles, closed handle).
`tests/interop/delivery/run-browser-focused.mjs` runs the same encrypt and
decrypt cells through the browser bundle in Chromium.

## Handle API

```typescript
import { SDK, OpenTDFError } from '@opentdf-local/tdf3';

await using sdk = await SDK.create('http://localhost:8080', {
  clientId: 'opentdf-sdk', clientSecret: 'secret', insecurePlaintext: true,
});
const object = await sdk.createTdf(plaintext, {
  attributes: ['https://example.com/attr/attr1/value/value1'],
  kasInformation: [{ url: 'http://localhost:8080/kas' }],
  mimeType: 'text/plain',
});
const manifest = await object.manifest();   // keyed by the reference's JSON tags
await object.close();
const ciphertext = object.bytes;             // or: await sdk.encryptBytes(plaintext, options)

const reader = await sdk.loadTdf(ciphertext, { kasAllowlist: ['http://localhost:8080'] });
const attributes = await reader.dataAttributes();
await reader.init();                         // fetch and unwrap keys now, under this call's signal
const metadata = await reader.unencryptedMetadata();
await reader.close();

try {
  const plain = await sdk.decryptBytes(ciphertext, { kasAllowlist: ['http://localhost:8080'] });
} catch (e) {
  if (e instanceof OpenTDFError) console.log(e.kind, e.httpStatus, e.message);
}
```

| Adapter symbol | Reference | Notes |
| --- | --- | --- |
| `SDK.create(endpoint, options?, call?)` | `New` | Fetches `/.well-known/opentdf-configuration` and the issuer's OpenID configuration unless `platformConfiguration` is given; `validateConnection` runs the health check first. |
| `sdk.createTdf(plaintext, options?, call?)` → `TDFObject` (`bytes`, `manifest()`, `size()`, `close()`), `sdk.encryptBytes` | `CreateTDFContext` over bytes, `TDFObject` | Single-KAS templates; RSA-2048 and P-256 wrapping. Streams arrive in Phase 7. |
| `sdk.loadTdf(ciphertext, options?, call?)` → `Reader` (`init`, `manifest`, `policy`, `dataAttributes`, `unencryptedMetadata`, `close`, `Symbol.asyncDispose`), `sdk.decryptBytes` | `LoadTDF`, `Reader`, `DecryptBytes` | The KAS allowlist comes from the registry unless `kasAllowlist` or `ignoreAllowlist` is set, as in the reference. |
| `sdk.isHealthy`, `platformConfiguration`, `getBaseKey`, `storeKasKeys`, `close`, `Symbol.asyncDispose` | same | `platformConfiguration()` returns the members as `Record<string, string>` (non-string members as compact JSON). |
| `isValidTdf`, `getTdfType`, `isPlatformEndpointMalformed`, `keyTypeToPolicyAlgorithm`, `policyAlgorithmToKeyType`, `version`, `tdfSpecVersion`, `supportedFeatures` | same | Free functions; every one returns a Promise. |

Options are plain objects with the shared-source field names in camelCase
(`Options`, `TDFOptions`, `ReaderOptions`); every absent field is the
reference default. Sizes and counts are `number`; `platformConfiguration` is
a `Record<string, unknown>` whose non-string members are carried as JSON
text; `tokenProvider` is `(signal: AbortSignal) => Promise<AccessToken>` and
replaces client credentials (the SDK then signs rewrap requests with the
reference's ephemeral ES256 key unless `dpopKeyPem` is set). Every method
takes an optional trailing `{ signal }`; aborting cancels the active call
(`kind: 'canceled'`). Options this build does not implement yet
(`tokenExchange`, `certExchange`, `tls`, `unixSocket`, service endpoint
overrides, assertions, target mode `4.2.2`, schema validation, RSA-4096 and
P-384/P-521 keys) fail with `kind: 'unsupported_feature'` and the message
`unsupported in this build: <option>`.

Manifests and policies are returned keyed by the reference's JSON tags
(`manifest.encryptionInformation.keyAccess[0].url`, `payload.mimeType`,
`policy.body.dataAttributes`), so they read like `manifest.json`; the
`Manifest`, `KeyAccess`, `PolicyObject` and related interfaces are exported.
Strings are UTF-8 `string`s and payloads `Uint8Array`s; the raw generated
`dist/main.js` keeps binary Go strings and bigint sizes.

## Service clients

`sdk.transport` is a Connect-ES `Transport` over the SDK's authenticated
transport (protobuf JSON through `sdk.call`), and `sdk.attributes`,
`sdk.namespaces` and the other services are `createClient` clients over it.
The messages are protobuf-es v2, exported as
`@opentdf-local/tdf3/protocol/<path>_pb.js`. Failures are `ConnectError`
whose `cause` is the `OpenTDFError`. The package depends on
`@bufbuild/protobuf` and `@connectrpc/connect`.

```ts
import { createClient } from '@connectrpc/connect';
import { KeyAccessServerRegistryService } from '@opentdf-local/tdf3/protocol/policy/kasregistry/key_access_server_registry_pb.js';

const { attributes } = await sdk.attributes.listAttributes({ namespace: 'example.com' });
const registry = createClient(KeyAccessServerRegistryService, sdk.transport);
const json = await sdk.call('policy.namespaces.NamespaceService', 'ListNamespaces', '{}');
```

The helpers are `sdk.listAttributes(namespace?)` (protobuf-es `Attribute`s),
`sdk.getEntityAttributes(entity)`, `sdk.validateAttributes(fqns)`,
`sdk.attributeExists(fqn)` and `sdk.attributeValueExists(fqn)`;
`sdk.auth().accessToken()` returns the SDK's token and `sdk.conn()` the core
endpoint. The `coreEndpoint`, `policyEndpoint`, `authorizationEndpoint`,
`entityResolutionEndpoint` and `wellknownEndpoint` options route those
services through the SDK's transport.

```ts
const fqns = await sdk.getEntityAttributes({ id: 'e1', entityType: { case: 'clientId', value: 'my-service' } });
const core = await SDK.create(platform, { clientId, clientSecret, coreEndpoint: 'https://core.example.com' });
```

## Errors

Every failure is an `OpenTDFError` with `kind`, `op`, `message` (the
reference's wording, see [error-wording.md](error-wording.md)),
`httpStatus`, `connectCode`, `serviceMessage`, `kasUrl`,
`requiredObligations` and `assertionId`. Boundary failures keep the
runtime's kind: `canceled` (an aborted signal), `deadline_exceeded`,
`closed` (a call after `close()`), `invalid_argument` (a wrong value type),
`poisoned`, `host_fault`. Switch on `kind`; the kinds are the `ErrorKind`
union of [api-spec.md](api-spec.md).

## Deprecated façade

`encrypt(config, payload, options?, call?)`, `decrypt(config, archive, call?)`
and `TDFError` remain with their previous shapes for one release, implemented
in the shared source as thin wrappers that construct an SDK per call
(`Config.IssuerURL` becomes the provided platform configuration,
`Config.KASURL` the KAS information and the allowlist, `Config.DPoP` the DPoP
algorithm). `TDFError.code` is now the `Error.kind` string;
`PolicyBase64`, `Dissem` and `SegmentHashAlgorithm` other than `GMAC` are no
longer supported, and an empty `Metadata` writes no `encryptedMetadata`
entry, as the reference does.

## CLI

`src/hosts/typescript/cmd/otdfgc/otdfgc.mjs` is the native Node CLI over
this package, built by `scripts/build-typescript-cli.sh <package> <launcher>`
and driven by the xtest harness as the `gcts` SDK type (see
[xtest.md](xtest.md)). It implements the same contract as the Go `otdfgc`:
`encrypt`, `decrypt`, `inspect` (the manifest read natively from the ZIP),
`supports <feature>` and `--version --json` with the same flags, exit codes
(0, 1 operation failed, 2 usage or `unsupported in this build: <flag>`) and
otdfctl's `Failed to encrypt:` / `Failed to decrypt file:` / `Failed to
connect to the platform.` / `Failed to authenticate with flag-provided client
credentials.` wording.
