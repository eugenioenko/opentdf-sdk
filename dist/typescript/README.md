# OpenTDF TDF3 — typescript

Generated from shared `src/` with released Goalchemy v0.3.0.

`npm ci && npm run build` (Node 24.15.0). Import `@opentdf-local/tdf3`; conditional exports select the Node CRC adapter, and the browser/default export uses portable Web Crypto and fetch.

See [API and examples](../../docs/generated-typescript-library.md) and the [native consumer](../../tests/interop/generatedtypescript) for encrypt/decrypt configuration, cancellation and ownership. This SDK supports the documented TDF3 byte profile, not the complete OpenTDF API. Credentials and local outputs must stay outside the distribution.

Dependency and compiler notices are included under `licenses/` (Python: `opentdf_tdf3/licenses/`). The SDK repository has not specified a license for its own source; dependency notices do not grant a license to that source.

Regeneration/formatting is owned by [the repository distribution command](../../README.md). Pre-format compiler diagnostic line maps are omitted because their line positions no longer describe formatted source. Build artifacts are intentionally excluded.
