package sdk

import (
	"bytes"
	"context"
	"crypto"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rsa"
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"errors"
	"io"
	"math/big"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	capcrypto "github.com/eugenioenko/goalchemy/lib/crypto"
	"opentdf-local/sdk/tdf"
)

// Independent RFC 7638 / JOSE verifier. The oracle uses no shared JWT/JWK helper.
func proofOracle(t *testing.T, token, method, htu, access string) (map[string]any, string, any) {
	t.Helper()
	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		t.Fatal("proof compact shape")
	}
	decode := func(s string) []byte {
		b, e := base64.RawURLEncoding.DecodeString(s)
		if e != nil {
			t.Fatal(e)
		}
		return b
	}
	var h, c map[string]any
	if json.Unmarshal(decode(parts[0]), &h) != nil || json.Unmarshal(decode(parts[1]), &c) != nil {
		t.Fatal("proof JSON")
	}
	if h["typ"] != "dpop+jwt" {
		t.Fatal("proof typ", h)
	}
	jwk, ok := h["jwk"].(map[string]any)
	if !ok {
		t.Fatal("public jwk")
	}
	for _, private := range []string{"d", "p", "q", "dp", "dq", "qi", "k"} {
		if _, exists := jwk[private]; exists {
			t.Fatal("private JWK leaked")
		}
	}
	var pub any
	var canonical []byte
	switch h["alg"] {
	case "RS256":
		if jwk["kty"] != "RSA" || len(jwk) != 3 {
			t.Fatal("RSA jwk")
		}
		n := new(big.Int).SetBytes(decode(jwk["n"].(string)))
		exp := new(big.Int).SetBytes(decode(jwk["e"].(string))).Int64()
		pub = &rsa.PublicKey{N: n, E: int(exp)}
		canonical, _ = json.Marshal(map[string]string{"e": jwk["e"].(string), "kty": "RSA", "n": jwk["n"].(string)})
	case "ES256":
		if jwk["kty"] != "EC" || jwk["crv"] != "P-256" || len(jwk) != 4 {
			t.Fatal("EC jwk")
		}
		x, y := decode(jwk["x"].(string)), decode(jwk["y"].(string))
		if len(x) != 32 || len(y) != 32 {
			t.Fatal("coordinate size")
		}
		pub = &ecdsa.PublicKey{Curve: elliptic.P256(), X: new(big.Int).SetBytes(x), Y: new(big.Int).SetBytes(y)}
		canonical, _ = json.Marshal(map[string]string{"crv": "P-256", "kty": "EC", "x": jwk["x"].(string), "y": jwk["y"].(string)})
	default:
		t.Fatal("proof algorithm")
	}
	sig := decode(parts[2])
	digest := sha256.Sum256([]byte(parts[0] + "." + parts[1]))
	switch p := pub.(type) {
	case *rsa.PublicKey:
		if p.N.BitLen() != 2048 || rsa.VerifyPKCS1v15(p, crypto.SHA256, digest[:], sig) != nil {
			t.Fatal("RSA proof signature")
		}
	case *ecdsa.PublicKey:
		if len(sig) != 64 || !ecdsa.Verify(p, digest[:], new(big.Int).SetBytes(sig[:32]), new(big.Int).SetBytes(sig[32:])) {
			t.Fatal("ES JOSE signature")
		}
	}
	iat, ok := c["iat"].(float64)
	if !ok || iat != float64(int64(iat)) || iat < float64(time.Now().Unix()-3) || iat > float64(time.Now().Unix()+3) {
		t.Fatal("real iat")
	}
	if c["htm"] != method || c["htu"] != htu {
		t.Fatal("full htm/htu", c)
	}
	id, ok := c["jti"].(string)
	if !ok || len(decode(id)) != 32 {
		t.Fatal("random jti")
	}
	if access == "" {
		if _, exists := c["ath"]; exists {
			t.Fatal("token proof ath")
		}
	} else {
		hash := sha256.Sum256([]byte(access))
		if c["ath"] != base64.RawURLEncoding.EncodeToString(hash[:]) {
			t.Fatal("token hash binding")
		}
	}
	hash := sha256.Sum256(canonical)
	return c, base64.RawURLEncoding.EncodeToString(hash[:]), pub
}
func bindingJWT(claims any) string {
	b, _ := json.Marshal(claims)
	return "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9." + base64.RawURLEncoding.EncodeToString(b) + ".c2ln"
}

