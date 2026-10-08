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
set -- \
    'shared.c' \
    'pkg_json_40c8871c00fc045f.c' \
    'pkg_tdf_7dcb6b0a0cf33145.c' \
    'pkg_src_ae63b81fa18b54da.c' \
    'pkg_library_de7cd4c7ad526690.c' \
    'main.c' \
    'rt/runtime/chan_close.c' \
    'rt/runtime/chan_make.c' \
    'rt/runtime/chan_recv.c' \
    'rt/runtime/chan_send.c' \
    'rt/runtime/integer_add.c' \
    'rt/runtime/integer_and.c' \
    'rt/runtime/integer_convert.c' \
    'rt/runtime/integer_div.c' \
    'rt/runtime/integer_mul.c' \
    'rt/runtime/integer_neg.c' \
    'rt/runtime/integer_or.c' \
    'rt/runtime/integer_rem.c' \
    'rt/runtime/integer_shl.c' \
    'rt/runtime/integer_shr.c' \
    'rt/runtime/integer_sub.c' \
    'rt/runtime/lib_callback_request.c' \
    'rt/runtime/lib_checksum_crc32_ieee.c' \
    'rt/runtime/lib_clock_unix.c' \
    'rt/runtime/lib_crypto_aes256_gcm_decrypt.c' \
    'rt/runtime/lib_crypto_aes256_gcm_encrypt.c' \
    'rt/runtime/lib_crypto_close.c' \
    'rt/runtime/lib_crypto_ecdh.c' \
    'rt/runtime/lib_crypto_es256_sign.c' \
    'rt/runtime/lib_crypto_generate_p256.c' \
    'rt/runtime/lib_crypto_generate_rsa2048.c' \
    'rt/runtime/lib_crypto_hkdf_sha256.c' \
    'rt/runtime/lib_crypto_hmac_sha256.c' \
    'rt/runtime/lib_crypto_hmac_sha256_verify.c' \
    'rt/runtime/lib_crypto_import_pem.c' \
    'rt/runtime/lib_crypto_public_jwk.c' \
    'rt/runtime/lib_crypto_public_pem.c' \
    'rt/runtime/lib_crypto_random.c' \
    'rt/runtime/lib_crypto_rs256_sign.c' \
    'rt/runtime/lib_crypto_rsa_oaep_decrypt.c' \
    'rt/runtime/lib_crypto_rsa_oaep_encrypt.c' \
    'rt/runtime/lib_crypto_sha256.c' \
    'rt/runtime/lib_encoding_base64_decode.c' \
    'rt/runtime/lib_encoding_base64_encode.c' \
    'rt/runtime/lib_encoding_base64_url_decode.c' \
    'rt/runtime/lib_encoding_base64_url_encode.c' \
    'rt/runtime/lib_http_do.c' \
    'rt/runtime/map_lookup.c' \
    'rt/runtime/map_make.c' \
    'rt/runtime/map_store.c' \
    'rt/runtime/select.c' \
    'rt/runtime/slice_append.c' \
    'rt/runtime/slice_copy.c' \
    'rt/runtime/slice_index.c' \
    'rt/runtime/slice_make.c' \
    'rt/runtime/slice_slice.c' \
    'rt/runtime/slice_store.c' \
    'rt/runtime/std_context_background.c' \
    'rt/runtime/std_context_canceled.c' \
    'rt/runtime/std_context_deadline_exceeded.c' \
    'rt/runtime/std_context_done.c' \
    'rt/runtime/std_context_err.c' \
    'rt/runtime/std_context_with_cancel.c' \
    'rt/runtime/std_context_with_timeout.c' \
    'rt/runtime/std_errors_is.c' \
    'rt/runtime/std_errors_new.c' \
    'rt/runtime/std_sync_mutex_lock.c' \
    'rt/runtime/std_sync_mutex_unlock.c' \
    'rt/runtime/string_concat.c' \
    'rt/runtime/string_from_bytes.c' \
    'rt/runtime/string_index.c' \
    'rt/runtime/string_slice.c' \
    'rt/runtime/string_to_bytes.c' \
    'rt/runtime/task_spawn.c' \
    'rt/types/bounds.c' \
    'rt/types/float.c' \
    'rt/types/func.c' \
    'rt/types/ints.c' \
    'rt/types/library.c' \
    'rt/types/map.c' \
    'rt/types/panic.c' \
    'rt/types/program.c' \
    'rt/types/sched.c' \
    'rt/types/utf8.c' \
    'rt/types/value.c'
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
