# Shared TDF format foundation

This is the first bounded part of Phase 3, providing ordinary Go packages accepted by Goalchemy's sequential gate. It does not encrypt, authenticate, contact KAS, or establish complete SDK interoperability. `opentdf-local/sdk` is a local development module identity, without a publication claim. The [protocol audit](tdf-poc.md) and [compatibility inventory](compatibility.md) retain the full eventual SDK scope.

## JSON

`tdf/json.Parse` and `Marshal` take explicit `Limits`. Defaults are 10 MiB input/output, depth 32 (root depth zero), 100,000 value nodes, and 10 MiB decoded bytes per string. Depth limits above 128 fail. Byte/node/depth bounds apply before unbounded allocation/recursion. Object names consume byte bounds. Duplicate checks use per-object maps; ordered `Names` correspond to ordered `Children`.

Strings require valid UTF-8, with correct escape decoding and UTF-16 surrogate pairs. Invalid/lone surrogates, overlong UTF-8, surrogate UTF-8, raw controls, invalid escapes, and truncation fail. Unicode is not normalized; decoded string bytes and embedded NUL are preserved. The deterministic compact serializer escapes quotes, backslashes and controls, validates UTF-8, and retains member order. It is not canonical JSON for assertion signing.

Generic numbers retain exact fractional/exponent lexemes without floating point. `Value.Integer(max)` accepts only valid nonnegative integer lexemes, including caller-constructed values, and checks the bound before multiplication. Manifest sizes reject exponent/fractional, negative and overflowing numbers. Parsing rejects decoded duplicate member names, trailing input, trailing commas and truncation. Serialization rejects malformed numbers, duplicate names, invalid object shape and unknown kinds. Escaping can expand output beyond the input byte length; output limits apply independently.

## ZIP

`tdf.WriteArchive(payload, manifest, limits)` returns fresh deterministic stored ZIP32 bytes with `0.payload` and `0.manifest.json`. It emits known sizes/CRCs, UTF-8 filename flags, no descriptors and the fixed DOS date 1980-01-01. This is bounded in-memory writing; streaming and ZIP64 emission remain later work.

`ReadArchive` accepts single-disk stored ZIP32/ZIP64, signed/unsigned 32/64-bit descriptors, ZIP64 extra sizes/offsets, ZIP64 EOCD/locator records and ordinary EOCD comments. It accepts the actual pinned Go/Web writer layouts, including Web's ZIP64 local extraction version 20. It requires matching central/local names, flags, method, sizes and CRCs. Physical local records must cover bytes from offset zero to the central directory exactly, without gaps, overlaps, aliased offsets or preambles. Counts, extents, descriptors and ZIP64 end metadata are checked before slicing. Offset lookup and physical traversal are linear in entry count.

Descriptor parsing evaluates signed/unsigned candidates against CRC, sizes and the next physical boundary. An unsigned descriptor whose CRC equals `0x08074b50` is supported. Go `archive/zip` itself misinterprets that case; the native test validates its independently constructed four-byte payload through `OpenRaw` and standard-library CRC. Ordinary fixtures are completely read through `archive/zip.Open`.

Default bounds are 128 MiB archive, 64 MiB payload, 10 MiB manifest and 64 entries. Archive bounds cannot exceed 2,147,483,647 bytes; entry bounds cannot exceed 65,534. ZIP64 values with a nonzero high 32-bit word or exceeding configured bounds fail. Every entry CRC is checked, including auxiliary entries. Returned payload/manifest bytes are independent copies. CRC is ZIP bookkeeping, without a cryptographic integrity claim.

`manifest.json` takes precedence over `0.manifest.json` in either physical order. Duplicate names are rejected. A present invalid/oversized preferred entry fails without fallback. Names are limited to 1,024 printable ASCII bytes, without backslashes or absolute paths. Files are never extracted to disk. Compression, ZIP encryption, multipart ZIP, arbitrary UTF-8 auxiliary names, HTML/self-extracting wrappers, streaming and larger archives remain explicit later parity work.

## Manifest and policy

`ParseManifest` provides typed payload, encryption, KAO, method, integrity, root-signature and segment fields. Bounds are 64 KAOs, 10,000 segments and 1,000 assertions. `Segment.HasSize` and `HasEncryptedSize` distinguish omission from explicit zero. `Integrity.ResolveSegment` substitutes defaults independently and requires framed ciphertext size to equal plaintext plus 28, with plaintext at most 4 MiB.

Parsing is inspection. Before engine use, call `Manifest.ValidateModern(payloadBytes)`. `Manifest.Raw` and `Policy.Raw` preserve the entire parsed tree, including unknown required-feature declarations; `Assertions` preserves complete JSON values. Serializers reject unknown fields rather than discarding them. Known unsupported variants can be inspected/serialized, but profile validation rejects them. Typed serializers check array bounds before constructing output trees.

