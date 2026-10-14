#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
COMPILER=${GOALCHEMY_BIN:-"$SDK/../goalchemy/out/typescript-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/typescript-tdf-library/sdk"}
# The package's runtime dependencies (@bufbuild/protobuf, @connectrpc/connect)
# from src/hosts/delivery/package-lock.json: the delivery tooling's install, or
# a local one.
DEPENDENCIES=${TDF_TS_NODE_MODULES:-${TDF_BROWSER_TOOLING:-}}
if [[ -z "$DEPENDENCIES" ]]; then
  LOCAL="$SDK/.local/typescript-dependencies"
  if ! cmp -s "$SDK/src/hosts/delivery/package-lock.json" "$LOCAL/package-lock.json"; then
    rm -rf "$LOCAL"
    mkdir -p "$LOCAL"
    cp "$SDK/src/hosts/delivery/package.json" "$SDK/src/hosts/delivery/package-lock.json" "$LOCAL/"
    (cd "$LOCAL"; npm ci --no-audit --no-fund --ignore-scripts >/dev/null)
  fi
  DEPENDENCIES="$LOCAL/node_modules"
fi
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
if [[ "$DEPENDENCIES" != /* ]]; then DEPENDENCIES="$(pwd)/$DEPENDENCIES"; fi
(cd "$SDK"; "$COMPILER" compile -gate cooperative -target typescript -out "$DEST" ./src/library ./src)
cp "$SDK/src/hosts/typescript/index.ts.in" "$DEST/index.ts"
cp "$SDK/src/hosts/typescript/node-index.ts.in" "$DEST/node-index.ts"
cp "$SDK/src/hosts/typescript/services.ts.in" "$DEST/services.ts"
rm -rf "$DEST/protocol"
cp -r "$SDK/src/hosts/typescript/protocol" "$DEST/protocol"
python3 - "$DEST" "$SDK/src/hosts/delivery/package.json" <<'PY'
import json,sys,pathlib
p=pathlib.Path(sys.argv[1]);tooling=json.loads(pathlib.Path(sys.argv[2]).read_text())['dependencies']
config=json.loads((p/'tsconfig.json').read_text());config['include'].extend(['index.ts','node-index.ts','services.ts','protocol/**/*.ts']);(p/'tsconfig.json').write_text(json.dumps(config,indent=2)+'\n')
package=json.loads((p/'package.json').read_text());package['name']='@opentdf-local/tdf3';package['files']=['dist','licenses']
package['exports']['.']={'types':'./dist/index.d.ts','node':'./dist/node-index.js','browser':'./dist/index.js','default':'./dist/index.js'}
package['exports']['./protocol/*']='./dist/protocol/*'
package['dependencies']={name:tooling[name] for name in ('@bufbuild/protobuf','@connectrpc/connect')}
(p/'package.json').write_text(json.dumps(package,indent=2)+'\n')
PY
# Linked for the build and for runs from this directory; npm pack leaves it out.
rm -rf "$DEST/node_modules"
ln -s "$DEPENDENCIES" "$DEST/node_modules"
(cd "$DEST"; "${TSC_BIN:-tsc}" -p .)
mkdir -p "$DEST/licenses"
cp "$SDK/../goalchemy/LICENSE" "$DEST/licenses/Goalchemy-LICENSE"
