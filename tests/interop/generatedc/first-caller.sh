#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail
SDK=$(cd "$(dirname "$0")/../../.." && pwd)
BASE=${TDF_C_FIRST_OUT:-"$SDK/.local/c-tdf-library/first-caller"}
PACKAGE=${TDF_C_PACKAGE:-"$SDK/../goalchemy/out/c-tdf-library/sdk/opentdf-tdf3-c-0.1.0-linux-x86_64.tar.gz"}
CURL_PREFIX=${TDF3_CURL_PREFIX:-"$SDK/.local/root-c-development-prerequisites/prefix"}
mkdir -p "$BASE/package"
tar -xzf "$PACKAGE" -C "$BASE/package"
${CC:-cc} -std=c17 ${CFLAGS:--O2} -Wall -Wextra -Werror -I"$BASE/package/include" -I"$SDK/../goalchemy/.toolchains/bdwgc/include" "$SDK/tests/interop/generatedc/first-caller.c" "$BASE/package/lib/libtdf3.a" -L"$CURL_PREFIX/usr/lib/x86_64-linux-gnu" -lcurl -lssl -lcrypto "$SDK/../goalchemy/.toolchains/bdwgc/lib/libgc.a" -lpthread -ldl -o "$BASE/consumer"
export LD_LIBRARY_PATH="$CURL_PREFIX/usr/lib/x86_64-linux-gnu:${LD_LIBRARY_PATH:-}"
for mode in ${TDF_C_FIRST_CASES:-main-async transient-async transient-sync main-sync external-async external-sync external-registered-sync overlap}; do
 set +e
 timeout 45 "$BASE/consumer" "$mode" > "$BASE/$mode.log" 2>&1
 result=$?
 set -e
 echo "$result" > "$BASE/$mode.status"
 if [[ "$result" != 0 ]]; then cat "$BASE/$mode.log"; exit "$result"; fi
 echo "PASS $mode"
done
sha256sum "$PACKAGE" "$BASE/package/lib/libtdf3.a" "$SDK/tests/interop/generatedc/first-caller.c" "$BASE/consumer" > "$BASE/artifact-sha256.txt"
