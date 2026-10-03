# Native SDK end-to-end benchmark

The README table contains only fresh **1 MiB, 10 MiB and 50 MiB** measurements
from the 2026-10-03 campaign. Each cell reports median elapsed **milliseconds**
from five complete encrypt/decrypt pairs after one warmup. All seven generated
native packages were rebuilt with released Goalchemy **v0.2.0**, commit
`90b1a019bd6def8ad59ebcf7f5bc0e8487d77bab`. The original OpenTDF Go SDK is the
eighth implementation. Earlier sample cells are not reused.

## Reproduce

Use adjacent checkouts at [references.lock.json](../references.lock.json), the
[documented native prerequisites](final-delivery.md), and the BASIC platform
profile with real KAS at `http://localhost:8080/kas` and local IdP at port 8888.
The runner does not start or reconfigure services. Build packages and installed
consumers first, using a new package directory:

```sh
mkdir -p .local/v020/compiler
(cd ../goalchemy && GOTOOLCHAIN=go1.25.14 go build -trimpath \
  -o ../sdk/.local/v020/compiler/goalchemy ./cmd/goalchemy)
python3 scripts/delivery-packages.py all --base .local/v020/delivery \
  --compiler .local/v020/compiler/goalchemy \
  --compiler-revision 90b1a019bd6def8ad59ebcf7f5bc0e8487d77bab
python3 scripts/delivery-consumers.py all --base .local/v020/delivery
python3 scripts/benchmark-sdk.py --packages .local/v020/delivery \
  --output .local/benchmarks/e2e-v020-reproduction \
  --sizes 1MiB,10MiB,50MiB --samples 5 --fresh
```

The measured campaign used the released compiler binary with SHA256
`61dec71c6b3dde1634f598f4476608b703ce965cecc33912afd3fc930e22040b`.
A local rebuild can have a different binary hash; package receipts record its
actual identity. SDK package versions remain 0.1.0. Both relative and absolute
output-path builds must pass before consumers or measurements run.

The defaults select all eight implementations and `1MiB,10MiB,50MiB`.
`--fresh` rejects an existing raw-results file; always choose a new output
directory for a new campaign. `--build-only` prepares native benchmark consumers;
`--skip-build --fresh` runs those consumers without rebuilding. The completed
campaign used explicit `--packages .local/v020/delivery`. The later correction
of that CLI default does not affect its recorded controller or timings.

Rust links the installed consumer's compiled SDK rlib. C, Java and C# link
installed native archives, JARs and assemblies. Go imports the installed module;
Node imports its native entry and Python its installed wheel. The reference
consumer imports the pinned `../platform/sdk` public API. Both benchmark Go
binaries use Go 1.25.1; the package compiler was built with Go 1.25.14.

## Setup and lifecycle

Original Go `SDK.New` runs **once before the warmup and timing loop** for each
size cell. Its RSA2048 response-session key and ES256 signer are reused across
all six pairs. `CreateTDF`, `LoadTDF` and `io.ReadAll` remain inside each interval.
`LoadTDF` uses the constructor's default session key; the wrapper does not
request `WithSessionKeyType`. Client `Close` runs outside timing after the loop.

Every host prepares public configuration, token providers and encryption
options outside the timer. Generated APIs expose stateless facades, so their
internal per-operation client/signing-key setup and per-decrypt RSA2048 session
creation remain timed. This compares the supported public APIs with harness
setup removed; their key lifecycles differ. No API changes were made to equalize
those lifecycles.

OAuth tokens and the public KAS PEM/kid are acquired before each host batch.
The original uses `oauth2.StaticTokenSource`; generated native providers return
the same externally acquired Bearer token during public calls. OAuth acquisition
and public-key discovery are untimed; SDK key import and internal authentication
work remain timed. No private-key PEM is supplied. All cells use RSA2048 wrapping
and response sessions, ES256 signing, GMAC segment integrity, 2 MiB segments,
the same permitted attribute, and no metadata or compression.

