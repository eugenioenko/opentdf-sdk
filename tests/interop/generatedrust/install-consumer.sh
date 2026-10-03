#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "$0")/../../.." && pwd)
BASE="$SDK/.local/rust-tdf-library"
PACKAGE="$SDK/../goalchemy/out/rust-tdf-library/sdk/target/package/opentdf-tdf3-0.1.0.crate"
mkdir -p "$BASE/installed" "$BASE/consumer/src"
tar -xzf "$PACKAGE" -C "$BASE/installed"
python3 - "$SDK" "$BASE" <<'PY'
import sys,pathlib
sdk,base=map(pathlib.Path,sys.argv[1:])
s=(sdk/'tests/interop/generatedrust/Cargo.toml.in').read_text().replace('@PACKAGE@',str(base/'installed/opentdf-tdf3-0.1.0'))
(base/'consumer/Cargo.toml').write_text(s)
(base/'consumer/src/main.rs').write_bytes((sdk/'tests/interop/generatedrust/main.rs').read_bytes())
PY
cargo fetch --manifest-path "$BASE/consumer/Cargo.toml"
CARGO_TARGET_DIR="$SDK/../goalchemy/out/rust-tdf-library/sdk/target" cargo build --locked --offline --release --manifest-path "$BASE/consumer/Cargo.toml"
mkdir -p "$BASE/consumer/target/release"
cp "$SDK/../goalchemy/out/rust-tdf-library/sdk/target/release/tdf3-native-consumer" "$BASE/consumer/target/release/tdf3-native-consumer"
