# Generated library acceptance requirements

This defines the Phase 4 library boundary required by the current shared SDK.
It is an acceptance checklist, not an implemented API or completion claim.
The user narrowed delivery to interoperable TDF3 SDKs on 2026-10-02. Phases 5–7 and the [delivery checklist](delivery-checklist.md) remain required. General platform services and full reference API parity are outside this boundary.

## Current source and compiler evidence

The SDK is stateful. [Config](../client.go) includes route arrays, native key
pointers and a token-provider function. `New` returns a `*Client` and can call
possibly suspending crypto even though it performs no network requests.
`Create`, `Decrypt` and `PublicKey` take contexts and may suspend; `Close` is
idempotent, cancels in-flight requests and preserves caller-owned keys.
[AccessToken](../auth.go) includes scheme, expiry and binding attestation.
[EncryptConfig and Decrypted](../tdf/engine.go) contain bytes, strings, arrays,
key handles and structured manifest data. [Error](../errors.go) preserves
stable categories, operation, service fields, obligations and a cause.

The [driver](../../goalchemy/internal/driver/emit.go) gates library emission
for Rust with `GCE006`. Go, TypeScript, Java, C# and Python value-library
emission is accepted; terminal acceptance is recorded in
[the delivery ledger](delivery-checklist.md). The existing
[C library emitter](../../goalchemy/internal/emit/c/c.go) rejects suspending
exports and only supports a limited scalar/string boundary. Neither this C API
nor a generated executable satisfies the SDK library requirement.

The [generated Go library boundary](generated-go-library.md) is accepted with real KAS RSA/P256/Bearer/enforced-DPoP and negative evidence. It adds serialized, cancellable calls and fresh source
initialization over the accepted [Go host lifecycle](../../goalchemy/docs/host-operations.md).
The [generated TypeScript library](generated-typescript-library.md) is accepted
for Node and an actual browser, with copied value boundaries and real KAS
evidence. The [generated Java library](generated-java-library.md) is accepted
with an independent named-package JAR consumer, copied byte/value/error boundaries,
native crypto/HTTP and real KAS profile/negative evidence. The
[generated C# library](generated-csharp-library.md) is accepted with a .NET 8
class library, built-in crypto/HTTP, native token providers, owned cancellable
Tasks and 93 boundary/lifecycle checks plus real-KAS matrices. The
[generated Python library](generated-python-library.md) is accepted with an
installable wheel, maintained crypto, bounded HTTP and owned cancellable
sync/async operations. The remaining Rust and C boundaries and final package/CI matrix
follow in Phases 6–7; terminal acceptance is recorded in
[the delivery ledger](delivery-checklist.md).

## Required boundary behavior

Each generated package needs an importable public API and accurate host types.
It must expose TDF3 encryption/decryption and the supported configuration,
metadata/result and error fields, and execute the shared SDK implementation.
A minimal façade may construct and close shared clients internally rather than
export every source type or method. Persistent client construction, PublicKey
and Close are optional public surfaces; if exposed, their documented ownership
and lifetime must work. Passing every call through an external executable does
not satisfy library acceptance.

Host byte inputs must be snapshotted before asynchronous submission. Results
must be independently owned, preserve empty/binary payloads and exact metadata,
and remain valid after later calls or Close. Distinguish absent versus explicitly
empty configuration where the shared API does, including IncludeMetadata and
HasSegmentSize. Document string conversion separately from arbitrary Go byte
strings; preserve integer ranges without silent host-number rounding.

Native key handles need explicit ownership and instance association. Pointer
aliases share lifetime; keys must never become copied source struct values.
The public façade may instead accept PEM configuration and own its imported
handles for each call; exposing native key handles is optional. Such imports
must close on success, failure and cancellation while preserving explicit KAS
key/kid configuration and token-provider/DPoP key matching.
Supplied AuthKey/KASPublicKey stay caller-owned, discovered PublicKey belongs to
the caller, and client-owned keys close with the client. Closing rejects new
acquisitions while an acquired operation snapshot remains valid through cleanup.
Browser configuration must support a host token provider and a matching DPoP
auth key without requiring client credentials in the browser.

Token-provider callbacks need a declared asynchronous bridge, preserving
cancellation and structured AccessToken/error results. Host callbacks never
drive source frames directly. Callback completion, rejection, duplicate/late
settlement and cancellation follow the owned host-operation lifecycle. Avoid
classifying every host exception as a recoverable source panic.

A library runtime must own initialization, scheduler, contexts, task roots and
native registrations. Initialize packages once per runtime; keep mutable caller
configuration, tokens and keys independent across calls/clients. Define overlapping
call behavior: safe serialization is acceptable, with cancellation while queued
and active; undocumented corruption of an active owner is not. If persistent
clients are exposed, they survive separate host calls and closing one must not
close another. Runtime retirement drops all registrations and background roots
after cleanup. Multiple runtime instances, if supported, must isolate mutable
globals and panic/completion state. Per-instance copies of every source global
and arbitrary exported source types are not a TDF3 delivery requirement.

Keep declared SDK failures separate from unrecovered source panic and adapter
implementation fault. Return typed SDK fields and a documented cancellation
category/cause without partial plaintext. Close may expose an asynchronous host
cleanup acknowledgement while preserving the source operation's idempotence;
the host must know when pending native resources have been released.

## Required consumer evidence

Use independent native consumers importing the built output, with no compiler
internals or executable-output scraping. Build/package delivery belongs to each
target phase; compiler library tests must nevertheless exercise real generated
exports, not only a handwritten runtime wrapper.

| Target | Consumer and boundary evidence |
| --- | --- |
| Go | Import generated package; contexts, byte slices, typed errors, repeated calls and independent caller state. |
| TypeScript | Import emitted JavaScript and declarations from Node and an actual browser; Promise operations, Uint8Array, cancellation and async provider; no Node imports in browser dependency graph. |
| Java | Import built classes/artifact; byte arrays, typed configuration/results/errors and cancelable async operations. |
| C# | Reference generated class library; byte arrays, typed configuration/results/errors and cancelable Tasks. |
| Python | Import generated module/package; bytes conversion, typed result/error behavior and documented async/cancellation entry. |
| Rust | Consume generated Cargo library; owned bytes, Result/future behavior and clear key/instance lifetimes. |
| C | Compile/link native consumer against generated headers/library; lengths, handles, drive/wake/cancel interface and explicit output/error/key releases. |

Compiler acceptance must prove actual encryption/decryption export construction
and repeated calls, supported configuration/results, copied input/output ownership,
async callbacks, the documented overlapping-call policy, independent caller state,
initialization behavior, source panic versus host fault, cleanup/shutdown and
rejection of stale completions. Optional persistent-client/multiple-instance
surfaces need their own lifetime/isolation checks. Exercise
aliases and suspended task roots under the applicable collector/sanitizers.
Retain all existing language, byte, runtime and unavailable-capability tests.

SDK acceptance adds real KAS interoperability in both directions against both
pinned references, with exact payload/metadata comparison and the required
RSA/EC, Bearer/DPoP and negative cases. Health checks, toy bytes-in/bytes-out
exports and native Go success are useful prerequisites; they do not prove
generated TDF3 SDK library acceptance across seven targets.
