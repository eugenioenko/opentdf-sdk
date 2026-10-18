package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/std/fmt"
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
	// ClientID identifies the OAuth client. Leave credentials and
	// TokenProviderName empty for unauthenticated discovery only.
	ClientID string
	// ClientSecret is the secret paired with ClientID for client-credentials
	// authentication.
	ClientSecret string
	// Scopes lists the OAuth scopes requested with client credentials.
	Scopes []string
	// TokenEndpoint overrides the discovered OAuth token endpoint when nonempty.
	TokenEndpoint string
	// TokenExchange reserves token-exchange settings. Nonzero values are currently
	// rejected as unsupported.
	TokenExchange TokenExchange
	// CertExchange reserves certificate-exchange settings. Nonzero values are currently
	// rejected as unsupported.
	CertExchange CertExchange
	// TokenProviderName names the host callback that supplies tokens instead of client
	// credentials.
	TokenProviderName string

	// PlatformConfiguration skips platform discovery when nonempty. Values
	// must include the configuration needed by the selected authentication.
	PlatformConfiguration PlatformConfiguration
	// ValidateConnection checks platform health during construction when true.
	ValidateConnection bool

	// InsecurePlaintext permits HTTP endpoints for trusted development services.
	InsecurePlaintext bool
	// InsecureSkipVerify disables TLS certificate verification. Use only with trusted
	// development services.
	InsecureSkipVerify bool
	// TLS reserves custom TLS settings. Nonzero values are currently rejected as
	// unsupported.
	TLS TLSOptions
	// UnixSocket reserves a Unix socket transport path. Nonempty values are currently
	// unsupported.
	UnixSocket string
	// TimeoutMillis limits each HTTP request in milliseconds. Zero uses 30 seconds;
	// negative values are rejected.
	TimeoutMillis int64

	// CoreEndpoint overrides the base URL for all platform services. More
	// specific service endpoints take precedence.
	CoreEndpoint string
	// PolicyEndpoint overrides the endpoint for policy services.
	PolicyEndpoint string
	// AuthorizationEndpoint overrides the endpoint for authorization services.
	AuthorizationEndpoint string
	// EntityResolutionEndpoint overrides the endpoint for entity resolution services.
	EntityResolutionEndpoint string
	// WellknownEndpoint overrides the endpoint for the well-known configuration service.
	WellknownEndpoint string

	// SessionEncryptionRSAPEM supplies the response-session private key.
	// Empty generates a new RSA-2048 key owned by this SDK.
	SessionEncryptionRSAPEM string
	// SessionSignerRSAPEM supplies a signing key when DPoPKeyPEM is empty.
	SessionSignerRSAPEM string
	// DPoPAlgorithm selects the DPoP signing algorithm. With no supplied key, the default
	// is ES256.
	DPoPAlgorithm SigningAlgorithm
	// DPoPKeyPEM supplies the PEM signing key for DPoP proofs. Empty generates an
	// ephemeral key.
	DPoPKeyPEM string
	// NoKIDInKAO omits key identifiers from newly created key access objects for legacy
	// compatibility.
	NoKIDInKAO bool

	// FulfillableObligationFQNs declares obligations the application can fulfill.
	FulfillableObligationFQNs []string

	// Logging. 0 = the host sink's default level; otherwise a slog level
	// (-4 debug, 0 info, 4 warn, 8 error) below which records are dropped.
	LogLevel int
}

// TokenExchange holds reserved OAuth token-exchange settings. This build rejects
// nonzero settings.
type TokenExchange struct {
	// SubjectToken is the token to exchange.
	SubjectToken string
	// Audience lists the requested token audiences.
	Audience []string
}

// CertExchange holds reserved certificate-exchange settings. This build rejects
// nonzero settings.
type CertExchange struct {
	// ClientCertPEM is the PEM client certificate.
	ClientCertPEM string
	// ClientKeyPEM is the matching PEM private key.
	ClientKeyPEM string
	// Audience lists the requested token audiences.
	Audience []string
}

// TLSOptions holds reserved TLS configuration. This build rejects nonzero settings.
type TLSOptions struct {
	// ClientCertPEM is the PEM certificate for client authentication.
	ClientCertPEM string
	// ClientKeyPEM is the matching PEM private key.
	ClientKeyPEM string
	// RootCAsPEM contains PEM certificates to trust as roots.
	RootCAsPEM string
	// Audience lists the intended service audiences.
	Audience []string
}

// PlatformConfigurationEntry is one top-level member of the well-known
// configuration. A string member keeps its value; any other member (the idp
// and base_key objects, booleans) carries its compact JSON text.
type PlatformConfigurationEntry struct {
	// Key is the top-level configuration member name.
	Key string
	// Value holds a string member directly, or compact JSON for other member types.
	Value string
}

// PlatformConfiguration is the platform's well-known configuration. The
// reference's map[string]interface{} cannot cross the Goalchemy boundary, so
// the members are a list of entries; the accessors read them as the
// reference does. Structure is subject to change; consume via the accessors.
type PlatformConfiguration struct {
	// Entries contains the configuration members. Prefer Get and Set over direct
	// mutation.
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
	coreEndpoint                       string
	policyEndpoint                     string
	authorizationEndpoint              string
	entityResolutionEndpoint           string
	wellknownEndpoint                  string
}

// ConnectRPCConnection is the reference's connection to the platform's
// services. Every connection uses the SDK's authenticated transport, so the
// endpoint is all a caller chooses.
type ConnectRPCConnection struct {
	// Endpoint is the service base URL used through the SDK's authenticated transport.
	Endpoint string
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
	for _, endpoint := range []string{opts.CoreEndpoint, opts.PolicyEndpoint, opts.AuthorizationEndpoint, opts.EntityResolutionEndpoint, opts.WellknownEndpoint} {
		if endpoint != "" && IsPlatformEndpointMalformed(endpoint) {
			return nil, newError(KindInvalidInput, "New", fmt.Sprintf("%v [%v]", ErrPlatformEndpointMalformed, endpoint))
		}
	}
	cfg.coreEndpoint = opts.CoreEndpoint
	cfg.policyEndpoint = opts.PolicyEndpoint
	cfg.authorizationEndpoint = opts.AuthorizationEndpoint
	cfg.entityResolutionEndpoint = opts.EntityResolutionEndpoint
	cfg.wellknownEndpoint = opts.WellknownEndpoint
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
