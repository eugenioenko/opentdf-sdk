package sdk

import (
	"context"
	"errors"
	neturl "net/url"
	"strings"
	"testing"

	"github.com/eugenioenko/goalchemy/std/bytes"
)

func providedConfiguration() PlatformConfiguration {
	var c PlatformConfiguration
	c.Set("idp", `{"issuer":"https://example.org","authorization_endpoint":"https://example.org/auth","token_endpoint":"https://example.org/token"}`)
	return c
}

// Ported from TestNew_ShouldCreateSDK: a provided configuration is used as is.
func TestNew_ShouldCreateSDK(t *testing.T) {
	s, err := New(context.Background(), "http://localhost:8080", Options{PlatformConfiguration: providedConfiguration(), ClientID: "myid", ClientSecret: "mysecret", TokenEndpoint: "https://example.org/token"})
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()
	iss, err := s.PlatformConfiguration().Issuer()
	if err != nil || iss != "https://example.org" {
		t.Fatal("issuer", iss, err)
	}
	authz, err := s.PlatformConfiguration().AuthzEndpoint()
	if err != nil || authz != "https://example.org/auth" {
		t.Fatal("authz", authz, err)
	}
	token, err := s.PlatformConfiguration().TokenEndpoint()
	if err != nil || token != "https://example.org/token" {
		t.Fatal("token", token, err)
	}
	if endpoint, _ := s.PlatformConfiguration().platformEndpoint(); endpoint != "http://localhost:8080" {
		t.Fatal("platform_endpoint", endpoint)
	}
	if s.tokenSource == nil || s.dpop == nil || s.dpop.algorithm != ES256 {
		t.Fatal("credentials wire a token source with a default ES256 DPoP key")
	}
}

// Ported from Test_PlatformConfiguration_BadCases.
func Test_PlatformConfiguration_BadCases(t *testing.T) {
	assertions := func(t *testing.T, s *SDK) {
		if _, err := s.PlatformConfiguration().Issuer(); !errors.Is(err, ErrPlatformIssuerNotFound) {
			t.Fatal("issuer", err)
		}
		if _, err := s.PlatformConfiguration().AuthzEndpoint(); !errors.Is(err, ErrPlatformAuthzEndpointNotFound) {
			t.Fatal("authz", err)
		}
		if _, err := s.PlatformConfiguration().TokenEndpoint(); !errors.Is(err, ErrPlatformTokenEndpointNotFound) {
			t.Fatal("token", err)
		}
	}
	var noIdpVals PlatformConfiguration
	noIdpVals.Set("idp", "{}")
	s, err := New(context.Background(), "http://localhost:8080", Options{PlatformConfiguration: noIdpVals})
	if err != nil {
		t.Fatal(err)
	}
	assertions(t, s)
	s.Close()
	var empty PlatformConfiguration
	empty.Set("platform_issuer", "https://example.org")
	s, err = New(context.Background(), "http://localhost:8080", Options{PlatformConfiguration: empty})
	if err != nil {
		t.Fatal(err)
	}
	assertions(t, s)
	s.Close()
}

// Ported from Test_ShouldCreateNewSDK_NoCredentials: an uncredentialed SDK
// has no token source and no DPoP key.
func Test_ShouldCreateNewSDK_NoCredentials(t *testing.T) {
	var c PlatformConfiguration
	c.Set("platform_issuer", "https://example.org")
	s, err := New(context.Background(), "http://localhost:8080", Options{PlatformConfiguration: c})
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()
	if s.tokenSource != nil || s.dpop != nil {
		t.Fatal("uncredentialed")
	}
	if _, err := New(context.Background(), "http://localhost:8080", Options{PlatformConfiguration: c, DPoPAlgorithm: ES256}); err == nil || !strings.Contains(err.Error(), "DPoP configured") {
		t.Fatal("DPoP without credentials must fail loudly", err)
	}
}

// Ported from TestIsPlatformEndpointMalformed.
func TestIsPlatformEndpointMalformed(t *testing.T) {
	for _, tc := range []struct {
		input    string
		expected bool
	}{
		{"https://example.com", false}, {"https://example.com:8080", false}, {"https://example.com/path", false},
		{"https://:8080", true}, {"example.com", true}, {"https://exa mple.com", true}, {"https://example:com", true},
		{"", true}, {"localhost:8080", true}, {"http://localhost:8080", false},
	} {
		if got := IsPlatformEndpointMalformed(tc.input); got != tc.expected {
			t.Fatalf("IsPlatformEndpointMalformed(%q) = %v", tc.input, got)
		}
	}
}

// IsPlatformEndpointMalformed matches the reference function run on Go's
// net/url.
func TestIsPlatformEndpointMalformedMatchesNetURL(t *testing.T) {
	inputs := []string{
		"https://example.com", "http://localhost:8080", "https://example.com/", "https://example.com/base/", "https://example.com:80:",
		"https://[::1]:8443", "https://[::1", "localhost:8080", "/relative", "https://example.com/a%2Fb", "https://exa mple.com",
		"https://example.com/realms/../x", "https://example.com?q=1#f", "https://example.com/%zz", "",
	}
	for _, input := range inputs {
		u, err := neturl.ParseRequestURI(input)
		want := err != nil || u.Hostname() == "" || strings.Contains(u.Hostname(), ":")
		if got := IsPlatformEndpointMalformed(input); got != want {
			t.Fatalf("IsPlatformEndpointMalformed(%q) = %v, reference %v", input, got, want)
		}
	}
}

