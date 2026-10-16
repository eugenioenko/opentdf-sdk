package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/sync"
	"github.com/eugenioenko/goalchemy/std/bytes"
	"github.com/eugenioenko/goalchemy/std/encoding/json"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/io"
	"github.com/eugenioenko/goalchemy/std/log/slog"
	"github.com/eugenioenko/goalchemy/std/net/url"
	"github.com/eugenioenko/goalchemy/std/strings"
	"opentdf-local/sdk/src/protocol/wellknownconfiguration"
	"opentdf-local/sdk/src/tdf"
)

var (
	// Failure while connecting to a service.
	// Check your configuration and/or retry.
	ErrGrpcDialFailed      = errors.New("failed to dial grpc endpoint")
	ErrShutdownFailed      = errors.New("failed to shutdown sdk")
	ErrPlatformUnreachable = errors.New("platform unreachable or not responding")
	// ErrHealthCheckUnsupported is returned by SDK.IsHealthy when the SDK is configured
	// in IPC mode, which does not support the gRPC Health protocol.
	ErrHealthCheckUnsupported        = errors.New("health check not supported in IPC mode")
	ErrPlatformConfigFailed          = errors.New("failed to retrieve platform configuration")
	ErrPlatformEndpointMalformed     = errors.New("platform endpoint is malformed")
	ErrPlatformIssuerNotFound        = errors.New("issuer not found in well-known idp configuration")
	ErrPlatformAuthzEndpointNotFound = errors.New("authorization_endpoint not found in well-known idp configuration")
	ErrPlatformTokenEndpointNotFound = errors.New("token_endpoint not found in well-known idp configuration")
	ErrPlatformEndpointNotFound      = errors.New("platform_endpoint not found in well-known configuration")
	ErrAccessTokenInvalid            = errors.New("access token is invalid")
	ErrNoAccessTokenSource           = errors.New("no access token source configured; SDK was created without credentials")
	ErrWellKnowConfigEmpty           = errors.New("well-known configuration is empty")
	ErrAttributeNotFound             = errors.New("attribute not found")
)

// SDK is the persistent client handle. Every collaborator hangs off it:
// nothing is package-level state.
type SDK struct {
	cfg              config
	platformEndpoint string
	kasKeyCache      *kasKeyCache
	transport        Transport
	clock            Clock
	client           *httpClient
	tokenSource      TokenSource
	dpop             *dpopSigner
	keyFetcher       KASKeyFetcher
	rewrapper        Rewrapper
	splitter         KeySplitter
	granter          Granter
	attributes       attributesService
	authorization    authorizationService
	mu               sync.Mutex
	closed           bool
}

// New creates an SDK for the platform at platformEndpoint. Unless
// opts.PlatformConfiguration is given it fetches the well-known
// configuration and resolves the token endpoint through OIDC discovery, as
// the reference does. ctx bounds that discovery; the reference's New has no
// context, but hosts cancel through the one every generated call carries.
func New(ctx context.Context, platformEndpoint string, opts Options) (*SDK, error) {
	cfg, err := newConfig(opts)
	if err != nil {
		return nil, err
	}
	s := &SDK{cfg: *cfg, platformEndpoint: platformEndpoint, transport: &libTransport{options: cfg.transportOptions}, clock: systemClock{}}
	if err := s.init(ctx); err != nil {
		s.Close()
		return nil, err
	}
	return s, nil
}

// newWith builds an SDK over the given collaborators; tests use it to
// replace the transport and clock.
func newWith(ctx context.Context, platformEndpoint string, opts Options, transport Transport, clock Clock) (*SDK, error) {
	cfg, err := newConfig(opts)
	if err != nil {
		return nil, err
	}
	s := &SDK{cfg: *cfg, platformEndpoint: platformEndpoint, transport: transport, clock: clock}
	if err := s.init(ctx); err != nil {
		s.Close()
		return nil, err
	}
	return s, nil
}

