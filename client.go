package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/lib/sync"
	"github.com/eugenioenko/goalchemy/lib/time"
	"opentdf-local/sdk/tdf"
	j "opentdf-local/sdk/tdf/json"
)

// KASRoute binds a manifest destination to a caller-trusted Connect API base.
// Paths on APIBaseURL are retained, supporting explicit reverse-proxy mounts.
type KASRoute struct {
	URL        string
	APIBaseURL string
}

// Config is immutable after New. ClientSecrets are server-side credentials.
// Browser hosts should supply TokenProvider with a matching AuthKey for DPoP.
type Config struct {
	PlatformURL   string
	KASURL        string
	AllowedKAS    []KASRoute
	IssuerURL     string
	TokenURL      string
	ClientID      string
	ClientSecret  string
	TokenProvider func(context.Context) (AccessToken, error)
	AllowHTTP     bool
	TimeoutMillis int64
	KASPublicKey  *crypto.Key
	KID           string
	// KASAlgorithm selects discovery and creation wrapping (default rsa:2048).
	KASAlgorithm string
	// SessionAlgorithm selects rewrap response sessions independently (default rsa:2048).
	SessionAlgorithm string
	AuthKey          *crypto.Key
	AuthAlgorithm    string
	DPoP             bool
}
type route struct {
	kas string
	api string
}
type Client struct {
	mu         sync.Mutex
	closed     bool
	config     Config
	routes     []route
	authKey    *crypto.Key
	ownAuth    bool
	thumbprint string
	nonces     []nonceEntry
	token      AccessToken
	tokenGate  chan bool
	lifetime   context.Context
	cancel     context.CancelFunc
}

