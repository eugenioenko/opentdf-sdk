#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
COMPILER=${GOALCHEMY_BIN:-"$SDK/../goalchemy/out/java-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/java-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
(cd "$SDK"; "$COMPILER" compile -gate cooperative -target java -out "$DEST" ./src/library)
cmp "$SDK/src/hosts/java/dependencies.lock.json" "$SDK/../goalchemy/targets/java/dependencies.lock.json"
mkdir -p "$DEST/src/io/opentdf/tdf3" "$DEST/lib" "$DEST/licenses"
cp "$SDK/src/hosts/java/TDF3.java.in" "$DEST/src/io/opentdf/tdf3/TDF3.java"
cp "$SDK/src/hosts/java/dependencies.lock.json" "$DEST/dependencies.lock.json"
"$SDK/../goalchemy/targets/java/tests/crypto-dependencies.sh" "$DEST/lib" >/dev/null
if [[ -n ${JAVA_HOME:-} ]]; then PATH="$JAVA_HOME/bin:$PATH"; fi
python3 - "$DEST" <<'PY'
import json,pathlib,subprocess,sys
p=pathlib.Path(sys.argv[1]);manifest=json.loads((p/'goalchemy.manifest.json').read_text())
sources=sorted({p/name for name in manifest['generated_files']+manifest['runtime_files'] if name.endswith('.java')})
if not sources or any(not source.is_file() for source in sources):
 raise RuntimeError('generated Java source inventory is incomplete')
subprocess.run(['javac','-nowarn','-encoding','UTF-8','-d',str(p/'classes'),
                *map(str,sources),str(p/'src/io/opentdf/tdf3/TDF3.java')],check=True)
PY
jar --create --date=2026-01-01T00:00:00Z --file "$DEST/tdf3-java.jar" -C "$DEST/classes" .
python3 - "$DEST" "$SDK" <<'PY'
import sys,pathlib,zipfile,hashlib,json
p=pathlib.Path(sys.argv[1]);sdk=pathlib.Path(sys.argv[2]);jar=p/'lib/bcprov-jdk18on-1.86.jar'
with zipfile.ZipFile(jar) as z:(p/'licenses/Bouncy-Castle-LICENSE.md').write_bytes(z.read('META-INF/LICENSE.md'))
(p/'licenses/Goalchemy-LICENSE').write_bytes((sdk/'../goalchemy/LICENSE').resolve().read_bytes())
files={str(f.relative_to(p)):hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted(p.rglob('*')) if f.is_file() and f.name!='package-sha256.json'}
(p/'package-sha256.json').write_text(json.dumps(files,indent=2)+'\n')
PY
printf '%s\n' "$DEST/tdf3-java.jar"
