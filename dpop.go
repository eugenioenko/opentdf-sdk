package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/clock"
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/lib/http"
	j "opentdf-local/sdk/tdf/json"
)

type nonceEntry struct {
	origin string
	value  string
}

// authJWK emits only the RFC 7638 required public members in lexical order.
func authJWK(key *crypto.Key, algorithm string) (j.Value, error) {
	expected := "ec:secp256r1"
	if algorithm == "RS256" {
		expected = "rsa:2048"
	}
	if e := validateWrappingKey(key, expected); e != nil {
		return j.Value{}, e
	}
	k, e := key.PublicJWK()
	if e != nil {
		return j.Value{}, e
	}
	if algorithm == "RS256" {
		return object([]string{"e", "kty", "n"}, []j.Value{j.Str(k[3]), j.Str(k[0]), j.Str(k[2])}), nil
	}
	return object([]string{"crv", "kty", "x", "y"}, []j.Value{j.Str(k[1]), j.Str(k[0]), j.Str(k[4]), j.Str(k[5])}), nil
}
func authThumbprint(key *crypto.Key, algorithm string) (string, error) {
	jwk, e := authJWK(key, algorithm)
	if e != nil {
		return "", e
	}
	data, e := j.Marshal(jwk, jsonLimits())
	if e != nil {
		return "", e
	}
	hash, e := crypto.SHA256(data)
	if e != nil {
		return "", e
	}
	return encoding.Base64URLEncode(hash)
}