// New validates trusted routing and authentication, without network calls.
// Supplied keys remain caller-owned pointers; Close never closes caller keys.
func New(config Config) (*Client, error) {
	if config.KASAlgorithm == "" {
		config.KASAlgorithm = "rsa:2048"
	}
	if config.SessionAlgorithm == "" {
		config.SessionAlgorithm = "rsa:2048"
	}
	if !wrappingAlgorithm(config.KASAlgorithm) {
		return nil, failure("new", "unsupported_kas_algorithm", nil)
	}
	if !wrappingAlgorithm(config.SessionAlgorithm) {
		return nil, failure("new", "unsupported_session_algorithm", nil)
	}
	if config.KASPublicKey != nil {
		if e := validateWrappingKey(config.KASPublicKey, config.KASAlgorithm); e != nil {
			return nil, e
		}
	} else if config.KID != "" {
		return nil, failure("new", "kid_without_key", nil)
	}
	if config.TimeoutMillis == 0 {
		config.TimeoutMillis = 15000
	}
	if config.TimeoutMillis < 1 || config.TimeoutMillis > 300000 {
		return nil, failure("new", "invalid_timeout", nil)
	}
	if config.AuthAlgorithm == "" {
		config.AuthAlgorithm = "ES256"
	}
	if config.AuthAlgorithm != "RS256" && config.AuthAlgorithm != "ES256" {
		return nil, failure("new", "unsupported_auth_algorithm", nil)
	}
	p, e := parseEndpoint(config.PlatformURL, config.AllowHTTP)
	if e != nil {
		return nil, e
	}
	config.PlatformURL = p.String()
	if config.KASURL == "" {
		config.KASURL = config.PlatformURL + "/kas"
	}
	primary, e := parseEndpoint(config.KASURL, config.AllowHTTP)
	if e != nil {
		return nil, e
	}
	config.KASURL = primary.String()
	c := &Client{config: config, tokenGate: make(chan bool, 1)}
	c.tokenGate <- true
	if len(config.AllowedKAS) == 0 {
		config.AllowedKAS = []KASRoute{{URL: config.KASURL, APIBaseURL: config.PlatformURL}}
	}
	if len(config.AllowedKAS) > 64 {
		return nil, failure("new", "route_limit", nil)
	}
	primaryAllowed := false
	for _, r := range config.AllowedKAS {
		kas, err := parseEndpoint(r.URL, config.AllowHTTP)
		if err != nil {
			return nil, err
		}
		api, err := parseEndpoint(r.APIBaseURL, config.AllowHTTP)
		if err != nil {
			return nil, err
		}
		for _, old := range c.routes {
			if old.kas == kas.String() {
				return nil, failure("new", "duplicate_route", nil)
			}
		}
		c.routes = append(c.routes, route{kas: kas.String(), api: api.String()})
		if kas.String() == config.KASURL {
			primaryAllowed = true
		}
	}
	if !primaryAllowed {
		return nil, failure("new", "kas_not_allowed", nil)
	}
	if config.TokenProvider == nil {
		if config.ClientID == "" || len(config.ClientID) > 4096 || config.ClientSecret == "" || len(config.ClientSecret) > 4096 {
			return nil, failure("new", "invalid_credentials", nil)
		}
		issuer, err := parseEndpoint(config.IssuerURL, config.AllowHTTP)
		if err != nil {
			return nil, err
		}
		config.IssuerURL = issuer.String()
		if config.TokenURL != "" {
			token, err := parseEndpoint(config.TokenURL, config.AllowHTTP)
			if err != nil {
				return nil, err
			}
			config.TokenURL = token.String()
		}
	}
	// Do not retain caller-owned route slices.
	config.AllowedKAS = nil
	c.config = config
	c.authKey = config.AuthKey
	if c.authKey == nil {
		if config.AuthAlgorithm == "RS256" {
			c.authKey, e = crypto.GenerateRSA2048()
		} else {
			c.authKey, e = crypto.GenerateP256()
		}
		if e != nil {
			return nil, failure("new", "auth_key", e)
		}
		c.ownAuth = true
	}
	// Validate key/algorithm agreement and possession without exporting private data.
	_, e = signRequestToken(c.authKey, config.AuthAlgorithm, []byte("{}"))
	if e != nil {
		if c.ownAuth {
			c.authKey.Close()
		}
		return nil, e
	}
	if config.DPoP {
		c.thumbprint, e = authThumbprint(c.authKey, config.AuthAlgorithm)
		if e != nil {
			if c.ownAuth {
				c.authKey.Close()
			}
			return nil, e
		}
	}
	c.lifetime, c.cancel = context.WithCancel(context.Background())
	return c, nil
}

