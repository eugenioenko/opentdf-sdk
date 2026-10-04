# Seven-target TDF3 delivery

The shared [Go façade](../library/library.go) is delivered as seven importable
native libraries. Each executes its bundled lowered implementation and native
crypto/HTTP capabilities. Production encryption and decryption require neither
the compiler nor an original SDK/reference process. TypeScript supports Node
and an actual browser through the same portable ESM package.

All seven target phases have accepted, signed source revisions. Final delivery
adds reproducible packages, independent installed consumers, focused real-KAS
jobs, and executable CI definitions. The root delivery review and final commits
are recorded separately in the [delivery checklist](delivery-checklist.md) and
[progress ledger](progress.md). This document does not claim a remotely executed
GitHub Actions run or a newly replayed full interoperability matrix.

## Build and install

The buffer optimization uses released Goalchemy v0.2.1.
Its initial verification covers Python BASIC interoperability and buffer
ownership. A subsequent [all-target benchmark](all-target-buffer-benchmarks.md)
built and installed the other six native packages, then passed fresh BASIC E2E
checks for all seven targets and independent stock-Go/KAS plus ZIP validation.
EC/DPoP matrices remain historical evidence and were not replayed for these local
changes. See [local optimization results](python-buffer-optimization.md). The
README performance table continues to describe the released v0.2.0 packages.

