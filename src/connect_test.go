package sdk

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

// SDK.Call is the reference's sdkconnect clients in JSON: the request
// reaches the service through the authenticated transport, and a service
// failure keeps its Connect code, message and HTTP status.
func TestCall(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	ctx := context.Background()
	out, err := s.Call(ctx, "policy.kasregistry.KeyAccessServerRegistryService", "ListKeyAccessServers", `{"pagination":{"limit":10,"offset":0}}`)
	if err != nil {
		t.Fatal(err)
	}
	var resp struct {
		KeyAccessServers []struct {
			URI string `json:"uri"`
		} `json:"keyAccessServers"`
	}
	if err := json.Unmarshal([]byte(out), &resp); err != nil || len(resp.KeyAccessServers) != 1 || resp.KeyAccessServers[0].URI != f.kasURL() {
		t.Fatal("response", out, err)
	}
	f.mu.Lock()
	auth, body := f.lastAuth, f.lastBody
	f.mu.Unlock()
	if auth != "Bearer access-token-Bearer" {
		t.Fatal("authorization", auth)
	}
	if body != `{"pagination":{"limit":10,"offset":0}}` {
		t.Fatal("request body", body)
	}
	if _, err := s.Call(ctx, "policy.kasregistry.KeyAccessServerRegistryService", "ListKeyAccessServers", ""); err != nil {
		t.Fatal("empty request is {}", err)
	}

	f.mode.Store("registry-error")
	_, err = s.Call(ctx, "policy.kasregistry.KeyAccessServerRegistryService", "ListKeyAccessServers", "{}")
	var e *Error
	if !errors.As(err, &e) || e.Kind != KindTransport || e.ConnectCode != "internal" || e.HTTPStatus != 500 || e.ServiceMessage != "registry down" || e.Message != "internal: registry down" || e.Op != "Call" {
		t.Fatalf("service error %#v", err)
	}
	f.mode.Store("")
	_, err = s.Call(ctx, "policy.attributes.AttributesService", "ListAttributes", "{}")
	if !errors.As(err, &e) || e.ConnectCode != "unimplemented" || e.HTTPStatus != 404 {
		t.Fatalf("unimplemented %#v", err)
	}

	for _, tc := range []struct{ service, method, request, want string }{
		{"kas.AccessService", "Rewrap", "{}", "unknown service: kas.AccessService"},
		{"policy.attributes.AttributesService", "List/Attributes", "{}", "invalid method name: List/Attributes"},
		{"policy.attributes.AttributesService", "ListAttributes", "[]", "request is not a JSON object"},
		{"policy.attributes.AttributesService", "ListAttributes", "{", "request is not a JSON object"},
	} {
		_, err := s.Call(ctx, tc.service, tc.method, tc.request)
		if !errors.As(err, &e) || e.Kind != KindInvalidInput || e.Message != tc.want {
			t.Fatalf("%s/%s: %v", tc.service, tc.method, err)
		}
	}

	cancelled, cancel := context.WithCancel(ctx)
	cancel()
	if _, err := s.Call(cancelled, "policy.attributes.AttributesService", "ListAttributes", "{}"); !errors.As(err, &e) || e.Kind != KindCanceled {
		t.Fatalf("cancelled %#v", err)
	}
	s.Close()
	if _, err := s.Call(ctx, "policy.attributes.AttributesService", "ListAttributes", "{}"); err == nil || !strings.Contains(err.Error(), "sdk is closed") {
		t.Fatal("closed", err)
	}
}

// A failed registry lookup keeps the service's code and status through the
// reader's wrapping.
func TestServiceErrorKinds(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	data, _, err := s.EncryptBytes(context.Background(), []byte("registry"), kasOptions(f, RSA2048Key))
	if err != nil {
		t.Fatal(err)
	}
	f.mode.Store("registry-error")
	_, err = s.DecryptBytes(context.Background(), data, ReaderOptions{})
	var e *Error
	if !errors.As(err, &e) || e.ConnectCode != "internal" || e.HTTPStatus != 500 || !strings.Contains(e.Message, "kasregistry.ListKeyAccessServers failed: internal: registry down") {
		t.Fatalf("registry failure %#v", err)
	}
	f.mode.Store("rewrap-unauthenticated")
	_, err = s.DecryptBytes(context.Background(), data, ReaderOptions{KasAllowlist: []string{f.kasURL()}})
	if !errors.As(err, &e) || e.Kind != KindAuth || e.ConnectCode != "unauthenticated" || e.HTTPStatus != 401 || !strings.Contains(e.Message, "unauthenticated: invalid token") {
		t.Fatalf("rewrap failure %#v", err)
	}
}

// A reply the client cannot read is a transport failure, as before the
// generated clients; a Connect error body keeps the service's code.
func TestServiceErrorReplies(t *testing.T) {
	for _, tc := range []struct {
		name, contentType, body string
		status                  int
		kind                    ErrorKind
		code, text              string
	}{
		{"content-type", "text/html", "{}", 200, KindTransport, "unknown", `unknown: invalid content-type: "text/html"; expecting "application/json"`},
		{"malformed", "application/json", "{bad", 200, KindTransport, "invalid_argument", "invalid_argument: unmarshal message"},
		{"not-connect", "text/plain", "no", 502, KindTransport, "unavailable", "unavailable: 502 Bad Gateway"},
		{"denied", "application/json", `{"code":"permission_denied","message":"forbidden"}`, 403, KindAuth, "permission_denied", "permission_denied: forbidden"},
		{"invalid", "application/json", `{"code":"invalid_argument","message":"bad"}`, 400, KindTransport, "invalid_argument", "invalid_argument: bad"},
	} {
		server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			w.Header().Set("Content-Type", tc.contentType)
			w.WriteHeader(tc.status)
			w.Write([]byte(tc.body))
		}))
		_, err := New(context.Background(), server.URL, Options{})
		server.Close()
		var e *Error
		if !errors.As(err, &e) || e.Kind != tc.kind || e.ConnectCode != tc.code || e.HTTPStatus != tc.status || !strings.Contains(e.Message, tc.text) || !errors.Is(err, ErrPlatformConfigFailed) {
			t.Fatalf("%s: %#v", tc.name, err)
		}
	}
}
