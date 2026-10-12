package sdk

import "github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"

// Get returns the raw value of a top-level member.
func (c PlatformConfiguration) Get(key string) (string, bool) {
	for _, e := range c.Entries {
		if e.Key == key {
			return e.Value, true
		}
	}
	return "", false
}

// Set replaces or adds a top-level member.
func (c *PlatformConfiguration) Set(key, value string) {
	for i := range c.Entries {
		if c.Entries[i].Key == key {
			c.Entries[i].Value = value
			return
		}
	}
	c.Entries = append(c.Entries, PlatformConfigurationEntry{Key: key, Value: value})
}

func (c PlatformConfiguration) clone() PlatformConfiguration {
	return PlatformConfiguration{Entries: append([]PlatformConfigurationEntry(nil), c.Entries...)}
}

// platformConfigurationFromJSON flattens a well-known configuration document:
// string members keep their value, every other member its compact JSON.
func platformConfigurationFromJSON(doc jsonvalue.Value) (PlatformConfiguration, error) {
	cfg := PlatformConfiguration{Entries: []PlatformConfigurationEntry{}}
	for _, key := range doc.Keys() {
		member := doc.Get(key)
		if member.Kind() == jsonvalue.KindString {
			cfg.Entries = append(cfg.Entries, PlatformConfigurationEntry{Key: key, Value: member.String()})
			continue
		}
		text, err := jsonvalue.Encode(member)
		if err != nil {
			return PlatformConfiguration{}, err
		}
		cfg.Entries = append(cfg.Entries, PlatformConfigurationEntry{Key: key, Value: string(text)})
	}
	return cfg, nil
}

func (c PlatformConfiguration) getIdpConfig() jsonvalue.Value {
	raw, ok := c.Get("idp")
	if !ok {
		return jsonvalue.Object()
	}
	idpCfg, err := jsonvalue.Parse([]byte(raw))
	if err != nil || idpCfg.Kind() != jsonvalue.KindObject {
		idpCfg = jsonvalue.Object()
	}
	return idpCfg
}

func (c PlatformConfiguration) Issuer() (string, error) {
	idpCfg := c.getIdpConfig()
	value, ok := idpCfg.Get("issuer").AsString()
	if !ok {
		return "", ErrPlatformIssuerNotFound
	}
	return value, nil
}

func (c PlatformConfiguration) AuthzEndpoint() (string, error) {
	idpCfg := c.getIdpConfig()
	value, ok := idpCfg.Get("authorization_endpoint").AsString()
	if !ok {
		return "", ErrPlatformAuthzEndpointNotFound
	}
	return value, nil
}

func (c PlatformConfiguration) TokenEndpoint() (string, error) {
	idpCfg := c.getIdpConfig()
	value, ok := idpCfg.Get("token_endpoint").AsString()
	if !ok {
		return "", ErrPlatformTokenEndpointNotFound
	}
	return value, nil
}

func (c PlatformConfiguration) platformEndpoint() (string, error) {
	value, ok := c.Get("platform_endpoint")
	if !ok {
		return "", ErrPlatformEndpointNotFound
	}
	return value, nil
}
