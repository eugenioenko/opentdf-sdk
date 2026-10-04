# OpenTDF TDF3 — csharp

Generated from shared `src/` with released Goalchemy v0.3.0.

`dotnet build main.csproj -c Release --locked-mode` (.NET SDK 8.0.425). Reference `OpenTDF.TDF3.dll` and deploy `System.IO.Hashing.dll` alongside it.

See [API and examples](../../docs/generated-csharp-library.md) and the [native consumer](../../tests/interop/generatedcsharp) for encrypt/decrypt configuration, cancellation and ownership. This SDK supports the documented TDF3 byte profile, not the complete OpenTDF API. Credentials and local outputs must stay outside the distribution.

Dependency and compiler notices are included under `licenses/` (Python: `opentdf_tdf3/licenses/`). The SDK repository has not specified a license for its own source; dependency notices do not grant a license to that source.

Regeneration/formatting is owned by [the repository distribution command](../../README.md). Pre-format compiler diagnostic line maps are omitted because their line positions no longer describe formatted source. Build artifacts are intentionally excluded.
