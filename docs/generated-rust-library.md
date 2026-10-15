# Generated Rust library

The `opentdf-tdf3` Cargo library imports as `opentdf_tdf3`. It is emitted from
the two export roots `src/library` (the deprecated `encrypt`/`decrypt`
façade) and `src` (the handle API of [api-spec.md](api-spec.md)); the
hand-written [host adapter](../src/hosts/rust/sdk.rs.in) presents that API in
Rust idiom and maps names, idioms and errors only. Production calls run
generated Rust and maintained native dependencies without a compiler, Go
process or stock SDK. [Compiler ownership](../../goalchemy/docs/rust-library-boundary.md)
describes source isolation and publication.

## Build and import

The observed platform is Rust/Cargo 1.98.0, Linux x86_64 GNU, with Go 1.25.14
used only to build the compiler. Cargo declares Rust 1.88, the highest declared
MSRV in its locked dependencies; that minimum and other platforms have not been
validated. Native builds need a C compiler, CMake, Make and Perl. OpenSSL and
AWS-LC build from vendored locked archives. Host applications must use
`panic=unwind`; native stack exhaustion, abort/OOM and catastrophic native
failures are outside managed cleanup guarantees.

From the workspace root:

```sh
(cd goalchemy; GOTOOLCHAIN=go1.25.14 go build \
  -o out/rust-tdf-library/goalchemy ./cmd/goalchemy)
bash sdk/scripts/build-generated-rust.sh
bash sdk/tests/interop/generatedrust/install-consumer.sh
```

The first command prepares the compiler; the second emits the generated crate
(`src/generated.rs`), adds `src/lib.rs` (façade), `src/sdk.rs` (handle API),
`src/services.rs` (service clients) and the generated prost messages of
`src/hosts/rust/protocol` as `src/protocol/`,
`GOALCHEMY_ROOT` selecting the Goalchemy checkout, verifies dependency archives and notices, builds with
`--locked --offline`, and creates
`goalchemy/out/rust-tdf-library/sdk/target/package/opentdf-tdf3-0.1.0.crate`.
The installer extracts that actual archive and builds a separately importing
native Cargo consumer with `--locked --offline`. It does not import a compiler
checkout. Cargo fetch populates the archive cache before offline builds. The
[Cargo lock](../src/hosts/rust/Cargo.lock) pins 202 external packages;
[dependency inventory](../src/hosts/rust/dependencies.lock.json) records every
archive checksum, license, declared MSRV and 369 retained notice files,
including bundled native notices in separate package/version paths.

