# Local Python buffer optimization

The local candidate reduces 50 MiB Python E2E time by 37.9% compared with a fresh
unchanged-package baseline. Both repositories use local
`perf/python-buffer-handling-local` branches. No changes or packages were pushed
or published; the README benchmark table remains the released v0.2.0 campaign.

| Python E2E median | 1 MiB | 10 MiB | 50 MiB |
| --- | ---: | ---: | ---: |
| Fresh v0.2.0 baseline | 159.72 ms | 266.69 ms | 1144.60 ms |
| Local optimized candidate | 155.30 ms | 199.59 ms | 710.35 ms |
| Reduction in median time | 2.8% | 25.2% | 37.9% |

Each cell has one warmup and five timed pairs. The unchanged
[Python harness](../tests/bench/python.py) times contiguous public encryption,
same-archive real-KAS decryption and complete plaintext materialization. Fixture
loading, configuration, OAuth, KAS public-key discovery, file I/O and independent
validation remain outside timing. Segment size is 2 MiB, with GMAC, RSA-2048
wrapping/response sessions, ES256 request signing and Bearer authentication.

The original 113-member installed package and the fresh candidate live in separate
virtual environments. Candidate native cryptography is byte-identical to the
baseline, so the crypto dependency did not change. Every one of the 36 retained
warmup/measured archives passed independent stock-Go real-KAS decryption, exact
plaintext comparison and ZIP CRC validation. Raw samples remain in ignored
`.local/benchmarks/python-buffer-before-2026-10-03/` and
`.local/benchmarks/python-buffer-after-2026-10-03/`. Small-file sample ranges
overlap; the 1 MiB difference is within observed run variation. These five-sample
medians are local observations, not a steady-state or cross-machine guarantee.

## Changes and ownership

[Client decryption](../client.go) prepares an opaque, owned archive once through
[the engine](../tdf/engine.go). ZIP CRC checks, manifest parsing and supported
profile validation still precede token acquisition and KAS routing. Recovered
keys authenticate policy binding, root, every segment and metadata before any
plaintext is returned. Failed preparation and zero-value prepared objects cannot
decrypt. Public manifest snapshots and returned plaintext/metadata cannot mutate
cached state. `DecryptWithPayloadKey` retains its complete validating entry path.

The Python facade retains immutable `bytes` inputs and takes bounded immutable
snapshots of mutable inputs at submission. The generated library also owns queued
typed, multidimensional and strided views before the caller can mutate them.
Generated output, native crypto input and string conversion use memoryviews to
avoid intermediate bytearray slices while still producing independent results.
Internal byte copy/append reads distinct backing through views; same-backing
operations retain snapshots for overlap safety. Native source execution still
receives owned mutable bytearray storage.

## Verification and local source identity

Focused checks passed 14 top-level tests and 47 subtests with zero skips. They
cover precredential error staging, CRC/profile rejection, late-segment and
metadata authentication failure, cached-state mutation, slice overlap, queued
input snapshots, cancellation and host lifecycle. Native primitive checks passed
133 assertions, generated library checks 71 and installed SDK boundary checks 31.
The fresh wheel is reproducible across relative/absolute build paths and matches
all 113 actual installed package members. The BASIC real-KAS matrix passed 35
comparisons across self round trips and both reference directions, plus native
provider/ownership checks and 11 rejection categories.

The changed shared SDK compiles/emits for all seven targets. This follow-up did
not rebuild non-Python native packages or replay EC/DPoP matrices; earlier
acceptance for those configurations is separately recorded historical evidence.

Goalchemy local commit is `4f5b428bf662a09991e05140484d4380e0401513`.
The candidate binary still reports compiler version 0.2.0, but it includes the
local runtime changes and is not the published v0.2.0 binary. Candidate compiler
SHA256 is `215efc0c817497593c76c8f4cad68cab448d0101925ed84dbd0e60c972a5a0f5`;
wheel SHA256 is `8aad9e095a596077d891782e4c4b28bd43aeb4ba0bebc3dbc1424072b296625b`.
Root verified the 16-file producing-source/test freeze, actual test events,
installed files, matrix command statuses, raw medians and all benchmark archive
hashes. Detailed receipts stay in ignored `.local/python-buffer-local/`.
Reference/CI pins point to the unpublished local compiler commit; hosted CI was
not triggered. Pin updates and these documentation changes occurred after timing
and do not change the measured producing source.
