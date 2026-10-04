#!/bin/sh
# Builds libgoalchemy.a; link it with bdwgc (-lgc, or GOALCHEMY_BDWGC's
# libgc.a) and -lpthread, and include goalchemy.h.
set -e
cd "$(dirname "$0")"
inc=""
if [ -n "$GOALCHEMY_BDWGC" ]; then inc="-I$GOALCHEMY_BDWGC/include"
elif pkg-config --exists bdw-gc 2>/dev/null; then inc=$(pkg-config --cflags bdw-gc); fi
mkdir -p obj
objects=""
set -- main.c rt/types/*.c
for source in rt/runtime/*.c; do
  if [ -f "$source" ]; then set -- "$@" "$source"; fi
done
for f do
  object="obj/$(echo "$f" | tr / _).o"
  ${CC:-cc} -std=c17 ${CFLAGS:--O2} ${CPPFLAGS:-} -Irt/types $inc -c "$f" -o "$object"
  objects="$objects $object"
done
rm -f libgoalchemy.a
${AR:-ar} rcs libgoalchemy.a $objects
if [ -f tdf3.c ]; then
 ${CC:-cc} -std=c17 ${CFLAGS:--O2} ${CPPFLAGS:-} -I. -c tdf3.c -o obj/tdf3.o
 cp libgoalchemy.a libtdf3.a
 ${AR:-ar} rcs libtdf3.a obj/tdf3.o
fi
