package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/encoding/protojson"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/strings"
	"opentdf-local/sdk/src/protocol/authorization"
	"opentdf-local/sdk/src/protocol/policy"
	"opentdf-local/sdk/src/protocol/policy/attributes"
)

const (
	// maxListAttributesPages caps the pagination loop in ListAttributes to prevent
	// unbounded memory growth if a server repeatedly returns a non-zero next_offset.
	maxListAttributesPages = 1000

	// maxValidateFQNs matches the server-side limit on GetAttributeValuesByFqns
	// so callers get a clear local error instead of a cryptic server rejection.
	maxValidateFQNs = 250
)

// discoveryError is the reference's fmt.Errorf("<prefix>: %w", err) as the
// helper's Error: the service's kind and Connect fields survive, and any
// other failure is a transport failure.
func discoveryError(op, prefix string, err error) *Error {
	e := wrapError("", op, prefix, err)
	if e.Kind == "" {
		e.Kind = KindTransport
	}
	e.Op = op
	return e
}

// ListAttributes returns all active attributes available on the platform, auto-paginating
// through all results. An optional namespace name or ID may be provided to filter results.
//
// Use this before calling CreateTDF() to see what attributes are available for data tagging.
func (s *SDK) ListAttributes(ctx context.Context, namespace ...string) ([]*policy.Attribute, error) {
	if err := s.check("ListAttributes"); err != nil {
		return nil, err
	}
	if len(namespace) > 1 {
		return nil, newError(KindInvalidInput, "ListAttributes", fmt.Sprintf("ListAttributes accepts at most one namespace filter, got %d", len(namespace)))
	}
	req := &attributes.ListAttributesRequest{}
	if len(namespace) == 1 {
		req.Namespace = namespace[0]
	}

	var result []*policy.Attribute
	for pages := 0; pages < maxListAttributesPages; pages++ {
		resp, err := s.attributes.ListAttributes(ctx, req)
		if err != nil {
			return nil, discoveryError("ListAttributes", "listing attributes", err)
		}
		result = append(result, resp.GetAttributes()...)

		nextOffset := resp.GetPagination().GetNextOffset()
		if nextOffset == 0 {
			return result, nil
		}
		req.Pagination = &policy.PageRequest{Offset: nextOffset}
	}
	return nil, newError(KindTransport, "ListAttributes", fmt.Sprintf("listing attributes: exceeded maximum page limit (%d)", maxListAttributesPages))
}

// AttributeExists reports whether the attribute definition identified by attributeFqn
// exists on the platform.
//
// attributeFqn should be an attribute-level FQN (no /value/ segment):
//
//	https://<namespace>/attr/<attribute_name>
//
// Returns (true, nil) if the attribute exists, (false, nil) if it does not,
// and (false, err) if a service error occurs.
func (s *SDK) AttributeExists(ctx context.Context, attributeFqn string) (bool, error) {
	if err := s.check("AttributeExists"); err != nil {
		return false, err
	}
	if _, err := NewAttributeNameFQN(attributeFqn); err != nil {
		return false, newError(KindInvalidInput, "AttributeExists", fmt.Sprintf("invalid attribute FQN %q: %v", attributeFqn, err))
	}

	_, err := s.attributes.GetAttribute(ctx, &attributes.GetAttributeRequest{
		Identifier: &attributes.GetAttributeRequest_Fqn{Fqn: attributeFqn},
	})
	if err != nil {
		if connectCodeOf(err) == "not_found" {
			return false, nil
		}
		return false, discoveryError("AttributeExists", "checking attribute existence", err)
	}
	return true, nil
}

// AttributeValueExists reports whether the attribute value FQN exists on the platform.
//
// fqn should be a full attribute value FQN (with /value/ segment):
//
//	https://<namespace>/attr/<attribute_name>/value/<value>
//
// Returns (true, nil) if the value exists, (false, nil) if it does not,
// and (false, err) if a service error occurs.
func (s *SDK) AttributeValueExists(ctx context.Context, fqn string) (bool, error) {
	if err := s.check("AttributeValueExists"); err != nil {
		return false, err
	}
	if _, err := NewAttributeValueFQN(fqn); err != nil {
		return false, newError(KindInvalidInput, "AttributeValueExists", fmt.Sprintf("invalid attribute value FQN %q: %v", fqn, err))
	}

	resp, err := s.attributes.GetAttributeValuesByFqns(ctx, &attributes.GetAttributeValuesByFqnsRequest{
		Fqns: []string{fqn},
	})
	if err != nil {
		if connectCodeOf(err) == "not_found" {
			return false, nil
		}
		return false, discoveryError("AttributeValueExists", "checking attribute value existence", err)
	}

	_, found := resp.GetFqnAttributeValues()[fqn]
	return found, nil
}

