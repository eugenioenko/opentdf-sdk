package sdk

import (
	"context"
	"errors"
	"testing"
)

// Ported from the reference's access_token_test.go.

type fakeAccessTokenSource struct {
	accessToken string
	err         error
}

func (f fakeAccessTokenSource) Token(context.Context) (AccessToken, error) {
	return AccessToken{Value: f.accessToken, Scheme: "Bearer"}, f.err
}

func TestAccessToken_ReturnsTokenFromSource(t *testing.T) {
	s := &SDK{tokenSource: fakeAccessTokenSource{accessToken: "test-token"}}

	tok, err := s.Auth().AccessToken(context.Background())
	if err != nil || tok.Value != "test-token" || tok.Scheme != "Bearer" {
		t.Fatal(tok, err)
	}
}

func TestAccessToken_NoTokenSource(t *testing.T) {
	s := &SDK{}

	tok, err := s.Auth().AccessToken(context.Background())
	var e *Error
	if !errors.Is(err, ErrNoAccessTokenSource) || !errors.As(err, &e) || e.Kind != KindAuth || tok != (AccessToken{}) {
		t.Fatal(tok, err)
	}
}

func TestAccessToken_EmptyToken(t *testing.T) {
	s := &SDK{tokenSource: fakeAccessTokenSource{accessToken: ""}}

	tok, err := s.Auth().AccessToken(context.Background())
	if !errors.Is(err, ErrAccessTokenInvalid) || tok != (AccessToken{}) {
		t.Fatal(tok, err)
	}
}

func TestAccessToken_SourceError(t *testing.T) {
	s := &SDK{tokenSource: fakeAccessTokenSource{err: errors.New("idp down")}}

	_, err := s.Auth().AccessToken(context.Background())
	var e *Error
	if !errors.As(err, &e) || e.Kind != KindAuth || e.Message != "idp down" {
		t.Fatalf("%#v", err)
	}
}

type recordCtxKey struct{}

type recordingTokenSource struct {
	token       string
	gotCtxValue any
}

func (r *recordingTokenSource) Token(ctx context.Context) (AccessToken, error) {
	r.gotCtxValue = ctx.Value(recordCtxKey{})
	return AccessToken{Value: r.token, Scheme: "Bearer"}, nil
}

func TestAccessToken_ForwardsContext(t *testing.T) {
	ctx := context.WithValue(context.Background(), recordCtxKey{}, "value")

	rec := &recordingTokenSource{token: "test-token"}
	s := &SDK{tokenSource: rec}

	tok, err := s.Auth().AccessToken(ctx)
	if err != nil || tok.Value != "test-token" {
		t.Fatal(tok, err)
	}
	if rec.gotCtxValue != "value" {
		t.Fatal("context should be forwarded unchanged", rec.gotCtxValue)
	}
}

// Through New: the client-credentials source behind the SDK's transport.
func TestAccessTokenFromPlatform(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	tok, err := s.Auth().AccessToken(context.Background())
	if err != nil || tok.Value != "access-token-Bearer" || tok.Scheme != "Bearer" || tok.ExpiresAt == 0 {
		t.Fatal(tok, err)
	}
	anonymous := f.newSDK(t, Options{})
	if _, err := anonymous.Auth().AccessToken(context.Background()); !errors.Is(err, ErrNoAccessTokenSource) {
		t.Fatal(err)
	}
	s.Close()
	if _, err := s.Auth().AccessToken(context.Background()); err == nil || err.Error() != "sdk is closed" {
		t.Fatal(err)
	}
}
