# Full OpenTDF SDK plan

Goal: make the Goalchemy-generated OpenTDF SDK fully compatible with the
reference Go SDK in `../platform/sdk`, on every Goalchemy target, and prove it
with the OpenTDF cross-client test suite in `../tests/xtest`.

"Fully compatible" means three things, checked in this order:

1. **Wire and format compatibility.** Every xtest cell that passes for the
   reference `go` SDK passes for each generated SDK, in both the encrypt and
   decrypt direction, against the same platform. A skipped cell counts as a
   failure unless this plan lists the feature as not planned.
2. **API compatibility.** Every exported symbol of `platform/sdk` (root
   package) has an equivalent in the shared source and in each target's
   generated package, expressed in that target's idiom. The
   [parity matrix](parity-matrix.md) tracks each symbol as done, planned
   (phase N) or not planned, with a reason.
3. **Behavioural compatibility.** Error wording that tests or users depend on,
   defaults (segment size, algorithms, allowlist handling), and limits match
   the reference. Where the reference has a documented bug fix pending, follow
   the fixed behaviour and record it.

This plan supersedes the narrowed TDF3-only goal in [plan.md](plan.md). It
keeps the delivery discipline of that plan: a phase is done only when its
verification items pass, acceptance is evidence against real KAS, and a
self-round-trip never proves interoperability.

## References and pins

| Repository | Path | Role |
| --- | --- | --- |
| Reference SDK | `../platform/sdk` (module `github.com/opentdf/platform/sdk`, `Version = "0.34.0"`, `TDFSpecVersion = "4.3.0"`) | The API and behaviour to match. Pin its platform revision in `references.lock.json`. |
| Reference CLI | `../platform/otdfctl` | What xtest drives for the `go` SDK; its flags define the CLI contract below. |
| Web SDK | `../web-sdk` | Second interop reference; its CLI is xtest's `js` SDK. |
| Cross-client tests | `../tests/xtest` with `../tests/otdf-sdk-mgr` and `../tests/otdf-local` | The acceptance suite. Changes there are kept minimal and upstreamable. |
| Compiler | `../goalchemy` | Pin v0.6.0 (handles, host streams, `std/io`, `std/http`, JSON streaming, HTTP transport options) in `references.lock.json`; later releases are pinned in dedicated bump PRs. Compiler and runtime gaps are fixed in Goalchemy first, released, then pinned; never patched around in the SDK. |

Record the exact revisions when Phase 0 starts and update them only in
dedicated "pin bump" PRs.

## Targets

Go, TypeScript (Node and browser), Python, Java, C#, Rust and C are required.
Swift is a Goalchemy target since v0.5.1. Every phase keeps the shared source
compiling and the Swift package building, but Swift is excluded from
per-phase xtest gating because its builds are slow; it joins the full matrix
in Phase 11. xtest evidence is gathered per target in the order Go,
TypeScript, Python, Java, C#, Rust, C, so that the fast targets expose
shared-source bugs before the slow builds run.

The API is specified in [api-spec.md](api-spec.md). Every phase implements
part of that document and changes it only through its change-control rule.

## Not planned

These reference features are out of scope. The SDK must fail clearly, with a
stable error, when a caller or a file requires one of them.

