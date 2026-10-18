---
title: Authentication and policy
description: Configure identity, DPoP, discovery, and trusted KAS endpoints.
sidebar:
  order: 2
---

Use client credentials in a trusted service, or supply a token provider from
your application. An uncredentialed client can discover configuration, but
protected operations still need authorization. Do not embed client secrets in
browser code.

## Token providers

The TypeScript adapter accepts this shape:

```typescript
const tokenProvider = async (signal: AbortSignal) => ({
  value: await obtainAccessToken(signal),
  scheme: 'Bearer' as const,
  expiresAt: expiryUnixSeconds,
});
```

`obtainAccessToken` and `expiryUnixSeconds` are application-provided. Tokens
exclude the authorization prefix. Expiry is UTC Unix seconds, not milliseconds.
Follow your language guide for its callback and cancellation API.

## Discovery and DPoP

Construction normally retrieves platform and identity-provider configuration.
Supplying platform configuration bypasses that discovery.

DPoP binds tokens to a signing key. A provider must supply matching key and
confirmation information. Do not relabel a Bearer token as DPoP. See the
[EC and DPoP profiles](../../reference/secure-profiles/).

## Trusted endpoints

Decryption reads KAS URLs from the archive. Keep allowlist checks enabled. Supply
trusted origins or let the platform registry provide them. TLS verification is
enabled by default; insecure settings are for controlled development services.

Token exchange, certificate exchange, custom TLS settings, and Unix sockets are
currently unsupported. Read an option's API comment before setting it.