func (s *SDK) init(ctx context.Context) error {
	cfg := &s.cfg
	// If KAS session key is not provided, generate a new one
	if cfg.kasSessionKey == nil {
		key, err := crypto.GenerateRSA2048()
		if err != nil {
			return wrapError(KindInvalidInput, "New", "failed to generate session key", err)
		}
		cfg.kasSessionKey = key
	}

	preAuth := &httpClient{transport: s.transport, clock: s.clock, timeoutMillis: cfg.timeoutMillis}

	if IsPlatformEndpointMalformed(s.platformEndpoint) {
		return newError(KindInvalidInput, "New", fmt.Sprintf("%v [%v]", ErrPlatformEndpointMalformed, s.platformEndpoint))
	}
	if cfg.shouldValidatePlatformConnectivity {
		if err := validateHealthyPlatformConnection(ctx, s.serviceEndpoint(coreServices), preAuth); err != nil {
			return err
		}
	}

	// If platformConfiguration is not provided, fetch it from the platform
	if !cfg.hasPlatformConfiguration {
		pcfg, err := s.getPlatformConfiguration(ctx, preAuth)
		if err != nil {
			return joinError(KindTransport, "New", []error{ErrPlatformConfigFailed, err})
		}
		cfg.PlatformConfiguration = pcfg
		cfg.hasPlatformConfiguration = true
		if cfg.tokenEndpoint == "" {
			cfg.tokenEndpoint, err = getTokenEndpoint(ctx, *cfg, preAuth)
			if err != nil {
				return wrapError(KindTransport, "New", "", err)
			}
		}
	}
	cfg.PlatformConfiguration.Set("platform_endpoint", s.platformEndpoint)

	tokenSource, dpop, err := s.buildIDPTokenSource(cfg)
	if err != nil {
		return err
	}
	s.tokenSource = tokenSource
	s.dpop = dpop
	s.client = &httpClient{transport: s.transport, clock: s.clock, timeoutMillis: cfg.timeoutMillis, tokenSource: tokenSource, dpop: dpop, tokenEndpoint: cfg.tokenEndpoint}
	s.kasKeyCache = newKasKeyCache(s.clock)
	s.keyFetcher = s
	s.rewrapper = newKASClient(s.client, dpop, s.clock, cfg.fulfillableObligationFQNs)
	s.splitter = defaultKeySplitter{}
	s.granter = noGranter{}
	s.attributes = serviceConnection{client: s.client, endpoint: s.serviceEndpoint(policyServices)}
	s.authorization = serviceConnection{client: s.client, endpoint: s.serviceEndpoint(authorizationServices)}
	return nil
}

func IsPlatformEndpointMalformed(e string) bool {
	u, err := url.ParseRequestURI(e)
	if err != nil || u.Hostname() == "" || strings.Contains(u.Hostname(), ":") {
		return true
	}
	return false
}

// buildIDPTokenSource builds the access token source and resolves the DPoP key
// once, returning it so the caller can give the DPoP transport the same key the
// token source binds to. The returned key is nil when DPoP is not in effect
// (no credentials, or no key configured for a custom token source).
func (s *SDK) buildIDPTokenSource(c *config) (TokenSource, *dpopSigner, error) {
	if c.tokenProviderName != "" {
		// A custom token source manages its own credentials, but a configured DPoP
		// key still drives the resource-request transport; without one the
		// reference's ephemeral key signs the rewrap requests.
		dpop, err := resolveDPoPKey(c)
		if err != nil {
			return nil, nil, wrapError(KindInvalidInput, "New", "failed to resolve DPoP key", err)
		}
		if dpop == nil {
			if dpop, err = defaultDPoPSigner(); err != nil {
				return nil, nil, err
			}
		}
		return newCallbackTokenSource(c.tokenProviderName, s.clock, dpop), dpop, nil
	}

	// There are uses for uncredentialed clients (i.e. consuming the well-known configuration).
	if c.clientCredentials == nil {
		// DPoP only takes effect once requests are credentialed. If the caller
		// explicitly configured a DPoP key but supplied no credentials, fail loudly
		// rather than silently returning an unbound client and downgrading the
		// caller's expected security posture.
		if c.dpopKeyPEM != "" || c.dpopAlgorithm != "" {
			return nil, nil, newError(KindInvalidInput, "New", "DPoP configured (WithDPoP*) but no client credentials or OAuth token source supplied")
		}
		return nil, nil, nil
	}

	dpop, err := resolveDPoPKey(c)
	if err != nil {
		return nil, nil, wrapError(KindInvalidInput, "New", "failed to resolve DPoP key", err)
	}

	// No DPoP key configured: auto-generate a default ephemeral ES256/P-256 key.
	if dpop == nil {
		if dpop, err = defaultDPoPSigner(); err != nil {
			return nil, nil, err
		}
	}

	tokenClient := &httpClient{transport: s.transport, clock: s.clock, timeoutMillis: c.timeoutMillis, dpop: dpop, tokenEndpoint: c.tokenEndpoint}
	tokenSource, err := newIDPAccessTokenSource(tokenClient, *c.clientCredentials, c.tokenEndpoint, c.scopes)
	if err != nil {
		return nil, nil, wrapError(KindInvalidInput, "New", "", err)
	}
	return tokenSource, dpop, nil
}

