package sdk

import (
	"context"
	"errors"
	"fmt"
	"reflect"
	"strings"
	"testing"

	"opentdf-local/sdk/src/protocol/authorization"
	"opentdf-local/sdk/src/protocol/policy"
	"opentdf-local/sdk/src/protocol/policy/attributes"
)

// Ported from the reference's discovery_test.go: the helpers over test
// doubles of the attributes and authorization services.

type mockDiscoveryAttributesClient struct {
	listAttributesFunc           func(ctx context.Context, req *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error)
	getAttributeValuesByFqnsFunc func(ctx context.Context, req *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error)
	getAttributeFunc             func(ctx context.Context, req *attributes.GetAttributeRequest) (*attributes.GetAttributeResponse, error)
}

func (m *mockDiscoveryAttributesClient) ListAttributes(ctx context.Context, req *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
	return m.listAttributesFunc(ctx, req)
}

func (m *mockDiscoveryAttributesClient) GetAttributeValuesByFqns(ctx context.Context, req *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
	return m.getAttributeValuesByFqnsFunc(ctx, req)
}

func (m *mockDiscoveryAttributesClient) GetAttribute(ctx context.Context, req *attributes.GetAttributeRequest) (*attributes.GetAttributeResponse, error) {
	return m.getAttributeFunc(ctx, req)
}

type mockDiscoveryAuthzClient struct {
	getEntitlementsFunc func(ctx context.Context, req *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error)
}

func (m *mockDiscoveryAuthzClient) GetEntitlements(ctx context.Context, req *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error) {
	return m.getEntitlementsFunc(ctx, req)
}

func newDiscoverySDK(attrClient attributesService, authzClient authorizationService) *SDK {
	return &SDK{attributes: attrClient, authorization: authzClient}
}

func makeAttr(fqn string) *policy.Attribute {
	return &policy.Attribute{Fqn: fqn}
}

func fqnMap(fqns ...string) map[string]*attributes.GetAttributeValuesByFqnsResponse_AttributeAndValue {
	m := make(map[string]*attributes.GetAttributeValuesByFqnsResponse_AttributeAndValue, len(fqns))
	for _, f := range fqns {
		m[f] = &attributes.GetAttributeValuesByFqnsResponse_AttributeAndValue{}
	}
	return m
}

// connectError is the SDK's Error for a Connect failure, as the service
// connection returns it: connect.NewError(code, errors.New(message)).
func connectError(code, message string) error {
	kind := KindTransport
	if code == "not_found" {
		kind = KindInvalidInput
	}
	return &Error{Kind: kind, Message: code + ": " + message, ConnectCode: code, ServiceMessage: message}
}

func requireErrorContains(t *testing.T, err error, texts ...string) {
	t.Helper()
	if err == nil {
		t.Fatal("expected an error")
	}
	for _, text := range texts {
		if !strings.Contains(err.Error(), text) {
			t.Fatalf("error %q does not contain %q", err.Error(), text)
		}
	}
}

// --- ListAttributes tests ---

