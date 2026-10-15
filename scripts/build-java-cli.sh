#!/usr/bin/env bash
# Builds the native Java CLI (src/hosts/java/cmd/otdfgc) against a generated
# Java package produced by scripts/build-generated-java.sh: the classes are
# compiled next to the package and the output is a launcher that runs them
# with the pinned JDK and the package's JARs on the classpath.
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
ROOT=${GOALCHEMY_ROOT:-"$SDK/../goalchemy"}
GENERATED=${1:-"$ROOT/out/java-tdf-library/sdk"}
OUT=${2:-"$SDK/.local/java-tdf-library/otdfgc"}
GENERATED=$(cd "$GENERATED" && pwd)
if [[ "$OUT" != /* ]]; then OUT="$(pwd)/$OUT"; fi
BUILD=${OTDFGC_BUILD_DIR:-"$(dirname "$GENERATED")/otdfgc-java-build"}
JDK=${JAVA_HOME:-"$ROOT/.toolchains/jdk-21.0.12.1+1"}
CP="$GENERATED/tdf3-java.jar:$GENERATED/lib/*"
[[ -f "$GENERATED/tdf3-java.jar" ]] || { echo "generated package has no tdf3-java.jar: $GENERATED" >&2; exit 1; }
rm -rf "$BUILD"
mkdir -p "$BUILD/classes" "$(dirname "$OUT")"
"$JDK/bin/javac" -encoding UTF-8 -cp "$CP" -d "$BUILD/classes" "$SDK/src/hosts/java/cmd/otdfgc/Main.java"
cat > "$OUT" <<LAUNCHER
#!/usr/bin/env bash
exec "$JDK/bin/java" -cp "$BUILD/classes:$CP" otdfgc.Main "\$@"
LAUNCHER
chmod +x "$OUT"