// defaultDPoPSigner is the reference's ephemeral DPoP key: ES256 over P-256.
func defaultDPoPSigner() (*dpopSigner, error) {
	key, err := generateDPoPKeyForAlg(ES256)
	if err != nil {
		return nil, wrapError(KindInvalidInput, "New", "failed to generate default DPoP key", err)
	}
	dpop, err := newDPoPSigner(key, ES256)
	if err != nil {
		key.Close()
		return nil, wrapError(KindInvalidInput, "New", "failed to generate default DPoP key", err)
	}
	return dpop, nil
}

// Close releases the session and DPoP keys. It is idempotent.
func (s *SDK) Close() error {
	if s == nil {
		return nil
	}
	s.mu.Lock()
	closed := s.closed
	s.closed = true
	s.mu.Unlock()
	if closed {
		return nil
	}
	if s.cfg.kasSessionKey != nil {
		s.cfg.kasSessionKey.Close()
		s.cfg.kasSessionKey = nil
	}
	if s.dpop != nil {
		s.dpop.close()
	}
	return nil
}

func (s *SDK) check(op string) error {
	s.mu.Lock()
	closed := s.closed
	s.mu.Unlock()
	if closed {
		return newError(KindInvalidInput, op, "sdk is closed")
	}
	return nil
}

// Logger returns the configured slog.Logger for this SDK instance
func (s *SDK) Logger() *slog.Logger {
	return s.cfg.logger
}

// Conn returns the connection the SDK reaches the platform's core services
// through: the platform endpoint, or WithCustomCoreConnection's.
func (s *SDK) Conn() ConnectRPCConnection {
	return ConnectRPCConnection{Endpoint: s.serviceEndpoint(coreServices)}
}

// PlatformConfiguration returns the platform's well-known configuration, with
// platform_endpoint set, as the reference's exported field does.
func (s *SDK) PlatformConfiguration() PlatformConfiguration {
	return s.cfg.PlatformConfiguration.clone()
}

// IsHealthy reports whether the platform's gRPC Health v1 endpoint is reachable and SERVING.
//
// Returns:
//   - (true, nil) when the platform reports SERVING.
//   - (false, nil) when the platform is reachable but reports NOT_SERVING or UNKNOWN.
//   - (false, error) wrapping ErrPlatformUnreachable on transport failure or ctx errors.
func (s *SDK) IsHealthy(ctx context.Context) (bool, error) {
	if s.client == nil {
		return false, sentinelError(KindUnsupportedFeature, ErrHealthCheckUnsupported)
	}
	healthy, err := checkPlatformHealth(ctx, s.serviceEndpoint(coreServices), s.client)
	if err != nil {
		return false, joinError(KindTransport, "IsHealthy", []error{ErrPlatformUnreachable, err})
	}
	return healthy, nil
}

type TdfType string

const (
	Invalid  TdfType = "Invalid"
	Standard TdfType = "Standard"
)

// String returns the string representation of the applies to state.
func (t TdfType) String() string {
	return string(t)
}

// ZIP file Signature
var zipSignature = []byte{0x50, 0x4B, 0x03, 0x04}

