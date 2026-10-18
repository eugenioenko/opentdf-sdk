# Native SDK end-to-end benchmark

## Current campaign

Measured on 2026-10-18 on an AMD Ryzen 7 6800H running Linux x86_64.
The seven measured generated SDKs use source at `948cf50`, including the
shared bulk buffer wiping fix, and the unchanged Goalchemy build at
`468ec2482d3a70f304f9a06c2186c9aff6cc28a3`. Go, TypeScript, Python, Java, C#,
and Rust use native pausing; C uses frames. Rust uses a release build. Swift
is omitted as described below. SDK source, compiler, and benchmark inputs
remain frozen throughout measurement. The benchmark harness is pinned at
`948cf50`; source and artifact hashes are recorded in local campaign receipts.
Toolchains: Go 1.25.14, Node 24.15.0, Python 3.10.12, Java 21.0.12.1,
.NET SDK 8.0.425, Rust 1.98.0, and GCC 11.4.0.

Both upstream references were source-built at these exact revisions and
remeasured for this campaign:
Go SDK 0.35.0 at `b04847659c0fe82a3b2cf47ca61f8dbba33218e6`, and Web SDK
0.22.0 at `59c42c894523f581134dcd3b77fd9ba55ce16af7` running in Node. These
benchmark-only checkouts do not change the CI oracle pins in `references.lock.json`.
The local BASIC platform uses `4d9e555bddf0c850190680807118fbac1851a9aa`
and real KAS at `http://localhost:8080/kas`, with Keycloak at port 8888.

## 100 MiB extension

The 100 MiB column is a separate measurement on the same machine and date,
using the same compiler, installed SDK packages, upstream references, toolchains,
and warmup policy. The extension harness is `824ffc5`, based on main at
`b786639`. Main's SDK source and host adapters are identical to `948cf50`.
Installed artifact hashes were verified before reusing these packages; they
were not rebuilt for this extension. The earlier five columns are unchanged.

The first extension measured generated Go, Rust, C, and both upstream
references at 104,857,600 bytes. TypeScript, Java, C#, and Python rejected these
inputs at Goalchemy's then-existing 64 MiB whole-buffer library boundary.
Their original limit labels are now replaced by the follow-up measurements below.

The five supported rows use three fresh processes and fifteen measured pairs
per row. All 90 retained archives pass independent Go/KAS plaintext and ZIP CRC
validation, with fifty 2 MiB segments, AES-256-GCM, and GMAC. Detailed samples
and the four rejection probes remain under ignored local storage. No 100 MiB
batch triggered the warmup warning; no samples or batches were replaced.

## Complete the 100 MiB byte-API measurements

The four remaining rows were measured on the same machine and date using
Goalchemy PR #117 at `5d5edf0529455e856062fbc42e64437275930b51`, the v0.7.4
candidate. This removes the fixed library value-size caps, including the
TypeScript and C# output conversions. The same byte APIs now accept 100 MiB;
streaming APIs were not substituted. Release adoption follows CI acceptance.

TypeScript, Java, C#, and Python packages were rebuilt twice reproducibly from
SDK `27cc668`, whose production source is identical to `948cf50`, and installed
through independent native consumers. The compiler binary, references, toolchains,
benchmark controller and native consumers were frozen before timing. The earlier
Go, Rust, C and reference cells are unchanged, as are all five smaller columns.

Each new row pools fifteen samples from three fresh processes under the same
forty-bulk/twenty-size-specific warmup policy. All 60 timed pairs matched
plaintext. All 72 retained archives passed independent Go/KAS decryption,
plaintext, ZIP CRC and fifty-segment AES-256-GCM/GMAC validation. No warmup
warnings triggered; no samples or batches were replaced. Frozen source, package,
binary and archive hashes match. The temporary token setting was restored and
a fresh token confirmed the original 300-second lifetime. Detailed evidence
stays under ignored local storage.

## Measurement policy

Payloads in the original campaign are exactly 1,024, 10,240, 102,400,
1,048,576, and 10,485,760 bytes. The extension adds 104,857,600 bytes.
Each result is the median of all fifteen measured encrypt/decrypt pairs:
five pairs in each of three fresh processes. Every process first performs
forty full 10 MiB pairs, then twenty pairs at the measured size. The bulk
payload is separate from the size-specific input. All targets run sequentially
after package builds finish, with ordinary runtime settings and inherited
JVM, .NET, Node, Go, and Python tuning removed.

One SDK handle is initialized before warmup and reused throughout each process.
Generated SDKs and the Go reference reuse their default RSA-2048 response session.
The Web reference reuses its client and ES256 signer; its public decrypt API
generates a fresh RSA-2048 response key for each archive inside the timed call.
This public API behavior is part of its end-to-end cost.

A contiguous monotonic timer covers public encryption followed by decryption
of that same archive, including full ciphertext and plaintext materialization.
SDK initialization, fixture file I/O, OAuth acquisition, KAS public-key discovery,
plaintext comparison, archive persistence, and independent validation are untimed.
Every target uses RSA-2048 KAS wrapping, ES256 signing, GMAC, 2 MiB segments,
one permitted attribute, and autoconfigure disabled. The platform and clients
share this machine. These are warm end-to-end latency measurements through a
local KAS, not isolated crypto throughput or Internet latency estimates.

## Correctness and evidence

Every warmup and measured pair compares the entire decrypted plaintext. The
last warmup and all five measured archives from every process are retained.
Each retained archive is independently decrypted through the upstream Go SDK
and real KAS, checked byte for byte, read through ZIP CRC validation, and
checked for AES-256-GCM, GMAC, and the expected segment count.

