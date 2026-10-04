// Package library is the shared, value-only TDF3 library façade. Each operation
// owns its client and all PEM imports. Native hosts supply callback registries
// at the generated entry boundary; protocol behavior remains here.
package library

import (
	"github.com/eugenioenko/goalchemy/lib/callback"
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/errors"
	"opentdf-local/sdk/src"
	"opentdf-local/sdk/src/tdf"
	j "opentdf-local/sdk/src/tdf/json"
)

type KASRoute struct {
	URL        string
	APIBaseURL string
}
type Config struct {
	PlatformURL  string
	KASURL       string
	AllowedKAS   []KASRoute
	IssuerURL    string
	TokenURL     string
	ClientID     string
	ClientSecret string
	// TokenProviderName resolves solely in this call's host callback registry.
	TokenProviderName string
	AllowHTTP         bool
	TimeoutMillis     int64
	KASPublicKeyPEM   string
	KID               string
	KASAlgorithm      string
	SessionAlgorithm  string
	AuthPrivateKeyPEM string
	AuthAlgorithm     string
	DPoP              bool
}
type EncryptOptions struct {
	PolicyBase64         string
	Attributes           []string
	Dissem               []string
	SegmentSize          int64
	HasSegmentSize       bool
	SegmentHashAlgorithm string
	MimeType             string
	Metadata             []byte
	IncludeMetadata      bool
}
type Decrypted struct {
	Payload     []byte
	Metadata    []byte
	HasMetadata bool
	// ManifestJSON contains the authenticated, supported manifest structure.
	ManifestJSON []byte
}
type Failure struct {
	Code                string
	Operation           string
	HTTPStatus          int
	ServerCode          string
	ServerMessage       string
	RequiredObligations []string
	CauseCategory       string
}

