#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "$0")/../../.." && pwd)
BASE=${TDF_C_FOCUSED_OUT:-"$SDK/.local/c-tdf-library"}
GOAL="$SDK/../goalchemy"
mkdir -p "$BASE/fixture" "$BASE/fixture-consumer"
cat > "$BASE/fixture/go.mod" <<MOD
module cfixture

go 1.25
require github.com/eugenioenko/goalchemy v0.0.0
replace github.com/eugenioenko/goalchemy => $GOAL
MOD
cp "$GOAL/targets/c/tests/library_fixture.go.in" "$BASE/fixture/fixture.go"
(cd "$BASE/fixture"; GOALCHEMY_ROOT="$GOAL" "$GOAL/out/c-tdf-library/goalchemy" compile -gate cooperative -target c -out "$BASE/fixture-library" .)
GOALCHEMY_BDWGC="$GOAL/.toolchains/bdwgc" sh "$BASE/fixture-library/build.sh"
(cd "$SDK"; GOTOOLCHAIN=go1.25.14 go run "$SDK/tests/interop/generatedc/native-proof.go" "$BASE/native-proof" produce)
${CC:-cc} -std=c17 ${CFLAGS:--O2} -Wall -Wextra -Werror -I"$BASE/fixture-library" -I"$GOAL/.toolchains/bdwgc/include" "$GOAL/targets/c/tests/library_consumer.c" "$BASE/fixture-library/libgoalchemy.a" -lssl -lcrypto "$GOAL/.toolchains/bdwgc/lib/libgc.a" -lpthread -ldl -o "$BASE/fixture-consumer/native"
"$BASE/fixture-consumer/native" "$BASE/native-proof"
(cd "$SDK"; GOTOOLCHAIN=go1.25.14 go run "$SDK/tests/interop/generatedc/native-proof.go" "$BASE/native-proof" verify)
