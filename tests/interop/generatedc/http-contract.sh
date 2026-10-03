#!/usr/bin/env bash
# SPDX-License-Identifier: Apache-2.0
set -euo pipefail
SDK=$(cd "$(dirname "$0")/../../.." && pwd)
ROOT="$SDK/../goalchemy"
COMPILER=${GOALCHEMY_BIN:-"$ROOT/out/c-tdf-library/goalchemy"}
BASE=${TDF_C_HTTP_OUT:-"$SDK/.local/c-tdf-library/http-contract"}
CURL_PREFIX=${TDF3_CURL_PREFIX:-"$SDK/.local/root-c-development-prerequisites/prefix"}
mkdir -p "$BASE"
(cd "$SDK/tests/interop/generatedc/httpfixture"; GOALCHEMY_ROOT="$ROOT" "$COMPILER" compile -gate cooperative -target c -out "$BASE/emitted" .)
export GOALCHEMY_BDWGC=${GOALCHEMY_BDWGC:-"$ROOT/.toolchains/bdwgc"}
export CPPFLAGS="${CPPFLAGS:-} -I$CURL_PREFIX/usr/include/x86_64-linux-gnu"
sh "$BASE/emitted/build.sh"
${CC:-cc} -std=c17 ${CFLAGS:--O2} -Wall -Wextra -Werror -I"$BASE/emitted" "$SDK/tests/interop/generatedc/http-consumer.c" "$BASE/emitted/libgoalchemy.a" "$GOALCHEMY_BDWGC/lib/libgc.a" -L"$CURL_PREFIX/usr/lib/x86_64-linux-gnu" -lcurl -lssl -lcrypto -lpthread -ldl -o "$BASE/consumer"
export LD_LIBRARY_PATH="$CURL_PREFIX/usr/lib/x86_64-linux-gnu:${LD_LIBRARY_PATH:-}"
python3 "$SDK/tests/interop/generatedc/http-contract.py" "$BASE/consumer" "$BASE/cases"
sha256sum "$BASE/emitted/libgoalchemy.a" "$SDK/tests/interop/generatedc/http-consumer.c" "$BASE/consumer" > "$BASE/artifact-sha256.txt"