type proofAttempt struct {
	origin, path, nonce, id string
	body                    []byte
}
type nonceReply struct {
	status int
	nonce  []string
}
type dpopFixture struct {
	f             *fixture
	oauth         *httptest.Server
	client        *Client
	mu            sync.Mutex
	attempts      []proofAttempt
	replies       []nonceReply
	tokenReplies  []nonceReply
	access, thumb string
	ids           map[string]bool
	tokenCalls    atomic.Int32
}

func newDPoPFixture(t *testing.T, wrapping, session, auth string) *dpopFixture {
	t.Helper()
	f := newAlgorithmFixture(t, wrapping)
	f.client.Close()
	d := &dpopFixture{f: f, ids: map[string]bool{}}
	original := f.server.Config.Handler
	record := func(r *http.Request, access string) (map[string]any, string) {
		claims, thumb, _ := proofOracle(t, r.Header.Get("DPoP"), r.Method, "http://"+r.Host+r.URL.EscapedPath(), access)
		body, e := io.ReadAll(r.Body)
		if e != nil {
			t.Error(e)
		}
		r.Body = io.NopCloser(bytes.NewReader(body))
		nonce, _ := claims["nonce"].(string)
		id := claims["jti"].(string)
		d.mu.Lock()
		defer d.mu.Unlock()
		if d.ids[id] {
			t.Error("proof jti reused")
		}
		d.ids[id] = true
		if d.thumb != "" && d.thumb != thumb {
			t.Error("auth key changed")
		}
		d.thumb = thumb
		d.attempts = append(d.attempts, proofAttempt{origin: r.Host, path: r.URL.Path, nonce: nonce, id: id, body: body})
		return claims, thumb
	}
	reply := func(w http.ResponseWriter, replies *[]nonceReply) bool {
		d.mu.Lock()
		var next nonceReply
		if len(*replies) > 0 {
			next = (*replies)[0]
			*replies = (*replies)[1:]
		}
		d.mu.Unlock()
		for _, n := range next.nonce {
			w.Header().Add("DPoP-Nonce", n)
		}
		w.Header().Set("Content-Type", "application/json")
		if next.status != 0 && next.status != 200 {
			w.WriteHeader(next.status)
			w.Write([]byte(`{"code":"unauthenticated"}`))
			return true
		}
		return false
	}
	d.oauth = httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		d.tokenCalls.Add(1)
		if r.Header.Get("Authorization") != "" {
			t.Error("token authorization")
		}
		_, thumb := record(r, "")
		if reply(w, &d.tokenReplies) {
			return
		}
		r.ParseForm()
		if r.Form.Get("client_id") != "client +&=?" || r.Form.Get("client_secret") != "secret +&=é?" || r.Form.Get("grant_type") != "client_credentials" {
			t.Error("OAuth form")
		}
		access := bindingJWT(map[string]any{"cnf": map[string]string{"jkt": thumb}, "exp": time.Now().Unix() + 300})
		d.mu.Lock()
		d.access = access
		d.mu.Unlock()
		json.NewEncoder(w).Encode(map[string]any{"access_token": access, "token_type": "dPoP", "expires_in": 300})
	}))
	f.server.Config.Handler = http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		d.mu.Lock()
		access := d.access
		d.mu.Unlock()
		if r.Header.Get("Authorization") != "DPoP "+access {
			t.Error("resource scheme")
		}
		record(r, access)
		if reply(w, &d.replies) {
			return
		}
		// The existing independent unwrap fixture checks the signed grouped request,
		// policy binding and session framing after this independent auth verifier.
		r.Header.Set("Authorization", "Bearer valid-token")
		original.ServeHTTP(w, r)
	})
	var e error
	d.client, e = New(Config{PlatformURL: f.server.URL, KASURL: f.server.URL + "/kas", AllowedKAS: []KASRoute{{URL: f.server.URL + "/kas", APIBaseURL: f.server.URL + "/rpc"}}, IssuerURL: d.oauth.URL, TokenURL: d.oauth.URL + "/token", ClientID: "client +&=?", ClientSecret: "secret +&=é?", AllowHTTP: true, KASAlgorithm: wrapping, SessionAlgorithm: session, AuthAlgorithm: auth, DPoP: true})
	if e != nil {
		t.Fatal(e)
	}
	f.client = d.client
	t.Cleanup(func() { d.client.Close(); d.oauth.Close() })
	return d
}
func (d *dpopFixture) snapshot() []proofAttempt {
	d.mu.Lock()
	defer d.mu.Unlock()
	return append([]proofAttempt(nil), d.attempts...)
}
func assertCode(t *testing.T, e error, code string) {
	t.Helper()
	var typed *Error
	if !errors.As(e, &typed) || typed.Code != code {
		t.Fatalf("want %s got %v", code, e)
	}
}

