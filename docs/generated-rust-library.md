# Generated Rust TDF3 library

The `opentdf-tdf3` Cargo library imports as `opentdf_tdf3` and executes the shared
[value facade](../src/library/library.go). Production calls run generated Rust and
maintained native dependencies without a compiler, Go process or stock SDK.
The [packaged README](../src/hosts/rust/README.md.in) documents the public API;
[compiler ownership](../../goalchemy/docs/rust-library-boundary.md) describes
source isolation and publication.

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

The first command prepares the compiler; the second emits `src/lib.rs`, the
facade and native runtime, verifies dependency archives and notices, builds with
`--locked --offline`, and creates
`goalchemy/out/rust-tdf-library/sdk/target/package/opentdf-tdf3-0.1.0.crate`.
The installer extracts that actual archive and builds a separately importing
native Cargo consumer with `--locked --offline`. It does not import a compiler
checkout. Cargo fetch populates the archive cache before offline builds. The
[Cargo lock](../src/hosts/rust/Cargo.lock) pins 173 external packages;
[dependency inventory](../src/hosts/rust/dependencies.lock.json) records every
archive checksum, license, declared MSRV and 316 retained notice files,
including bundled native notices in separate package/version paths.

The historical final-phase archive SHA-256 is
`08a7093aff4ec2d377921e83de833c4068f4b72ea3f45b7c26aee036785aa5e1`.
The original matrix archive `c68a42c0c464416dd1f000c996d7208f06a9ee3a0b163fb9d86d088c48945aad`
is retained unchanged. The repaired archive has 410 files including source, Cargo.lock, the inventory and notices. It contains
no VCS receipt. The preserved pre-format archive is separate from this final
artifact. These identities belong to the earlier delivery acceptance; fresh
Goalchemy v0.2.0 package identities are recorded with the
[new benchmark receipts](benchmark-results.json). Packaging is local; nothing is published.

| Direct dependency | Role | License |
| --- | --- | --- |
| openssl 0.10.81 | Maintained crypto APIs; vendored OpenSSL 3.6.3 | Apache-2.0 |
| reqwest 0.13.5 | Verified HTTP with rustls and gzip | MIT OR Apache-2.0 |
| tokio 1.53.1 | Dedicated native transport runtime | MIT |
| base64 0.22.1 | Strict canonical encoding | MIT OR Apache-2.0 |
| crc32fast 1.5.2 | Direct IEEE CRC32 API with host-selected CPU acceleration | MIT OR Apache-2.0 |

Transitive rustls uses AWS-LC; reqwest selects a local provider without installing
a process-global default. Full native and transitive license texts are packaged.
`crc32fast` was already present in the transitive inventory and is now also a
direct dependency of the generated SDK; its checksum and notices remain pinned.

## Owned API

```rust,no_run
use opentdf_tdf3::{Config, EncryptOptions, CallOptions, encrypt, decrypt};
let config = Config {
    platform_url: "https://platform.example".into(),
    kas_url: "https://platform.example/kas".into(),
    issuer_url: "https://issuer.example".into(),
    client_id: "server-client".into(),
    client_secret: "read-from-server-storage".into(),
    ..Default::default()
};
let archive = encrypt(config.clone(), vec![0, 255],
    EncryptOptions { include_metadata: true, metadata: vec![], ..Default::default() },
    CallOptions::default()).wait()?;
let result = decrypt(config, archive, CallOptions::default()).wait()?;
assert_eq!(result.Payload, vec![0, 255]);
assert!(result.HasMetadata && result.Metadata.is_empty());
# Ok::<(), opentdf_tdf3::LibraryError>(())
```

`Config`, `KASRoute`, `EncryptOptions` and `AccessToken` own native Rust Strings,
vectors and exact-width integers. `Operation<T>` implements native Future and
provides `wait`, `cancel` and `cancellation`. Recursive owned arguments detach
at submission. Generic Go strings are byte vectors, preserving arbitrary bytes.
Decrypted payload, metadata and manifest are owned; `HasMetadata` distinguishes
absence from encrypted empty metadata. Structured `LibraryError` fields retain
SDK codes, operations, server diagnostics, obligations and cause category;
`diagnostic` retains exact source bytes and `message` supplies display text.
`HostWire` is exported for typed inspection of field values.

Every acquired operation initializes fresh source state on a dedicated 16 MiB
owner. Calls serialize source initialization, execution and retirement. No
source Rc/RefCell/V/H/frame becomes Send/Sync or crosses native threads. The
coordinator holds its entry reservation through owner join and TLS teardown;
owned outputs/errors publish afterward. Library entry does not replace panic
hooks, change process thread policy or install a global TLS crypto provider.

Queued cancellation releases queued arguments independently of a held active
call. Active cancellation and host faults wait for actual native release ACKs.
Drop requests stop for pending operations; wait/await is required to observe
cleanup completion. Dropping a completed operation does not cancel a shared
CallOptions token. Native crypto completes synchronously before its next source
cancellation boundary. A native token callback receives owned bytes and a stop
flag outside runtime locks and must return only after releasing resources.
Reentrant callbacks must not wait for this serialized library. DPoP providers
supply the token scheme, exact expiry and confirmation thumbprint matching an
explicit owned authentication private key.

Crypto implements RSA2048 OAEP SHA1/MGF1SHA1 and RS256, P256 ECDH32 and ES256
P1363, AES256GCM, HKDF/HMAC/SHA256, maintained constant-time MAC comparison and
native randomness. Strict bounded PEM supports SPKI/PKCS8/PKCS1/certificates;
JWK and base64 encodings are canonical. Key aliases share owner-associated
Close state and cleanup. OpenSSL's legacy HMAC API rejects an empty native key;
the adapter supplies the equivalent SHA256 zero-padded block to maintained
HMAC. Independent native Go verifies the empty-key/empty-message result and
verification parity along with the ordinary vectors.

Native HTTP uses verified roots/hostnames, GET/POST, no redirects/retries,
request/response header and body bounds, monotonic deadlines and active stop.
Dedicated transport runtime teardown precedes ACK. OS resolver jobs can delay
settlement until release. Maintained Hyper independently bounds parser buffers;
accepted header bytes are limited to 64 KiB. Generic crypto inputs are limited
to 64 MiB, PEM to 64 KiB, and HKDF output to 8160 bytes. Shared TDF segmentation
and limits remain in the shared implementation.

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
