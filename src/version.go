package sdk

const (
	// The latest version of TDF Spec currently targeted by the SDK.
	// By default, new files will conform to this version of the spec
	// and, where possible, older versions will still be readable.
	tdfSpecVersion = "4.3.0"

	// The three-part semantic version number of this SDK
	version = "0.1.0"

	// ReferenceVersion is the platform/sdk release this SDK matches.
	ReferenceVersion = "0.34.0"
)

// TDFSpecVersion is the spec version new files target. Goalchemy exports
// functions, not constants, so the generated packages read it through this
// and Version.
func TDFSpecVersion() string { return tdfSpecVersion }

// Version is the three-part semantic version of this SDK.
func Version() string { return version }

// SupportedFeatures returns a list of optional features supported by this SDK build.
// Used by xtest integration harness for feature detection.
//
// These strings are part of the stable API surface. The xtest harness silently
// SKIPs (rather than fails) tests gated on an unknown feature string, so removing
// or renaming a feature here must be coordinated with opentdf/tests before merging
// to avoid quietly disabling coverage. A feature is added only in the change
// whose xtest cells prove it.
func SupportedFeatures() []string {
	return []string{
		"assertions",             // signed manifest assertions, verified on read with the payload key (test_tdf_assertions_unkeyed)
		"assertion_verification", // caller-supplied HS256 and RS256 verification keys (test_tdf_assertions_with_keys)
		"autoconfigure",          // attribute grants and mapped keys drive the key split (Phase 4 test_abac cells)
		"better-messages-2024",   // tamper failures name the check that failed (Phase 2 tamper cells)
		"ecwrap",                 // EC P-256 wrapped KAOs and EC sessions (test_tdf_roundtrip[ztdf-ecwrap])
		"hexless",                // TDF spec 4.3.0: base64 integrity values and schemaVersion
		"kasallowlist",           // reader rejects KAS URLs outside the allowlist (test_tdf_with_malicious_kao)
		"key_management",         // mapped KAS keys with kids (Phase 4 test_abac key mapping cells)
		"mechanism-rsa-4096",     // RSA-4096 wrapped KAOs (test_key_mapping_extended_rsa_mechanisms)
		"multikao",               // duplicate KAOs on one KAS and split (test_tdf_with_duplicate_kao_same_kas)
		"ns_grants",              // namespace-level grants and keys (Phase 4 test_abac cells)
	}
}