// Close is idempotent and cancels in-flight HTTP operations. No key handle is copied.
func (c *Client) Close() {
	if c == nil {
		return
	}
	c.mu.Lock()
	c.closed = true
	c.token = AccessToken{}
	c.nonces = nil
	key := c.authKey
	owned := c.ownAuth
	c.mu.Unlock()
	if c.cancel != nil {
		c.cancel()
	}
	if owned && key != nil {
		key.Close()
	}
}
func (c *Client) check(ctx context.Context, op string) error {
	if c == nil || c.lifetime == nil {
		return failure(op, "uninitialized", nil)
	}
	if ctx == nil {
		return failure(op, "invalid_context", nil)
	}
	if ctx.Err() != nil {
		return failure(op, "canceled", ctx.Err())
	}
	c.mu.Lock()
	closed := c.closed
	c.mu.Unlock()
	if closed {
		return failure(op, "closed", nil)
	}
	return nil
}
func (c *Client) route(raw string) (string, error) {
	u, e := parseEndpoint(raw, c.config.AllowHTTP)
	if e != nil {
		return "", e
	}
	for _, r := range c.routes {
		if r.kas == u.String() {
			return r.api, nil
		}
	}
	return "", failure("destination", "kas_not_allowed", nil)
}
func (c *Client) exchange(ctx context.Context, method, url string, headers []string, body []byte, op string) (j.Value, error) {
	return c.exchangeProof(ctx, method, url, headers, body, op, "", false)
}
func (c *Client) resourceExchange(ctx context.Context, method, url string, token AccessToken, body []byte, op string) (j.Value, error) {
	headers := []string{"Content-Type", "application/json", "Connect-Protocol-Version", "1", "Authorization", token.Scheme + " " + token.Value}
	return c.exchangeProof(ctx, method, url, headers, body, op, token.Value, c.config.DPoP)
}
func (c *Client) exchangeProof(ctx context.Context, method, url string, headers []string, body []byte, op, token string, proof bool) (j.Value, error) {
	if e := c.check(ctx, op); e != nil {
		return j.Value{}, e
	}
	child, cancel := c.operationContext(ctx)
	status, responseHeaders, data, e := c.proofHTTP(child, method, url, headers, body, token, proof)
	cancel()
	if e != nil {
		if status == 401 {
			c.mu.Lock()
			c.token = AccessToken{}
			c.mu.Unlock()
		}
		if proof {
			return j.Value{}, e
		}
		return j.Value{}, failure(op, "transport", e)
	}
	if status < 200 || status >= 300 {
		code := "http_status"
		server := ""
		message := ""
		v, err := j.Parse(data, jsonLimits())
		if err == nil {
			server = text(v, "code")
			message = text(v, "message")
		}
		if status == 401 {
			c.mu.Lock()
			c.token = AccessToken{}
			c.mu.Unlock()
			code = "unauthenticated"
		}
		if status == 403 {
			code = "denied"
		}
		return j.Value{}, &Error{Code: code, Operation: op, HTTPStatus: status, ServerCode: server, ServerMessage: message}
	}
	contentTypes := 0
	for i := 0; i+1 < len(responseHeaders); i += 2 {
		if lower(responseHeaders[i]) == "content-type" {
			contentTypes++
			typ := lower(responseHeaders[i+1])
			end := len(typ)
			for j, b := range []byte(typ) {
				if b == ';' {
					end = j
					break
				}
			}
			if typ[:end] != "application/json" {
				return j.Value{}, failure(op, "invalid_content_type", nil)
			}
		}
	}
	if contentTypes != 1 {
		return j.Value{}, failure(op, "invalid_content_type", nil)
	}
	v, e := j.Parse(data, jsonLimits())
	if e != nil || v.Kind != j.Object {
		return j.Value{}, failure(op, "invalid_json", e)
	}
	if e := c.check(ctx, op); e != nil {
		return j.Value{}, e
	}
	return v, nil
}
func (c *Client) operationContext(ctx context.Context) (context.Context, context.CancelFunc) {
	child, cancel := context.WithTimeout(ctx, time.Duration(c.config.TimeoutMillis)*time.Millisecond)
	go func() {
		select {
		case <-c.lifetime.Done():
			cancel()
		case <-child.Done():
		}
	}()
	return child, cancel
}

// PublicKey discovers a wrapping key through the trusted primary route. The
// returned handle belongs to the caller and must be closed after use.
func (c *Client) PublicKey(ctx context.Context) (*crypto.Key, string, error) {
	if e := c.check(ctx, "public_key"); e != nil {
		return nil, "", e
	}
	api, e := c.route(c.config.KASURL)
	if e != nil {
		return nil, "", e
	}
	token, e := c.accessToken(ctx)
	if e != nil {
		return nil, "", e
	}
	body, e := j.Marshal(object([]string{"algorithm", "fmt", "v"}, []j.Value{j.Str(c.config.KASAlgorithm), j.Str("pkcs8"), j.Str("2")}), jsonLimits())
	if e != nil {
		return nil, "", e
	}
	v, e := c.resourceExchange(ctx, "POST", api+"/kas.AccessService/PublicKey", token, body, "public_key")
	if e != nil {
		return nil, "", e
	}
	if !keysAllowed(v, []string{"publicKey", "kid"}) || text(v, "publicKey") == "" || text(v, "kid") == "" {
		return nil, "", failure("public_key", "invalid_response", nil)
	}
	key, e := importWrappingSPKI(text(v, "publicKey"), c.config.KASAlgorithm)
	if e != nil {
		return nil, "", failure("public_key", "invalid_key", e)
	}
	return key, text(v, "kid"), nil
}

