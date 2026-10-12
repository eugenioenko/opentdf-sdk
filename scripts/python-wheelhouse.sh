#!/usr/bin/env bash
# Fills a wheelhouse with the wheels of src/hosts/python/dependencies.lock.json
# (downloading the missing ones) and verifies every artifact's SHA-256 against
# the lock. Usage: python-wheelhouse.sh <wheelhouse> [python]
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
WHEELHOUSE=${1:?wheelhouse directory}
PYTHON=${2:-python3}
mkdir -p "$WHEELHOUSE"
"$PYTHON" - "$SDK/src/hosts/python/dependencies.lock.json" "$WHEELHOUSE" <<'PY'
import hashlib,json,pathlib,subprocess,sys
lock=json.load(open(sys.argv[1]));wheels=pathlib.Path(sys.argv[2])
for item in lock:
    artifact=wheels/item['artifact']
    if not artifact.exists():
        name=next(v[6:] for v in item['metadata'] if v.startswith('Name: '))
        version=next(v[9:] for v in item['metadata'] if v.startswith('Version: '))
        subprocess.run([sys.executable,'-m','pip','-q','download','--no-deps','--only-binary=:all:','-d',str(wheels),name+'=='+version],check=True)
    if hashlib.sha256(artifact.read_bytes()).hexdigest()!=item['sha256']:
        raise SystemExit('wheel differs from lock: '+item['artifact'])
PY