func TestDPoPIndependentAlgorithmMatrix(t *testing.T) {
	for _, w := range []string{"rsa:2048", "ec:secp256r1"} {
		for _, s := range []string{"rsa:2048", "ec:secp256r1"} {
			for _, a := range []string{"RS256", "ES256"} {
				t.Run(w+"/"+s+"/"+a, func(t *testing.T) {
					d := newDPoPFixture(t, w, s, a)
					d.tokenReplies = []nonceReply{{status: 400, nonce: []string{"idp-initial"}}, {status: 200, nonce: []string{"idp-next"}}}
					d.replies = []nonceReply{{status: 401, nonce: []string{"kas-initial"}}, {status: 200, nonce: []string{"kas-next"}}, {status: 401, nonce: []string{"kas-rotated"}}, {status: 200, nonce: []string{"kas-final"}}}
					plain := []byte{0, 255, 1, 128}
					metadata := []byte(`{"independent":7}`)
					data, e := d.client.Create(context.Background(), plain, tdf.EncryptConfig{Metadata: metadata})
					if e != nil {
						t.Fatal(e)
					}
					got, e := d.client.Decrypt(context.Background(), data)
					if e != nil || !bytes.Equal(got.Payload, plain) || !bytes.Equal(got.Metadata, metadata) {
						t.Fatal("DPoP decrypt", e)
					}
					attempts := d.snapshot()
					if len(attempts) != 6 || d.tokenCalls.Load() != 2 {
						t.Fatal("bounded challenges", len(attempts))
					}
					wants := []string{"", "idp-initial", "", "kas-initial", "kas-next", "kas-rotated"}
					for i, x := range attempts {
						if x.nonce != wants[i] {
							t.Fatalf("nonce/origin[%d]: %s", i, x.nonce)
						}
					}
					if !bytes.Equal(attempts[0].body, attempts[1].body) || !bytes.Equal(attempts[2].body, attempts[3].body) || !bytes.Equal(attempts[4].body, attempts[5].body) {
						t.Fatal("retry changed body/SRT/session")
					}
					key, _, e := d.client.PublicKey(context.Background())
					if e != nil {
						t.Fatal(e)
					}
					key.Close()
					last := d.snapshot()
					if last[len(last)-1].nonce != "kas-final" || d.tokenCalls.Load() != 2 {
						t.Fatal("successful nonce/cache update")
					}
					d.client.Close()
					if len(d.client.nonces) != 0 || d.client.token.Value != "" {
						t.Fatal("close clears caches")
					}
				})
			}
		}
	}
}
func TestDPoPNonceRetryBounds(t *testing.T) {
	cases := []struct {
		name    string
		seed    string
		replies []nonceReply
		count   int
		code    string
	}{
		{"plain401", "", []nonceReply{{status: 401}}, 1, "unauthenticated"},
		{"same401", "same", []nonceReply{{status: 401, nonce: []string{"same"}}}, 1, "unauthenticated"},
		{"same400", "same", []nonceReply{{status: 400, nonce: []string{"same"}}}, 1, "http_status"},
		{"second-challenge", "", []nonceReply{{status: 401, nonce: []string{"one"}}, {status: 401, nonce: []string{"two"}}}, 2, "unauthenticated"},
		{"duplicate", "", []nonceReply{{status: 401, nonce: []string{"one", "two"}}}, 1, "invalid_nonce"},
		{"empty", "", []nonceReply{{status: 200, nonce: []string{""}}}, 1, "invalid_nonce"},
		{"comma", "", []nonceReply{{status: 401, nonce: []string{"one, two"}}}, 1, "invalid_nonce"},
		{"space", "", []nonceReply{{status: 401, nonce: []string{"bad nonce"}}}, 1, "invalid_nonce"},
		{"quote", "", []nonceReply{{status: 401, nonce: []string{`bad"nonce`}}}, 1, "invalid_nonce"},
		{"backslash", "", []nonceReply{{status: 401, nonce: []string{`bad\nonce`}}}, 1, "invalid_nonce"},
		{"large", "", []nonceReply{{status: 401, nonce: []string{strings.Repeat("x", 1025)}}}, 1, "invalid_nonce"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			d := newDPoPFixture(t, "rsa:2048", "rsa:2048", "ES256")
			d.replies = tc.replies
			if tc.seed != "" {
				u, _ := parseEndpoint(d.f.server.URL, true)
				d.client.cacheNonce(u.origin, tc.seed)
			}
			_, _, e := d.client.PublicKey(context.Background())
			assertCode(t, e, tc.code)
			var typed *Error
			if !errors.As(e, &typed) || typed.HTTPStatus != tc.replies[tc.count-1].status {
				t.Fatal("nonce/status diagnostics", e)
			}
			attempts := d.snapshot()
			if len(attempts)-1 != tc.count {
				t.Fatal("retry count", len(attempts)-1)
			}
			if (tc.code == "unauthenticated" || tc.code == "invalid_nonce" && tc.replies[0].status == 401) && d.client.token.Value != "" {
				t.Fatal("final 401 must evict token")
			}
		})
	}
}

