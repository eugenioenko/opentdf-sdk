#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$1" && pwd)
PACKAGE=${TDF_JAVA_PACKAGE:-"$SDK/../goalchemy/out/java-tdf-library/sdk"}
JDK=${JAVA_HOME:-"$SDK/../goalchemy/.toolchains/jdk-21.0.12.1+1"}
SOURCE="$SDK/tests/interop/generatedjava/Consumer.java"
CLASSES="$SDK/.local/java-tdf-library/consumer-classes"
CP="$PACKAGE/tdf3-java.jar:$PACKAGE/lib/bcprov-jdk18on-1.86.jar"
mkdir -p "$CLASSES"
if [[ ! -f "$CLASSES/consumer/Consumer.class" || "$SOURCE" -nt "$CLASSES/consumer/Consumer.class" ]]; then
 "$JDK/bin/javac" -encoding UTF-8 -cp "$CP" -d "$CLASSES" "$SOURCE"
fi
exec "$JDK/bin/java" -cp "$CLASSES:$CP" consumer.Consumer "$@"
