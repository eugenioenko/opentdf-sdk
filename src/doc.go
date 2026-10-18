// Package sdk provides a persistent OpenTDF client for creating and decrypting
// TDF3 archives through an OpenTDF platform and its Key Access Services.
//
// Create a client with New, reuse it for EncryptBytes and DecryptBytes, and call
// SDK.Close when finished. Close readers returned by SDK.LoadTDF before closing
// their SDK. Options, TDFOptions, and ReaderOptions describe supported settings
// and defaults. Error.Kind provides stable error categories across languages.
//
// Byte-array helpers buffer archives in memory; stream operations process
// payloads in segments. Reader metadata and policy access do not by themselves
// establish payload authenticity or authorization.
// Native host adapters expose language-specific handles, cancellation, and byte
// containers; consult the generated library guide for the target you consume.
package sdk