| Feature | Reason |
| --- | --- |
| Post-quantum KAOs: X-Wing, secp+ML-KEM hybrids, pure ML-KEM (`mechanism-xwing`, `mechanism-secpmlkem`, `mechanism-mlkem`) | Goalchemy will not add post-quantum primitives (goalchemy #21, closed). The reader rejects such KAOs with `unsupported key access type`; the xtest shim answers no to these features. `test_pqc.py` gates only the encrypt SDK, so Phase 1 adds a decrypt-side gate in `tdfs.py` (upstreamable) so that reference-writer, gc-reader PQ cells skip instead of fail. |
| `WithIPC` in-process connection | Needs an in-process Go server; impossible from a generated library. `WithIPC` maps to the Unix-socket transport on targets that have it, and reports unsupported on Java and in browsers. |
| Windows named pipes, plaintext HTTP/2 | Not available in the standard libraries of all targets. |
| `WithExtraClientOptions` (Connect client options) | Go-specific type. Equivalent configuration is exposed through the SDK's own transport options. |
| `codegen/` and `audit/` packages | Build tooling and service-side helpers, not client API. |
| NanoTDF | Not in `platform/sdk` at the pinned revision. |

Everything else in the reference root package, `auth`, `auth/oauth`,
`experimental/tdf`, `experimental/tdf/keysplit`, `httputil` and `sdkconnect` is
in scope; the parity matrix decides the phase.

## Architecture decisions

These are fixed before Phase 1 and recorded here so workers do not relitigate
them.

1. **Handles replace the per-call façade.** Goalchemy #22 added exported
   handles, instances and host streams. The public API becomes a persistent
   `*SDK` handle created by `New`, with `*Reader`, `*TDFObject`,
   `*BulkDecryptPrepared` and `*ChunkedWriter` as derived handles. The current
   `library.Encrypt`/`library.Decrypt` façade stays as a thin convenience layer
   over the handle API for one release, then is removed.
2. **Options are structs in the shared source.** Goalchemy has no generics and
   functional options translate badly to most targets. The shared source takes
   `Options`, `TDFOptions`, `ReaderOptions` and `BulkOptions` structs with
   "has" flags where the reference distinguishes unset from zero. The generated
   Go package gets a hand-written `options.go` in its host adapter that provides
   the reference's `With*` functional options over those structs, so Go
   consumers can port from the reference with minimal edits. Other targets use
   builders or keyword arguments as their idiom dictates. The
   [API mapping](#api-mapping) table fixes the shape per target.
3. **Streams are host streams.** `CreateTDF(writer, reader)`, `Reader.Read`,
   `WriteTo`, `ReadAt`, `Seek`, `DecryptTo` and `DecryptFile` use Goalchemy
   `std/io` host streams in both directions. Byte helpers (`DecryptBytes`,
   `EncryptBytes`) stay for convenience and are implemented over the streams.
4. **Platform services are generated.** Policy, authorization, well-known,
   KAS registry, entity resolution and KAS service clients are generated from
   `../platform/protocol` with `protoc-gen-goalchemy` and `std/connect`,
   exactly like the reference's `sdkconnect` wrappers. No hand-written JSON
   for service calls remains after Phase 3.
5. **Gaps are fixed in Goalchemy, not worked around.** The reference imports
   several packages Goalchemy lacks. Each gets a Goalchemy issue in Phase 0 and
   is delivered before the phase that needs it:
   - `net/url`: URL parsing and normalisation (resource locators, KAS URL
     origin stripping, allowlist comparison). Needed by Phase 2.
   - `regexp`: the reference uses it for FQN and URL validation. Prefer
     hand-written parsers in the shared source; add `std/regexp` only if a
     parser becomes unreadable.
   - JSON canonicalisation (`gowebpki/jcs`, RFC 8785) for assertion hashes.
     Pure logic; implement as `std/encoding/json/jcs` or in the SDK. Phase 5.
   - `os`: temp file plus rename for `DecryptFile`, `os.Open`-style streaming
     file access. Phase 7.
   - JSON schema validation (`gojsonschema`) for `WithSchemaValidation`: a
     bounded validator for the manifest schema, not a general engine. Phase 6.
   - `crypto/x509`, `encoding/pem` for client certificates: only what the
     cert-exchange and mTLS options need; `lib/crypto` already imports PEM
     keys and `lib/http.DoWith` takes PEM certificates. Phase 8.
   - `math/big`, `encoding/asn1`: avoid; the native crypto layer owns key
     formats.
   - `golang.org/x/oauth2`, `jwx`: already replaced by subset Go in `src/`
     (JWT signing, JWK thumbprints); extend rather than import.
6. **Supported features are advertised from the source.** `SupportedFeatures()`
   returns exactly the xtest feature names the SDK passes, and the xtest shim
   reads it. A feature is added to that list only in the PR whose xtest cells
   prove it.
7. **One CLI contract for all targets.** Each target ships a small native CLI
   (`otdfgc`) that consumes its generated package and implements the
   [CLI contract](#cli-contract). It is both the xtest driver and the
   independent native consumer required by the delivery checklist.

## API mapping

The shared source defines the API once. The table fixes each target's idiom
for the two handle types that appear everywhere; derived handles follow the
same pattern.

| Target | `SDK` handle | `Reader` handle | Streams | Options | Errors |
| --- | --- | --- | --- | --- | --- |
| Go | `*sdk.SDK`, methods take `context.Context`, `Close() error`; `With*` functional options in the host adapter | `*sdk.Reader` implementing `io.Reader`, `io.ReaderAt`, `io.Seeker`, `io.WriterTo` | standard `io` | `sdk.Options` struct plus `With*` | `*sdk.Error` with `Kind`, `Op`, `HTTPStatus`, `RequiredObligations`; `errors.Is` sentinels mirror `tdferrors.go` |
| TypeScript | `class SDK` with Promise methods, `close()`, `Symbol.asyncDispose` | `class Reader` with `read`, `readAt`, `seek`, `stream()` (`ReadableStream`), async iteration | Node `Readable`/`Writable`, web streams, `Uint8Array`, `Blob` | plain object with optional fields | `SDKError` class with the same fields |
| Python | `class SDK` with sync methods and `async` twins, context manager | file-like `Reader` | file-like objects and `bytes` | keyword arguments | `SDKError` exception |
| Java | `SDK implements AutoCloseable`, blocking plus `*Async` | `Reader` with `InputStream` view and `readAt` | `InputStream`/`OutputStream`, byte channels | builder | `SDKException` |
| C# | `SDK : IAsyncDisposable`, `Task` methods with `CancellationToken` | `Reader` exposing a `System.IO.Stream` | `Stream` | options record | `SdkException` |
| Rust | `Sdk` with `Result` methods, `close()` | `Reader` implementing `Read`, `Seek` | `Read`/`Write` | options struct with `Default` | `SdkError` enum |
| C | `otdf_sdk*` with `otdf_sdk_new`/`_close`, every call returns a status | `otdf_reader*` with read/read_at/seek/close | callback streams (`gxc_stream`) | options struct with explicit lengths | status code plus `otdf_last_error` |
| Swift | `final class SDK` with `async throws` methods | `Reader` with `AsyncSequence<Data>` and `readAt` | `GoalchemyStream` | struct with defaults | `SDKError` enum |

Method names keep the reference's names in each target's casing
(`CreateTDF` → `createTdf`, `create_tdf`, `createTDF` as the target's style
guide says). Field names of `Manifest`, `KeyAccess`, `Assertion` and the
other manifest types keep the reference's JSON tags.

## CLI contract

Every target's `otdfgc` CLI accepts the same arguments, so one `cli.sh` serves
all targets in xtest. The flags are the subset of `otdfctl` that xtest's
`sdk/go/cli.sh` uses, plus the DPoP flags that only the `js` shim maps
(`XT_WITH_DPOP`, `XT_WITH_DPOP_KEY`; `test_dpop.py` also overrides `CLIENTID`),
plus `supports`. The shim reads both `XT_WITH_KAS_ALLOWLIST` (what `tdfs.py`
sets) and `XT_WITH_KAS_ALLOW_LIST` (what the upstream shims read).

```
otdfgc encrypt <in> --out <file> --host <url> --with-client-creds <json>
    [--attr <fqn,...>] [--mime-type <t>] [--with-assertions <file|json>]
    [--wrapping-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]
    [--target-mode 4.2.2|4.3.0] [--policy-mode encrypted|plaintext]
    [--ecdsa-binding] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
otdfgc decrypt <in> --out <file> --host <url> --with-client-creds <json>
    [--with-assertion-verification-keys <file>] [--no-verify-assertions]
    [--session-key-algorithm rsa:2048|ec:secp256r1] [--kas-allowlist <url,...|*>]
    [--dpop ...] [--dpop-key ...] [--tls-no-verify]
otdfgc inspect <in>                  # prints the manifest as JSON
otdfgc supports <feature>            # exit 0 if SupportedFeatures() lists it
otdfgc --version --json              # {"sdk_version","schema_version","supported_features","target"}
```

Exit codes: 0 success, 1 operation failed (message on stderr, matching the
reference wording where tests grep it), 2 usage error. Phase 1 writes the exact
stderr strings the tests assert on into `docs/error-wording.md`, collected by
reading every `expected_patterns`, `rewrap_403_pattern` and `assert ... in`
check in `../tests/xtest/test_*.py`.

## xtest integration

The tests repository identifies an SDK by `sdk/<type>/dist/<version>/cli.sh`.
Register one type per target so that versions mean SDK versions and the
matrix can compare them:

| xtest type | Target |
| --- | --- |
| `gcgo` | Go |
| `gcts` | TypeScript (Node) |
| `gcpy` | Python |
| `gcjava` | Java |
| `gccs` | C# |
| `gcrs` | Rust |
| `gcc` | C |
| `gcswift` | Swift |

The harness lives in a repository the user controls,
`eugenioenko/opentdf-sdk-tests`: a single-commit snapshot of the `opentdf/tests`
tree (no upstream history; GitHub refuses a fork under this account's SAML
binding and push protection rejects the upstream history). `../tests` is that
repository's clone. To take upstream changes, add `opentdf/tests` as a remote
and rebase the harness patches onto it. Changes there are kept small:

- `xtest/tdfs.py`: extend `sdk_type`; add `("autoconfigure" | "ns_grants" | "multikao" | "better-messages-2024", gc*)` cases only once those features pass; everything else is answered by `cli.sh supports`.
- `xtest/sdk/<type>/{Makefile,cli.sh}`: `Makefile` builds `dist/<version>/otdfgc` from `src/<version>` (a checkout or symlink of this repository) using `scripts/build-generated-<target>.sh` and the target's CLI build; `cli.sh` is the shared shim that forwards to `otdfgc` and maps `XT_WITH_*` variables to flags. One shim file, copied by each Makefile.
- `xtest/sdk/Makefile`: add the eight targets.
- `otdf-sdk-mgr`: add the types to `schema.py`, `config.py` (git URL of this repository, source build only) and `installers.py` so `install tip <type>` and `checkout` work. Release installs are added when this repository publishes packages (Phase 11).
- `.github/workflows/xtest.yml`: not changed in this repository's PRs; this repository runs its own matrix (Phase 11).

The browser build of TypeScript cannot be driven by `cli.sh`. It is covered by
this repository's Playwright job, which runs the same encrypt and decrypt
cells through the browser bundle against the platform.

### Scoreboard

`docs/xtest-scoreboard.md` is regenerated by `scripts/xtest-scoreboard.py`
from pytest's JUnit XML: one row per test, one column per target, values
`pass`, `fail`, `skip:<reason>` or `n/a`. The reference `go` SDK's column is
the target to match. Every phase's exit criteria name the rows that must turn
green. The scoreboard is committed with each phase and attached to the
release notes.

## Implementation rules

- **Port the reference, don't reinvent it.** The reference Go SDK is the
  primary source: each shared-source file mirrors a reference file
  (`granter.go`, `key_splitter.go`, `assertion.go`, `kas_client.go`, ...)
  with the same function and type names, the same control flow and the same
  error wording, changed only where the Goalchemy subset or the API spec
  requires. A reviewer must be able to diff the two files. The web SDK is
  consulted only for browser behaviour and for cases where the two
  references disagree. Reference unit tests are ported alongside the code.
- **Compose, don't entangle.** Every collaborator the reference hides behind
  a struct field becomes an interface in the shared source with one default
  implementation: `TokenSource`, `KeyFetcher` (KAS public keys), `KeySplitter`,
  `Granter` (attribute resolution), `Rewrapper` (KAS client), `Transport`
  (HTTP exchange), `Clock`. `New` wires the defaults; tests and wrappers
  supply replacements. No package-level state: everything hangs off the
  `SDK` instance. The `Client` and `TDFReader` interfaces in the API spec are
  the public composition points; a wrapper delegates to them and may replace
  any of the internal interfaces above.
- **Small, explicit types across the boundary.** Options and results are
  plain structs; handles are the only stateful objects; no callbacks except
  the host token source and host streams.

## Working rules

- One worker at a time, one phase per worker task, one PR per phase part.
  Branch from the latest `main`, rebase before opening the PR, merge with a
  merge commit when CI passes. Workers report and stop; the orchestrator
  reviews and merges.
- To learn how an existing test behaves, read it and read its CI history in
  `../tests` (`gh run list -w xtest.yml`, job logs). Do not run the existing
  xtest or reference suites locally to find out what they do. Run locally
  only the cells this phase targets, with `--sdks "<gc type>"` and explicit
  test ids, against the local platform from `otdf-local`.
- Local builds and tests are throttled: `systemd-run --user --scope -p MemoryMax=10G -p CPUQuota=800% --`, `GOMAXPROCS=4`, `go test -p 2`, one target at a time. Full matrices run in CI.
- Goalchemy changes go to `../goalchemy` as their own PRs with their own CI,
  are released, and the release is pinned here. No third-party dependencies in
  Goalchemy; targets use their standard libraries or the native dependencies
  already recorded in their `target.yaml`.
- Shared source stays in the Goalchemy subset: no generics, no floats in
  protocol code, no reflection-dependent calls. `goalchemy check` runs in CI
  on every PR.
- Generated distributions under `dist/` are regenerated only at the end of a
  phase, in a separate "regenerate distributions" PR, with
  `scripts/dist-distributions.py check` proving idempotence.
- Tests per phase, in this order: shared-source unit tests in `src/` that
  compare with the reference (ported reference tests where they exist), the
  phase's xtest cells run locally for Go and TypeScript against `otdf-local`,
  then the phase's PR CI. PR CI runs the offline checks and the Go and
  TypeScript real-KAS jobs; the full seven-target matrix runs on `main` after
  merge and on manual dispatch. A phase is not blocked on the full matrix,
  but a red `main` is fixed before the next phase starts.
- Every phase ends with: scoreboard updated, parity matrix updated, docs for
  the new API in `docs/generated-<target>-library.md`, and the progress log
  entry in `docs/progress.md`.
- Nothing is published to a package registry until Phase 11.

## Phases

Each phase lists its goal, the reference symbols it covers, the tasks, and the
verification that makes it done. Phases are sequential; a phase may be split
into several PRs where noted.

### Phase 0: Scope lock and gap inventory

Goal: fixed references, a complete parity matrix, and Goalchemy issues for
every missing capability.

Tasks:
1. Bump `references.lock.json` to the current platform, web-sdk and
   Goalchemy v0.6.0; regenerate `docs/reference-api.json` and
   `docs/reference-subpackages.json` with `scripts/reference-api.py`.
2. Write `docs/parity-matrix.md`: every root symbol from `reference-api.json`
   and every exported symbol of `auth`, `auth/oauth`, `experimental/tdf`,
   `experimental/tdf/keysplit`, `httputil` and `sdkconnect`, with
   status `done`, `phase N` or `not planned: <reason>`.
3. Inventory the reference's non-OpenTDF imports (`go list -f '{{join .Imports "\n"}}'` on `sdk` and `lib/ocrypto`) and map each to a Goalchemy package, a subset-Go rewrite, or a not-planned entry. Open the Goalchemy issues listed under architecture decision 5, each with the phase that needs it.
4. Read `../tests/xtest` end to end and write `docs/xtest-coverage.md`: every test function, the features it gates on, the `XT_WITH_*` inputs it uses, the error patterns it asserts, and the phase that will make it pass.
5. Write `docs/error-wording.md` from step 4.
6. Confirm the current CI is green on `main` and record the baseline scoreboard as "all cells n/a".

Verification:
- `parity-matrix.md` has a row for every symbol in the regenerated indices and no row without a status.
- Every `xtest` test function appears in `xtest-coverage.md` with a phase.
- Goalchemy issues exist and are linked from `parity-matrix.md`.

### Phase 1: xtest harness and the Go CLI

Goal: the existing TDF3 façade runs under xtest as `gcgo`, and the scoreboard
exists.

Tasks:
1. In `../tests` (fork branch): add the eight SDK types as described under
   [xtest integration](#xtest-integration); only `gcgo` has a working
   Makefile in this phase.
2. Add `hosts/go/cmd/otdfgc`, a native Go CLI built on the generated Go
   package, implementing the [CLI contract](#cli-contract) with the flags
   the current façade can honour (`encrypt`, `decrypt`, `inspect`,
   `supports`, `--version`). The CLI is native per target; nothing of it
   lives in the shared source.
3. Add `SupportedFeatures()` and `Version` to the shared source; the CLI reads them.
4. Add `scripts/xtest-scoreboard.py` and `scripts/xtest-run.sh` (starts
   `otdf-local`, builds the requested gc types, runs a pytest selection with
   JUnit output, regenerates the scoreboard).
5. Run `test_tdfs.py::test_tdf_roundtrip` and `test_manifest_validity` for
   `--sdks "go gcgo"` in both directions; fix façade bugs the harness finds.

Verification:
- `test_tdf_roundtrip[ztdf]` passes for `gcgo→gcgo`, `gcgo→go`, `go→gcgo`, `gcgo→js`, `js→gcgo` at size `small`.
- `test_manifest_validity` passes for `gcgo`.
- The scoreboard shows every other cell as `skip` or `fail` with a reason, not `n/a`.
- `cli.sh supports <feature>` answers from `SupportedFeatures()` for every feature name in `tdfs.py`, and unknown names exit 2.

### Phase 2: Handle-based core API and platform discovery

Goal: `New(endpoint, options)` with discovery, the `SDK` and `Reader` handles,
and byte-level `CreateTDF`/`LoadTDF`/`DecryptBytes` on all targets.

Reference symbols: `New`, `SDK.Close`, `SDK.CreateTDF` and `CreateTDFContext`
(over bytes in this phase, via `EncryptBytes`; streams come in Phase 7),
`LoadTDF`, the `Manifest`, `KeyAccess`, `PolicyObject` and related manifest
types from `manifest.go` (needed by `Reader.Manifest`), `Reader.Init`, `Reader.Manifest`, `Reader.Policy`,
`Reader.DataAttributes`, `Reader.UnencryptedMetadata`, `DecryptBytes`,
`TDFObject.Manifest`, `TDFObject.Size`, `WithClientCredentials`,
`WithTokenEndpoint`, `WithPlatformConfiguration`, `PlatformConfiguration`
and its methods, `WithInsecurePlaintextConn`, `WithInsecureSkipVerifyConn`,
`WithLogger` (host `slog` sink wiring; options such as level come with the
auth work in Phase 8), `WithConnectionValidation`, `IsHealthy`, `WithKasInformation`,
`KASInfo`, `WithMimeType`, `WithMetaData`, `WithSegmentSize`,
`WithWrappingKeyAlg` (RSA-2048, P-256), `WithSessionKeyType`,
`WithKasAllowlist`, `WithIgnoreAllowlist`, `AllowList`, `WithMaxManifestSize`,
`GetBaseKey`, `StoreKASKeys`, `KASKeyFetcher`, `IsValidTdf`, `GetTdfType`,
`TdfType`, `Error`, `tdferrors.go` and `decrypterrors.go` sentinels,
`basekeyerrors.go`.

Tasks (three PRs: shared source and Go; TypeScript, Python; Java, C#, Rust, C, Swift):
1. Implement `std/net/url` in Goalchemy (issue from Phase 0) and pin the release.
2. Restructure `src/`: `sdk.go` (handle, options, discovery), `tdf.go`,
   `reader.go`, `kas_client.go`, `allowlist.go`, `basekey.go`,
   `platformconfig.go`, `errors.go`, mirroring the reference file layout so
   future ports are file-by-file diffs.
3. Discovery on `New`: fetch `/.well-known/opentdf-configuration` unless
   `WithPlatformConfiguration` is given; derive issuer, token endpoint and
   base key; fetch the KAS allowlist from the registry on first decrypt unless
   `WithKasAllowlist`/`WithIgnoreAllowlist` is set; cache KAS public keys per
   URL, algorithm and kid with the reference's staleness rules.
4. Export `*SDK`, `*Reader`, `*TDFObject` as Goalchemy handles; add the Go
   `With*` adapter in `hosts/go`.
5. Error model: structured `Error` with the reference's sentinel values and
   wording from `docs/error-wording.md`.
6. Update each target's CLI to the new API (`--host` plus credentials is
   enough; no `KASURL` in configuration).
7. Keep the old façade as a wrapper; mark it deprecated in docs.

Verification:
- `test_tdf_roundtrip`, `test_manifest_validity`, `test_tdf_with_unbound_policy`, `test_tdf_with_altered_policy_binding`, `test_tdf_with_altered_root_sig`, `test_tdf_with_altered_seg_sig_wrong`, `test_tdf_with_altered_enc_seg_size`, `test_tdf_altered_payload_end`, `test_tdf_with_malicious_kao` pass for every gc type in both directions against `go` and `js`.
- `test_policytypes.py` (`or`, `and`, `hierarchy`) passes for every gc type with explicit attributes.
- Unit tests in `src/` compare discovery, allowlist normalisation and key caching with the reference by running the same inputs through both.
- Native consumer tests per target construct one `SDK`, run 100 encrypt and decrypt calls, close, and check no handle or key leaks (the per-target leak checks already in `tests/interop`).

### Phase 3: Generated platform service clients

Goal: every service client the reference exposes, generated from
`../platform/protocol`.

Reference symbols: `sdkconnect.*` (16 service interfaces), `SDK.Conn`,
`ConnectRPCConnection`, `WithCustom*Connection`, `SDK.Attributes`,
`Namespaces`, `ResourceMapping`, `SubjectMapping`, `KeyAccessServerRegistry`,
`Authorization`, `AuthorizationV2`, `EntityResoution`, `EntityResolutionV2`,
`Unsafe`, `Actions`, `RegisteredResources`, `Obligations`, `KeyManagement`,
`DynamicValueMapping`, `WellKnown`, `ListAttributes`, `GetEntityAttributes`,
`ValidateAttributes`, `AttributeExists`, `AttributeValueExists`,
`SDK.Auth`, `AuthClient.AccessToken`.

Tasks:
1. Add `scripts/generate-protocol.sh`: runs `protoc-gen-goalchemy` on the
   pinned `../platform/protocol` for the services above into
   `src/protocol/`; CI checks the output is current.
2. Route every generated client through the SDK's authenticated transport
   (token source, DPoP, allowlist, timeouts). The KAS client's `Rewrap` and
   `PublicKey` move to the generated client with the protobuf JSON codec;
   remove the hand-written request JSON.
3. Expose the clients as fields or accessors on the `SDK` handle in each
   target (Goalchemy handles for the clients; request and response messages
   as generated message types).
4. Support `WithCustom*Connection` as per-service endpoint overrides.

Verification:
- `test_self.py` equivalents: a per-target consumer lists namespaces, creates an attribute, and reads it back through the generated clients.
- The existing Rewrap and PublicKey xtest cells still pass with the generated KAS client.
- Generated code is byte-identical on regeneration in CI.

### Phase 4: Autoconfigure, key splitting and key management

Goal: the reference's attribute-driven KAS selection, multi-KAO manifests and
all non-PQ wrapping mechanisms.

Reference symbols: `WithAutoconfigure`, `WithDataAttributes`,
`WithDataAttributeValues`, `AttributeNameFQN`, `AttributeValueFQN` and their
methods, `granter.go` (rule evaluation, grants on namespace, attribute and
value, key mappings), `KeySplitter`, `DefaultKeySplitter`, `Split`,
`SplitResult` and its methods, `KASPublicKey`, `WithNoKIDInKAO`,
`WithWrappingKeyAlg` for RSA-4096, P-384, P-521, `ResourceLocator` and its
methods, `KeyTypeToPolicyAlgorithm`, `PolicyAlgorithmToKeyType`,
`Reader.buildKey` semantics (XOR reconstruction, OR alternatives, duplicate
KAO tolerance), `experimental/tdf/keysplit`.

Tasks (two PRs: writer side; reader side and extended mechanisms):
1. Port `granter.go` and `key_splitter.go` file by file; the split plan and
   KAO template logic must produce the same KAO order, split ids and kids as
   the reference for the same policy (compare with a Go test that runs both
   against fixture attribute definitions).
2. Writer: multiple KAOs per manifest, base key fallback, default KAS
   resolution order (explicit `WithKasInformation`, grants, base key,
   platform default), `sid` handling, `WithNoKIDInKAO`.
3. Reader: group KAOs by split id, request all in one rewrap, reconstruct the
   DEK by XOR, tolerate duplicate KAOs (`multikao`), surface per-KAO
   failures with the reference's error wording.
4. Extended mechanisms: RSA-4096 and EC P-384/P-521 wrapping and sessions
   using `lib/crypto` (already available); reject PQ KAOs explicitly.
5. Advertise `autoconfigure`, `ns_grants`, `key_management`, `multikao`,
   `mechanism-rsa-4096`, `mechanism-ec-curves-384-521`, `attribute_traversal`.

Verification:
- All `test_abac.py` `test_autoconfigure_*` and `test_key_mapping_*` cells pass for every gc type, except the PQ mechanism cells, which skip with the not-planned reason.
- `test_tdf_with_duplicate_kao_same_kas` passes.
- `test_tdf_roundtrip[ztdf-ecwrap]` passes in both directions with `ecwrap` advertised.
- A split-plan differential test against the reference passes for the fixture policies in `../tests/xtest/fixtures/attributes.py`.

### Phase 5: Assertions

Goal: assertion creation, signing, verification and the system-metadata
assertion, with reference-identical hashes.

Reference symbols: `AssertionConfig`, `Assertion` and its methods,
`AssertionKey`, `AssertionKeyAlg`, `AssertionType`, `Scope`, `AppliesToState`,
`BindingMethod`, `Statement` (including the JSON object statement form),
`Binding`, `AssertionVerificationKeys`, `WithAssertions`,
`WithSystemMetadataAssertion`, `GetSystemMetadataAssertionConfig`,
`WithAssertionVerificationKeys`, `WithDisableAssertionVerification`,
`ErrAssertionFailure`.

Tasks:
1. JSON canonicalisation (RFC 8785) in subset Go, with the reference's test
   vectors; Goalchemy issue from Phase 0.
2. Port `assertion.go`: hash over the canonical statement, binding signature
   as JWS with HS256 (DEK-derived default key) or RS256 (caller key), the
   `assertionHash` and `assertionSig` claims, legacy hex mode.
3. Reader: verify every assertion by default, key lookup by assertion id with
   default key fallback, explicit disable, exact failure wording.
4. CLI: `--with-assertions` (file or JSON), `--with-assertion-verification-keys`, `--no-verify-assertions`; advertise `assertions` and `assertion_verification`.

Verification:
- `test_tdf_assertions_unkeyed`, `test_tdf_assertions_with_keys`, `test_tdf_assertions_422_format`, `test_manifest_validity_with_assertions`, `test_tdf_with_altered_assertion_statement`, `test_tdf_with_altered_assertion_with_keys` pass for every gc type in both directions.
- `test_legacy.py::test_decrypt_object_statement_value_json` passes.
- Hash vectors: assertions produced by the gc SDK verify in the reference Go SDK and in the web SDK, and vice versa, for HS256 and RS256.

### Phase 6: Target modes, legacy readers and hardening

Goal: 4.2.2 output, every legacy and tamper case the suite has, and the
reference's schema validation.

Reference symbols: `WithTargetMode`, `TDFSpecVersion`,
`WithSchemaValidation`, `SchemaValidationIntensity`, `RootIntegrityAlg`,
`SegmentIntegrityAlg`, `IntegrityAlgorithm`, `TDFFormat`, `WithPolicyFrom`,
`WithInputSize`.

Tasks:
1. Writer target mode `4.2.2`: hex-before-base64 integrity values, omitted
   `schemaVersion`, legacy assertion binding; advertise `hexaflexible`.
2. Reader: legacy detection by absent `schemaVersion`, hex decoding, absent
   root `alg`, segment size defaults from manifest-level defaults
   (`chunky`), ZIP64 reads, both manifest entry names with precedence.
3. Reject a GMAC root signature (`gmac_root_rejected`), unknown integrity
   algorithms, oversized manifests, malformed KAOs, with reference wording.
4. Manifest schema validation at the reference's intensities using the
   schema in `../tests/xtest/manifest.schema.json` as the test oracle.
5. `WithPolicyFrom` and `WithInputSize`.
6. Record the reference's own pending fixes (DSPX-4590, DSPX-4703) and follow
   the fixed behaviour.

Verification:
- Every `test_legacy.py` cell passes for every gc type as decrypt SDK.
- Every `test_root_signature.py` cell passes, including the forged-GMAC cases without `XT_FORCE_SUPPORTS`.
- `test_chunky_roundtrip` passes for every gc type in both directions with `js`.
- `test_tdf_spec_target_422` and `test_tdf_spec_target_430` pass.
- All remaining `test_tdfs.py` tamper cells pass.

### Phase 7: Streaming and large payloads

Goal: stream-based creation and reading with bounded memory, file helpers and
ZIP64 writing, so the `chunky`, `medium` and `large` sizes pass.

Reference symbols: `CreateTDF(writer io.Writer, reader io.Reader)`,
`CreateTDFContext`, `Reader.Read`, `Reader.ReadAt`, `Reader.Seek`,
`Reader.WriteTo`, `DecryptTo`, `DecryptFile`, `WithInputSize`,
`inputSizeUnknown`, `internal/zipstream` behaviour (ZIP32 and ZIP64 modes,
data descriptors, the 2 GiB switch), `Reader.UnsafePayloadKeyRetrieval`.

Tasks (two PRs: writer streaming; reader streaming and file helpers):
1. Goalchemy `std/os` additions (temp file, rename, open for streaming) from
   the Phase 0 issue; pin the release.
2. Port `internal/zipstream` to subset Go as a streaming writer and reader
   over `std/io`; switch to ZIP64 at 2 GiB (`zip64-at-2gib`).
3. Writer: segment the input as it arrives, unknown input size, segment
   count limits, constant memory per segment.
4. Reader: lazy payload access by segment, `ReadAt` offset mapping from
   cumulative segment sizes, `Seek`, `WriteTo` streaming decrypt, `DecryptTo`,
   `DecryptFile` with temp file and rename and no partial output on failure.
5. Host stream idioms per target as fixed in the API mapping; browser:
   `ReadableStream` in and out, uploads collected only where the platform
   cannot stream.
6. Benchmark against the reference at 1, 10, 50 MiB and 2.1 GiB with the
   existing benchmark harness; record peak memory per target.

Verification:
- `test_tdf_roundtrip` and `test_chunky_roundtrip` pass with `--sizes small,chunky,medium` for every gc type; `large` passes for Go, TypeScript, Java, C#, Rust and C (Python and Swift recorded if slower than the CI budget).
- `test_zip64.py::test_zip64_band_roundtrip` passes with `zip64-at-2gib` advertised.
- Peak memory for a 2.1 GiB round trip stays under 256 MiB on every target except where the benchmark records a documented reason.
- `tdf_readat_test.go` and `tdf_hybrid_test.go` from the reference are ported as shared-source unit tests and pass.

### Phase 8: Authentication and transport parity

Goal: every auth option and token source the reference has, and the DPoP test
lane.

Reference symbols: `WithDPoPAlgorithm`, `WithDPoPJWK`, `WithDPoPKeyPEM`,
`SigningAlgorithm`, `WithOAuthAccessTokenSource`, `WithTokenExchange`,
`WithTLSCredentials`, `WithIPC` (mapped as described), `auth.AccessToken`,
`AccessTokenSource`, `AccessTokenCredential`, `DPoPTransport`,
`NewDPoPHTTPClient`, `TokenAddingInterceptor`, `IDPAccessTokenSource`,
`OAuthAccessTokenSource`, `IDPTokenExchangeTokenSource`,
`CertExchangeTokenSource`, `NewCertExchangeTokenSource`,
`NewDPoPValidationHTTPClient`, `auth/oauth` (client credentials, token
exchange, cert exchange, DPoP nonce handling), `httputil`.

Tasks:
1. Token sources as host callbacks plus built-in client-credentials,
   token-exchange and certificate-exchange flows in subset Go over
   `lib/http.DoWith` (mTLS, custom CA, skip-verify, unix socket).
2. DPoP: algorithm selection (ES256, RS256, plus ES384/ES512/RS384/RS512 as
   the reference allows), caller-supplied JWK or PEM key, nonce challenge
   retry and cache, `ath` claim, JWK thumbprint binding, `cnf.jkt` checks.
3. Logging through `std/log/slog` with the host sink; `WithLogger`.
4. CLI: `--dpop`, `--dpop-key`, `--tls-no-verify`; advertise `dpop`,
   `dpop_nonce_challenge`, `connectrpc`.

Verification:
- `test_dpop.py` happy-path and nonce-retry cells pass for every gc type; the negative cells (which use direct HTTP) pass against the same platform.
- Per-target consumer tests exercise each token source against the local Keycloak, including a browser token-provider flow with no client secret.
- The existing EC and DPoP profile matrices in `tests/interop/profiles` still pass.

### Phase 9: Obligations and bulk decryption

Goal: obligation fulfilment and the bulk APIs.

Reference symbols: `WithFulfillableObligationFQNs`,
`WithTDFFulfillableObligationFQNs`, `Reader.Obligations`,
`RequiredObligations`, `BulkDecrypt`, `PrepareBulkDecrypt`,
`BulkDecryptPrepared.BulkDecrypt`, `BulkTDF`, `BulkDecryptRequest`,
`BulkDecryptOption` and its `With*` functions, `BulkErrors`,
`FromBulkErrors`, `WithTDFType`.

Tasks:
1. Send fulfillable FQNs in the signed request token; parse required
   obligations from rewrap metadata; expose `Reader.Obligations`; include
   required obligations in the decrypt error with the reference wording.
   The `required obligations: [...]` suffix the tests grep is appended by
   `otdfctl`, not the SDK (`otdfctl/cmd/handlers/tdf.go`), so `otdfgc` does
   the same.
2. Bulk: group TDFs by policy, one rewrap per group, per-TDF error
   collection, `BulkErrors` unwrapping; expose as a handle per target with a
   list of streams or byte buffers.
3. Advertise `obligations`.

Verification:
- `test_abac.py::test_obligations_*` cells pass for every gc type.
- A bulk consumer test per target decrypts 50 TDFs under three policies with two rewrap calls and reports per-item failures.
- Audit-log assertions (`test_audit_logs_integration.py`; `test_audit_logs.py` is a harness self-test) pass with the gc types as encrypt and decrypt SDKs.

### Phase 10: Remaining API parity

Goal: the parity matrix has no `phase N` rows left; every row is `done` or
`not planned`.

Reference symbols: everything still open, expected to include `ChunkedWriter`
and its options (`NewChunkedWriter`, `WriteSegment`, `Finalize`,
`GetManifest`, `WithChunked*`), `experimental/tdf` (`NewWriter`, `Reader`,
options), `IsPlatformEndpointMalformed`, `ResourceLocator` serialisation to
and from readers, `Statement.UnmarshalJSON` quirks, `Version`,
`TDFSpecVersion`, `SupportedFeatures`, `WithCustom*Connection` edge cases,
`sdkconnect` services not yet exercised.

Tasks:
1. Port the chunked writer: out-of-order segment encryption, manifest
   finalisation with a segment subset, parallel segment calls serialized by
   the instance lock (document the serialization).
2. Port `experimental/tdf` as a separate importable package per target.
3. Close every remaining matrix row; for each `not planned` row add the
   reason and the error the SDK produces.
4. Port the reference's root-package unit tests that test public behaviour
   (`tdf_test.go`, `sdk_test.go`, `granter_test.go`, `key_splitter_test.go`,
   `assertion_test.go`, `resource_locator_test.go`, `discovery_test.go`,
   `basekey_test.go`, `kas_client_test.go`, `chunked_test.go`) to shared-source
   tests, skipping only what depends on Go-only types.

Verification:
- `parity-matrix.md` has zero `phase N` rows.
- Ported reference tests pass natively and on every target through the language fixtures.
- The full xtest selection used by `../tests/.github/workflows/xtest.yml` for `go` passes for every gc type, with skips only for not-planned features.

### Phase 11: Packaging, documentation, CI and release

Goal: publishable packages, documentation at the level of the reference
quickstart, and a CI matrix that proves compatibility on every push.

Tasks:
1. Regenerate all distributions with the pinned Goalchemy; per-target
   `README.md` with the reference quickstart translated (`New`, `CreateTDF`,
   `LoadTDF`, policy clients), option tables generated from the shared source.
2. CI in this repository: build every target, run `goalchemy check`, unit
   tests, the Playwright browser job, and an xtest job per target that
   starts the platform with `otdf-local`, installs the gc type with
   `otdf-sdk-mgr install tip`, runs the same selection as the upstream
   workflow, and publishes the scoreboard as an artifact. Full matrices on
   `main` and manual dispatch; PRs run Go plus one rotating target.
3. `otdf-sdk-mgr` release installs for the gc types once packages are
   published.
4. Swift joins the xtest matrix; its cells are fixed or recorded.
5. Benchmarks for every target: extend `scripts/benchmark-sdk.py` and the
   per-target harnesses to the full API (byte encrypt and decrypt, streamed
   encrypt and decrypt, `ReadAt`, bulk decrypt of 50 TDFs, autoconfigured
   multi-KAS encrypt) at 1, 10, 50 MiB and 2.1 GiB, against the reference Go
   SDK and the web SDK in Node, using the steady-state policy in
   [benchmarks.md](benchmarks.md). Publish the table in the README and the
   raw results under `docs/benchmarks/`.
6. Release: version the shared source like the reference (`Version`), tag,
   publish the scoreboard, parity matrix and benchmark table in the release
   notes, and open the upstream PR in `opentdf/tests` with the harness
   changes.

Verification:
- CI green on `main` with the full matrix; scoreboard shows every reference-passing cell as `pass` for every target, PQ cells as `skip: not planned`.
- Independent native consumers build from the published packages on every target.
- Benchmarks recorded for all eight targets with validated archives.
- Documentation links and whitespace checks pass; `docs/final-delivery.md` is rewritten for the full SDK.

## Decisions taken with the user

- Swift builds in every phase but is gated only in Phase 11.
- The harness lives in `eugenioenko/opentdf-sdk-tests`, a snapshot of `opentdf/tests`; eight SDK types.
- `experimental/tdf` and the chunked writer stay in Phase 10.
- One reference pin (`platform/sdk` 0.34.0) for the whole plan, bumped in Phase 11.
- Package names and publication identities are chosen in Phase 11 with the user.
