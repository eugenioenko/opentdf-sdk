package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
)

// Auth returns an AuthClient for authentication-related operations.
func (s *SDK) Auth() *AuthClient {
	return &AuthClient{sdk: s}
}

// AuthClient groups authentication operations for the SDK.
type AuthClient struct {
	sdk *SDK
}

// AccessToken returns a valid access token for the SDK's configured credentials.
// It returns ErrNoAccessTokenSource if the SDK was created without credentials, and
// ErrAccessTokenInvalid if the token source returns an empty token. The
// token carries its scheme and expiry; the reference returns the value only.
func (a *AuthClient) AccessToken(ctx context.Context) (AccessToken, error) {
	if err := a.sdk.check("AccessToken"); err != nil {
		return AccessToken{}, err
	}
	if a.sdk.tokenSource == nil {
		return AccessToken{}, sentinelError(KindAuth, ErrNoAccessTokenSource)
	}
	token, err := a.sdk.tokenSource.Token(ctx)
	if err != nil {
		return AccessToken{}, boundaryError(KindAuth, "AccessToken", err)
	}
	if token.Value == "" {
		return AccessToken{}, sentinelError(KindAuth, ErrAccessTokenInvalid)
	}
	return token, nil
}
