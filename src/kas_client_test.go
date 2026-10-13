package sdk

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"net"
	neturl "net/url"
	"strings"
	"testing"

	"github.com/eugenioenko/goalchemy/lib/crypto"
)

// Ported from TestCreatingRequest: the signed request token carries the
// protojson rewrap request with the KAO's fields.
func TestCreatingRequest(t *testing.T) {
	key, err := crypto.GenerateRSA2048()
	if err != nil {
		t.Fatal(err)
	}
	defer key.Close()
	signer, err := newDPoPSigner(key, RS256)
	if err != nil {
		t.Fatal(err)
	}
	client := newKASClient(nil, signer, &fakeClock{now: 1700000000}, []string{"https://example.com/obl/a/value/b"})
	request := []*rewrapPolicyRequest{{
		Policy: rewrapPolicy{ID: "policy", Body: "cG9saWN5"},
		KeyAccessObjects: []*rewrapKeyAccessObject{{KeyAccessObjectID: "kao-0", KeyAccessObject: rewrapKeyAccess{
			KeyType: "type1", KasURL: "https://kas.example.org", Protocol: "protocol one", WrappedKey: []byte("wrapped"),
			PolicyBinding: PolicyBinding{Alg: "HS256", Hash: "somehash"}, EncryptedMetadata: "encrypted",
		}}},
	}}
	body, err := client.getRewrapRequest(request, "-----BEGIN PUBLIC KEY-----\nx\n-----END PUBLIC KEY-----")
	if err != nil {
		t.Fatal(err)
	}
	var req struct {
		SignedRequestToken string `json:"signedRequestToken"`
	}
	if err := json.Unmarshal(body, &req); err != nil || req.SignedRequestToken == "" {
		t.Fatal("didn't produce a signed request token", err)
	}
	parts := strings.Split(req.SignedRequestToken, ".")
	if len(parts) != 3 {
		t.Fatal("compact JWS")
	}
	header, _ := base64.RawURLEncoding.DecodeString(parts[0])
	if string(header) != `{"alg":"RS256","typ":"JWT"}` {
		t.Fatal("header", string(header))
	}
	claims, _ := base64.RawURLEncoding.DecodeString(parts[1])
	var c struct {
		RequestBody string `json:"requestBody"`
		Iat         int64  `json:"iat"`
		Exp         int64  `json:"exp"`
	}
	if err := json.Unmarshal(claims, &c); err != nil || c.Iat != 1700000000 || c.Exp != 1700000060 {
		t.Fatal("claims", string(claims), err)
	}
	var unsigned struct {
		ClientPublicKey string `json:"clientPublicKey"`
		Requests        []struct {
			KeyAccessObjects []struct {
				KeyAccessObjectID string         `json:"keyAccessObjectId"`
				KeyAccessObject   map[string]any `json:"keyAccessObject"`
			} `json:"keyAccessObjects"`
			Policy map[string]string `json:"policy"`
		} `json:"requests"`
		KeyAccess map[string]any `json:"keyAccess"`
		Policy    string         `json:"policy"`
	}
	if err := json.Unmarshal([]byte(c.RequestBody), &unsigned); err != nil {
		t.Fatal(err)
	}
	if len(unsigned.Requests) != 1 || len(unsigned.Requests[0].KeyAccessObjects) != 1 || unsigned.Requests[0].Policy["id"] != "policy" || unsigned.Requests[0].Policy["body"] != "cG9saWN5" {
		t.Fatal("request shape", c.RequestBody)
	}
	kao := unsigned.Requests[0].KeyAccessObjects[0].KeyAccessObject
	binding := kao["policyBinding"].(map[string]any)
	if kao["url"] != "https://kas.example.org" || kao["protocol"] != "protocol one" || kao["wrappedKey"] != base64.StdEncoding.EncodeToString([]byte("wrapped")) || binding["alg"] != "HS256" || binding["hash"] != "somehash" || kao["encryptedMetadata"] != "encrypted" || kao["type"] != "type1" {
		t.Fatal("kao", kao)
	}
	// The legacy single-KAO fields are mirrored, as the reference does.
	if unsigned.KeyAccess["url"] != "https://kas.example.org" || unsigned.Policy != "cG9saWN5" {
		t.Fatal("legacy fields", c.RequestBody)
	}
	headers, err := client.rewrapContextHeaders()
	if err != nil || headers[0] != additionalRewrapContextHeader {
		t.Fatal(err)
	}
	ctxJSON, _ := base64.StdEncoding.DecodeString(headers[1])
	if string(ctxJSON) != `{"obligations":{"fulfillableFQNs":["https://example.com/obl/a/value/b"]}}` {
		t.Fatal("rewrap context", string(ctxJSON))
	}
	if _, err := client.getRewrapRequest(nil, ""); err == nil {
		t.Fatal("no requests")
	}
}

