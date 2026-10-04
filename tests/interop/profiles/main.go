// Reference-only live profile runner. Secrets and proofs remain in memory or ignored .local storage.
package main

import (
	"bytes"
	"context"
	"crypto"
	"crypto/aes"
	"crypto/cipher"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"encoding/pem"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"net/url"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	capcrypto "github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/lestrrat-go/jwx/v2/jwa"
	"github.com/lestrrat-go/jwx/v2/jwk"
	"github.com/lestrrat-go/jwx/v2/jws"
	"github.com/lestrrat-go/jwx/v2/jwt"
	"github.com/opentdf/platform/lib/ocrypto"
	reference "github.com/opentdf/platform/sdk"
	"github.com/opentdf/platform/sdk/auth/oauth"
	"opentdf-local/sdk/src/tdf"
)

const platform = "http://localhost:8080"
const kas = platform + "/kas"
const issuer = "http://localhost:8888/auth/realms/opentdf"
const tokenURL = issuer + "/protocol/openid-connect/token"
const allowed = "https://example.com/attr/attr1/value/value1"
const denied = "https://example.com/attr/attr1/value/value2"

var root string
var client = &http.Client{Timeout: 15 * time.Second}

func must[T any](v T, e error) T {
	if e != nil {
		panic(e)
	}
	return v
}
func check(e error) {
	if e != nil {
		panic(e)
	}
}
func write(name string, data []byte) {
	check(os.WriteFile(filepath.Join(root, ".local/profiles", name), data, 0600))
}
func pins() {
	lock := struct {
		Repositories map[string]struct{ Path, Revision string }
	}{}
	check(json.Unmarshal(must(os.ReadFile(filepath.Join(root, "references.lock.json"))), &lock))
	for _, name := range []string{"platform", "web-sdk"} {
		p := lock.Repositories[name]
		ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
		actual := must(exec.CommandContext(ctx, "git", "-C", filepath.Join(root, p.Path), "rev-parse", "HEAD").Output())
		cancel()
		if strings.TrimSpace(string(actual)) != p.Revision {
			panic("reference pin mismatch: " + name)
		}
		ctx, cancel = context.WithTimeout(context.Background(), 10*time.Second)
		changed := must(exec.CommandContext(ctx, "git", "-C", filepath.Join(root, p.Path), "status", "--porcelain", "--untracked-files=no").Output())
		cancel()
		if len(changed) != 0 {
			panic("tracked reference changes: " + name)
		}
	}
}
func provision(profile string) {
	dir := filepath.Join(root, ".local/profiles")
	check(os.MkdirAll(dir, 0700))
	kekPath := filepath.Join(dir, "root.hex")
	var kek []byte
	if b, e := os.ReadFile(kekPath); os.IsNotExist(e) {
		kek = make([]byte, 32)
		_, e = rand.Read(kek)
		check(e)
		write("root.hex", []byte(hex.EncodeToString(kek)))
	} else {
		check(e)
		kek = must(hex.DecodeString(string(b)))
	}
	ecPath := filepath.Join(dir, "ec-private.pem")
	var ecPEM []byte
	if b, e := os.ReadFile(ecPath); os.IsNotExist(e) {
		k := must(ecdsa.GenerateKey(elliptic.P256(), rand.Reader))
		ecPEM = pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: must(x509.MarshalPKCS8PrivateKey(k))})
		write("ec-private.pem", ecPEM)
	} else {
		check(e)
		ecPEM = b
	}
	config := string(must(os.ReadFile(filepath.Join(root, "dev/opentdf.yaml"))))
	config = strings.Replace(config, "registered_kas_uri: http://localhost:8080", "registered_kas_uri: http://localhost:8080/kas", 1)
	config = strings.Replace(config, "key_management: false", "key_management: true\n    root_key: "+hex.EncodeToString(kek), 1)
	config = strings.Replace(config, "ec_tdf_enabled: false", "ec_tdf_enabled: true", 1)
	if profile == "dpop" {
		config = strings.Replace(config, "enforce: false", "enforce: true\n      require_nonce: true\n      strict_htu: true\n      nonce_expiration: 5m", 1)
	}
	write("effective.yaml", []byte(config))
	// Wrap private PEM with nonce12 || AES-256-GCM ciphertext/tag, matching pinned BasicManager.
	g := must(cipher.NewGCM(must(aes.NewCipher(kek))))
	sql := "BEGIN;\nSET LOCAL search_path TO opentdf_policy, public;\n"
	for _, k := range []struct {
		kid, id string
		alg     int
		priv    []byte
	}{{"profile-r1", "e3b8fa9a-164a-47ae-a87f-b18b609f9fb1", 1, must(os.ReadFile(filepath.Join(root, ".local/keys/kas-private.pem")))}, {"profile-e1", "e3b8fa9a-164a-47ae-a87f-b18b609f9fe1", 3, ecPEM}} {
		block, _ := pem.Decode(k.priv)
		if block == nil {
			panic("private PEM invalid")
		}
		var pk crypto.PublicKey
		if p, e := x509.ParsePKCS8PrivateKey(block.Bytes); e == nil {
			pk = p.(crypto.Signer).Public()
		} else if p, e := x509.ParsePKCS1PrivateKey(block.Bytes); e == nil {
			pk = &p.PublicKey
		} else {
			panic("unsupported private PEM")
		}
		pub := pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: must(x509.MarshalPKIXPublicKey(pk))})
		nonce := make([]byte, g.NonceSize())
		_, e := rand.Read(nonce)
		check(e)
		wrapped := g.Seal(nonce, nonce, k.priv, nil)
		pubCtx := must(json.Marshal(map[string]string{"pem": base64.StdEncoding.EncodeToString(pub)}))
		privCtx := must(json.Marshal(map[string]string{"key_id": "profile-root", "wrapped_key": base64.StdEncoding.EncodeToString(wrapped)}))
		sql += fmt.Sprintf("INSERT INTO opentdf_policy.key_access_server_keys (id,key_access_server_id,key_id,key_algorithm,key_status,key_mode,public_key_ctx,private_key_ctx,legacy) VALUES ('%s','34f2acdc-3d9c-4e92-80b6-90fe4dc9afcb','%s',%d,1,1,'%s'::jsonb,'%s'::jsonb,false);\n", k.id, k.kid, k.alg, pubCtx, privCtx)
	}
	sql += "INSERT INTO opentdf_policy.base_keys (key_access_server_key_id) VALUES ('e3b8fa9a-164a-47ae-a87f-b18b609f9fe1');\n"
	// Grant both fixture values, retaining the PDP's distinction between entitled and denied subjects.
	sql += "INSERT INTO opentdf_policy.attribute_value_public_key_map (value_id,key_access_server_key_id) SELECT v.id,'e3b8fa9a-164a-47ae-a87f-b18b609f9fe1' FROM opentdf_policy.attribute_values v JOIN opentdf_policy.attribute_definitions d ON d.id=v.attribute_definition_id WHERE d.name='attr1' AND v.value IN ('value1','value2');\nCOMMIT;\n"
	write("registry.sql", []byte(sql))
	fmt.Println("PASS generated isolated profile configuration and wrapped-key registry SQL")
}
func authKey() jwk.Key {
	k := must(jwk.FromRaw(must(ecdsa.GenerateKey(elliptic.P256(), rand.Reader))))
	check(k.Set(jwk.AlgorithmKey, jwa.ES256))
	return k
}
func request(path, token, scheme, proof string, body []byte) (int, http.Header, []byte) {
	r := must(http.NewRequest("POST", path, bytes.NewReader(body)))
	r.Header.Set("Content-Type", "application/json")
	r.Header.Set("Connect-Protocol-Version", "1")
	if token != "" {
		r.Header.Set("Authorization", scheme+" "+token)
	}
	if proof != "" {
		r.Header.Set("DPoP", proof)
	}
	res := must(client.Do(r))
	defer res.Body.Close()
	return res.StatusCode, res.Header, must(io.ReadAll(io.LimitReader(res.Body, 2<<20)))
}
func bearer() string {
	r := must(http.NewRequest("POST", tokenURL, strings.NewReader(url.Values{"grant_type": {"client_credentials"}, "client_id": {"opentdf-sdk"}, "client_secret": {"secret"}}.Encode())))
	r.Header.Set("Content-Type", "application/x-www-form-urlencoded")
	res := must(client.Do(r))
	defer res.Body.Close()
	if res.StatusCode != 200 {
		panic("Bearer token request failed")
	}
	v := struct {
		AccessToken string `json:"access_token"`
	}{}
	check(json.NewDecoder(res.Body).Decode(&v))
	return v.AccessToken
}
func proof(k jwk.Key, token, path, nonce string, edit func(map[string]interface{})) string {
	h := sha256.Sum256([]byte(token))
	v := map[string]interface{}{"jti": hex.EncodeToString(mustRandom(16)), "iat": time.Now().Unix(), "htm": "POST", "htu": path, "ath": base64.RawURLEncoding.EncodeToString(h[:])}
	if nonce != "" {
		v["nonce"] = nonce
	}
	if edit != nil {
		edit(v)
	}
	t := jwt.New()
	for n, x := range v {
		check(t.Set(n, x))
	}
	hdr := jws.NewHeaders()
	check(hdr.Set("typ", "dpop+jwt"))
	check(hdr.Set("jwk", must(jwk.PublicKeyOf(k))))
	return string(must(jwt.Sign(t, jwt.WithKey(jwa.ES256, k, jws.WithProtectedHeaders(hdr)))))
}
func mustRandom(n int) []byte { b := make([]byte, n); _, e := rand.Read(b); check(e); return b }
func active() string {
	profile := strings.TrimSpace(string(must(os.ReadFile(filepath.Join(root, ".local/profiles/active")))))
	if profile != "ec" && profile != "dpop" {
		panic("select verified ec or dpop profile before running secure helper")
	}
	return profile
}
func token(k jwk.Key) string {
	t := must(oauth.GetAccessToken(client, tokenURL, nil, oauth.ClientCredentials{ClientID: "opentdf-sdk", ClientAuth: "secret"}, k))
	if !strings.EqualFold(t.TokenType, "DPoP") {
		panic("expected DPoP bound token")
	}
	claims := must(jwt.Parse([]byte(t.AccessToken), jwt.WithVerify(false), jwt.WithValidate(false)))
	cnf, ok := claims.Get("cnf")
	if !ok {
		panic("missing cnf")
	}
	j := must(json.Marshal(cnf))
	v := map[string]string{}
	check(json.Unmarshal(j, &v))
	thumb := must(k.Thumbprint(crypto.SHA256))
	if v["jkt"] != base64.RawURLEncoding.EncodeToString(thumb) {
		panic("token thumbprint mismatch")
	}
	return t.AccessToken
}
func liveCheck() {
	profile := ""
	if len(os.Args) > 2 {
		profile = os.Args[2]
	} else {
		profile = active()
	}
	os.Remove(filepath.Join(root, ".local/profiles", "check-"+profile+".json"))
	discoveryResponse := must(client.Get(platform + "/.well-known/opentdf-configuration"))
	defer discoveryResponse.Body.Close()
	if discoveryResponse.StatusCode != 200 {
		panic("platform discovery failed")
	}
	discovery := struct {
		BaseKey struct {
			KasURI    string                               `json:"kas_uri"`
			PublicKey struct{ Algorithm, Kid, PEM string } `json:"public_key"`
		} `json:"base_key"`
		NonceRequired  bool   `json:"dpop_nonce_required"`
		SupportsDPoP   bool   `json:"supports_dpop"`
		PlatformIssuer string `json:"platform_issuer"`
	}{}
	check(json.NewDecoder(discoveryResponse.Body).Decode(&discovery))
	if discovery.BaseKey.KasURI != kas || discovery.BaseKey.PublicKey.Kid != "profile-e1" || discovery.BaseKey.PublicKey.Algorithm != "ec:secp256r1" || discovery.NonceRequired != (profile == "dpop") || !discovery.SupportsDPoP || discovery.PlatformIssuer != issuer {
		panic("unexpected live discovery profile")
	}
	config := string(must(os.ReadFile(filepath.Join(root, ".local/profiles/effective.yaml"))))
	if !strings.Contains(config, "key_management: true") || !strings.Contains(config, "ec_tdf_enabled: true") {
		panic("effective config missing EC key management")
	}
	report := map[string]interface{}{"profile": profile, "key_management": true, "ec_tdf_enabled": true, "rsa_kid": "profile-r1", "ec_kid": "profile-e1", "base_key": discovery.BaseKey.PublicKey.Kid, "native_nonce_enforced": profile == "dpop", "discovery_verified": true, "key_management_evidence": "effective config; live BasicManager EC rewrap is verified separately by smoke"}
	raw := bearer()
	k := authKey()
	bound := token(k)
	nonce := ""
	if profile == "ec" {
		path := platform + "/policy.attributes.AttributesService/ListAttributes"
		status, _, _ := request(path, "", "", "", []byte(`{}`))
		if status != 401 {
			panic("unauthenticated EC policy request accepted")
		}
		report["unauthenticated"] = status
		status, _, _ = request(path, raw, "Bearer", "", []byte(`{}`))
		if status != 200 {
			panic("ordinary Bearer EC policy request rejected")
		}
		report["ordinary_bearer"] = status
	}
	if profile == "dpop" {
		path := platform + "/policy.attributes.AttributesService/ListAttributes"
		body := []byte(`{}`)
		expect := func(label string, code int, proofValue, tk, scheme string) (http.Header, []byte) {
			s, h, b := request(path, tk, scheme, proofValue, body)
			if s != code {
				panic(fmt.Sprintf("%s expected %d got %d", label, code, s))
			}
			report[label] = s
			return h, b
		}
		expect("ordinary_bearer", 401, "", raw, "Bearer")
		expect("missing_proof", 401, "", bound, "DPoP")
		h, _ := expect("initial_nonce_challenge", 401, proof(k, bound, path, "", nil), bound, "DPoP")
		nonce = h.Get("DPoP-Nonce")
		if nonce == "" || !strings.Contains(h.Get("WWW-Authenticate"), "use_dpop_nonce") {
			panic("no native nonce challenge")
		}
		p := proof(k, bound, path, nonce, nil)
		expect("valid_proof", 200, p, bound, "DPoP")
		expect("proof_replay", 401, p, bound, "DPoP")
		expect("nonce_reuse_fresh_jti", 200, proof(k, bound, path, nonce, nil), bound, "DPoP")
		expect("wrong_nonce", 401, proof(k, bound, path, "invalid-nonce", nil), bound, "DPoP")
		expect("wrong_key", 401, proof(authKey(), bound, path, nonce, nil), bound, "DPoP")
		expect("invalid_proof_signature", 401, corruptSignature(proof(k, bound, path, nonce, nil)), bound, "DPoP")
		expect("incorrect_htm", 401, proof(k, bound, path, nonce, func(v map[string]interface{}) { v["htm"] = "GET" }), bound, "DPoP")
		expect("missing_ath", 401, proof(k, bound, path, nonce, func(v map[string]interface{}) { delete(v, "ath") }), bound, "DPoP")
		expect("expired", 401, proof(k, bound, path, nonce, func(v map[string]interface{}) { v["iat"] = time.Now().Add(-2 * time.Hour).Unix() }), bound, "DPoP")
		expect("incorrect_ath", 401, proof(k, bound, path, nonce, func(v map[string]interface{}) { v["ath"] = "incorrect" }), bound, "DPoP")
		expect("incorrect_htu", 401, proof(k, bound, path, nonce, func(v map[string]interface{}) { v["htu"] = platform + "/wrong" }), bound, "DPoP")
		expect("path_only_htu", 401, proof(k, bound, path, nonce, func(v map[string]interface{}) { v["htu"] = "/policy.attributes.AttributesService/ListAttributes" }), bound, "DPoP")
		expect("bound_bearer_with_valid_proof", 200, proof(k, bound, path, nonce, nil), bound, "Bearer")
	}
	for _, a := range []struct{ alg, kid string }{{"rsa:2048", "profile-r1"}, {"ec:secp256r1", "profile-e1"}} {
		path := platform + "/kas.AccessService/PublicKey"
		body := must(json.Marshal(map[string]string{"algorithm": a.alg, "fmt": "pkcs8", "v": "2"}))
		p := ""
		tk := raw
		scheme := "Bearer"
		if profile == "dpop" {
			tk = bound
			scheme = "DPoP"
			p = proof(k, bound, path, nonce, nil)
		}
		status, _, b := request(path, tk, scheme, p, body)
		if status != 200 {
			panic(fmt.Sprintf("public key %s failed: status %d", a.alg, status))
		}
		v := struct{ Kid, PublicKey string }{}
		check(json.Unmarshal(b, &v))
		if v.Kid != a.kid {
			panic("wrong registry kid")
		}
		block, _ := pem.Decode([]byte(v.PublicKey))
		if block == nil {
			panic("invalid SPKI")
		}
		pub := must(x509.ParsePKIXPublicKey(block.Bytes))
		if a.kid == "profile-r1" {
			r, ok := pub.(*rsa.PublicKey)
			if !ok || r.N.BitLen() != 2048 {
				panic("not RSA2048")
			}
		}
		if a.kid == "profile-e1" {
			p, ok := pub.(*ecdsa.PublicKey)
			if !ok || p.Curve != elliptic.P256() {
				panic("not P256")
			}
		}
	}
	write("check-"+profile+".json", append(must(json.MarshalIndent(report, "", "  ")), '\n'))
	fmt.Println("PASS live", profile, "key IDs, P256 and authentication/nonce matrix")
}
func smoke() {
	profile := active()
	os.Remove(filepath.Join(root, ".local/profiles", "smoke-"+profile+".json"))
	os.Remove(filepath.Join(root, ".local/profiles", "cli-"+profile+".json"))
	if profile == "dpop" {
		os.Remove(filepath.Join(root, ".local/profiles/srt-dpop.json"))
	}
	s := must(reference.New(platform, reference.WithClientCredentials("opentdf-sdk", "secret", nil), reference.WithInsecurePlaintextConn(), reference.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil)))))
	defer s.Close()
	s.Conn().Client.Timeout = 15 * time.Second
	payload := append(bytes.Repeat([]byte{0, 255, 127, 19}, 9000), []byte("profile plaintext\n")...)
	cases := []map[string]interface{}{}
	// Default/granted registry selection must actually emit EC, not just report EC discovery.
	for _, attr := range []string{allowed, denied} {
		var out bytes.Buffer
		_, e := s.CreateTDF(&out, bytes.NewReader(payload), reference.WithDataAttributes(attr))
		check(e)
		ar := must(tdf.ReadArchive(out.Bytes(), tdf.DefaultArchiveLimits()))
		m := must(tdf.ParseManifest(ar.Manifest))
		if len(m.Encryption.KeyAccess) != 1 || m.Encryption.KeyAccess[0].Type != "ec-wrapped" || m.Encryption.KeyAccess[0].KID != "profile-e1" {
			panic("default registry did not select EC")
		}
		write(map[bool]string{true: "denied", false: "allowed"}[attr == denied]+"-"+profile+".tdf", out.Bytes())
		for _, session := range []ocrypto.KeyType{ocrypto.EC256Key, ocrypto.RSA2048Key} {
			reader := must(s.LoadTDF(bytes.NewReader(out.Bytes()), reference.WithKasAllowlist([]string{kas}), reference.WithSessionKeyType(session)))
			plain, err := io.ReadAll(reader)
			if attr == denied {
				if err == nil || len(plain) != 0 || !strings.Contains(strings.ToLower(err.Error()), "permission_denied") {
					panic("expected real policy denial")
				}
			} else {
				check(err)
				if !bytes.Equal(plain, payload) {
					panic("plaintext mismatch")
				}
			}
			cases = append(cases, map[string]interface{}{"producer": "pinned-go-default-registry", "kid": "profile-e1", "wrapping": "ec:secp256r1", "session": session, "denied": attr == denied, "bytes": len(plain)})
		}
	}
	// New engine's independently constructed EC KAO crosses real KAS using pinned reader sessions.
	req := must(http.NewRequest("POST", platform+"/kas.AccessService/PublicKey", strings.NewReader(`{"algorithm":"ec:secp256r1","fmt":"pkcs8","v":"2"}`)))
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Connect-Protocol-Version", "1")
	response := must(s.Conn().Client.Do(req))
	defer response.Body.Close()
	if response.StatusCode != 200 {
		panic("reference authenticated EC public key failed")
	}
	pub := reference.KASInfo{}
	check(json.NewDecoder(response.Body).Decode(&pub))
	// JSON uses kid, whereas KASInfo's exported field is KID; both match case insensitively.
	key := must(capcrypto.ImportPEM(pub.PublicKey))
	defer key.Close()
	data := must(tdf.Encrypt(payload, tdf.EncryptConfig{KASPublicKey: key, KASURL: kas, KID: pub.KID, Algorithm: "ec:secp256r1", Attributes: []string{allowed}}))
	for _, session := range []ocrypto.KeyType{ocrypto.EC256Key, ocrypto.RSA2048Key} {
		r := must(s.LoadTDF(bytes.NewReader(data), reference.WithKasAllowlist([]string{kas}), reference.WithSessionKeyType(session)))
		plain := must(io.ReadAll(r))
		if !bytes.Equal(plain, payload) {
			panic("engine plaintext mismatch")
		}
		cases = append(cases, map[string]interface{}{"producer": "new-engine", "wrapping": "ec:secp256r1", "session": session, "bytes": len(plain)})
	}
	if profile == "dpop" {
		srtCheck()
	}
	cliCheck(profile, payload)
	write("smoke-"+profile+".json", append(must(json.MarshalIndent(cases, "", "  ")), '\n'))
	fmt.Println("PASS live", profile, "EC KAO, EC/RSA sessions, exact binary plaintext, PDP denial")
}
func corruptSignature(s string) string {
	parts := strings.Split(s, ".")
	sig := must(base64.RawURLEncoding.DecodeString(parts[2]))
	sig[0] ^= 1
	parts[2] = base64.RawURLEncoding.EncodeToString(sig)
	return strings.Join(parts, ".")
}
func srtCheck() {
	k := authKey()
	tk := token(k)
	policyPath := platform + "/policy.attributes.AttributesService/ListAttributes"
	code, h, _ := request(policyPath, tk, "DPoP", proof(k, tk, policyPath, "", nil), []byte(`{}`))
	if code != 401 || h.Get("DPoP-Nonce") == "" {
		panic("SRT probe missing nonce challenge")
	}
	nonce := h.Get("DPoP-Nonce")
	data := must(os.ReadFile(filepath.Join(root, ".local/profiles/allowed-dpop.tdf")))
	a := must(tdf.ReadArchive(data, tdf.DefaultArchiveLimits()))
	m := struct {
		Encryption struct {
			Policy    string
			KeyAccess []json.RawMessage
		} `json:"encryptionInformation"`
	}{}
	check(json.Unmarshal(a.Manifest, &m))
	session := must(ecdsa.GenerateKey(elliptic.P256(), rand.Reader))
	pub := pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: must(x509.MarshalPKIXPublicKey(&session.PublicKey))})
	rb := must(json.Marshal(map[string]interface{}{"policy": m.Encryption.Policy, "keyAccess": m.Encryption.KeyAccess[0], "clientPublicKey": string(pub), "algorithm": "ec:secp256r1"}))
	srt := func(key jwk.Key, edit func(map[string]interface{})) string {
		v := map[string]interface{}{"requestBody": string(rb), "iat": time.Now().Unix(), "exp": time.Now().Add(time.Minute).Unix()}
		if edit != nil {
			edit(v)
		}
		t := jwt.New()
		for n, x := range v {
			check(t.Set(n, x))
		}
		return string(must(jwt.Sign(t, jwt.WithKey(jwa.ES256, key))))
	}
	path := platform + "/kas.AccessService/Rewrap"
	report := map[string]int{}
	for _, c := range []struct {
		name   string
		srt    string
		status int
	}{{"valid_srt", srt(k, nil), 200}, {"srt_wrong_auth_key", srt(authKey(), nil), 401}, {"srt_invalid_signature", corruptSignature(srt(k, nil)), 401}, {"srt_expired", srt(k, func(v map[string]interface{}) {
		v["iat"] = time.Now().Add(-2 * time.Hour).Unix()
		v["exp"] = time.Now().Add(-2 * time.Hour).Unix()
	}), 401}} {
		body := must(json.Marshal(map[string]string{"signedRequestToken": c.srt}))
		status, _, _ := request(path, tk, "DPoP", proof(k, tk, path, nonce, nil), body)
		if status != c.status {
			panic(fmt.Sprintf("%s expected %d got %d", c.name, c.status, status))
		}
		report[c.name] = status
	}
	write("srt-dpop.json", append(must(json.MarshalIndent(report, "", "  ")), '\n'))
	fmt.Println("PASS real KAS SRT auth-key binding, signature and expiry checks")
}
func cliCheck(profile string, payload []byte) {
	dir := filepath.Join(root, ".local/profiles")
	write("client-creds.json", []byte(`{"clientId":"opentdf-sdk","clientSecret":"secret"}`))
	report := []map[string]interface{}{}
	for _, consumer := range []string{"go", "web"} {
		for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
			output := filepath.Join(dir, consumer+"-"+profile+"-"+strings.ReplaceAll(session, ":", "_")+".out")
			if e := os.Remove(output); e != nil && !os.IsNotExist(e) {
				panic(e)
			}
			ctx, cancel := context.WithTimeout(context.Background(), 45*time.Second)
			var cmd *exec.Cmd
			if consumer == "go" {
				cmd = exec.CommandContext(ctx, filepath.Join(root, ".local/bin/otdfctl"), "decrypt", filepath.Join(dir, "allowed-"+profile+".tdf"), "--session-key-algorithm", session, "--kas-allowlist", kas, "--host", platform, "--with-client-creds-file", filepath.Join(dir, "client-creds.json"), "--out", output)
			} else {
				args := []string{filepath.Join(root, ".local/web-cli/bin/opentdf.mjs"), "decrypt", filepath.Join(dir, "allowed-"+profile+".tdf"), "--rewrapKeyType", session, "--allowList", platform, "--platformUrl", platform, "--kasEndpoint", kas, "--oidcEndpoint", issuer, "--clientId", "opentdf-sdk", "--clientSecret", "secret", "--logLevel", "error", "--output", output}
				if profile == "dpop" {
					args = append(args, "--dpop")
				}
				cmd = exec.CommandContext(ctx, "node", args...)
			}
			// Capture diagnostics only in memory; do not save proofs, tokens or decoded auth logs.
			diagnostics, err := cmd.CombinedOutput()
			timedOut := ctx.Err() != nil
			cancel()
			row := map[string]interface{}{"consumer": consumer, "session": session, "success": err == nil}
			if timedOut {
				panic("reference CLI timed out")
			}
			if consumer == "web" && profile == "dpop" {
				if err == nil {
					panic("stock web unexpectedly passed enforced nonce profile; update compatibility evidence")
				}
				text := strings.ToLower(string(diagnostics))
				if !strings.Contains(text, "unauthenticated") && !strings.Contains(text, "unauthorized") {
					panic("web failed for unexpected reason")
				}
				if b, e := os.ReadFile(output); e == nil && len(b) > 0 {
					panic("web auth failure returned plaintext")
				}
				row["limitation"] = "stock web DPoP auth rejected by native enforced nonce service"
				causes := []string{}
				for _, cause := range []string{"incorrect `ath`", "incorrect `htu`", "use_dpop_nonce", "nonce", "unauthenticated", "unauthorized", "401"} {
					if strings.Contains(text, cause) {
						causes = append(causes, cause)
					}
				}
				if len(causes) == 0 {
					panic("stock web failure has no auth markers")
				}
				row["diagnostic_markers"] = causes
			} else {
				if err != nil {
					panic(fmt.Sprintf("reference %s CLI %s failed (diagnostics suppressed)", consumer, session))
				}
				if !bytes.Equal(must(os.ReadFile(output)), payload) {
					panic("CLI plaintext mismatch")
				}
			}
			report = append(report, row)
		}
	}
	write("cli-"+profile+".json", append(must(json.MarshalIndent(report, "", "  ")), '\n'))
	fmt.Println("PASS reference CLI matrix; stock-web enforced DPoP limitation recorded when applicable")
}
func main() {
	root = must(filepath.Abs("../../.."))
	slog.SetDefault(slog.New(slog.NewTextHandler(io.Discard, nil)))
	pins()
	switch os.Args[1] {
	case "pins":
		fmt.Println("PASS pinned tracked-clean references")
	case "provision":
		provision(os.Args[2])
	case "check":
		liveCheck()
	case "smoke":
		smoke()
	default:
		panic("unknown command")
	}
}
