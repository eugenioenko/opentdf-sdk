---
title: Errors, cancellation, and cleanup
description: Handle stable error kinds and release native resources.
sidebar:
  order: 3
---

## Structured errors

Switch on stable error kinds instead of matching message text. Categories cover
invalid input, unsupported features, authentication, transport, KAS denials,
integrity, assertions, cancellation, and deadlines. The [API reference](../../api/)
describes each kind. Native adapters can also report invalid arguments and
closed handles.

Service-supplied diagnostic messages may contain sensitive information. Choose
what to expose in application logs.

Platform calls also expose Connect service codes such as `not_found`. The
[missing-attribute example](../platform-services/#handle-a-missing-attribute)
shows how to handle that code in all eight languages without hiding other errors.

## Cancellation

Use your language's cancellation API, such as `AbortSignal`, a Go context, or
a C# cancellation token. Cancellation is cooperative and must complete resource
cleanup. Each language guide describes its operation API.

## Ownership

Finish outstanding calls before closing their owners. Close reader and result
handles before the SDK. Use context managers, disposal, or `finally` where
available.

Some shared-source results are plain values while native adapters expose owned
handles. Follow the native guide for ownership instead of inferring it from a
Go signature.

Reading a manifest or policy does not verify payload authenticity. Successful
decryption performs the required integrity and enabled assertion checks.
