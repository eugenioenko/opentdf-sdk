# OpenTDF TDF3 — c

Generated from shared `src/` with released Goalchemy v0.4.0.

`bash build-sdk.sh` (C17, OpenSSL 3, libcurl and BDWGC 8.2.8 development files). Include `tdf3.h` and link `libtdf3.a`, libcurl, libssl, libcrypto, libgc, pthread and dl.

See [API and examples](../../docs/generated-c-library.md) and the [native consumer](../../tests/interop/generatedc) for encrypt/decrypt configuration, cancellation and ownership. This SDK supports the documented TDF3 byte profile, not the complete OpenTDF API. Credentials and local outputs must stay outside the distribution.

Dependency and compiler notices are included under `licenses/` (Python: `opentdf_tdf3/licenses/`). The SDK repository has not specified a license for its own source; dependency notices do not grant a license to that source.

Regeneration/formatting is owned by [the repository distribution command](../../README.md). Pre-format compiler diagnostic line maps are omitted because their line positions no longer describe formatted source. Build artifacts are intentionally excluded.
