#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$1" && pwd)
exec "${TDF_RUST_CONSUMER:-$SDK/.local/rust-tdf-library/consumer/target/release/tdf3-native-consumer}" "$@"
