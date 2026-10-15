#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=${GOALCHEMY_ROOT:-"$SDK/../goalchemy"}
COMPILER=${GOALCHEMY_BIN:-"$ROOT/out/java-tdf-library/goalchemy"}
DEST=${1:-"$ROOT/out/java-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
(cd "$SDK"; "$COMPILER" compile -gate cooperative -target java -out "$DEST" ./src/library ./src)
cmp "$SDK/src/hosts/java/dependencies.lock.json" "$ROOT/targets/java/dependencies.lock.json"
mkdir -p "$DEST/src/io/opentdf/tdf3" "$DEST/lib" "$DEST/licenses"
for adapter in "$SDK"/src/hosts/java/*.java.in; do cp "$adapter" "$DEST/src/io/opentdf/tdf3/$(basename "${adapter%.in}")"; done
rm -rf "$DEST/src/io/opentdf/tdf3/protocol"
cp -r "$SDK/src/hosts/java/protocol/io/opentdf/tdf3/protocol" "$DEST/src/io/opentdf/tdf3/protocol"
cp "$SDK/src/hosts/java/dependencies.lock.json" "$SDK/src/hosts/java/protocol-dependencies.lock.json" "$DEST/"
cp "$SDK"/src/hosts/java/licenses/* "$DEST/licenses/"
"$ROOT/targets/java/tests/crypto-dependencies.sh" "$DEST/lib" >/dev/null
# The service clients' protobuf-java runtime, verified against its lock.
python3 - "$SDK/src/hosts/java/protocol-dependencies.lock.json" "$DEST/lib" "${JAVA_DEPENDENCY_CACHE:-$SDK/.local/java-dependencies}" <<'PY'
import hashlib,json,os,pathlib,shutil,sys,tempfile,time,urllib.request
lock,dest,cache=sys.argv[1],pathlib.Path(sys.argv[2]),pathlib.Path(sys.argv[3])
cache.mkdir(parents=True,exist_ok=True)
for d in json.loads(pathlib.Path(lock).read_text())['dependencies']:
 cached=cache/f"{d['artifact']}-{d['version']}.jar"
 if not cached.exists() or hashlib.sha256(cached.read_bytes()).hexdigest()!=d['sha256']:
  fd,name=tempfile.mkstemp(prefix=cached.name+'.',suffix='.part',dir=cache);os.close(fd);part=pathlib.Path(name)
  try:
   for attempt in range(3):
    try:urllib.request.urlretrieve(d['url'],part);break
    except OSError as e:
     if attempt==2:raise
     print(f"retrying {d['url']} after {e}",file=sys.stderr);time.sleep(2**attempt)
   if hashlib.sha256(part.read_bytes()).hexdigest()!=d['sha256']:raise SystemExit('Java protocol dependency checksum mismatch: '+d['url'])
   part.replace(cached)
  finally:part.unlink(missing_ok=True)
 shutil.copyfile(cached,dest/cached.name)
PY
if [[ -z ${JAVA_HOME:-} && -x "$ROOT/.toolchains/jdk-21.0.12.1+1/bin/javac" ]]; then JAVA_HOME="$ROOT/.toolchains/jdk-21.0.12.1+1"; fi
if [[ -n ${JAVA_HOME:-} ]]; then PATH="$JAVA_HOME/bin:$PATH"; fi
python3 - "$DEST" <<'PY'
import json,pathlib,subprocess,sys
p=pathlib.Path(sys.argv[1]);manifest=json.loads((p/'goalchemy.manifest.json').read_text())
sources=sorted({p/name for name in manifest['generated_files']+manifest['runtime_files'] if name.endswith('.java')})
if not sources or any(not source.is_file() for source in sources):
 raise RuntimeError('generated Java source inventory is incomplete')
adapter=sorted(str(f) for f in (p/'src/io/opentdf/tdf3').rglob('*.java'))
(p/'javac-sources.txt').write_text('\n'.join([*map(str,sources),*adapter])+'\n')
subprocess.run(['javac','-nowarn','-encoding','UTF-8','-cp',str(p/'lib')+'/*','-d',str(p/'classes'),'@'+str(p/'javac-sources.txt')],check=True)
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
