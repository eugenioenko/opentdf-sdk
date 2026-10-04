#!/bin/sh
set -eu
dotnet build main.csproj -c Release -o lib --nologo
