package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/std/encoding/hex"
	"github.com/eugenioenko/goalchemy/std/encoding/json/jcs"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/strings"
	"github.com/eugenioenko/goalchemy/std/time"
)

const (
	SystemMetadataAssertionID = "system-metadata"
	SystemMetadataSchemaV1    = "system-metadata-v1"

	kAssertionSignature = "assertionSig"
	kAssertionHash      = "assertionHash"
)

var errAssertionVerifyKeyFailure = errors.New("assertion: failed to verify with provided key")

// AssertionKey is a key for signing or verifying an assertion binding. The
// reference holds key material as interface{}; here Key is text so it crosses
// the Goalchemy boundary: the raw HS256 secret, or a PEM block for RS256 (a
// private key to sign, a public or private key to verify).
type AssertionKey struct {
	Alg AssertionKeyAlg
	Key string
}

// Algorithm returns the algorithm of the key.
func (k AssertionKey) Algorithm() AssertionKeyAlg { return k.Alg }

// IsEmpty returns true if the key and the algorithm are empty.
func (k AssertionKey) IsEmpty() bool { return k.Key == "" && k.Alg == "" }

// AssertionKeyEntry names the verification key of one assertion ID.
type AssertionKeyEntry struct {
	AssertionID string
	Alg         AssertionKeyAlg
	Key         string
}

// AssertionVerificationKeys holds the reader's verification keys: the
// reference's map of assertion ID to key is a list of entries here, because
// maps do not cross the Goalchemy boundary.
type AssertionVerificationKeys struct {
	DefaultKey AssertionKey
	Keys       []AssertionKeyEntry
}

// Get returns the key for the given assertion ID or the default key if the
// key is not found. If the default key is not set, it returns an empty key.
func (k AssertionVerificationKeys) Get(assertionID string) (AssertionKey, error) {
	for _, e := range k.Keys {
		if e.AssertionID == assertionID {
			return AssertionKey{Alg: e.Alg, Key: e.Key}, nil
		}
	}
	if k.DefaultKey.IsEmpty() {
		return AssertionKey{}, nil
	}
	return k.DefaultKey, nil
}

// IsEmpty returns true if the default key and the keys are empty.
func (k AssertionVerificationKeys) IsEmpty() bool {
	return k.DefaultKey.IsEmpty() && len(k.Keys) == 0
}

// signAssertions builds and signs the manifest assertion list. Each assertion
// is bound to the payload by signing the aggregate hash of all segment hashes
// followed by the assertion's own hash; pre-4.3.0 writers append the hex form
// of that hash rather than its raw bytes, which useHex selects. Assertions
// are signed with HS256 over defaultKey unless the config supplies a key.
func signAssertions(aggregateHash []byte, configs []AssertionConfig, defaultKey []byte, useHex bool) ([]Assertion, error) {
	var signed []Assertion
	for _, assertion := range configs {
		tmpAssertion := Assertion{
			ID:             assertion.ID,
			Type:           assertion.Type,
			Scope:          assertion.Scope,
			Statement:      assertion.Statement,
			AppliesToState: assertion.AppliesToState,
		}
		hashOfAssertionAsHex, err := tmpAssertion.GetHash()
		if err != nil {
			return nil, err
		}
		hashOfAssertion, err := hex.DecodeString(string(hashOfAssertionAsHex))
		if err != nil {
			return nil, fmt.Errorf("error decoding hex string: %w", err)
		}
		completeHash := make([]byte, 0, len(aggregateHash)+len(hashOfAssertionAsHex))
		completeHash = append(completeHash, aggregateHash...)
		if useHex {
			completeHash = append(completeHash, hashOfAssertionAsHex...)
		} else {
			completeHash = append(completeHash, hashOfAssertion...)
		}
		encoded, err := encoding.Base64Encode(completeHash)
		if err != nil {
			return nil, err
		}
		assertionSigningKey := AssertionKey{Alg: AssertionKeyAlgHS256, Key: string(defaultKey)}
		if !assertion.SigningKey.IsEmpty() {
			assertionSigningKey = assertion.SigningKey
		}
		if err := tmpAssertion.Sign(string(hashOfAssertionAsHex), encoded, assertionSigningKey); err != nil {
			return nil, fmt.Errorf("failed to sign assertion: %w", err)
		}
		signed = append(signed, tmpAssertion)
	}
	return signed, nil
}

// Sign signs the assertion with the given hash and signature using the key,
// and sets the binding to the resulting JWS.
func (a *Assertion) Sign(hash, sig string, key AssertionKey) error {
	claims := jsonvalue.Object(kAssertionHash, jsonvalue.String(hash), kAssertionSignature, jsonvalue.String(sig))
	token, err := signAssertionJWT(key, claims)
	if err != nil {
		return fmt.Errorf("signing assertion failed: %w", err)
	}
	a.Binding.Method = string(JWS)
	a.Binding.Signature = token
	return nil
}

