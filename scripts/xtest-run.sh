#!/usr/bin/env bash
# Builds the requested generated SDK types into the tests repository, runs a
# pytest selection against the local platform with JUnit output, and
# regenerates docs/xtest-scoreboard.md. See docs/xtest.md.
#
#   scripts/xtest-run.sh [--types gcgo[,..]] [--version main] [--sdks "go gcgo"]
#       [--junit <file>] [--label <text>] [--no-platform] [--no-build] -- <pytest args>
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
TESTS=${XTEST_REPO:-"$SDK/../tests"}
TYPES=gcgo
VERSION=main
SDKS=""
JUNIT=""
LABEL=""
PLATFORM=1
BUILD=1
while [[ $# -gt 0 ]]; do
  case "$1" in
    --types) TYPES=$2; shift 2 ;;
    --version) VERSION=$2; shift 2 ;;
    --sdks) SDKS=$2; shift 2 ;;
    --junit) JUNIT=$2; shift 2 ;;
    --label) LABEL=$2; shift 2 ;;
    --no-platform) PLATFORM=0; shift ;;
    --no-build) BUILD=0; shift ;;
    --) shift; break ;;
    *) echo "unknown option: $1" >&2; exit 2 ;;
  esac
done
if [[ $# -eq 0 ]]; then
  echo 'usage: scripts/xtest-run.sh [options] -- <pytest selection and options>' >&2
  exit 2
fi
if [[ ! -d "$TESTS/xtest" ]]; then
  echo "tests repository not found at $TESTS; set XTEST_REPO" >&2
  exit 1
fi
TESTS=$(cd "$TESTS" && pwd)
export PATH="$HOME/.local/bin:$PATH" UV_PYTHON="${UV_PYTHON:-3.14}" GOMAXPROCS="${GOMAXPROCS:-4}"
if [[ -z "$SDKS" ]]; then SDKS=${TYPES//,/ }; fi
if [[ -z "$JUNIT" ]]; then
  mkdir -p "$SDK/out/xtest"
  JUNIT="$SDK/out/xtest/$(date -u +%Y%m%dT%H%M%SZ).xml"
fi
if [[ "$JUNIT" != /* ]]; then JUNIT="$(pwd)/$JUNIT"; fi

if [[ $BUILD == 1 ]]; then
  for type in ${TYPES//,/ }; do
    mkdir -p "$TESTS/xtest/sdk/$type/src"
    ln -sfn "$SDK" "$TESTS/xtest/sdk/$type/src/$VERSION"
    make -C "$TESTS/xtest/sdk" "$type" "VERSIONS=$VERSION"
  done
fi

if [[ $PLATFORM == 1 ]]; then
  PLATFORM_DIR=${OTDF_LOCAL_PLATFORM_DIR:-"$TESTS/xtest/platform/ref"}
  if [[ ! -d "$PLATFORM_DIR" ]]; then
    REVISION=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["repositories"]["platform"]["revision"])' "$SDK/references.lock.json")
    echo "Creating the pinned platform checkout $PLATFORM_DIR at $REVISION"
    git -C "$SDK/../platform" worktree add --detach "$PLATFORM_DIR" "$REVISION"
    # Same template as the upstream workflow: the dev config with EC wrapping on.
    sed 's/^\( *\)ec_tdf_enabled: false/\1ec_tdf_enabled: true/' "$PLATFORM_DIR/opentdf-dev.yaml" > "$PLATFORM_DIR/opentdf.yaml"
    (cd "$PLATFORM_DIR" && bash .github/scripts/init-temp-keys.sh)
  fi
  export OTDF_LOCAL_PLATFORM_DIR="$PLATFORM_DIR"
  if ! curl -fsS --max-time 3 http://localhost:8080/healthz >/dev/null 2>&1; then
    (cd "$TESTS/otdf-local" && uv run otdf-local up)
  fi
  eval "$(cd "$TESTS/otdf-local" && uv run otdf-local env)"
fi

cd "$TESTS/xtest"
set -a
# shellcheck disable=SC1091
source test.env
set +a
status=0
uv run pytest "$@" --sdks "$SDKS" --junitxml "$JUNIT" || status=$?
echo "JUnit results: $JUNIT"
python3 "$SDK/scripts/xtest-scoreboard.py" --label "$LABEL" "$JUNIT"
exit $status
