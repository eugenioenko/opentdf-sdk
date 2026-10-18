---
title: Encrypt and decrypt
description: Reuse a client, apply policy, and decrypt through KAS.
sidebar:
  order: 1
---

Create one SDK client and reuse it. It retains discovered configuration,
authentication state, and keys. The older per-call facade is deprecated.

1. Create a client with your platform URL and authentication options.
2. Encrypt plaintext with attribute-value FQNs for your policy.
3. Store or transmit the returned archive as binary data.
4. Decrypt with an identity authorized by the platform.
5. Close reader and result handles, then close the client.

KAS authorizes access to key material. The SDK decrypts the payload locally.

## TypeScript example

After installing the [TypeScript package](../../languages/typescript/):

```typescript
import { SDK } from '@opentdf-local/tdf3';

const sdk = await SDK.create(platformUrl, { tokenProvider });
try {
  const plaintext = new TextEncoder().encode('A confidential note');
  const ciphertext = await sdk.encryptBytes(plaintext, {
    attributes: ['https://example.com/attr/classification/value/private'],
  });
  const restored = await sdk.decryptBytes(ciphertext, {
    kasAllowlist: ['https://kas.example.com'],
  });
  console.log(new TextDecoder().decode(restored));
} finally {
  await sdk.close();
}
```

The platform, attribute, and KAS origin must match your deployment. See
[authentication](../authentication/) for `tokenProvider`. Other language guides
show their native equivalents.

## Important options

- Attributes define policy. Default autoconfiguration resolves grants and KAS keys.
- Metadata is encrypted separately from payload bytes.
- Reader allowlists constrain KAS origins an archive can contact.
- Assertions are verified by default. Explicit signing keys need matching verification keys.

The [API reference](../../api/) defines each field. Check
[compatibility limits](../../reference/compatibility/) before choosing a profile.
Payload APIs currently buffer data in memory; stream-shaped signatures do not
promise bounded-memory streaming.