// validateBinding checks consistency, not JWT authenticity. Authentication of
// the issuer belongs to the trusted token endpoint/provider and resource server.
func (c *Client) validateBinding(token AccessToken) error {
	dots := []int{}
	for i, b := range []byte(token.Value) {
		if b == '.' {
			dots = append(dots, i)
		}
	}
	// Recognize a compact JWT by its JOSE JSON header, not token punctuation.
	// Explicitly attested opaque providers may legitimately return dotted values.
	jwt := false
	if len(dots) > 0 && dots[0] > 0 {
		header, err := encoding.Base64URLDecode(token.Value[:dots[0]])
		if err == nil {
			v, err := j.Parse(header, jsonLimits())
			jwt = err == nil && v.Kind == j.Object && (text(v, "alg") != "" || lower(text(v, "typ")) == "jwt")
		}
	}
	if !jwt {
		if token.ConfirmationJKT != c.thumbprint {
			return failure("auth", "token_binding", nil)
		}
		return nil
	}
	if len(dots) != 2 || dots[1] == len(token.Value)-1 {
		return failure("auth", "token_binding", nil)
	}
	claims, e := encoding.Base64URLDecode(token.Value[dots[0]+1 : dots[1]])
	if e != nil {
		return failure("auth", "token_binding", e)
	}
	v, e := j.Parse(claims, jsonLimits())
	if e != nil {
		return failure("auth", "token_binding", e)
	}
	cnf, ok := v.Get("cnf")
	if !ok || cnf.Kind != j.Object || text(cnf, "jkt") != c.thumbprint {
		return failure("auth", "token_binding", nil)
	}
	return nil
}
func (c *Client) dpopProof(method, htu, token, nonce string) (string, error) {
	jwk, e := authJWK(c.authKey, c.config.AuthAlgorithm)
	if e != nil {
		return "", failure("dpop", "auth_key", e)
	}
	random, e := crypto.Random(32)
	if e != nil {
		return "", failure("dpop", "random", e)
	}
	id, e := encoding.Base64URLEncode(random)
	if e != nil {
		return "", e
	}
	now := clock.Unix()
	if now <= 0 {
		return "", failure("dpop", "clock", nil)
	}
	header, e := j.Marshal(object([]string{"alg", "typ", "jwk"}, []j.Value{j.Str(c.config.AuthAlgorithm), j.Str("dpop+jwt"), jwk}), jsonLimits())
	if e != nil {
		return "", e
	}
	names := []string{"jti", "iat", "htm", "htu"}
	values := []j.Value{j.Str(id), j.Num(integerString(now)), j.Str(method), j.Str(htu)}
	if token != "" {
		hash, err := crypto.SHA256([]byte(token))
		if err != nil {
			return "", err
		}
		ath, err := encoding.Base64URLEncode(hash)
		if err != nil {
			return "", err
		}
		names = append(names, "ath")
		values = append(values, j.Str(ath))
	}
	if nonce != "" {
		names = append(names, "nonce")
		values = append(values, j.Str(nonce))
	}
	claims, e := j.Marshal(object(names, values), jsonLimits())
	if e != nil {
		return "", e
	}
	return signJWT(c.authKey, c.config.AuthAlgorithm, header, claims)
}
func responseNonce(headers []string) (string, error) {
	value := ""
	count := 0
	for i := 0; i+1 < len(headers); i += 2 {
		if lower(headers[i]) == "dpop-nonce" {
			count++
			value = headers[i+1]
		}
	}
	if count == 0 {
		return "", nil
	}
	if count != 1 || len(value) == 0 || len(value) > 1024 {
		return "", failure("dpop", "invalid_nonce", nil)
	}
	// RFC 9449 nonce ABNF is unquoted visible ASCII excluding quote/backslash.
	for _, b := range []byte(value) {
		if b < 33 || b > 126 || b == '"' || b == '\\' {
			return "", failure("dpop", "invalid_nonce", nil)
		}
	}
	return value, nil
}
func (c *Client) cachedNonce(origin string) string {
	c.mu.Lock()
	value := ""
	for _, entry := range c.nonces {
		if entry.origin == origin {
			value = entry.value
		}
	}
	c.mu.Unlock()
	return value
}
func (c *Client) cacheNonce(origin, nonce string) {
	c.mu.Lock()
	if !c.closed {
		found := false
		for i := range c.nonces {
			if c.nonces[i].origin == origin {
				c.nonces[i].value = nonce
				found = true
			}
		}
		// Trusted routes cap resources at 64, plus platform metadata and token origin.
		if !found && len(c.nonces) < 66 {
			c.nonces = append(c.nonces, nonceEntry{origin: origin, value: nonce})
		}
	}
	c.mu.Unlock()
}
func (c *Client) proofHTTP(ctx context.Context, method, url string, headers []string, body []byte, token string, proof bool) (int, []string, []byte, error) {
	if !proof {
		return http.Do(ctx, method, url, headers, body, 1<<20, c.config.TimeoutMillis)
	}
	u, e := parseEndpoint(url, c.config.AllowHTTP)
	if e != nil {
		return 0, nil, nil, e
	}
	// Preserve the complete request path. parseEndpoint's route normalization
	// removes one trailing slash, which is meaningful in a DPoP HTTP URI.
	path := u.path
	if len(url) > 0 && url[len(url)-1] == '/' {
		path += "/"
	}
	if path == "" {
		path = "/"
	}
	htu := u.origin + path
	nonce := c.cachedNonce(u.origin)
	for attempt := 0; attempt < 2; attempt++ {
		if e := c.check(ctx, "dpop"); e != nil {
			return 0, nil, nil, e
		}
		jwt, e := c.dpopProof(method, htu, token, nonce)
		if e != nil {
			return 0, nil, nil, e
		}
		outgoing := make([]string, len(headers), len(headers)+2)
		copy(outgoing, headers)
		outgoing = append(outgoing, "DPoP", jwt)
		status, responseHeaders, data, e := http.Do(ctx, method, url, outgoing, body, 1<<20, c.config.TimeoutMillis)
		if e != nil {
			return 0, nil, nil, failure("dpop", "transport", e)
		}
		received, e := responseNonce(responseHeaders)
		if e != nil {
			return status, responseHeaders, nil, &Error{Code: "invalid_nonce", Operation: "dpop", HTTPStatus: status, Cause: e}
		}
		challenge := status == 400 || status == 401
		if received != "" && (challenge || status >= 200 && status < 300) {
			c.cacheNonce(u.origin, received)
		}
		if attempt == 0 && challenge && received != "" && received != nonce {
			nonce = received
			continue
		}
		return status, responseHeaders, data, nil
	}
	return 0, nil, nil, failure("dpop", "retry_limit", nil)
}
