# Native SDK end-to-end benchmark

The README reports one table for **10 KiB, 100 KiB and 1 MiB** of binary
plaintext. Each cell is the median elapsed **milliseconds** of five newly
measured complete encrypt/decrypt pairs. The timer is contiguous: encryption
produces an archive, decryption consumes that same archive through the real KAS,
and the timer stops after complete owned plaintext is available. These values
are actual end-to-end measurements; they are not sums of earlier separate
operation medians. All eight implementations were run again in a fresh campaign.

## Reproduce

Use the accepted installed packages indexed by
`.local/phase7/final-packages-v2`, the pinned platform checkout in
`references.lock.json`, and the existing BASIC profile with real KAS at
`http://localhost:8080/kas` and local IdP at port 8888. Package and runtime
installation follow the delivery instructions. The runner does not start,
stop or reconfigure services and does not invoke the Goalchemy compiler.

From the SDK repository, choose a new output directory:

```sh
python3 scripts/benchmark-sdk.py --output .local/benchmarks/e2e-reproduction
```

The defaults select all eight implementations and `10KiB,100KiB,1MiB`.
`--build-only` builds native benchmark consumers; `--skip-build` reuses those
consumers. Existing successful end-to-end cells are reused only within their
output directory; a new directory produces a new campaign. `--rerun` explicitly
selects a `TARGET:e2e:SIZE` cell when needed. Rust links the accepted consumer's
already compiled SDK rlib. C, Java and C# link installed native archives/JARs/
assemblies. Go imports the accepted generated package; Node and Python import
installed modules. The reference consumer imports the pinned `../platform/sdk`
public API. Both Go consumers are built with `GOTOOLCHAIN=go1.25.1`.

## Setup and lifecycle

Original Go `SDK.New` executes **once before the warmup and timing loop** for
each implementation/size cell. Its RSA2048 response-session key and ES256 signer
are reused for all six pairs. `CreateTDF`, `LoadTDF` and `io.ReadAll` run inside
each end-to-end interval. `LoadTDF` uses the constructor's default session key;
the wrapper does not request `WithSessionKeyType` and generate a replacement.
Client `Close` executes after the loop, outside timing.

All hosts prepare their public configuration, token provider, and encryption
options outside the timer. Generated portable APIs expose stateless encryption
and decryption facades. Their internal per-operation client/signing-key
construction and per-decrypt RSA2048 session generation remain inside the timed
public API calls. The original reusable client/session and generated facade
session lifecycles therefore differ. This table compares the actual supported
public APIs with harness setup removed; it does not claim equal key lifecycles.
No production or exported API changes were made to create a synthetic match.

OAuth tokens and the shared public KAS PEM/kid are acquired before each host
batch. The original uses `oauth2.StaticTokenSource`; generated native providers
return the same externally acquired Bearer token during public API calls.
Neither OAuth acquisition nor public-key discovery is timed. Public KAS-key
import and any internal SDK authentication work remain timed. Each batch fits
the local token lifetime. No private-key PEMs are supplied. Algorithms are
RSA2048 KAS wrapping and response sessions, ES256 signing, GMAC segment integrity,
and 2 MiB segment size, with the same permitted attribute and no metadata or
compression.

## Timing and correctness

Each process preloads deterministic input, executes one untimed warmup pair,
then records five pairs using its native monotonic clock. Timing includes
public API input/output ownership copies, crypto, policy/key wrapping, ZIP
construction/parsing, actual KAS request and unwrap, integrity verification,
and plaintext materialization. File reads/writes, process/module startup,
configuration/provider/options construction, and exact-output checking are
outside timing. Result disposal after checking is also outside timing.

Rust's APIs consume `Vec<u8>` inputs. The harness prepares fresh owned input,
configuration/options clones and provider registries before each timer. To
retain every encrypted archive for independent validation, it clones the
newly produced archive for the consuming decrypt call **inside** the contiguous
interval. This Rust-specific retention cost is included and recorded; the
harness never pauses or splits the interval for an untimed copy.

Every warmup and timed pair compares the complete final plaintext byte for byte
with the input after timing. Every freshly encrypted archive is also retained
and independently decrypted by stock Go through the real KAS outside timing,
then checked exactly. There are 24 cells, 144 complete pairs including warmups,
120 measured samples, and 144 independent archive validation checks. The
reused original response-session key is not a plaintext-key cache: each fresh
archive still requires a real KAS rewrap request. Generated decryption does the
same. No mock KAS or private/offline crypto shortcut is used.

Inputs are deterministic arbitrary binary bytes: byte `i` is
`(i * 131 + (i >> 8) * 17) & 255`. Each supported file fits one 2 MiB segment.
Targets run sequentially on one host with no other SDK benchmark load. There
is no forced garbage collection, allocator reset, CPU pinning or heap tuning;
natural allocation/GC during each interval remains timed. One warmup and five
samples describe this local run, not steady-state JIT behavior or long-tail
latency. RSA randomness and KAS/scheduling variation contribute to sample spread.
Browser execution is excluded at the user's request.

## Receipts and prior experiments

The fresh accepted campaign is under ignored
`.local/benchmarks/e2e-2026-10-03`. `raw.jsonl` retains the actual five sample
values per cell, medians, ranges, standard deviations, exact commands, payload/
archive hashes, correctness results, expected KAS call counts and source hashes.
`summary.json` contains the 24 end-to-end cells; `tables.md` is the single
milliseconds-only table. `environment.json` records hardware/kernel/toolchains,
SDK/platform heads, package receipts/member hashes, linked Rust rlib identity,
source hashes and compiled consumer hashes. Frozen source/module inputs and
symbolizable consumer binaries remain in the ignored campaign directory.
Private token files stay ignored and use mode 0600. Safe exports contain no tokens.

The run uses the same AMD Ryzen 7 6800H Linux x86_64 host and accepted native
packages as the earlier experiments. Both benchmark Go binaries use Go 1.25.1;
other installed runtimes are recorded in the campaign receipt. The CPU profiles
in [Go profiling findings](go-profiles.md) belong to the earlier **fresh-client,
separate-operation** experiment. They explain that old lifecycle comparison;
they do not describe the new reused-original-client end-to-end table. Historical
samples and profiles remain separately attributed and are neither summed nor
relabelled as end-to-end samples. The former redundant-RSA baseline correction
also remains historical evidence; all 24 current cells are newly measured.