// GetTdfType returns the type of TDF based on the reader.
// Reader is reset after the check.
func GetTdfType(reader io.ReadSeeker) TdfType {
	numBytes := 4
	buffer := make([]byte, numBytes)
	n, err := reader.Read(buffer)
	if err != nil {
		return Invalid
	}

	// Reset the reader to its original position
	_, err = reader.Seek(0, io.SeekStart)
	if err != nil {
		return Invalid
	}

	if n < numBytes {
		return Invalid
	}

	// Check if the first 4 bytes match the ZIP signature
	if bytes.Equal(buffer, zipSignature) {
		return Standard
	}

	return Invalid
}

// Indicates JSON Schema validation failed for the manifest or header of the TDF file.
// Some invalid manifests are still usable, so this file may still be usable.
var ErrInvalidPerSchema = errors.New("manifest was not valid")

// readArchive reads the whole TDF from reader and returns it with the reader
// rewound, so repeated validation on one reader sees the same bytes.
func readArchive(reader io.ReadSeeker, maxManifestSize int64) (tdf.Archive, error) {
	if _, err := reader.Seek(0, io.SeekStart); err != nil {
		return tdf.Archive{}, err
	}
	data, err := io.ReadAll(reader)
	if err != nil {
		return tdf.Archive{}, err
	}
	if _, err := reader.Seek(0, io.SeekStart); err != nil {
		return tdf.Archive{}, err
	}
	limits := tdf.DefaultArchiveLimits()
	if maxManifestSize > 0 && maxManifestSize < int64(limits.ManifestBytes) {
		limits.ManifestBytes = int(maxManifestSize)
	}
	return tdf.ReadArchive(data, limits)
}

// Detects whether, or not the reader is a valid TDF. It first checks if it can "open" it
// Then attempts to extract a manifest, then finally it validates the manifest.
// If any of the checks fail, it will return false.
func IsValidTdf(reader io.ReadSeeker) (bool, error) {
	archive, err := readArchive(reader, 0)
	if err != nil {
		return false, wrapError(KindNotDecryptable, "IsValidTdf", "zipstream.NewTDFReader failed", err)
	}
	valid, err := isValidManifest(string(archive.Manifest), Lax)
	return valid, boundaryError(KindInvalidInput, "IsValidTdf", err)
}

// isValidManifest checks the manifest's required members. Until the schema
// validator of Phase 6 arrives this covers the lax schema's required lists.
func isValidManifest(manifest string, intensity SchemaValidationIntensity) (bool, error) {
	if intensity == Skip {
		return true, nil
	}
	doc, err := jsonvalue.Parse([]byte(manifest))
	if err != nil || doc.Kind() != jsonvalue.KindObject {
		return false, errors.New("could not validate manifest.json")
	}
	missing := []string{}
	require := func(v jsonvalue.Value, path string, keys []string) {
		for _, k := range keys {
			if !v.Has(k) {
				missing = append(missing, path+"."+k)
			}
		}
	}
	require(doc, "manifest", []string{"payload", "encryptionInformation"})
	require(doc.Get("payload"), "payload", []string{"type", "url", "protocol", "isEncrypted"})
	enc := doc.Get("encryptionInformation")
	require(enc.Get("method"), "method", []string{"algorithm", "isStreamable"})
	require(enc.Get("integrityInformation"), "integrityInformation", []string{"rootSignature", "segmentSizeDefault", "segments", "encryptedSegmentSizeDefault"})
	if len(missing) > 0 {
		return false, fmt.Errorf("%w: missing %s", ErrInvalidPerSchema, strings.Join(missing, ", "))
	}
	var m Manifest
	if err := json.Unmarshal([]byte(manifest), &m); err != nil {
		return false, fmt.Errorf("%w: %w", ErrInvalidPerSchema, err)
	}
	return true, nil
}

