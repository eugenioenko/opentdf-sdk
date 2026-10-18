#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=${GOALCHEMY_ROOT:-"$SDK/../goalchemy"}
COMPILER=${GOALCHEMY_BIN:-"$ROOT/out/rust-tdf-library/goalchemy"}
DEST=${1:-"$ROOT/out/rust-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
(cd "$SDK"; GOALCHEMY_ROOT="$ROOT" "$COMPILER" compile -gate cooperative --pause-model=native -target rust -out "$DEST" ./src)
mv "$DEST/src/lib.rs" "$DEST/src/generated.rs"
cp "$SDK/src/hosts/rust/lib.rs.in" "$DEST/src/lib.rs"
cp "$SDK/src/hosts/rust/sdk.rs.in" "$DEST/src/sdk.rs"
cp "$SDK/src/hosts/rust/services.rs.in" "$DEST/src/services.rs"
rm -rf "$DEST/src/protocol"
cp -r "$SDK/src/hosts/rust/protocol" "$DEST/src/protocol"
# The service clients' prost messages and pbjson serde codec.
python3 - "$DEST" <<'PY'
from pathlib import Path
import sys
p=Path(sys.argv[1])/'Cargo.toml';s=p.read_text().replace('name = "goalchemy-generated"','name = "opentdf-tdf3"')
s=s.replace('[dependencies]\n','[dependencies]\nprost = "=0.14.4"\npbjson = "=0.8.0"\npbjson-types = "=0.8.0"\nserde = "=1.0.229"\nserde_json = "=1.0.151"\n',1)
p.write_text(s)
PY
if [[ -f "$SDK/src/hosts/rust/Cargo.lock" ]]; then cp "$SDK/src/hosts/rust/Cargo.lock" "$DEST/Cargo.lock"; fi
cargo fetch --locked --manifest-path "$DEST/Cargo.toml"
python3 "$SDK/src/hosts/rust/dependency-inventory.py" "$DEST"
cmp "$SDK/src/hosts/rust/dependencies.lock.json" "$DEST/dependencies.lock.json"
cp "$SDK/src/hosts/rust/README.md.in" "$DEST/README.md"
cargo build --locked --offline --release --manifest-path "$DEST/Cargo.toml"
cargo package --locked --offline --no-verify --allow-dirty --manifest-path "$DEST/Cargo.toml"