The historical final-phase archive SHA-256 is
`08a7093aff4ec2d377921e83de833c4068f4b72ea3f45b7c26aee036785aa5e1`.
The original matrix archive `c68a42c0c464416dd1f000c996d7208f06a9ee3a0b163fb9d86d088c48945aad`
is retained unchanged. The repaired archive has 410 files including source, Cargo.lock, the inventory and notices. It contains
no VCS receipt. The preserved pre-format archive is separate from this final
artifact. These identities belong to the earlier delivery acceptance; fresh
performance measurements appear in the
[current performance table](../README.md#end-to-end-performance). Packaging is local; nothing is published.

| Direct dependency | Role | License |
| --- | --- | --- |
| openssl 0.10.81 | Maintained crypto APIs; vendored OpenSSL 3.6.3 | Apache-2.0 |
| reqwest 0.13.5 | Verified HTTP with rustls and gzip | MIT OR Apache-2.0 |
| tokio 1.53.1 | Dedicated native transport runtime | MIT |
| base64 0.22.1 | Strict canonical encoding | MIT OR Apache-2.0 |
| crc32fast 1.5.2 | Direct IEEE CRC32 API with host-selected CPU acceleration | MIT OR Apache-2.0 |
| prost 0.14.4 | Protobuf messages of the service clients | Apache-2.0 |
| pbjson 0.8.0, pbjson-types 0.8.0 | Protobuf JSON (serde) for those messages and the well-known types | MIT |
| serde 1.0.229, serde_json 1.0.151 | The JSON codec at the boundary | MIT OR Apache-2.0 |

Transitive rustls uses AWS-LC; reqwest selects a local provider without installing
a process-global default. Full native and transitive license texts are packaged.
`crc32fast` was already present in the transitive inventory and is now also a
direct dependency of the generated SDK; its checksum and notices remain pinned.

## Handle API

```rust,no_run
use opentdf_tdf3::{Sdk, Options, TdfOptions, ReaderOptions, KasInfo, Kind};
let sdk = Sdk::new("http://localhost:8080", Options {
    client_id: "opentdf-sdk".into(), client_secret: "secret".into(),
    insecure_plaintext: true, ..Default::default()
})?;
let object = sdk.create_tdf(&plaintext, &TdfOptions {
    attributes: vec!["https://example.com/attr/attr1/value/value1".into()],
    kas_information: vec![KasInfo::new("http://localhost:8080/kas")],
    mime_type: "text/plain".into(), ..Default::default()
})?;
let manifest = object.manifest()?;          // typed, the reference's JSON tags in snake_case
object.close()?;
let ciphertext = object.bytes;              // or: sdk.encrypt_bytes(&plaintext, &options)?
let reading = ReaderOptions { kas_allowlist: Some(vec!["http://localhost:8080".into()]), ..Default::default() };
let reader = sdk.load_tdf(&ciphertext, &reading)?;
let attributes = reader.data_attributes()?;
reader.init()?;                             // fetch and unwrap keys now
let metadata = reader.unencrypted_metadata()?;
reader.close()?;
match sdk.decrypt_bytes(&ciphertext, &reading) {
    Ok(plain) => {}
    Err(e) if e.kind == Kind::RewrapForbidden => {}
    Err(e) => eprintln!("{} {} {}", e.kind.as_str(), e.http_status, e.message),
}
let plain = sdk.decrypt_bytes_async(&ciphertext, &reading).await?; // dropping the future cancels
sdk.close()?;
# Ok::<(), opentdf_tdf3::Error>(())
```

| Adapter symbol | Reference | Notes |
| --- | --- | --- |
| `Sdk::new(endpoint, Options)`, `Sdk::new_async` | `New` | Fetches `/.well-known/opentdf-configuration` and the issuer's OpenID configuration unless `platform_configuration` is given; `validate_connection` runs the health check first. |
| `sdk.create_tdf(&[u8], &TdfOptions)` → `TdfObject` (`bytes`, `manifest`, `size`, `close`), `sdk.encrypt_bytes` | `CreateTDFContext` over bytes, `TDFObject` | Single-KAS templates; RSA-2048 and P-256 wrapping. Streams arrive in Phase 7. |
| `sdk.load_tdf(&[u8], &ReaderOptions)` → `Reader` (`init`, `manifest`, `policy`, `data_attributes`, `unencrypted_metadata`, `close`), `sdk.decrypt_bytes` | `LoadTDF`, `Reader`, `DecryptBytes` | The KAS allowlist comes from the registry unless `kas_allowlist` or `ignore_allowlist` is set, as in the reference. |
| `sdk.is_healthy`, `platform_configuration`, `get_base_key`, `store_kas_keys`, `close` | same | `platform_configuration()` returns a `BTreeMap<String, String>` (non-string members as compact JSON). |
| `is_valid_tdf`, `get_tdf_type`, `is_platform_endpoint_malformed`, `key_type_to_policy_algorithm`, `policy_algorithm_to_key_type`, `version`, `tdf_spec_version`, `supported_features` | same | Free functions; each is a library call. |

Every method returns `Result<_, opentdf_tdf3::Error>`; `create_tdf`,
`load_tdf`, `decrypt_bytes`, `encrypt_bytes`, `Reader::init` and
`unencrypted_metadata` and `Sdk::new` have `*_async` twins returning a
`Future`, and dropping that future cancels the library call. Options are
structs with `Default` (`Options`, `TdfOptions`, `ReaderOptions`, `KasInfo`,
`TokenExchange`, `CertExchange`, `TlsOptions`) using the shared field names
in snake_case; every default is the reference default. Strings are UTF-8
`String`s, sizes `i64`, payloads `&[u8]`/`Vec<u8>`. `token_provider` takes a
`Provider` built with `token_provider(|request| ...)` and replaces client
credentials; the SDK then signs rewrap requests with the reference's
ephemeral ES256 key unless `dpop_key_pem` is set. Options this build does
not implement yet (`token_exchange`, `cert_exchange`, `tls`, `unix_socket`,
service endpoint overrides, assertions, target mode `4.2.2`, schema
validation, RSA-4096 and P-384/P-521 keys) fail with
`Kind::UnsupportedFeature` and the message `unsupported in this build:
<option>`. `Manifest`, `PolicyObject`, `KeyAccess`, `SimpleKasKey` and the
other manifest types are typed structs whose fields are the reference's JSON
tags in snake_case (`encryption_information.key_access[0].url`,
`payload.mime_type`).

## Service clients

`sdk.attributes()`, `sdk.namespaces()` and the other accessors return clients
named after the reference's `sdkconnect` interfaces (`AttributesServiceClient`,
`NamespaceServiceClient`, ...; the unsafe service is `sdk.r#unsafe()`) over the
prost messages of `opentdf_tdf3::protocol` (`policy::attributes::...`). Each
RPC is a snake_case method returning `Result` with an `*_async` twin. The
messages are generated by `protoc-gen-prost` 0.5.0 and their protobuf JSON
serde implementations by `protoc-gen-prost-serde` 0.4.0 (pbjson, unknown
fields ignored), the prost ecosystem's standard pair (buf's
`neoeinstein-prost` and `neoeinstein-prost-serde` plugins); well-known types
come from `pbjson-types`. Messages cross as JSON through `sdk.call` and the
SDK's authenticated transport; a failure the service reports is an `Error`
with `connect_code`.

