package sdk

import (
	"bytes"
	"context"
	"crypto"
	"crypto/ecdh"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/hmac"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"encoding/pem"
	"errors"
	"math/big"
	"net/http"
	"net/http/httptest"
	"net/url"
	"os/exec"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	capcrypto "github.com/eugenioenko/goalchemy/lib/crypto"
	"opentdf-local/sdk/tdf"
)

type wireKAO struct {
	Type, URL, Protocol, WrappedKey, Kid, Sid, EncryptedMetadata, EphemeralPublicKey string
	PolicyBinding                                                                    struct{ Alg, Hash string }
}
type wireBody struct {
	ClientPublicKey string
	Requests        []struct {
		Policy           struct{ ID, Body string }
		KeyAccessObjects []struct {
			KeyAccessObjectID string
			KeyAccessObject   wireKAO
		}
	}
}
type fixture struct {
	t             *testing.T
	server        *httptest.Server
	kas           *rsa.PrivateKey
	ecKAS         *ecdsa.PrivateKey
	algorithm     string
	public        *capcrypto.Key
	client        *Client
	tokenCalls    atomic.Int32
	resourceCalls atomic.Int32
	mode          atomic.Value
	sessions      sync.Map
}

func mustValue[T any](t *testing.T, v T, e error) T {
	t.Helper()
	if e != nil {
		t.Fatal(e)
	}
	return v
}
func newFixture(t *testing.T) *fixture {
	t.Helper()
	return newAlgorithmFixture(t, "rsa:2048")
}
func newAlgorithmFixture(t *testing.T, algorithm string) *fixture {
	t.Helper()
	f := &fixture{t: t, algorithm: algorithm}
	f.mode.Store("")
	var err error
	f.kas, err = rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		t.Fatal(err)
	}
	f.ecKAS, err = ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatal(err)
	}
	var wrappingPublic any = &f.kas.PublicKey
	if algorithm == "ec:secp256r1" {
		wrappingPublic = &f.ecKAS.PublicKey
	}
	der, err := x509.MarshalPKIXPublicKey(wrappingPublic)
	if err != nil {
		t.Fatal(err)
	}
	pub := string(pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: der}))
	f.public, err = capcrypto.ImportPEM(pub)
	if err != nil {
		t.Fatal(err)
	}
	f.server = httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		mode := f.mode.Load().(string)
		w.Header().Set("Content-Type", "application/json")
		if mode == "slow" {
			select {
			case <-r.Context().Done():
				return
			case <-time.After(2 * time.Second):
			}
		}
		if r.URL.Path == "/.well-known/opentdf-configuration" {
			issuer := f.server.URL + "/issuer"
			endpoint := issuer + "/protocol/openid-connect/token"
			if mode == "issuer-mismatch" {
				issuer = "https://evil.example/issuer"
			}
			if mode == "token-destination" {
				endpoint = "https://evil.example/token"
			}
			json.NewEncoder(w).Encode(map[string]any{"idp": map[string]any{"issuer": issuer, "token_endpoint": endpoint}})
			return
		}
		if r.URL.Path == "/issuer/protocol/openid-connect/token" {
			f.tokenCalls.Add(1)
			r.ParseForm()
			if r.Form.Get("client_id") != "client +&=?" || r.Form.Get("client_secret") != "secret +&=é?" || r.Form.Get("grant_type") != "client_credentials" {
				t.Error("incorrect form encoding")
				w.WriteHeader(400)
				return
			}
			response := map[string]any{"access_token": "valid-token", "token_type": "Bearer", "expires_in": 300}
			switch mode {
			case "oauth-dpop":
				response["token_type"] = "DPoP"
			case "oauth-empty":
				response["access_token"] = ""
			case "oauth-expired":
				response["expires_in"] = 0
			case "oauth-fractional":
				response["expires_in"] = 1.5
			case "oauth-missing":
				delete(response, "expires_in")
			case "oauth-limit":
				response["expires_in"] = 86401
			}
			json.NewEncoder(w).Encode(response)
			return
		}
		f.resourceCalls.Add(1)
		if mode == "large" {
			w.Write(bytes.Repeat([]byte(" "), (1<<20)+1))
			return
		}
		if mode == "invalid-json" {
			w.Write([]byte("{bad"))
			return
		}
		if mode == "wrong-content-type" {
			w.Header().Set("Content-Type", "text/html")
			w.Write([]byte(`{}`))
			return
		}
		if r.Header.Get("Authorization") != "Bearer valid-token" {
			w.WriteHeader(401)
			w.Write([]byte(`{"code":"unauthenticated"}`))
			return
		}
		if r.Header.Get("Connect-Protocol-Version") != "1" || r.Header.Get("Content-Type") != "application/json" {
			t.Error("incorrect Connect headers")
		}
		if mode == "redirect" {
			w.Header().Set("Location", f.server.URL+"/credentials-leak")
			w.WriteHeader(307)
			return
		}
		if r.URL.Path == "/rpc/kas.AccessService/PublicKey" {
			var req struct{ Algorithm, Fmt, V string }
			if json.NewDecoder(r.Body).Decode(&req) != nil || req.Algorithm != algorithm || req.Fmt != "pkcs8" || req.V != "2" {
				t.Error("pinned PublicKey request")
			}
			responsePEM := pub
			switch mode {
			case "discovery-mismatch":
				if algorithm == "rsa:2048" {
					responsePEM = testSPKI(t, &f.ecKAS.PublicKey)
				} else {
					responsePEM = testSPKI(t, &f.kas.PublicKey)
				}
			case "discovery-private":
				responsePEM = testPrivatePEM(t, f.ecKAS)
			case "discovery-curve":
				wrong, _ := ecdsa.GenerateKey(elliptic.P384(), rand.Reader)
				responsePEM = testSPKI(t, &wrong.PublicKey)
			}
			json.NewEncoder(w).Encode(map[string]string{"publicKey": responsePEM, "kid": "r1"})
			return
		}
		if r.URL.Path != "/rpc/kas.AccessService/Rewrap" {
			t.Error("unexpected route", r.URL.Path)
			w.WriteHeader(404)
			return
		}
		var outer struct{ SignedRequestToken string }
		if json.NewDecoder(r.Body).Decode(&outer) != nil {
			t.Error("outer request")
			return
		}
		parts := strings.Split(outer.SignedRequestToken, ".")
		if len(parts) != 3 {
			t.Error("SRT shape")
			return
		}
		header, _ := base64.RawURLEncoding.DecodeString(parts[0])
		var h struct{ Alg, Typ string }
		json.Unmarshal(header, &h)
		claims, _ := base64.RawURLEncoding.DecodeString(parts[1])
		var c struct {
			RequestBody string
			Iat, Exp    int64
		}
		if json.Unmarshal(claims, &c) != nil || c.Exp-c.Iat != 60 || c.Iat < time.Now().Unix()-2 || c.Iat > time.Now().Unix()+2 {
			t.Error("SRT claims")
		}
		pubpem, err := f.client.authKey.PublicPEM()
		if err != nil {
			f.client.mu.Lock()
			closed := f.client.closed
			f.client.mu.Unlock()
			if !closed {
				t.Error(err)
			}
			return
		}
		block, _ := pem.Decode([]byte(pubpem))
		key, err := x509.ParsePKIXPublicKey(block.Bytes)
		if err != nil {
			t.Error(err)
			return
		}
		signature, _ := base64.RawURLEncoding.DecodeString(parts[2])
		digest := sha256.Sum256([]byte(parts[0] + "." + parts[1]))
		valid := false
		switch p := key.(type) {
		case *rsa.PublicKey:
			valid = h.Alg == "RS256" && rsa.VerifyPKCS1v15(p, crypto.SHA256, digest[:], signature) == nil
		case *ecdsa.PublicKey:
			valid = h.Alg == "ES256" && len(signature) == 64 && ecdsa.Verify(p, digest[:], new(big.Int).SetBytes(signature[:32]), new(big.Int).SetBytes(signature[32:]))
		}
		if !valid {
			t.Error("independent SRT signature")
		}
		var body wireBody
		if json.Unmarshal([]byte(c.RequestBody), &body) != nil || len(body.Requests) != 1 || len(body.Requests[0].KeyAccessObjects) != 1 {
			t.Error("requestBody shape")
			return
		}
		group := body.Requests[0]
		entry := group.KeyAccessObjects[0]
		kao := entry.KeyAccessObject
		if group.Policy.ID != "policy" || entry.KeyAccessObjectID != "kao-0" || kao.Kid != "r1" || kao.Protocol != "kas" || kao.URL != f.server.URL+"/kas" {
			t.Error("request fields")
		}
		cipher, err := base64.StdEncoding.DecodeString(kao.WrappedKey)
		if err != nil {
			t.Error(err)
			return
		}
		var share []byte
		if algorithm == "ec:secp256r1" {
			if kao.Type != "ec-wrapped" || kao.EphemeralPublicKey == "" {
				t.Error("EC KAO fields missing")
				return
			}
			ephemeralBlock, _ := pem.Decode([]byte(kao.EphemeralPublicKey))
			if ephemeralBlock == nil {
				t.Error("EC ephemeral PEM")
				return
			}
			ephemeral, parseErr := x509.ParsePKIXPublicKey(ephemeralBlock.Bytes)
			if parseErr != nil {
				t.Error(parseErr)
				return
			}
			priv, _ := f.ecKAS.ECDH()
			peer, _ := ephemeral.(*ecdsa.PublicKey).ECDH()
			secret, deriveErr := priv.ECDH(peer)
			if deriveErr != nil {
				t.Error(deriveErr)
				return
			}
			share, err = testGCMOpen(testHKDF(t, secret), cipher)
		} else {
			if kao.Type != "wrapped" || kao.EphemeralPublicKey != "" {
				t.Error("RSA KAO fields")
				return
			}
			share, err = rsa.DecryptOAEP(sha1.New(), rand.Reader, f.kas, cipher, nil)
		}
		if err != nil {
			t.Error(err)
			return
		}
		digestBinding := hmac.New(sha256.New, share)
		digestBinding.Write([]byte(group.Policy.Body))
		wantBinding := base64.StdEncoding.EncodeToString([]byte(hex.EncodeToString(digestBinding.Sum(nil))))
		if kao.PolicyBinding.Alg != "HS256" || kao.PolicyBinding.Hash != wantBinding {
			t.Error("policy bytes/binding not preserved")
		}
		block, _ = pem.Decode([]byte(body.ClientPublicKey))
		session, err := x509.ParsePKIXPublicKey(block.Bytes)
		if err != nil {
			t.Error(err)
			return
		}
		if _, exists := f.sessions.LoadOrStore(body.ClientPublicKey, true); exists {
			t.Error("session key reused")
		}
		if mode == "recovered-length" {
			share = share[:31]
		}
		var wrapped []byte
		responsePublic := ""
		switch peer := session.(type) {
		case *rsa.PublicKey:
			if peer.N.BitLen() != 2048 {
				t.Error("RSA session size")
			}
			wrapped, err = rsa.EncryptOAEP(sha1.New(), rand.Reader, peer, share, nil)
		case *ecdsa.PublicKey:
			if peer.Curve != elliptic.P256() {
				t.Error("session curve")
				return
			}
			private, keyErr := ecdh.P256().GenerateKey(rand.Reader)
			if keyErr != nil {
				t.Error(keyErr)
				return
			}
			public, _ := peer.ECDH()
			secret, deriveErr := private.ECDH(public)
			if deriveErr != nil {
				t.Error(deriveErr)
				return
			}
			responsePublic = testSPKI(t, private.PublicKey())
			wrapped = testGCMSeal(t, testHKDF(t, secret), share)
		default:
			t.Error("unsupported session type")
			return
		}
		if err != nil {
			t.Error(err)
			return
		}
		result := map[string]any{"keyAccessObjectId": "kao-0", "status": "permit", "kasWrappedKey": base64.StdEncoding.EncodeToString(wrapped)}
		response := map[string]any{"responses": []any{map[string]any{"policyId": "policy", "results": []any{result}}}}
		if responsePublic != "" {
			response["sessionPublicKey"] = responsePublic
		}
		switch mode {
		case "session-missing":
			delete(response, "sessionPublicKey")
		case "session-empty", "rsa-session-empty":
			response["sessionPublicKey"] = ""
		case "session-malformed":
			response["sessionPublicKey"] = "invalid PEM"
		case "session-type":
			response["sessionPublicKey"] = testSPKI(t, &f.kas.PublicKey)
		case "session-private":
			response["sessionPublicKey"] = testPrivatePEM(t, f.ecKAS)
		case "session-curve":
			wrong, _ := ecdsa.GenerateKey(elliptic.P384(), rand.Reader)
			response["sessionPublicKey"] = testSPKI(t, &wrong.PublicKey)
		case "session-tamper":
			response["sessionPublicKey"] = testSPKI(t, &f.ecKAS.PublicKey)
		case "session-json-type":
			response["sessionPublicKey"] = 7
		case "frame-short":
			result["kasWrappedKey"] = base64.StdEncoding.EncodeToString(wrapped[:len(wrapped)-1])
		case "frame-long":
			result["kasWrappedKey"] = base64.StdEncoding.EncodeToString(append(wrapped, 0))
		case "nonce-tamper":
			wrapped[0] ^= 1
			result["kasWrappedKey"] = base64.StdEncoding.EncodeToString(wrapped)
		case "tag-tamper":
			wrapped[len(wrapped)-1] ^= 1
			result["kasWrappedKey"] = base64.StdEncoding.EncodeToString(wrapped)
		case "policy":
			response["responses"].([]any)[0].(map[string]any)["policyId"] = "wrong"
		case "id":
			result["keyAccessObjectId"] = "wrong"
		case "status":
			result["status"] = "ok"
		case "duplicate":
			response["responses"] = []any{response["responses"].([]any)[0], response["responses"].([]any)[0]}
		case "missing":
			response["responses"] = []any{}
		case "denied":
			delete(result, "kasWrappedKey")
			result["status"] = "fail"
			result["error"] = "authorization denied"
		case "obligation":
			result["metadata"] = map[string]any{"X-Required-Obligations": []string{"https://example.com/required"}}
		case "malformed-obligation":
			result["metadata"] = map[string]any{"X-Required-Obligations": "wrong"}
		case "unwrap":
			result["kasWrappedKey"] = base64.StdEncoding.EncodeToString(make([]byte, 256))
		case "both":
			result["error"] = "unexpected"
		}
		json.NewEncoder(w).Encode(response)
	}))
	f.client, err = New(Config{PlatformURL: f.server.URL, KASURL: f.server.URL + "/kas", AllowedKAS: []KASRoute{{URL: f.server.URL + "/kas", APIBaseURL: f.server.URL + "/rpc"}}, IssuerURL: f.server.URL + "/issuer", ClientID: "client +&=?", ClientSecret: "secret +&=é?", AllowHTTP: true, KASAlgorithm: algorithm})
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { f.client.Close(); f.public.Close(); f.server.Close() })
	return f
}
func (f *fixture) encrypt(t *testing.T) []byte {
	t.Helper()
	data, e := tdf.Encrypt([]byte{0, 255, 128, 1}, tdf.EncryptConfig{KASPublicKey: f.public, KASURL: f.server.URL + "/kas", KID: "r1", Algorithm: f.algorithm, Metadata: []byte("metadata")})
	if e != nil {
		t.Fatal(e)
	}
	return data
}
func TestOrdinaryDiscoveryRewrapAndCaching(t *testing.T) {
	f := newFixture(t)
	data, e := f.client.Create(context.Background(), []byte{0, 255, 128, 1}, tdf.EncryptConfig{Metadata: []byte("metadata"), SegmentHashAlgorithm: "HS256"})
	if e != nil {
		t.Fatal(e)
	}
	for i := 0; i < 2; i++ {
		d, e := f.client.Decrypt(context.Background(), data)
		if e != nil || !bytes.Equal(d.Payload, []byte{0, 255, 128, 1}) || string(d.Metadata) != "metadata" {
			t.Fatal("decrypt", e)
		}
	}
	if f.tokenCalls.Load() != 1 {
		t.Fatal("token not cached")
	}
	f.client.mu.Lock()
	f.client.token.ExpiresAt = time.Now().Unix()
	f.client.mu.Unlock()
	if _, e := f.client.Decrypt(context.Background(), data); e != nil {
		t.Fatal(e)
	}
	if f.tokenCalls.Load() != 2 {
		t.Fatal("expired token not refreshed")
	}
}
func TestStrictResponseAndTamper(t *testing.T) {
	f := newFixture(t)
	data := f.encrypt(t)
	for _, mode := range []string{"policy", "id", "status", "duplicate", "missing", "denied", "obligation", "malformed-obligation", "unwrap", "both", "redirect", "large", "invalid-json", "wrong-content-type"} {
		t.Run(mode, func(t *testing.T) {
			f.mode.Store(mode)
			d, e := f.client.Decrypt(context.Background(), data)
			if e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 {
				t.Fatal("failed open")
			}
			if mode == "obligation" {
				var typed *Error
				if !errors.As(e, &typed) || len(typed.RequiredObligations) != 1 {
					t.Fatal("obligation missing")
				}
			}
		})
	}
	f.mode.Store("")
	a, e := tdf.ReadArchive(data, tdf.DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	a.Payload[12] ^= 1
	bad, e := tdf.WriteArchive(a.Payload, a.Manifest, tdf.DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	d, e := f.client.Decrypt(context.Background(), bad)
	if e == nil || len(d.Payload) != 0 {
		t.Fatal("tamper accepted")
	}
}
func TestAllowlistBeforeCredentials(t *testing.T) {
	f := newFixture(t)
	data := f.encrypt(t)
	a, e := tdf.ReadArchive(data, tdf.DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	m, e := tdf.ParseManifest(a.Manifest)
	if e != nil {
		t.Fatal(e)
	}
	m.Encryption.KeyAccess[0].URL = f.server.URL + "/evil"
	manifest, e := m.Marshal()
	if e != nil {
		t.Fatal(e)
	}
	bad, e := tdf.WriteArchive(a.Payload, manifest, tdf.DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	if _, e = f.client.Decrypt(context.Background(), bad); e == nil {
		t.Fatal("allowlist accepted")
	}
	if f.tokenCalls.Load() != 0 || f.resourceCalls.Load() != 0 {
		t.Fatal("credentials transmitted")
	}
}
func TestDestinationProfileAgainstURLParser(t *testing.T) {
	for _, raw := range []string{"HTTPS://EXAMPLE.COM:443/proxy/", "http://localhost:80/kas", "http://127.0.0.1:8080/kas", "https://foo9.example/kas"} {
		u, e := parseEndpoint(raw, true)
		if e != nil {
			t.Fatal(raw, e)
		}
		standard, e := url.Parse(raw)
		if e != nil {
			t.Fatal(e)
		}
		host := strings.ToLower(standard.Hostname())
		port := standard.Port()
		if standard.Scheme == "https" && port == "443" || standard.Scheme == "http" && port == "80" {
			port = ""
		}
		expected := strings.ToLower(standard.Scheme) + "://" + host
		if port != "" {
			expected += ":" + port
		}
		expected += strings.TrimSuffix(standard.Path, "/")
		if u.String() != expected {
			t.Fatal(raw, u.String(), expected)
		}
	}
	for _, raw := range []string{"https://user@example.com/kas", "https://example.com/kas?x=1", "https://example.com/#kas", "https://example.com/%2fkas", "https://example.com/a/../kas", "https://example.com//kas", "https://example.com\\evil/kas", "https://[::1]/kas", "https://example.com:/kas", "https://example.com:1:2/kas", "http://127.1/kas", "http://0177.0.0.1/kas", "http://0x7f000001/kas", "http://0x7f00000c/kas", "http://0x7f.0.0.1/kas", "http://example.0x7f/kas", "http://example.123/kas", "http://99999999999999999999999999999999999999999999999999999999999999.0.0.1/kas", "https://example.com./kas"} {
		if _, e := parseEndpoint(raw, true); e == nil {
			t.Fatal("unsafe URL accepted", raw)
		}
	}
	if _, e := parseEndpoint("http://localhost/kas", false); e == nil {
		t.Fatal("HTTP without permission")
	}
}
func TestProviderInvalidTokensAndExpiry(t *testing.T) {
	f := newFixture(t)
	f.client.Close()
	var calls atomic.Int32
	token := AccessToken{Value: "valid-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}
	var err error
	f.client, err = New(Config{PlatformURL: f.server.URL, KASURL: f.server.URL + "/kas", AllowedKAS: []KASRoute{{URL: f.server.URL + "/kas", APIBaseURL: f.server.URL + "/rpc"}}, AllowHTTP: true, TokenProvider: func(ctx context.Context) (AccessToken, error) { calls.Add(1); return token, nil }})
	if err != nil {
		t.Fatal(err)
	}
	data := f.encrypt(t)
	for _, invalid := range []AccessToken{{Value: "valid-token", Scheme: "DPoP", ExpiresAt: time.Now().Unix() + 300}, {Value: "valid-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() - 1}, {Value: "bad\r\ntoken", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}, {Value: "", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}} {
		token = invalid
		d, e := f.client.Decrypt(context.Background(), data)
		if e == nil || len(d.Payload) != 0 {
			t.Fatal("invalid token accepted")
		}
	}
	token = AccessToken{Value: "stale-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}
	if _, e := f.client.Decrypt(context.Background(), data); e == nil {
		t.Fatal("stale token accepted")
	}
	token = AccessToken{Value: "valid-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}
	if _, e := f.client.Decrypt(context.Background(), data); e != nil {
		t.Fatal("401 did not invalidate cache", e)
	}
	if calls.Load() != 6 {
		t.Fatal("provider calls", calls.Load())
	}
	if f.tokenCalls.Load() != 0 {
		t.Fatal("provider used client secret auth")
	}
}
func TestCancellationDeadlinesCloseAndConcurrency(t *testing.T) {
	f := newFixture(t)
	data := f.encrypt(t)
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if _, e := f.client.Decrypt(ctx, data); !errors.Is(e, context.Canceled) {
		t.Fatal("canceled context", e)
	}
	if f.tokenCalls.Load() != 0 {
		t.Fatal("canceled request sent")
	}
	f.mode.Store("slow")
	ctx, cancel = context.WithTimeout(context.Background(), 30*time.Millisecond)
	start := time.Now()
	_, e := f.client.Decrypt(ctx, data)
	cancel()
	if e == nil || time.Since(start) > time.Second {
		t.Fatal("deadline not propagated", e)
	}
	f.mode.Store("")
	var wg sync.WaitGroup
	for i := 0; i < 8; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			if _, e := f.client.Decrypt(context.Background(), data); e != nil {
				t.Error(e)
			}
		}()
	}
	wg.Wait()
	if f.tokenCalls.Load() != 1 {
		t.Fatal("concurrent token cache", f.tokenCalls.Load())
	}
	f.mode.Store("slow")
	done := make(chan error, 1)
	go func() { _, e := f.client.Decrypt(context.Background(), data); done <- e }()
	time.Sleep(30 * time.Millisecond)
	f.client.Close()
	select {
	case e := <-done:
		if e == nil {
			t.Fatal("close succeeded in flight")
		}
	case <-time.After(time.Second):
		t.Fatal("Close did not cancel HTTP")
	}
	f.client.Close()
	if _, e := f.client.authKey.PublicPEM(); e == nil {
		t.Fatal("owned auth key still open")
	}
	if _, e := f.client.Decrypt(context.Background(), data); e == nil {
		t.Fatal("closed client used")
	}
}
func TestCallerOwnedKeysAndRS256(t *testing.T) {
	f := newFixture(t)
	f.client.Close()
	key, e := capcrypto.GenerateRSA2048()
	if e != nil {
		t.Fatal(e)
	}
	defer key.Close()
	f.client, e = New(Config{PlatformURL: f.server.URL, KASURL: f.server.URL + "/kas", AllowedKAS: []KASRoute{{URL: f.server.URL + "/kas", APIBaseURL: f.server.URL + "/rpc"}}, AllowHTTP: true, AuthKey: key, AuthAlgorithm: "RS256", KASPublicKey: f.public, KID: "r1", TokenProvider: func(context.Context) (AccessToken, error) {
		return AccessToken{Value: "valid-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}, nil
	}})
	if e != nil {
		t.Fatal(e)
	}
	data, e := f.client.Create(context.Background(), []byte("RSA"), tdf.EncryptConfig{})
	if e != nil {
		t.Fatal(e)
	}
	d, e := f.client.Decrypt(context.Background(), data)
	if e != nil || string(d.Payload) != "RSA" {
		t.Fatal(e)
	}
	f.client.Close()
	if _, e := key.PublicPEM(); e != nil {
		t.Fatal("caller auth key closed", e)
	}
	if _, e := f.public.PublicPEM(); e != nil {
		t.Fatal("caller KAS key closed", e)
	}
	if _, e := New(Config{DPoP: true}); e == nil {
		t.Fatal("DPoP accepted missing configuration")
	}
}
func TestCloseAndTimeoutCancelProvider(t *testing.T) {
	for _, closeClient := range []bool{false, true} {
		t.Run(map[bool]string{false: "timeout", true: "close"}[closeClient], func(t *testing.T) {
			entered := make(chan bool, 1)
			c, e := New(Config{PlatformURL: "https://platform.example", TimeoutMillis: 50, TokenProvider: func(ctx context.Context) (AccessToken, error) {
				entered <- true
				<-ctx.Done()
				return AccessToken{}, ctx.Err()
			}})
			if e != nil {
				t.Fatal(e)
			}
			defer c.Close()
			done := make(chan error, 1)
			go func() { _, _, e := c.PublicKey(context.Background()); done <- e }()
			<-entered
			if closeClient {
				c.Close()
			}
			select {
			case e := <-done:
				if e == nil {
					t.Fatal("provider completed successfully")
				}
			case <-time.After(time.Second):
				t.Fatal("provider cancellation not propagated")
			}
		})
	}
	var nilClient *Client
	nilClient.Close()
	if _, _, e := nilClient.PublicKey(context.Background()); e == nil {
		t.Fatal("nil accepted")
	}
	var zero Client
	zero.Close()
	if _, e := zero.Create(context.Background(), nil, tdf.EncryptConfig{}); e == nil {
		t.Fatal("zero accepted")
	}
}
func TestURLProfileAgainstWHATWG(t *testing.T) {
	inputs := []string{"HTTPS://EXAMPLE.COM:443/proxy/", "http://localhost:80/kas", "http://127.0.0.1:8080/kas", "http://0x7f000001/kas", "http://0x7f00000c/kas", "http://0x7f.0.0.1/kas", "http://0177.0.0.1/kas", "http://127.1/kas", "http://example.123/kas", "http://example.0x7f/kas", "http://99999999999999999999999999999999999999999999999999999999999999.0.0.1/kas"}
	input, _ := json.Marshal(inputs)
	script := `const inputs=JSON.parse(process.argv[1]);process.stdout.write(JSON.stringify(inputs.map(x=>{try{return new URL(x).href}catch{return ""}})))`
	result, e := exec.Command("node", "-e", script, string(input)).Output()
	if e != nil {
		t.Fatal("Node WHATWG URL oracle required for native acceptance", e)
	}
	var normalized []string
	if json.Unmarshal(result, &normalized) != nil || len(normalized) != len(inputs) {
		t.Fatal("Node oracle shape")
	}
	for i, raw := range inputs {
		shared, e := parseEndpoint(raw, true)
		if i < 3 {
			if e != nil || shared.String() != strings.TrimSuffix(normalized[i], "/") {
				t.Fatal("portable canonicalization", raw, e, normalized[i])
			}
		} else if e == nil {
			t.Fatal("WHATWG numeric ambiguity accepted", raw, normalized[i])
		}
	}
	if normalized[3] != "http://127.0.0.1/kas" || normalized[5] != "http://127.0.0.1/kas" {
		t.Fatal("independent hex normalization changed")
	}
}
func TestExactBoundPolicyAndSID(t *testing.T) {
	f := newFixture(t)
	policy, e := tdf.NewPolicy([]string{"attribute"}, nil)
	if e != nil {
		t.Fatal(e)
	}
	raw, e := policy.Marshal()
	if e != nil {
		t.Fatal(e)
	}
	var pretty bytes.Buffer
	if e := json.Indent(&pretty, raw, "", " "); e != nil {
		t.Fatal(e)
	}
	bound := base64.StdEncoding.EncodeToString(pretty.Bytes())
	data, e := tdf.Encrypt([]byte("exact policy"), tdf.EncryptConfig{KASPublicKey: f.public, KASURL: f.server.URL + "/kas", KID: "r1", PolicyBase64: bound})
	if e != nil {
		t.Fatal(e)
	}
	a, e := tdf.ReadArchive(data, tdf.DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	m, e := tdf.ParseManifest(a.Manifest)
	if e != nil {
		t.Fatal(e)
	}
	m.Encryption.KeyAccess[0].SID = "share-1"
	manifest, e := m.Marshal()
	if e != nil {
		t.Fatal(e)
	}
	data, e = tdf.WriteArchive(a.Payload, manifest, tdf.DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	request, e := rewrapBody(m, "session public")
	if e != nil {
		t.Fatal(e)
	}
	var body wireBody
	if json.Unmarshal(request, &body) != nil || body.Requests[0].Policy.Body != bound || body.Requests[0].KeyAccessObjects[0].KeyAccessObject.Sid != "share-1" {
		t.Fatal("bound policy/SID lost")
	}
	d, e := f.client.Decrypt(context.Background(), data)
	if e != nil || string(d.Payload) != "exact policy" {
		t.Fatal(e)
	}
}
func TestDiscoveryAndOAuthValidationBeforeResourceAccess(t *testing.T) {
	f := newFixture(t)
	data := f.encrypt(t)
	for _, mode := range []string{"issuer-mismatch", "token-destination", "oauth-dpop", "oauth-empty", "oauth-expired", "oauth-fractional", "oauth-missing", "oauth-limit"} {
		t.Run(mode, func(t *testing.T) {
			f.mode.Store(mode)
			d, e := f.client.Decrypt(context.Background(), data)
			if e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 {
				t.Fatal("invalid discovery/auth accepted")
			}
			if f.resourceCalls.Load() != 0 {
				t.Fatal("resource credentials transmitted")
			}
			if (mode == "issuer-mismatch" || mode == "token-destination") && f.tokenCalls.Load() != 0 {
				t.Fatal("secret transmitted on untrusted discovery")
			}
		})
	}
}
