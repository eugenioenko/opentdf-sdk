#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
GOALCHEMY=${GOALCHEMY_DIR:-"$SDK/../goalchemy"}
COMPILER=${GOALCHEMY_BIN:-"$GOALCHEMY/out/go-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/go-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
ADAPTERS=(token options errors call services discovery)
HOST=()
for adapter in "${ADAPTERS[@]}"; do HOST+=("--go-host=src/hosts/go/$adapter.go.in"); done
(cd "$SDK"; "$COMPILER" compile -gate cooperative --pause-model=native "${HOST[@]}" -target go -out "$DEST" ./src)
for adapter in "${ADAPTERS[@]}"; do
  cp "$SDK/src/hosts/go/$adapter.go.in" "$DEST/$adapter.go"
  GOTOOLCHAIN=go1.25.14 gofmt -w "$DEST/$adapter.go"
done
# The service clients use the reference's protocol/go messages; the module
# requirements and checksums are pinned in the host adapter.
if [[ "$(cat "$DEST/go.mod")" != "$(printf 'module goalchemyout\n\ngo 1.25')" ]]; then
  echo "unexpected generated go.mod" >&2
  exit 1
fi
cp "$SDK/src/hosts/go/go.mod.in" "$DEST/go.mod"
cp "$SDK/src/hosts/go/go.sum.in" "$DEST/go.sum"
(cd "$DEST"; GOTOOLCHAIN=go1.25.14 go build ./...)
mkdir -p "$DEST/licenses"
cp "$GOALCHEMY/LICENSE" "$DEST/licenses/Goalchemy-LICENSE"
