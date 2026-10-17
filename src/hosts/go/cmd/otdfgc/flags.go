package main

import (
	"encoding/json"
	"fmt"
	"os"
	"strings"

	g "goalchemyout"
)

// flagSpec is the otdfctl flag subset of the CLI contract; bool flags take no
// value. Every contract flag is declared so that it parses, and the ones this
// build cannot honour are rejected afterwards with a reason instead of being
// ignored.
type flagSpec struct {
	name string
	bool bool
}

var commonFlags = []flagSpec{
	{name: "out"}, {name: "host"}, {name: "with-client-creds"}, {name: "with-client-creds-file"},
	{name: "dpop"}, {name: "dpop-key"}, {name: "tls-no-verify", bool: true}, {name: "log-level"},
}

var encryptFlags = append([]flagSpec{
	{name: "attr"}, {name: "mime-type"}, {name: "with-assertions"}, {name: "wrapping-key-algorithm"},
	{name: "target-mode"}, {name: "policy-mode"}, {name: "ecdsa-binding", bool: true},
}, commonFlags...)

var decryptFlags = append([]flagSpec{
	{name: "with-assertion-verification-keys"}, {name: "no-verify-assertions", bool: true},
	{name: "session-key-algorithm"}, {name: "kas-allowlist"},
}, commonFlags...)

type parsed struct {
	values     map[string]string
	set        map[string]bool
	positional []string
}

func parseFlags(args []string, spec []flagSpec) (*parsed, error) {
	specs := map[string]flagSpec{}
	for _, s := range spec {
		specs[s.name] = s
	}
	p := &parsed{values: map[string]string{}, set: map[string]bool{}}
	for i := 0; i < len(args); i++ {
		a := args[i]
		if !strings.HasPrefix(a, "-") || a == "-" {
			p.positional = append(p.positional, a)
			continue
		}
		name := strings.TrimLeft(a, "-")
		value, inline := "", false
		if eq := strings.IndexByte(name, '='); eq >= 0 {
			name, value, inline = name[:eq], name[eq+1:], true
		}
		if name == "o" {
			name = "out"
		}
		s, ok := specs[name]
		if !ok {
			return nil, fmt.Errorf("unknown flag: %s", a)
		}
		if s.bool {
			if inline && value != "true" && value != "false" {
				return nil, fmt.Errorf("flag --%s takes no value", name)
			}
			p.set[name] = !inline || value == "true"
			continue
		}
		if !inline {
			if i+1 >= len(args) {
				return nil, fmt.Errorf("flag --%s needs a value", name)
			}
			i++
			value = args[i]
		}
		p.values[name] = value
		p.set[name] = true
	}
	return p, nil
}

func (p *parsed) value(name string) string { return p.values[name] }
func (p *parsed) flag(name string) bool    { return p.set[name] }

// unsupportedRule names a flag this build cannot honour, optionally only for
// values other than the ones listed in allowed.
type unsupportedRule struct {
	name    string
	allowed []string
}

var encryptUnsupported = []unsupportedRule{
	{name: "with-assertions"},
	{name: "ecdsa-binding"},
	{name: "target-mode", allowed: []string{"4.3.0"}},
	{name: "policy-mode", allowed: []string{"encrypted"}},
	{name: "wrapping-key-algorithm", allowed: []string{"rsa:2048", "ec:secp256r1", "rsa:4096", "ec:secp384r1", "ec:secp521r1"}},
	{name: "dpop", allowed: []string{"ES256", "ES384", "ES512", "RS256", "RS384", "RS512"}},
	{name: "with-client-creds-file"},
}

var decryptUnsupported = []unsupportedRule{
	{name: "with-assertion-verification-keys"},
	{name: "session-key-algorithm", allowed: []string{"rsa:2048", "ec:secp256r1", "rsa:4096", "ec:secp384r1", "ec:secp521r1"}},
	{name: "dpop", allowed: []string{"ES256", "ES384", "ES512", "RS256", "RS384", "RS512"}},
	{name: "with-client-creds-file"},
}

func (p *parsed) unsupported(rules []unsupportedRule) string {
	for _, r := range rules {
		if !p.set[r.name] {
			continue
		}
		if len(r.allowed) == 0 {
			return "--" + r.name
		}
		v := p.values[r.name]
		allowed := false
		for _, a := range r.allowed {
			if v == a {
				allowed = true
			}
		}
		if !allowed {
			return "--" + r.name + " " + v
		}
	}
	return ""
}

type clientCredentials struct {
	ClientID     string `json:"clientId"`
	ClientSecret string `json:"clientSecret"`
}

// options builds the SDK options from --host, --with-client-creds and the
// DPoP and TLS flags; the SDK discovers everything else from the platform.
func (p *parsed) options() (string, []g.Option, int, error) {
	host := strings.TrimRight(p.value("host"), "/")
	if host == "" {
		return "", nil, exitUsage, fmt.Errorf("--host is required\n%s", usage)
	}
	creds := p.value("with-client-creds")
	if creds == "" {
		return "", nil, exitUsage, fmt.Errorf("--with-client-creds is required\n%s", usage)
	}
	var c clientCredentials
	if err := json.Unmarshal([]byte(creds), &c); err != nil || c.ClientID == "" || c.ClientSecret == "" {
		return "", nil, exitUsage, fmt.Errorf("--with-client-creds must be JSON with clientId and clientSecret")
	}
	opts := []g.Option{g.WithClientCredentials(c.ClientID, c.ClientSecret, nil)}
	if strings.HasPrefix(strings.ToLower(host), "http://") {
		opts = append(opts, g.WithInsecurePlaintextConn())
	}
	if p.flag("tls-no-verify") {
		opts = append(opts, g.WithInsecureSkipVerifyConn())
	}
	if alg := p.value("dpop"); alg != "" {
		opts = append(opts, g.WithDPoPAlgorithm(g.SigningAlgorithm(alg)))
	}
	if key := p.value("dpop-key"); key != "" {
		if p.value("dpop") == "" {
			return "", nil, exitUsage, fmt.Errorf("--dpop-key needs --dpop")
		}
		pem, err := os.ReadFile(key)
		if err != nil {
			return "", nil, exitError, fmt.Errorf("Failed to read DPoP key %s: %s", key, err)
		}
		opts = append(opts, g.WithDPoPKeyPEM(pem))
	}
	return host, opts, exitSuccess, nil
}