func TestListAttributes_Empty(t *testing.T) {
	attrClient := &mockDiscoveryAttributesClient{
		listAttributesFunc: func(_ context.Context, _ *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
			return &attributes.ListAttributesResponse{}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	result, err := s.ListAttributes(t.Context())
	if err != nil || len(result) != 0 {
		t.Fatal(result, err)
	}
}

func TestListAttributes_SinglePage(t *testing.T) {
	expected := []*policy.Attribute{
		makeAttr("https://example.com/attr/level/value/high"),
		makeAttr("https://example.com/attr/level/value/low"),
	}
	attrClient := &mockDiscoveryAttributesClient{
		listAttributesFunc: func(_ context.Context, _ *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
			return &attributes.ListAttributesResponse{Attributes: expected}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	result, err := s.ListAttributes(t.Context())
	if err != nil || !reflect.DeepEqual(expected, result) {
		t.Fatal(result, err)
	}
}

func TestListAttributes_MultiPage(t *testing.T) {
	page1 := []*policy.Attribute{makeAttr("https://example.com/attr/a/value/1")}
	page2 := []*policy.Attribute{makeAttr("https://example.com/attr/b/value/2")}

	calls := 0
	attrClient := &mockDiscoveryAttributesClient{
		listAttributesFunc: func(_ context.Context, req *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
			calls++
			if req.GetPagination().GetOffset() == 0 {
				return &attributes.ListAttributesResponse{
					Attributes: page1,
					Pagination: &policy.PageResponse{NextOffset: 1},
				}, nil
			}
			return &attributes.ListAttributesResponse{
				Attributes: page2,
				Pagination: &policy.PageResponse{NextOffset: 0},
			}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	result, err := s.ListAttributes(t.Context())
	if err != nil {
		t.Fatal(err)
	}
	if calls != 2 {
		t.Fatal("should have paginated twice", calls)
	}
	if !reflect.DeepEqual(append(page1, page2...), result) {
		t.Fatal(result)
	}
}

func TestListAttributes_NamespaceFilter(t *testing.T) {
	var capturedReq *attributes.ListAttributesRequest
	attrClient := &mockDiscoveryAttributesClient{
		listAttributesFunc: func(_ context.Context, req *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
			capturedReq = req
			return &attributes.ListAttributesResponse{}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	if _, err := s.ListAttributes(t.Context(), "my-namespace"); err != nil {
		t.Fatal(err)
	}
	if capturedReq.GetNamespace() != "my-namespace" {
		t.Fatal(capturedReq.GetNamespace())
	}
}

func TestListAttributes_TooManyNamespaces(t *testing.T) {
	s := newDiscoverySDK(nil, nil)

	_, err := s.ListAttributes(t.Context(), "a", "b")
	requireErrorContains(t, err, "ListAttributes accepts at most one namespace filter, got 2")
}

func TestListAttributes_PageLimitExceeded(t *testing.T) {
	attrClient := &mockDiscoveryAttributesClient{
		listAttributesFunc: func(_ context.Context, _ *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
			// Always return a non-zero next_offset to simulate a runaway server.
			return &attributes.ListAttributesResponse{
				Attributes: []*policy.Attribute{makeAttr("https://example.com/attr/a/value/1")},
				Pagination: &policy.PageResponse{NextOffset: 1},
			}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	_, err := s.ListAttributes(t.Context())
	requireErrorContains(t, err, "exceeded maximum page limit")
}

func TestListAttributes_ServiceError(t *testing.T) {
	attrClient := &mockDiscoveryAttributesClient{
		listAttributesFunc: func(_ context.Context, _ *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
			return nil, errors.New("service unavailable")
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	_, err := s.ListAttributes(t.Context())
	requireErrorContains(t, err, "listing attributes", "service unavailable")
}

// --- AttributeExists tests ---

func TestAttributeExists_Found(t *testing.T) {
	attrFQN := "https://example.com/attr/department"
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeFunc: func(_ context.Context, req *attributes.GetAttributeRequest) (*attributes.GetAttributeResponse, error) {
			if req.GetFqn() != attrFQN {
				t.Fatal("request", req.GetFqn())
			}
			return &attributes.GetAttributeResponse{Attribute: &policy.Attribute{Fqn: attrFQN}}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	exists, err := s.AttributeExists(t.Context(), attrFQN)
	if err != nil || !exists {
		t.Fatal(exists, err)
	}
}

func TestAttributeExists_NotFound(t *testing.T) {
	attrFQN := "https://example.com/attr/nonexistent"
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeFunc: func(_ context.Context, _ *attributes.GetAttributeRequest) (*attributes.GetAttributeResponse, error) {
			return nil, connectError("not_found", "attribute not found")
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	exists, err := s.AttributeExists(t.Context(), attrFQN)
	if err != nil || exists {
		t.Fatal(exists, err)
	}
}

func TestAttributeExists_ServiceError(t *testing.T) {
	attrFQN := "https://example.com/attr/department"
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeFunc: func(_ context.Context, _ *attributes.GetAttributeRequest) (*attributes.GetAttributeResponse, error) {
			return nil, connectError("unavailable", "service unavailable")
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	exists, err := s.AttributeExists(t.Context(), attrFQN)
	if exists {
		t.Fatal("exists")
	}
	requireErrorContains(t, err, "checking attribute existence")
	var e *Error
	if !errors.As(err, &e) || e.ConnectCode != "unavailable" || e.Op != "AttributeExists" || e.Message != "checking attribute existence: unavailable: service unavailable" {
		t.Fatalf("%#v", err)
	}
}

func TestAttributeExists_InvalidFQN(t *testing.T) {
	s := newDiscoverySDK(nil, nil)

	exists, err := s.AttributeExists(t.Context(), "not-a-fqn")
	if exists {
		t.Fatal("exists")
	}
	requireErrorContains(t, err, "invalid attribute FQN")
	if !errors.Is(err, ErrInvalid) {
		t.Fatal(err)
	}
}

func TestAttributeExists_RejectsValueFQN(t *testing.T) {
	// A value FQN (with /value/) is not a valid attribute-level FQN.
	s := newDiscoverySDK(nil, nil)

	exists, err := s.AttributeExists(t.Context(), "https://example.com/attr/clearance/value/secret")
	if exists {
		t.Fatal("exists")
	}
	requireErrorContains(t, err, "invalid attribute FQN")
}

// --- AttributeValueExists tests ---

func TestAttributeValueExists_Found(t *testing.T) {
	fqn := "https://example.com/attr/department/value/finance"
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeValuesByFqnsFunc: func(_ context.Context, _ *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
			return &attributes.GetAttributeValuesByFqnsResponse{FqnAttributeValues: fqnMap(fqn)}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	exists, err := s.AttributeValueExists(t.Context(), fqn)
	if err != nil || !exists {
		t.Fatal(exists, err)
	}
}

func TestAttributeValueExists_NotFound(t *testing.T) {
	fqn := "https://example.com/attr/department/value/finance"
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeValuesByFqnsFunc: func(_ context.Context, _ *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
			return &attributes.GetAttributeValuesByFqnsResponse{FqnAttributeValues: fqnMap()}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	exists, err := s.AttributeValueExists(t.Context(), fqn)
	if err != nil || exists {
		t.Fatal(exists, err)
	}
}

func TestAttributeValueExists_NotFoundError(t *testing.T) {
	fqn := "https://example.com/attr/department/value/finance"
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeValuesByFqnsFunc: func(_ context.Context, _ *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
			return nil, connectError("not_found", "resource not found")
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	exists, err := s.AttributeValueExists(t.Context(), fqn)
	if err != nil || exists {
		t.Fatal(exists, err)
	}
}

func TestAttributeValueExists_ServiceError(t *testing.T) {
	fqn := "https://example.com/attr/department/value/finance"
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeValuesByFqnsFunc: func(_ context.Context, _ *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
			return nil, errors.New("network error")
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	exists, err := s.AttributeValueExists(t.Context(), fqn)
	if exists {
		t.Fatal("exists")
	}
	requireErrorContains(t, err, "checking attribute value existence")
}

func TestAttributeValueExists_InvalidFQN(t *testing.T) {
	s := newDiscoverySDK(nil, nil)

	exists, err := s.AttributeValueExists(t.Context(), "not-a-fqn")
	if exists {
		t.Fatal("exists")
	}
	requireErrorContains(t, err, "invalid attribute value FQN")
}

func TestAttributeValueExists_RejectsAttributeFQN(t *testing.T) {
	// An attribute-level FQN (without /value/) is not a valid value FQN.
	s := newDiscoverySDK(nil, nil)

	exists, err := s.AttributeValueExists(t.Context(), "https://example.com/attr/department")
	if exists {
		t.Fatal("exists")
	}
	requireErrorContains(t, err, "invalid attribute value FQN")
}

// --- ValidateAttributes tests ---

func TestValidateAttributes_Empty(t *testing.T) {
	s := newDiscoverySDK(nil, nil)
	if err := s.ValidateAttributes(t.Context(), nil); err != nil {
		t.Fatal(err)
	}
}

func TestValidateAttributes_AllFound(t *testing.T) {
	fqns := []string{
		"https://example.com/attr/level/value/high",
		"https://example.com/attr/type/value/secret",
	}
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeValuesByFqnsFunc: func(_ context.Context, _ *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
			return &attributes.GetAttributeValuesByFqnsResponse{FqnAttributeValues: fqnMap(fqns...)}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	if err := s.ValidateAttributes(t.Context(), fqns); err != nil {
		t.Fatal(err)
	}
}

func TestValidateAttributes_SomeMissing(t *testing.T) {
	fqns := []string{
		"https://example.com/attr/level/value/high",
		"https://example.com/attr/type/value/missing",
	}
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeValuesByFqnsFunc: func(_ context.Context, _ *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
			return &attributes.GetAttributeValuesByFqnsResponse{
				FqnAttributeValues: fqnMap(fqns[0]),
			}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	err := s.ValidateAttributes(t.Context(), fqns)
	if !errors.Is(err, ErrAttributeNotFound) {
		t.Fatal(err)
	}
	requireErrorContains(t, err, "https://example.com/attr/type/value/missing")
	if err.Error() != "attribute not found: https://example.com/attr/type/value/missing" {
		t.Fatal(err)
	}
}

func TestValidateAttributes_AllMissing(t *testing.T) {
	fqns := []string{
		"https://example.com/attr/a/value/x",
		"https://example.com/attr/b/value/y",
	}
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeValuesByFqnsFunc: func(_ context.Context, _ *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
			return &attributes.GetAttributeValuesByFqnsResponse{FqnAttributeValues: fqnMap()}, nil
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	err := s.ValidateAttributes(t.Context(), fqns)
	if !errors.Is(err, ErrAttributeNotFound) {
		t.Fatal(err)
	}
	if err.Error() != "attribute not found: https://example.com/attr/a/value/x, https://example.com/attr/b/value/y" {
		t.Fatal(err)
	}
}

func TestValidateAttributes_TooManyFQNs(t *testing.T) {
	fqns := make([]string, maxValidateFQNs+1)
	for i := range fqns {
		fqns[i] = fmt.Sprintf("https://example.com/attr/level/value/v%d", i)
	}
	s := newDiscoverySDK(nil, nil)

	err := s.ValidateAttributes(t.Context(), fqns)
	requireErrorContains(t, err, "too many attribute FQNs")
}

func TestValidateAttributes_InvalidFQNFormat(t *testing.T) {
	s := newDiscoverySDK(nil, nil)

	err := s.ValidateAttributes(t.Context(), []string{"not-a-valid-fqn"})
	requireErrorContains(t, err, "invalid attribute value FQN", "not-a-valid-fqn")
	if err.Error() != `invalid attribute value FQN "not-a-valid-fqn": invalid type: attribute regex fail for [not-a-valid-fqn]` {
		t.Fatal(err)
	}
}

func TestValidateAttributes_ServiceError(t *testing.T) {
	fqns := []string{"https://example.com/attr/level/value/high"}
	attrClient := &mockDiscoveryAttributesClient{
		getAttributeValuesByFqnsFunc: func(_ context.Context, _ *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
			return nil, errors.New("network error")
		},
	}
	s := newDiscoverySDK(attrClient, nil)

	err := s.ValidateAttributes(t.Context(), fqns)
	requireErrorContains(t, err, "validating attributes", "network error")
}

// --- GetEntityAttributes tests ---

func TestGetEntityAttributes_NilEntity(t *testing.T) {
	s := newDiscoverySDK(nil, nil)
	_, err := s.GetEntityAttributes(t.Context(), nil)
	requireErrorContains(t, err, "entity must not be nil")
}

func TestGetEntityAttributes_Found(t *testing.T) {
	expectedFQNs := []string{
		"https://example.com/attr/clearance/value/secret",
		"https://example.com/attr/country/value/us",
	}
	authzClient := &mockDiscoveryAuthzClient{
		getEntitlementsFunc: func(_ context.Context, req *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error) {
			if len(req.GetEntities()) != 1 {
				t.Fatal("entities", len(req.GetEntities()))
			}
			return &authorization.GetEntitlementsResponse{
				Entitlements: []*authorization.EntityEntitlements{
					{EntityId: "e1", AttributeValueFqns: expectedFQNs},
				},
			}, nil
		},
	}
	s := newDiscoverySDK(nil, authzClient)

	entity := &authorization.Entity{
		Id:         "e1",
		EntityType: &authorization.Entity_EmailAddress{EmailAddress: "alice@example.com"},
	}
	result, err := s.GetEntityAttributes(t.Context(), entity)
	if err != nil || !reflect.DeepEqual(expectedFQNs, result) {
		t.Fatal(result, err)
	}
}

func TestGetEntityAttributes_NoEntitlements(t *testing.T) {
	authzClient := &mockDiscoveryAuthzClient{
		getEntitlementsFunc: func(_ context.Context, _ *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error) {
			return &authorization.GetEntitlementsResponse{}, nil
		},
	}
	s := newDiscoverySDK(nil, authzClient)

	entity := &authorization.Entity{
		Id:         "e1",
		EntityType: &authorization.Entity_ClientId{ClientId: "my-service"},
	}
	result, err := s.GetEntityAttributes(t.Context(), entity)
	if err != nil || len(result) != 0 {
		t.Fatal(result, err)
	}
}

func TestGetEntityAttributes_IDMismatch(t *testing.T) {
	authzClient := &mockDiscoveryAuthzClient{
		getEntitlementsFunc: func(_ context.Context, _ *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error) {
			return &authorization.GetEntitlementsResponse{
				Entitlements: []*authorization.EntityEntitlements{
					{EntityId: "other-entity", AttributeValueFqns: []string{"https://example.com/attr/a/value/x"}},
				},
			}, nil
		},
	}
	s := newDiscoverySDK(nil, authzClient)

	entity := &authorization.Entity{
		Id:         "e1",
		EntityType: &authorization.Entity_EmailAddress{EmailAddress: "alice@example.com"},
	}
	result, err := s.GetEntityAttributes(t.Context(), entity)
	if err != nil || len(result) != 0 {
		t.Fatal("should return empty when no entitlement matches the requested entity ID", result, err)
	}
}

func TestGetEntityAttributes_EmptyEntityID(t *testing.T) {
	authzClient := &mockDiscoveryAuthzClient{
		getEntitlementsFunc: func(_ context.Context, _ *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error) {
			return &authorization.GetEntitlementsResponse{
				Entitlements: []*authorization.EntityEntitlements{
					{EntityId: "some-entity", AttributeValueFqns: []string{"https://example.com/attr/a/value/x"}},
				},
			}, nil
		},
	}
	s := newDiscoverySDK(nil, authzClient)

	result, err := s.GetEntityAttributes(t.Context(), &authorization.Entity{})
	if err != nil || len(result) != 0 {
		t.Fatal("entity with empty ID should not receive entitlements belonging to another entity", result, err)
	}
}

func TestGetEntityAttributes_ServiceError(t *testing.T) {
	authzClient := &mockDiscoveryAuthzClient{
		getEntitlementsFunc: func(_ context.Context, _ *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error) {
			return nil, errors.New("auth service unavailable")
		},
	}
	s := newDiscoverySDK(nil, authzClient)

	entity := &authorization.Entity{
		Id:         "e1",
		EntityType: &authorization.Entity_Uuid{Uuid: "550e8400-e29b-41d4-a716-446655440000"},
	}
	_, err := s.GetEntityAttributes(t.Context(), entity)
	requireErrorContains(t, err, "getting entity attributes", "auth service unavailable")
}

// --- The JSON boundary of the typed helpers ---

func TestDiscoveryJSON(t *testing.T) {
	active := true
	attrClient := &mockDiscoveryAttributesClient{
		listAttributesFunc: func(_ context.Context, req *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
			if req.GetNamespace() != "example.com" {
				t.Fatal("namespace", req.GetNamespace())
			}
			return &attributes.ListAttributesResponse{Attributes: []*policy.Attribute{{
				Id: "a1", Name: "level", Fqn: "https://example.com/attr/level", Active: &active,
				Rule:   policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_HIERARCHY,
				Values: []*policy.Value{{Id: "v1", Value: "high", Fqn: "https://example.com/attr/level/value/high"}},
			}}}, nil
		},
	}
	var entity *authorization.Entity
	authzClient := &mockDiscoveryAuthzClient{
		getEntitlementsFunc: func(_ context.Context, req *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error) {
			entity = req.GetEntities()[0]
			return &authorization.GetEntitlementsResponse{Entitlements: []*authorization.EntityEntitlements{{EntityId: "e1", AttributeValueFqns: []string{"https://example.com/attr/level/value/high"}}}}, nil
		},
	}
	s := newDiscoverySDK(attrClient, authzClient)

	attrs, err := s.ListAttributesJSON(t.Context(), []string{"example.com"})
	if err != nil || len(attrs) != 1 {
		t.Fatal(attrs, err)
	}
	want := `{"id":"a1","name":"level","rule":"ATTRIBUTE_RULE_TYPE_ENUM_HIERARCHY","values":[{"id":"v1","value":"high","fqn":"https://example.com/attr/level/value/high"}],"fqn":"https://example.com/attr/level","active":true}`
	if attrs[0] != want {
		t.Fatalf("attribute JSON\n%s\n%s", attrs[0], want)
	}

	fqns, err := s.GetEntityAttributesJSON(t.Context(), `{"id":"e1","clientId":"opentdf-sdk"}`)
	if err != nil || !reflect.DeepEqual(fqns, []string{"https://example.com/attr/level/value/high"}) {
		t.Fatal(fqns, err)
	}
	if entity.GetId() != "e1" || entity.GetClientId() != "opentdf-sdk" {
		t.Fatalf("entity %#v", entity)
	}
	if _, err := s.GetEntityAttributesJSON(t.Context(), ""); err == nil || err.Error() != "entity must not be nil" {
		t.Fatal(err)
	}
	var e *Error
	if _, err := s.GetEntityAttributesJSON(t.Context(), `{"id":`); !errors.As(err, &e) || e.Kind != KindInvalidInput {
		t.Fatal(err)
	}
}

// The reference's granter_test.go FQN cases, against the regexp-equivalent
// parsers.
func TestAttributeFromURL(t *testing.T) {
	for _, tc := range []struct {
		n, u       string
		auth, name string
	}{
		{"letter", "http://e/attr/a", "http://e", "a"},
		{"number", "http://e/attr/1", "http://e", "1"},
		{"emoji", "http://e/attr/%F0%9F%98%81", "http://e", "%F0%9F%98%81"},
		{"dash", "http://a-b.com/attr/b-c", "http://a-b.com", "b-c"},
		{"nested", "https://e.com/x/attr/y/attr/a", "https://e.com/x/attr/y", "a"},
	} {
		t.Run(tc.n, func(t *testing.T) {
			a, err := newAttributeNameFQN(tc.u)
			if err != nil || a.String() != tc.u {
				t.Fatal(a, err)
			}
			m := matchAttributeNameFQN(tc.u)
			if m[1] != tc.auth || m[2] != tc.name {
				t.Fatal(m)
			}
		})
	}
}

func TestAttributeFromMalformedURL(t *testing.T) {
	for _, tc := range []struct {
		n, u string
	}{
		{"no name", "http://e/attr"},
		{"invalid prefix 1", "hxxp://e/attr/a"},
		{"invalid prefix 2", "e/attr/a"},
		{"invalid prefix 3", "file://e/attr/a"},
		{"invalid prefix 4", "http:///attr/a"},
		{"bad encoding", "https://a/attr/%😁"},
		{"with value", "http://e/attr/a/value/b"},
		{"empty name", "http://e/attr/"},
		{"space", "http://e/attr/a b"},
	} {
		t.Run(tc.n, func(t *testing.T) {
			a, err := newAttributeNameFQN(tc.u)
			if !errors.Is(err, ErrInvalid) || a.String() != "" {
				t.Fatal(a, err)
			}
		})
	}
}

func TestAttributeValueFromURL(t *testing.T) {
	for _, tc := range []struct {
		n, u              string
		auth, name, value string
	}{
		{"number", "http://e/attr/a/value/1", "http://e", "a", "1"},
		{"space", "http://e/attr/a/value/%20", "http://e", "a", "%20"},
		{"emoji", "http://e/attr/a/value/%F0%9F%98%81", "http://e", "a", "%F0%9F%98%81"},
		{"numberdef", "http://e/attr/1/value/one", "http://e", "1", "one"},
		{"valuevalue", "http://e/attr/value/value/one", "http://e", "value", "one"},
		{"dash", "http://a-b.com/attr/b-c/value/c-d", "http://a-b.com", "b-c", "c-d"},
	} {
		t.Run(tc.n, func(t *testing.T) {
			a, err := newAttributeValueFQN(tc.u)
			if err != nil || a.String() != tc.u {
				t.Fatal(a, err)
			}
			m := matchAttributeValueFQN(tc.u)
			if m[1] != tc.auth || m[2] != tc.name || m[3] != tc.value {
				t.Fatal(m)
			}
		})
	}
}

func TestAttributeValueFromMalformedURL(t *testing.T) {
	for _, tc := range []struct {
		n, u, want string
	}{
		{"no name", "http://e/attr/value/1", "invalid type: attribute regex fail for [http://e/attr/value/1]"},
		{"no value", "http://e/attr/who/value", "invalid type: attribute regex fail for [http://e/attr/who/value]"},
		{"invalid prefix 1", "hxxp://e/attr/a/value/1", "invalid type: attribute regex fail for [hxxp://e/attr/a/value/1]"},
		{"invalid prefix 2", "e/attr/a/a/value/1", "invalid type: attribute regex fail for [e/attr/a/a/value/1]"},
		{"bad encoding", "https://a/attr/emoji/value/%😁", "invalid type: error in attribute value [%😁]"},
		{"empty name", "http://e/attr//value/0", "invalid type: empty url path parts are not allowed"},
		{"slash name", "http://e/attr///value/0", "invalid type: slash not allowed in name or values"},
		{"slash in name", "http://0/attr/0/0/value/0", "invalid type: slash not allowed in name or values"},
		{"empty value", "http://e/attr/a/value/", "invalid type: empty url path parts are not allowed"},
	} {
		t.Run(tc.n, func(t *testing.T) {
			a, err := newAttributeValueFQN(tc.u)
			if !errors.Is(err, ErrInvalid) || a.String() != "" || err.Error() != tc.want {
				t.Fatal(a, err)
			}
		})
	}
}
