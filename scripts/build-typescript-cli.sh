#!/usr/bin/env bash
# Builds the native Node CLI (src/hosts/typescript/cmd/otdfgc) against a
# generated TypeScript package produced by scripts/build-generated-typescript.sh:
# the CLI script is copied next to the package and the output is a launcher
# that runs it with the package's Node entry.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
GENERATED=${1:-"$SDK/../goalchemy/out/typescript-tdf-library/sdk"}
OUT=${2:-"$SDK/.local/typescript-tdf-library/otdfgc"}
GENERATED=$(cd "$GENERATED" && pwd)
if [[ "$OUT" != /* ]]; then OUT="$(pwd)/$OUT"; fi
BUILD=${OTDFGC_BUILD_DIR:-"$(dirname "$GENERATED")/otdfgc-build"}
rm -rf "$BUILD"
mkdir -p "$BUILD" "$(dirname "$OUT")"
cp "$SDK/src/hosts/typescript/cmd/otdfgc/otdfgc.mjs" "$BUILD/otdfgc.mjs"
printf '{"type":"module","private":true}\n' > "$BUILD/package.json"
[[ -f "$GENERATED/dist/node-index.js" ]] || { echo "generated package has no dist/node-index.js: $GENERATED" >&2; exit 1; }
cat > "$OUT" <<LAUNCHER
#!/usr/bin/env bash
export OTDFGC_TS_PACKAGE="$GENERATED/dist/node-index.js"
exec node "$BUILD/otdfgc.mjs" "\$@"
LAUNCHER
chmod +x "$OUT"
