#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=${GOALCHEMY_ROOT:-"$SDK/../goalchemy"}
COMPILER=${GOALCHEMY_BIN:-"$ROOT/out/swift-tdf-library/goalchemy"}
DEST=${1:-"$SDK/out/swift-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
(cd "$SDK"; GOALCHEMY_ROOT="$ROOT" "$COMPILER" compile -gate cooperative -target swift -out "$DEST" ./src/library)
mkdir -p "$DEST/Sources/OpenTDFTDF3"
cp "$SDK/src/hosts/swift/TDF3.swift.in" "$DEST/Sources/OpenTDFTDF3/TDF3.swift"
python3 - "$DEST" <<'PY'
from pathlib import Path
import sys
p = Path(sys.argv[1]) / 'Package.swift'
s = p.read_text()
s = s.replace('name: "GoalchemyGenerated",\n    products:', 'name: "OpenTDFTDF3",\n    products:', 1)
s = s.replace('products: [.library(name: "GoalchemyGenerated", targets: ["GoalchemyGenerated"])],',
              'products: [.library(name: "OpenTDFTDF3", targets: ["OpenTDFTDF3"])],', 1)
s = s.replace('targets: [\n', 'targets: [\n        .target(name: "OpenTDFTDF3", dependencies: ["GoalchemyGenerated"], path: "Sources/OpenTDFTDF3"),\n', 1)
s = s.replace('exclude: ["rt/native",', 'exclude: ["Sources", "rt/native",', 1)
p.write_text(s)
PY
cp "$SDK/src/hosts/swift/README.md.in" "$DEST/README.md"
swift build --package-path "$DEST" -c release