// Ported from Test_New_ShouldFailWithDisconnectedPlatform.
func Test_New_ShouldFailWithDisconnectedPlatform(t *testing.T) {
	s, err := New(context.Background(), "http://localhost:9", Options{ValidateConnection: true})
	if !errors.Is(err, ErrPlatformUnreachable) || s != nil {
		t.Fatal("expected ErrPlatformUnreachable", err)
	}
	var c PlatformConfiguration
	c.Set("platform_issuer", "https://example.org")
	s, err = New(context.Background(), "http://localhost:9", Options{PlatformConfiguration: c, ValidateConnection: true})
	if !errors.Is(err, ErrPlatformUnreachable) || s != nil {
		t.Fatal("validates even with platform configuration provided", err)
	}
	var e *Error
	if !errors.As(err, &e) || e.Kind != KindTransport {
		t.Fatal("kind", err)
	}
	if _, err := New(context.Background(), "not a url", Options{PlatformConfiguration: c}); !errors.Is(err, ErrPlatformEndpointMalformed) {
		t.Fatal("malformed endpoint", err)
	}
}

// Ported from TestNew_ShouldValidateStandardTdf, TestIsValidTdf_AcceptsSpecManifestName,
// TestNew_ShouldNotValidateBadStandardTdf and TestIsInvalid_MissingRequiredManifestPayloadField.
func TestIsValidTdf(t *testing.T) {
	for _, fixture := range []string{goodStandardTdf, specNamedManifestTdf} {
		in := bytes.NewReader(mustDecode(t, fixture))
		for i := 0; i < 2; i++ {
			valid, err := IsValidTdf(in)
			if err != nil || !valid {
				t.Fatal("valid tdf, reader reset", i, err)
			}
		}
		if GetTdfType(in) != Standard {
			t.Fatal("standard type")
		}
	}
	valid, err := IsValidTdf(bytes.NewReader(mustDecode(t, badStandardTdf)))
	if valid || err == nil {
		t.Fatal("bad central directory must fail", err)
	}
	valid, err = IsValidTdf(bytes.NewReader(mustDecode(t, missingPayloadTypeTdf)))
	if valid || !errors.Is(err, ErrInvalidPerSchema) {
		t.Fatal("missing payload.type is ErrInvalidPerSchema", err)
	}
	if GetTdfType(bytes.NewReader([]byte("not a zip file"))) != Invalid || GetTdfType(bytes.NewReader([]byte("PK"))) != Invalid {
		t.Fatal("invalid type")
	}
}

func TestDiscovery(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	if iss, err := s.PlatformConfiguration().Issuer(); err != nil || iss != f.issuer() {
		t.Fatal("issuer from the well-known document", iss, err)
	}
	if s.cfg.tokenEndpoint != f.issuer()+"/protocol/openid-connect/token" {
		t.Fatal("token endpoint from OIDC discovery", s.cfg.tokenEndpoint)
	}
	if e, _ := s.PlatformConfiguration().platformEndpoint(); e != f.server.URL {
		t.Fatal("platform_endpoint", e)
	}
	healthy, err := s.IsHealthy(context.Background())
	if err != nil || !healthy {
		t.Fatal("healthy", healthy, err)
	}
	f.mode.Store("unhealthy")
	if healthy, err := s.IsHealthy(context.Background()); err != nil || healthy {
		t.Fatal("not serving", healthy, err)
	}
	f.mode.Store("wellknown-error")
	if _, err := New(context.Background(), f.server.URL, f.options()); !errors.Is(err, ErrPlatformConfigFailed) {
		t.Fatal("well-known failure", err)
	}
	f.mode.Store("")
	// WithTokenEndpoint skips OIDC discovery; a provided configuration skips
	// the well-known fetch and the token endpoint stays empty, as in the reference.
	s2 := f.newSDK(t, Options{ClientID: "opentdf", ClientSecret: "secret", TokenEndpoint: "http://example.org/token"})
	if s2.cfg.tokenEndpoint != "http://example.org/token" {
		t.Fatal("explicit token endpoint", s2.cfg.tokenEndpoint)
	}
	s3 := f.newSDK(t, Options{ClientID: "opentdf", ClientSecret: "secret", PlatformConfiguration: providedConfiguration()})
	if s3.cfg.tokenEndpoint != "" {
		t.Fatal("provided configuration does not trigger discovery", s3.cfg.tokenEndpoint)
	}
	if err := s.Close(); err != nil || s.Close() != nil {
		t.Fatal("close is idempotent")
	}
	if _, err := s.DecryptBytes(context.Background(), nil, ReaderOptions{}); err == nil {
		t.Fatal("closed sdk rejects calls")
	}
}

func TestUnsupportedOptions(t *testing.T) {
	var c PlatformConfiguration
	c.Set("platform_issuer", "https://example.org")
	for name, opts := range map[string]Options{
		"TokenExchange": {TokenExchange: TokenExchange{SubjectToken: "x"}},
		"CertExchange":  {CertExchange: CertExchange{ClientCertPEM: "x"}},
		"TLS":           {TLS: TLSOptions{RootCAsPEM: "x"}},
		"UnixSocket":    {UnixSocket: "/tmp/x"},
	} {
		opts.PlatformConfiguration = c
		_, err := New(context.Background(), "http://localhost:8080", opts)
		var e *Error
		if !errors.As(err, &e) || e.Kind != KindUnsupportedFeature || !strings.HasPrefix(e.Message, "unsupported in this build: ") {
			t.Fatalf("%s: %v", name, err)
		}
	}
}
