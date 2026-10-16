# Error wording the xtest suite depends on

Every string or regex that `../tests/xtest/test_*.py` asserts against SDK
output, the reference source that produces it, and the
[api-spec.md](api-spec.md) error kind it maps to. Collected by reading every
`expected_patterns`, `rewrap_403_pattern`, `re.search` and `assert ... in`
check in the test modules (see [xtest-coverage.md](xtest-coverage.md)). File
references are `platform/sdk` (the reference SDK) or `platform/otdfctl` (the
reference CLI) at the pinned platform revision `4d9e555bddf0`; web-sdk lines
are given where a pattern alternative exists only for the js SDK.

All greps are case-insensitive. `expect_error=True` folds stderr into stdout,
so the generated CLI may print on either stream; otdfctl prints
`Failed to decrypt file: <error chain>` (`otdfctl/cmd/tdf/decrypt.go:103`) and
`Failed to encrypt: <error chain>` (`otdfctl/cmd/tdf/encrypt.go:218`) with the
SDK's wrapped error text after the colon. The generated `otdfgc` CLI prints
the same prefix and the `Error.Message` chain, so the substrings below
appear verbatim.

## Reference error values

The sentinels the patterns match, with their exact text
(`platform/sdk/tdferrors.go`, `decrypterrors.go`):

| Reference value | Text | File:line | api-spec kind |
| --- | --- | --- | --- |
| `ErrTampered` | `tamper detected` | `sdk/tdferrors.go:33` | `KindTampered` (every tamper kind unwraps to it; `Error()` carries the `[tamper detected] ` prefix) |
| `ErrRootSigValidation` | `[tamper detected] tdf: failed integrity check on root signature` | `sdk/tdferrors.go:34` | `KindRootSignature` |
| `ErrSegSizeMismatch` | `[tamper detected] tdf: mismatch encrypted segment size in manifest` | `sdk/tdferrors.go:35` | `KindSegmentSize` |
| `ErrSegSizeUnresolved` | `[tamper detected] tdf: segment size invalid or missing from manifest, with no default to fall back on` | `sdk/tdferrors.go:36` | `KindSegmentSize` |
| `ErrSegSigValidation` | `[tamper detected] tdf: failed integrity check on segment hash` | `sdk/tdferrors.go:37` | `KindSegmentSignature` |
| `ErrGMACSignatureFailed` | `[tamper detected] tdf: ciphertext too short for a gmac signature` | `sdk/tdferrors.go:38` | `KindSegmentSignature` |
| `ErrTDFPayloadReadFail` | `[tamper detected] tdf: fail to read payload from tdf` | `sdk/tdferrors.go:39` | `KindPayloadRead` |
| `ErrRootSignatureFailure` | `[tamper detected] tdf: issue verifying root signature` | `sdk/tdferrors.go:41` | `KindRootSignature` |
| `ErrRewrapBadRequest` | `[tamper detected] tdf: rewrap request 400` | `sdk/tdferrors.go:42` | `KindRewrapBadRequest` |
| `ErrUnsupportedRootIntegrityAlgorithm` | `tdf: unsupported root integrity algorithm` (followed by `: "GMAC"`, `sdk/tdf.go:1696`) | `sdk/tdferrors.go:46` | `KindUnsupportedRootIntegrity` |
| `ErrUnsupportedSegmentIntegrityAlgorithm` | `tdf: unsupported segment integrity algorithm` | `sdk/tdferrors.go:51` | `KindUnsupportedSegmentIntegrity` |
| `kasGenericBadRequest` | `desc = bad request` (substring matched in the KAS error) | `sdk/tdferrors.go:67` | classifier input |
| `ErrKASRequestError` | `tdf: KAS request error` | `sdk/tdferrors.go:70` | `KindKASRequest` |
| `ErrRewrapForbidden` | `tdf: rewrap request 403` | `sdk/tdferrors.go:71` | `KindRewrapForbidden` |
| `ErrAssertionFailure{ID}` | `[tamper detected] tdf: issue verifying assertions, id: <id>` | `sdk/tdferrors.go:79-81` | `KindAssertionFailure` (`AssertionID`) |
| `ErrTDFNotDecryptable` | `tdf: input is not decryptable` | `sdk/decrypterrors.go:12` | `KindNotDecryptable` |
| `ErrTDFDecryptFailed` | `tdf: decrypt failed` | `sdk/decrypterrors.go:20` | `KindDecryptFailed` |

