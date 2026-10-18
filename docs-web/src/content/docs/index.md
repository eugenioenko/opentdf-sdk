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
- Browse [types, functions, and methods](./api/) from the implemented source.

## Two API layers

The **language guides** describe the native adapters applications import,
including installation, idiomatic names, byte containers, and cancellation.

The **generated reference** describes the shared Go implementation and the
compiler's native declarations beneath those adapters. Their names can differ
from the package API you consume.

For agents, use [llms.txt](./llms.txt), the [JSON catalog](./api.json), or the
[portable Markdown reference](./reference/index.md).