func TestDPoPTokenBindingContract(t *testing.T) {
	for _, algorithm := range []string{"ES256", "RS256"} {
		t.Run(algorithm, func(t *testing.T) {
			var key *capcrypto.Key
			var e error
			if algorithm == "RS256" {
				key, e = capcrypto.GenerateRSA2048()
			} else {
				key, e = capcrypto.GenerateP256()
			}
			if e != nil {
				t.Fatal(e)
			}
			defer key.Close()
			// Independently obtain the RFC 7638 thumbprint by verifying a real proof.
			c, e := New(Config{PlatformURL: "https://example.com", AuthAlgorithm: algorithm, AuthKey: key, DPoP: true, TokenProvider: func(context.Context) (AccessToken, error) { return AccessToken{}, nil }})
			if e != nil {
				t.Fatal(e)
			}
			proof, e := c.dpopProof("POST", "https://example.com/", "", "")
			if e != nil {
				t.Fatal(e)
			}
			_, thumb, _ := proofOracle(t, proof, "POST", "https://example.com/", "")
			cases := []struct{ name, value, scheme, confirmation, code string }{
				{"bound-jwt", bindingJWT(map[string]any{"cnf": map[string]string{"jkt": thumb}}), "DPoP", "", ""},
				{"unbound-jwt", bindingJWT(map[string]any{"sub": "subject"}), "DPoP", thumb, "token_binding"},
				{"mismatched-jwt", bindingJWT(map[string]any{"cnf": map[string]string{"jkt": "wrong"}}), "DPoP", thumb, "token_binding"},
				{"nonobject-cnf", bindingJWT(map[string]any{"cnf": thumb}), "DPoP", thumb, "token_binding"},
				{"duplicate-cnf", "eyJhbGciOiJSUzI1NiJ9." + base64.RawURLEncoding.EncodeToString([]byte(`{"cnf":{},"cnf":{"jkt":"`+thumb+`"}}`)) + ".c2ln", "DPoP", thumb, "token_binding"},
				{"malformed-jwt", "eyJhbGciOiJSUzI1NiJ9.bad.c2ln", "DPoP", thumb, "token_binding"},
				{"opaque-bound", "opaque", "DPoP", thumb, ""},
				{"opaque-dotted", "opaque.token.with.dots", "DPoP", thumb, ""},
				{"opaque-three-parts", "opaque.token.signature", "DPoP", thumb, ""},
				{"opaque-unbound", "opaque", "DPoP", "", "token_binding"},
				{"opaque-mismatch", "opaque", "DPoP", "wrong", "token_binding"},
				{"no-downgrade", "opaque", "Bearer", thumb, "unsupported_token_scheme"},
				{"invalid-expiry", "opaque", "DPoP", thumb, "invalid_token"},
			}
			for _, tc := range cases {
				t.Run(tc.name, func(t *testing.T) {
					expiry := time.Now().Unix() + 60
					if tc.name == "invalid-expiry" {
						expiry = time.Now().Unix() - 1
					}
					c.config.TokenProvider = func(context.Context) (AccessToken, error) {
						return AccessToken{Value: tc.value, Scheme: tc.scheme, ExpiresAt: expiry, ConfirmationJKT: tc.confirmation}, nil
					}
					c.token = AccessToken{}
					got, e := c.accessToken(context.Background())
					if tc.code != "" {
						assertCode(t, e, tc.code)
						if got.Value != "" {
							t.Fatal("failure leaked token")
						}
					} else if e != nil || got.Value != tc.value {
						t.Fatal(e)
					}
				})
			}
			c.Close()
			if _, e := key.PublicPEM(); e != nil {
				t.Fatal("borrowed auth key closed")
			}
		})
	}
}
func TestDPoPProviderCacheCancellationClose(t *testing.T) {
	key, e := capcrypto.GenerateP256()
	if e != nil {
		t.Fatal(e)
	}
	defer key.Close()
	// Provider binding comes from independent stdlib JWK encoding.
	var calls atomic.Int32
	var mode atomic.Int32
	entered := make(chan bool, 1)
	var thumb string
	c, e := New(Config{PlatformURL: "https://example.com", AuthKey: key, DPoP: true, TimeoutMillis: 80, TokenProvider: func(ctx context.Context) (AccessToken, error) {
		calls.Add(1)
		if mode.Load() != 0 {
			entered <- true
			<-ctx.Done()
			return AccessToken{Value: "opaque", Scheme: "DPoP", ExpiresAt: time.Now().Unix() + 300, ConfirmationJKT: thumb}, nil
		}
		return AccessToken{Value: "opaque", Scheme: "DPoP", ExpiresAt: time.Now().Unix() + 300, ConfirmationJKT: thumb}, nil
	}})
	if e != nil {
		t.Fatal(e)
	}
	defer c.Close()
	p, e := c.dpopProof("GET", "https://example.com/", "", "")
	if e != nil {
		t.Fatal(e)
	}
	_, thumb, _ = proofOracle(t, p, "GET", "https://example.com/", "")
	var wg sync.WaitGroup
	for i := 0; i < 16; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			if _, e := c.accessToken(context.Background()); e != nil {
				t.Error(e)
			}
		}()
	}
	wg.Wait()
	if calls.Load() != 1 {
		t.Fatal("provider refresh serialized", calls.Load())
	}
	c.mu.Lock()
	c.token.ExpiresAt = time.Now().Unix() + 4
	c.mu.Unlock()
	if _, e := c.accessToken(context.Background()); e != nil {
		t.Fatal(e)
	}
	if calls.Load() != 2 {
		t.Fatal("refresh margin")
	}
	c.mu.Lock()
	c.token = AccessToken{}
	c.mu.Unlock()
	mode.Store(1)
	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan error, 1)
	go func() { _, e := c.accessToken(ctx); done <- e }()
	<-entered
	cancel()
	assertCode(t, <-done, "canceled")
	if c.token.Value != "" {
		t.Fatal("canceled callback repopulated token")
	}
	go func() { _, e := c.accessToken(context.Background()); done <- e }()
	<-entered
	assertCode(t, <-done, "canceled")
	go func() { _, e := c.accessToken(context.Background()); done <- e }()
	<-entered
	c.Close()
	e = <-done
	var typed *Error
	if !errors.As(e, &typed) || typed.Code != "canceled" && typed.Code != "closed" {
		t.Fatal(e)
	}
	c.cacheNonce("https://example.com", "late")
	if len(c.nonces) != 0 || c.token.Value != "" {
		t.Fatal("Close repopulated caches")
	}
}
func TestDPoPFullURIProofNormalization(t *testing.T) {
	c, e := New(Config{PlatformURL: "https://example.com", DPoP: true, TokenProvider: func(context.Context) (AccessToken, error) { return AccessToken{}, nil }})
	if e != nil {
		t.Fatal(e)
	}
	defer c.Close()
	var target string
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		proofOracle(t, r.Header.Get("DPoP"), "POST", target, "")
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(`{}`))
	}))
	defer server.Close()
	c.config.AllowHTTP = true
	for _, path := range []string{"", "/", "/nested/", "/nested/path"} {
		raw := strings.Replace(server.URL, "http://", "HTTP://", 1) + path
		target = server.URL + path
		if path == "" {
			target += "/"
		}
		_, e := c.exchangeProof(context.Background(), "POST", raw, []string{"Content-Type", "application/json"}, []byte(`{}`), "oracle", "", true)
		if e != nil {
			t.Fatal(e)
		}
	}
	for _, raw := range []string{"https://EXAMPLE.com:443/", "http://EXAMPLE.com:80/nested/"} {
		u, e := parseEndpoint(raw, true)
		if e != nil {
			t.Fatal(e)
		}
		expected := "https://example.com/"
		if strings.HasPrefix(raw, "http:") {
			expected = "http://example.com/nested/"
		}
		path := u.path
		if strings.HasSuffix(raw, "/") {
			path += "/"
		}
		if path == "" {
			path = "/"
		}
		p, e := c.dpopProof("POST", u.origin+path, "", "")
		if e != nil {
			t.Fatal(e)
		}
		proofOracle(t, p, "POST", expected, "")
	}
	c.authKey.Close()
	_, e = c.exchangeProof(context.Background(), "POST", server.URL, nil, nil, "oracle", "", true)
	assertCode(t, e, "auth_key")
}