## Timing and correctness

Each process preloads deterministic input, performs one untimed warmup, then
records five pairs with its native monotonic clock. A single contiguous timer
covers public encryption followed by decryption of that same new archive,
including owned result copies, crypto, policy/key wrapping, ZIP construction
and parsing, a real KAS request and unwrap, integrity checks, and complete
plaintext materialization. File I/O, process/module startup, harness setup,
exact plaintext checking and result disposal are outside timing.

Rust's API consumes `Vec<u8>` inputs. Fresh owned input, configuration/options
clones and provider registries are prepared before each timer. Retaining the
new archive for independent validation requires an archive clone for the
consuming decrypt call **inside** the interval; this cost remains included.

Every pair compares the full plaintext against its input after timing. All
**144 archives** (24 cells, 120 measured pairs and 24 warmups) are retained and
independently decrypted by stock Go through real KAS outside timing, then
checked exactly. Python's standard ZIP reader separately reads both members to
EOF, verifying CRC values. The controller records both validations per archive.
Platform audit logs confirm **288 distinct real KAS rewrap requests**: 144 native
pair decryptions and 144 independent stock-Go checks. Only safe audit counts and
hashes are exported. The reused original session key does not cache plaintext
keys: every archive still requires KAS rewrap.

Inputs are deterministic binary bytes: byte `i` is
`(i * 131 + (i >> 8) * 17) & 255`. The three sizes use one, five and 25 segments.
Every manifest was checked for AES-256-GCM, GMAC and the expected segment count.

The measured packages delegate IEEE CRC32 to Go `hash/crc32`, Java
`java.util.zip.CRC32`, Python `zlib.crc32` and Node `node:zlib.crc32`. Node package
exports automatically select its native entry; browser/default exports retain
the portable fallback. C# deploys Microsoft's official **System.IO.Hashing 8.0.0**
NuGet package and uses `Crc32.HashToUInt32`; it is a first-party dependency rather
than part of the shared runtime. Rust directly depends on **crc32fast 1.5.2**.
C and browser TypeScript retain slicing-by-8 fallback because their standard
platforms provide no CRC32 API. Acceleration is chosen by the runtime or library;
these results do not claim a particular hardware instruction executed.

Installed-package checks observed real Node builtin CRC calls during public
KAS encrypt/decrypt, verified C# native IL delegation and deployed dependency,
and verified Rust's direct dependency and linked implementation. The portable
browser package graph still contains no Node imports, `Buffer` or `process`.

Targets ran sequentially on one AMD Ryzen 7 6800H Linux x86_64 host without
competing SDK builds or benchmarks. There is no forced GC, allocator reset,
CPU pinning or heap tuning. Natural allocation/GC, RSA randomness, local KAS
and scheduling variation remain in these samples. Five samples and one warmup
characterize this local run, not steady-state JIT or long-tail latency. Browser
execution is excluded.

## Receipts

Safe samples, environment, package/member hashes, source identities, archive
manifest checks and validation counts are in
[benchmark-results.json](benchmark-results.json). The ignored campaign directory
`.local/benchmarks/e2e-v020-2026-10-03` retains `raw.jsonl`, `summary.json`,
`tables.md`, `environment.json`, all archives, input fixtures and compiled
consumers. Private token files stay ignored with mode 0600 and are not exported.

The initial preparation freeze and actual measured-source freeze are preserved
separately. The final source freeze records two post-campaign changes: adding
the missing pinned compiler checkout to offline CI and updating the future
package-directory default. Neither changes measured timing code, installed
packages or the explicit campaign command. No campaign was replayed.

[Earlier Go profiles](go-profiles.md) describe a historical fresh-client,
separate-operation experiment. They are not samples from this table and were
not summed, relabelled or reused. Unrelated full interoperability matrices were
not repeated for this benchmark update; hosted SDK CI remains a separate check.
