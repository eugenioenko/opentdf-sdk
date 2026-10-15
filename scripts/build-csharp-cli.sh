#!/usr/bin/env bash
# Builds the native .NET CLI (src/hosts/csharp/cmd/otdfgc) against a generated
# C# package produced by scripts/build-generated-csharp.sh: a console project
# referencing the package's DLLs is built next to the package and the output
# is a launcher that runs it with the pinned .NET runtime.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=${GOALCHEMY_ROOT:-"$SDK/../goalchemy"}
GENERATED=${1:-"$ROOT/out/csharp-tdf-library/sdk"}
OUT=${2:-"$SDK/.local/csharp-tdf-library/otdfgc"}
GENERATED=$(cd "$GENERATED" && pwd)
if [[ "$OUT" != /* ]]; then OUT="$(pwd)/$OUT"; fi
BUILD=${OTDFGC_BUILD_DIR:-"$(dirname "$GENERATED")/otdfgc-csharp-build"}
DOTNET=${DOTNET_BIN:-"$ROOT/.toolchains/dotnet/dotnet"}
[[ -f "$GENERATED/lib/OpenTDF.TDF3.dll" ]] || { echo "generated package has no lib/OpenTDF.TDF3.dll: $GENERATED" >&2; exit 1; }
rm -rf "$BUILD"
mkdir -p "$BUILD" "$(dirname "$OUT")"
cp "$SDK/src/hosts/csharp/cmd/otdfgc/Program.cs" "$BUILD/Program.cs"
python3 - "$BUILD/otdfgc.csproj" "$GENERATED/lib" <<'PY'
import pathlib,sys,html
lib=pathlib.Path(sys.argv[2])
pathlib.Path(sys.argv[1]).write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><Nullable>disable</Nullable><AssemblyName>otdfgc</AssemblyName><RootNamespace>otdfgc</RootNamespace><InvariantGlobalization>true</InvariantGlobalization></PropertyGroup><ItemGroup><Reference Include="OpenTDF.TDF3"><HintPath>'+html.escape(str(lib/'OpenTDF.TDF3.dll'))+'</HintPath></Reference><Reference Include="System.IO.Hashing"><HintPath>'+html.escape(str(lib/'System.IO.Hashing.dll'))+'</HintPath></Reference><Reference Include="Google.Protobuf"><HintPath>'+html.escape(str(lib/'Google.Protobuf.dll'))+'</HintPath></Reference></ItemGroup></Project>\n')
PY
"$DOTNET" build "$BUILD/otdfgc.csproj" -c Release -o "$BUILD/bin" --nologo >&2
cat > "$OUT" <<LAUNCHER
#!/usr/bin/env bash
export DOTNET_CLI_TELEMETRY_OPTOUT=1 DOTNET_NOLOGO=1
exec "$DOTNET" "$BUILD/bin/otdfgc.dll" "\$@"
LAUNCHER
chmod +x "$OUT"
