#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
COMPILER=${GOALCHEMY_BIN:-"$SDK/../goalchemy/out/typescript-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/typescript-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
(cd "$SDK"; "$COMPILER" compile -gate cooperative -target typescript -out "$DEST" ./library)
cp "$SDK/hosts/typescript/index.ts.in" "$DEST/index.ts"
cp "$SDK/hosts/typescript/node-index.ts.in" "$DEST/node-index.ts"
python3 - "$DEST" <<'PY'
import json,sys,pathlib
p=pathlib.Path(sys.argv[1]);config=json.loads((p/'tsconfig.json').read_text());config['include'].extend(['index.ts','node-index.ts']);(p/'tsconfig.json').write_text(json.dumps(config,indent=2)+'\n');package=json.loads((p/'package.json').read_text());package['name']='@opentdf-local/tdf3';package['files']=['dist','licenses'];package['exports']['.']={'types':'./dist/index.d.ts','node':'./dist/node-index.js','browser':'./dist/index.js','default':'./dist/index.js'};(p/'package.json').write_text(json.dumps(package,indent=2)+'\n')
PY
(cd "$DEST"; "${TSC_BIN:-tsc}" -p .)
mkdir -p "$DEST/licenses"
cp "$SDK/../goalchemy/LICENSE" "$DEST/licenses/Goalchemy-LICENSE"
