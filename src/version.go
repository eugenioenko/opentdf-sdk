package sdk

const (
	// The latest version of TDF Spec currently targeted by the SDK.
	// By default, new files will conform to this version of the spec
	// and, where possible, older versions will still be readable.
	TDFSpecVersion = "4.3.0"

	// The three-part semantic version number of this SDK
	Version = "0.1.0"

	// ReferenceVersion is the platform/sdk release this SDK matches.
	ReferenceVersion = "0.34.0"
)

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
		"autoconfigure",        // attribute grants and mapped keys drive the key split (Phase 4 test_abac cells)
		"better-messages-2024", // tamper failures name the check that failed (Phase 2 tamper cells)
		"ecwrap",               // EC P-256 wrapped KAOs and EC sessions (test_tdf_roundtrip[ztdf-ecwrap])
		"hexless",              // TDF spec 4.3.0: base64 integrity values and schemaVersion
		"kasallowlist",         // reader rejects KAS URLs outside the allowlist (test_tdf_with_malicious_kao)
		"key_management",       // mapped KAS keys with kids (Phase 4 test_abac key mapping cells)
		"ns_grants",            // namespace-level grants and keys (Phase 4 test_abac cells)
	}
}
