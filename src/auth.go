package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/callback"
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/net/url"
	"github.com/eugenioenko/goalchemy/std/strconv"
	"github.com/eugenioenko/goalchemy/std/strings"
)

// AccessToken is an access token with its scheme ("Bearer" or "DPoP"),
// expiry as UTC Unix seconds (0 = never) and, for opaque DPoP tokens from a
// host provider, the attested JWK thumbprint.
type AccessToken struct {
	Value           string
	Scheme          string
	ExpiresAt       int64
	ConfirmationJKT string
}

// TokenSource supplies access tokens: the reference's auth.AccessTokenSource.
type TokenSource interface {
	Token(ctx context.Context) (AccessToken, error)
}

const (
	tokenExpirationBuffer = 10
	tokenRequestMaxBytes  = 1 << 20
)

// idpAccessTokenSource obtains client-credentials tokens from the IdP, bound
// to the SDK's DPoP key, and caches them until shortly before they expire.
type idpAccessTokenSource struct {
	client           *httpClient
	credentials      clientCredentials
	idpTokenEndpoint url.URL
	scopes           []string
	gate             chan bool
	token            AccessToken
	hasToken         bool
}

func newIDPAccessTokenSource(client *httpClient, credentials clientCredentials, idpTokenEndpoint string, scopes []string) (*idpAccessTokenSource, error) {
	endpoint, err := url.Parse(idpTokenEndpoint)
	if err != nil {
		return nil, fmt.Errorf("invalid url [%s]: %w", idpTokenEndpoint, err)
	}
	t := &idpAccessTokenSource{client: client, credentials: credentials, idpTokenEndpoint: *endpoint, scopes: scopes, gate: make(chan bool, 1)}
	t.gate <- true
	return t, nil
}

func (t *idpAccessTokenSource) expired(now int64) bool {
	if !t.hasToken {
		return true
	}
	if t.token.ExpiresAt == 0 {
		return false
	}
	return now > t.token.ExpiresAt-tokenExpirationBuffer
}

func (t *idpAccessTokenSource) Token(ctx context.Context) (AccessToken, error) {
	select {
	case <-ctx.Done():
		return AccessToken{}, wrapError(KindCanceled, "auth", "", ctx.Err())
	case <-t.gate:
	}
	defer func() { t.gate <- true }()
	if !t.expired(t.client.clock.Now()) {
		return t.token, nil
	}
	token, err := getAccessToken(ctx, t.client, t.idpTokenEndpoint.String(), t.scopes, t.credentials)
	if err != nil {
		return AccessToken{}, wrapError(KindAuth, "auth", "error getting access token", err)
	}
	t.token = token
	t.hasToken = true
	return token, nil
}

// getAccessToken performs the client-credentials grant as the reference's
// oauth.GetAccessToken does: HTTP Basic client authentication, a DPoP proof
// without ath, and the DPoP-Nonce retry handled by the client.
func getAccessToken(ctx context.Context, client *httpClient, tokenEndpoint string, scopes []string, credentials clientCredentials) (AccessToken, error) {
	formData := url.Values{}
	formData.Set("grant_type", "client_credentials")
	if len(scopes) > 0 {
		formData.Set("scope", strings.Join(scopes, " "))
	}
	basic, err := encoding.Base64Encode([]byte(credentials.ClientID + ":" + credentials.ClientAuth))
	if err != nil {
		return AccessToken{}, err
	}
	headers := []string{"Accept", "application/json", "Content-Type", "application/x-www-form-urlencoded", "Authorization", "Basic " + basic}
	status, responseHeaders, data, err := client.do(ctx, "POST", tokenEndpoint, headers, []byte(formData.Encode()), "", true)
	if err != nil {
		return AccessToken{}, wrapError(KindTransport, "auth", "error making token request", err)
	}
	if status < 200 || status >= 300 {
		message := ""
		code := ""
		if v, err := jsonvalue.Parse(data); err == nil && v.Kind() == jsonvalue.KindObject {
			code, _ = v.Get("error").AsString()
			message, _ = v.Get("error_description").AsString()
		}
		text := "got status " + strconv.Itoa(status) + " when making token request"
		if code != "" {
			text += ": " + code
		}
		return AccessToken{}, &Error{Kind: KindAuth, Op: "auth", Message: text, HTTPStatus: status, ConnectCode: code, ServiceMessage: message}
	}
	if !isJSONContentType(responseHeaders) {
		return AccessToken{}, newError(KindAuth, "auth", "unexpected content type from token endpoint")
	}
	v, err := jsonvalue.Parse(data)
	if err != nil || v.Kind() != jsonvalue.KindObject {
		return AccessToken{}, newError(KindAuth, "auth", "invalid token response")
	}
	value, ok := v.Get("access_token").AsString()
	if !ok || value == "" {
		return AccessToken{}, newError(KindAuth, "auth", "token response has no access_token")
	}
	token := AccessToken{Value: value, Scheme: tokenTypeFromOAuthTokenType(v.Get("token_type").String())}
	if expires, ok := v.Get("expires_in").AsInt64(); ok && expires > 0 {
		token.ExpiresAt = client.clock.Now() + expires
	}
	return token, nil
}

// tokenTypeFromOAuthTokenType normalises the token_type value: "dpop" in any
// case is DPoP, everything else is Bearer, as the reference does.
func tokenTypeFromOAuthTokenType(tokenType string) string {
	if strings.EqualFold(tokenType, "DPoP") {
		return "DPoP"
	}
	return "Bearer"
}