How the reader composes them (`platform/sdk/tdf.go`):

- Root signature: `fmt.Errorf("%w: %w", ErrRootSignatureFailure, ErrRootSigValidation)` at `tdf.go:1479`, so the text is `[tamper detected] tdf: issue verifying root signature: [tamper detected] tdf: failed integrity check on root signature`. A segment-hash edit also changes the aggregate hash, so this fires before the per-segment check.
- Segment hash: `ErrSegSigValidation` at `tdf.go:1058` and `tdf.go:1180`; segment size: `ErrSegSizeMismatch` at `tdf.go:1048`, `tdf.go:1170`, `tdf.go:1226`, `tdf.go:1486`.
- Assertions: `tdf.go:1506`, `:1518` (`assertion verification failed`), `:1524`, `:1545` (`assertion hash missmatch`, sic), `:1549` (`failed integrity check on assertion signature`), each wrapped in `ErrAssertionFailure`.
- KAS errors: `getKasErrorToReturn` (`tdf.go:1778-1798`) joins `ErrRewrapBadRequest` when the per-KAO error contains `InvalidArgument` (`codes.InvalidArgument.String()`) and `desc = bad request`, `ErrKASRequestError` when it contains `InvalidArgument` with any other description, and `ErrRewrapForbidden` when it contains `PermissionDenied`. The KAS side emits `bad request` for policy-binding and key failures (`service/kas/access/rewrap.go:486-904`, `err400("bad request")`) and `forbidden` / `forbidden: <category>` for denials (`rewrap.go:1018`, `:1094`, `err403`). The joined error keeps the KAS text, so both `tdf: rewrap request 403` and the KAS `forbidden` appear.
- Allowlist: `KasAllowlist: kas url <url> is not allowed` at `tdf.go:1589` (and `bulk.go:187`), raised before any request is sent.
- Split reconstruction: `splitKey.unable to reconstruct split key: <skipped splits>` at `tdf.go:1456`, joined with every per-KAO error.
- Missing policy in the rewrap response: `could not find policy in rewrap response` at `tdf.go:1600`.
- Obligations: otdfctl, not the SDK, appends `\nrequired obligations: [<fqns>]` to a failed decrypt when `Reader.Obligations` returns any (`otdfctl/pkg/handlers/tdf.go:302-313`). The generated CLI must do the same.

## Patterns by test