// Create encrypts bytes with the primary trusted KAS. Options carry policy,
// segmentation and metadata. Wrapping keys/URL/kid are resolved by the client.
func (c *Client) Create(ctx context.Context, plain []byte, options tdf.EncryptConfig) ([]byte, error) {
	if e := c.check(ctx, "create"); e != nil {
		return nil, e
	}
	if options.Algorithm != "" && options.Algorithm != c.config.KASAlgorithm {
		return nil, failure("create", "unsupported_kas_algorithm", nil)
	}
	if options.KASPublicKey != nil && options.KASPublicKey != c.config.KASPublicKey || options.KASURL != "" && options.KASURL != c.config.KASURL || options.KID != "" && options.KID != c.config.KID {
		return nil, failure("create", "conflicting_kas_options", nil)
	}
	key := c.config.KASPublicKey
	kid := c.config.KID
	owned := false
	var e error
	if key == nil {
		key, kid, e = c.PublicKey(ctx)
		if e != nil {
			return nil, e
		}
		owned = true
	}
	options.KASPublicKey = key
	options.KASURL = c.config.KASURL
	options.KID = kid
	options.Algorithm = c.config.KASAlgorithm
	if e := validateWrappingKey(key, c.config.KASAlgorithm); e != nil {
		if owned {
			key.Close()
		}
		return nil, e
	}
	data, e := tdf.Encrypt(plain, options)
	if owned && key != nil {
		key.Close()
	}
	if e != nil {
		return nil, failure("create", "encryption", e)
	}
	if e := c.check(ctx, "create"); e != nil {
		return nil, e
	}
	return data, nil
}

// Decrypt validates the manifest and KAS route before obtaining any token. It
// sends a signed grouped request and decrypts only after real KAS authorization.
func (c *Client) Decrypt(ctx context.Context, data []byte) (tdf.Decrypted, error) {
	if e := c.check(ctx, "decrypt"); e != nil {
		return tdf.Decrypted{}, e
	}
	prepared, stage, e := tdf.PrepareDecryption(data)
	if e != nil {
		return tdf.Decrypted{}, failure("decrypt", stage, e)
	}
	m := prepared.Manifest()
	if m.Encryption.KeyAccess[0].Type != "wrapped" && m.Encryption.KeyAccess[0].Type != "ec-wrapped" {
		return tdf.Decrypted{}, failure("decrypt", "unsupported_kas_algorithm", nil)
	}
	api, e := c.route(m.Encryption.KeyAccess[0].URL)
	if e != nil {
		return tdf.Decrypted{}, e
	}
	token, e := c.accessToken(ctx)
	if e != nil {
		return tdf.Decrypted{}, e
	}
	var session *crypto.Key
	if c.config.SessionAlgorithm == "ec:secp256r1" {
		session, e = crypto.GenerateP256()
	} else {
		session, e = crypto.GenerateRSA2048()
	}
	if e != nil {
		return tdf.Decrypted{}, failure("rewrap", "session_key", e)
	}
	result, e := c.rewrap(ctx, m, api, token, session)
	session.Close()
	if e != nil {
		return tdf.Decrypted{}, e
	}
	decrypted, e := prepared.Decrypt(result)
	// Best-effort zeroing of the recovered share; host allocations may retain copies.
	for i := range result {
		result[i] = 0
	}
	if e != nil {
		return tdf.Decrypted{}, failure("decrypt", "integrity", e)
	}
	if e := c.check(ctx, "decrypt"); e != nil {
		return tdf.Decrypted{}, e
	}
	return decrypted, nil
}
func (c *Client) rewrap(ctx context.Context, m tdf.Manifest, api string, token AccessToken, session *crypto.Key) ([]byte, error) {
	public, e := session.PublicPEM()
	if e != nil {
		return nil, failure("rewrap", "session_key", e)
	}
	body, e := rewrapBody(m, public)
	if e != nil {
		return nil, failure("rewrap", "request", e)
	}
	srt, e := signRequestToken(c.authKey, c.config.AuthAlgorithm, body)
	if e != nil {
		return nil, e
	}
	request, e := j.Marshal(object([]string{"signedRequestToken"}, []j.Value{j.Str(srt)}), jsonLimits())
	if e != nil {
		return nil, e
	}
	v, e := c.resourceExchange(ctx, "POST", api+"/kas.AccessService/Rewrap", token, request, "rewrap")
	if e != nil {
		return nil, e
	}
	wrapped, e := parseRewrap(v, c.config.SessionAlgorithm)
	if e != nil {
		return nil, e
	}
	var key []byte
	if c.config.SessionAlgorithm == "ec:secp256r1" {
		public, err := importWrappingSPKI(text(v, "sessionPublicKey"), "ec:secp256r1")
		if err != nil {
			return nil, failure("rewrap", "session_public_key", err)
		}
		secret, err := crypto.ECDH(session, public)
		public.Close()
		if err != nil {
			return nil, failure("rewrap", "session_unwrap", err)
		}
		salt, err := crypto.SHA256([]byte("TDF"))
		if err != nil {
			wipe(secret)
			return nil, failure("rewrap", "session_unwrap", err)
		}
		wrapping, err := crypto.HKDFSHA256(secret, salt, nil, 32)
		wipe(secret)
		if err != nil {
			return nil, failure("rewrap", "session_unwrap", err)
		}
		key, e = crypto.AES256GCMDecrypt(wrapping, wrapped[:12], wrapped[12:], nil)
		wipe(wrapping)
	} else {
		key, e = crypto.RSAOAEPDecrypt(session, wrapped)
	}
	if e != nil || len(key) != 32 {
		for i := range key {
			key[i] = 0
		}
		return nil, failure("rewrap", "session_unwrap", e)
	}
	return key, nil
}

