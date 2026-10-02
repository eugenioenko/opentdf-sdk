package main

import (
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
	"errors"
	"io"
	"math/big"
	"net/http"
	"net/url"
	"strings"
	"time"

	g "goalchemyout"
)

// This is an independent host OAuth provider. It performs no TDF operation and
// imports no shared SDK code. All token/private key values stay in memory.
func dpopProvider(ctx context.Context, cfg g.Config, mismatched bool) (g.AccessToken, error) {
	block, _ := pem.Decode([]byte(cfg.AuthPrivateKeyPEM))
	if block == nil {
		return g.AccessToken{}, errors.New("provider key")
	}
	key, err := x509.ParsePKCS8PrivateKey(block.Bytes)
	if err != nil {
		return g.AccessToken{}, err
	}
	if mismatched {
		key, err = ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
		if err != nil {
			return g.AccessToken{}, err
		}
	}
	enc := base64.RawURLEncoding.EncodeToString
	jwk := map[string]string{}
	algorithm := ""
	switch k := key.(type) {
	case *ecdsa.PrivateKey:
		jwk = map[string]string{"kty": "EC", "crv": "P-256", "x": enc(k.X.FillBytes(make([]byte, 32))), "y": enc(k.Y.FillBytes(make([]byte, 32)))}
		algorithm = "ES256"
	case *rsa.PrivateKey:
		jwk = map[string]string{"kty": "RSA", "n": enc(k.N.Bytes()), "e": enc(big.NewInt(int64(k.E)).Bytes())}
		algorithm = "RS256"
	default:
		return g.AccessToken{}, errors.New("provider algorithm")
	}
	canonical, err := json.Marshal(jwk)
	if err != nil {
		return g.AccessToken{}, err
	}
	thumb := sha256.Sum256(canonical)
	endpoint := "http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token"
	nonce := ""
	for attempt := 0; attempt < 3; attempt++ {
		jti := make([]byte, 16)
		if _, err = rand.Read(jti); err != nil {
			return g.AccessToken{}, err
		}
		header, _ := json.Marshal(map[string]any{"typ": "dpop+jwt", "alg": algorithm, "jwk": jwk})
		claims := map[string]any{"htu": endpoint, "htm": "POST", "iat": time.Now().Unix(), "jti": enc(jti)}
		if nonce != "" {
			claims["nonce"] = nonce
		}
		body, _ := json.Marshal(claims)
		unsigned := enc(header) + "." + enc(body)
		digest := sha256.Sum256([]byte(unsigned))
		var sig []byte
		switch k := key.(type) {
		case *ecdsa.PrivateKey:
			r, s, e := ecdsa.Sign(rand.Reader, k, digest[:])
			if e != nil {
				return g.AccessToken{}, e
			}
			sig = append(r.FillBytes(make([]byte, 32)), s.FillBytes(make([]byte, 32))...)
		case *rsa.PrivateKey:
			sig, err = rsa.SignPKCS1v15(rand.Reader, k, crypto.SHA256, digest[:])
			if err != nil {
				return g.AccessToken{}, err
			}
		}
		form := url.Values{"grant_type": {"client_credentials"}, "client_id": {"opentdf-sdk"}, "client_secret": {"secret"}}
		request, err := http.NewRequestWithContext(ctx, "POST", endpoint, strings.NewReader(form.Encode()))
		if err != nil {
			return g.AccessToken{}, err
		}
		request.Header.Set("Content-Type", "application/x-www-form-urlencoded")
		request.Header.Set("DPoP", unsigned+"."+enc(sig))
		response, err := (&http.Client{Timeout: 10 * time.Second}).Do(request)
		if err != nil {
			return g.AccessToken{}, err
		}
		data, err := io.ReadAll(io.LimitReader(response.Body, 128*1024))
		response.Body.Close()
		if err != nil {
			return g.AccessToken{}, err
		}
		if response.StatusCode == 400 || response.StatusCode == 401 {
			if next := response.Header.Get("DPoP-Nonce"); next != "" && next != nonce {
				nonce = next
				continue
			}
		}
		if response.StatusCode != 200 {
			return g.AccessToken{}, errors.New("provider endpoint")
		}
		var token struct {
			Value   string `json:"access_token"`
			Scheme  string `json:"token_type"`
			Expires int64  `json:"expires_in"`
		}
		if err = json.Unmarshal(data, &token); err != nil {
			return g.AccessToken{}, err
		}
		if !strings.EqualFold(token.Scheme, "DPoP") {
			return g.AccessToken{}, errors.New("provider token scheme")
		}
		return g.AccessToken{Value: token.Value, Scheme: "DPoP", ExpiresAt: time.Now().Unix() + token.Expires, ConfirmationJKT: enc(thumb[:])}, nil
	}
	return g.AccessToken{}, errors.New("provider nonce retries")
}
