#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
COMPILER=${GOALCHEMY_BIN:-"$SDK/../goalchemy/out/csharp-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/csharp-tdf-library/sdk"}
mkdir -p "$DEST"
DEST=$(cd "$DEST" && pwd)
if [[ "$COMPILER" != /* ]]; then COMPILER="$(pwd)/$COMPILER"; fi
DOTNET=${DOTNET_BIN:-"$SDK/../goalchemy/.toolchains/dotnet/dotnet"}
(cd "$SDK"; "$COMPILER" compile -gate cooperative -target csharp -out "$DEST" ./library)
cp "$SDK/hosts/csharp/TDF3.cs.in" "$DEST/TDF3.cs"
cp "$SDK/hosts/csharp/dependencies.lock.json" "$DEST/dependencies.lock.json"
python3 - "$DEST/main.csproj" <<'PY'
import sys,pathlib,html
p=pathlib.Path(sys.argv[1]);p.write_text(p.read_text().replace('<TargetFramework>', '<AssemblyName>OpenTDF.TDF3</AssemblyName>\n    <Deterministic>true</Deterministic>\n    <PathMap>'+html.escape(str(p.parent))+'=/_/opentdf</PathMap>\n    <TargetFramework>'))
PY
"$DOTNET" build "$DEST/main.csproj" -c Release -o "$DEST/lib" --nologo
mkdir -p "$DEST/licenses"
cp "$SDK/../goalchemy/LICENSE" "$DEST/licenses/Goalchemy-LICENSE"
cp "$SDK/../goalchemy/.toolchains/dotnet/LICENSE.txt" "$DEST/licenses/DotNet-LICENSE"
cp "$SDK/../goalchemy/.toolchains/dotnet/ThirdPartyNotices.txt" "$DEST/licenses/DotNet-ThirdPartyNotices"
python3 - "$DEST" <<'PY'
import sys,pathlib,hashlib,json
p=pathlib.Path(sys.argv[1]);files={str(f.relative_to(p)):hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted(p.rglob('*')) if f.is_file() and f.name!='package-sha256.json'}
(p/'package-sha256.json').write_text(json.dumps(files,indent=2)+'\n')
PY
printf '%s\n' "$DEST/lib/OpenTDF.TDF3.dll"
