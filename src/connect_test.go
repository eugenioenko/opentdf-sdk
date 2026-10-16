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
	_, err = s.Call(ctx, "policy.namespaces.NamespaceService", "ListNamespaces", "{}")
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

// The helpers over the generated clients and the SDK's transport: the
// requests are protobuf JSON, unknown reply fields are ignored, pagination
// follows nextOffset and not_found is false rather than an error.
func TestHelpersThroughPlatform(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	ctx := context.Background()
	attrs, err := s.ListAttributes(ctx, "example.com")
	if err != nil || len(attrs) != 2 || attrs[0].GetFqn() != "https://example.com/attr/a" || attrs[1].GetId() != "a2" {
		t.Fatal(attrs, err)
	}
	for fqn, want := range map[string]bool{"https://example.com/attr/a": true, "https://example.com/attr/missing": false} {
		if got, err := s.AttributeExists(ctx, fqn); err != nil || got != want {
			t.Fatal(fqn, got, err)
		}
	}
	if got, err := s.AttributeValueExists(ctx, "https://example.com/attr/a/value/1"); err != nil || !got {
		t.Fatal(got, err)
	}
	if got, err := s.AttributeValueExists(ctx, "https://example.com/attr/a/value/missing"); err != nil || got {
		t.Fatal(got, err)
	}
	if err := s.ValidateAttributes(ctx, []string{"https://example.com/attr/a/value/1"}); err != nil {
		t.Fatal(err)
	}
	err = s.ValidateAttributes(ctx, []string{"https://example.com/attr/a/value/1", "https://example.com/attr/a/value/missing"})
	var e *Error
	if !errors.Is(err, ErrAttributeNotFound) || !errors.As(err, &e) || e.Kind != KindInvalidInput || e.Op != "ValidateAttributes" || e.Message != "attribute not found: https://example.com/attr/a/value/missing" {
		t.Fatalf("%#v", err)
	}
	f.mode.Store("registry-error")
	_, err = s.ListAttributes(ctx)
	f.mode.Store("")
	if !errors.As(err, &e) || e.Kind != KindInvalidInput || e.ConnectCode != "not_found" || e.HTTPStatus != 404 || e.Op != "ListAttributes" || e.Message != "listing attributes: not_found: no attributes" {
		t.Fatalf("%#v", err)
	}
	fqns, err := s.GetEntityAttributesJSON(ctx, `{"id":"e1","clientId":"opentdf-sdk"}`)
	if err != nil || len(fqns) != 1 || fqns[0] != "https://example.com/attr/attr1/value/opentdf-sdk" {
		t.Fatal(fqns, err)
	}
	f.mu.Lock()
	auth := f.lastAuth
	f.mu.Unlock()
	if auth != "Bearer access-token-Bearer" {
		t.Fatal("authorization", auth)
	}

	if _, err := s.AttributeExists(ctx, "https://example.com/attr"); err == nil || !strings.Contains(err.Error(), `invalid attribute FQN "https://example.com/attr"`) {
		t.Fatal(err)
	}
}

// WithCustom*Connection: the core endpoint replaces the platform endpoint for
// every service, and the per-service endpoints replace it for their own
// services, all through the SDK's authenticated transport.
func TestCustomServiceEndpoints(t *testing.T) {
	core := newFakePlatform(t)
	policy := newFakePlatform(t)
	ers := newFakePlatform(t)
	ctx := context.Background()

	dead := "http://127.0.0.1:1"
	opts := core.options()
	opts.CoreEndpoint = core.server.URL
	opts.ValidateConnection = true
	s, err := New(ctx, dead, opts)
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()
	if s.Conn().Endpoint != core.server.URL {
		t.Fatal("conn", s.Conn())
	}
	if ok, err := s.AttributeExists(ctx, "https://example.com/attr/a"); err != nil || !ok {
		t.Fatal(ok, err)
	}
	if healthy, err := s.IsHealthy(ctx); err != nil || !healthy {
		t.Fatal(healthy, err)
	}
	if _, err := s.Call(ctx, "entityresolution.v2.EntityResolutionService", "CreateEntityChainsFromTokens", "{}"); err == nil || !strings.Contains(err.Error(), "unimplemented") {
		t.Fatal(err)
	}
	served := strings.Join(core.served(), " ")
	for _, path := range []string{"/grpc.health.v1.Health/Check", "/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration", "/policy.attributes.AttributesService/GetAttribute", "/entityresolution.v2.EntityResolutionService/CreateEntityChainsFromTokens"} {
		if !strings.Contains(served, path) {
			t.Fatal("core endpoint did not serve", path, served)
		}
	}

	opts = core.options()
	opts.PolicyEndpoint = policy.server.URL
	opts.EntityResolutionEndpoint = ers.server.URL
	opts.WellknownEndpoint = policy.server.URL
	s, err = New(ctx, core.server.URL, opts)
	if err != nil {
		t.Fatal(err)
	}
	defer s.Close()
	core.served()
	if s.Conn().Endpoint != core.server.URL {
		t.Fatal("conn", s.Conn())
	}
	if ok, err := s.AttributeValueExists(ctx, "https://example.com/attr/a/value/1"); err != nil || !ok {
		t.Fatal(ok, err)
	}
	if _, err := s.GetEntityAttributesJSON(ctx, `{"id":"e1","clientId":"c"}`); err != nil {
		t.Fatal(err)
	}
	s.Call(ctx, "entityresolution.EntityResolutionService", "ResolveEntities", "{}")
	if _, err := s.Call(ctx, "wellknownconfiguration.WellKnownService", "GetWellKnownConfiguration", "{}"); err != nil {
		t.Fatal(err)
	}
	if got := strings.Join(core.served(), " "); got != "/authorization.AuthorizationService/GetEntitlements" {
		t.Fatal("core served", got)
	}
	// The well-known configuration, and so the issuer, comes from the policy fake.
	if got := strings.Join(policy.served(), " "); got != "/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration /auth/realms/opentdf/.well-known/openid-configuration /auth/realms/opentdf/protocol/openid-connect/token /policy.attributes.AttributesService/GetAttributeValuesByFqns /wellknownconfiguration.WellKnownService/GetWellKnownConfiguration" {
		t.Fatal("policy served", got)
	}
	if got := strings.Join(ers.served(), " "); got != "/entityresolution.EntityResolutionService/ResolveEntities" {
		t.Fatal("entity resolution served", got)
	}

	opts = core.options()
	opts.AuthorizationEndpoint = "localhost:8080"
	if _, err := New(ctx, core.server.URL, opts); err == nil || err.Error() != "platform endpoint is malformed [localhost:8080]" {
		t.Fatal(err)
	}
}
