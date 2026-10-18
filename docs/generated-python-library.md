# Generated Python library

The shared source compiles into the installable `opentdf-tdf3` wheel
(import `opentdf_tdf3`), emitted from the export root `src` (the handle API
of [api-spec.md](api-spec.md)). The hand-written
[host adapter](../src/hosts/python/__init__.py.in) presents that API in
Python idiom: it maps names, idioms and errors and implements no protocol.
Production calls use the generated Python, native `cryptography` and the
standard library; no compiler, Go process or external executable.

## Build and installation

Python 3.10 on Linux x86-64 is the observed environment (the locked `cffi`
wheel is `cp310`); Python 3.10+ is the API baseline. From `goalchemy/`,
build `go build -o out/python-tdf-library/goalchemy
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
| `sdk.create_tdf(plaintext, **options)` → `TDFObject` (`bytes`, `manifest()`, `size()`, `close()`, `with`), `sdk.encrypt_bytes` | `CreateTDFContext` over bytes, `TDFObject` | Key access templates from autoconfigure or explicit KAS information, split across KAS; RSA-2048/4096 and P-256/384/521 wrapping. |
| `sdk.encrypt_to(output, plaintext, **options)` → `TDFObject` (empty `bytes`) | `CreateTDFContext` over streams | `output` is a writable file-like object, `plaintext` a readable one (or `bytes`); the payload is encrypted one segment at a time. A seekable input or `input_size=` selects ZIP32; an unknown size writes ZIP64. |
| `sdk.load_tdf(ciphertext_or_file, **options)` → `Reader` (`init`, `manifest`, `policy`, `data_attributes`, `unencrypted_metadata`, `read(size=-1)`, `seek(offset, whence=0)`, `read_at(offset, length)`, `write_to(output)`, `as_file()`, iteration, `close`, `with`), `sdk.decrypt_bytes`, `sdk.decrypt_to(output, ciphertext_or_file, **options)`, `sdk.decrypt_file(input_path, output_path, **options)` | `LoadTDF`, `Reader`, `DecryptBytes`, `DecryptTo`, `DecryptFile` | A seekable readable file-like object is read segment by segment; `as_file()` is the plaintext as an unbuffered binary file (`io.RawIOBase`) sharing the reader's position. Call `init` first when the SDK uses a `token_provider` (file reads carry no callbacks). `decrypt_file` writes beside the output and renames into place. One rewrap request per KAS; the DEK is the XOR of one share per split id, any KAO of a split will do, duplicates are skipped; post-quantum KAOs fail as `unsupported_key_access`. RSA and EC sessions of every non-PQ key type. The KAS allowlist comes from the registry unless `kas_allowlist` or `ignore_allowlist` is set, as in the reference. |
| `assertions=[{'id', 'type', 'scope', 'applies_to_state', 'statement': {...}, 'signing_key': {'alg', 'key'}}]`, `system_metadata_assertion=True`; `assertion_verification_keys={'default_key': {'alg', 'key'}, 'keys': {assertion_id: {'alg', 'key'}}}`, `disable_assertion_verification=True` | `WithAssertions`, `WithSystemMetadataAssertion`, `WithAssertionVerificationKeys`, `WithDisableAssertionVerification` | Assertions are signed into the manifest with HS256 over the payload key or the config's `signing_key` (HS256 secret or RS256 private PEM as `str`) and verified on read by default, by assertion id with a default key, or with the payload key; failures are `kind == 'assertion_failure'` with `assertion_id`. |
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
(`token_exchange`, `cert_exchange`, `tls`, `unix_socket`, target mode `4.2.2`, schema validation, post-quantum key types) fail with `kind == 'unsupported_feature'` and the message
`unsupported in this build: <option>`.

Manifests and policies are dicts keyed by the reference's JSON tags
(`manifest['encryptionInformation']['keyAccess'][0]['url']`,
`manifest['payload']['mimeType']`, `policy['body']['dataAttributes']`), so
they read like `manifest.json`.

## Service clients

`sdk.attributes`, `sdk.namespaces` and the other services take and return
the protobuf messages of `opentdf_tdf3.protocol` (the `protobuf` package):
one snake_case method per RPC with an `*_async` twin. Calls are protobuf
JSON through `sdk.call` and the SDK's authenticated transport; failures are
`OpenTDFError` with `connect_code`.

```python
from opentdf_tdf3.protocol.policy.attributes.attributes_pb2 import ListAttributesRequest

resp = sdk.attributes.list_attributes(ListAttributesRequest(namespace='example.com'))
text = sdk.call('policy.namespaces.NamespaceService', 'ListNamespaces', '{}')
```

The helpers are `sdk.list_attributes(namespace=None)` (protobuf `Attribute`
messages), `sdk.get_entity_attributes(entity)`, `sdk.validate_attributes(fqns)`,
`sdk.attribute_exists(fqn)` and `sdk.attribute_value_exists(fqn)`, each with
an `*_async` twin; `sdk.auth().access_token()` returns the SDK's
`AccessToken` and `sdk.conn()` the core endpoint. The `core_endpoint`,
`policy_endpoint`, `authorization_endpoint`, `entity_resolution_endpoint`
and `wellknown_endpoint` keyword options route those services through the
SDK's transport.

```python
from opentdf_tdf3.protocol.authorization.authorization_pb2 import Entity

fqns = sdk.get_entity_attributes(Entity(id='e1', client_id='my-service'))
sdk.validate_attributes(['https://example.com/attr/level/value/high'])
```

With autoconfigure on (the default), `attributes=` drive the reference's key
split (grants and mapped keys, else the base key, else the default KAS);
`attribute_values=` (protobuf `Value` messages with their definitions) plan
offline. `new_attribute_value_fqn(fqn)` and `new_attribute_name_fqn(fqn)`
return `{'url', 'key'}`.

```python
with sdk.create_tdf(data, attributes=['https://example.com/attr/level/value/high']) as obj: ...
```

## Errors

Every failure is an `OpenTDFError` with `kind`, `op`, `message` (the
reference's wording, see [error-wording.md](error-wording.md)),
`http_status`, `connect_code`, `service_message`, `kas_url`,
`required_obligations` and `assertion_id`. Boundary failures keep the
runtime's kind: `canceled`, `deadline_exceeded`, `closed` (a call after
`close()`), `invalid_argument` (a wrong value type), `poisoned`,
`host_fault`. Switch on `kind`; the kinds are the `ErrorKind` list of
[api-spec.md](api-spec.md).

## CLI

`src/hosts/python/cmd/otdfgc/otdfgc.py` is the native Python CLI over this
wheel, built by `scripts/build-python-cli.sh <package> <launcher>` (a venv
with the locked runtime wheels and the SDK wheel beside the generated
package) and driven by the xtest harness as the `gcpy` SDK type (see
[xtest.md](xtest.md)). It implements the same contract as the Go `otdfgc`:
`encrypt`, `decrypt`, `inspect`, `supports <feature>` and `--version --json`
with the same flags, exit codes (0, 1 operation failed, 2 usage or
`unsupported in this build: <flag>`) and otdfctl's wording.
