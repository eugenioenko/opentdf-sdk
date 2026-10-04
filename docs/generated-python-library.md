# Generated Python TDF3 library

The installable `opentdf-tdf3` wheel imports as `opentdf_tdf3` and executes the
shared [value façade](../src/library/library.go). Production calls use generated
Python, native `cryptography` and the Python standard library. They require no
compiler, Go process, stock SDK or external executable. The package provides
per-call clients, with PEM key imports owned and released by each operation.
Persistent clients and public key handles are outside this exported surface.

## Build and installation

The observed environment is CPython 3.10.12 on Linux x86-64, Go 1.25.14 for
compilation, and `cryptography` 50.0.2 backed by bundled OpenSSL 4.0.3.
Python 3.10+ is the API baseline. The recorded native wheel needs glibc 2.34+;
other platforms require their corresponding maintained dependency wheels and
separate validation. No system crypto installation substitutes for the pin.

From the workspace root:

```sh
python3 -m venv sdk/.local/python-tdf-library/venv
sdk/.local/python-tdf-library/venv/bin/pip download --only-binary=:all: \
  --dest sdk/.local/python-tdf-library/wheels \
  cryptography==50.0.2 cffi==2.1.1 pycparser==3.0 typing-extensions==4.16.0 \
  setuptools==80.9.0 wheel==0.45.1
sdk/.local/python-tdf-library/venv/bin/pip install --no-index \
  --find-links sdk/.local/python-tdf-library/wheels \
  cryptography==50.0.2 cffi==2.1.1 pycparser==3.0 typing-extensions==4.16.0 \
  setuptools==80.9.0 wheel==0.45.1
(cd goalchemy; GOTOOLCHAIN=go1.25.14 go build \
  -o out/python-tdf-library/goalchemy ./cmd/goalchemy)
sdk/scripts/build-generated-python.sh
python3 -m venv sdk/.local/python-tdf-library/consumer-venv
sdk/.local/python-tdf-library/consumer-venv/bin/pip install --no-index \
  --find-links sdk/.local/python-tdf-library/wheels \
  goalchemy/out/python-tdf-library/sdk/dist/opentdf_tdf3-0.1.0-py3-none-any.whl
sdk/.local/python-tdf-library/consumer-venv/bin/python -I -c \
  'import opentdf_tdf3; print(opentdf_tdf3.__file__)'
```

Verify downloaded artifact and license hashes against the
[dependency lock](../src/hosts/python/dependencies.lock.json) before use. The build
script verifies dependency wheel hashes and installed versions, removes its own
generated package/build trees before compilation,
packages distinct relative notice paths, and fixes wheel timestamps through
`SOURCE_DATE_EPOCH`. Its output includes `package-sha256.json`. Two clean builds
of the final acceptance-repaired wheel produced byte-identical output with
SHA-256 `b0fcf60e4154765426d971ddd6bd7edbde509b8b37bb399911f8bc72e483f2b6`.
The preserved pre-acceptance packaging wheel has SHA-256
`56ff5044543e7c4f36c01077e76900eedca665753d1f3edaa2f2a5bc44f530eb`.

| Dependency | Role | License |
| --- | --- | --- |
| cryptography 50.0.2 | Runtime crypto; contains its OpenSSL implementation | Apache-2.0 OR BSD-3-Clause; bundled notices retained |
| cffi 2.1.1 | Runtime native support | MIT-0 |
| pycparser 3.0 | Runtime cffi dependency | BSD-3-Clause |
| typing-extensions 4.16.0 | Runtime dependency, needed by cryptography on Python 3.10 | PSF-2.0 |
| setuptools 80.9.0 | Build only | MIT; vendored notices retained |
| wheel 0.45.1 | Build only | MIT |

