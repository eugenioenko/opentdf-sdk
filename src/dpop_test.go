package sdk

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"strings"
	"testing"

	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/std/net/url"
)

func decodeProof(t *testing.T, proof string) (map[string]any, map[string]any) {
	t.Helper()
	parts := strings.Split(proof, ".")
	if len(parts) != 3 {
		t.Fatal("compact proof")
	}
	var header, claims map[string]any
	h, _ := base64.RawURLEncoding.DecodeString(parts[0])
	c, _ := base64.RawURLEncoding.DecodeString(parts[1])
	if json.Unmarshal(h, &header) != nil || json.Unmarshal(c, &claims) != nil {
		t.Fatal("proof json")
	}
	return header, claims
}

// The token request carries a DPoP proof without ath and the resource request
// a proof with ath once the IdP issues a DPoP-bound token; the nonce
// challenge is retried once.
func TestDPoPProofs(t *testing.T) {
	f := newFakePlatform(t)
	f.tokenType.Store("DPoP")
	f.nonce.Store("server-nonce")
	s := f.newSDK(t, Options{ClientID: "opentdf", ClientSecret: "secret", DPoPAlgorithm: RS256})
	data, _, err := s.EncryptBytes(context.Background(), []byte("dpop"), kasOptions(f, RSA2048Key))
	if err != nil {
		t.Fatal(err)
	}
	if out, err := s.DecryptBytes(context.Background(), data, ReaderOptions{}); err != nil || string(out) != "dpop" {
		t.Fatal(err)
	}
	f.mu.Lock()
	proof, auth := f.lastDPoP, f.lastAuth
	f.mu.Unlock()
	header, claims := decodeProof(t, proof)
	if header["alg"] != "RS256" || header["typ"] != "dpop+jwt" || header["jwk"].(map[string]any)["kty"] != "RSA" {
		t.Fatal("token proof header", header)
	}
	if claims["htm"] != "POST" || claims["htu"] != f.issuer()+"/protocol/openid-connect/token" || claims["nonce"] != "server-nonce" || claims["ath"] != nil {
		t.Fatal("token proof claims", claims)
	}
	if f.tokenCalls.Load() != 2 {
		t.Fatal("one nonce retry", f.tokenCalls.Load())
	}
	if auth != "DPoP access-token-DPoP" {
		t.Fatal("resource authorization", auth)
	}
	_, _, err = s.EncryptBytes(context.Background(), []byte("again"), kasOptions(f, RSA2048Key))
	if err != nil || f.tokenCalls.Load() != 2 {
		t.Fatal("token cached", err, f.tokenCalls.Load())
	}
}

func TestDPoPKeys(t *testing.T) {
	ec, _ := crypto.GenerateP384()
	pem, _ := ec.PrivatePEM()
	ec.Close()
	signer, err := resolveDPoPKey(&config{dpopKeyPEM: pem})
	if err != nil || signer.algorithm != ES384 {
		t.Fatal("inferred algorithm", err)
	}
	proof, err := signer.proof(&fakeClock{now: 42}, "GET", "https://x/", "tok", "")
	if err != nil {
		t.Fatal(err)
	}
	header, claims := decodeProof(t, proof)
	if header["alg"] != "ES384" || header["jwk"].(map[string]any)["crv"] != "P-384" || claims["iat"] != float64(42) || claims["ath"] == nil || claims["jti"] == "" {
		t.Fatal(header, claims)
	}
	signer.close()
	if _, err := resolveDPoPKey(&config{dpopKeyPEM: pem, dpopAlgorithm: ES256}); err == nil || !strings.Contains(err.Error(), "does not match EC key curve") {
		t.Fatal("curve mismatch", err)
	}
	if _, err := resolveDPoPKey(&config{dpopKeyPEM: pem, dpopAlgorithm: RS256}); err == nil || !strings.Contains(err.Error(), "requires an RSA key") {
		t.Fatal("family mismatch", err)
	}
	if _, err := resolveDPoPKey(&config{dpopAlgorithm: "HS256"}); err == nil || !strings.Contains(err.Error(), "allowed: "+dpopAllowedAlgs) {
		t.Fatal("algorithm allow-list", err)
	}
	if _, err := resolveDPoPKey(&config{dpopKeyPEM: "not a pem"}); err == nil {
		t.Fatal("bad pem")
	}
	public, _ := crypto.GenerateP256()
	publicPEM, _ := public.PublicPEM()
	public.Close()
	if _, err := resolveDPoPKey(&config{dpopKeyPEM: publicPEM}); err == nil || !strings.Contains(err.Error(), "private signing material") {
		t.Fatal("public-only key", err)
	}
	if signer, err := resolveDPoPKey(&config{}); err != nil || signer != nil {
		t.Fatal("none configured")
	}
}