// Verify checks the binding signature of the assertion with key and returns
// the hash and signature claims. The JWS header must name the key's
// algorithm; a key carried in the header (jwk, x5c) is not consulted.
func (a Assertion) Verify(key AssertionKey) (string, string, error) {
	claims, err := verifyAssertionJWT(key, a.Binding.Signature)
	if err != nil {
		return "", "", fmt.Errorf("%w: %w", errAssertionVerifyKeyFailure, err)
	}
	hashClaim, found := claims.Lookup(kAssertionHash)
	if !found {
		return "", "", errors.New("hash claim not found")
	}
	hash, ok := hashClaim.AsString()
	if !ok {
		return "", "", errors.New("hash claim is not a string")
	}
	sigClaim, found := claims.Lookup(kAssertionSignature)
	if !found {
		return "", "", errors.New("signature claim not found")
	}
	sig, ok := sigClaim.AsString()
	if !ok {
		return "", "", errors.New("signature claim is not a string")
	}
	return hash, sig, nil
}

// GetHash returns the SHA-256 of the assertion's RFC 8785 canonical JSON,
// without its binding, in hex.
func (a Assertion) GetHash() ([]byte, error) {
	statement := jsonvalue.Object()
	if a.Statement.Format != "" {
		statement = statement.Set("format", jsonvalue.String(a.Statement.Format))
	}
	if a.Statement.Schema != "" {
		statement = statement.Set("schema", jsonvalue.String(a.Statement.Schema))
	}
	if a.Statement.Value != "" {
		statement = statement.Set("value", jsonvalue.String(a.Statement.Value))
	}
	object := jsonvalue.Object("id", jsonvalue.String(a.ID), "type", jsonvalue.String(string(a.Type)), "scope", jsonvalue.String(string(a.Scope)))
	if a.AppliesToState != "" {
		object = object.Set("appliesToState", jsonvalue.String(string(a.AppliesToState)))
	}
	object = object.Set("statement", statement)
	transformed, err := jcs.Encode(object)
	if err != nil {
		return nil, fmt.Errorf("jcs.Transform failed: %w", err)
	}
	sum, err := crypto.SHA256(transformed)
	if err != nil {
		return nil, err
	}
	return []byte(hex.EncodeToString(sum)), nil
}

// signAssertionJWT returns the compact JWS of claims under key: HS256 over the
// raw secret, or RS256 with a PEM private key.
func signAssertionJWT(key AssertionKey, claims jsonvalue.Value) (string, error) {
	header := jsonvalue.Object("alg", jsonvalue.String(string(key.Alg)), "typ", jsonvalue.String("JWT"))
	switch key.Alg {
	case AssertionKeyAlgHS256:
		h, err := jsonvalue.Encode(header)
		if err != nil {
			return "", err
		}
		c, err := jsonvalue.Encode(claims)
		if err != nil {
			return "", err
		}
		eh, err := encoding.Base64URLEncode(h)
		if err != nil {
			return "", err
		}
		ec, err := encoding.Base64URLEncode(c)
		if err != nil {
			return "", err
		}
		input := eh + "." + ec
		mac, err := crypto.HMACSHA256([]byte(key.Key), []byte(input))
		if err != nil {
			return "", err
		}
		s, err := encoding.Base64URLEncode(mac)
		if err != nil {
			return "", err
		}
		return input + "." + s, nil
	case AssertionKeyAlgRS256:
		k, err := crypto.ImportPEM(key.Key)
		if err != nil {
			return "", fmt.Errorf("RS256 signing key: %w", err)
		}
		defer k.Close()
		return signJWT(k, RS256, header, claims)
	}
	return "", errors.New("unsupported assertion key algorithm: " + string(key.Alg))
}

