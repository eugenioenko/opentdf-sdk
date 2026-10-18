package main

import (
	"encoding/json"
	"fmt"
	"os"

	g "goalchemyout"
)

// fileOrJSON returns the argument's bytes: the contents of the file it names,
// or the argument itself when it is JSON text, as otdfctl accepts both.
func fileOrJSON(arg string) ([]byte, error) {
	if json.Valid([]byte(arg)) {
		return []byte(arg), nil
	}
	data, err := os.ReadFile(arg)
	if err != nil {
		return nil, fmt.Errorf("%s is neither JSON nor a readable file: %w", arg, err)
	}
	return data, nil
}

type assertionKeyJSON struct {
	Alg string `json:"alg"`
	Key string `json:"key"`
}

type assertionJSON struct {
	ID             string `json:"id"`
	Type           string `json:"type"`
	Scope          string `json:"scope"`
	AppliesToState string `json:"appliesToState"`
	Statement      struct {
		Format string          `json:"format"`
		Schema string          `json:"schema"`
		Value  json.RawMessage `json:"value"`
	} `json:"statement"`
	SigningKey *assertionKeyJSON `json:"signingKey"`
}

// parseAssertions reads --with-assertions: a JSON list of assertion configs
// whose statement value is a string or an object (kept as compact JSON) and
// whose optional signingKey is {alg, key} with the HS256 secret or the RS256
// private key PEM as text.
func parseAssertions(arg string) ([]g.AssertionConfig, error) {
	data, err := fileOrJSON(arg)
	if err != nil {
		return nil, err
	}
	var list []assertionJSON
	if err := json.Unmarshal(data, &list); err != nil {
		return nil, fmt.Errorf("assertions must be a JSON list: %w", err)
	}
	configs := make([]g.AssertionConfig, 0, len(list))
	for _, a := range list {
		c := g.AssertionConfig{ID: a.ID, Type: g.AssertionType(a.Type), Scope: g.Scope(a.Scope), AppliesToState: g.AppliesToState(a.AppliesToState)}
		c.Statement.Format = a.Statement.Format
		c.Statement.Schema = a.Statement.Schema
		if len(a.Statement.Value) > 0 {
			var text string
			if json.Unmarshal(a.Statement.Value, &text) == nil {
				c.Statement.Value = text
			} else {
				var object map[string]any
				if json.Unmarshal(a.Statement.Value, &object) != nil {
					return nil, fmt.Errorf("assertion %s: statement value is neither a string nor an object", a.ID)
				}
				compact, err := json.Marshal(object)
				if err != nil {
					return nil, err
				}
				c.Statement.Value = string(compact)
			}
		}
		if a.SigningKey != nil {
			if a.SigningKey.Alg != "HS256" && a.SigningKey.Alg != "RS256" {
				return nil, fmt.Errorf("assertion %s: unsupported signing key algorithm %s", a.ID, a.SigningKey.Alg)
			}
			c.SigningKey = g.AssertionKey{Alg: g.AssertionKeyAlg(a.SigningKey.Alg), Key: a.SigningKey.Key}
		}
		configs = append(configs, c)
	}
	return configs, nil
}

// parseVerificationKeys reads --with-assertion-verification-keys:
// {"keys": {id: {alg, key}}, "defaultKey": {alg, key}} with the HS256 secret
// or the RS256 public key PEM as text.
func parseVerificationKeys(arg string) (g.AssertionVerificationKeys, error) {
	data, err := fileOrJSON(arg)
	if err != nil {
		return g.AssertionVerificationKeys{}, err
	}
	var file struct {
		DefaultKey *assertionKeyJSON           `json:"defaultKey"`
		Keys       map[string]assertionKeyJSON `json:"keys"`
	}
	if err := json.Unmarshal(data, &file); err != nil {
		return g.AssertionVerificationKeys{}, fmt.Errorf("assertion verification keys must be a JSON object: %w", err)
	}
	keys := g.AssertionVerificationKeys{}
	if file.DefaultKey != nil {
		keys.DefaultKey = g.AssertionKey{Alg: g.AssertionKeyAlg(file.DefaultKey.Alg), Key: file.DefaultKey.Key}
	}
	for id, k := range file.Keys {
		keys.Keys = append(keys.Keys, g.AssertionKeyEntry{AssertionID: id, Alg: g.AssertionKeyAlg(k.Alg), Key: k.Key})
	}
	return keys, nil
}
