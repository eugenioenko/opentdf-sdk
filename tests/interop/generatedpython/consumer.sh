#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$1" && pwd)
PYTHON=${TDF_PYTHON_CONSUMER:-"$SDK/.local/python-tdf-library/consumer-venv/bin/python"}
exec "$PYTHON" -I "$SDK/tests/interop/generatedpython/consumer.py" "$@"
