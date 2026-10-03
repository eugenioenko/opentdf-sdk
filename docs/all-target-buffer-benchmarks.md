# Local all-target buffer benchmark comparison

The local buffer changes clearly reduce Python E2E time at 10 and 50 MiB.
Node also shows a smaller 50 MiB improvement. The other targets do not establish
a uniform performance gain: several medians improve, but their sample ranges
overlap, and Java/C# barely change at 50 MiB. All seven native SDKs pass the
benchmark's real-KAS correctness checks.

These are fresh measurements from 2026-10-03, separate from both the released
README results and the [earlier Python-only campaign](python-buffer-optimization.md).
Measurements were accepted locally on `perf/python-buffer-handling-local` before
PR publication. No code changed while preparing or running this comparison. The
README/public results and releases remain unchanged. The compiler dependency is
[Goalchemy PR #10](https://github.com/eugenioenko/goalchemy/pull/10). Browser
measurements remain excluded.

## Fresh baseline medians

| SDK | 1 MiB | 10 MiB | 50 MiB |
| --- | ---: | ---: | ---: |
| Original OpenTDF Go | 53.42 ms | 99.91 ms | 216.09 ms |
| Generated Go | 87.97 ms | 105.68 ms | 248.86 ms |
| TypeScript (Node) | 99.11 ms | 250.47 ms | 792.19 ms |
| Java | 120.30 ms | 419.18 ms | 1621.66 ms |
| C# | 251.53 ms | 267.08 ms | 585.90 ms |
| Python | 164.74 ms | 241.33 ms | 1085.96 ms |
| Rust | 81.58 ms | 126.27 ms | 627.81 ms |
| C | 157.68 ms | 183.36 ms | 581.91 ms |

## Local candidate medians

| SDK | 1 MiB | 10 MiB | 50 MiB |
| --- | ---: | ---: | ---: |
| Original OpenTDF Go | 43.87 ms | 99.87 ms | 223.77 ms |
| Generated Go | 54.74 ms | 91.53 ms | 229.75 ms |
| TypeScript (Node) | 114.24 ms | 190.00 ms | 729.85 ms |
| Java | 172.48 ms | 398.75 ms | 1604.60 ms |
| C# | 272.32 ms | 267.62 ms | 570.04 ms |
| Python | 150.42 ms | 191.01 ms | 702.17 ms |
| Rust | 65.35 ms | 131.05 ms | 593.37 ms |
| C | 101.22 ms | 359.15 ms | 519.91 ms |

Each value is milliseconds for contiguous public encryption followed by public
decryption of the same fresh archive, including complete plaintext materialization.
Each cell has one warmup followed by five measured pairs. File sizes use MiB
(1,048,576 bytes), not decimal MB.

## Interpretation and variability

At 50 MiB, Python's median falls from 1085.96 to 702.17 ms (35.3% less time).
Node falls from 792.19 to 729.85 ms (7.9% less). Their observed five-sample ranges
do not overlap. Python's 10 MiB ranges also do not overlap. These observations
support a benefit on this machine, without establishing a cross-machine guarantee.

Generated Go, Rust and C have lower 50 MiB medians by 7.7%, 5.5% and 10.7%,
respectively, but overlapping sample ranges make the effect less certain. Java
and C# change by just 1.1% and 2.7%. The unchanged original-Go control itself
moves by 17.9% at 1 MiB and becomes 3.6% slower at 50 MiB, demonstrating variation
between campaigns. Small-file percentage changes should not be attributed entirely
to the optimization. The five-sample minimum/maximum below is an observed range,
not a confidence interval; positive reduction means less median time.

| SDK | Baseline 50 MiB range | Candidate 50 MiB range | Median time reduction |
| --- | ---: | ---: | ---: |
| Original OpenTDF Go | 182.79–220.67 ms | 199.37–238.65 ms | -3.6% |
| Generated Go | 215.97–271.95 ms | 198.09–256.25 ms | 7.7% |
| TypeScript (Node) | 784.24–809.02 ms | 720.28–766.08 ms | 7.9% |
| Java | 1599.65–1827.98 ms | 1518.61–1728.16 ms | 1.1% |
| C# | 565.61–858.69 ms | 559.91–865.34 ms | 2.7% |
| Python | 1075.51–1135.70 ms | 694.04–776.43 ms | 35.3% |
| Rust | 601.59–661.79 ms | 572.63–622.79 ms | 5.5% |
| C | 511.93–637.39 ms | 450.75–577.69 ms | 10.7% |

The full campaign initially showed Java 1 MiB 43.4% slower and C 10 MiB 95.9%
slower. Root repeated only those two comparisons with the same packages,
harnesses and settings, baseline then candidate for each target. Each repeat
again uses one warmup and five measured pairs. All original full-campaign
samples remain intact; repeats are separate and are not pooled into or substituted
for the tables above.

| Cell | Full baseline | Full candidate | Repeat baseline | Repeat candidate |
| --- | ---: | ---: | ---: | ---: |
| Java 1MiB | 120.30 ms | 172.48 ms | 175.22 ms | 119.47 ms |
| C 10MiB | 183.36 ms | 359.15 ms | 269.19 ms | 287.24 ms |

Java's direction reverses on repeat, with wide overlapping ranges
(108.16–228.32 versus 112.71–238.51 ms). C's repeat is 6.7% slower, with overlapping
ranges (182.55–285.64 versus 179.07–406.61 ms); the initial near-doubling does not
persist. These repeats do not establish that either target is uniformly faster
or slower. The evidence supports keeping the Python improvement and recording
a modest Node benefit, while leaving other target performance claims qualified.

## Method and source identity

The unchanged [controller](../scripts/benchmark-sdk.py) and
[native harnesses](../tests/bench/) use separate baseline and candidate installed
packages. Baseline timings were freshly collected from the accepted v0.2.0
packages; no previous timing values were copied. Candidate Go, Node, Java, C#,
Rust and C packages were built and installed afresh. Their package contents
match across relative and absolute build paths. Candidate Python reuses the
exact previously accepted reproducible wheel and installed environment, with
all 113 SDK import members checked against their frozen hashes. Baseline and
candidate native toolchains match: Go 1.25.1, Node 24.15.0, Python 3.10.12,
Temurin 21.0.12.1+1, .NET SDK 8.0.425, Rust 1.98.0 and GCC 11.4.0.

All builds finished before timing. The full baseline and candidate campaigns
ran sequentially without concurrent builds. Both use the exact same original-Go
control binary, SHA256
`5e05e9c2df3dc19c078a53ecd7f5ec2fe0e69db2a2f728947b9c9d909f7284da`.
A separately built candidate reference binary was retained but not used after its
digest differed; this difference is not assigned a cause.

Inputs, configuration, OAuth token acquisition and KAS public-key discovery are
outside timing. The BASIC profile uses real local KAS, Bearer authentication,
RSA-2048 wrapping/response sessions, ES256 signed requests, AES-256-GCM/GMAC and
2 MiB segments (1, 5 and 25 segments at the three sizes). Original Go creates one
SDK client and response key before each cell and reuses them across that cell.
Generated SDKs retain their stateless public-call lifecycle, including timed
response-key generation during decryption. These lifecycles are unchanged within
each target's baseline/candidate comparison; absolute cross-target timings include
that API lifecycle difference. Filesystem I/O, exact plaintext comparison and
independent validation are outside the measured interval.

The producing SDK source is commit
`a46fd1e23ae4a7633062e3e5f7243c3a3532e6e4`, and Goalchemy source is local commit
`4f5b428bf662a09991e05140484d4380e0401513`. The frozen candidate compiler SHA256 is
`215efc0c817497593c76c8f4cad68cab448d0101925ed84dbd0e60c972a5a0f5`;
it reports 0.2.0 but contains unreleased changes and is not the release
binary. Baseline packages use the published v0.2.0 compiler. Platform remains
pinned to `f2635158b681fa970aafce7eacf108a453521f63`; the original Go SDK and
reference validator come from that checkout.

## Acceptance and retained evidence

The two complete campaigns cover 48 cells and 240 measured pairs, with 288 fresh
warmup/measured archives. Focused repeats add four cells, 20 measured pairs and
24 archives. All 312 archives passed independent stock-Go real-KAS decryption,
exact plaintext comparison and independent ZIP CRC validation. Root checked raw
statuses, recomputed all medians and checked every retained archive's actual
hash and byte count, including uniqueness across campaigns.

Root also checked 1,637 frozen source files, 1,290 distributed/installed package
member hashes and the 113 actual Python SDK import members. Relative/absolute
package equality and build/install receipt statuses passed for all seven targets.
This is BASIC benchmark interoperability evidence; EC/DPoP matrices and hosted CI
were not replayed in this task.

Detailed logs, source freezes, raw samples and archives remain ignored under
`.local/buffer-all-targets-2026-10-03/`. The `baseline/` and `candidate/` directories
contain `raw.jsonl`, `summary.json`, `environment.json` and root acceptance receipts;
`followup/baseline/` and `followup/candidate/` retain the separate anomaly repeats.
`native-artifacts-receipt.json` records packages and installed members;
`preparation-handoff-control-amendment.json` records the exact reference-control
selection. `root-preparation-followup-acceptance.json` records root's final source,
package and all-archive verification. Development credentials and generated
artifacts remain outside git.
