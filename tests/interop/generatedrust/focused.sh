#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "$0")/../../.." && pwd)
BASE="$SDK/.local/rust-tdf-library"
GOAL="$SDK/../goalchemy"
mkdir -p "$BASE/fixture/src" "$BASE/fixture-consumer/src"
cat > "$BASE/fixture/go.mod" <<MOD
module rustfixture

go 1.25
require github.com/eugenioenko/goalchemy v0.0.0
replace github.com/eugenioenko/goalchemy => $GOAL
MOD
cp "$GOAL/targets/rust/tests/library_fixture.go.in" "$BASE/fixture/fixture.go"
(cd "$BASE/fixture"; GOALCHEMY_ROOT="$GOAL" "$GOAL/out/rust-tdf-library/goalchemy" compile -gate cooperative -target rust -out "$BASE/fixture-library" .)
cat > "$BASE/fixture-consumer/Cargo.toml" <<TOML
[package]
name = "rust-fixture-consumer"
version = "0.1.0"
edition = "2021"
[dependencies]
goalchemy-generated = { path = "$BASE/fixture-library" }
TOML
cp "$GOAL/targets/rust/tests/library_consumer.rs.in" "$BASE/fixture-consumer/src/main.rs"
(cd "$SDK"; GOTOOLCHAIN=go1.25.14 go run "$SDK/tests/interop/generatedrust/native-proof.go" "$BASE/native-proof" produce)
RUST_NATIVE_PROOF="$BASE/native-proof" GOALCHEMY_GC_THRESHOLD=1 CARGO_TARGET_DIR="$GOAL/out/rust-tdf-library/sdk/target" cargo run --offline --release --manifest-path "$BASE/fixture-consumer/Cargo.toml"
(cd "$SDK"; GOTOOLCHAIN=go1.25.14 go run "$SDK/tests/interop/generatedrust/native-proof.go" "$BASE/native-proof" verify)