func (e *Failure) Error() string { return "sdk: " + e.Operation + ": " + e.Code }
func wrap(op string, err error) error {
	if err == nil {
		return nil
	}
	out := &Failure{Code: "failure", Operation: op}
	if e, ok := err.(*sdk.Error); ok {
		out.Code = e.Code
		out.Operation = e.Operation
		out.HTTPStatus = e.HTTPStatus
		out.ServerCode = e.ServerCode
		out.ServerMessage = e.ServerMessage
		out.RequiredObligations = append([]string(nil), e.RequiredObligations...)
	}
	if errors.Is(err, context.Canceled) {
		out.CauseCategory = "canceled"
	} else if errors.Is(err, context.DeadlineExceeded) {
		out.CauseCategory = "deadline_exceeded"
	} else {
		out.CauseCategory = "source"
	}
	return out
}
func prepare(config Config) (sdk.Config, *crypto.Key, *crypto.Key, error) {
	c := sdk.Config{PlatformURL: config.PlatformURL, KASURL: config.KASURL, IssuerURL: config.IssuerURL, TokenURL: config.TokenURL, ClientID: config.ClientID, ClientSecret: config.ClientSecret, AllowHTTP: config.AllowHTTP, TimeoutMillis: config.TimeoutMillis, KID: config.KID, KASAlgorithm: config.KASAlgorithm, SessionAlgorithm: config.SessionAlgorithm, AuthAlgorithm: config.AuthAlgorithm, DPoP: config.DPoP}
	for _, r := range config.AllowedKAS {
		c.AllowedKAS = append(c.AllowedKAS, sdk.KASRoute{URL: r.URL, APIBaseURL: r.APIBaseURL})
	}
	var kas, auth *crypto.Key
	var err error
	if config.KASPublicKeyPEM != "" {
		kas, err = crypto.ImportPEM(config.KASPublicKeyPEM)
		if err != nil {
			return sdk.Config{}, nil, nil, wrap("import_kas_key", err)
		}
		c.KASPublicKey = kas
	}
	if config.AuthPrivateKeyPEM != "" {
		auth, err = crypto.ImportPEM(config.AuthPrivateKeyPEM)
		if err != nil {
			kas.Close()
			return sdk.Config{}, nil, nil, wrap("import_auth_key", err)
		}
		c.AuthKey = auth
	}
	if config.TokenProviderName != "" {
		name := config.TokenProviderName
		c.TokenProvider = func(ctx context.Context) (sdk.AccessToken, error) {
			response, e := callback.Request(ctx, name, []byte("{}"))
			if e != nil {
				return sdk.AccessToken{}, e
			}
			return parseToken(response)
		}
	}
	return c, kas, auth, nil
}
func parseToken(data []byte) (sdk.AccessToken, error) {
	v, e := j.Parse(data, j.Limits{Bytes: 128 * 1024, Depth: 2, Nodes: 10, StringBytes: 65536})
	if e != nil || v.Kind != j.Object {
		return sdk.AccessToken{}, errors.New("provider: invalid token response")
	}
	for _, n := range v.Names {
		if n != "value" && n != "scheme" && n != "expiresAt" && n != "confirmationJKT" {
			return sdk.AccessToken{}, errors.New("provider: unsupported token field")
		}
	}
	value, ok := v.Get("value")
	if !ok || value.Kind != j.String {
		return sdk.AccessToken{}, errors.New("provider: missing value")
	}
	scheme, ok := v.Get("scheme")
	if !ok || scheme.Kind != j.String {
		return sdk.AccessToken{}, errors.New("provider: missing scheme")
	}
	expiration, ok := v.Get("expiresAt")
	if !ok || expiration.Kind != j.String {
		return sdk.AccessToken{}, errors.New("provider: missing decimal expiry")
	}
	var seconds int64
	if len(expiration.Text) == 0 || len(expiration.Text) > 19 {
		return sdk.AccessToken{}, errors.New("provider: invalid expiry")
	}
	for _, b := range []byte(expiration.Text) {
		if b < '0' || b > '9' || seconds > (9223372036854775807-int64(b-'0'))/10 {
			return sdk.AccessToken{}, errors.New("provider: invalid expiry")
		}
		seconds = seconds*10 + int64(b-'0')
	}
	confirmation, ok := v.Get("confirmationJKT")
	if ok && confirmation.Kind != j.String {
		return sdk.AccessToken{}, errors.New("provider: invalid confirmation")
	}
	return sdk.AccessToken{Value: value.Text, Scheme: scheme.Text, ExpiresAt: seconds, ConfirmationJKT: confirmation.Text}, nil
}
func Encrypt(ctx context.Context, config Config, input []byte, options EncryptOptions) ([]byte, error) {
	c, kas, auth, err := prepare(config)
	if err != nil {
		return nil, err
	}
	defer kas.Close()
	defer auth.Close()
	client, err := sdk.New(c)
	if err != nil {
		return nil, wrap("new", err)
	}
	defer client.Close()
	// SegmentSize remains int64 through all host boundaries; accepted source int
	// is 64-bit. Negative/large values preserve the shared clamping behavior.
	data, err := client.Create(ctx, input, tdf.EncryptConfig{PolicyBase64: options.PolicyBase64, Attributes: options.Attributes, Dissem: options.Dissem, SegmentSize: int(options.SegmentSize), HasSegmentSize: options.HasSegmentSize, SegmentHashAlgorithm: options.SegmentHashAlgorithm, MimeType: options.MimeType, Metadata: options.Metadata, IncludeMetadata: options.IncludeMetadata})
	if err != nil {
		return nil, wrap("create", err)
	}
	return data, nil
}
func Decrypt(ctx context.Context, config Config, input []byte) (Decrypted, error) {
	c, kas, auth, err := prepare(config)
	if err != nil {
		return Decrypted{}, err
	}
	defer kas.Close()
	defer auth.Close()
	client, err := sdk.New(c)
	if err != nil {
		return Decrypted{}, wrap("new", err)
	}
	defer client.Close()
	result, err := client.Decrypt(ctx, input)
	if err != nil {
		return Decrypted{}, wrap("decrypt", err)
	}
	manifest, err := result.Manifest.Marshal()
	if err != nil {
		return Decrypted{}, wrap("manifest", err)
	}
	return Decrypted{Payload: result.Payload, Metadata: result.Metadata, HasMetadata: result.Manifest.Encryption.KeyAccess[0].EncryptedMetadata != "", ManifestJSON: manifest}, nil
}
