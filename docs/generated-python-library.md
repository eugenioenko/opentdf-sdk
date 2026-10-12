# Generated Python library

The shared source compiles into the installable `opentdf-tdf3` wheel
(import `opentdf_tdf3`), emitted from the two export roots `src/library`
(the deprecated `encrypt`/`decrypt` façade) and `src` (the handle API of
[api-spec.md](api-spec.md)). The hand-written
[host adapter](../src/hosts/python/__init__.py.in) presents that API in
Python idiom: it maps names, idioms and errors and implements no protocol.
Production calls use the generated Python, native `cryptography` and the
standard library; no compiler, Go process or external executable.

## Build and installation

Python 3.10 on Linux x86-64 is the observed environment (the locked `cffi`
wheel is `cp310`); Python 3.10+ is the API baseline. From `goalchemy/`,
build `GOTOOLCHAIN=go1.25.14 go build -o out/python-tdf-library/goalchemy
./cmd/goalchemy`, then run `scripts/build-generated-python.sh [destination]`.
The script needs a build environment with the locked dependencies: pass it
as `TDF_PYTHON` (a venv's `python`) with `TDF_WHEELHOUSE`, or let it make the
default one under `.local/python-tdf-library/` (a `python3.10` venv and a
wheelhouse filled by `scripts/python-wheelhouse.sh`, which downloads the
wheels of [dependencies.lock.json](../src/hosts/python/dependencies.lock.json)
and verifies their SHA-256). The output is `dist/opentdf_tdf3-0.1.0-py3-none-any.whl`
plus `package-sha256.json`; nothing is published.

| Dependency | Role | License |
| --- | --- | --- |
| cryptography 50.0.2 | Runtime crypto; contains its OpenSSL implementation | Apache-2.0 OR BSD-3-Clause; bundled notices retained |
| cffi 2.1.1 | Runtime native support | MIT-0 |
| pycparser 3.0 | Runtime cffi dependency | BSD-3-Clause |
| typing-extensions 4.16.0 | Runtime dependency, needed by cryptography on Python 3.10 | PSF-2.0 |
| setuptools 80.9.0 | Build only | MIT; vendored notices retained |
| wheel 0.45.1 | Build only | MIT |

`tests/interop/generatedpython/consumer.py` is the independent consumer,
run with `python -I` from a venv holding only the runtime wheels and the SDK
wheel: it drives the real platform (encrypt, decrypt, 100 round trips with
the async twins, a typed negative) and, in `controlled` mode, the offline
checks (discovery failures, deadlines, untrusted TLS, token 401, allowlist,
invalid input, unsupported options, 100 reader handles, closed handle).

## Handle API

```python
from opentdf_tdf3 import SDK, OpenTDFError

with SDK('http://localhost:8080', client_id='opentdf-sdk', client_secret='secret',
         insecure_plaintext=True) as sdk:
    with sdk.create_tdf(plaintext, attributes=['https://example.com/attr/attr1/value/value1'],
                        kas_information=[{'url': 'http://localhost:8080/kas'}],
                        mime_type='text/plain') as obj:
        manifest = obj.manifest()        # keyed by the reference's JSON tags
        ciphertext = obj.bytes           # or: sdk.encrypt_bytes(plaintext, **options)
    with sdk.load_tdf(ciphertext, kas_allowlist=['http://localhost:8080']) as reader:
        attributes = reader.data_attributes()
        reader.init()                    # fetch and unwrap keys now
        metadata = reader.unencrypted_metadata()
    try:
        plain = sdk.decrypt_bytes(ciphertext, kas_allowlist=['http://localhost:8080'])
    except OpenTDFError as e:
        print(e.kind, e.http_status, e.message)

async with await SDK.create_async('http://localhost:8080', client_id=..., client_secret=...) as sdk:
    data = await sdk.encrypt_bytes_async(plaintext, attributes=[...])
    plain = await sdk.decrypt_bytes_async(data, ignore_allowlist=True)
```

| Adapter symbol | Reference | Notes |
| --- | --- | --- |
| `SDK(endpoint, *, timeout=None, **options)`, `await SDK.create_async(endpoint, **options)` | `New` | Fetches `/.well-known/opentdf-configuration` and the issuer's OpenID configuration unless `platform_configuration` is given; `validate_connection=True` runs the health check first. |
| `sdk.create_tdf(plaintext, **options)` → `TDFObject` (`bytes`, `manifest()`, `size()`, `close()`, `with`), `sdk.encrypt_bytes` | `CreateTDFContext` over bytes, `TDFObject` | Single-KAS templates; RSA-2048 and P-256 wrapping. Streams arrive in Phase 7. |
| `sdk.load_tdf(ciphertext, **options)` → `Reader` (`init`, `manifest`, `policy`, `data_attributes`, `unencrypted_metadata`, `close`, `with`), `sdk.decrypt_bytes` | `LoadTDF`, `Reader`, `DecryptBytes` | The KAS allowlist comes from the registry unless `kas_allowlist` or `ignore_allowlist` is set, as in the reference. |
| `sdk.is_healthy`, `platform_configuration`, `get_base_key`, `store_kas_keys`, `close`, `with` | same | `platform_configuration()` returns the members as a `dict` of strings (non-string members as compact JSON). |
| `is_valid_tdf`, `get_tdf_type`, `is_platform_endpoint_malformed`, `key_type_to_policy_algorithm`, `policy_algorithm_to_key_type`, `version`, `tdf_spec_version`, `supported_features` | same | Free functions. |

Every method has an `_async` twin returning a coroutine (`create_tdf_async`,
`load_tdf_async`, `decrypt_bytes_async`, `reader.init_async`, ...), and
`SDK`, `TDFObject` and `Reader` are both context managers and async context
managers. Options are keyword arguments with the shared-source field names in
snake_case (`Options`, `TDFOptions`, `ReaderOptions`); every absent option
is the reference default, and an unknown name raises `TypeError`. Sizes are
`int`, strings `str`, payloads `bytes`; nested options are dicts
(`kas_information=[{'url': ..., 'public_key': ..., 'kid': ..., 'algorithm': ...}]`).
`platform_configuration` is a `dict` whose non-string members are carried as
JSON text. `token_provider` is a callable receiving a `ProviderRequest` and
returning an `AccessToken` (or a coroutine or `Future` of one); it replaces
client credentials, and the SDK then signs rewrap requests with the
reference's ephemeral ES256 key unless `dpop_key_pem` is set. The sync
methods take `timeout=` seconds: when it expires the call is canceled and
`OpenTDFError(kind='deadline_exceeded')` is raised; cancelling an awaiting
task cancels the call. Options this build does not implement yet
(`token_exchange`, `cert_exchange`, `tls`, `unix_socket`, service endpoint
overrides, assertions, target mode `4.2.2`, schema validation, RSA-4096 and
P-384/P-521 keys) fail with `kind == 'unsupported_feature'` and the message
`unsupported in this build: <option>`.

Manifests and policies are dicts keyed by the reference's JSON tags
(`manifest['encryptionInformation']['keyAccess'][0]['url']`,
`manifest['payload']['mimeType']`, `policy['body']['dataAttributes']`), so
they read like `manifest.json`.

## Errors

Every failure is an `OpenTDFError` with `kind`, `op`, `message` (the
reference's wording, see [error-wording.md](error-wording.md)),
`http_status`, `connect_code`, `service_message`, `kas_url`,
`required_obligations` and `assertion_id`. Boundary failures keep the
runtime's kind: `canceled`, `deadline_exceeded`, `closed` (a call after
`close()`), `invalid_argument` (a wrong value type), `poisoned`,
`host_fault`. Switch on `kind`; the kinds are the `ErrorKind` list of
[api-spec.md](api-spec.md).

## Deprecated façade

`encrypt`, `decrypt`, `encrypt_sync`, `decrypt_sync`, `encrypt_async`,
`decrypt_async`, `Config`, `KASRoute`, `EncryptOptions`, `Decrypted`,
`Operation` and `TDFError` remain with their previous shapes for one release,
implemented in the shared source as thin wrappers that construct an SDK per
call. `TDFError.code` is now the `Error.kind` string; `policy_base64`,
`dissem` and `segment_hash_algorithm` other than `GMAC` are no longer
supported, and empty metadata writes no `encryptedMetadata` entry, as the
reference does.

## CLI

`src/hosts/python/cmd/otdfgc/otdfgc.py` is the native Python CLI over this
wheel, built by `scripts/build-python-cli.sh <package> <launcher>` (a venv
with the locked runtime wheels and the SDK wheel beside the generated
package) and driven by the xtest harness as the `gcpy` SDK type (see
[xtest.md](xtest.md)). It implements the same contract as the Go `otdfgc`:
`encrypt`, `decrypt`, `inspect`, `supports <feature>` and `--version --json`
with the same flags, exit codes (0, 1 operation failed, 2 usage or
`unsupported in this build: <flag>`) and otdfctl's wording.
