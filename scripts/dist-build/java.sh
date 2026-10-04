#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
mkdir -p lib classes
python3 - <<'PY'
import hashlib,json,pathlib,urllib.request
for dependency in json.loads(pathlib.Path('dependencies.lock.json').read_text())['dependencies']:
 p=pathlib.Path('lib')/(dependency['artifact']+'-'+dependency['version']+'.jar')
 if not p.exists():urllib.request.urlretrieve(dependency['url'],p)
 if hashlib.sha256(p.read_bytes()).hexdigest()!=dependency['sha256']:raise SystemExit('dependency checksum mismatch')
PY
javac -nowarn -encoding UTF-8 -d classes Generated.java rt/types/*.java rt/runtime/*.java src/io/opentdf/tdf3/TDF3.java
jar --create --date=2026-01-01T00:00:00Z --file tdf3-java.jar -C classes .
