#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "$0")/../../.." && pwd)
BASE=${TDF_C_CONSUMER_OUT:-"$SDK/.local/c-tdf-library/installed-consumer"}
PACKAGE=${TDF_C_PACKAGE:-"$SDK/../goalchemy/out/c-tdf-library/sdk/opentdf-tdf3-c-0.1.0-linux-x86_64.tar.gz"}
mkdir -p "$BASE/package"
tar -xzf "$PACKAGE" -C "$BASE/package"
CURL_PREFIX=${TDF3_CURL_PREFIX:-"$SDK/.local/root-c-development-prerequisites/prefix"}
${CC:-cc} -std=c17 -DGC_THREADS ${CFLAGS:--O2} -Wall -Wextra -Werror -Wno-deprecated-declarations -I"$BASE/package/include" -I"$SDK/../goalchemy/targets/c/tests/harness" -I"$SDK/../goalchemy/.toolchains/bdwgc/include" -I"$CURL_PREFIX/usr/include/x86_64-linux-gnu" "$SDK/tests/interop/generatedc/main.c" "$SDK/../goalchemy/targets/c/tests/harness/json.c" "$BASE/package/lib/libtdf3.a" -L"$CURL_PREFIX/usr/lib/x86_64-linux-gnu" -lcurl -lssl -lcrypto "$SDK/../goalchemy/.toolchains/bdwgc/lib/libgc.a" -lpthread -ldl -o "$BASE/consumer"
sha256sum "$PACKAGE" "$BASE/package/lib/libtdf3.a" "$BASE/consumer" > "$BASE/artifact-sha256.txt"
