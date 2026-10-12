package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/std/log/slog"
)

// SigningAlgorithm identifies a JWS signing algorithm (RFC 7518 §3.1).
type SigningAlgorithm string

// Supported JWS signing algorithms for DPoP proof tokens (RFC 9449 §4.2).
const (
	ES256 SigningAlgorithm = "ES256"
	ES384 SigningAlgorithm = "ES384"
	ES512 SigningAlgorithm = "ES512"
	RS256 SigningAlgorithm = "RS256"
	RS384 SigningAlgorithm = "RS384"
	RS512 SigningAlgorithm = "RS512"
)

// Options configures New. Every zero value is the reference default: no
// credentials, platform configuration fetched from the well-known endpoint,
// TLS verified, an ephemeral ES256 DPoP key, RSA-2048 session key. The
// reference's With* functional options are provided by the Go host adapter
// over this struct. TokenExchange, CertExchange and TLS are values rather
// than the api-spec's pointers because pointers do not cross the Goalchemy
// boundary; their zero value means unset.
type Options struct {
	// Authentication. Client credentials or a host token provider; none means
	// an uncredentialed client, which can still read the well-known
	// configuration.
	ClientID          string
	ClientSecret      string
	Scopes            []string
	TokenEndpoint     string
	TokenExchange     TokenExchange
	CertExchange      CertExchange
	TokenProviderName string

	// Platform configuration. Empty means fetch /.well-known.
	PlatformConfiguration PlatformConfiguration
	ValidateConnection    bool

	// Transport.
	InsecurePlaintext        bool
	InsecureSkipVerify       bool
	TLS                      TLSOptions
	UnixSocket               string
	TimeoutMillis            int64
	CoreEndpoint             string
	PolicyEndpoint           string
	AuthorizationEndpoint    string
	EntityResolutionEndpoint string
	WellknownEndpoint        string

	// Keys.
	SessionEncryptionRSAPEM string
	SessionSignerRSAPEM     string
	DPoPAlgorithm           SigningAlgorithm
	DPoPKeyPEM              string
	NoKIDInKAO              bool

	// Policy.
	FulfillableObligationFQNs []string

	// Logging. 0 = the host sink's default level; otherwise a slog level
	// (-4 debug, 0 info, 4 warn, 8 error) below which records are dropped.
	LogLevel int
}

type TokenExchange struct {
	SubjectToken string
	Audience     []string
}

type CertExchange struct {
	ClientCertPEM string
	ClientKeyPEM  string
	Audience      []string
}

type TLSOptions struct {
	ClientCertPEM string
	ClientKeyPEM  string
	RootCAsPEM    string
	Audience      []string
}

// PlatformConfigurationEntry is one top-level member of the well-known
// configuration. A string member keeps its value; any other member (the idp
// and base_key objects, booleans) carries its compact JSON text.
type PlatformConfigurationEntry struct {
	Key   string
	Value string
}

// PlatformConfiguration is the platform's well-known configuration. The
// reference's map[string]interface{} cannot cross the Goalchemy boundary, so
// the members are a list of entries; the accessors read them as the
// reference does. Structure is subject to change; consume via the accessors.
type PlatformConfiguration struct {
	Entries []PlatformConfigurationEntry
}

// Internal config struct for building SDK options.
type config struct {
	PlatformConfiguration              PlatformConfiguration
	hasPlatformConfiguration           bool
	transportOptions                   []string
	clientCredentials                  *clientCredentials
	tokenProviderName                  string
	tokenEndpoint                      string
	scopes                             []string
	kasSessionKey                      *crypto.Key
	dpopKeyPEM                         string
	dpopAlgorithm                      SigningAlgorithm
	tdfFeatures                        tdfFeatures
	shouldValidatePlatformConnectivity bool
	fulfillableObligationFQNs          []string
	logger                             *slog.Logger
	timeoutMillis                      int64
	insecurePlaintext                  bool
}

type clientCredentials struct {
	ClientID   string
	ClientAuth string
}

// Options specific to TDF protocol features
type tdfFeatures struct {
	// For backward compatibility, don't store the KID in the KAO.
	noKID bool
}

// newConfig applies opts in the order the reference's With* options would,
// rejecting the options this build does not implement yet.
func newConfig(opts Options) (*config, error) {
	cfg := &config{timeoutMillis: defaultTimeoutMillis}
	if opts.TokenExchange.SubjectToken != "" || len(opts.TokenExchange.Audience) > 0 {
		return nil, unsupported("New", "TokenExchange")
	}
	if opts.CertExchange.ClientCertPEM != "" || opts.CertExchange.ClientKeyPEM != "" || len(opts.CertExchange.Audience) > 0 {
		return nil, unsupported("New", "CertExchange")
	}
	if opts.TLS.ClientCertPEM != "" || opts.TLS.ClientKeyPEM != "" || opts.TLS.RootCAsPEM != "" || len(opts.TLS.Audience) > 0 {
		return nil, unsupported("New", "TLS")
	}
	if opts.UnixSocket != "" {
		return nil, unsupported("New", "UnixSocket")
	}
	if opts.CoreEndpoint != "" || opts.PolicyEndpoint != "" || opts.AuthorizationEndpoint != "" || opts.EntityResolutionEndpoint != "" || opts.WellknownEndpoint != "" {
		return nil, unsupported("New", "custom service endpoints")
	}
	if opts.ClientID != "" || opts.ClientSecret != "" {
		cfg.clientCredentials = &clientCredentials{ClientID: opts.ClientID, ClientAuth: opts.ClientSecret}
		cfg.scopes = append([]string(nil), opts.Scopes...)
	}
	cfg.tokenProviderName = opts.TokenProviderName
	cfg.tokenEndpoint = opts.TokenEndpoint
	if len(opts.PlatformConfiguration.Entries) > 0 {
		cfg.PlatformConfiguration = opts.PlatformConfiguration.clone()
		cfg.hasPlatformConfiguration = true
	}
	cfg.shouldValidatePlatformConnectivity = opts.ValidateConnection
	cfg.insecurePlaintext = opts.InsecurePlaintext
	if opts.InsecureSkipVerify {
		cfg.transportOptions = []string{"insecure-skip-verify", "true"}
	}
	if opts.TimeoutMillis < 0 {
		return nil, newError(KindInvalidInput, "New", "TimeoutMillis must not be negative")
	}
	if opts.TimeoutMillis > 0 {
		cfg.timeoutMillis = opts.TimeoutMillis
	}
	if opts.SessionEncryptionRSAPEM != "" {
		key, err := crypto.ImportPEM(opts.SessionEncryptionRSAPEM)
		if err != nil {
			return nil, wrapError(KindInvalidInput, "New", "SessionEncryptionRSAPEM", err)
		}
		cfg.kasSessionKey = key
	}
	cfg.dpopKeyPEM = opts.DPoPKeyPEM
	cfg.dpopAlgorithm = opts.DPoPAlgorithm
	if opts.SessionSignerRSAPEM != "" && cfg.dpopKeyPEM == "" {
		cfg.dpopKeyPEM = opts.SessionSignerRSAPEM
	}
	cfg.tdfFeatures.noKID = opts.NoKIDInKAO
	cfg.fulfillableObligationFQNs = append([]string(nil), opts.FulfillableObligationFQNs...)
	cfg.logger = newLogger(opts.LogLevel)
	return cfg, nil
}

func newLogger(level int) *slog.Logger {
	if level == 0 {
		return slog.Default()
	}
	return slog.New(slog.NewHostHandler(&slog.HandlerOptions{Level: slog.Level(level)}))
}
