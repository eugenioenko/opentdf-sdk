package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
)

// Should match:
// https://github.com/opentdf/platform/blob/main/service/wellknownconfiguration/wellknown_configuration.go#L25
const (
	baseKeyWellKnown = "base_key"
	baseKeyAlg       = "algorithm"
	baseKeyPublicKey = "public_key"
)

// Policy algorithm names as protojson spells the policy.Algorithm enum; the
// generated enum replaces them in Phase 3.
const (
	AlgorithmUnspecified = "ALGORITHM_UNSPECIFIED"
	AlgorithmRSA2048     = "ALGORITHM_RSA_2048"
	AlgorithmRSA4096     = "ALGORITHM_RSA_4096"
	AlgorithmECP256      = "ALGORITHM_EC_P256"
	AlgorithmECP384      = "ALGORITHM_EC_P384"
	AlgorithmECP521      = "ALGORITHM_EC_P521"
)

// SimpleKasPublicKey is the policy.SimpleKasPublicKey message.
type SimpleKasPublicKey struct {
	Algorithm string `json:"algorithm"`
	Kid       string `json:"kid"`
	Pem       string `json:"pem"`
}

// SimpleKasKey is the policy.SimpleKasKey message.
type SimpleKasKey struct {
	KasURI    string             `json:"kasUri"`
	PublicKey SimpleKasPublicKey `json:"publicKey"`
	KasID     string             `json:"kasId"`
}

// KasPublicKey is the policy.KasPublicKey message.
type KasPublicKey struct {
	Pem string `json:"pem"`
	Kid string `json:"kid"`
	Alg string `json:"alg"`
}

// KasPublicKeySet is the policy.KasPublicKeySet message.
type KasPublicKeySet struct {
	Keys []KasPublicKey `json:"keys"`
}

func parseKeyType(alg string) (KeyType, error) {
	switch KeyType(alg) {
	case RSA2048Key, RSA4096Key, EC256Key, EC384Key, EC521Key:
		return KeyType(alg), nil
	}
	return "", errors.New("unsupported key type: " + alg)
}

func getKasKeyAlg(alg string) (string, error) {
	kt, err := parseKeyType(alg)
	if err != nil {
		return AlgorithmUnspecified, fmt.Errorf("invalid alg [%s]: %w", alg, err)
	}
	return KeyTypeToPolicyAlgorithm(kt)
}

func KeyTypeToPolicyAlgorithm(kt KeyType) (string, error) {
	switch kt {
	case RSA2048Key:
		return AlgorithmRSA2048, nil
	case RSA4096Key:
		return AlgorithmRSA4096, nil
	case EC256Key:
		return AlgorithmECP256, nil
	case EC384Key:
		return AlgorithmECP384, nil
	case EC521Key:
		return AlgorithmECP521, nil
	default:
		return AlgorithmUnspecified, newError(KindInvalidInput, "KeyTypeToPolicyAlgorithm", "unknown key type: "+string(kt))
	}
}

func PolicyAlgorithmToKeyType(alg string) (KeyType, error) {
	switch alg {
	case AlgorithmRSA2048:
		return RSA2048Key, nil
	case AlgorithmRSA4096:
		return RSA4096Key, nil
	case AlgorithmECP256:
		return EC256Key, nil
	case AlgorithmECP384:
		return EC384Key, nil
	case AlgorithmECP521:
		return EC521Key, nil
	default:
		return "", newError(KindInvalidInput, "PolicyAlgorithmToKeyType", "unsupported algorithm: "+alg)
	}
}

func formatAlg(alg string) (string, error) {
	kt, err := PolicyAlgorithmToKeyType(alg)
	return string(kt), err
}

// GetBaseKey retrieves the platform base KAS key from the well-known configuration.
// The returned key material is expected to be public (algorithm, KID, PEM).
func (s *SDK) GetBaseKey(ctx context.Context) (SimpleKasKey, error) {
	key, err := getBaseKey(ctx, s)
	if err != nil {
		return SimpleKasKey{}, boundaryError(KindBaseKeyInvalid, "GetBaseKey", err)
	}
	return *key, nil
}

func getBaseKey(ctx context.Context, s *SDK) (*SimpleKasKey, error) {
	configuration, err := s.fetchWellKnownConfiguration(ctx, s.client)
	if err != nil {
		return nil, wrapError(KindTransport, "GetBaseKey", "unable to retrieve config information, and none was provided", err)
	}
	if configuration.Kind() != jsonvalue.KindObject || configuration.Len() == 0 {
		return nil, sentinelError(KindBaseKeyNotFound, ErrWellKnowConfigEmpty)
	}

	baseKeyStructure, ok := configuration.Lookup(baseKeyWellKnown)
	if !ok {
		return nil, sentinelError(KindBaseKeyNotFound, ErrBaseKeyNotFound)
	}

	if baseKeyStructure.Kind() != jsonvalue.KindObject {
		return nil, ErrBaseKeyInvalidFormat
	}

	simpleKasKey, err := parseSimpleKasKey(baseKeyStructure)
	if err != nil {
		return nil, err
	}

	return simpleKasKey, nil
}

func parseSimpleKasKey(baseKeyMap jsonvalue.Value) (*SimpleKasKey, error) {
	if baseKeyMap.Len() == 0 {
		return nil, ErrBaseKeyEmpty
	}

	publicKey, ok := baseKeyMap.Lookup(baseKeyPublicKey)
	if !ok || publicKey.Kind() != jsonvalue.KindObject {
		return nil, ErrBaseKeyInvalidFormat
	}

	alg, ok := publicKey.Get(baseKeyAlg).AsString()
	if !ok {
		return nil, ErrBaseKeyInvalidFormat
	}
	a, err := getKasKeyAlg(alg)
	if err != nil {
		return nil, fmt.Errorf("%w: %w", ErrMarshalBaseKeyFailed, err)
	}
	return &SimpleKasKey{
		KasURI: baseKeyMap.Get("kas_uri").String(),
		KasID:  baseKeyMap.Get("kas_id").String(),
		PublicKey: SimpleKasPublicKey{
			Algorithm: a,
			Kid:       publicKey.Get("kid").String(),
			Pem:       publicKey.Get("pem").String(),
		},
	}, nil
}
