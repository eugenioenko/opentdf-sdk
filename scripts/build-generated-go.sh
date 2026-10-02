#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
COMPILER=${GOALCHEMY_BIN:-"$SDK/../goalchemy/out/go-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/go-tdf-library/sdk"}
mkdir -p "$DEST"
(cd "$SDK"; "$COMPILER" compile -gate cooperative -target go -out "$DEST" ./library)
cp "$SDK/hosts/go/token.go.in" "$DEST/token.go"
GOTOOLCHAIN=go1.25.14 gofmt -w "$DEST/token.go"
(cd "$DEST"; GOTOOLCHAIN=go1.25.14 go build ./...)