`Manifest.Encryption.Policy` retains the exact decoded JSON string bytes containing base64. `DecodePolicy` creates a separate view without changing that string. Binding and rewrap must use the original base64 string, regardless of whitespace/member order in decoded policy JSON. `Policy.Marshal` creates new policies and must not reconstruct an already-bound policy. The pinned Go writer emits `dataAttributes:null` and `dissem:null` when there are no attributes; these are accepted as empty arrays without changing the original encoded policy. Missing array fields and non-array/non-null types fail.

The deliberately narrow modern validation profile requires:

- Schema `4.3.0`, encrypted reference `0.payload`, ZIP protocol, split encryption and streamable `AES-256-GCM`.
- Exactly one KAO using `kas` protocol, nonempty URL, optional schema absent or `1.0`, typed case-insensitive HS256 binding whose standard-base64 value encodes 64 lowercase ASCII hex bytes. `kid` and `sid` are preserved for grouped requests; one nonempty SID is permitted.
- RSA `wrapped` ciphertext exactly 256 decoded bytes, without an ephemeral key; or `ec-wrapped` frame exactly 60 decoded bytes with a nonempty ephemeral public key. PEM/curve and KAS destination authorization checks belong to the later engine.
- Explicit case-insensitive HS256 root with a 32-byte signature. GMAC segment hashes decode to 16 bytes; HS256 hashes decode to 32. Missing/empty segment algorithm falls back to root, matching Web; unknown algorithms and GMAC roots fail. Root-algorithm omission is rejected rather than reproducing historical Go defaults.
- Positive plaintext defaults at most 4 MiB, ciphertext defaults exactly 28 bytes larger, consistent segment pairs, and ciphertext sizes summing exactly to the supplied payload length (at most 64 MiB). Explicit zero plaintext with explicit 28-byte ciphertext is valid. Web zero segments/payload and Go one encrypted empty segment both work.
- Policy UUID nonempty and at most 128 bytes, bounded attribute/dissemination arrays and nonempty attribute names. Display names, default flags, public keys and KAS URLs are preserved. Attribute semantics/authorization remain later engine work.

Legacy/missing/alternate versions, legacy string bindings, raw-digest policy bindings, all assertions, multiple KAOs/shares, remote/hybrid key access, unknown fields (including obligations), and additional payload/encryption/signature profiles fail usefully. These initial exclusions remain required later parity work. Encryption/key-access profile names remain case-sensitive; integrity/binding algorithm names tolerate case differences.

`ParseEncryptedMetadata` handles Web's default empty metadata frame. Encoded input is at most 1 MiB and decodes base64 JSON `{ciphertext,iv}`. Ciphertext is a complete frame of at least 28 bytes; the separately decoded 12-byte IV must equal the frame prefix. No metadata decryption/authentication occurs. Nonempty method IV is separately checked as 12 bytes and never substituted for payload nonces. Larger metadata profiles remain later work.

## Verification

From `sdk/`:

```sh
GOTOOLCHAIN=go1.25.14 GOCACHE=$PWD/.local/go-build-cache GOMODCACHE=$PWD/.local/go-mod-cache go test -race ./tdf/...
GOTOOLCHAIN=go1.25.14 .local/bin/goalchemy check ./tdf/... ./tests/sourcecheck/format
GOTOOLCHAIN=go1.25.14 .local/bin/goalchemy compile -target go -out .local/format-probe/go ./tests/sourcecheck/format
GOTOOLCHAIN=go1.25.14 go run ./tests/sourcecheck/reference .local/interop
```

Run `GOTOOLCHAIN=go1.25.14 go run .` inside `.local/format-probe/go/`. That emitted probe executes Unicode/NUL JSON, ZIP binary bytes/CRC, policy and typed manifest serialization, and explicit-empty segment resolution. It demonstrates codec emission, not generated SDK encryption/networking.

The native reference runner requires all six existing Phase 1 `{small,binary,empty}.{go,web}.tdf` files; missing files fail. It reads every entry independently with `archive/zip`, compares exact bytes, validates typed manifests and preserves policy strings across manifest serialization. Fixtures stay in ignored local storage, separate from offline tests. No ciphertext is decrypted and no KAS calls occur in this runner.

Offline tests use independent standard-library JSON/ZIP/CRC oracles, unsigned/signed ZIP32/ZIP64 fixtures, truncation/offset/size/CRC/compression negatives, manifest/policy vectors, unsupported required fields and metadata framing. Fuzz targets cover malformed bounded JSON/archives. See [Go models](../../platform/sdk/manifest.go), [Go ZIP primitives](../../platform/sdk/internal/zipstream/zip_primitives.go), [Go selection](../../platform/sdk/internal/zipstream/tdf3_reader.go), [Web ZIP](../../web-sdk/lib/tdf3/src/utils/zip-writer.ts), [Web TDF](../../web-sdk/lib/tdf3/src/tdf.ts) and [Web metadata](../../web-sdk/lib/tdf3/src/models/encryption-information.ts) for pinned protocol evidence.

Next work must authenticate root/segments, encrypt/decrypt payload/metadata, reconstruct keys, validate configured KAS destinations, authenticate, rewrap and run real bidirectional SDK interoperability before Phase 3 acceptance.
