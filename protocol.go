package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/clock"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"opentdf-local/sdk/tdf"
	j "opentdf-local/sdk/tdf/json"
)

func jsonLimits() j.Limits {
	return j.Limits{Bytes: 1 << 20, Depth: 20, Nodes: 10000, StringBytes: 1 << 20}
}
func object(names []string, values []j.Value) j.Value { return j.Obj(names, values) }
func text(v j.Value, name string) string {
	x, ok := v.Get(name)
	if !ok || x.Kind != j.String {
		return ""
	}
	return x.Text
}
func integerString(n int64) string {
	if n == 0 {
		return "0"
	}
	b := make([]byte, 0, 20)
	for n > 0 {
		b = append(b, byte(n%10)+'0')
		n /= 10
	}
	for i := 0; i < len(b)/2; i++ {
		b[i], b[len(b)-1-i] = b[len(b)-1-i], b[i]
	}
	return string(b)
}

// signRequestToken uses an exact JSON STRING requestBody, current iat and a
// sixty-second expiry. ES256 signatures are the capability's raw 64-byte JOSE form.
func signRequestToken(key *crypto.Key, algorithm string, body []byte) (string, error) {
	now := clock.Unix()
	if now <= 0 {
		return "", failure("srt", "clock", nil)
	}
	header, e := j.Marshal(object([]string{"alg", "typ"}, []j.Value{j.Str(algorithm), j.Str("JWT")}), jsonLimits())
	if e != nil {
		return "", e
	}
	claims, e := j.Marshal(object([]string{"requestBody", "iat", "exp"}, []j.Value{j.Str(string(body)), j.Num(integerString(now)), j.Num(integerString(now + 60))}), jsonLimits())
	if e != nil {
		return "", e
	}
	return signJWT(key, algorithm, header, claims)
}
func signJWT(key *crypto.Key, algorithm string, header, claims []byte) (string, error) {
	h, e := encoding.Base64URLEncode(header)
	if e != nil {
		return "", e
	}
	p, e := encoding.Base64URLEncode(claims)
	if e != nil {
		return "", e
	}
	input := h + "." + p
	var sig []byte
	if algorithm == "RS256" {
		sig, e = crypto.RS256Sign(key, []byte(input))
	} else if algorithm == "ES256" {
		sig, e = crypto.ES256Sign(key, []byte(input))
	} else {
		return "", failure("srt", "unsupported_algorithm", nil)
	}
	if e != nil {
		return "", failure("srt", "signing", e)
	}
	s, e := encoding.Base64URLEncode(sig)
	if e != nil {
		return "", e
	}
	return input + "." + s, nil
}
func rewrapBody(m tdf.Manifest, public string) ([]byte, error) {
	k := m.Encryption.KeyAccess[0]
	names := []string{"type", "url", "protocol", "wrappedKey", "policyBinding"}
	values := []j.Value{j.Str(k.Type), j.Str(k.URL), j.Str(k.Protocol), j.Str(k.WrappedKey), object([]string{"alg", "hash"}, []j.Value{j.Str(k.Binding.Algorithm), j.Str(k.Binding.Hash)})}
	if k.EphemeralPublicKey != "" {
		names = append(names, "ephemeralPublicKey")
		values = append(values, j.Str(k.EphemeralPublicKey))
	}
	if k.KID != "" {
		names = append(names, "kid")
		values = append(values, j.Str(k.KID))
	}
	if k.SID != "" {
		names = append(names, "sid")
		values = append(values, j.Str(k.SID))
	}
	if k.EncryptedMetadata != "" {
		names = append(names, "encryptedMetadata")
		values = append(values, j.Str(k.EncryptedMetadata))
	}
	kao := object(names, values)
	policy := object([]string{"id", "body"}, []j.Value{j.Str("policy"), j.Str(m.Encryption.Policy)})
	entry := object([]string{"keyAccessObjectId", "keyAccessObject"}, []j.Value{j.Str("kao-0"), kao})
	group := object([]string{"policy", "keyAccessObjects"}, []j.Value{policy, j.Arr([]j.Value{entry})})
	return j.Marshal(object([]string{"clientPublicKey", "requests"}, []j.Value{j.Str(public), j.Arr([]j.Value{group})}), jsonLimits())
}
func keysAllowed(v j.Value, names []string) bool {
	if v.Kind != j.Object {
		return false
	}
	for _, n := range v.Names {
		found := false
		for _, allowed := range names {
			if n == allowed {
				found = true
			}
		}
		if !found {
			return false
		}
	}
	return true
}
func parseRewrap(v j.Value, algorithm string) ([]byte, error) {
	if !wrappingAlgorithm(algorithm) {
		return nil, failure("rewrap", "unsupported_session_algorithm", nil)
	}
	if !keysAllowed(v, []string{"responses", "sessionPublicKey"}) {
		return nil, failure("rewrap", "unsupported_response", nil)
	}
	public, hasPublic := v.Get("sessionPublicKey")
	if algorithm == "rsa:2048" && hasPublic && (public.Kind != j.String || public.Text != "") {
		return nil, failure("rewrap", "unsupported_response", nil)
	}
	groups, ok := v.Get("responses")
	if !ok || groups.Kind != j.Array || len(groups.Children) != 1 {
		return nil, failure("rewrap", "response_groups", nil)
	}
	g := groups.Children[0]
	if !keysAllowed(g, []string{"policyId", "results"}) || text(g, "policyId") != "policy" {
		return nil, failure("rewrap", "response_policy", nil)
	}
	results, ok := g.Get("results")
	if !ok || results.Kind != j.Array || len(results.Children) != 1 {
		return nil, failure("rewrap", "response_results", nil)
	}
	r := results.Children[0]
	if !keysAllowed(r, []string{"keyAccessObjectId", "status", "kasWrappedKey", "error", "metadata"}) || text(r, "keyAccessObjectId") != "kao-0" {
		return nil, failure("rewrap", "response_id", nil)
	}
	metadata, has := r.Get("metadata")
	if has {
		if !keysAllowed(metadata, []string{"X-Required-Obligations"}) {
			return nil, failure("rewrap", "unsupported_metadata", nil)
		}
		obligations, exists := metadata.Get("X-Required-Obligations")
		if exists {
			if obligations.Kind != j.Array {
				return nil, failure("rewrap", "invalid_obligations", nil)
			}
			required := []string{}
			for _, o := range obligations.Children {
				if o.Kind != j.String || o.Text == "" {
					return nil, failure("rewrap", "invalid_obligations", nil)
				}
				required = append(required, o.Text)
			}
			if len(required) > 0 {
				return nil, &Error{Code: "required_obligations", Operation: "rewrap", RequiredObligations: required}
			}
		}
	}
	_, hasError := r.Get("error")
	_, hasKey := r.Get("kasWrappedKey")
	if text(r, "status") == "fail" {
		if hasKey || !hasError || text(r, "error") == "" {
			return nil, failure("rewrap", "invalid_result", nil)
		}
		return nil, &Error{Operation: "rewrap", Code: "rewrap_failed", ServerMessage: text(r, "error")}
	}
	if text(r, "status") != "permit" || hasError || !hasKey {
		return nil, failure("rewrap", "invalid_result", nil)
	}
	key, e := encoding.Base64Decode(text(r, "kasWrappedKey"))
	size := 256
	if algorithm == "ec:secp256r1" {
		size = 60
	}
	if e != nil || len(key) != size {
		return nil, failure("rewrap", "wrapped_key", e)
	}
	if algorithm == "ec:secp256r1" {
		if !hasPublic || public.Kind != j.String || public.Text == "" {
			return nil, failure("rewrap", "session_public_key", nil)
		}
	}
	return key, nil
}