Use adjacent `sdk`, `goalchemy`, `platform`, and `web-sdk` checkouts. Exact source
revisions are in [references.lock.json](../references.lock.json); Goalchemy is
pinned to commit `de26e4aa18f38f75bdf7a43c7b19b33cfaec9673`, released as
[v0.2.1](https://github.com/eugenioenko/goalchemy/releases/tag/v0.2.1) after
[Goalchemy PR #10](https://github.com/eugenioenko/goalchemy/pull/10) passed hosted CI
and was merged.
Platform remains pinned to
`f2635158b681fa970aafce7eacf108a453521f63`, and Web SDK to
`55a0521b1499b392c75373e11ec5930c6a43f0c7`.

Build one compiler from that Goalchemy revision and supply it to any helper:

```bash
cd goalchemy
GOTOOLCHAIN=go1.25.14 go build -trimpath -o out/final-delivery/goalchemy ./cmd/goalchemy
cd ../sdk
GOALCHEMY_BIN=../goalchemy/out/final-delivery/goalchemy bash scripts/build-generated-go.sh .local/generated/go
```

Every `scripts/build-generated-<target>.sh` accepts a destination and
`GOALCHEMY_BIN`. Both may be relative to the caller's working directory or
absolute. Clean delivery checks build twice from different caller directories
and compare the distributed members. Tool installations, compiler output,
packages, consumer builds, caches, logs, keys, and profiles stay ignored.

| Target | Distributed package and independent consumer | Tested prerequisites and API documentation |
| --- | --- | --- |
| Go | Source module with bundled runtime; separate importing Go module and executable | Go1.25.14; [Go API](generated-go-library.md) |
| TypeScript | ESM JavaScript, public declarations and notices; packed npm archive, extracted Node consumer, declaration compilation and browser bundle | Node24.15.0, TypeScript6.0.3; [Node/browser API](generated-typescript-library.md) |
| Java | `tdf3-java.jar` and locked provider JAR; separate `javac` consumer | Temurin21.0.12.1+1, Bouncy Castle1.86; [Java API](generated-java-library.md) |
| C# | `OpenTDF.TDF3.dll`, `System.IO.Hashing.dll`, locked dependency metadata and notices; separate .NET project referencing both assemblies | .NET SDK8.0.425/runtime8.0.31, Microsoft System.IO.Hashing8.0.0; [C# API](generated-csharp-library.md) |
| Python | Wheel; isolated venv installs the wheel and hash-verified runtime wheels with no index | Python3.10, cryptography50.0.2/CFFI2.1.1; [Python API](generated-python-library.md) |
| Rust | `opentdf-tdf3-0.1.0.crate`; independent extraction, native consumer and Cargo target directory | Rust1.98.0, locked Cargo dependencies; [Rust API](generated-rust-library.md) |
| C | Linux x86_64 archive containing headers, generated sources and `libtdf3.a`; separate archive-linked native consumers | GCC11.4.0/C17, OpenSSL3.0.2, curl7.81.0, Boehm8.2.8; [C API](generated-c-library.md) |

[delivery-bootstrap.py](../scripts/delivery-bootstrap.py) verifies the reference
pins and fetches target prerequisites from pinned locks. The compiler and
package subprocesses require the pinned Go toolchain's standard-library root;
bootstrap records that environment explicitly for runners without an inherited
`GOROOT`. Failed pre-service compiler/package logs are uploaded with receipt
hashes; authentication and service logs remain private.
[delivery-packages.py](../scripts/delivery-packages.py) checks two-path builds;
[delivery-consumers.py](../scripts/delivery-consumers.py) installs and builds
independent consumers. [The job entrypoint](../scripts/delivery-job.py) runs
these in order and uses the installed packages for real-service operations.
The detailed scripts retain exact commands and terminal receipts.

Reproducibility covers distributed package members, including library binaries,
JavaScript/declarations, archives and notices. It excludes caches, native test
executables, PDB/debug output and diagnostic source maps. The original
final-delivery acceptance packages used compiler SHA256
`01a5398d6cc2d26d9a519abcfe181541b474dff5479ed12e98954e9680153810`.
Their production source members match the final accepted target trees; Go's
only source difference is diagnostic source-position comments. An external
concurrent Goalchemy formatting CLI branch advanced the host checkout to
`dcabfbf0cdb6bd3cf63989da02a8046270952de4`; the tested compiler and CI used
`765fcbc51241204d283ae68ff15e9957bc91c865` at that acceptance. Its seven
CLI/diagnostic/doc files are recorded separately and change no embedded
library, target or spec source.
The v0.2.0 compiler introduced native CRC32. Packages built from that release
were used for the [released-baseline benchmark](benchmarks.md); the original
package hashes above remain historical acceptance evidence. The subsequent
[buffer comparison](all-target-buffer-benchmarks.md) measures the runtime changes
now included in v0.2.1, using its separately recorded pre-release compiler.
The verified v0.2.0 compiler binary SHA256 is
`61dec71c6b3dde1634f598f4476608b703ce965cecc33912afd3fc930e22040b`.
SDK package versions remain 0.1.0; the current compiler's version is 0.2.1. Node package
imports select its native CRC entry; browser/default imports remain portable.
C# deploys Microsoft's first-party hashing package, Rust directly uses
`crc32fast`, and C keeps its slicing-by-8 fallback. Host APIs choose acceleration;
no particular hardware instruction is guaranteed.

## Executable jobs and observed coverage

[tdf3-delivery.yml](../.github/workflows/tdf3-delivery.yml) defines a fast
shared-source offline job and seven distinct required native-package/KAS jobs.
Each target runs on its own Ubuntu22.04 VM with pinned adjacent checkouts,
digest-pinned service base images, a unique Compose project/image, independent
package installations and private fixtures. A target serializes BASIC, EC and
enforced-DPoP profile changes. The TypeScript job also runs actual Chromium.
C bootstrap verifies exact native dependency hashes and fails on drift or
unavailable pinned Ubuntu artifacts. The workflow bounds jobs and subprocesses,
cleans up its own services, and uploads safe manifests rather than private keys,
credentials or token diagnostics.

Pull requests and manual runs default to `focused`; manual dispatch can request
`full`. Focused jobs run a real-KAS binary smoke, both reference directions,
owned/repeated calls and typed malformed-archive rejection for each final native
package. C also runs a public importer before any fixture collector setup.
Go/TypeScript EC and DPoP jobs cover the previously missing explicit-empty
metadata case across both wrapping algorithms, both response-session algorithms
and both signing algorithms. Actual browser checks accompany TypeScript.
Full definitions enumerate all seven cases, policy/authentication/transport
negatives, distinct captured outputs and retained-archive integrity supplements.
Full definitions were reviewed statically; they were not replayed for final
delivery merely because packaging or a CI job label changed.

For a disposable Linux x86_64 checkout with the listed prerequisites, Docker,
and free local service ports, the same entrypoint can run locally:

```bash
TDF_COMPOSE_PROJECT=phase7-go-local TDF_PLATFORM_IMAGE=phase7-go-local-platform:local \
  python3 scripts/delivery-job.py go --mode focused
docker compose --project-name phase7-go-local --env-file dev/images.env \
  -f dev/compose.yaml down --remove-orphans
```

The actual final local execution used one reviewed private checkout/project,
`phase7-delivery-20261003`, with fresh keys/database/realm fixtures. Independent
packages and consumers were prepared before startup. The actual BASIC smokes
passed all seven native targets and Chromium145.0.7632.6; EC and enforced-DPoP
missing-case jobs passed Go, TypeScript Node and that browser. The browser bundle
contains 88 production graph inputs, no Node imports/Buffer/process dependency,
and a host-only token broker. DPoP browser execution observed a genuine
CORS-readable nonce401 challenge and successful generated retries.
The private project was removed after terminal results. The root restored the
original services and independently verified BASIC health, issuer/audience
Bearer credentials and genuine RSA KAS operation. A restarted-stack readiness
gap was fixed by waiting for the provisioned issuer before platform startup;
its bounded command-order and failure-stop proof changes no SDK/runtime behavior.

Retained local review evidence under ignored `.local/phase7/` includes:

- `packages/<target>/receipt.json`, final corrected package/consumer projections
  under `final-packages-v2/`, and content hashes for all distributed/installed members;
- `offline/receipt.json`: 134 named shared-source format/rejection subchecks;
- `focused-final-coverage-v5.json`: exact passing coverage for 14 actual focused
  target/profile environments, with no missing required focused comparison;
- `accepted-final-correspondence.json`: read-only source correspondence and the
  canonical 24 native/browser profile environments assembled from accepted
  historical full evidence and separately hashed newly closed cases;
- `private-job/`: startup, actual BASIC/EC/DPoP jobs, browser jobs and cleanup
  status/log identities, including preserved failed receipts and their corrections.

The initial Go smoke hit a fixture self-copy error after its reference checks;
the corrected bounded Go smoke passed. The first C ownership control lost the
GC-managed parsed-config backing behind its fixture's malloc route array.
Keeping the fixture JSON root reachable repaired that consumer; the unchanged
SDK archive then passed only the pending lifetime/error/no-preinit controls.
Original failures and preceding successful comparisons remain preserved.
Neither correction changes the SDK runtime or relabels the failed aggregate job green.

## Supported profile and limits

The required byte API supports modern TDF3/manifest4.3.0; stored ZIP32/ZIP64
entries and descriptors; preferred `manifest.json` and legacy filename
`0.manifest.json`; AES256-GCM; GMAC/HS256 segments and HS256 root integrity;
exact policy binding; RSA2048/P256 wrapping and independent RSA/P256 response
sessions; RS256/ES256 request signing; explicit/discovered keys; trusted KAS
routes; and Bearer/enforced-DPoP with native token providers.
Empty, binary, exact-boundary, multiple-segment, HS256, metadata and explicit-empty
metadata cases are enumerated separately. See [compatibility](compatibility.md)
for the precise supported schema and rejected mandatory profiles.

Payload, metadata and metadata presence are separate properties. Stock Go omits
empty metadata; stock Web encrypts empty metadata even when its option is absent.
Generated results therefore derive `HasMetadata` from each producer's immutable
manifest. Generated explicit-empty metadata remains present. The stock Web
reader's API does not expose decrypted metadata, so its payload success is not
a metadata-reader proof.

Each accepted native DPoP matrix retains four stock Web nonce401 failures,
separately classified by wrapping and signing algorithm. Reference formats
created under Bearer and consumed by generated enforced-DPoP code prove format
interoperability and generated authentication; they do not prove successful
stock Web enforced-DPoP authentication. Historical browser binding/replay and
native transport/cancellation/cleanup negatives remain attached to their tested
source identities. Final-package focused checks supplement that evidence.

Unsupported mandatory features fail before successful decryption: assertions,
multiple shares, obligations requiring fulfillment, legacy4.2.2 profiles,
unknown algorithms/types and unsupported curves, compressed/multipart archive
variants, and invalid grouped policy IDs/statuses. Shared-source offline proofs
cover these parsing/protocol rules; native error propagation and real policy,
integrity, token, transport and cleanup proofs come from accepted target evidence
and the focused final checks. Broader stream/file/range/assertion APIs and full
OpenTDF feature parity remain outside this delivery.

Dependency locks and notices live under [hosts](../hosts) and the distributed
packages. Goalchemy's bundled runtime carries its Apache2 license; native
providers retain their own notices. The SDK repository has no top-level license
decision, and this delivery does not infer one from dependencies. Package names
such as `goalchemyout` and `@opentdf-local/tdf3` remain development identities.
Public repository availability does not constitute package-registry publication;
public module naming, repository licensing, and release/bootstrap metadata need
explicit release decisions.