// ValidateAttributes checks that all provided attribute value FQNs exist on the platform.
// This provides fail-fast behavior: validate attributes before calling CreateTDF() to avoid
// late-stage decryption failures caused by missing or misspelled attributes.
//
// fqns should be full attribute value FQNs in the form:
//
//	https://<namespace>/attr/<attribute_name>/value/<value>
//
// Returns ErrAttributeNotFound if any FQNs are missing, with the missing FQNs listed in
// the error message. The reference takes the FQNs variadically; a slice
// crosses the Goalchemy boundary.
func (s *SDK) ValidateAttributes(ctx context.Context, fqns []string) error {
	if err := s.check("ValidateAttributes"); err != nil {
		return err
	}
	if len(fqns) == 0 {
		return nil
	}

	if len(fqns) > maxValidateFQNs {
		return newError(KindInvalidInput, "ValidateAttributes", fmt.Sprintf("too many attribute FQNs: %d exceeds maximum of %d", len(fqns), maxValidateFQNs))
	}

	for _, fqn := range fqns {
		if _, err := NewAttributeValueFQN(fqn); err != nil {
			return newError(KindInvalidInput, "ValidateAttributes", fmt.Sprintf("invalid attribute value FQN %q: %v", fqn, err))
		}
	}

	resp, err := s.attributes.GetAttributeValuesByFqns(ctx, &attributes.GetAttributeValuesByFqnsRequest{
		Fqns: fqns,
	})
	if err != nil {
		return discoveryError("ValidateAttributes", "validating attributes", err)
	}

	found := resp.GetFqnAttributeValues()
	var missing []string
	for _, fqn := range fqns {
		if _, ok := found[fqn]; !ok {
			missing = append(missing, fqn)
		}
	}
	if len(missing) > 0 {
		return newError(KindInvalidInput, "ValidateAttributes", fmt.Sprintf("%v: %s", ErrAttributeNotFound, strings.Join(missing, ", ")))
	}
	return nil
}

// GetEntityAttributes returns the attribute value FQNs assigned to an entity (PE or NPE).
// Use this to inspect what attributes a user, service account, or other entity has been
// granted before making authorization decisions or constructing access policies.
//
// Returns a slice of attribute value FQNs (e.g., "https://example.com/attr/clearance/value/secret").
func (s *SDK) GetEntityAttributes(ctx context.Context, entity *authorization.Entity) ([]string, error) {
	if err := s.check("GetEntityAttributes"); err != nil {
		return nil, err
	}
	if entity == nil {
		return nil, newError(KindInvalidInput, "GetEntityAttributes", "entity must not be nil")
	}

	resp, err := s.authorization.GetEntitlements(ctx, &authorization.GetEntitlementsRequest{
		Entities: []*authorization.Entity{entity},
	})
	if err != nil {
		return nil, discoveryError("GetEntityAttributes", "getting entity attributes", err)
	}

	// GetEntitlements returns a slice of EntityEntitlements keyed by entity ID.
	// Even though we only request one entity, we must match by ID to locate the
	// correct entry — the response slice position is not guaranteed to correspond
	// to the request slice position.
	entityID := entity.GetId()
	for _, e := range resp.GetEntitlements() {
		if e.GetEntityId() == entityID {
			return e.GetAttributeValueFqns(), nil
		}
	}
	return nil, nil
}

// ListAttributesJSON is ListAttributes across the Goalchemy boundary: each
// attribute is a policy.Attribute in protobuf JSON, which the hosts decode
// with their own generated messages.
func (s *SDK) ListAttributesJSON(ctx context.Context, namespace []string) ([]string, error) {
	attrs, err := s.ListAttributes(ctx, namespace...)
	if err != nil {
		return nil, err
	}
	out := make([]string, 0, len(attrs))
	for _, a := range attrs {
		data, err := protojson.Marshal(a)
		if err != nil {
			return nil, boundaryError(KindTransport, "ListAttributes", err)
		}
		out = append(out, string(data))
	}
	return out, nil
}

// GetEntityAttributesJSON is GetEntityAttributes across the Goalchemy
// boundary: entity is an authorization.Entity in protobuf JSON, and an
// empty entity is the reference's nil.
func (s *SDK) GetEntityAttributesJSON(ctx context.Context, entity string) ([]string, error) {
	if strings.TrimSpace(entity) == "" {
		return s.GetEntityAttributes(ctx, nil)
	}
	e := &authorization.Entity{}
	if err := protojson.Unmarshal([]byte(entity), e); err != nil {
		return nil, wrapError(KindInvalidInput, "GetEntityAttributes", "entity is not an authorization.Entity in protobuf JSON", err)
	}
	return s.GetEntityAttributes(ctx, e)
}
