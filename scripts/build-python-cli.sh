#!/usr/bin/env bash
# Builds the native Python CLI (src/hosts/python/cmd/otdfgc) against a wheel
# produced by scripts/build-generated-python.sh: a virtual environment next to
# the generated package with the locked runtime wheels and the SDK wheel, the
# CLI module beside it, and a launcher at the output path.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
GENERATED=${1:-"$SDK/../goalchemy/out/python-tdf-library/sdk"}
OUT=${2:-"$SDK/.local/python-tdf-library/otdfgc"}
GENERATED=$(cd "$GENERATED" && pwd)
if [[ "$OUT" != /* ]]; then OUT="$(pwd)/$OUT"; fi
BUILD=${OTDFGC_BUILD_DIR:-"$(dirname "$GENERATED")/otdfgc-python-build"}
WHEELHOUSE=${TDF_WHEELHOUSE:-"$SDK/.local/python-tdf-library/wheels"}
PYTHON=${TDF_PYTHON_BASE:-python3.10}
command -v "$PYTHON" >/dev/null || PYTHON=python3
WHEEL=$(ls "$GENERATED"/dist/opentdf_tdf3-*.whl | head -n 1)
[[ -f "$WHEEL" ]] || { echo "generated package has no wheel under $GENERATED/dist" >&2; exit 1; }
rm -rf "$BUILD"
mkdir -p "$BUILD" "$(dirname "$OUT")"
"$PYTHON" -m venv "$BUILD/venv"
"$SDK/scripts/python-wheelhouse.sh" "$WHEELHOUSE" "$BUILD/venv/bin/python"
mapfile -t RUNTIME < <(python3 - "$SDK/src/hosts/python/dependencies.lock.json" "$WHEELHOUSE" <<'PY'
import json,sys
for item in json.load(open(sys.argv[1])):
    if item['role']=='runtime': print(sys.argv[2]+'/'+item['artifact'])
PY
)
"$BUILD/venv/bin/python" -m pip -q install --no-index --no-deps "${RUNTIME[@]}" "$WHEEL"
cp "$SDK/src/hosts/python/cmd/otdfgc/otdfgc.py" "$BUILD/otdfgc.py"
cat > "$OUT" <<LAUNCHER
#!/usr/bin/env bash
exec "$BUILD/venv/bin/python" -I "$BUILD/otdfgc.py" "\$@"
LAUNCHER
chmod +x "$OUT"
