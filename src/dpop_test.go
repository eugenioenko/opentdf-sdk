package sdk

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"strings"
	"testing"

	"github.com/eugenioenko/goalchemy/lib/crypto"
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
	for _, c := range []struct{ in, htu, origin string }{
		{"HTTPS://Example.COM:443/kas/", "https://example.com/kas/", "https://example.com"},
		{"http://localhost:8080/kas.AccessService/Rewrap?x=1#f", "http://localhost:8080/kas.AccessService/Rewrap", "http://localhost:8080"},
		{"http://localhost", "http://localhost/", "http://localhost"},
	} {
		htu, origin, err := normalizeURI(c.in)
		if err != nil || htu != c.htu || origin != c.origin {
			t.Fatalf("normalizeURI(%q) = %q %q %v", c.in, htu, origin, err)
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