| Test (module) | Pattern (regex, case-insensitive) | Reference source | api-spec kind |
| --- | --- | --- | --- |
| `test_tdf_with_unbound_policy`, `test_tdf_with_altered_policy_binding` (`test_tdfs.py`, `assert_kas_request_error`) | `KAS request error\|rewrap request 400\|bad request\|InvalidArgument\|tamper\|InvalidFileError\|could not find policy in rewrap response` | `ErrRewrapBadRequest` (`tdferrors.go:42`) joined with the KAS `bad request` text (`rewrap.go:486`, `:705`, `:888`) by `tdf.go:1790`; `ErrKASRequestError` (`tdferrors.go:70`, `tdf.go:1792`); `could not find policy in rewrap response` (`tdf.go:1600`); `InvalidFileError` is web-sdk `lib/src/errors.ts:60` | `KindRewrapBadRequest` (unwraps to `ErrTampered`) or `KindKASRequest` |
| `test_tdf_with_altered_root_sig` (`assert_tamper_error(type="root")`) | `root` must appear; with `better-messages-2024`: `tamper\|IntegrityError`; otherwise `integrity\|signature\|bad request` | `ErrRootSignatureFailure` plus `ErrRootSigValidation` (`tdferrors.go:34`, `:41`; `tdf.go:1479`); `IntegrityError` is web-sdk `lib/src/errors.ts:69` | `KindRootSignature` |
| `test_tdf_with_altered_seg_sig_wrong` (`type="signature"`) | `signature`; `tamper\|IntegrityError` | the root signature check fails first (`tdf.go:1479`, text contains `root signature`) | `KindRootSignature` |
| `test_tdf_with_altered_enc_seg_size` (`type=""`) | `tamper\|IntegrityError\|integrity check` | `ErrSegSizeMismatch` (`tdferrors.go:35`; `tdf.go:1048`/`:1170`/`:1226`) | `KindSegmentSize` |
| `test_tdf_altered_payload_end` (`type="segment"`) | `segment`; `tamper\|IntegrityError\|integrity check` | `ErrSegSigValidation` (`tdferrors.go:37`; `tdf.go:1058`/`:1180`) | `KindSegmentSignature` |
| `test_tdf_with_altered_assertion_statement`, `test_tdf_with_altered_assertion_with_keys` (`type="assertion"`) | `assertion`; `tamper\|IntegrityError\|integrity check` | `ErrAssertionFailure` text `issue verifying assertions, id: <id>` (`tdferrors.go:80`) wrapped at `tdf.go:1545` (`assertion hash missmatch`) or `:1549` | `KindAssertionFailure` |
| `test_tdf_with_malicious_kao` | `allowlist\|not allowed\|disallowed KASes\|AggregateError` | `KasAllowlist: kas url <url> is not allowed` (`tdf.go:1589`); `disallowed KASes` and `AggregateError` are js-only | `KindAllowlistRejected` |
| `test_or_attributes_success`, `test_and_attributes_success`, `test_hierarchy_attributes_success` (`test_policytypes.py`, `decrypt_or_dont`) | `forbidden\|unable to reconstruct split key` | `splitKey.unable to reconstruct split key` (`tdf.go:1456`) joined with `tdf: rewrap request 403` (`tdferrors.go:71`, `tdf.go:1795`) and the KAS `forbidden` (`rewrap.go:1018`); web-sdk `403 for [<url>]; rewrap permission denied: forbidden` (`lib/src/access/access-fetch.ts:79`) | `KindRewrapForbidden` (`KindNotDecryptable` when every split fails) |
| `test_obligations_not_entitled` (`test_abac.py`) | must match `tdf: rewrap request 403\|403 for \[https?://[^\]]+\]; rewrap permission denied`; must not match `required\s*obligations` | `ErrRewrapForbidden` (`tdferrors.go:71`, `tdf.go:1795`); otdfctl appends `required obligations:` only when `Reader.Obligations` is non-empty (`otdfctl/pkg/handlers/tdf.go:312-313`), and KAS returns none for an unentitled client | `KindRewrapForbidden`, empty `RequiredObligations` |
| `test_obligations_not_fulfillable`, `test_obligations_client_scoped` | the obligation value FQN (for example `https://<ns>/obl/test_obligation_scs/value/geofence`) and the 403 pattern | `required obligations: [<fqns>]` (`otdfctl/pkg/handlers/tdf.go:313`) from `RequiredObligations.FQNs` (`sdk/tdf.go:1607`, `:1823`); 403 as above | `KindObligationsRequired` (`RequiredObligations` populated) over `KindRewrapForbidden` |
| `test_decrypt_rejects_kao_kas_registration_when_disabled` | `bad request\|400` | `ErrRewrapBadRequest` `tdf: rewrap request 400` (`tdferrors.go:42`) with the KAS `bad request` (`rewrap.go:807-835`) | `KindRewrapBadRequest` |
| `test_root_signature.py` controls and exploits (`assert_decrypt_fails`) | any non-empty output that contains none of the infrastructure markers `failed to connect to the platform`, `failed to authenticate`, `connection refused`, `econnrefused`, `fetch failed`, `getaddrinfo`, `no such host`, `unauthorized_client`, `invalid_client`, `connectexception`, `unknownhostexception` | rejection text: `tdf: unsupported root integrity algorithm: "GMAC"` (`tdferrors.go:46`, `tdf.go:1695-1696`) for the forged cases, `ErrRootSignatureFailure` for truncate/reverse/duplicate without forgery; the markers come from otdfctl `Failed to connect to the platform. Is the platform accepting connections at '<endpoint>'?` (`otdfctl/cmd/common/common.go:198`) and `Failed to authenticate with flag-provided client credentials.` (`common.go:204`), node (`fetch failed`, `ECONNREFUSED`), the JVM (`ConnectException`, `UnknownHostException`) and the OIDC server (`unauthorized_client`, `invalid_client`) | `KindUnsupportedRootIntegrity`, `KindRootSignature`; infrastructure failures are `KindTransport`/`KindAuth` and must keep their distinct wording so they are never scored as rejections |
| `test_zip64_band_roundtrip` (`zip64_reader_is_broken`) | none asserted on the message; java < 0.19.0 must fail | n/a | n/a |
| `test_audit_logs_on_tampered_file` | none on the message; a rewrap `error` audit event must exist | the altered binding reaches KAS and is refused with `bad request` (`rewrap.go:888`) | `KindRewrapBadRequest` |

