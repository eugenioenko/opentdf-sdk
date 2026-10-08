#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"
# Set GOALCHEMY_BDWGC to your BDWGC 8.2.8 prefix. No compiler checkout is needed.
: "${GOALCHEMY_BDWGC:?Set GOALCHEMY_BDWGC to a BDWGC 8.2.8 installation}"
export CPPFLAGS="${CPPFLAGS:-} $(pkg-config --cflags libcurl)"
bash build.sh
${CC:-cc} -std=c17 ${CFLAGS:--O2} -Wall -Wextra -Werror -I. -c tdf3.c -o obj/tdf3.o
cp libgoalchemy.a libtdf3.a
ar rcs libtdf3.a obj/tdf3.o
