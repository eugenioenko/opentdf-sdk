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

The [authentication examples](../../languages/authentication/) show client
credentials, application token providers, and DPoP in all eight languages.
The same client provides the
[platform service accessors](../platform-services/).

## Token providers

Connect your application's login and refresh flow through the native token
provider. Tokens exclude the authorization prefix. Expiry is absolute UTC Unix
seconds, not milliseconds; TypeScript represents it as `bigint`. Follow the
[provider example](../../languages/authentication/#application-token-provider)
for your adapter's callback and cancellation API.

## Discovery and DPoP

Construction normally retrieves platform and identity-provider configuration.
Supplying platform configuration bypasses that discovery.

DPoP binds tokens to a signing key. A provider must supply matching key and
confirmation information. Do not relabel a Bearer token as DPoP. See the
[local EC and DPoP test profiles](../../contributing/secure-profiles/) for
contributor examples.

## Trusted endpoints

Decryption reads KAS URLs from the archive. Keep allowlist checks enabled. Supply
trusted origins or let the platform registry provide them. TLS verification is
enabled by default; insecure settings are for controlled development services.

Token exchange, certificate exchange, custom TLS settings, and Unix sockets are
currently unsupported. Read an option's API comment before setting it.