// checkPlatformHealth issues a single gRPC Health v1 Check against the platform endpoint and
// reports whether the response is SERVING. ctx controls deadline and cancellation.
func checkPlatformHealth(ctx context.Context, endpoint string, client *httpClient) (bool, error) {
	checkURL, err := url.JoinPath(endpoint, "grpc.health.v1.Health", "Check")
	if err != nil {
		return false, err
	}
	base := strings.TrimSuffix(checkURL, healthProcedure)
	res, err := callJSON(ctx, client, base, healthProcedure, jsonvalue.Object(), "IsHealthy")
	if err != nil {
		return false, err
	}
	// The reference compares the proto enum; Connect JSON names it either way.
	status := res.Get("status").String()
	return status == "SERVING" || status == "SERVING_STATUS_SERVING", nil
}

// validateHealthyPlatformConnection is the construction-time reachability gate used by New when
// WithConnectionValidation is set.
func validateHealthyPlatformConnection(ctx context.Context, platformEndpoint string, client *httpClient) error {
	healthy, err := checkPlatformHealth(ctx, platformEndpoint, client)
	if err != nil {
		return joinError(KindTransport, "New", []error{ErrPlatformUnreachable, err})
	}
	if !healthy {
		return sentinelError(KindTransport, ErrPlatformUnreachable)
	}
	return nil
}

// fetchWellKnownConfiguration calls the well-known service with the
// generated client, as the reference's getPlatformConfiguration does.
func (s *SDK) fetchWellKnownConfiguration(ctx context.Context, client *httpClient) (jsonvalue.Value, error) {
	conn, call := client.newServiceClient(s.serviceEndpoint(wellknownServices), nil)
	wellKnownConfig := wellknownconfiguration.NewWellKnownServiceClient(conn)
	response, err := wellKnownConfig.GetWellKnownConfiguration(ctx, &wellknownconfiguration.GetWellKnownConfigurationRequest{})
	if err != nil {
		return jsonvalue.Value{}, call.serviceError("GetWellKnownConfiguration", err)
	}
	return response.GetConfiguration(), nil
}

func (s *SDK) getPlatformConfiguration(ctx context.Context, client *httpClient) (PlatformConfiguration, error) {
	configuration, err := s.fetchWellKnownConfiguration(ctx, client)
	if err != nil {
		return PlatformConfiguration{}, joinError(KindTransport, "New", []error{errors.New("unable to retrieve config information, and none was provided"), err})
	}
	return platformConfigurationFromJSON(configuration)
}

// getTokenEndpoint resolves the token endpoint from the issuer's OpenID
// configuration, as the reference does.
func getTokenEndpoint(ctx context.Context, c config, client *httpClient) (string, error) {
	issuerURL, ok := c.PlatformConfiguration.Get("platform_issuer")
	if !ok {
		return "", errors.New("platform_issuer is not set, or is not a string")
	}

	oidcConfigURL, err := url.JoinPath(issuerURL, ".well-known/openid-configuration")
	if err != nil {
		return "", fmt.Errorf("invalid issuer URL %q: %w", issuerURL, err)
	}

	status, _, body, err := client.do(ctx, "GET", oidcConfigURL, nil, nil, "", false)
	if err != nil {
		return "", err
	}
	if status < 200 || status >= 300 {
		return "", fmt.Errorf("openid configuration request to %s returned status %d", oidcConfigURL, status)
	}

	config, err := jsonvalue.Parse(body)
	if err != nil {
		return "", err
	}
	tokenEndpoint, ok := config.Get("token_endpoint").AsString()
	if !ok {
		return "", errors.New("token_endpoint not found in well-known configuration")
	}

	return tokenEndpoint, nil
}

// StoreKASKeys caches the given values as the public keys associated with the
// KAS at the given URL, replacing any existing keys that are cached for that URL
// with the same algorithm and URL.
// Only one key per url and algorithm is stored in the cache,
// so only store the most recent known key per url & algorithm pair.
func (s *SDK) StoreKASKeys(url string, keys KasPublicKeySet) error {
	for _, key := range keys.Keys {
		alg, err := formatAlg(key.Alg)
		if err != nil {
			return boundaryError(KindInvalidInput, "StoreKASKeys", err)
		}
		s.kasKeyCache.store(KASInfo{
			URL:       url,
			PublicKey: key.Pem,
			KID:       key.Kid,
			Algorithm: alg,
		})
	}
	return nil
}
