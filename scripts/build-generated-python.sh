#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
COMPILER=${GOALCHEMY_BIN:-"$SDK/../goalchemy/out/python-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/python-tdf-library/sdk"}
PYTHON=${TDF_PYTHON:-"$SDK/.local/python-tdf-library/venv/bin/python"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
rm -rf "$DEST/opentdf_tdf3" "$DEST/build" "$DEST/opentdf_tdf3.egg-info"
(cd "$SDK"; "$COMPILER" compile -gate cooperative -target python -out "$DEST/opentdf_tdf3/_generated" ./src/library)
cp "$SDK/src/hosts/python/__init__.py.in" "$DEST/opentdf_tdf3/__init__.py"
cp "$SDK/src/hosts/python/dependencies.lock.json" "$DEST/dependencies.lock.json"
cat > "$DEST/pyproject.toml" <<'TOML'
[build-system]
requires = ["setuptools==80.9.0", "wheel==0.45.1"]
build-backend = "setuptools.build_meta"
[project]
name = "opentdf-tdf3"
version = "0.1.0"
description = "Shared-source generated OpenTDF TDF3 byte SDK"
requires-python = ">=3.10"
dependencies = ["cryptography==50.0.2", "cffi==2.1.1", "pycparser==3.0", "typing-extensions==4.16.0"]
[tool.setuptools.packages.find]
where = ["."]
include = ["opentdf_tdf3*"]
TOML
mkdir -p "$DEST/opentdf_tdf3/licenses"
cp "$SDK/../goalchemy/LICENSE" "$DEST/opentdf_tdf3/licenses/Goalchemy-LICENSE"
"$PYTHON" - "$DEST" "${TDF_WHEELHOUSE:-$SDK/.local/python-tdf-library/wheels}" <<'PY'
import sys,pathlib,zipfile,hashlib,json,importlib.metadata
p=pathlib.Path(sys.argv[1]);wheelhouse=pathlib.Path(sys.argv[2])
lock=json.loads((p/'dependencies.lock.json').read_text())
for item in lock:
 artifact=wheelhouse/item['artifact']
 if not artifact.is_file() or hashlib.sha256(artifact.read_bytes()).hexdigest()!=item['sha256']:
  raise RuntimeError('dependency artifact differs from lock: '+item['artifact'])
 name=next(v[6:] for v in item['metadata'] if v.startswith('Name: '))
 version=next(v[9:] for v in item['metadata'] if v.startswith('Version: '))
 if importlib.metadata.version(name)!=version:raise RuntimeError('installed dependency version differs from lock: '+name)


for item in lock:
 wheel=wheelhouse/item['artifact']
 with zipfile.ZipFile(wheel) as z:
  for n in z.namelist():
   if not n.endswith('/') and ('/licenses/' in n or n.endswith('/LICENSE')):
    dest=p/'opentdf_tdf3/licenses'/wheel.name.split('-')[0]/pathlib.Path(n).relative_to(n.split('/')[0])
    dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(z.read(n))
(p/'opentdf_tdf3/py.typed').touch()
PY
cat >> "$DEST/pyproject.toml" <<'TOML'
[tool.setuptools.package-data]
opentdf_tdf3 = ["py.typed", "licenses/**/*"]
TOML
SOURCE_DATE_EPOCH=1700000000 "$PYTHON" -m pip wheel --no-deps --no-build-isolation --no-index -w "$DEST/dist" "$DEST"
"$PYTHON" - "$DEST" <<'PY'
import sys,pathlib,hashlib,json
p=pathlib.Path(sys.argv[1]);files={str(f.relative_to(p)):hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted(p.rglob('*')) if f.is_file() and f.name!='package-sha256.json' and '__pycache__' not in f.parts}
(p/'package-sha256.json').write_text(json.dumps(files,indent=2)+'\n')
PY
printf '%s\n' "$DEST/dist/opentdf_tdf3-0.1.0-py3-none-any.whl"