func TestDPoPConcurrentNonceCacheAndHTTPShutdown(t *testing.T) {
	d := newDPoPFixture(t, "ec:secp256r1", "ec:secp256r1", "ES256")
	d.replies = []nonceReply{{status: 401, nonce: []string{"shared"}}, {status: 200}}
	k, _, e := d.client.PublicKey(context.Background())
	if e != nil {
		t.Fatal(e)
	}
	k.Close()
	var wg sync.WaitGroup
	for i := 0; i < 12; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			k, _, e := d.client.PublicKey(context.Background())
			if e != nil {
				t.Error(e)
				return
			}
			k.Close()
		}()
	}
	wg.Wait()
	if d.tokenCalls.Load() != 1 {
		t.Fatal("concurrent token acquisition")
	}
	for _, a := range d.snapshot()[2:] {
		if a.nonce != "shared" {
			t.Fatal("concurrent nonce", a.nonce)
		}
	}
	// Real in-flight resource cancellation and a late response must not restore caches.
	entered := make(chan bool, 1)
	old := d.f.server.Config.Handler
	d.f.server.Config.Handler = http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		entered <- true
		select {
		case <-r.Context().Done():
			return
		case <-time.After(time.Second):
			old.ServeHTTP(w, r)
		}
	})
	done := make(chan error, 1)
	go func() { _, _, e := d.client.PublicKey(context.Background()); done <- e }()
	<-entered
	d.client.Close()
	var typed *Error
	e = <-done
	if !errors.As(e, &typed) || typed.Code != "transport" && typed.Code != "closed" && typed.Code != "canceled" {
		t.Fatal(e)
	}
	if len(d.client.nonces) != 0 || d.client.token.Value != "" {
		t.Fatal("inflight response repopulated closed cache")
	}
}

