#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$1" && pwd)
PACKAGE=${TDF_CSHARP_PACKAGE:-"$SDK/../goalchemy/out/csharp-tdf-library/sdk"}
DOTNET=${DOTNET_BIN:-"$SDK/../goalchemy/.toolchains/dotnet/dotnet"}
BUILD="$SDK/.local/csharp-tdf-library/consumer"
mkdir -p "$BUILD"
STAMP=$(sha256sum "$PACKAGE/lib/OpenTDF.TDF3.dll" "$SDK/tests/interop/generatedcsharp/Consumer.cs" | sha256sum | cut -d' ' -f1)
if [[ ! -f "$BUILD/stamp" ]] || [[ $(cat "$BUILD/stamp") != "$STAMP" ]]; then
  cp "$SDK/tests/interop/generatedcsharp/Consumer.cs" "$BUILD/Consumer.cs"
  python3 - "$BUILD/consumer.csproj" "$PACKAGE/lib/OpenTDF.TDF3.dll" <<'PY'
import pathlib,sys,html
pathlib.Path(sys.argv[1]).write_text('<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net8.0</TargetFramework><Nullable>disable</Nullable></PropertyGroup><ItemGroup><Reference Include="OpenTDF.TDF3"><HintPath>'+html.escape(str(pathlib.Path(sys.argv[2]).resolve()))+'</HintPath></Reference></ItemGroup></Project>\n')
PY
  "$DOTNET" build "$BUILD/consumer.csproj" -c Release -o "$BUILD/bin" --nologo >&2
  printf '%s\n' "$STAMP" > "$BUILD/stamp"
fi
exec "$DOTNET" "$BUILD/bin/consumer.dll" "$@"
