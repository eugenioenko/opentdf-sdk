#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=${GOALCHEMY_ROOT:-"$SDK/../goalchemy"}
COMPILER=${GOALCHEMY_BIN:-"$ROOT/out/swift-tdf-library/goalchemy"}
DEST=${1:-"$SDK/out/swift-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
(cd "$SDK"; GOALCHEMY_ROOT="$ROOT" "$COMPILER" compile -gate cooperative -target swift -out "$DEST" ./src/library ./src)
mkdir -p "$DEST/Sources/OpenTDFTDF3"
cp "$SDK/src/hosts/swift/TDF3.swift.in" "$DEST/Sources/OpenTDFTDF3/TDF3.swift"
cp "$SDK/src/hosts/swift/OpenTDF.swift.in" "$DEST/Sources/OpenTDFTDF3/OpenTDF.swift"
cp "$SDK/src/hosts/swift/Services.swift.in" "$DEST/Sources/OpenTDFTDF3/Services.swift"
rm -rf "$DEST/Sources/OpenTDFTDF3/Protocol"
cp -r "$SDK/src/hosts/swift/protocol" "$DEST/Sources/OpenTDFTDF3/Protocol"
# The service clients' SwiftProtobuf runtime, pinned to the protoc-gen-swift
# release that generated the messages (scripts/generate-protocol.sh).
python3 - "$DEST" "$SDK/scripts/generate-protocol.sh" <<'PY'
from pathlib import Path
import json,re,sys
p = Path(sys.argv[1]) / 'Package.swift'
pins = dict(re.findall(r'^(SWIFT_PROTOBUF_(?:VERSION|COMMIT))=(\S+)$', Path(sys.argv[2]).read_text(), re.M))
s = p.read_text()
s = s.replace('name: "GoalchemyGenerated",\n    products:', 'name: "OpenTDFTDF3",\n    products:', 1)
s = s.replace('products: [.library(name: "GoalchemyGenerated", targets: ["GoalchemyGenerated"])],',
              'products: [.library(name: "OpenTDFTDF3", targets: ["OpenTDFTDF3"])],\n    dependencies: [.package(url: "https://github.com/apple/swift-protobuf.git", exact: "' + pins['SWIFT_PROTOBUF_VERSION'] + '")],', 1)
s = s.replace('targets: [\n', 'targets: [\n        .target(name: "OpenTDFTDF3", dependencies: ["GoalchemyGenerated", .product(name: "SwiftProtobuf", package: "swift-protobuf")], path: "Sources/OpenTDFTDF3"),\n', 1)
s = s.replace('exclude: ["rt/native",', 'exclude: ["Sources", "rt/native",', 1)
p.write_text(s)
resolved = {'pins': [{'identity': 'swift-protobuf', 'kind': 'remoteSourceControl', 'location': 'https://github.com/apple/swift-protobuf.git',
                      'state': {'revision': pins['SWIFT_PROTOBUF_COMMIT'], 'version': pins['SWIFT_PROTOBUF_VERSION']}}], 'version': 2}
(Path(sys.argv[1]) / 'Package.resolved').write_text(json.dumps(resolved, indent=2) + '\n')
PY
cp "$SDK/src/hosts/swift/README.md.in" "$DEST/README.md"
if ! command -v swift >/dev/null && [[ -d "$ROOT/.toolchains/swift-6.4.0/usr/bin" ]]; then export PATH="$ROOT/.toolchains/swift-6.4.0/usr/bin:$PATH"; fi
if ! pkg-config --exists libcurl 2>/dev/null && [[ -d "$ROOT/.toolchains/pkgconfig" ]]; then export PKG_CONFIG_PATH="$ROOT/.toolchains/pkgconfig:${PKG_CONFIG_PATH:-}"; fi
swift build --package-path "$DEST" -c release
