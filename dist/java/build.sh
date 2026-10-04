#!/bin/sh
set -eu
cd "$(dirname "$0")"
if [ -n "${JAVA_HOME:-}" ]; then PATH="$JAVA_HOME/bin:$PATH"; fi
mkdir -p classes
javac -nowarn -encoding UTF-8 -d classes Generated.java rt/types/*.java rt/runtime/*.java
jar --create --date=2026-01-01T00:00:00Z --file goalchemy-generated.jar -C classes .
