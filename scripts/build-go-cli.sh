#!/usr/bin/env bash
# Builds the native Go CLI (src/hosts/go/cmd/otdfgc) against a generated Go
# package produced by scripts/build-generated-go.sh.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
GENERATED=${1:-"$SDK/../goalchemy/out/go-tdf-library/sdk"}
OUT=${2:-"$SDK/.local/go-tdf-library/otdfgc"}
GENERATED=$(cd "$GENERATED" && pwd)
if [[ "$OUT" != /* ]]; then OUT="$(pwd)/$OUT"; fi
BUILD=${OTDFGC_BUILD_DIR:-"$(dirname "$GENERATED")/otdfgc-build"}
rm -rf "$BUILD"
mkdir -p "$BUILD" "$(dirname "$OUT")"
cp "$SDK"/src/hosts/go/cmd/otdfgc/*.go "$SDK/src/hosts/go/cmd/otdfgc/go.mod" "$BUILD/"
(cd "$BUILD"; GOTOOLCHAIN=go1.25.14 go mod edit -replace "goalchemyout=$GENERATED"; GOTOOLCHAIN=go1.25.14 go build -o "$OUT" .)
