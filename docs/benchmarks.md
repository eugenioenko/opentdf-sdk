# Native SDK end-to-end benchmark

The README table reports fresh **1 MiB, 10 MiB and 50 MiB** measurements
from the 2026-10-04 steady-state campaign. Each cell is the median elapsed
**milliseconds** across 15 complete encrypt/decrypt pairs: five samples in each
of three fresh processes. Every process first performs 40 full pairs at 50 MiB
and 20 pairs at its measured size. All seven generated native packages are
rebuilt with released Goalchemy **v0.2.1**, commit
`de26e4aa18f38f75bdf7a43c7b19b33cfaec9673`. The pinned original OpenTDF Go SDK
is the eighth implementation. Earlier timing cells and diagnostic profiles are
excluded. SDK implementation and compiler/runtime source remain unchanged.

## Reproduce

Use adjacent checkouts at [references.lock.json](../references.lock.json), the
[documented native prerequisites](final-delivery.md), and the BASIC platform
profile with real KAS at `http://localhost:8080/kas` and local IdP at port 8888.
The runner does not start or reconfigure services. Set the local Keycloak
`opentdf-sdk` client attribute `access.token.lifespan` to `21600` seconds
before running; acquire a new token and confirm its lifetime. Build fresh packages and
installed consumers before timing, using a new package directory:

```sh
mkdir -p .local/steady-state-reproduction/compiler
(cd ../goalchemy && GOTOOLCHAIN=go1.25.14 go build -trimpath \
  -o ../sdk/.local/steady-state-reproduction/compiler/goalchemy ./cmd/goalchemy)
python3 scripts/delivery-packages.py all \
  --base .local/steady-state-reproduction/delivery \
  --compiler .local/steady-state-reproduction/compiler/goalchemy \
  --compiler-revision de26e4aa18f38f75bdf7a43c7b19b33cfaec9673
python3 scripts/delivery-consumers.py all \
  --base .local/steady-state-reproduction/delivery
python3 scripts/benchmark-sdk.py \
  --packages .local/steady-state-reproduction/delivery \
  --output .local/steady-state-reproduction/campaign \
  --sizes 1MiB,10MiB,50MiB --samples 5 --batches 3 \
  --warmups 20 --bulk-warmups 40 --fresh
```

Package receipts record the actual compiler binary and package/member hashes.
SDK package versions remain 0.1.0. Both relative and absolute output-path builds
must pass before installed consumers or measurements run. `--fresh` rejects an
existing raw-results file; choose a new output directory for a new campaign.
`--build-only` prepares native benchmark consumers; `--skip-build` uses those
consumers without rebuilding.

Rust links the installed consumer's compiled SDK rlib. C, Java and C# link
installed native archives, JARs and assemblies. Go imports the installed module;
Node imports its native entry and Python its installed wheel. The reference
consumer imports the pinned `../platform/sdk` public API. Both benchmark Go
binaries use Go 1.25.14 with the matching GOROOT.

## Warmup and normal runtime settings

Warmup and measurement run in the same process. The fixed policy was selected
before the campaign using separate Java pilots. A normal-JVM 50 MiB pilot with
240 warmups showed the slow path in pairs 1–7, followed by a drop to the faster
path around pairs 8–12. Three fresh normal JVMs then checked the proposed
40-pair bulk warmup followed by 20 size-specific pairs at 1, 10 and 50 MiB.
A separate JFR run examined compilation; its timings are excluded from the
README table. A flat short timing window alone was insufficient evidence of
completed warmup in the earlier investigation.

The same fixed bulk and size-specific warmup applies to all implementations.
Bulk warmup exercises the full segmentation, crypto and allocation paths before
the smaller size is measured. This intentionally reports performance after
substantial use, rather than startup or the first few calls. Three fresh
processes expose variation between runtime instances. Complete warmup histories
and each batch's five samples are retained; the table pools all 15 samples,
without selecting the fastest process or discarding slow samples.

Final timings use normal runtime settings: no compiler-threshold flags, JFR,
intrinsic diagnostics, forced GC, allocator reset, CPU pinning or heap tuning.
Targets run sequentially on one AMD Ryzen 7 6800H Linux x86_64 host without
competing SDK builds or benchmarks. Natural allocation/GC, RSA randomness,
local KAS and scheduling variation remain. Browser execution is excluded.

