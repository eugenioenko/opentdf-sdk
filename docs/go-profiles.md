# Go CPU profiles at 1 MiB

> Historical fresh-client, separate-operation diagnostics. These profiles do not
> describe the current end-to-end table, which reuses the original Go client
> initialized outside timing. No old operation medians or profile timings were
> summed or relabelled to produce that new table.

The main reason for the large encryption ratio is the fresh-client lifecycle:
the original SDK constructs an RSA2048 response-session key even for encryption.
The generated SDK creates its response-session key during decryption. CPU
profiling measured this distinction directly. It does not show that generated
AES encryption is intrinsically faster.

Four separate diagnostic runs accumulated approximately five seconds of wall
time each, with one untimed warmup and fresh public operation lifecycle per call.
The original decrypt reused the constructor's session key after the benchmark
wrapper's redundant `WithSessionKeyType` option was removed. Algorithms, real
KAS, payload, common reference archive, and token boundary match the benchmark.
These runs had profiling/label/phase-timer overhead and never replaced published
five-sample timings.

Original encryption completed 54 operations. Its CPU profile sampled 4.63 CPU
seconds: `SDK.New` accounted for 4.43 seconds cumulatively (95.68%), and RSA key
generation for 4.37 seconds (94.38%). Those overlapping cumulative percentages
use all 4.63 sampled CPU seconds as their denominator and must not be added.
The phase labels separately assigned 4.43 seconds to client setup and 0.13
seconds to the public operation. Instrumented wall medians were 69.28 ms for
`New`, 1.65 ms for `CreateTDF`, and 71.45 ms for the complete call. Phase medians
do not necessarily sum to the full-call median.

Generated encryption completed 646 operations and sampled 4.62 CPU seconds.
ZIP CRC32 accounted for 1.86 seconds cumulatively (40.26%; 38.96% flat), and
`runtime.mallocgc` for 0.87 seconds cumulatively (18.83%). Its full public-call
wall median was 6.80 ms, including its internal client setup. There were no
sampled RSA key-generation frames for encryption; the optional RSA source
report records that absence. In these instrumented runs, the original
`CreateTDF` payload operation was shorter than the generated complete API call.
The large published fresh-call advantage therefore should not be interpreted
as a payload-only comparison. A reused original client would be a different
measurement and was not tested here.

Original decryption completed 24 operations and sampled 3.79 CPU seconds.
`SDK.New` accounted for 1.94 seconds cumulatively (51.19%); RSA key generation
accounted for 1.93 seconds (50.92%). `Reader.ReadAt` accounted for 1.05 seconds
(27.70%). Phase-labelled CPU was 1.94 seconds for setup and 1.19 seconds for
load/read. Instrumented wall medians were 72.08 ms for `New`, 104.58 ms for
`LoadTDF` plus `io.ReadAll`, and 176.73 ms for the full call.

Generated decryption completed 62 operations and sampled 3.87 CPU seconds.
RSA session-key generation accounted for 3.09 seconds cumulatively (79.84%),
archive reading for 0.31 seconds (8.01%), and CRC32 for 0.26 seconds (6.72%).
The full public-call wall median was 75.89 ms. Its internal setup is opaque to
the public facade and remains within that public-operation phase; the host
configuration timing is not equivalent to original `SDK.New`.

CPU profiles sample executing work, including allocator/GC work. They do not
attribute time blocked on KAS/network responses or scheduling. The full-call
wall measurements include that waiting, but this run does not isolate network
latency precisely. Background runtime CPU can be unlabelled; the phase-label
counts above are therefore distinct from total CPU sample counts. Plaintext
comparison ran outside each operation wall timer and was labelled separately
in the CPU profile. Every decrypted output matched exactly. Warmup, first and
last archives from each encryption profile were independently decrypted by
stock Go through real KAS after profiling stopped.

Reproduce with existing accepted packages and the benchmark fixtures/services:

```sh
python3 scripts/profile-go-sdk.py \
  --campaign .local/benchmarks/campaign \
  --output .local/benchmarks/profiles/new-run --seconds 5
```

The runner builds only a diagnostic Go consumer with `GOTOOLCHAIN=go1.25.1`.
Each target/operation runs in its own process, with shared module imports
initialized before profiling. The output directory retains the symbolizable
`build/profiler`, exact source, `go.mod`/`go.sum`, four `cpu.pprof` files,
`instrumented.json` phase records, CPU top/calltree/label/source reports and a
safe summary with binary/source/profile hashes. `--resume` reuses already
captured profiles. The accepted run is under ignored
`.local/benchmarks/profiles/corrected-go`; the main README reports only the
separate benchmark medians. No SDK, compiler or service configuration changed.