// verifyAssertionJWT checks the compact JWS token under key and returns its
// claims.
func verifyAssertionJWT(key AssertionKey, token string) (jsonvalue.Value, error) {
	parts := strings.Split(token, ".")
	if len(parts) != 3 {
		return jsonvalue.Value{}, errors.New("invalid compact serialization format: invalid number of segments")
	}
	headerJSON, err := encoding.Base64URLDecode(parts[0])
	if err != nil {
		return jsonvalue.Value{}, fmt.Errorf("failed to decode protected headers: %w", err)
	}
	header, err := jsonvalue.Parse(headerJSON)
	if err != nil {
		return jsonvalue.Value{}, fmt.Errorf("failed to parse protected headers: %w", err)
	}
	if alg := header.Get("alg").String(); alg != string(key.Alg) {
		return jsonvalue.Value{}, errors.New("could not verify message using any of the signatures or keys")
	}
	sig, err := encoding.Base64URLDecode(parts[2])
	if err != nil {
		return jsonvalue.Value{}, fmt.Errorf("failed to decode signature: %w", err)
	}
	input := []byte(parts[0] + "." + parts[1])
	var valid bool
	switch key.Alg {
	case AssertionKeyAlgHS256:
		valid, err = crypto.HMACSHA256Verify([]byte(key.Key), input, sig)
	case AssertionKeyAlgRS256:
		k, err := crypto.ImportPEM(key.Key)
		if err != nil {
			return jsonvalue.Value{}, fmt.Errorf("RS256 verification key: %w", err)
		}
		defer k.Close()
		valid, err = crypto.RS256Verify(k, input, sig)
		if err != nil {
			return jsonvalue.Value{}, err
		}
	default:
		return jsonvalue.Value{}, errors.New("unsupported assertion key algorithm: " + string(key.Alg))
	}
	if err != nil {
		return jsonvalue.Value{}, err
	}
	if !valid {
		return jsonvalue.Value{}, errors.New("could not verify message using any of the signatures or keys")
	}
	claimsJSON, err := encoding.Base64URLDecode(parts[1])
	if err != nil {
		return jsonvalue.Value{}, fmt.Errorf("failed to decode payload: %w", err)
	}
	claims, err := jsonvalue.Parse(claimsJSON)
	if err != nil || claims.Kind() != jsonvalue.KindObject {
		return jsonvalue.Value{}, errors.New("failed to parse payload")
	}
	return claims, nil
}

// GetSystemMetadataAssertionConfig returns the default system-metadata
// assertion. The host operating system, architecture and runtime version
// are omitted until the runtime reports them (Goalchemy #74).
func GetSystemMetadataAssertionConfig() (AssertionConfig, error) {
	metadata := jsonvalue.Object(
		"tdf_spec_version", jsonvalue.String(TDFSpecVersion()),
		"creation_date", jsonvalue.String(time.Now().Format(time.RFC3339)),
		"sdk_version", jsonvalue.String("Goalchemy-"+Version()),
	)
	metadataJSON, err := jsonvalue.Encode(metadata)
	if err != nil {
		return AssertionConfig{}, fmt.Errorf("failed to marshal system metadata: %w", err)
	}
	return AssertionConfig{
		ID:             SystemMetadataAssertionID,
		Type:           BaseAssertion,
		Scope:          PayloadScope,
		AppliesToState: Unencrypted,
		Statement: Statement{
			Format: "json",
			Schema: SystemMetadataSchemaV1,
			Value:  string(metadataJSON),
		},
	}, nil
}

// assertionError is the reference's fmt.Errorf("%w: ...", ErrAssertionFailure{ID}).
func assertionError(id, detail string) *Error {
	e := newError(KindAssertionFailure, "Init", ErrAssertionFailure{ID: id}.Error()+": "+detail)
	e.AssertionID = id
	return e
}

// verifyAssertions checks every manifest assertion against the payload key
// and the reader's verification keys, as the reference reader does.
func (r *Reader) verifyAssertions(aggregateHash, payloadKey []byte) error {
	for _, assertion := range r.manifest.Assertions {
		assertionKey := AssertionKey{Alg: AssertionKeyAlgHS256, Key: string(payloadKey)}
		if !r.config.verifiers.IsEmpty() {
			foundKey, err := r.config.verifiers.Get(assertion.ID)
			if err != nil {
				return assertionError(assertion.ID, err.Error())
			}
			if !foundKey.IsEmpty() {
				assertionKey = foundKey
			}
		}
		assertionHash, assertionSig, err := assertion.Verify(assertionKey)
		if err != nil {
			if errors.Is(err, errAssertionVerifyKeyFailure) {
				e := newError(KindAssertionFailure, "Init", "assertion verification failed: "+err.Error())
				e.AssertionID = assertion.ID
				return e
			}
			return assertionError(assertion.ID, "assertion verification failed: "+err.Error())
		}
		hashOfAssertionAsHex, err := assertion.GetHash()
		if err != nil {
			return assertionError(assertion.ID, "failed to get hash of assertion: "+err.Error())
		}
		hashOfAssertion, err := hex.DecodeString(string(hashOfAssertionAsHex))
		if err != nil {
			return fmt.Errorf("error decoding hex string: %w", err)
		}
		if r.manifest.TDFVersion == "" {
			hashOfAssertion = hashOfAssertionAsHex
		}
		completeHash := make([]byte, 0, len(aggregateHash)+len(hashOfAssertion))
		completeHash = append(completeHash, aggregateHash...)
		completeHash = append(completeHash, hashOfAssertion...)
		base64Hash, err := encoding.Base64Encode(completeHash)
		if err != nil {
			return err
		}
		if string(hashOfAssertionAsHex) != assertionHash {
			return assertionError(assertion.ID, "assertion hash missmatch")
		}
		if assertionSig != base64Hash {
			return assertionError(assertion.ID, "failed integrity check on assertion signature")
		}
	}
	return nil
}
