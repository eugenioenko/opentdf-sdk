# Native SDK benchmark

The README has separate encryption and decryption tables for **10 KiB, 100 KiB,
and 1 MiB** of binary plaintext. A cell shows the median of five measured calls
in milliseconds and its duration divided by the original OpenTDF Go SDK median
for that operation and size. The original Go SDK is therefore 1.00× in every
column; lower ratios mean less elapsed time. These are local measurements, not
throughput guarantees.

## Reproduce

Use the accepted installed packages indexed by
`.local/phase7/final-packages-v2`, the pinned platform checkout in
`references.lock.json`, and the existing BASIC profile with real KAS at
`http://localhost:8080/kas` and local IdP at port 8888. Package/dependency and
runtime installation follow the delivery instructions. The runner does not
start, stop, or reconfigure services and does not invoke the Goalchemy compiler.

From the SDK repository:

```sh
python3 scripts/benchmark-sdk.py --output .local/benchmarks/reproduction
```

The default sizes are `10KiB,100KiB,1MiB`. `--targets` accepts `reference,go,
typescript,java,csharp,python,rust,c`. An existing output directory reuses its
successful cells without repeating them; use a new directory for a new campaign.
`--build-only` builds benchmark consumers; `--skip-build` reuses them. Only
consumer wrappers are built. Rust links the accepted consumer's already compiled
SDK rlib; C, Java and C# link the installed archive/JAR/assembly. Go imports the
accepted generated package, and TypeScript/Python import their installed modules.
The original Go consumer imports the pinned `../platform/sdk` public API.

## What is timed

Each host process reads fixtures before the loop, runs one warmup, then measures
five sequential calls with its native monotonic clock. The timer starts before
fresh per-call configuration/provider construction and stops after the full
public API completes and exposes an owned in-memory archive or plaintext.
Decryption includes the public archive load, real KAS rewrap/response unwrap,
segment integrity verification, and plaintext materialization. Encryption
includes policy/key wrapping, segment encryption/integrity, ZIP construction,
and public API output copies. Required API copies and facade work remain timed.

The original Go boundary is `SDK.New` followed by `CreateTDF`, or `LoadTDF` plus
`io.ReadAll`, followed by `Close`. Its constructor creates an RSA session key
even for encryption; this public lifecycle cost is included. `Close` currently
returns immediately. Generated facades create fresh clients internally for each
call and have no corresponding client-close API. All calls use ES256 signing
with fresh ephemeral signing keys and RSA2048 KAS wrapping/response sessions.
Private-key PEM parsing does not enter this comparison: no private PEM keys are
supplied. Public KAS PEM import remains inside each public operation as needed.

Untimed work includes process/module startup, fixture generation/read/write,
OAuth token acquisition, KAS public-key discovery, and exact-output verification.
Public-key discovery supplies the same PEM/kid to every implementation; the
original constructor receives an empty platform configuration to avoid remote
discovery. Native token providers return an externally acquired Bearer token
inside the API call. The original uses `oauth2.StaticTokenSource`. Python's
harness refreshes its token before each warmup/sample, outside the timer, to
support a batch longer than the 300-second local token lifetime. Other batches
reuse their pre-acquired token. The provider and KAS work during each SDK call
remain timed. No OAuth credentials appear in results.

Rust's public API consumes a `Vec<u8>`, so the harness prepares a fresh owned
input before the timer. Other hosts reuse their preloaded input. Rust's actual
facade/runtime copies remain timed. C frees detached owned results after timing;
managed hosts release outputs normally. There is no explicit forced garbage
collection, allocator reset, CPU pinning, or heap tuning. Natural allocation/GC
and runtime scheduling during the measured interval are included. One warmup
allows startup paths to execute but does not establish steady-state JIT behavior
for every runtime; five samples cannot characterize long-tail latency. Fresh
RSA key generation also contributes substantial sample variation.

## Inputs and correctness

Inputs contain arbitrary deterministic bytes: byte `i` is
`(i * 131 + (i >> 8) * 17) & 255`. Each archive uses 2 MiB segments, GMAC segment
integrity, the same permitted data attribute, and no metadata/compression.
For smaller files there is one segment. Each size's decryption fixture is the
original Go warmup archive, shared unchanged by all eight implementations.
Every encryption output, including the warmup output, is independently decrypted
through stock Go and the real KAS outside timing and compared byte for byte.
Every timed decryption is compared with the exact input after timing. A fresh
client/session prevents an offline unwrap cache; these public APIs request the
real KAS for each decryption. No mock KAS, plaintext shortcut, or private crypto
entry point is used. Browser measurements were skipped at the user's request.

## Receipts and attribution

Ignored `.local/benchmarks/campaign/raw.jsonl` retains the actual individual
sample values, ranges, standard deviations, medians, commands, correctness
results, payload/archive hashes, and failures. `summary.json` derives the final
48 native cells; `tables.md` derives the two README tables. `environment.json`
records CPU, memory, OS/kernel, toolchain versions, SDK/platform heads, package
receipts/member hashes, linked Rust rlib identity, and consumer/source hashes.
Tokens reside only in ignored private files with mode 0600.

The 1 MiB measurements were completed first and retained without replay. Their
exact original runner copies and native binaries are preserved under
`harness-v1`, with `identity.json`; original Go source copies there also preserve
the pre-gofmt build input. Subsequent source changes formatted wrappers, excluded
the unused browser runner, added an untimed Python token refresh, and selected
smaller fixtures/resumption in the controller. The public API timing and crypto
configuration were unchanged. Later receipts retain their own runner hashes.
The original `size_mib=1` receipts remain untouched; final derived results add
`size_bytes=1048576` and the `1MiB` label. Completed 10 MiB experiments, an
interrupted Python 10 MiB batch, and two browser launch failures remain separate
historical receipts and are excluded from the final tables.

The campaign ran on an AMD Ryzen 7 6800H Linux x86_64 workstation with
32,118,324 KiB physical memory and local KAS/IdP. Both benchmark Go binaries
record Go 1.25.1 in `go version -m`; the SDK-directory `go version` probe in
`environment.json` reports Go 1.25.14 because directory toolchain selection
differs. Binary build metadata in `harness-v2/identity.json` is authoritative.
Other runtimes were Node 24.15.0, Python 3.10.12, Temurin Java 21.0.12.1,
.NET SDK 8.0.425, Rust 1.98.0 and GCC 11.4.0. Targets ran
sequentially with no other SDK benchmark load. These numbers reflect this host,
installed runtime versions, fresh-operation lifecycle, and local KAS latency;
compare equivalent application lifecycles when applying them elsewhere.
