#!/usr/bin/env bash
# Builds the native Rust CLI (src/hosts/rust/cmd/otdfgc) against a generated
# Rust crate produced by scripts/build-generated-rust.sh: a Cargo binary crate
# depending on the generated crate by path is built next to the package and
# the output is the release binary.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
GENERATED=${1:-"$SDK/../goalchemy/out/rust-tdf-library/sdk"}
OUT=${2:-"$SDK/.local/rust-tdf-library/otdfgc"}
GENERATED=$(cd "$GENERATED" && pwd)
if [[ "$OUT" != /* ]]; then OUT="$(pwd)/$OUT"; fi
BUILD=${OTDFGC_BUILD_DIR:-"$(dirname "$GENERATED")/otdfgc-rust-build"}
[[ -f "$GENERATED/Cargo.toml" ]] || { echo "generated package has no Cargo.toml: $GENERATED" >&2; exit 1; }
rm -rf "$BUILD"
mkdir -p "$BUILD/src" "$(dirname "$OUT")"
sed "s|@PACKAGE@|$GENERATED|" "$SDK/src/hosts/rust/cmd/otdfgc/Cargo.toml.in" > "$BUILD/Cargo.toml"
cp "$SDK/src/hosts/rust/cmd/otdfgc/main.rs" "$BUILD/src/main.rs"
cp "$GENERATED/Cargo.lock" "$BUILD/Cargo.lock"
cargo fetch --manifest-path "$BUILD/Cargo.toml"
CARGO_TARGET_DIR="$BUILD/target" cargo build --offline --release --manifest-path "$BUILD/Cargo.toml"
cp "$BUILD/target/release/otdfgc" "$OUT"