## URL parsing

URL errors come from Goalchemy's `std/net/url` (v0.6.3), which reproduces Go's
`net/url` text for a module declaring `go 1.25` built with the pinned go1.25.14
toolchain: `parse "<url>": <reason>` (`*url.Error`), `invalid URL escape "%zz"`,
`invalid port ":x" after host`, `invalid character " " in host name`,
`missing protocol scheme`, `first path segment in URL cannot contain colon`,
`net/url: invalid control character in URL`. The SDK wraps them with the
reference's prefixes: `cannot parse kas url(<url>): ` (`sdk/tdf_config.go:392`,
`sdk/kas_client.go:87`, `:476`), `invalid issuer URL "<url>": ` (`sdk/sdk.go:587`) and, new
with v0.6.3, `invalid url [<endpoint>]: ` when `New` is given a token endpoint
`url.Parse` rejects (`sdk/idp_access_token_source.go:73`, `KindInvalidInput`;
previously the request was attempted and failed as a transport error).

Changes from the interim `src/internal/urlutil` parser, all now matching Go:
non-ASCII bytes in a quoted URL print as UTF-8 (`"https://exámple.com:9x/"`)
instead of `\xc3\xa1` escapes; a non-ASCII host such as `exámple.com` is
accepted instead of failing with `invalid character "\xc3" in host name`; and a
bracketed host must be an IPv6 address, so `https://[zz]/` fails with
`invalid host: ParseAddr("zz"): unable to parse IP` and `http://[1.2.3.4]/` with
`invalid IP-literal`, where the interim parser accepted both. A host with
several colons (`https://example.com:80:`) is still accepted.

## Service calls (Phase 3 part 1)

KAS `Rewrap` and `PublicKey`, the well-known configuration, the registry
allowlist, the health check and `SDK.Call` now run on the generated Connect
clients (Goalchemy `std/connect`, which follows connect-go) over the SDK's
transport. A Connect error the service sends reads as before,
`<code>: <message>` (`permission_denied: rpc error: code = PermissionDenied
desc = forbidden`), so the KAS classifier (`getKasErrorToReturn`) and every
xtest pattern above match unchanged. What changed, all now as connect-go
reports it:

| Case | Before | Now |
| --- | --- | --- |
| Error status without a Connect body | code from a local table (`400` → `invalid_argument`), message the code alone | connect-go's mapping (`400` → `internal`, `404` → `unimplemented`, `502` → `unavailable`, other → `unknown`), message `<code>: <status line>` (`unavailable: 502 Bad Gateway`) |
| 2xx reply that is not JSON | `unexpected content type: text/html` | `unknown: invalid content-type: "text/html"; expecting "application/json"` |
| 2xx reply that is not valid JSON | `invalid JSON response` | `invalid_argument: unmarshal message: ...`, kind `transport` |
| Context already cancelled or expired | `context canceled` | `canceled: context canceled` (`deadline_exceeded: ...`) |
| `permission_denied` / `invalid_argument` from a non-KAS service | kinds `rewrap_forbidden` / `kas_request` | kinds `auth` / `invalid_input`; KAS calls keep the old kinds |

