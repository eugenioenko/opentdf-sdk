package main

import (
	"bytes"
	"context"
	"crypto"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"io"
	"math/big"
	"net/http"
	"net/http/httptest"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"sync"
	"time"

	capcrypto "github.com/eugenioenko/goalchemy/lib/crypto"
	shared "opentdf-local/sdk"
	"opentdf-local/sdk/tdf"
)

var wireClient = &http.Client{Timeout: 15 * time.Second, CheckRedirect: func(*http.Request, []*http.Request) error { return http.ErrUseLastResponse }}

func assertEnforcedDiscovery() {
	r := must(wireClient.Get(platform + "/.well-known/opentdf-configuration"))
	defer r.Body.Close()
	if r.StatusCode != 200 {
		panic("enforced discovery status")
	}
	v := map[string]any{}
	check(json.NewDecoder(r.Body).Decode(&v))
	if v["dpop_nonce_required"] != true {
		panic("live profile does not require DPoP nonce")
	}
	if strings.TrimSpace(string(must(os.ReadFile(filepath.Join(sdk, ".local/profiles/active"))))) != "dpop" {
		panic("live profile is not DPoP")
	}
}
func stockWebDPoPFailure(label string) string {
	output := filepath.Join(run, label+".web-dpop-negative.out")
	remove(output)
	ctx, cancel := context.WithTimeout(context.Background(), 45*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "node", filepath.Join(sdk, ".local/web-cli/bin/opentdf.mjs"), "decrypt", filepath.Join(run, label+".tdf"), "--rewrapKeyType", "ec:secp256r1", "--allowList", platform, "--output", output, "--platformUrl", platform, "--kasEndpoint", kas, "--oidcEndpoint", issuer, "--clientId", "opentdf-sdk", "--clientSecret", "secret", "--logLevel", "error", "--dpop")
	diagnostic, e := cmd.CombinedOutput()
	if ctx.Err() != nil || e == nil {
		panic("stock Web DPoP expected actual authentication rejection")
	}
	text := strings.ToLower(string(diagnostic))
	if !strings.Contains(text, "unauthenticated") && !strings.Contains(text, "unauthorized") {
		panic("stock Web DPoP failed without HTTP401 authentication evidence")
	}
	if b, e := os.ReadFile(output); e == nil && len(b) > 0 {
		panic("stock Web DPoP failure emitted plaintext")
	}
	return "stock Web --dpop: actual HTTP401/unauthenticated on enforced native nonce profile; no plaintext"
}

// Independent native test oracle/mutation transport. No helper retrieves or
// unwraps a payload key. Success bodies contain KAS ciphertext shares only;
// actual shared client sessions perform all unwraps and integrity checks.
type fixtureAuth struct {
	private          any
	public           any
	algorithm, thumb string
	jwk              map[string]string
	handle           *capcrypto.Key
}

