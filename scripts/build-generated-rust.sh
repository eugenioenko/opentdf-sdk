#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
COMPILER=${GOALCHEMY_BIN:-"$SDK/../goalchemy/out/rust-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/rust-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
(cd "$SDK"; GOALCHEMY_ROOT="$SDK/../goalchemy" "$COMPILER" compile -gate cooperative -target rust -out "$DEST" ./src/library)
mv "$DEST/src/lib.rs" "$DEST/src/generated.rs"
cp "$SDK/src/hosts/rust/lib.rs.in" "$DEST/src/lib.rs"
python3 - "$DEST" <<'PY'
from pathlib import Path
import sys
p=Path(sys.argv[1])/'Cargo.toml';s=p.read_text().replace('name = "goalchemy-generated"','name = "opentdf-tdf3"');p.write_text(s)
PY
if [[ -f "$SDK/src/hosts/rust/Cargo.lock" ]]; then cp "$SDK/src/hosts/rust/Cargo.lock" "$DEST/Cargo.lock"; fi
cargo fetch --locked --manifest-path "$DEST/Cargo.toml"
python3 "$SDK/src/hosts/rust/dependency-inventory.py" "$DEST"
cmp "$SDK/src/hosts/rust/dependencies.lock.json" "$DEST/dependencies.lock.json"
cp "$SDK/src/hosts/rust/README.md.in" "$DEST/README.md"
cargo build --locked --offline --release --manifest-path "$DEST/Cargo.toml"
cargo package --locked --offline --no-verify --allow-dirty --manifest-path "$DEST/Cargo.toml"
