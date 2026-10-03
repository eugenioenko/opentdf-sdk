#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "$0")/../../.." && pwd)
GOAL="$SDK/../goalchemy"
BASE=${TDF_RUST_PURE_OUT:-"$SDK/.local/rust-tdf-library/acceptance-repair-pure"}
mkdir -p "$BASE/source" "$BASE/consumer/src" "$BASE/executable-source"
printf 'module purevalue\n\ngo 1.25\nrequire github.com/eugenioenko/goalchemy v0.0.0\nreplace github.com/eugenioenko/goalchemy => %s\n' "$GOAL" > "$BASE/source/go.mod"
printf 'package plain\nfunc Echo(payload []byte) []byte { return payload }\n' > "$BASE/source/plain.go"
(cd "$BASE/source"; GOALCHEMY_ROOT="$GOAL" "$GOAL/out/rust-tdf-library/goalchemy" compile -gate cooperative -target rust -out "$BASE/emitted" .)
cat > "$BASE/consumer/Cargo.toml" <<TOML
[package]
name = "rust-pure-value-consumer"
version = "0.1.0"
edition = "2021"
[features]
default = ["native"]
native = ["goalchemy-generated/native"]
[dependencies]
goalchemy-generated = { path = "$BASE/emitted", default-features = false }
TOML
cat > "$BASE/consumer/src/main.rs" <<'RS'
use goalchemy_generated::{Echo, CallOptions};
fn main() {
    for _ in 0..20 {
        let input = vec![0, 255, 128, 0];
        assert_eq!(Echo(input.clone(), CallOptions::default()).wait().unwrap(), input);
        assert!(Echo(vec![], CallOptions::default()).wait().unwrap().is_empty());
    }
    println!("PASS independently importing pure Echo library: 40 normal/repeated/empty owned results");
}
RS
cargo generate-lockfile --offline --manifest-path "$BASE/consumer/Cargo.toml"
CARGO_TARGET_DIR="$GOAL/out/rust-tdf-library/sdk/target" cargo run --locked --offline --release --manifest-path "$BASE/consumer/Cargo.toml"
CARGO_TARGET_DIR="$GOAL/out/rust-tdf-library/sdk/target" cargo run --locked --offline --release --no-default-features --manifest-path "$BASE/consumer/Cargo.toml"
printf 'module pureexecutable\n\ngo 1.25\nrequire github.com/eugenioenko/goalchemy v0.0.0\nreplace github.com/eugenioenko/goalchemy => %s\n' "$GOAL" > "$BASE/executable-source/go.mod"
printf 'package main\nfunc main(){ if len([]byte{0,255})!=2 { panic("bytes") } }\n' > "$BASE/executable-source/main.go"
(cd "$BASE/executable-source"; GOALCHEMY_ROOT="$GOAL" "$GOAL/out/rust-tdf-library/goalchemy" compile -gate cooperative -target rust -out "$BASE/executable" .)
(cd "$BASE/executable"; sh run.sh)
printf 'PASS std-only generated executable\n'
