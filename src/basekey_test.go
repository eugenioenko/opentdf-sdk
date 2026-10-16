package sdk

import (
	"context"
	"errors"
	"strings"
	"testing"
)

// Ported from TestGetKasKeyAlg, TestFormatAlg and TestFormatAlg_GetKasKeyAlg_RoundTrip
// (the post-quantum key types are not planned).
func TestKeyTypeAlgorithmRoundTrip(t *testing.T) {
	for _, tc := range []struct {
		algStr   string
		expected string
	}{
		{"rsa:2048", AlgorithmRSA2048}, {"rsa:4096", AlgorithmRSA4096}, {"ec:secp256r1", AlgorithmECP256}, {"ec:secp384r1", AlgorithmECP384}, {"ec:secp521r1", AlgorithmECP521},
	} {
		result, err := getKasKeyAlg(tc.algStr)
		if err != nil || result.String() != tc.expected {
			t.Fatalf("getKasKeyAlg(%q) = %q %v", tc.algStr, result, err)
		}
		formatted, err := formatAlg(result)
		if err != nil || formatted != tc.algStr {
			t.Fatalf("formatAlg(%q) = %q %v", result, formatted, err)
		}
	}
	for _, bad := range []string{"unsupported", ""} {
		if result, err := getKasKeyAlg(bad); err == nil || result.String() != AlgorithmUnspecified {
			t.Fatalf("getKasKeyAlg(%q)", bad)
		}
	}
	for _, bad := range []string{AlgorithmUnspecified, "ALGORITHM_99"} {
		if _, err := formatAlg(policyAlgorithmOf(bad)); err == nil {
			t.Fatalf("formatAlg(%q)", bad)
		}
	}
}

// Ported from BaseKeyTestSuite.
func TestGetBaseKey(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	f.baseKey.Store("rsa")
	key, err := s.GetBaseKey(context.Background())
	if err != nil || key.KasURI != f.kasURL() || key.PublicKey.Kid != "r1" || key.PublicKey.Algorithm != AlgorithmRSA2048 || !strings.HasPrefix(key.PublicKey.Pem, "-----BEGIN PUBLIC KEY-----") {
		t.Fatal("base key", key, err)
	}
	for mode, want := range map[string]error{"invalid": ErrBaseKeyInvalidFormat, "empty": ErrBaseKeyEmpty, "no-public-key": ErrBaseKeyInvalidFormat, "string-public-key": ErrBaseKeyInvalidFormat, "": ErrBaseKeyNotFound} {
		f.baseKey.Store(mode)
		if _, err := s.GetBaseKey(context.Background()); !errors.Is(err, want) {
			t.Fatalf("%s: %v", mode, err)
		}
	}
	f.mode.Store("wellknown-error")
	_, err = s.GetBaseKey(context.Background())
	if err == nil || !strings.Contains(err.Error(), "unable to retrieve config information") || !strings.Contains(err.Error(), "service unavailable") {
		t.Fatal("service error", err)
	}
	f.mode.Store("wellknown-empty")
	if _, err := s.GetBaseKey(context.Background()); !errors.Is(err, ErrWellKnowConfigEmpty) {
		t.Fatal("empty configuration", err)
	}
}