// Ported from TestParseBaseUrl.
func TestParseBaseUrl(t *testing.T) {
	for _, tc := range []struct {
		input       string
		expected    string
		expectError bool
	}{
		{"https://example.com:8080/path", "https://example.com:8080", false},
		{"https://example.com/path", "https://example.com", false},
		{"http://example.com", "http://example.com", false},
		{"https://exa mple.com", "", true},
	} {
		result, err := parseBaseURL(tc.input)
		if tc.expectError != (err != nil) || result != tc.expected {
			t.Fatalf("parseBaseURL(%q) = %q, %v", tc.input, result, err)
		}
	}
}

// parseBaseURL matches the reference function run on Go's net/url, results
// and error text included.
func TestParseBaseURLMatchesNetURL(t *testing.T) {
	reference := func(rawURL string) (string, error) {
		u, err := neturl.Parse(rawURL)
		if err != nil {
			return "", err
		}
		addr := u.Hostname()
		if u.Port() != "" {
			addr = net.JoinHostPort(u.Hostname(), u.Port())
		}
		return fmt.Sprintf("%s://%s", u.Scheme, addr), nil
	}
	for _, input := range []string{
		"https://example.com:8080/path", "HTTPS://Example.COM/kas", "http://[::1]:8080/kas", "https://[fe80::1%25en0]:443/",
		"https://example.com:80:", "https://exa mple.com", "http://[::1", "https://example.com:port/", "://missing",
		"http://exam%zzple.com", "http://localhost\x7f/kas", "kas.example.com:8080", "", "http://user@host/p",
	} {
		got, err := parseBaseURL(input)
		want, wantErr := reference(input)
		if got != want || (err == nil) != (wantErr == nil) || err != nil && err.Error() != wantErr.Error() {
			t.Fatalf("parseBaseURL(%q) = %q, %v; reference %q, %v", input, got, err, want, wantErr)
		}
	}
}

// Ported from TestKasKeyCache_NoKID.
func TestKasKeyCache_NoKID(t *testing.T) {
	cache := newKasKeyCache(&fakeClock{now: 1000})
	const testURL, testAlgorithm, testPubKey = "https://kas.example.org", "ec:secp256r1", "test-public-key"
	cache.store(KASInfo{URL: testURL, Algorithm: testAlgorithm, KID: "specific-kid", PublicKey: testPubKey})
	if k := cache.get(testURL, testAlgorithm, "specific-kid"); k == nil || k.PublicKey != testPubKey || k.KID != "specific-kid" {
		t.Fatal("specific kid")
	}
	if k := cache.get(testURL, testAlgorithm, ""); k == nil || k.PublicKey != testPubKey || k.KID != "specific-kid" {
		t.Fatal("empty kid finds through iteration")
	}
	if cache.get(testURL, testAlgorithm, "different-kid") != nil {
		t.Fatal("different kid must not iterate")
	}
	cache.store(KASInfo{URL: testURL, Algorithm: testAlgorithm, KID: "different-kid", PublicKey: "another-public-key"})
	if k := cache.get(testURL, testAlgorithm, "different-kid"); k == nil || k.PublicKey != "another-public-key" {
		t.Fatal("different kid")
	}
	if cache.get(testURL, testAlgorithm, "specific-kid") == nil || cache.get(testURL, testAlgorithm, "") == nil {
		t.Fatal("both keys retrievable")
	}
	cache.store(KASInfo{URL: testURL, Algorithm: testAlgorithm, KID: "", PublicKey: "empty-kid-public-key"})
	if k := cache.get(testURL, testAlgorithm, ""); k == nil || k.PublicKey != "empty-kid-public-key" || k.KID != "" {
		t.Fatal("empty kid key")
	}
	if cache.get("https://other", testAlgorithm, "") != nil || cache.get(testURL, "rsa:2048", "") != nil {
		t.Fatal("misses")
	}
}

