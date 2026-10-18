---
title: OpenTDF SDK
description: TDF3 encryption and decryption through native APIs backed by one shared implementation.
template: splash
hero:
  title: OpenTDF across languages
  tagline: Encrypt once. Enforce access through your OpenTDF platform. Consume a native SDK in your language.
  actions:
    - text: Get started
      link: ./getting-started/
      icon: right-arrow
    - text: Browse the API
      link: ./api/
      variant: secondary
---

One shared Go implementation provides TDF3 encryption and decryption for **Go,
TypeScript, Python, Java, C#, Rust, C, and Swift**. TypeScript supports Node and
browsers. Swift is experimental, with a Linux baseline.

This project is a proof of concept. Package names are development identities,
and full OpenTDF SDK parity is outside the verified scope.

## Start here

- [Build and install](./getting-started/) the SDK for your language.
- [Encrypt and decrypt](./guides/encrypt-and-decrypt/) with a persistent client.
- [Configure authentication](./guides/authentication/) and [handle failures](./guides/errors-and-lifetimes/).
- [Authenticate in your language](./languages/authentication/) with client credentials, an application token provider, or DPoP.
- [Call platform services](./guides/platform-services/) with examples in every language.
- Browse the [SDK API Reference](./api/) for types, functions, and methods.
- Explore the [RPC API Reference](./api/rpc/) by service, with nested request and response fields and enums.

## Find the API you need

The **language guides** describe the native adapters applications import,
including installation, idiomatic names, byte containers, and cancellation.

The **SDK reference** describes the shared Go implementation and the
compiler's native declarations beneath those adapters. Their names can differ
from the package API you consume.

The **RPC reference** describes the platform services exposed by the SDK's
native clients. Browse their actual request and response messages, then switch
language tabs for native method declarations and usage examples.

For agents, use [llms.txt](./llms.txt), the [JSON catalog](./api.json), or the
[portable Markdown reference](./reference/index.md).
