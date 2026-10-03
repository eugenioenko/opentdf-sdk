#!/usr/bin/env bash
set -euo pipefail
SDK=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
COMPILER=${GOALCHEMY_BIN:-"$SDK/../goalchemy/out/c-tdf-library/goalchemy"}
DEST=${1:-"$SDK/../goalchemy/out/c-tdf-library/sdk"}
mkdir -p "$DEST"
(cd "$SDK"; GOALCHEMY_ROOT="$SDK/../goalchemy" "$COMPILER" compile -gate cooperative -target c -out "$DEST" ./library)
export GOALCHEMY_BDWGC=${GOALCHEMY_BDWGC:-"$SDK/../goalchemy/.toolchains/bdwgc"}
CURL_PREFIX=${TDF3_CURL_PREFIX:-"$SDK/.local/root-c-development-prerequisites/prefix"}
if [[ -d "$CURL_PREFIX/usr/lib/x86_64-linux-gnu/pkgconfig" ]]; then
 export PKG_CONFIG_LIBDIR="$CURL_PREFIX/usr/lib/x86_64-linux-gnu/pkgconfig"
 export PKG_CONFIG_SYSROOT_DIR="$CURL_PREFIX" PKG_CONFIG_ALLOW_SYSTEM_LIBS=1
 export LD_LIBRARY_PATH="$CURL_PREFIX/usr/lib/x86_64-linux-gnu:${LD_LIBRARY_PATH:-}"
fi
CURL_CFLAGS=$(pkg-config --cflags libcurl)
CURL_LIBS=$(pkg-config --libs libcurl)
export CPPFLAGS="${CPPFLAGS:-} $CURL_CFLAGS"
sh "$DEST/build.sh"
cp "$SDK/hosts/c/tdf3.h" "$SDK/hosts/c/tdf3.c" "$DEST/"
${CC:-cc} -std=c17 ${CFLAGS:--O2} -Wall -Wextra -Werror -I"$DEST" -c "$DEST/tdf3.c" -o "$DEST/obj/tdf3.o"
cp "$DEST/libgoalchemy.a" "$DEST/libtdf3.a"
ar rcs "$DEST/libtdf3.a" "$DEST/obj/tdf3.o"
rm -rf "$DEST/package"
mkdir -p "$DEST/package/include" "$DEST/package/lib" "$DEST/package/src"
cp "$DEST/tdf3.h" "$DEST/goalchemy.h" "$DEST/library.h" "$DEST/package/include/"
cp "$DEST/libtdf3.a" "$DEST/package/lib/"
cp "$DEST/main.c" "$DEST/build.sh" "$DEST/tdf3.c" "$DEST/package/src/"
cp -r "$DEST/rt" "$DEST/package/src/"
cp "$DEST/tdf3.h" "$DEST/library.h" "$DEST/goalchemy.h" "$DEST/package/src/"
cp "$SDK/hosts/c/dependencies.lock.json" "$DEST/package/"
cp -r "$SDK/hosts/c/licenses" "$DEST/package/"
cp "$SDK/docs/generated-c-library.md" "$DEST/package/README.md"
cat > "$DEST/native-link.env" <<LINK
TDF3_INCLUDE=$DEST
TDF3_LIBRARY=$DEST/libtdf3.a
TDF3_NATIVE_LIBS=$CURL_LIBS -lssl -lcrypto $GOALCHEMY_BDWGC/lib/libgc.a -lpthread -ldl
TDF3_RUNTIME_PATH=$CURL_PREFIX/usr/lib/x86_64-linux-gnu
LINK
python3 - "$DEST" <<'PY'
import pathlib,hashlib,json,sys
p=pathlib.Path(sys.argv[1]);files={str(f.relative_to(p/'package')):hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted((p/'package').rglob('*')) if f.is_file()}
(p/'package-sha256.json').write_text(json.dumps(files,indent=2)+'\n')
PY
(cd "$DEST/package"; tar --sort=name --mtime=@0 --owner=0 --group=0 --numeric-owner -cf - . | gzip -n > ../opentdf-tdf3-c-0.1.0-linux-x86_64.tar.gz)