`HTTPStatus`, `ConnectCode` and `ServiceMessage` are filled as before; a
failure of the SDK's own transport (token source, DPoP, connection) keeps
its text and kind. Requests carry no `Accept` header and, as connect-go without a deadline
and as browsers' CORS preflights against the platform's default allow-list
need, no `Connect-Timeout-Ms` (`std/connect` v0.6.5 sends it only with
`Client.SendTimeout`, which the SDK leaves off; `TimeoutMillis` still
bounds each call). The unsigned rewrap request is `protojson` of the generated
`kas.UnsignedRewrapRequest`: fields in proto declaration order and an empty
`policyBinding.alg` omitted, as the reference's `protojson.Marshal` writes
it (whitespace aside).

## Attribute helpers, AccessToken and endpoint overrides (Phase 3 part 3)

No xtest pattern matches these; they are the reference's wording, in the
`Error.Message` of the kind shown. A service failure keeps its own kind and
Connect fields after the prefix (`listing attributes: not_found: ...` is
`invalid_input` with `ConnectCode` `not_found`); any other failure under a
prefix is `transport`. `Op` is the helper's name.

| Text | Reference | Kind |
| --- | --- | --- |
| `ListAttributes accepts at most one namespace filter, got <n>` | `sdk/discovery.go:38` | `invalid_input` |
| `listing attributes: <cause>` | `sdk/discovery.go:49` | the cause's, else `transport` |
| `listing attributes: exceeded maximum page limit (1000)` | `sdk/discovery.go:64` | `transport` |
| `invalid attribute FQN "<fqn>": <ErrInvalid reason>` | `sdk/discovery.go:78` | `invalid_input` |
| `checking attribute existence: <cause>` (`not_found` is `false` instead) | `sdk/discovery.go:88` | the cause's, else `transport` |
| `invalid attribute value FQN "<fqn>": <ErrInvalid reason>` | `sdk/discovery.go:103`, `:151` | `invalid_input` |
| `checking attribute value existence: <cause>` (`not_found` is `false` instead) | `sdk/discovery.go:113` | the cause's, else `transport` |
| `too many attribute FQNs: <n> exceeds maximum of 250` | `sdk/discovery.go:146` | `invalid_input` |
| `validating attributes: <cause>` | `sdk/discovery.go:159` | the cause's, else `transport` |
| `attribute not found: <fqn>, <fqn>` (`ErrAttributeNotFound`) | `sdk/discovery.go:170` | `invalid_input` |
| `entity must not be nil` | `sdk/discovery.go:196` | `invalid_input` |
| `getting entity attributes: <cause>` | `sdk/discovery.go:203` | the cause's, else `transport` |
| `invalid type: attribute regex fail`, `invalid type: attribute regex fail for [<fqn>]`, `invalid type: error in attribute name [<name>]`, `invalid type: error in attribute value [<value>]`, `invalid type: empty url path parts are not allowed`, `invalid type: slash not allowed in name or values` (`ErrInvalid` reasons) | `sdk/granter.go:60-141` | (inside the FQN errors above) |
| `no access token source configured; SDK was created without credentials` (`ErrNoAccessTokenSource`) | `sdk/auth_client.go:24` | `auth` |
| `access token is invalid` (`ErrAccessTokenInvalid`, an empty token) | `sdk/auth_client.go:31` | `auth` |
| `platform endpoint is malformed [<endpoint>]` for a malformed `WithCustom*Connection` endpoint | `sdk/sdk.go:151` (the platform endpoint's check) | `invalid_input` |

At the pinned platform revision `GetAttributeValuesByFqns` answers
`not_found: resource not found` when any requested value is missing, so
`ValidateAttributes` reports `validating attributes: not_found: resource not
found` there and `attribute not found: ...` only against a service that
omits missing values; both are the reference's behaviour.

## Autoconfigure and key splitting (Phase 4 part 1)

The writer's planning errors, from the ported `granter.go`, `key_splitter.go`
and `tdf.go`; `CreateTDF` reports them with kind `kas_request` unless the
cause carries its own (a service error keeps its Connect code).

| Message | Reference | Kind |
| --- | --- | --- |
| `failed generate plan: <cause>` | `sdk/tdf.go:501` | `kas_request` |
| `no default KAS specified; required for grantless plans` | `sdk/granter.go:732` | (inside `failed generate plan`) |
| `no key found for resource locator [<kas>#<kid>]` | `sdk/granter.go:798` | (inside `failed generate plan`) |
| `invalid algorithm [<alg>] for kas <kas> with kid [<kid>]: unsupported algorithm: <alg>` | `sdk/granter.go:802`, `sdk/basekey.go:83` | (inside `failed generate plan`) |
| `no defintion or grant found for [<fqn>]` (sic) | `sdk/granter.go:895` | (inside `failed generate plan`) |
| `no associated definition with value [<fqn>]`, `no associated namespace with definition [<def>] from value [<fqn>]` (offline values) | `sdk/granter.go:658,662` | `kas_request` |
| `unable to retrieve public key from KAS at [<kas>]: <cause>` | `sdk/tdf.go:524,613` | `kas_request` |
| `empty KAS key found for splitID:[<sid>], kas:[<kas>]: tdf: kas public key is missing` | `sdk/tdf.go:529` | `kas_request` |
| `splitID:[<sid>]: share names no KAS; its bytes could never be unwrapped`, `no key access objects generated` | `sdk/key_splitter.go:452,481` | `kas_request` |
| `unknown key type: <type>` (`KeyTypeToPolicyAlgorithm`), `unsupported algorithm: <alg>` (`PolicyAlgorithmToKeyType`) | `sdk/basekey.go:54,83` | `invalid_input` |
| `invalid alg [<alg>]: unrecognized key type: <alg>` (base key) | `sdk/basekey.go:23`, `lib/ocrypto/key_type.go:32` | `base_key_invalid` |
| `chunked: ...` (`SplitResult.Validate`, `VerifyReconstruction`), `chunked: default splitter requires a default KAS; ...` (`ErrSplitterRequiresDefaultKAS`), `chunked: unsupported KAS key algorithm: kas <url>: ...` (`ErrSplitterUnsupportedAlgorithm`) | `sdk/key_splitter.go` | returned as is (shared source only) |
| `Attributes and AttributeValues are exclusive: AttributeValues replaces the attributes` | none (struct options) | `invalid_input` |
| `unsupported in this build: wrapping key algorithm <post-quantum type>` | none (not planned) | `unsupported_feature` |

The base key's unknown algorithm now reads `unrecognized key type` (ocrypto's
`ParseKeyType`) instead of the earlier `unsupported key type`.

