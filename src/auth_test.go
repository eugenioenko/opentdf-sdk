package sdk

import (
	"context"
	"fmt"
	neturl "net/url"
	"strings"
	"testing"
)

type recordingTransport struct {
	url  string
	body []byte
}

func (r *recordingTransport) Do(ctx context.Context, method, url string, headers []string, body []byte, maxResponseBytes int, timeoutMillis int64) (int, []string, []byte, error) {
	r.url = url
	r.body = body
	return 200, []string{"Content-Type", "application/json"}, []byte(`{"access_token":"t","token_type":"Bearer","expires_in":60}`), nil
}

// The client-credentials form is built with url.Values as in the reference's
// getAccessTokenRequest (sdk/auth/oauth/oauth.go).
func TestGetAccessTokenForm(t *testing.T) {
	for _, scopes := range [][]string{nil, {"openid"}, {"openid", "profile email", "a+b&c=d/é"}} {
		rec := &recordingTransport{}
		client := &httpClient{transport: rec, clock: &fakeClock{}}
		if _, err := getAccessToken(context.Background(), client, "https://idp.example/token", scopes, clientCredentials{ClientID: "id", ClientAuth: "secret"}); err != nil {
			t.Fatal(err)
		}
		want := neturl.Values{}
		want.Set("grant_type", "client_credentials")
		if len(scopes) > 0 {
			want.Set("scope", strings.Join(scopes, " "))
		}
		if string(rec.body) != want.Encode() {
			t.Fatalf("form %q, want %q", rec.body, want.Encode())
		}
	}
}

// Ported from NewIDPAccessTokenSource (sdk/idp_access_token_source.go): the
// token endpoint is parsed up front and requests go to its String form.
func TestIDPAccessTokenSourceEndpoint(t *testing.T) {
	for _, endpoint := range []string{"://missing-scheme", "http://idp.example/%zz", "http://idp\x7f.example/token"} {
		_, err := newIDPAccessTokenSource(&httpClient{}, clientCredentials{}, endpoint, nil)
		_, refErr := neturl.Parse(endpoint)
		if err == nil || refErr == nil || err.Error() != fmt.Sprintf("invalid url [%s]: %v", endpoint, refErr) {
			t.Fatalf("newIDPAccessTokenSource(%q) = %v, reference %v", endpoint, err, refErr)
		}
	}
	rec := &recordingTransport{}
	source, err := newIDPAccessTokenSource(&httpClient{transport: rec, clock: &fakeClock{}}, clientCredentials{ClientID: "id", ClientAuth: "secret"}, "HTTPS://IDP.example/realms/a%20b/token", nil)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := source.Token(context.Background()); err != nil {
		t.Fatal(err)
	}
	ref, _ := neturl.Parse("HTTPS://IDP.example/realms/a%20b/token")
	if rec.url != ref.String() {
		t.Fatalf("token request to %q, want %q", rec.url, ref.String())
	}
}

func TestNewRejectsUnparseableTokenEndpoint(t *testing.T) {
	_, err := newWith(context.Background(), "https://platform.example", Options{ClientID: "id", ClientSecret: "secret", TokenEndpoint: "://missing-scheme", PlatformConfiguration: providedConfiguration()}, &recordingTransport{}, &fakeClock{})
	if err == nil || !strings.HasPrefix(err.Error(), "invalid url [://missing-scheme]: ") {
		t.Fatal(err)
	}
	if e, ok := err.(*Error); !ok || e.Kind != KindInvalidInput {
		t.Fatalf("%#v", err)
	}
}
