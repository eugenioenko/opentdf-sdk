#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "$0")/../../.." && pwd)
export LD_LIBRARY_PATH="${TDF3_CURL_PREFIX:-"$SDK/.local/root-c-development-prerequisites/prefix"}/usr/lib/x86_64-linux-gnu:${LD_LIBRARY_PATH:-}"
exec "${TDF_C_CONSUMER_OUT:-"$SDK/.local/c-tdf-library/installed-consumer"}/consumer" "$@"