// Ported from TestKasKeyCache_Expiration, with the clock injected.
func TestKasKeyCache_Expiration(t *testing.T) {
	clock := &fakeClock{now: 1000}
	cache := newKasKeyCache(clock)
	keyInfo := KASInfo{URL: "https://kas.example.org", Algorithm: "ec:secp256r1", KID: "test-kid", PublicKey: "test-public-key"}
	cache.store(keyInfo)
	if cache.get(keyInfo.URL, keyInfo.Algorithm, keyInfo.KID) == nil || cache.get(keyInfo.URL, keyInfo.Algorithm, "") == nil {
		t.Fatal("fresh")
	}
	clock.now += 6 * 60
	if cache.get(keyInfo.URL, keyInfo.Algorithm, keyInfo.KID) != nil {
		t.Fatal("expired key with specific KID should not be returned")
	}
	if cache.get(keyInfo.URL, keyInfo.Algorithm, "") != nil {
		t.Fatal("expired key should not be found with empty KID lookup")
	}
	if _, exists := cache.c[kasKeyRequest{keyInfo.URL, keyInfo.Algorithm, keyInfo.KID}]; exists {
		t.Fatal("expired key should be removed from cache")
	}
	cache.store(keyInfo)
	cache.clear()
	if cache.get(keyInfo.URL, keyInfo.Algorithm, keyInfo.KID) != nil {
		t.Fatal("clear")
	}
}

// Ported from Test_StoreKASKeys: stored keys are served from the cache
// without a KAS request.
func Test_StoreKASKeys(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	if err := s.StoreKASKeys(f.kasURL(), KasPublicKeySet{Keys: []KasPublicKey{{Pem: "a-pem", Kid: "stored", Alg: AlgorithmRSA2048}}}); err != nil {
		t.Fatal(err)
	}
	k, err := s.getPublicKey(context.Background(), f.kasURL(), "rsa:2048", "")
	if err != nil || k.PublicKey != "a-pem" || k.KID != "stored" || f.publicKeyCalls.Load() != 0 {
		t.Fatal("cached", k, err, f.publicKeyCalls.Load())
	}
	k, err = s.getPublicKey(context.Background(), f.kasURL(), "ec:secp256r1", "")
	if err != nil || k.KID != "e1" || f.publicKeyCalls.Load() != 1 {
		t.Fatal("fetched", k, err)
	}
	if _, err = s.getPublicKey(context.Background(), f.kasURL(), "ec:secp256r1", ""); err != nil || f.publicKeyCalls.Load() != 1 {
		t.Fatal("second lookup served from the cache")
	}
	if err := s.StoreKASKeys(f.kasURL(), KasPublicKeySet{Keys: []KasPublicKey{{Alg: "bogus"}}}); err == nil {
		t.Fatal("unknown algorithm")
	}
}

// Ported from Test_retrieveObligationsFromMetadata and its variants.
func TestRetrieveObligationsFromMetadata(t *testing.T) {
	k := &KASClient{}
	parse := func(s string) []string {
		v, err := parseJSON(t, s)
		if err != nil {
			t.Fatal(err)
		}
		return k.retrieveObligationsFromMetadata(v)
	}
	if got := parse(`{"X-Required-Obligations":["a","b"]}`); len(got) != 2 || got[0] != "a" || got[1] != "b" {
		t.Fatal(got)
	}
	if got := parse(`{}`); len(got) != 0 {
		t.Fatal(got)
	}
	if got := parse(`{"X-Required-Obligations":"not a list"}`); len(got) != 0 {
		t.Fatal(got)
	}
	if got := parse(`{"X-Required-Obligations":[]}`); len(got) != 0 {
		t.Fatal(got)
	}
}