// Ported from TestNormalizeURI (sdk/auth/dpop_transport_test.go).
func TestNormalizeURI(t *testing.T) {
	for _, tt := range []struct{ name, url, want string }{
		{"https default port stripped", "https://example.com:443/path", "https://example.com/path"},
		{"http default port stripped", "http://example.com:80/path", "http://example.com/path"},
		{"https non-default port kept", "https://example.com:8443/path", "https://example.com:8443/path"},
		{"http non-default port kept", "http://example.com:8080/path", "http://example.com:8080/path"},
		{"scheme and host lowercased, path preserved", "HTTPS://EXAMPLE.COM/Path", "https://example.com/Path"},
		{"escaped reserved path preserved", "https://example.com/a%2Fb", "https://example.com/a%2Fb"},
		{"query and fragment dropped", "https://example.com/p?a=b#frag", "https://example.com/p"},
		{"empty path normalized to slash", "https://example.com", "https://example.com/"},
		{"uppercase host with default port", "HTTPS://EXAMPLE.COM:443/Path", "https://example.com/Path"},
		{"ipv6 default port stripped", "https://[::1]:443/path", "https://[::1]/path"},
		{"ipv6 non-default port kept", "https://[::1]:8443/path", "https://[::1]:8443/path"},
		{"ipv6 literal ending in 443 kept", "https://[fe80::443]/path", "https://[fe80::443]/path"},
	} {
		u, err := url.Parse(tt.url)
		if err != nil {
			t.Fatalf("%s: parse %q: %v", tt.name, tt.url, err)
		}
		if got := normalizeURI(u); got != tt.want {
			t.Fatalf("%s: normalizeURI(%q) = %q, want %q", tt.name, tt.url, got, tt.want)
		}
	}
}

// Ported from TestGetOrigin (sdk/auth/dpop_transport_test.go).
func TestGetOrigin(t *testing.T) {
	for _, tt := range []struct{ name, url, want string }{
		{"no port", "https://example.com/path", "https://example.com"},
		{"default https port stripped", "https://example.com:443/path", "https://example.com"},
		{"default http port stripped", "http://example.com:80/path", "http://example.com"},
		{"non-default port kept", "https://example.com:8443/path", "https://example.com:8443"},
		{"scheme and host lowercased", "HTTPS://EXAMPLE.COM:443/p", "https://example.com"},
		{"ipv6 default port stripped", "https://[::1]:443/path", "https://[::1]"},
		{"ipv6 non-default port kept", "https://[::1]:8443/path", "https://[::1]:8443"},
	} {
		u, err := url.Parse(tt.url)
		if err != nil {
			t.Fatalf("%s: parse %q: %v", tt.name, tt.url, err)
		}
		if got := getOrigin(u); got != tt.want {
			t.Fatalf("%s: getOrigin(%q) = %q, want %q", tt.name, tt.url, got, tt.want)
		}
	}
}

func TestProviderTokenValidation(t *testing.T) {
	token, err := parseProviderToken([]byte(`{"value":"opaque","scheme":"DPoP","expiresAt":"9223372036854775807","confirmationJKT":"matching"}`))
	if err != nil || token.ExpiresAt != 9223372036854775807 || token.ConfirmationJKT != "matching" {
		t.Fatal("exact expiry/binding", err)
	}
	for _, wire := range []string{`{"value":"x","scheme":"Bearer","expiresAt":9007199254740993}`, `{"value":"x","scheme":"Bearer","expiresAt":"9223372036854775808"}`, `{"value":"x","scheme":"Bearer","expiresAt":"1","extra":"x"}`, `[]`} {
		if _, err := parseProviderToken([]byte(wire)); err == nil {
			t.Fatal("invalid provider wire accepted", wire)
		}
	}
	if validToken(AccessToken{Value: "x", Scheme: "Bearer", ExpiresAt: 10}, 10) || !validToken(AccessToken{Value: "x", Scheme: "Bearer"}, 10) || validToken(AccessToken{Value: "bad token", Scheme: "Bearer"}, 0) {
		t.Fatal("validToken")
	}
	if validateBinding(AccessToken{Value: "opaque", ConfirmationJKT: "a"}, "a") != nil || validateBinding(AccessToken{Value: "opaque", ConfirmationJKT: "b"}, "a") == nil {
		t.Fatal("opaque binding")
	}
	jwt := base64.RawURLEncoding.EncodeToString([]byte(`{"alg":"RS256"}`)) + "." + base64.RawURLEncoding.EncodeToString([]byte(`{"cnf":{"jkt":"thumb"}}`)) + ".sig"
	if validateBinding(AccessToken{Value: jwt}, "thumb") != nil || validateBinding(AccessToken{Value: jwt}, "other") == nil {
		t.Fatal("jwt binding")
	}
}
