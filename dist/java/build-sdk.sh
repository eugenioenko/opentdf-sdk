#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
if [[ -n ${JAVA_HOME:-} ]]; then export PATH="$JAVA_HOME/bin:$PATH"; fi
mkdir -p lib classes
python3 - <<'PY'
import hashlib,json,pathlib,urllib.request
for dependency in json.loads(pathlib.Path('dependencies.lock.json').read_text())['dependencies']:
 p=pathlib.Path('lib')/(dependency['artifact']+'-'+dependency['version']+'.jar')
 if not p.exists():urllib.request.urlretrieve(dependency['url'],p)
 if hashlib.sha256(p.read_bytes()).hexdigest()!=dependency['sha256']:raise SystemExit('dependency checksum mismatch')
PY
python3 - <<'PY'
import json,pathlib,subprocess
manifest=json.loads(pathlib.Path('goalchemy.manifest.json').read_text())
sources=sorted({name for name in manifest['generated_files']+manifest['runtime_files'] if name.endswith('.java')})
if not sources or any(not pathlib.Path(name).is_file() for name in sources):
 raise RuntimeError('generated Java source inventory is incomplete')
subprocess.run(['javac','-nowarn','-encoding','UTF-8','-d','classes',
                *sources,'src/io/opentdf/tdf3/TDF3.java'],check=True)
PY
jar --create --date=2026-01-01T00:00:00Z --file tdf3-java.jar -C classes .