## Shim and CLI contract strings

Strings the shims themselves assert or emit, which the generated `cli.sh`
copies (`../tests/xtest/sdk/go/cli.sh`):

| String | Where | Meaning |
| --- | --- | --- |
| `Unknown feature: <name>` and exit 2 | `cli.sh supports` | feature not in `tdfs.feature_type`; `tdfs.SDK.supports` treats any non-zero exit as unsupported |
| `chunky unsupported: see DSPX-4590`, `zip64-at-2gib unsupported: see DSPX-4590`, `gmac_root_rejected is not probeable: the fix adds no CLI surface`, exit 1 | `cli.sh supports` | explicit no; the gc shim answers these from `SupportedFeatures()` instead |
| `Unsupported container format: <fmt>` and exit 2 | `cli.sh` | only `ztdf` is accepted positionally |
| `Incorrect argument provided` and exit 1 | `cli.sh` | verb other than `encrypt`/`decrypt`/`supports` |
| `sdk_version`, `schema_version`, `supported_features` | `otdfctl --version --json` (`otdfctl/cmd/root.go:33-37`) | the gc CLI's `--version --json` keys; `supports dpop` and `dpop_nonce_challenge` are answered from `supported_features` (`sdk/version.go:21-27`) |
| `<out>.tdf` renamed to `<out>` | `cli.sh` encrypt | otdfctl appends `.tdf`; the gc CLI writes `--out` exactly, which the shim tolerates |
