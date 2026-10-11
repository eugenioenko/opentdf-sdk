#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
GOALCHEMY=${GOALCHEMY_DIR:-"$SDK/../goalchemy"}
COMPILER=${GOALCHEMY_BIN:-"$GOALCHEMY/out/go-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/go-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
(cd "$SDK"; "$COMPILER" compile -gate cooperative -target go -out "$DEST" ./src/library)
cp "$SDK/src/hosts/go/token.go.in" "$DEST/token.go"
GOTOOLCHAIN=go1.25.14 gofmt -w "$DEST/token.go"
(cd "$DEST"; GOTOOLCHAIN=go1.25.14 go build ./...)
mkdir -p "$DEST/licenses"
cp "$GOALCHEMY/LICENSE" "$DEST/licenses/Goalchemy-LICENSE"