func TestDPoPOAuthOpaqueAndJWTBinding(t *testing.T) {
	for _, tc := range []struct {
		name, scheme, code string
		claims             any
	}{
		{"opaque", "DPoP", "", nil},
		{"dotted-opaque", "DPoP", "", nil},
		{"unbound-jwt", "DPoP", "token_binding", map[string]any{"sub": "unbound"}},
		{"wrong-jwt", "DPoP", "token_binding", map[string]any{"cnf": map[string]string{"jkt": "wrong"}}},
		{"bearer-downgrade", "Bearer", "unsupported_token_scheme", nil},
	} {
		t.Run(tc.name, func(t *testing.T) {
			var c *Client
			server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				if r.Header.Get("Authorization") != "" {
					t.Error("OAuth token authorization")
				}
				proofOracle(t, r.Header.Get("DPoP"), "POST", "http://"+r.Host+"/token", "")
				access := "opaque-token"
				if tc.name == "dotted-opaque" {
					access = "opaque.token.with.dots"
				}
				if tc.claims != nil {
					access = bindingJWT(tc.claims)
				}
				w.Header().Set("Content-Type", "application/json")
				json.NewEncoder(w).Encode(map[string]any{"access_token": access, "token_type": tc.scheme, "expires_in": 300})
			}))
			defer server.Close()
			var e error
			c, e = New(Config{PlatformURL: server.URL, IssuerURL: server.URL, TokenURL: server.URL + "/token", ClientID: "client", ClientSecret: "secret", AllowHTTP: true, DPoP: true})
			if e != nil {
				t.Fatal(e)
			}
			defer c.Close()
			token, e := c.accessToken(context.Background())
			if tc.code != "" {
				assertCode(t, e, tc.code)
				if token.Value != "" {
					t.Fatal("failed binding token leaked")
				}
			} else if e != nil || token.Scheme != "DPoP" || token.ConfirmationJKT == "" {
				t.Fatal("opaque OAuth attestation", e)
			}
		})
	}
}
func TestDPoPOAuthRetryBounds(t *testing.T) {
	for _, tc := range []struct {
		name    string
		replies []nonceReply
		count   int
		code    string
	}{
		{"plain401", []nonceReply{{status: 401}}, 1, "unauthenticated"},
		{"same400", []nonceReply{{status: 400, nonce: []string{"one"}}, {status: 400, nonce: []string{"one"}}}, 2, "http_status"},
		{"second-distinct400", []nonceReply{{status: 400, nonce: []string{"one"}}, {status: 400, nonce: []string{"two"}}}, 2, "http_status"},
		{"duplicate", []nonceReply{{status: 400, nonce: []string{"one", "two"}}}, 1, "invalid_nonce"},
	} {
		t.Run(tc.name, func(t *testing.T) {
			d := newDPoPFixture(t, "rsa:2048", "rsa:2048", "RS256")
			d.tokenReplies = tc.replies
			_, _, e := d.client.PublicKey(context.Background())
			assertCode(t, e, tc.code)
			if d.tokenCalls.Load() != int32(tc.count) || d.f.resourceCalls.Load() != 0 {
				t.Fatal("OAuth replay/resource bound")
			}
		})
	}
}
func TestDPoPRedirectDoesNotLeakProof(t *testing.T) {
	var leaked atomic.Int32
	dest := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) { leaked.Add(1) }))
	defer dest.Close()
	source := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		proofOracle(t, r.Header.Get("DPoP"), "POST", "http://"+r.Host+"/token", "")
		w.Header().Set("Location", dest.URL)
		w.Header().Set("DPoP-Nonce", "redirect-nonce")
		w.WriteHeader(307)
	}))
	defer source.Close()
	c, e := New(Config{PlatformURL: source.URL, IssuerURL: source.URL, TokenURL: source.URL + "/token", ClientID: "client", ClientSecret: "secret", AllowHTTP: true, DPoP: true})
	if e != nil {
		t.Fatal(e)
	}
	defer c.Close()
	_, e = c.accessToken(context.Background())
	assertCode(t, e, "http_status")
	if leaked.Load() != 0 || len(c.nonces) != 0 {
		t.Fatal("redirect proof/nonce leak")
	}
}

