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
| `src/hosts/go/cmd/otdfgc` | The native Go CLI over the generated package: `encrypt`, `decrypt`, `inspect`, `supports <feature>`, `--version --json`. Exit 0 on success, 1 when the operation fails (reference wording on stderr where the façade can produce it), 2 for usage errors and for every contract flag this build cannot honour (`unsupported in this build: <flag>`). |
| `scripts/build-go-cli.sh <generated-dir> <binary>` | Builds `otdfgc` against a package produced by `scripts/build-generated-go.sh`. |
| `tests: xtest/sdk/gc-cli.sh` | The one shim shared by every gc type; each type's Makefile copies it to `dist/<version>/cli.sh`. It maps the `XT_WITH_*` variables to `otdfgc` flags and answers `supports` for every name in `tdfs.feature_type` by asking `otdfgc supports`, which reads `SupportedFeatures()`; unknown names exit 2. |
| `tests: xtest/sdk/gc.mk`, `xtest/sdk/<type>/Makefile` | Build `dist/<version>/otdfgc` from `src/<version>` (a checkout or symlink of this repository): clone and build the Goalchemy release pinned in `references.lock.json`, run `scripts/build-generated-<target>.sh`, then the target's CLI build. Only `gcgo` builds today; the other types fail with "not built yet". |
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

- `--types` (default `gcgo`): the gc types to build; each gets
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
`TDFSpecVersion` and `ReferenceVersion`, and is exported through the façade
(`library.SupportedFeatures()`, `Version()`, `TDFSpecVersion()`), which is what
`otdfgc --version --json` and `otdfgc supports` read. A feature name is added
there only in the change whose xtest cells prove it; `hexless` (4.3.0 output
and input) is proven by the Phase 1 round trips.
