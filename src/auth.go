package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/clock"
	"github.com/eugenioenko/goalchemy/lib/context"
	j "opentdf-local/sdk/src/tdf/json"
)

// AccessToken retains token scheme, expiry and optional opaque binding attestation.
// Scheme must match Config.DPoP. ExpiresAt is UTC Unix seconds in the future.
// The provider owns refresh; the client caches until five seconds before expiry.
type AccessToken struct {
	Value     string
	Scheme    string
	ExpiresAt int64
	// ConfirmationJKT is the provider attestation for an opaque DPoP token.
	// JWT tokens must instead contain matching cnf.jkt claims.
	ConfirmationJKT string
}

func formEscape(s string) string {
	const hex = "0123456789ABCDEF"
	out := make([]byte, 0, len(s))
	for _, b := range []byte(s) {
		if b >= 'a' && b <= 'z' || b >= 'A' && b <= 'Z' || b >= '0' && b <= '9' || b == '-' || b == '_' || b == '.' || b == '~' {
			out = append(out, b)
		} else if b == ' ' {
			out = append(out, '+')
		} else {
			out = append(out, '%', hex[int(b)>>4], hex[int(b)&15])
		}
	}
	return string(out)
}
func validToken(t AccessToken, now int64) bool {
	if (t.Scheme != "Bearer" && t.Scheme != "DPoP") || len(t.Value) == 0 || len(t.Value) > 65536 || t.ExpiresAt <= now {
		return false
	}
	for _, b := range []byte(t.Value) {
		if b < 33 || b > 126 {
			return false
		}
	}
	return true
}
func (c *Client) accessToken(ctx context.Context) (AccessToken, error) {
	if e := c.check(ctx, "auth"); e != nil {
		return AccessToken{}, e
	}
	child, cancel := c.operationContext(ctx)
	select {
	case <-child.Done():
		cancel()
		return AccessToken{}, failure("auth", "canceled", child.Err())
	case <-c.tokenGate:
	}
	token, e := c.acquireToken(child)
	c.tokenGate <- true
	cancel()
	return token, e
}
func (c *Client) acquireToken(ctx context.Context) (AccessToken, error) {
	if e := c.check(ctx, "auth"); e != nil {
		return AccessToken{}, e
	}
	now := clock.Unix()
	c.mu.Lock()
	cached := c.token
	c.mu.Unlock()
	if validToken(cached, now+5) {
		return cached, nil
	}
	var token AccessToken
	var e error
	if c.config.TokenProvider != nil {
		token, e = c.config.TokenProvider(ctx)
	} else {
		token, e = c.clientCredentials(ctx)
	}
	if e != nil {
		if c.config.TokenProvider == nil {
			return AccessToken{}, e
		}
		return AccessToken{}, failure("auth", "token_acquisition", e)
	}
	expected := "Bearer"
	if c.config.DPoP {
		expected = "DPoP"
	}
	if token.Scheme != expected {
		return AccessToken{}, failure("auth", "unsupported_token_scheme", nil)
	}
	if !validToken(token, clock.Unix()) {
		return AccessToken{}, failure("auth", "invalid_token", nil)
	}
	if c.config.DPoP {
		if e := c.validateBinding(token); e != nil {
			return AccessToken{}, e
		}
	}
	if e := c.check(ctx, "auth"); e != nil {
		return AccessToken{}, e
	}
	c.mu.Lock()
	if !c.closed {
		c.token = token
	}
	c.mu.Unlock()
	return token, nil
}
func (c *Client) tokenEndpoint(ctx context.Context) (string, error) {
	if c.config.TokenURL != "" {
		return c.config.TokenURL, nil
	}
	// Caller supplies the expected issuer. Metadata cannot expand credential routes.
	v, e := c.exchange(ctx, "GET", c.config.PlatformURL+"/.well-known/opentdf-configuration", nil, nil, "discovery")
	if e != nil {
		return "", e
	}
	idp, ok := v.Get("idp")
	if !ok || idp.Kind != j.Object {
		return "", failure("discovery", "invalid_idp", nil)
	}
	issuer, e := parseEndpoint(text(idp, "issuer"), c.config.AllowHTTP)
	if e != nil || issuer.String() != c.config.IssuerURL {
		return "", failure("discovery", "issuer_mismatch", e)
	}
	token, e := parseEndpoint(text(idp, "token_endpoint"), c.config.AllowHTTP)
	if e != nil {
		return "", e
	}
	// Ordinary OpenID discovery token route. Custom routes must be configured explicitly.
	expected := c.config.IssuerURL + "/protocol/openid-connect/token"
	if token.String() != expected {
		return "", failure("discovery", "token_destination_not_allowed", nil)
	}
	return token.String(), nil
}
func (c *Client) clientCredentials(ctx context.Context) (AccessToken, error) {
	endpoint, e := c.tokenEndpoint(ctx)
	if e != nil {
		return AccessToken{}, e
	}
	body := []byte("grant_type=client_credentials&client_id=" + formEscape(c.config.ClientID) + "&client_secret=" + formEscape(c.config.ClientSecret))
	v, e := c.exchangeProof(ctx, "POST", endpoint, []string{"Content-Type", "application/x-www-form-urlencoded"}, body, "oauth", "", c.config.DPoP)
	if e != nil {
		return AccessToken{}, e
	}
	expiration, ok := v.Get("expires_in")
	if !ok {
		return AccessToken{}, failure("oauth", "missing_expiration", nil)
	}
	seconds, e := expiration.Integer(86400)
	if e != nil || seconds == 0 {
		return AccessToken{}, failure("oauth", "invalid_expiration", e)
	}
	scheme := text(v, "token_type")
	if lower(scheme) == "bearer" {
		scheme = "Bearer"
	}
	if lower(scheme) == "dpop" {
		scheme = "DPoP"
	}
	token := AccessToken{Value: text(v, "access_token"), Scheme: scheme, ExpiresAt: clock.Unix() + int64(seconds)}
	// An authenticated trusted endpoint declaring DPoP attests opaque token binding.
	if c.config.DPoP && scheme == "DPoP" {
		token.ConfirmationJKT = c.thumbprint
	}
	return token, nil
}
