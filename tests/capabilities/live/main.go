// This is a native acceptance runner, not shared SDK source. stdlib JSON/ZIP
// reads the pinned reference policy fixture; operations under test use lib caps.
package main

import (
	"archive/zip"
	"bytes"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"github.com/eugenioenko/goalchemy/lib/clock"
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	caphttp "github.com/eugenioenko/goalchemy/lib/http"
	"io"
	"os"
)

func must[T any](v T, e error) T {
	if e != nil {
		panic(e)
	}
	return v
}
func jsonBytes(v any) []byte { return must(json.Marshal(v)) }
func request(method, url string, headers []string, body []byte) []byte {
	status, _, out, e := caphttp.Do(context.Background(), method, url, headers, body, 1<<20, 10000)
	if e != nil {
		panic(e)
	}
	if status != 200 {
		panic(fmt.Sprintf("HTTP %d: %s", status, out))
	}
	return out
}
func main() {
	discovery := map[string]any{}
	if e := json.Unmarshal(request("GET", "http://localhost:8888/auth/realms/opentdf/.well-known/openid-configuration", nil, nil), &discovery); e != nil {
		panic(e)
	}
	if discovery["issuer"] != "http://localhost:8888/auth/realms/opentdf" {
		panic("issuer")
	}
	// Workspace-local development fixture credentials, matching platform-check.py.
	tokenResponse := map[string]any{}
	if e := json.Unmarshal(request("POST", discovery["token_endpoint"].(string), []string{"Content-Type", "application/x-www-form-urlencoded"}, []byte("grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret")), &tokenResponse); e != nil {
		panic(e)
	}
	if tokenResponse["token_type"] != "Bearer" {
		panic("Bearer profile")
	}
	headers := []string{"Content-Type", "application/json", "Connect-Protocol-Version", "1", "Authorization", "Bearer " + tokenResponse["access_token"].(string)}
	publicResponse := struct {
		PublicKey string `json:"publicKey"`
		Kid       string `json:"kid"`
	}{}
	if e := json.Unmarshal(request("POST", "http://localhost:8080/kas.AccessService/PublicKey", headers, []byte(`{"algorithm":"rsa:2048"}`)), &publicResponse); e != nil {
		panic(e)
	}
	if publicResponse.Kid != "r1" {
		panic("KAS RSA key r1")
	}
	kasKey := must(crypto.ImportPEM(publicResponse.PublicKey))
	defer kasKey.Close()
	fixture := "../../.local/interop/small.go.tdf"
	if len(os.Args) > 1 {
		fixture = os.Args[1]
	}
	archive := must(zip.OpenReader(fixture))
	defer archive.Close()
	var policy string
	for _, entry := range archive.File {
		if entry.Name == "0.manifest.json" {
			r := must(entry.Open())
			data := must(io.ReadAll(io.LimitReader(r, 1<<20)))
			r.Close()
			m := struct {
				EncryptionInformation struct {
					Policy string `json:"policy"`
				} `json:"encryptionInformation"`
			}{}
			if e := json.Unmarshal(data, &m); e != nil {
				panic(e)
			}
			policy = m.EncryptionInformation.Policy
		}
	}
	if policy == "" {
		panic("allowed reference policy fixture missing")
	}
	share := must(crypto.Random(32))
	wrapped := must(crypto.RSAOAEPEncrypt(kasKey, share))
	binding := must(crypto.HMACSHA256(share, []byte(policy)))
	session := must(crypto.GenerateRSA2048())
	defer session.Close()
	auth := must(crypto.GenerateP256())
	defer auth.Close()
	sessionPEM := must(session.PublicPEM())
	kao := map[string]any{"type": "wrapped", "url": "http://localhost:8080/kas", "protocol": "kas", "kid": "r1", "wrappedKey": must(encoding.Base64Encode(wrapped)), "policyBinding": map[string]any{"alg": "HS256", "hash": must(encoding.Base64Encode([]byte(hex.EncodeToString(binding))))}}
	unsigned := map[string]any{"clientPublicKey": sessionPEM, "requests": []any{map[string]any{"policy": map[string]any{"id": "cap-policy", "body": policy}, "keyAccessObjects": []any{map[string]any{"keyAccessObjectId": "cap-kao", "keyAccessObject": kao}}}}}
	now := clock.Unix()
	claims := map[string]any{"requestBody": string(jsonBytes(unsigned)), "iat": now, "exp": now + 60}
	jwtHeader := map[string]any{"alg": "ES256", "typ": "JWT"}
	signingInput := must(encoding.Base64URLEncode(jsonBytes(jwtHeader))) + "." + must(encoding.Base64URLEncode(jsonBytes(claims)))
	sig := must(crypto.ES256Sign(auth, []byte(signingInput)))
	srt := signingInput + "." + must(encoding.Base64URLEncode(sig))
	// Independently verify signing output here even though Bearer KAS skips SRT
	// signature validation without an authenticated DPoP key.
	if ok, e := crypto.ES256Verify(auth, []byte(signingInput), sig); e != nil || !ok {
		panic("SRT signing")
	}
	response := struct {
		Responses []struct {
			PolicyID string `json:"policyId"`
			Results  []struct {
				ID     string `json:"keyAccessObjectId"`
				Status string `json:"status"`
				Key    string `json:"kasWrappedKey"`
				Error  string `json:"error"`
			} `json:"results"`
		} `json:"responses"`
	}{}
	data := request("POST", "http://localhost:8080/kas.AccessService/Rewrap", headers, jsonBytes(map[string]any{"signedRequestToken": srt}))
	if e := json.Unmarshal(data, &response); e != nil {
		panic(e)
	}
	if len(response.Responses) != 1 || response.Responses[0].PolicyID != "cap-policy" || len(response.Responses[0].Results) != 1 {
		panic("response IDs")
	}
	result := response.Responses[0].Results[0]
	if result.ID != "cap-kao" || result.Status != "permit" {
		panic(fmt.Sprintf("KAS result: %s %s", result.Status, result.Error))
	}
	reply := must(encoding.Base64Decode(result.Key))
	unwrapped := must(crypto.RSAOAEPDecrypt(session, reply))
	if !bytes.Equal(unwrapped, share) {
		panic("KAS wrapped share differs")
	}
	fmt.Println("PASS native capabilities: real issuer Bearer auth, RSA key r1 import, OAEP/HMAC-bound KAS Rewrap, independent session unwrap matches 32 bytes; DPoP enforcement not tested")
}