The lock records exact downloaded wheel SHA-256s, metadata and license hashes.
Build-only packages do not become production imports. Original wheel archives,
installation receipts and dependency observations are ignored under
`.local/python-tdf-library/`. Crypto maintenance and installation information
comes from [PyCA documentation](https://cryptography.io/en/50.0.2/installation/)
and the [release metadata](https://pypi.org/project/cryptography/50.0.2/).

## Native API and ownership

```python
import asyncio
from opentdf_tdf3 import Config, EncryptOptions, encrypt_sync, decrypt_async

config = Config(platform_url="https://platform.example", kas_url="https://platform.example/kas",
                issuer_url="https://issuer.example", client_id="server-client",
                client_secret="read-from-server-secret-storage")
archive = encrypt_sync(config, b"\x00\xffpayload",
                       EncryptOptions(metadata=b"", include_metadata=True))
result = asyncio.run(decrypt_async(config, archive))
assert result.payload == b"\x00\xffpayload"
assert result.has_metadata and result.metadata == b""
```

`Config`, `KASRoute`, `EncryptOptions` and `AccessToken` are typed native
dataclasses. `Decrypted` is frozen and owns immutable `payload`, `metadata`
and UTF-8 `manifest_json` fields plus `has_metadata`. `encrypt` and `decrypt`
return an `Operation[T]`; `.result(timeout)` blocks, `.cancel()` requests
cancellation, and `await operation` waits asynchronously. The timeout argument
of `.result()` limits the caller's wait; it does not cancel the operation.
`encrypt_sync`/`decrypt_sync` block through cleanup. `encrypt_async` and
`decrypt_async` expose native coroutine entry points.

Payload and metadata accept `bytes`, `bytearray` or `memoryview`; submission
bounds the actual byte count before copying, including typed and multidimensional
views, and rejects released views as `invalid_argument`. It copies inputs before
a worker can acquire the source owner. Configuration routes,
arrays and nested generic values are snapshotted. Mutating submitted inputs or
later results cannot change earlier output. Metadata absence and explicit-empty
metadata differ: set `include_metadata=True` with `metadata=b""` for the latter.
Native text uses strict UTF-8, rejecting lone surrogates. Generic compiler value
exports use byte strings, preserving arbitrary source-Go string bytes. Exact
Python integers retain signed int64 limits; bool, float and out-of-range values
cannot silently become source integers. Shared segment-size/default behavior
remains in the shared implementation.

Each generated runtime serializes overlapping operations FIFO. Package import
runs no source initialization and changes no process recursion limit, thread
stack policy or TLS policy. The acquired calling owner initializes fresh source
state for each operation, constructs source contexts/values on that owner,
and copies results and typed errors while it is alive. Retirement waits native
ACKs and clears source frames, contexts, timers, polling closures, globals,
field-reference tables, callback registrations and native key material. Native
user continuations run outside owner reservations and queue locks.

Queued cancellation settles after its copied inputs and queue roots are
released, independently of a held active call. Active cancellation wakes the
owner, cancels source contexts and requests transport/provider stop. Success or
failure becomes visible only after sockets, bodies and provider work release.
Canceling an awaiting Python task likewise awaits cleanup before propagating
`asyncio.CancelledError`, including repeated cancellation requests. Ordinary
CPU/native crypto executes synchronously on the serialized owner; it observes
cancellation at subsequent source boundaries. No thread is forcibly terminated.

`TDFError` distinguishes `source`, `canceled`, `deadline_exceeded`,
`source_panic`, `source_fatal`, `host_fault` and `invalid_argument`. It owns
`code`, `operation`, `http_status`, `server_code`, `server_message`,
`required_obligations` and `cause_category`. Shared error fields are preserved:
for a canceled/deadline HTTP exchange, `code` may be `transport` while `kind`
and `cause_category` identify cancellation or expiry. Declared SDK errors do not
become source panics. Unexpected acquisition, submission, callback and cleanup
faults become `host_fault`; library source panic/fatal does not exit the host
process. Error tracebacks do not retain retired source owners before publication.

## Providers, crypto and transport

Pass `provider=` to any operation to supply a native token callback without
source client credentials. It receives `ProviderRequest`, whose `payload` is
owned bytes and `cancellation` is a `threading.Event`. Return an `AccessToken`,
a coroutine returning one, or a `concurrent.futures.Future` settled only after
provider resource cleanup. Raise `ProviderRejected` for declared token failure;
unexpected exceptions are adapter faults. Provider callbacks execute on native
workers, never source frames. Coroutine cancellation awaits provider `finally`.
A synchronous/Future provider must honor the stop event and acknowledge cleanup
by returning/settling; a provider that never releases prevents safe completion.

`AccessToken.expires_at` is exact nonnegative int64 Unix seconds. For DPoP, pass
an explicit matching `auth_private_key_pem`, set `dpop=True`, and return scheme
`DPoP` with the correct `confirmation_jkt`. The shared implementation validates
binding, expiry, KAS trust and nonce retries. Native provider evidence includes
both RS256 and ES256 matching keys and rejects a mismatched key.

[Native capabilities](capabilities.md) use maintained `cryptography`
implementations: AES-256-GCM, OAEP SHA-1/MGF1-SHA-1/empty label, RS256, ES256
P1363, P-256 ECDH32, HKDF/HMAC/SHA-256, randomness, and strict PEM/JWK conversion.
Private PEM imports use the maintained format-aware parser, accepting valid
P-256 PKCS8 without the optional public point while rejecting mislabeled
PKCS1/SEC1, trailing DER, extra PEM blocks and unsupported keys.
Keys are opaque owner-associated registry IDs. Aliases share idempotent Close;
closed/foreign handles cannot acquire material. The API exports PEM configuration
and keeps key lifetimes internal. No private-material erasure guarantee is made.
Canonical base64/base64url uses bounded standard-library encoding.

HTTP uses a dedicated `http.client` connection, bounded request/response headers
and body, GET/POST only, no redirect following, and `ssl.create_default_context`
for verified HTTPS. Request deadlines include adapter construction/copying and
use owner-monotonic time; `timeout_millis=0` selects the shared 15s default, with
an explicit range of 1..300000ms. Cancellation shuts down the socket and ACK
follows response/connection close. Hostname resolution/native socket acquisition
cannot always be forcibly interrupted by CPython; settlement still waits for
actual release. Host policy/catastrophic interpreter or native-extension crashes
cannot promise managed cleanup. Source ordinary calls are bounded to 128 nested
calls and cooperative frames to 512, matching the accepted CPython host limits.
Payload limits, segmentation, metadata limits and explicit mandatory-feature
rejection remain those of the [shared engine](../src/tdf/engine.go).

## Verification receipts

All evidence lives under ignored `.local/python-tdf-library/`; compiler output
is `../goalchemy/out/python-tdf-library/`. The importing consumer runs its
installed wheel through `python -I`, outside generated module search paths.

| Check | Terminal evidence |
| --- | --- |
| Independent primitive vectors and OpenSSL interop | 113 checks; `focused-terminal-boundary.log` |
| Actual generated generic library | 52 checks; `focused-root-retirement-supplement.log` |
| Python byte/host/compiler branches | `focused-compiler-native.log`, status 0 |
| Genuine emitted owner/HTTP/fatal/library integration | `focused-integration.log`, status 0 |
| Python language regressions | `python-language.log`, status 0 |
| Python target conformance | 203/203; `python-harness-encoding-repair.log` |
| Installed package/import policy preservation | `focused-package-import.log`, status 0 |
| Clean wheel reproducibility/BASIC runtime equivalence | `clean-repro-compare.log`, status 0 |
| BASIC real KAS | 35 comparisons, eleven typed negatives; `basic/results.json` |
| EC real KAS | 280 comparisons, eight denied-policy expansions; `ec/results.json` |
| Enforced DPoP real KAS | 226 comparisons, ten negatives and four explicit stock Web limitations; `dpop/results.json` |
| Controlled transport rejections | Nine original successes plus active-cancel/BOM focused repair; each `consumer.status` is 0 for retained passing cases |

Seven canonical cases are empty, binary, exact segment boundary, multisegment,
HS256, encrypted metadata and explicit-empty metadata. Expanded matrices cover
RSA/P-256 wrapping and sessions, RS256/ES256, explicit key/kid and discovery,
both references/directions, native matching-token callbacks and typed denials.
Original failures remain visible: misspelled initial mapping IDs; native-defer
and terminal-result boundary probes; the allocation-only foreign-owner fixture
assertion; base64 encode result arity (201/203); stale flattened-license
reproducibility; and the controlled cancellation-code fixture assertion. Focused
repairs preserve original logs/statuses and successful artifacts. The
pre-acceptance `56ff5044…` wheel's 86 executable Python files exactly match the
retained BASIC-tested wheel; that equality applies to the license-only
packaging repair. No additional BASIC subset was run for that repair.

The single acceptance review identified valid omitted-public-point PKCS8 import
and memoryview byte-count bounds. Their bounded repair evidence is retained
separately under `.local/python-tdf-library-acceptance-repair/`, with the original
handoff, freeze, wheels and real-KAS matrices unchanged:

| Acceptance repair check | Terminal evidence |
| --- | --- |
| Native crypto, malformed-key rejection and native Go omitted-point parity | 131 checks; `focused-native-importing.log`, status 0 |
| Actual generated generic library with omitted-point import/sign/ECDH | 53 checks; same focused log, status 0 |
| Installed ordinary/typed/multidimensional buffers, ownership and rejection before source entry | 27 checks; `installed-buffer.log`, status 0 |
| Installed omitted-point import/public/sign/ECDH and native Go parity | `installed-native-proof-repair.log`, status 0; retained `native-proof/results.json` artifact hashes |
| Clean repaired-wheel reproducibility and executable delta | `artifact-delta.json`; two identical `b0fcf60e…` wheels |
| Catalog and generated-spec freshness | `catalog.log` and `freshness.log`, status 0 |

The final wheel differs from `56ff5044…` in exactly two executable modules:
`_generated/rt/runtime/lib_crypto_close.py` and
`_generated/rt/types/library.py`, plus wheel `RECORD`. All other entries are
byte-identical. The first installed native proof's unreachable-verifier fixture
failure remains retained; the corrected proof uses the maintained backend to
verify the native Go signature without expanding SDK exports. These repairs
change no protocol or transport behavior and require no additional KAS matrix.

The stock Web reader does not expose decrypted metadata. Its enforced nonce
DPoP limitations remain separately recorded; retained stock-Go/Web formats
created under Bearer are consumed by generated Python under enforced DPoP.
Successful generated auth is not a claim of successful stock Web enforced auth.
BASIC was restored and verified by the root after the enforced profile.
All seven target libraries are accepted. [Final delivery](final-delivery.md)
records clean package and CI evidence separately. Package-registry publication
requires a separate release action.
