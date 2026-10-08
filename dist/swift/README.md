# Swift source distribution

Experimental Linux SwiftPM package exposing `OpenTDFTDF3` with Foundation
`Data` inputs/results. Requires Swift 6.4, OpenSSL 3, libcurl and zlib, with
`pkg-config` and the development headers installed. macOS/iOS integration
remains unverified.

```sh
swift build -c release
```

Add this directory as a local SwiftPM dependency and import `OpenTDFTDF3`.
The `GoalchemyGenerated` target contains the package-based lowered source and
shared runtime; `OpenTDFTDF3` provides the SDK facade. No Goalchemy executable
is needed to build or use this source distribution. See
[the generated Swift API guide](../../docs/generated-swift-library.md) for
configuration, token providers, operations and cancellation.

Compiler/source ownership is recorded in the generated manifest. Diagnostic
line maps describe pre-format output and are omitted. Compilation caches,
native binaries, keys and tokens are excluded from the distribution.

Compiler and dependency notices are included in the source package. The SDK
repository has not specified a license for its own source; dependency notices
do not grant a license to that source.