func wipe(data []byte) {
	for i := range data {
		data[i] = 0
	}
}
func wrappingAlgorithm(algorithm string) bool {
	return algorithm == "rsa:2048" || algorithm == "ec:secp256r1"
}
func validateWrappingKey(key *crypto.Key, algorithm string) error {
	jwk, e := key.PublicJWK()
	if e != nil || len(jwk) != 6 {
		return failure("public_key", "invalid_key", e)
	}
	if algorithm == "rsa:2048" && jwk[0] == "RSA" && jwk[1] == "" && jwk[4] == "" && jwk[5] == "" {
		n, e := encoding.Base64URLDecode(jwk[2])
		if e == nil && len(n) == 256 && n[0] >= 128 && jwk[3] != "" {
			return nil
		}
	}
	if algorithm == "ec:secp256r1" && jwk[0] == "EC" && jwk[1] == "P-256" && jwk[2] == "" && jwk[3] == "" {
		x, e := encoding.Base64URLDecode(jwk[4])
		y, err := encoding.Base64URLDecode(jwk[5])
		if e == nil && err == nil && len(x) == 32 && len(y) == 32 {
			return nil
		}
	}
	return failure("public_key", "unsupported_algorithm", nil)
}
func importWrappingSPKI(pem string, algorithm string) (*crypto.Key, error) {
	// Wire keys must be SPKI public material. ImportPEM's wider private/certificate
	// support is deliberately not accepted in discovery or response sessions.
	prefix := "-----BEGIN PUBLIC KEY-----"
	if len(pem) < len(prefix) || pem[:len(prefix)] != prefix {
		return nil, failure("public_key", "invalid_spki", nil)
	}
	key, e := crypto.ImportPEM(pem)
	if e != nil {
		return nil, e
	}
	if e := validateWrappingKey(key, algorithm); e != nil {
		key.Close()
		return nil, e
	}
	return key, nil
}
