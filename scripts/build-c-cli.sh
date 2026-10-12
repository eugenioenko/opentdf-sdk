#!/usr/bin/env bash
# Builds the native C CLI (src/hosts/c/cmd/otdfgc) against a generated C
# library produced by scripts/build-generated-c.sh, linking libtdf3.a and the
# native dependencies recorded in its native-link.env; the output is a
# launcher that runs the binary with the libcurl runtime path.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
GENERATED=${1:-"$SDK/../goalchemy/out/c-tdf-library/sdk"}
OUT=${2:-"$SDK/.local/c-tdf-library/otdfgc"}
GENERATED=$(cd "$GENERATED" && pwd)
if [[ "$OUT" != /* ]]; then OUT="$(pwd)/$OUT"; fi
BUILD=${OTDFGC_BUILD_DIR:-"$(dirname "$GENERATED")/otdfgc-c-build"}
[[ -f "$GENERATED/native-link.env" ]] || { echo "generated package has no native-link.env: $GENERATED" >&2; exit 1; }
# shellcheck disable=SC1090
source "$GENERATED/native-link.env"
rm -rf "$BUILD"
mkdir -p "$BUILD" "$(dirname "$OUT")"
# shellcheck disable=SC2086
${CC:-cc} -std=c17 ${CFLAGS:--O2} -Wall -Wextra -Werror -I"$TDF3_INCLUDE" "$SDK/src/hosts/c/cmd/otdfgc/otdfgc.c" "$TDF3_LIBRARY" $TDF3_NATIVE_LIBS -o "$BUILD/otdfgc"
cat > "$OUT" <<LAUNCHER
#!/usr/bin/env bash
export LD_LIBRARY_PATH="$TDF3_RUNTIME_PATH:\${LD_LIBRARY_PATH:-}"
exec "$BUILD/otdfgc" "\$@"
LAUNCHER
chmod +x "$OUT"
