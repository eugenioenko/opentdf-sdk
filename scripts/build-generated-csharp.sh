#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=${GOALCHEMY_ROOT:-"$SDK/../goalchemy"}
COMPILER=${GOALCHEMY_BIN:-"$ROOT/out/csharp-tdf-library/goalchemy"}
DEST=${1:-"$ROOT/out/csharp-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
DOTNET=${DOTNET_BIN:-"$ROOT/.toolchains/dotnet/dotnet"}
(cd "$SDK"; "$COMPILER" compile -gate cooperative -target csharp -pause-model native -out "$DEST" ./src/library ./src)
cp "$SDK/src/hosts/csharp/TDF3.cs.in" "$DEST/TDF3.cs"
cp "$SDK/src/hosts/csharp/Sdk.cs.in" "$DEST/Sdk.cs"
cp "$SDK/src/hosts/csharp/Services.cs.in" "$DEST/Services.cs"
rm -rf "$DEST/protocol"
cp -r "$SDK/src/hosts/csharp/protocol" "$DEST/protocol"
cp "$SDK/src/hosts/csharp/dependencies.lock.json" "$DEST/dependencies.lock.json"
# The service clients' Google.Protobuf runtime joins the generated project's
# locked restore with the lock's content hash.
python3 - "$DEST/main.csproj" "$DEST/packages.lock.json" "$SDK/src/hosts/csharp/dependencies.lock.json" <<'PY'
import sys,pathlib,html,json
p=pathlib.Path(sys.argv[1]);s=p.read_text()
dependency=next(d for d in json.loads(pathlib.Path(sys.argv[3]).read_text())['dependencies'] if d['name']=='Google.Protobuf')
s=s.replace('<TargetFramework>', '<AssemblyName>OpenTDF.TDF3</AssemblyName>\n    <CopyLocalLockFileAssemblies>true</CopyLocalLockFileAssemblies>\n    <Deterministic>true</Deterministic>\n    <PathMap>'+html.escape(str(p.parent))+'=/_/opentdf</PathMap>\n    <TargetFramework>',1)
s=s.replace('  </ItemGroup>','    <PackageReference Include="Google.Protobuf" Version="['+dependency['version']+']" />\n  </ItemGroup>\n  <ItemGroup>\n    <Compile Include="protocol/**/*.cs" />\n  </ItemGroup>',1)
p.write_text(s)
lock=pathlib.Path(sys.argv[2]);packages=json.loads(lock.read_text())
packages['dependencies']['net8.0']['Google.Protobuf']={'type':'Direct','requested':'['+dependency['version']+', '+dependency['version']+']','resolved':dependency['version'],'contentHash':dependency['nuget_content_hash']}
packages['dependencies']['net8.0']=dict(sorted(packages['dependencies']['net8.0'].items()))
lock.write_text(json.dumps(packages,indent=2)+'\n')
PY
"$DOTNET" build "$DEST/main.csproj" -c Release -o "$DEST/lib" --nologo
mkdir -p "$DEST/licenses"
cp "$ROOT/LICENSE" "$DEST/licenses/Goalchemy-LICENSE"
cp "$ROOT/.toolchains/dotnet/LICENSE.txt" "$DEST/licenses/DotNet-LICENSE"
cp "$ROOT/.toolchains/dotnet/ThirdPartyNotices.txt" "$DEST/licenses/DotNet-ThirdPartyNotices"
NUGET_CACHE=$(DOTNET_CLI_UI_LANGUAGE=en "$DOTNET" nuget locals global-packages --list | sed -n 's/^global-packages: //p')
python3 - "$SDK" "$DEST" "$NUGET_CACHE" <<'PY'
import hashlib,json,pathlib,shutil,sys,zipfile
sdk,dest,cache=map(pathlib.Path,sys.argv[1:])
for dependency in json.loads((sdk/'src/hosts/csharp/dependencies.lock.json').read_text())['dependencies']:
 archive=cache/dependency['name'].lower()/dependency['version']/dependency['artifact']
 if hashlib.sha256(archive.read_bytes()).hexdigest()!=dependency['sha256']:
  raise SystemExit(dependency['name']+' package checksum mismatch')
 if not (dest/'lib'/(dependency['name']+'.dll')).is_file():
  raise SystemExit(dependency['name']+' runtime DLL missing from package')
 if dependency['name']=='System.IO.Hashing':
  with zipfile.ZipFile(archive) as package:
   for notice in ('LICENSE.TXT','THIRD-PARTY-NOTICES.TXT'):
    (dest/'licenses'/('System.IO.Hashing-'+notice)).write_bytes(package.read(notice))
 else:
  shutil.copyfile(sdk/'src/hosts/csharp/licenses'/(dependency['name']+'-LICENSE'),dest/'licenses'/(dependency['name']+'-LICENSE'))
PY
python3 - "$DEST" <<'PY'
import sys,pathlib,hashlib,json
p=pathlib.Path(sys.argv[1]);files={str(f.relative_to(p)):hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted(p.rglob('*')) if f.is_file() and f.name!='package-sha256.json'}
(p/'package-sha256.json').write_text(json.dumps(files,indent=2)+'\n')
PY
printf '%s\n' "$DEST/lib/OpenTDF.TDF3.dll"