```rust,no_run
use opentdf_tdf3::protocol::policy::attributes::ListAttributesRequest;
let attrs = sdk.attributes().list_attributes(&ListAttributesRequest { namespace: "example.com".into(), ..Default::default() })?;
let json = sdk.call("policy.namespaces.NamespaceService", "ListNamespaces", "{}")?;
# Ok::<(), opentdf_tdf3::Error>(())
```

Every failure is an `Error` with `kind` (a `Kind` enum; `Kind::Boundary`
carries the runtime's kind for `closed`, `poisoned`, `invalid_argument`,
`source_panic`, `host_fault`), `op`, `message` (the reference's wording, see
[error-wording.md](error-wording.md)), `http_status`, `connect_code`,
`service_message`, `kas_url`, `required_obligations` and `assertion_id`.

The deprecated façade (`encrypt`, `decrypt`, `Config`, `EncryptOptions`,
`Decrypted`, `Operation`, `LibraryError`) keeps its previous shape for one
release, implemented in the shared source as thin wrappers that construct an
SDK per call; its `Code` field is now the `Error.kind` string and empty
metadata writes no `encryptedMetadata` entry, as the reference does.

`src/hosts/rust/cmd/otdfgc/main.rs` is the native Rust CLI over this crate,
built by `scripts/build-rust-cli.sh <package> <binary>` and driven by the
xtest harness as the `gcrs` SDK type (see [xtest.md](xtest.md)); it
implements the Go `otdfgc` contract (`inspect` loads the manifest through the
handle API with a provided platform configuration). The consumer in
`tests/interop/generatedrust/main.rs` drives the real platform and, in
`controlled` mode, the offline checks of
`tests/interop/delivery/controlled_platform.py`. The verification evidence
below predates the handle API and was gathered through the façade.

## Verification and interoperability

Focused evidence is retained under `.local/rust-tdf-library/`: strict native
capability conformance 203/203; an independent importing generic Cargo consumer
with 102 ownership/value/crypto checks and native Go interchange; compiler,
Rust language, runtime and contract checks; 24 std-only host-operation and four
byte-storage probes; catalog validity and 644 current generated files. The
installed final archive passes native Config/Vec/Result source encryption and
malformed-archive recovery. Exact arbitrary panic diagnostic bytes, int64
limits, initialization isolation, failed-import pointer nil, key aliases and
Close, queued/active provider release, shared cancellation and native Futures
are covered. Original failing runs and their bounded repairs remain preserved.

BASIC real KAS receipts cover all seven canonical cases, both stock references
in both directions, 35 comparisons, the real native OAuth provider, ownership
and cancellation recovery, and 11 typed zero-plaintext rejections. Eleven
controlled actual-SDK transport negatives exercise 401/403, redirect leakage,
content type, malformed/oversize/truncated bodies, untrusted TLS certificates,
BOM routing, deadline and active cancellation. No positive mock KAS is used;
deadline/cancel receipts observe socket release before settlement.