func newFixtureAuth(algorithm string) *fixtureAuth {
	a := &fixtureAuth{algorithm: algorithm}
	if algorithm == "RS256" {
		k := must(rsa.GenerateKey(rand.Reader, 2048))
		a.private = k
		a.public = &k.PublicKey
		a.jwk = map[string]string{"kty": "RSA", "n": base64.RawURLEncoding.EncodeToString(k.N.Bytes()), "e": base64.RawURLEncoding.EncodeToString(big.NewInt(int64(k.E)).Bytes())}
	} else {
		k := must(ecdsa.GenerateKey(elliptic.P256(), rand.Reader))
		a.private = k
		a.public = &k.PublicKey
		a.jwk = map[string]string{"kty": "EC", "crv": "P-256", "x": base64.RawURLEncoding.EncodeToString(k.X.FillBytes(make([]byte, 32))), "y": base64.RawURLEncoding.EncodeToString(k.Y.FillBytes(make([]byte, 32)))}
	}
	h := sha256.Sum256(must(json.Marshal(a.jwk)))
	a.thumb = base64.RawURLEncoding.EncodeToString(h[:])
	a.handle = must(capcrypto.ImportPEM(string(pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: must(x509.MarshalPKCS8PrivateKey(a.private))}))))
	return a
}
func (a *fixtureAuth) sign(header, claims any) string {
	h := base64.RawURLEncoding.EncodeToString(must(json.Marshal(header)))
	c := base64.RawURLEncoding.EncodeToString(must(json.Marshal(claims)))
	input := h + "." + c
	digest := sha256.Sum256([]byte(input))
	var signature []byte
	switch k := a.private.(type) {
	case *rsa.PrivateKey:
		signature = must(rsa.SignPKCS1v15(rand.Reader, k, crypto.SHA256, digest[:]))
	case *ecdsa.PrivateKey:
		r, s, e := ecdsa.Sign(rand.Reader, k, digest[:])
		check(e)
		signature = append(r.FillBytes(make([]byte, 32)), s.FillBytes(make([]byte, 32))...)
	}
	return input + "." + base64.RawURLEncoding.EncodeToString(signature)
}
func (a *fixtureAuth) proof(token, path, nonce string, edit func(map[string]any)) string {
	random := make([]byte, 32)
	_, e := rand.Read(random)
	check(e)
	ath := sha256.Sum256([]byte(token))
	v := map[string]any{"jti": base64.RawURLEncoding.EncodeToString(random), "iat": time.Now().Unix(), "htm": "POST", "htu": path, "ath": base64.RawURLEncoding.EncodeToString(ath[:])}
	if nonce != "" {
		v["nonce"] = nonce
	}
	if edit != nil {
		edit(v)
	}
	return a.sign(map[string]any{"alg": a.algorithm, "typ": "dpop+jwt", "jwk": a.jwk}, v)
}
func (a *fixtureAuth) verify(token string) (map[string]any, map[string]any) {
	p := strings.Split(token, ".")
	if len(p) != 3 {
		panic("fixture JWT shape")
	}
	h, c := map[string]any{}, map[string]any{}
	check(json.Unmarshal(must(base64.RawURLEncoding.DecodeString(p[0])), &h))
	check(json.Unmarshal(must(base64.RawURLEncoding.DecodeString(p[1])), &c))
	signature := must(base64.RawURLEncoding.DecodeString(p[2]))
	digest := sha256.Sum256([]byte(p[0] + "." + p[1]))
	if h["alg"] != a.algorithm {
		panic("fixture JWT signing algorithm")
	}
	switch k := a.public.(type) {
	case *rsa.PublicKey:
		check(rsa.VerifyPKCS1v15(k, crypto.SHA256, digest[:], signature))
	case *ecdsa.PublicKey:
		if len(signature) != 64 || !ecdsa.Verify(k, digest[:], new(big.Int).SetBytes(signature[:32]), new(big.Int).SetBytes(signature[32:])) {
			panic("fixture JOSE signature")
		}
	}
	return h, c
}
func wireRequest(ctx context.Context, path, token, proof string, body []byte) (int, http.Header, []byte, error) {
	req, e := http.NewRequestWithContext(ctx, "POST", path, bytes.NewReader(body))
	if e != nil {
		return 0, nil, nil, e
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Connect-Protocol-Version", "1")
	req.Header.Set("Authorization", "DPoP "+token)
	req.Header.Set("DPoP", proof)
	response, e := wireClient.Do(req)
	if e != nil {
		return 0, nil, nil, e
	}
	defer response.Body.Close()
	data, e := io.ReadAll(io.LimitReader(response.Body, 1<<20))
	return response.StatusCode, response.Header, data, e
}
func corruptSignature(token string) string {
	p := strings.Split(token, ".")
	signature := must(base64.RawURLEncoding.DecodeString(p[2]))
	signature[0] ^= 1
	p[2] = base64.RawURLEncoding.EncodeToString(signature)
	return strings.Join(p, ".")
}

type protocolProxy struct {
	auth        *fixtureAuth
	other       *fixtureAuth
	url         string
	mu          sync.Mutex
	mode, nonce string
	evidence    []map[string]any
}

func (p *protocolProxy) forward(w http.ResponseWriter, r *http.Request) {
	body, e := io.ReadAll(io.LimitReader(r.Body, 1<<20))
	if e != nil {
		http.Error(w, "fixture read", 500)
		return
	}
	auth := strings.TrimPrefix(r.Header.Get("Authorization"), "DPoP ")
	header, claims := p.auth.verify(r.Header.Get("DPoP"))
	if header["typ"] != "dpop+jwt" || claims["htm"] != "POST" || claims["htu"] != p.url+r.URL.Path {
		panic("shared incoming DPoP route")
	}
	expectedATH := sha256.Sum256([]byte(auth))
	if claims["ath"] != base64.RawURLEncoding.EncodeToString(expectedATH[:]) {
		panic("shared incoming ath")
	}
	incomingJWK := must(json.Marshal(header["jwk"]))
	wantJWK := must(json.Marshal(p.auth.jwk))
	if !bytes.Equal(incomingJWK, wantJWK) {
		panic("shared auth JWK did not match borrowed test key")
	}
	outer := map[string]string{}
	check(json.Unmarshal(body, &outer))
	srt := outer["signedRequestToken"]
	_, srtClaims := p.auth.verify(srt)
	rb, ok := srtClaims["requestBody"].(string)
	if !ok {
		panic("shared modern SRT requestBody is not JSON string")
	}
	var request struct {
		Requests []struct {
			Policy           struct{ ID, Body string }
			KeyAccessObjects []struct {
				KeyAccessObjectID string
				KeyAccessObject   json.RawMessage
			}
		}
		ClientPublicKey string
	}
	check(json.Unmarshal([]byte(rb), &request))
	if len(request.Requests) != 1 || len(request.Requests[0].KeyAccessObjects) != 1 || request.Requests[0].Policy.ID != "policy" || request.Requests[0].KeyAccessObjects[0].KeyAccessObjectID != "kao-0" || request.ClientPublicKey == "" {
		panic("shared grouped modern SRT")
	}
	p.mu.Lock()
	mode := p.mode
	nonce := p.nonce
	p.mu.Unlock()
	// Separately prove destination DPoP/token/nonce acceptance before each mutation.
	preflight := platform + "/policy.attributes.AttributesService/ListAttributes"
	requestKey := []byte(`{}`)
	status, headers, _, e := wireRequest(r.Context(), preflight, auth, p.auth.proof(auth, preflight, nonce, nil), requestKey)
	if e != nil {
		http.Error(w, "fixture transport", 502)
		return
	}
	if status == 401 && headers.Get("DPoP-Nonce") != "" {
		nonce = headers.Get("DPoP-Nonce")
		status, headers, _, e = wireRequest(r.Context(), preflight, auth, p.auth.proof(auth, preflight, nonce, nil), requestKey)
	}
	if e != nil || status != 200 {
		panic("fixture destination DPoP/nonce preflight rejected")
	}
	if headers.Get("DPoP-Nonce") != "" {
		nonce = headers.Get("DPoP-Nonce")
	}
	p.mu.Lock()
	p.nonce = nonce
	p.mu.Unlock()
	switch mode {
	case "srt-wrong-authkey":
		srt = p.other.sign(map[string]any{"alg": p.other.algorithm, "typ": "JWT"}, srtClaims)
	case "srt-invalid-signature":
		srt = corruptSignature(srt)
	case "srt-expired":
		srtClaims["iat"] = time.Now().Add(-2 * time.Hour).Unix()
		srtClaims["exp"] = time.Now().Add(-time.Hour).Unix()
		srt = p.auth.sign(map[string]any{"alg": p.auth.algorithm, "typ": "JWT"}, srtClaims)
	case "srt-future-iat":
		srtClaims["iat"] = time.Now().Add(2 * time.Hour).Unix()
		srtClaims["exp"] = time.Now().Add(3 * time.Hour).Unix()
		srt = p.auth.sign(map[string]any{"alg": p.auth.algorithm, "typ": "JWT"}, srtClaims)
	case "srt-body-tamper":
		parts := strings.Split(srt, ".")
		srtClaims["requestBody"] = rb + " "
		parts[1] = base64.RawURLEncoding.EncodeToString(must(json.Marshal(srtClaims)))
		srt = strings.Join(parts, ".")
	case "srt-missing-body":
		delete(srtClaims, "requestBody")
		srt = p.auth.sign(map[string]any{"alg": p.auth.algorithm, "typ": "JWT"}, srtClaims)
	case "srt-object-body":
		srtClaims["requestBody"] = map[string]any{"requests": []any{}}
		srt = p.auth.sign(map[string]any{"alg": p.auth.algorithm, "typ": "JWT"}, srtClaims)
	}
	outer["signedRequestToken"] = srt
	body = must(json.Marshal(outer))
	target := platform + r.URL.Path
	proof := p.auth.proof(auth, target, nonce, nil)
	switch mode {
	case "proof-invalid-signature":
		proof = corruptSignature(proof)
	case "proof-wrong-key":
		proof = p.other.proof(auth, target, nonce, nil)
	case "proof-wrong-htm":
		proof = p.auth.proof(auth, target, nonce, func(c map[string]any) { c["htm"] = "GET" })
	case "proof-wrong-htu":
		proof = p.auth.proof(auth, target, nonce, func(c map[string]any) { c["htu"] = platform + "/wrong" })
	case "proof-path-only-htu":
		proof = p.auth.proof(auth, target, nonce, func(c map[string]any) { c["htu"] = r.URL.Path })
	case "proof-wrong-ath":
		proof = p.auth.proof(auth, target, nonce, func(c map[string]any) { c["ath"] = "wrong" })
	case "proof-missing-ath":
		proof = p.auth.proof(auth, target, nonce, func(c map[string]any) { delete(c, "ath") })
	case "proof-expired":
		proof = p.auth.proof(auth, target, nonce, func(c map[string]any) { c["iat"] = time.Now().Add(-2 * time.Hour).Unix() })
	}
	status, headers, data, e := wireRequest(r.Context(), target, auth, proof, body)
	if e != nil {
		http.Error(w, "fixture transport", 502)
		return
	}
	if mode == "valid" {
		if status != 200 {
			v := map[string]any{}
			_ = json.Unmarshal(data, &v)
			panic(fmt.Sprintf("unmodified shared modern SRT baseline rejected: status=%d challenge=%q code=%v message=%v", status, headers.Get("WWW-Authenticate"), v["code"], v["message"]))
		}
	} else if strings.HasPrefix(mode, "srt-") {
		expected := 401
		if mode == "srt-missing-body" || mode == "srt-object-body" {
			expected = 400
		}
		if status != expected || headers.Get("WWW-Authenticate") != "" {
			panic(fmt.Sprintf("%s status %d challenge %q; expected actual SRT error", mode, status, headers.Get("WWW-Authenticate")))
		}
		v := map[string]any{}
		check(json.Unmarshal(data, &v))
		message, _ := v["message"].(string)
		if !strings.Contains(message, "request") {
			panic("actual SRT failure missing request-token/body diagnostic")
		}
	} else if strings.HasPrefix(mode, "proof-") {
		if status != 401 || !strings.Contains(headers.Get("WWW-Authenticate"), "invalid_dpop_proof") {
			panic("actual DPoP negative not rejected by auth middleware")
		}
	}
	p.mu.Lock()
	p.evidence = append(p.evidence, map[string]any{"case": mode, "platform_status": status, "destination_auth_nonce_preflight": 200, "grouped_srt": true, "auth_algorithm": p.auth.algorithm})
	p.mu.Unlock()
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_, _ = w.Write(data)
}
func protocolNegatives(wrapping, session, auth string, data []byte, negatives *[]string) {
	a := newFixtureAuth(auth)
	defer a.handle.Close()
	other := newFixtureAuth(auth)
	defer other.handle.Close()
	proxy := &protocolProxy{auth: a, other: other, mode: "valid"}
	server := httptest.NewServer(http.HandlerFunc(proxy.forward))
	defer server.Close()
	proxy.url = server.URL
	c := must(shared.New(shared.Config{PlatformURL: platform, KASURL: kas, AllowedKAS: []shared.KASRoute{{URL: kas, APIBaseURL: server.URL}}, IssuerURL: issuer, ClientID: "opentdf-sdk", ClientSecret: "secret", AllowHTTP: true, KASAlgorithm: wrapping, SessionAlgorithm: session, AuthAlgorithm: auth, AuthKey: a.handle, DPoP: true}))
	defer c.Close()
	compare(c, data, []byte("negative"), []byte(`{"negative":true}`), "labeled mutation proxy valid grouped SRT baseline")
	modes := []string{"srt-wrong-authkey", "srt-invalid-signature", "srt-expired", "srt-future-iat", "srt-body-tamper", "srt-missing-body", "srt-object-body", "proof-invalid-signature", "proof-wrong-key", "proof-wrong-htm", "proof-wrong-htu", "proof-path-only-htu", "proof-wrong-ath", "proof-missing-ath", "proof-expired"}
	for _, mode := range modes {
		proxy.mu.Lock()
		proxy.mode = mode
		proxy.mu.Unlock()
		expected := "unauthenticated"
		if mode == "srt-missing-body" || mode == "srt-object-body" {
			expected = "http_status"
		}
		zero(c, data, wrapping+"/"+session+"/"+auth+"/"+mode, expected)
		*negatives = append(*negatives, wrapping+"/"+auth+"/"+session+"/"+mode)
	}
	name := strings.ReplaceAll(wrapping+"-"+session+"-"+auth, ":", "-")
	check(os.WriteFile(filepath.Join(run, "protocol-"+name+".json"), must(json.MarshalIndent(map[string]any{"scope": "labeled native mutation proxy: shared modern grouped SRT preserved/mutated; destination proof signed by same bound fixture key; real enforced platform rejects; no key recovery", "cases": proxy.evidence}, "", "  ")), 0600))
	invalid := must(shared.New(shared.Config{PlatformURL: platform, KASURL: kas, AllowHTTP: true, KASAlgorithm: wrapping, SessionAlgorithm: session, AuthAlgorithm: auth, AuthKey: a.handle, DPoP: true, TokenProvider: func(context.Context) (shared.AccessToken, error) {
		return shared.AccessToken{Value: "invalid-opaque-token", Scheme: "DPoP", ExpiresAt: time.Now().Unix() + 300, ConfirmationJKT: a.thumb}, nil
	}}))
	zero(invalid, data, "invalid opaque provider", "unauthenticated")
	invalid.Close()
	*negatives = append(*negatives, wrapping+"/"+auth+"/"+session+"/invalid-opaque-token")
	unbound := must(shared.New(shared.Config{PlatformURL: platform, KASURL: kas, AllowHTTP: true, KASAlgorithm: wrapping, SessionAlgorithm: session, AuthAlgorithm: auth, AuthKey: a.handle, DPoP: true, TokenProvider: func(context.Context) (shared.AccessToken, error) {
		return shared.AccessToken{Value: a.sign(map[string]any{"alg": auth, "typ": "JWT"}, map[string]any{"cnf": map[string]string{"jkt": "wrong"}}), Scheme: "DPoP", ExpiresAt: time.Now().Unix() + 300, ConfirmationJKT: a.thumb}, nil
	}}))
	zero(unbound, data, "mismatched JWT binding", "token_binding")
	unbound.Close()
	*negatives = append(*negatives, wrapping+"/"+auth+"/"+session+"/jwt-binding-mismatch")
}

// Rewrite ZIP CRCs after bounded manifest mutations: failure must come from the
// actual client's profile/integrity checks rather than the ZIP checksum parser.
func manifestTamperNegatives(c *shared.Client, data []byte, prefix string, negatives *[]string) {
	for _, mode := range []string{"root-signature", "segment-hash", "encrypted-metadata"} {
		a := must(tdf.ReadArchive(data, tdf.DefaultArchiveLimits()))
		m := map[string]any{}
		check(json.Unmarshal(a.Manifest, &m))
		encryption := m["encryptionInformation"].(map[string]any)
		integrity := encryption["integrityInformation"].(map[string]any)
		flip := func(s string) string {
			b := must(base64.StdEncoding.DecodeString(s))
			b[len(b)-1] ^= 1
			return base64.StdEncoding.EncodeToString(b)
		}
		switch mode {
		case "root-signature":
			root := integrity["rootSignature"].(map[string]any)
			root["sig"] = flip(root["sig"].(string))
		case "segment-hash":
			segments := integrity["segments"].([]any)
			segment := segments[0].(map[string]any)
			segment["hash"] = flip(segment["hash"].(string))
		case "encrypted-metadata":
			kaos := encryption["keyAccess"].([]any)
			kao := kaos[0].(map[string]any)
			encoded := must(base64.StdEncoding.DecodeString(kao["encryptedMetadata"].(string)))
			metadata := map[string]string{}
			check(json.Unmarshal(encoded, &metadata))
			metadata["ciphertext"] = flip(metadata["ciphertext"])
			kao["encryptedMetadata"] = base64.StdEncoding.EncodeToString(must(json.Marshal(metadata)))
		}
		archive := must(tdf.WriteArchive(a.Payload, must(json.Marshal(m)), tdf.DefaultArchiveLimits()))
		zero(c, archive, prefix+"/"+mode+"-tamper", "integrity")
		*negatives = append(*negatives, prefix+"/"+mode+"-tamper")
	}
}
