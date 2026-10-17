# Running xtest against the generated SDKs

The cross-client suite in the tests repository
(`../tests`, a fork of `opentdf/tests` at
[eugenioenko/opentdf-sdk-tests](https://github.com/eugenioenko/opentdf-sdk-tests))
drives each generated SDK through its native `otdfgc` CLI, registered there as
the SDK types `gcgo`, `gcts`, `gcpy`, `gcjava`, `gccs`, `gcrs`, `gcc` and
`gcswift` (see [full-sdk-plan.md](full-sdk-plan.md), "xtest integration").
This document covers the two scripts that connect this repository to that
harness and the scoreboard they maintain.

## Pieces

| Where | What |
| --- | --- |
| `src/hosts/<target>/cmd/otdfgc` | The native CLIs over the generated Go package, TypeScript package, Python wheel, Java JAR, .NET assembly, Rust crate, C library and Swift package (`gcgo`, `gcts`, `gcpy`, `gcjava`, `gccs`, `gcrs`, `gcc`, `gcswift`): `encrypt`, `decrypt`, `inspect`, `supports <feature>`, `--version --json` with the same flags and wording. Exit 0 on success, 1 when the operation fails (the `Error.Message` chain after otdfctl's prefix), 2 for usage errors and for every contract flag this build cannot honour (`unsupported in this build: <flag>`). |
| `scripts/build-<target>-cli.sh <generated-dir> <binary>` | Build `otdfgc` against a package produced by the matching `scripts/build-generated-<target>.sh`: a Go, Rust or Swift binary, a C binary behind a launcher that sets the libcurl runtime path, or a launcher over `node`, a venv, the pinned JDK or the pinned .NET runtime. |
| `tests: xtest/sdk/gc-cli.sh` | The one shim shared by every gc type; each type's Makefile copies it to `dist/<version>/cli.sh`. It maps the `XT_WITH_*` variables to `otdfgc` flags and answers `supports` for every name in `tdfs.feature_type` by asking `otdfgc supports`, which reads `SupportedFeatures()`; unknown names exit 2. |
| `tests: xtest/sdk/gc.mk`, `xtest/sdk/<type>/Makefile` | Build `dist/<version>/otdfgc` from `src/<version>` (a checkout or symlink of this repository): clone and build Goalchemy at the revision pinned in `references.lock.json` (a sibling `../goalchemy` checkout at that revision is used as is), run `scripts/build-generated-<target>.sh`, then the target's CLI build. All eight types build. The Java, C#, C and Swift builds first run the Goalchemy checkout's `scripts/fetch-toolchains.sh` for the pinned JDK, .NET, bdwgc or Swift toolchain; `GOALCHEMY_CHECKOUT=<dir>` reuses a checkout at the pinned revision instead of a clone. |
| `tests: otdf-sdk-mgr` | `install tip [--ref <ref>] gcgo` and `checkout gcgo <ref>` work (source only; no release artifacts yet). |
| `scripts/xtest-run.sh` | Builds the requested types into the tests repository via a symlink to this checkout, starts the platform when needed, runs a pytest selection with `--junitxml`, and regenerates the scoreboard. |
| `scripts/xtest-scoreboard.py` | JUnit XML to [xtest-scoreboard.md](xtest-scoreboard.md). |

## Running a selection

```sh
systemd-run --user --scope -p MemoryMax=10G -p CPUQuota=800% -- \
  scripts/xtest-run.sh --types gcgo --sdks "go gcgo" --label "phase 1" -- \
  test_tdfs.py::test_tdf_roundtrip test_tdfs.py::test_manifest_validity \
  --sizes small --containers ztdf -v
```

- `--types` (default `gcgo`; any of the eight): the gc types to build; each gets
  `xtest/sdk/<type>/src/<version>` symlinked to this checkout and
  `make -C xtest/sdk <type> VERSIONS=<version>`.
- `--version` (default `main`): the dist name, so the SDKs appear to pytest
  as `gcgo@main`.
- `--sdks`: passed to pytest; defaults to the gc types alone. Reference SDKs
  are installed in the tests repository with
  `otdf-sdk-mgr install stable go` / `install stable js`.
- `--junit` (default `out/xtest/<timestamp>.xml`) and `--label` (recorded
  in the scoreboard preamble).
- `--no-build` reuses the existing dist; `--no-platform` skips platform
  handling and the `otdf-local env` export (then pass `--no-audit-logs`).
- Everything after `--` goes to pytest. Run only the cells the phase
  targets; the full matrix runs in CI.

The platform is the one `otdf-local` manages. The script points
`OTDF_LOCAL_PLATFORM_DIR` at `xtest/platform/ref` in the tests repository
(ignored there) and creates it on first use as a `git worktree` of
`../platform` at the revision in `references.lock.json`, with `opentdf.yaml`
copied from `opentdf-dev.yaml` with `ec_tdf_enabled` switched on (as the
upstream workflow does; the reference `go` reader's EC-session re-decrypt in
`test_tdf_roundtrip` needs it) and the temporary keys from
`.github/scripts/init-temp-keys.sh`, so the pinned reference checkout is never
modified. `otdf-local up` builds the service with `go run`, provisions
Keycloak and the fixtures, and starts the additional KAS instances; its `env`
output supplies the audit-log paths the suite asserts by default. Stop it with
`uv run otdf-local down` in `../tests/otdf-local` when done.

Python tooling: the harness needs `uv` and Python 3.14 (`UV_PYTHON=3.14`;
3.15 cannot build `pydantic-core` yet). Keep local builds throttled as the
plan's working rules say (`systemd-run`, `GOMAXPROCS=4`).

## Scoreboard

`scripts/xtest-scoreboard.py <junit.xml>...` rewrites
`docs/xtest-scoreboard.md`: one row per test function in
[xtest-coverage.md](xtest-coverage.md) (tests the XML names that the coverage
document lacks are appended), one column per gc type, each cell the worst
result over every parametrised case that drove that type as encrypt or
decrypt SDK (`fail` > `skip:<reason>` > `pass`), `n/a` when the selection did
not run it. Rows for harness self-tests stay `n/a`. Commit the regenerated
file with each phase.

## Supported features

`SupportedFeatures()` lives in `src/version.go` with `Version`,
`TDFSpecVersion` and `ReferenceVersion` and is exported from the `sdk` root
of the generated package (`sdk.SupportedFeatures(ctx)`), which is what
`otdfgc --version --json` and `otdfgc supports` read. A feature name is added
there only in the change whose xtest cells prove it: `hexless` (4.3.0 output
and input) by the Phase 1 round trips, `kasallowlist` and
`better-messages-2024` by the Phase 2 tamper and allowlist cells. Phase 4 part 1
adds `autoconfigure`, `ns_grants` and `key_management` (the `test_abac.py`
autoconfigure, key mapping and base key cells) and `ecwrap`
(`test_tdf_roundtrip[ztdf-ecwrap]` and the EC key mapping cells), each
passing with gcgo as encrypt and as decrypt SDK against go and js.