// callbackTokenSource asks the host's registered token provider for tokens
// and validates them: scheme, expiry and, for DPoP tokens, the binding to the
// SDK's DPoP key.
type callbackTokenSource struct {
	name  string
	clock Clock
	dpop  *dpopSigner
	gate  chan bool
	token AccessToken
}

func newCallbackTokenSource(name string, clock Clock, dpop *dpopSigner) *callbackTokenSource {
	t := &callbackTokenSource{name: name, clock: clock, dpop: dpop, gate: make(chan bool, 1)}
	t.gate <- true
	return t
}

func (t *callbackTokenSource) Token(ctx context.Context) (AccessToken, error) {
	select {
	case <-ctx.Done():
		return AccessToken{}, wrapError(KindCanceled, "auth", "", ctx.Err())
	case <-t.gate:
	}
	defer func() { t.gate <- true }()
	now := t.clock.Now()
	if validToken(t.token, now+tokenExpirationBuffer) {
		return t.token, nil
	}
	response, err := callback.Request(ctx, t.name, []byte("{}"))
	if err != nil {
		return AccessToken{}, wrapError(KindAuth, "auth", "token provider failed", err)
	}
	token, err := parseProviderToken(response)
	if err != nil {
		return AccessToken{}, wrapError(KindAuth, "auth", "", err)
	}
	if !validToken(token, now) {
		return AccessToken{}, newError(KindAuth, "auth", "token provider returned an invalid or expired token")
	}
	if token.Scheme == "DPoP" {
		if t.dpop == nil {
			return AccessToken{}, newError(KindAuth, "auth", "token provider returned a DPoP token but no DPoP key is configured")
		}
		if err := validateBinding(token, t.dpop.thumbprint); err != nil {
			return AccessToken{}, err
		}
	}
	t.token = token
	return token, nil
}

func validToken(t AccessToken, now int64) bool {
	if (t.Scheme != "Bearer" && t.Scheme != "DPoP") || len(t.Value) == 0 || len(t.Value) > 65536 || t.ExpiresAt != 0 && t.ExpiresAt <= now {
		return false
	}
	for _, b := range []byte(t.Value) {
		if b < 33 || b > 126 {
			return false
		}
	}
	return true
}

// parseProviderToken decodes the host token wire form:
// {"value","scheme","expiresAt" (decimal string),"confirmationJKT"}.
func parseProviderToken(data []byte) (AccessToken, error) {
	v, err := jsonvalue.ParseWithLimits(data, jsonvalue.Limits{Bytes: 128 * 1024, Depth: 2, Nodes: 10, StringBytes: 65536})
	if err != nil || v.Kind() != jsonvalue.KindObject {
		return AccessToken{}, errors.New("provider: invalid token response")
	}
	for _, n := range v.Keys() {
		if n != "value" && n != "scheme" && n != "expiresAt" && n != "confirmationJKT" {
			return AccessToken{}, errors.New("provider: unsupported token field")
		}
	}
	value, ok := v.Get("value").AsString()
	if !ok {
		return AccessToken{}, errors.New("provider: missing value")
	}
	scheme, ok := v.Get("scheme").AsString()
	if !ok {
		return AccessToken{}, errors.New("provider: missing scheme")
	}
	expiration, ok := v.Get("expiresAt").AsString()
	if !ok {
		return AccessToken{}, errors.New("provider: missing decimal expiry")
	}
	var seconds int64
	if len(expiration) == 0 || len(expiration) > 19 {
		return AccessToken{}, errors.New("provider: invalid expiry")
	}
	for _, b := range []byte(expiration) {
		if b < '0' || b > '9' || seconds > (9223372036854775807-int64(b-'0'))/10 {
			return AccessToken{}, errors.New("provider: invalid expiry")
		}
		seconds = seconds*10 + int64(b-'0')
	}
	confirmation := ""
	if c, exists := v.Lookup("confirmationJKT"); exists {
		s, ok := c.AsString()
		if !ok {
			return AccessToken{}, errors.New("provider: invalid confirmation")
		}
		confirmation = s
	}
	return AccessToken{Value: value, Scheme: scheme, ExpiresAt: seconds, ConfirmationJKT: confirmation}, nil
}

// validateBinding checks that a DPoP token is bound to thumbprint: a JWT must
// carry a matching cnf.jkt claim, an opaque token a matching attestation.
func validateBinding(token AccessToken, thumbprint string) error {
	parts := strings.Split(token.Value, ".")
	jwt := false
	if len(parts) >= 2 && parts[0] != "" {
		header, err := encoding.Base64URLDecode(parts[0])
		if err == nil {
			v, err := jsonvalue.Parse(header)
			jwt = err == nil && v.Kind() == jsonvalue.KindObject && (v.Get("alg").String() != "" || strings.EqualFold(v.Get("typ").String(), "jwt"))
		}
	}
	if !jwt {
		if token.ConfirmationJKT != thumbprint {
			return newError(KindAuth, "auth", "token is not bound to the DPoP key")
		}
		return nil
	}
	if len(parts) != 3 || parts[2] == "" {
		return newError(KindAuth, "auth", "token is not bound to the DPoP key")
	}
	claims, err := encoding.Base64URLDecode(parts[1])
	if err != nil {
		return newError(KindAuth, "auth", "token is not bound to the DPoP key")
	}
	v, err := jsonvalue.Parse(claims)
	if err != nil || v.Get("cnf").Get("jkt").String() != thumbprint {
		return newError(KindAuth, "auth", "token is not bound to the DPoP key")
	}
	return nil
}