The pre-fix campaign and earlier 50 MiB bulk-warmup diagnostics are retained
separately. None of those timing cells are reused. Every reported cell was
measured after the wipe fix using the common 10 MiB bulk-warmup policy.

The controller preserves all samples and warmup histories. A cell is flagged
when its last-ten-warmup median and measured median are both more than 20%
below its first-ten-warmup median. Flags require investigation before results
are accepted. Any whole-cell replacement retains the initial evidence and
records why it was superseded; the fastest samples are never selected.

C# at 1 KiB triggered this warning in one batch: its first-to-last warmup
median fell 20.14%. The final-ten warmup medians and measured medians agreed
within 3% in every batch, and the three measured process medians were 21.11,
21.18, and 21.54 ms. That agreement supports accepting the measured windows.
All fifteen original samples are retained, with the original warning and its
review recorded in the local campaign report. No samples or batches were replaced.

Rust at 10 MiB also warned in its first process, with a 27.88% warmup median
fall. Its final-ten warmup median was 74.36 ms and measured median 73.80 ms.
The three processes' final-ten warmup medians agreed within 4.4%, and the pooled
median of all fifteen measurements was 73.66 ms, within those final warmup
medians. The measured windows had converged after the higher initial warmups.
This review also retains the original warning and every sample without reruns.

The Web reference at 100 KiB warned in one batch. Measured pairs ranged from
32.98 to 131.16 ms, with high timings also appearing after the lower warmup
window. Its bulk-warmup half-medians stayed within 6% in every process. The
public API's fresh RSA key generation on every decrypt adds variable work to
these small-payload timings. All fifteen samples and the original warning are
retained. Small differences between Web-reference medians should be read
alongside this spread, rather than treated as precise speed changes.

For the seven reported SDKs, package receipts record the compiler hash,
reproducible relative and absolute packages, installed members, and native
importers. Detailed samples, warmup histories, receipts, tokens, keys, packages,
binaries, and archives stay under ignored `.local/`; they are not committed.

## Reproduce

Use the SDK checkout at `948cf50` and the upstream source revisions above. See the
native prerequisites in [final-delivery.md](final-delivery.md). Let `COMPILER`,
`DELIVERY`, and the campaign directories below be absolute paths. Build the
frozen compiler from the Goalchemy checkout, then build and install each package:

```sh
GOTOOLCHAIN=go1.25.14 go build -trimpath -o COMPILER ./cmd/goalchemy
# From the SDK checkout, with native toolchains on PATH:
for target in go typescript java csharp python rust c; do
  python3 scripts/delivery-packages.py "$target" --base DELIVERY \
    --compiler COMPILER --compiler-revision 468ec2482d3a70f304f9a06c2186c9aff6cc28a3
  python3 scripts/delivery-consumers.py "$target" --base DELIVERY
done
```

Start and verify the BASIC platform separately. Configure its benchmark
client to issue tokens valid for at least one hour. This campaign temporarily set the
local `opentdf-sdk` client's token lifetime to 21,600 seconds and restored its
previous setting afterward. The benchmark controller does not start services
or change their configuration.

```sh
python3 scripts/benchmark-sdk.py --packages DELIVERY --output NATIVE \
  --reference-source UPSTREAM_PLATFORM \
  --sizes 1KiB,10KiB,100KiB,1MiB,10MiB \
  --samples 5 --batches 3 --warmups 20 --bulk-warmups 40 --bulk-size 10MiB --fresh
```

Build the Web package from its frozen checkout with `npm ci` and `npm pack`
in `lib/`, then install that tarball in a separate npm project:

```sh
python3 scripts/benchmark-sdk.py --targets web --packages DELIVERY \
  --web-source UPSTREAM_WEB --web-revision 59c42c894523f581134dcd3b77fd9ba55ce16af7 \
  --web-package WEB_INSTALL/node_modules/@opentdf/sdk --output WEB \
  --reference-environment NATIVE/environment.json \
  --sizes 1KiB,10KiB,100KiB,1MiB,10MiB \
  --samples 5 --batches 3 --warmups 20 --bulk-warmups 40 --bulk-size 10MiB --fresh
```

For the 100 MiB extension, use the harness at `824ffc5` with packages built
from the identical SDK source at `948cf50`. Use new output directories and the
same references and runtime versions. Run the native command with
`--targets reference,go,rust,c --sizes 100MiB`, and the Web command with
`--sizes 100MiB`. Keep all sample, process, and warmup arguments unchanged.
For the four-row follow-up, use SDK `27cc668` and Goalchemy `5d5edf0` with the
same controller. Regenerate and install TypeScript, Java, C# and Python;
retain the earlier Go/Rust/C consumer receipts needed to prepare the controller.
Select `--targets typescript,java,csharp,python --sizes 100MiB` with a new output
directory and unchanged sample/process/warmup arguments. The default list stays
at the original five sizes; 100 MiB is selected explicitly.

`--build-only` prepares a campaign without timing. `--skip-build` reuses its
frozen binaries and rejects changed controller or benchmark sources.
`--fresh` rejects an existing raw-results file. A new campaign must use new
output directories. The Go binaries use Go 1.25.14. Web and generated
TypeScript use the same Node 24.15.0 runtime. Native packages are imported as
installed Go modules, npm packages, wheels, JARs, assemblies, Rust rlibs, and C
archives rather than executable wrappers.

## Swift performance follow-up

Before the wipe fix, a fresh Swift 6.4 release build and optimized native
importer passed correctness checks. The user allowed Swift to be omitted to
avoid delaying the campaign, so Swift is not rebuilt or measured in this
rerun. No earlier Swift timings are reused; they remain in Git history.
