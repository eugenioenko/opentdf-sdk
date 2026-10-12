package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/lib/sync"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/errors"
)

// validDPoPAlgs is the single source of truth for the JWS algorithms permitted
// for DPoP proofs (RFC 9449 §4.2, restricted to the asymmetric families we
// support).
var validDPoPAlgs = []SigningAlgorithm{ES256, ES384, ES512, RS256, RS384, RS512}

// Valid reports whether a is a SigningAlgorithm supported for DPoP proofs.
func (a SigningAlgorithm) Valid() bool {
	for _, v := range validDPoPAlgs {
		if v == a {
			return true
		}
	}
	return false
}

const dpopAllowedAlgs = "ES256, ES384, ES512, RS256, RS384, RS512"

// dpopSigner holds the DPoP key for the lifetime of an SDK: it signs the
// token-endpoint and resource proofs, binds the access token through the
// JWK thumbprint, and remembers the nonce each origin last issued.
type dpopSigner struct {
	key        *crypto.Key
	algorithm  SigningAlgorithm
	thumbprint string
	mu         sync.Mutex
	nonces     map[string]string
}

func newDPoPSigner(key *crypto.Key, algorithm SigningAlgorithm) (*dpopSigner, error) {
	thumbprint, err := jwkThumbprint(key)
	if err != nil {
		return nil, err
	}
	return &dpopSigner{key: key, algorithm: algorithm, thumbprint: thumbprint, nonces: map[string]string{}}, nil
}

func (d *dpopSigner) close() {
	if d != nil && d.key != nil {
		d.key.Close()
		d.key = nil
	}
}

// generateDPoPKeyForAlg generates an ephemeral DPoP private key for the given algorithm.
// Supported algorithms: ES256, ES384, ES512, RS256, RS384, RS512.
func generateDPoPKeyForAlg(alg SigningAlgorithm) (*crypto.Key, error) {
	switch alg {
	case ES256:
		return crypto.GenerateP256()
	case ES384:
		return crypto.GenerateP384()
	case ES512:
		return crypto.GenerateP521()
	case RS256, RS384, RS512:
		return crypto.GenerateRSA2048()
	}
	return nil, errors.New("unsupported DPoP algorithm \"" + string(alg) + "\"; allowed: " + dpopAllowedAlgs)
}

// loadDPoPKeyFromPEM parses a PEM-encoded private key. The DPoP algorithm is
// inferred from the key type: EC P-256 → ES256, P-384 → ES384, P-521 → ES512,
// RSA → RS256.
func loadDPoPKeyFromPEM(pem string) (*crypto.Key, SigningAlgorithm, error) {
	key, err := crypto.ImportPEM(pem)
	if err != nil {
		return nil, "", errors.New("failed to parse DPoP key PEM: " + err.Error())
	}
	if _, err := key.PrivatePEM(); err != nil {
		key.Close()
		return nil, "", errors.New("DPoP key PEM must contain private signing material; a public-only key cannot sign proofs")
	}
	alg, err := signingAlgorithmForKey(key)
	if err != nil {
		key.Close()
		return nil, "", err
	}
	return key, alg, nil
}

// validateDPoPKey ensures the algorithm family matches the key type and, for
// EC keys, that the curve matches the ES algorithm.
func validateDPoPKey(key *crypto.Key, alg SigningAlgorithm) error {
	if !alg.Valid() {
		return errors.New("unsupported DPoP JWK algorithm \"" + string(alg) + "\"; allowed: " + dpopAllowedAlgs)
	}
	inferred, err := signingAlgorithmForKey(key)
	if err != nil {
		return err
	}
	switch alg {
	case RS256, RS384, RS512:
		if inferred != RS256 {
			return errors.New("DPoP algorithm \"" + string(alg) + "\" requires an RSA key")
		}
	default:
		if inferred == RS256 {
			return errors.New("DPoP algorithm \"" + string(alg) + "\" requires an EC key, got key type \"RSA\"")
		}
		if inferred != alg {
			return errors.New("DPoP algorithm \"" + string(alg) + "\" does not match EC key curve (expected \"" + string(inferred) + "\" for this curve)")
		}
	}
	return nil
}

// resolveDPoPKey returns the DPoP signer for the config by the reference's
// fixed priority: a PEM key (with an optional algorithm override), an
// algorithm alone (fresh ephemeral key), else nil, nil.
func resolveDPoPKey(c *config) (*dpopSigner, error) {
	switch {
	case c.dpopKeyPEM != "":
		key, alg, err := loadDPoPKeyFromPEM(c.dpopKeyPEM)
		if err != nil {
			return nil, errors.New("failed to load DPoP key from PEM: " + err.Error())
		}
		if c.dpopAlgorithm != "" {
			if !c.dpopAlgorithm.Valid() {
				key.Close()
				return nil, errors.New("invalid DPoP algorithm override \"" + string(c.dpopAlgorithm) + "\"; allowed: " + dpopAllowedAlgs)
			}
			alg = c.dpopAlgorithm
		}
		if err := validateDPoPKey(key, alg); err != nil {
			key.Close()
			return nil, err
		}
		return newDPoPSigner(key, alg)
	case c.dpopAlgorithm != "":
		key, err := generateDPoPKeyForAlg(c.dpopAlgorithm)
		if err != nil {
			return nil, errors.New("failed to generate DPoP key: " + err.Error())
		}
		return newDPoPSigner(key, c.dpopAlgorithm)
	}
	return nil, nil
}

func (d *dpopSigner) cachedNonce(origin string) string {
	d.mu.Lock()
	value := d.nonces[origin]
	d.mu.Unlock()
	return value
}

func (d *dpopSigner) cacheNonce(origin, nonce string) {
	d.mu.Lock()
	d.nonces[origin] = nonce
	d.mu.Unlock()
}

// proof builds a DPoP proof JWT (RFC 9449 §4.2) for method and htu. token is
// hashed into ath for resource requests; nonce is the server's last nonce.
func (d *dpopSigner) proof(clock Clock, method, htu, token, nonce string) (string, error) {
	jwk, err := publicJWK(d.key)
	if err != nil {
		return "", err
	}
	random, err := crypto.Random(32)
	if err != nil {
		return "", err
	}
	id, err := encoding.Base64URLEncode(random)
	if err != nil {
		return "", err
	}
	header := jsonvalue.Object("alg", jsonvalue.String(string(d.algorithm)), "typ", jsonvalue.String("dpop+jwt"), "jwk", jwk)
	claims := jsonvalue.Object("jti", jsonvalue.String(id), "iat", jsonvalue.Int(clock.Now()), "htm", jsonvalue.String(method), "htu", jsonvalue.String(htu))
	if token != "" {
		hash, err := crypto.SHA256([]byte(token))
		if err != nil {
			return "", err
		}
		ath, err := encoding.Base64URLEncode(hash)
		if err != nil {
			return "", err
		}
		claims = claims.Set("ath", jsonvalue.String(ath))
	}
	if nonce != "" {
		claims = claims.Set("nonce", jsonvalue.String(nonce))
	}
	return signJWT(d.key, d.algorithm, header, claims)
}
