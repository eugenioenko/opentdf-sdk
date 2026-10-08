#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$1" && pwd)
exec "${TDF_SWIFT_CONSUMER:-$SDK/.local/swift-tdf-library/consumer/.build/release/Consumer}" "$@"
