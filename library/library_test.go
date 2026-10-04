package library

import (
	"errors"
	"github.com/eugenioenko/goalchemy/lib/context"
	"opentdf-local/sdk/src"
	"testing"
)

func TestTokenProviderWireRetainsExactExpiry(t *testing.T) {
	token, err := parseToken([]byte(`{"value":"opaque","scheme":"DPoP","expiresAt":"9223372036854775807","confirmationJKT":"matching"}`))
	if err != nil || token.ExpiresAt != 9223372036854775807 || token.ConfirmationJKT != "matching" {
		t.Fatal("exact expiry/binding", err)
	}
	for _, wire := range []string{`{"value":"x","scheme":"Bearer","expiresAt":9007199254740993}`, `{"value":"x","scheme":"Bearer","expiresAt":"9223372036854775808"}`, `{"value":"x","scheme":"Bearer","expiresAt":"1","extra":"x"}`, `{"value":"x","value":"y","scheme":"Bearer","expiresAt":"1"}`} {
		if _, err := parseToken([]byte(wire)); err == nil {
			t.Fatal("invalid provider wire accepted")
		}
	}
}
func TestFailurePreservesSDKFieldsAndCancellation(t *testing.T) {
	source := &sdk.Error{Code: "denied", Operation: "rewrap", HTTPStatus: 403, ServerCode: "permission_denied", ServerMessage: "bounded diagnostic", RequiredObligations: []string{"host obligation"}, Cause: context.Canceled}
	err := wrap("outer", source)
	failure, ok := err.(*Failure)
	if !ok || failure.Code != "denied" || failure.Operation != "rewrap" || failure.HTTPStatus != 403 || failure.ServerCode != "permission_denied" || failure.ServerMessage != "bounded diagnostic" || failure.CauseCategory != "canceled" || len(failure.RequiredObligations) != 1 {
		t.Fatal("typed fields")
	}
	source.RequiredObligations[0] = "mutated"
	if failure.RequiredObligations[0] != "host obligation" {
		t.Fatal("obligation ownership")
	}
	if errors.Is(err, context.Canceled) {
		t.Fatal("source wire failure promises cause identity")
	}
}
