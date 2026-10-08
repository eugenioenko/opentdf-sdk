// swift-tools-version: 6.0
import PackageDescription

let package = Package(
  name: "OpenTDFTDF3",
  products: [.library(name: "OpenTDFTDF3", targets: ["OpenTDFTDF3"])],
  targets: [
    .target(name: "OpenTDFTDF3", dependencies: ["GoalchemyGenerated"], path: "Sources/OpenTDFTDF3"),
    .systemLibrary(
      name: "COpenSSL", path: "system/openssl", pkgConfig: "openssl",
      providers: [.apt(["libssl-dev"]), .brew(["openssl@3"])]),
    .systemLibrary(
      name: "CZlib", path: "system/zlib", pkgConfig: "zlib",
      providers: [.apt(["zlib1g-dev"]), .brew(["zlib"])]),
    .systemLibrary(
      name: "CCurl", path: "system/curl", pkgConfig: "libcurl",
      providers: [.apt(["libcurl4-openssl-dev"]), .brew(["curl"])]),
    .target(
      name: "GoalchemyNative", dependencies: ["COpenSSL", "CZlib", "CCurl"],
      path: "rt/native", sources: ["GoalchemyNative.c"], publicHeadersPath: ".",
      linkerSettings: [.linkedLibrary("crypto"), .linkedLibrary("z"), .linkedLibrary("curl")]),
    .target(
      name: "GoalchemyGenerated", dependencies: ["GoalchemyNative"], path: ".",
      exclude: [
        "Sources", "rt/native", "system", "README.md", "LICENSE", "build.sh",
        "goalchemy.manifest.json",
      ],
      sources: [
        "shared.swift", "pkg_json_40c8871c00fc045f.swift", "pkg_tdf_7dcb6b0a0cf33145.swift",
        "pkg_src_ae63b81fa18b54da.swift", "pkg_library_de7cd4c7ad526690.swift", "Generated.swift",
        "rt/runtime/core_chan_close.swift", "rt/runtime/core_chan_make.swift",
        "rt/runtime/core_chan_recv.swift", "rt/runtime/core_chan_send.swift",
        "rt/runtime/core_integer_add.swift", "rt/runtime/core_integer_and.swift",
        "rt/runtime/core_integer_convert.swift", "rt/runtime/core_integer_div.swift",
        "rt/runtime/core_integer_mul.swift", "rt/runtime/core_integer_neg.swift",
        "rt/runtime/core_integer_or.swift", "rt/runtime/core_integer_rem.swift",
        "rt/runtime/core_integer_shl.swift", "rt/runtime/core_integer_shr.swift",
        "rt/runtime/core_integer_sub.swift", "rt/runtime/core_map_lookup.swift",
        "rt/runtime/core_map_make.swift", "rt/runtime/core_map_store.swift",
        "rt/runtime/core_select.swift", "rt/runtime/core_slice_append.swift",
        "rt/runtime/core_slice_copy.swift", "rt/runtime/core_slice_index.swift",
        "rt/runtime/core_slice_make.swift", "rt/runtime/core_slice_slice.swift",
        "rt/runtime/core_string_concat.swift", "rt/runtime/core_string_from_bytes.swift",
        "rt/runtime/core_string_index.swift", "rt/runtime/core_string_slice.swift",
        "rt/runtime/core_string_to_bytes.swift", "rt/runtime/core_task_spawn.swift",
        "rt/runtime/lib_callback_request.swift", "rt/runtime/lib_checksum_crc32_ieee.swift",
        "rt/runtime/lib_clock_unix.swift", "rt/runtime/lib_crypto_aes256_gcm_decrypt.swift",
        "rt/runtime/lib_crypto_aes256_gcm_encrypt.swift", "rt/runtime/lib_crypto_close.swift",
        "rt/runtime/lib_crypto_ecdh.swift", "rt/runtime/lib_crypto_es256_sign.swift",
        "rt/runtime/lib_crypto_generate_p256.swift", "rt/runtime/lib_crypto_generate_rsa2048.swift",
        "rt/runtime/lib_crypto_hkdf_sha256.swift", "rt/runtime/lib_crypto_hmac_sha256.swift",
        "rt/runtime/lib_crypto_hmac_sha256_verify.swift", "rt/runtime/lib_crypto_import_pem.swift",
        "rt/runtime/lib_crypto_public_jwk.swift", "rt/runtime/lib_crypto_public_pem.swift",
        "rt/runtime/lib_crypto_random.swift", "rt/runtime/lib_crypto_rs256_sign.swift",
        "rt/runtime/lib_crypto_rsa_oaep_decrypt.swift",
        "rt/runtime/lib_crypto_rsa_oaep_encrypt.swift", "rt/runtime/lib_crypto_sha256.swift",
        "rt/runtime/lib_encoding_base64_decode.swift",
        "rt/runtime/lib_encoding_base64_encode.swift",
        "rt/runtime/lib_encoding_base64_url_decode.swift",
        "rt/runtime/lib_encoding_base64_url_encode.swift", "rt/runtime/lib_http_do.swift",
        "rt/runtime/std_context_background.swift", "rt/runtime/std_context_canceled.swift",
        "rt/runtime/std_context_deadline_exceeded.swift", "rt/runtime/std_context_done.swift",
        "rt/runtime/std_context_err.swift", "rt/runtime/std_context_with_cancel.swift",
        "rt/runtime/std_context_with_timeout.swift", "rt/runtime/std_errors_is.swift",
        "rt/runtime/std_errors_new.swift", "rt/runtime/std_sync_mutex_lock.swift",
        "rt/runtime/std_sync_mutex_unlock.swift", "rt/types/Channels.swift",
        "rt/types/Collections.swift", "rt/types/Crypto.swift", "rt/types/HTTP.swift",
        "rt/types/Library.swift", "rt/types/Native.swift", "rt/types/Numeric.swift",
        "rt/types/Program.swift", "rt/types/Value.swift",
      ]),
  ],
  swiftLanguageModes: [.v5]
)