## Setup and lifecycle

Original Go `SDK.New` runs **once before warmup** in each fresh process. Its
RSA2048 response-session key and ES256 signer are reused throughout that batch.
`CreateTDF`, `LoadTDF` and `io.ReadAll` remain inside each interval. `LoadTDF`
uses the constructor's default session key; the wrapper does not request
`WithSessionKeyType`. Client `Close` runs outside timing after the loop.

Every host prepares public configuration, token providers and encryption
options outside the timer. Generated APIs expose stateless facades, so their
internal per-operation client/signing-key setup and per-decrypt RSA2048 session
creation remain timed. Their key lifecycles differ from the reference SDK.
No API changes were made to equalize those lifecycles.

OAuth tokens and the public KAS PEM/kid are acquired before each batch. The
local Keycloak `opentdf-sdk` client uses a six-hour access-token lifetime via
its `access.token.lifespan` client attribute, verified both from the token
response and the JWT lifetime. The harness uses that pre-acquired token
throughout each process; no token refresh or per-pair token-file reads are
needed. This development setting is local to the benchmark client.
The original uses `oauth2.StaticTokenSource`; generated native providers return
the pre-acquired Bearer token during public calls. OAuth acquisition and
public-key discovery are untimed; SDK key import and internal authentication
work remain timed. No private-key PEM is supplied.
All cells use RSA2048 wrapping and response sessions, ES256 signing, GMAC segment
integrity, 2 MiB segments, the same permitted attribute, and no metadata or
compression.

## Timing and correctness

Each process preloads deterministic input and uses its native monotonic clock.
A single contiguous timer covers public encryption followed by decryption of
that same new archive, including owned result copies, crypto, policy/key
wrapping, ZIP construction and parsing, a real KAS request and unwrap, integrity
checks, and complete plaintext materialization. File I/O, process/module
startup, harness setup, exact plaintext checking and result disposal are outside
timing.

Rust's API consumes `Vec<u8>` inputs. Fresh owned input, configuration/options
clones and provider registries are prepared before each timer. Retaining the
new archive for independent validation requires an archive clone for the
consuming decrypt call **inside** the interval; this cost remains included.

Every warmup and measured pair compares the full plaintext against its input
after timing. Only the final size-specific warmup archive and all five measured
archives are retained per batch. Each retained archive is independently
decrypted by stock Go through real KAS outside timing, then checked exactly.
Python's standard ZIP reader separately reads both members to EOF to verify
CRC values. Manifest checks verify AES-256-GCM, GMAC and the expected segment
count. Warmup pairs whose archives are not retained receive the native full
plaintext check; they are not described as independently validated archives.

Inputs are deterministic binary bytes: byte `i` is
`(i * 131 + (i >> 8) * 17) & 255`. The three sizes use one, five and 25 segments.
The original reused session key does not cache plaintext keys: every pair
still requires KAS rewrap.

The measured packages delegate IEEE CRC32 to Go `hash/crc32`, Java
`java.util.zip.CRC32`, Python `zlib.crc32` and Node `node:zlib.crc32`. C# deploys
Microsoft's **System.IO.Hashing 8.0.0** and uses `Crc32.HashToUInt32`; Rust depends
on **crc32fast 1.5.2**. C and browser TypeScript retain slicing-by-8 fallback.
Acceleration is selected by the runtime or library; these timings do not prove
that a particular hardware instruction executed.

## Receipts and limits

Safe samples, warmup histories, runtime settings, package/member hashes,
source identities and independent archive checks are in
[benchmark-results.json](benchmark-results.json). The ignored campaign directory
`.local/steady-state-2026-10-04/campaign` retains raw receipts, summaries,
environment metadata, retained archives, fixtures and compiled consumers.
Private token files remain ignored with mode 0600 and are not exported.

These are local steady-state E2E timings, including KAS service and RSA
variation. Fifteen samples per cell do not establish long-tail latency or a
universal language ranking. [Earlier Go profiles](go-profiles.md) and the
[historical buffer comparison](all-target-buffer-benchmarks.md) use different
campaigns and are not samples from this table. Full interoperability/CI matrices
were not repeated for these benchmark-only changes.
