package sdk

import (
	"errors"
	"fmt"
	"net"
	neturl "net/url"
	"strings"
	"testing"
)

// Ported from TestWithKasInformation, TestWithMetaData, TestWithMimeType.
func TestTDFOptionsFields(t *testing.T) {
	cfg, err := newTDFConfig(TDFOptions{KasInformation: []KASInfo{{URL: "https://example.com/1"}, {URL: "https://example.com/2"}}, Metadata: "foo", MimeType: "bar", Attributes: []string{"https://example.com/attr/Classification/value/S"}})
	if err != nil {
		t.Fatal(err)
	}
	if len(cfg.kasInfoList) != 2 || cfg.kasInfoList[0].URL != "https://example.com/1" || cfg.kasInfoList[1].URL != "https://example.com/2" {
		t.Fatal("kas information", cfg.kasInfoList)
	}
	if cfg.metaData != "foo" || cfg.mimeType != "bar" || len(cfg.attributes) != 1 || !cfg.autoconfigure {
		t.Fatal("fields", cfg)
	}
	if _, err := newTDFConfig(TDFOptions{Attributes: []string{""}}); err == nil {
		t.Fatal("empty attribute")
	}
}

// Ported from TestWithSegmentSize.
func TestWithSegmentSize(t *testing.T) {
	for _, tc := range []struct {
		name     string
		size     int64
		expected int64
	}{
		{"defaultSize", 0, defaultSegmentSize},
		{"inRangeSize", 1024 * 1024, 1024 * 1024},
		{"tooSmallSize", 1, minSegmentSize},
		{"tooLargeSize", maxSegmentSize + 1, maxSegmentSize},
	} {
		cfg, err := newTDFConfig(TDFOptions{SegmentSize: tc.size})
		if err != nil || cfg.defaultSegmentSize != tc.expected {
			t.Fatalf("%s: %d %v", tc.name, cfg.defaultSegmentSize, err)
		}
	}
}

// Ported from TestWithTargetMode, narrowed to what this build accepts.
func TestUnsupportedTDFOptions(t *testing.T) {
	for name, opts := range map[string]TDFOptions{
		"TargetMode 4.2.2": {TargetMode: "4.2.2"},
		"Assertions":       {Assertions: []AssertionConfig{{ID: "1"}}},
		"SystemMetadata":   {SystemMetadataAssertion: true},
		"RSA4096":          {WrappingKeyAlgorithm: RSA4096Key},
	} {
		_, err := newTDFConfig(opts)
		var e *Error
		if !errors.As(err, &e) || e.Kind != KindUnsupportedFeature {
			t.Fatalf("%s: %v", name, err)
		}
	}
	if cfg, err := newTDFConfig(TDFOptions{TargetMode: "4.3.0"}); err != nil || cfg.useHex {
		t.Fatal("4.3.0 is the default mode", err)
	}
	if _, err := newTDFConfig(TDFOptions{WrappingKeyAlgorithm: "bogus"}); err == nil {
		t.Fatal("bogus key type")
	}
	if _, err := newTDFConfig(TDFOptions{InputSize: -1}); err == nil {
		t.Fatal("negative input size")
	}
}

// Ported from TestAllowList_Add.
func TestAllowList_Add(t *testing.T) {
	for _, tc := range []struct {
		name        string
		kasURL      string
		entry       string
		expectError bool
	}{
		{"Valid URL with port", "https://example.com:443", "https://example.com:443", false},
		{"Valid URL without port", "https://example.com/something", "https://example.com:443", false},
		{"Valid http URL with port", "http://example.com:8080/hello", "http://example.com:8080", false},
		{"Valid http URL without port", "http://example.com", "http://example.com", false},
		{"Url without scheme", "example.com:8080", "https://example.com:8080", false},
		{"Url without scheme and port", "example.com", "https://example.com:443", false},
		{"Invalid URL", "http://:8080", "", true},
		{"Empty URL", "", "", true},
	} {
		allowList := make(AllowList)
		err := allowList.Add(tc.kasURL)
		if tc.expectError {
			if err == nil {
				t.Fatalf("%s: expected an error", tc.name)
			}
			continue
		}
		if err != nil || !allowList[tc.entry] {
			t.Fatalf("%s: %v %v", tc.name, err, allowList)
		}
	}
}

