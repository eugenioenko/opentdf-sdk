package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"opentdf-local/sdk/src/protocol/policy"
)

// Should match:
// https://github.com/opentdf/platform/blob/main/service/wellknownconfiguration/wellknown_configuration.go#L25
const (
	baseKeyWellKnown = "base_key"
	baseKeyAlg       = "algorithm"
	baseKeyPublicKey = "public_key"
)

// Policy algorithm names as protojson spells the policy.Algorithm enum: the
// exported helpers and SimpleKasKey carry these across the Goalchemy
// boundary, where the generated enum type does not cross.
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
	// Algorithm is the policy algorithm enum's JSON name, such as ALGORITHM_RSA_2048.
	Algorithm string `json:"algorithm"`
	// Kid identifies this public key at the KAS.
	Kid string `json:"kid"`
	// Pem contains the PEM public key.
	Pem string `json:"pem"`
}

// SimpleKasKey is the policy.SimpleKasKey message.
type SimpleKasKey struct {
	// KasURI is the KAS endpoint associated with PublicKey.
	KasURI string `json:"kasUri"`
	// PublicKey contains the key material, identifier, and algorithm.
	PublicKey SimpleKasPublicKey `json:"publicKey"`
	// KasID is the platform registry identifier for the KAS.
	KasID string `json:"kasId"`
}

// KasPublicKey is the policy.KasPublicKey message.
type KasPublicKey struct {
	// Pem contains the PEM public key.
	Pem string `json:"pem"`
	// Kid identifies this public key at the KAS.
	Kid string `json:"kid"`
	// Alg is the policy KAS public-key algorithm enum's JSON name.
	Alg string `json:"alg"`
}

// KasPublicKeySet is the policy.KasPublicKeySet message.
type KasPublicKeySet struct {
	// Keys lists the public keys to store in the SDK's KAS cache.
	Keys []KasPublicKey `json:"keys"`
}

func parseKeyType(alg string) (KeyType, error) {
	switch KeyType(alg) {
	case RSA2048Key, RSA4096Key, EC256Key, EC384Key, EC521Key,
		mlkem768Key, mlkem1024Key,
		hybridXWingKey, hybridSecp256r1MLKEM768Key, hybridSecp384r1MLKEM1024Key:
		return KeyType(alg), nil
	default:
		return "", fmt.Errorf("unrecognized key type: %s", alg)
	}
}

func getKasKeyAlg(alg string) (policy.Algorithm, error) {
	kt, err := parseKeyType(alg)
	if err != nil {
		return policy.Algorithm_ALGORITHM_UNSPECIFIED, fmt.Errorf("invalid alg [%s]: %w", alg, err)
	}
	return keyTypeToPolicyAlgorithm(kt)
}

// KeyTypeToPolicyAlgorithm returns the policy.Algorithm for a key type, by
// its protojson name.
func KeyTypeToPolicyAlgorithm(kt KeyType) (string, error) {
	alg, err := keyTypeToPolicyAlgorithm(kt)
	if err != nil {
		return AlgorithmUnspecified, boundaryError(KindInvalidInput, "KeyTypeToPolicyAlgorithm", err)
	}
	return alg.String(), nil
}

// PolicyAlgorithmToKeyType returns the key type of a policy.Algorithm given
// by its protojson name.
func PolicyAlgorithmToKeyType(alg string) (KeyType, error) {
	value, ok := policy.Algorithm_value[alg]
	if !ok {
		return "", newError(KindInvalidInput, "PolicyAlgorithmToKeyType", "unsupported algorithm: "+alg)
	}
	kt, err := policyAlgorithmToKeyType(policy.Algorithm(value))
	if err != nil {
		return "", boundaryError(KindInvalidInput, "PolicyAlgorithmToKeyType", err)
	}
	return kt, nil
}

func keyTypeToPolicyAlgorithm(kt KeyType) (policy.Algorithm, error) {
	switch kt {
	case RSA2048Key:
		return policy.Algorithm_ALGORITHM_RSA_2048, nil
	case RSA4096Key:
		return policy.Algorithm_ALGORITHM_RSA_4096, nil
	case EC256Key:
		return policy.Algorithm_ALGORITHM_EC_P256, nil
	case EC384Key:
		return policy.Algorithm_ALGORITHM_EC_P384, nil
	case EC521Key:
		return policy.Algorithm_ALGORITHM_EC_P521, nil
	case hybridXWingKey:
		return policy.Algorithm_ALGORITHM_HPQT_XWING, nil
	case hybridSecp256r1MLKEM768Key:
		return policy.Algorithm_ALGORITHM_HPQT_SECP256R1_MLKEM768, nil
	case hybridSecp384r1MLKEM1024Key:
		return policy.Algorithm_ALGORITHM_HPQT_SECP384R1_MLKEM1024, nil
	case mlkem768Key:
		return policy.Algorithm_ALGORITHM_MLKEM_768, nil
	case mlkem1024Key:
		return policy.Algorithm_ALGORITHM_MLKEM_1024, nil
	default:
		return policy.Algorithm_ALGORITHM_UNSPECIFIED, fmt.Errorf("unknown key type: %s", string(kt))
	}
}

func policyAlgorithmToKeyType(alg policy.Algorithm) (KeyType, error) {
	switch alg {
	case policy.Algorithm_ALGORITHM_RSA_2048:
		return RSA2048Key, nil
	case policy.Algorithm_ALGORITHM_RSA_4096:
		return RSA4096Key, nil
	case policy.Algorithm_ALGORITHM_EC_P256:
		return EC256Key, nil
	case policy.Algorithm_ALGORITHM_EC_P384:
		return EC384Key, nil
	case policy.Algorithm_ALGORITHM_EC_P521:
		return EC521Key, nil
	case policy.Algorithm_ALGORITHM_HPQT_XWING:
		return hybridXWingKey, nil
	case policy.Algorithm_ALGORITHM_HPQT_SECP256R1_MLKEM768:
		return hybridSecp256r1MLKEM768Key, nil
	case policy.Algorithm_ALGORITHM_HPQT_SECP384R1_MLKEM1024:
		return hybridSecp384r1MLKEM1024Key, nil
	case policy.Algorithm_ALGORITHM_MLKEM_768:
		return mlkem768Key, nil
	case policy.Algorithm_ALGORITHM_MLKEM_1024:
		return mlkem1024Key, nil
	case policy.Algorithm_ALGORITHM_UNSPECIFIED:
		fallthrough
	default:
		return "", fmt.Errorf("unsupported algorithm: %s", alg)
	}
}

func formatAlg(alg policy.Algorithm) (string, error) {
	kt, err := policyAlgorithmToKeyType(alg)
	return string(kt), err
}

// policyAlgorithmOf is the policy.Algorithm named by its protojson name, as
// SimpleKasKey carries it.
func policyAlgorithmOf(name string) policy.Algorithm {
	return policy.Algorithm(policy.Algorithm_value[name])
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
			Algorithm: a.String(),
			Kid:       publicKey.Get("kid").String(),
			Pem:       publicKey.Get("pem").String(),
		},
	}, nil
}
