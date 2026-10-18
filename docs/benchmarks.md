# Native SDK end-to-end benchmark

## Current campaign

Measured on 2026-10-18 on an AMD Ryzen 7 6800H running Linux x86_64.
All eight generated packages use the merged SDK source at `5cc75d4` and a fresh
Goalchemy build at `468ec2482d3a70f304f9a06c2186c9aff6cc28a3`.
Go, TypeScript, Python, Java, C#, Rust, and Swift use native pausing; C uses frames.
Rust uses a release build. The reported matrix covers seven generated SDKs
and both references; Swift is omitted as described below. No language
implementation changed during measurement. The benchmark harness is pinned at
`2880d3d`; its hashes and toolchain versions are included in the timing evidence.

Both upstream references were built and measured again in this campaign:
Go SDK 0.35.0 at `b04847659c0fe82a3b2cf47ca61f8dbba33218e6`, and Web SDK
0.22.0 at `59c42c894523f581134dcd3b77fd9ba55ce16af7` running in Node. These
benchmark-only checkouts do not change the CI oracle pins in `references.lock.json`.
The local BASIC platform uses `4d9e555bddf0c850190680807118fbac1851a9aa`
and real KAS at `http://localhost:8080/kas`, with Keycloak at port 8888.

## Measurement policy

Payloads are exactly 1,024, 10,240, 102,400, 1,048,576, and 10,485,760 bytes.
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

The initial 50 MiB bulk-warmup attempt is retained separately as diagnostics;
none of its timing cells are reused. Every reported cell uses the common
10 MiB bulk-warmup policy.

The controller preserves all samples and warmup histories. A cell is flagged
when its last-ten-warmup median and measured median are both more than 20%
below its first-ten-warmup median. Flags require investigation before results
are accepted. Any whole-cell replacement retains the initial evidence and
records why it was superseded; the fastest samples are never selected.

For the seven reported SDKs, package receipts record the compiler hash,
reproducible relative and absolute packages, installed members, and native
importers. Raw tokens, keys,
packages, binaries, and archives stay under ignored `.local/`. Sanitized timing
evidence and exact versions are in
[the timing report](benchmark-results/2026-10-18.json).

## Reproduce

Use the SDK checkout at benchmark harness `2880d3d`, which retains the SDK
implementation from `5cc75d4`, and the upstream source revisions above. See the
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

`--build-only` prepares a campaign without timing. `--skip-build` reuses its
frozen binaries and rejects changed controller or benchmark sources.
`--fresh` rejects an existing raw-results file. A new campaign must use new
output directories. The Go binaries use Go 1.25.14. Web and generated
TypeScript use the same Node 24.15.0 runtime. Native packages are imported as
installed Go modules, npm packages, wheels, JARs, assemblies, Rust rlibs, and C
archives rather than executable wrappers.

## Swift performance follow-up

A fresh Swift 6.4 release build and an optimized native importer passed
correctness checks. The user requested omitting Swift if it delayed the timing
campaign, so no Swift measurements are included. Its earlier stateless-facade
numbers are superseded and remain in Git history.