// Ported from TestAllowList_IsAllowed.
func TestAllowList_IsAllowed(t *testing.T) {
	allowList := make(AllowList)
	for _, u := range []string{"https://example.com:443", "https://another.com", "http://one-more.com", "without-schema:8080"} {
		if err := allowList.Add(u); err != nil {
			t.Fatal(err)
		}
	}
	for _, tc := range []struct {
		kasURL   string
		expected bool
	}{
		{"https://example.com:443", true}, {"https://another.com", true}, {"https://without-schema:8080", true}, {"http://one-more.com", true},
		{"http://another.com", false}, {"https://another.com:9090", false}, {"https://notallowed.com", false}, {"invalid-url", false},
	} {
		if got := allowList.IsAllowed(tc.kasURL); got != tc.expected {
			t.Fatalf("IsAllowed(%q) = %v", tc.kasURL, got)
		}
	}
	var none AllowList
	if none.IsAllowed("https://example.com") {
		t.Fatal("nil allowlist allows nothing")
	}
}

// getKasAddress matches the reference function run on Go's net/url, results
// and error text included.
func TestGetKasAddressMatchesNetURL(t *testing.T) {
	reference := func(kasURL string) (string, error) {
		if !strings.Contains(kasURL, "://") {
			kasURL = "https://" + kasURL
		}
		parsedURL, err := neturl.Parse(kasURL)
		if err != nil {
			return "", fmt.Errorf("cannot parse kas url(%s): %w", kasURL, err)
		}
		if parsedURL.Hostname() == "" {
			return "", fmt.Errorf("no host parsed from url: %s", kasURL)
		}
		if parsedURL.Port() != "" {
			parsedURL.Host = net.JoinHostPort(parsedURL.Hostname(), parsedURL.Port())
		} else if parsedURL.Scheme == "https" {
			parsedURL.Host = net.JoinHostPort(parsedURL.Hostname(), "443")
		}
		parsedURL.Path = ""
		parsedURL.RawQuery = ""
		parsedURL.Fragment = ""
		return parsedURL.String(), nil
	}
	for _, input := range []string{
		"https://example.com:443", "https://example.com/something", "example.com:8080", "example.com", "http://:8080", "",
		"HTTPS://Example.COM/a%2Fb?x=1#f", "https://[::1]/kas", "http://[::1]:8080/kas", "https://example.com:80:",
		"https://exa mple.com", "kas.example.com/%zz", "https://example.com:port", "http://user:pw@host:1/p", "[::1]:9000",
	} {
		got, err := getKasAddress(input)
		want, wantErr := reference(input)
		if got != want || (err == nil) != (wantErr == nil) || err != nil && err.Error() != wantErr.Error() {
			t.Fatalf("getKasAddress(%q) = %q, %v; reference %q, %v", input, got, err, want, wantErr)
		}
	}
}

// Ported from TestWithKasAllowlist and TestWithIgnoreAllowlist.
func TestReaderOptionsAllowlist(t *testing.T) {
	for _, kasList := range [][]string{{"https://example.com:443", "https://another.com"}, {}} {
		cfg, err := newTDFReaderConfig(nil, ReaderOptions{KasAllowlist: kasList})
		if err != nil {
			t.Fatal(err)
		}
		defer cfg.closeSessionKey()
		for _, kasURL := range kasList {
			if !cfg.kasAllowlist.IsAllowed(kasURL) {
				t.Fatal("expected KAS URL to be allowed", kasURL)
			}
		}
	}
	for _, ignore := range []bool{true, false} {
		cfg, err := newTDFReaderConfig(nil, ReaderOptions{IgnoreAllowlist: ignore})
		if err != nil || cfg.ignoreAllowList != ignore {
			t.Fatal("ignore allowlist", err)
		}
		cfg.closeSessionKey()
	}
	if _, err := newTDFReaderConfig(nil, ReaderOptions{KasAllowlist: []string{"http://:8080"}}); err == nil {
		t.Fatal("invalid allowlist entry")
	}
	if _, err := newTDFReaderConfig(nil, ReaderOptions{SchemaValidation: Lax}); !errors.Is(err, &Error{Kind: KindUnsupportedFeature, Message: "unsupported in this build: SchemaValidation"}) {
		t.Fatal("schema validation unsupported", err)
	}
	cfg, err := newTDFReaderConfig(nil, ReaderOptions{SessionKeyType: EC256Key, MaxManifestSize: 1024})
	if err != nil || cfg.kasSessionKeyType != EC256Key || !cfg.ownsSessionKey || cfg.maxManifestSize != 1024 {
		t.Fatal("EC session key", err)
	}
	cfg.closeSessionKey()
}