func TestDPoPSecretlessProviderResourceRoundTrip(t *testing.T) {
	for _, auth := range []string{"ES256", "RS256"} {
		t.Run(auth, func(t *testing.T) {
			d := newDPoPFixture(t, "ec:secp256r1", "ec:secp256r1", auth)
			token, e := d.client.accessToken(context.Background())
			if e != nil {
				t.Fatal(e)
			}
			borrowed := d.client.authKey
			var calls atomic.Int32
			c, e := New(Config{PlatformURL: d.f.server.URL, KASURL: d.f.server.URL + "/kas", AllowedKAS: []KASRoute{{URL: d.f.server.URL + "/kas", APIBaseURL: d.f.server.URL + "/rpc"}}, AllowHTTP: true, KASAlgorithm: "ec:secp256r1", SessionAlgorithm: "ec:secp256r1", AuthAlgorithm: auth, AuthKey: borrowed, DPoP: true, TokenProvider: func(context.Context) (AccessToken, error) { calls.Add(1); return token, nil }})
			if e != nil {
				t.Fatal(e)
			}
			defer c.Close()
			d.f.client = c
			d.replies = []nonceReply{{status: 401, nonce: []string{"provider-nonce"}}, {status: 200}}
			public, _, e := c.PublicKey(context.Background())
			if e != nil {
				t.Fatal(e)
			}
			public.Close()
			data := d.f.encrypt(t)
			result, e := c.Decrypt(context.Background(), data)
			if e != nil || !bytes.Equal(result.Payload, []byte{0, 255, 128, 1}) || string(result.Metadata) != "metadata" {
				t.Fatal("provider resource/SRT round trip", e)
			}
			if calls.Load() != 1 || d.tokenCalls.Load() != 1 {
				t.Fatal("secretless provider unexpectedly used OAuth")
			}
			c.Close()
			if _, e := borrowed.PublicPEM(); e != nil {
				t.Fatal("provider closed borrowed auth key")
			}
		})
	}
}
