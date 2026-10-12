package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/errors"
)

// signJWT signs header.claims as a compact JWS with the given algorithm. ES*
// signatures are the capability's raw JOSE form.
func signJWT(key *crypto.Key, algorithm SigningAlgorithm, header, claims jsonvalue.Value) (string, error) {
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
	var sig []byte
	switch algorithm {
	case RS256:
		sig, err = crypto.RS256Sign(key, []byte(input))
	case RS384:
		sig, err = crypto.RS384Sign(key, []byte(input))
	case RS512:
		sig, err = crypto.RS512Sign(key, []byte(input))
	case ES256:
		sig, err = crypto.ES256Sign(key, []byte(input))
	case ES384:
		sig, err = crypto.ES384Sign(key, []byte(input))
	case ES512:
		sig, err = crypto.ES512Sign(key, []byte(input))
	default:
		return "", errors.New("unsupported signing algorithm: " + string(algorithm))
	}
	if err != nil {
		return "", err
	}
	s, err := encoding.Base64URLEncode(sig)
	if err != nil {
		return "", err
	}
	return input + "." + s, nil
}

// publicJWK returns the RFC 7638 required public members of key in lexical
// order, as the DPoP proof header and thumbprint need them.
func publicJWK(key *crypto.Key) (jsonvalue.Value, error) {
	k, err := key.PublicJWK()
	if err != nil {
		return jsonvalue.Value{}, err
	}
	if len(k) != 6 {
		return jsonvalue.Value{}, errors.New("unexpected JWK shape")
	}
	if k[0] == "RSA" {
		return jsonvalue.Object("e", jsonvalue.String(k[3]), "kty", jsonvalue.String(k[0]), "n", jsonvalue.String(k[2])), nil
	}
	return jsonvalue.Object("crv", jsonvalue.String(k[1]), "kty", jsonvalue.String(k[0]), "x", jsonvalue.String(k[4]), "y", jsonvalue.String(k[5])), nil
}

// jwkThumbprint is the RFC 7638 SHA-256 thumbprint of key, base64url.
func jwkThumbprint(key *crypto.Key) (string, error) {
	jwk, err := publicJWK(key)
	if err != nil {
		return "", err
	}
	data, err := jsonvalue.Encode(jwk)
	if err != nil {
		return "", err
	}
	hash, err := crypto.SHA256(data)
	if err != nil {
		return "", err
	}
	return encoding.Base64URLEncode(hash)
}

// signingAlgorithmForKey infers the DPoP algorithm from the key type:
// EC P-256 → ES256, P-384 → ES384, P-521 → ES512, RSA → RS256.
func signingAlgorithmForKey(key *crypto.Key) (SigningAlgorithm, error) {
	k, err := key.PublicJWK()
	if err != nil {
		return "", err
	}
	if len(k) != 6 {
		return "", errors.New("unexpected JWK shape")
	}
	switch {
	case k[0] == "RSA":
		return RS256, nil
	case k[1] == "P-256":
		return ES256, nil
	case k[1] == "P-384":
		return ES384, nil
	case k[1] == "P-521":
		return ES512, nil
	}
	return "", errors.New("unsupported key type " + k[0] + " for DPoP; only EC and RSA keys are supported")
}

func integerText(n int64) string {
	if n == 0 {
		return "0"
	}
	negative := n < 0
	if negative {
		n = -n
	}
	b := make([]byte, 0, 20)
	for n > 0 {
		b = append(b, byte(n%10)+'0')
		n /= 10
	}
	if negative {
		b = append(b, '-')
	}
	for i := 0; i < len(b)/2; i++ {
		b[i], b[len(b)-1-i] = b[len(b)-1-i], b[i]
	}
	return string(b)
}

func base64Text(s string) ([]byte, error) { return encoding.Base64Decode(s) }
