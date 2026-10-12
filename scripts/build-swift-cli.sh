#!/usr/bin/env bash
# Builds the native Swift CLI (src/hosts/swift/cmd/otdfgc) against a generated
# SwiftPM package produced by scripts/build-generated-swift.sh: an executable
# package depending on the generated package by path is built next to it and
# the output is the release binary.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
GENERATED=${1:-"$SDK/out/swift-tdf-library/sdk"}
OUT=${2:-"$SDK/.local/swift-tdf-library/otdfgc"}
GENERATED=$(cd "$GENERATED" && pwd)
if [[ "$OUT" != /* ]]; then OUT="$(pwd)/$OUT"; fi
BUILD=${OTDFGC_BUILD_DIR:-"$(dirname "$GENERATED")/otdfgc-swift-build"}
[[ -f "$GENERATED/Package.swift" ]] || { echo "generated package has no Package.swift: $GENERATED" >&2; exit 1; }
rm -rf "$BUILD"
mkdir -p "$BUILD/Sources/otdfgc" "$(dirname "$OUT")"
sed -e "s|@PACKAGE@|$GENERATED|" -e "s|@PACKAGE_NAME@|$(basename "$GENERATED")|" "$SDK/src/hosts/swift/cmd/otdfgc/Package.swift.in" > "$BUILD/Package.swift"
cp "$SDK/src/hosts/swift/cmd/otdfgc/main.swift" "$BUILD/Sources/otdfgc/main.swift"
swift build --package-path "$BUILD" -c release
cp "$BUILD/.build/release/otdfgc" "$OUT"