Metadata presence is asserted against each immutable producer manifest's
nonempty `encryptedMetadata` field. Stock Go omits encrypted metadata when given
an empty string; its empty-metadata archive therefore has no metadata. Stock
Web encrypts empty metadata even for its binary case. Generated Rust honors
explicit presence. Stock Web's reader does not expose decrypted metadata.

EC real KAS evidence covers 280 comparisons and eight policy denials. Each case
runs RSA2048/P256 wrapping, RSA2048/P256 response sessions and RS256/ES256
authentication, with explicit imported keys/kids and discovery. Both stock
references produce and consume actual TDF formats; generated decryption checks
native metadata presence against producer archives.

Enforced DPoP receipts cover 226 comparisons and ten negatives, including
matching native providers for RS256 and ES256 without source credentials and
rejection of a mismatched provider authentication key. Generated operations
succeed with the actual enforced nonce profile. Four stock Web decrypt attempts
fail with authentication 401 and are recorded as limitations, never successful
interop. Independently retained stock Go/Web archives produced under Bearer
are decrypted with generated enforced DPoP authentication. Root restored and
verified BASIC after all three matrices. No worker switched profiles or
committed changes; acceptance and signed phase commits belong to root.

Cargo normalizes the archive's manifest and sorts the renamed root package in
its lock. The packaged lock SHA-256 is
`aebf1c1830e500152f470986e060ddef92caa09b164881c909a08f1d09d75430`;
the tracked/build lock is
`d11584bca466ec8259c66b22f5dfe897f67592c5fc5360d1d12c1accbc35c77f`.
Every lock record is identical when sorted; no dependency changes occur.
The final audit verifies every extracted archive member, generated Rust source,
authored runtime member and retained notice against the delivered bytes.


## Bounded acceptance repairs

Native arrays/slices now check their length against the source u32 descriptor
before element conversion or source allocation. Unrepresentable lengths return
InvalidArgument without source success. Every descriptor-representable length
remains supported subject to existing operation limits. Independent native
64-bit proofs reject a valid zero-sized exported-record vector of length
4,294,967,296, nested and fixed-array forms, and recover with ordinary byte and
recursive-value calls.

Imported RSA public exponents must be positive odd integers at least three and
fit the native Go integer width. Private RSA and P256 imports use maintained
OpenSSL private-key consistency checks before extracting public material. Valid
e3 public keys, ordinary RSA/P256 imports and PKCS8 with omitted public point
remain supported. The original Rust parser accepted malformed exponent, private
CRT/prime, zero-scalar and encoded-point fixtures; the repaired importer returns
the declared invalid error and nil key. Independent native Go rejects six
parity fixtures.

Go 1.25's ASN.1 parsers normalize some encoded private material: a changed RSA D
with intact valid CRT can be accepted; an EC public point is reconstructed from
the scalar, ignoring an inconsistent encoded point. The malformed RSA fixture
therefore includes inconsistent CRT and the EC fixture includes a separate
invalid-scalar parity negative. Rust explicitly rejects mathematically
inconsistent encoded D/Q using maintained checks, with independent Go curve
operations proving the inconsistency. This representation validation difference
is recorded separately from native-Go rejection parity.

The repair supplement passes 136 native boundary checks. Original source freeze,
crate and BASIC/EC/DPoP receipts remain immutable. The two-area repair package member delta
is confined to generated collection conversion, library length validation and
private-key validation. Positive KAS evidence uses representable vectors and
valid keys; the narrowed invalid-input paths do not change those protocol or
transport results. A separately extracted repaired package is rebuilt and smoke
tested; no extra KAS matrix is counted or claimed.


The same acceptance review also covers pure generic value libraries. Library
emission includes its native crypto cleanup dependency even when source exports
use no crypto. Native-only runtime module reexports follow their feature guards;
std-only capabilities stay available. An independently importing pure byte Echo
consumer passes 40 calls with default native features and 40 with
`--no-default-features`; a std-only generated executable also passes. The final
archive differs from the preserved two-area repair archive only in
`src/rt/mod.rs` feature attributes. Default native bindings are identical; the
original and two-area source/package/proof freezes remain separately retained.
