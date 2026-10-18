// Package library is the deprecated value-only TDF3 façade: Encrypt and
// Decrypt construct an SDK per call over the handle API in package sdk.
//
// Deprecated: use sdk.New, SDK.EncryptBytes and SDK.DecryptBytes.
package library

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/bytes"
	"github.com/eugenioenko/goalchemy/std/encoding/json"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/strings"
	"opentdf-local/sdk/src"
)

// KASRoute is kept for source compatibility; the SDK now routes through the
// KAS URL in the manifest and the platform's KAS allowlist.
type KASRoute struct {
	// URL identifies an extra KAS URL allowed during decryption.
	URL string
	// APIBaseURL is retained for source compatibility and is not used for routing.
	APIBaseURL string
}

// Config configures the deprecated per-call facade. Prefer the persistent SDK and
// sdk.Options.
type Config struct {
	// PlatformURL is the OpenTDF platform base URL.
	PlatformURL string
	// KASURL overrides the default KAS endpoint at PlatformURL/kas.
	KASURL string
	// AllowedKAS adds KAS URLs to the decryption allowlist.
	AllowedKAS []KASRoute
	// IssuerURL supplies an issuer override when TokenURL is also set.
	IssuerURL string
	// TokenURL overrides the discovered OAuth token endpoint.
	TokenURL string
	// ClientID identifies the OAuth client.
	ClientID string
	// ClientSecret is the OAuth client secret.
	ClientSecret string
	// TokenProviderName resolves solely in this call's host callback registry.
	TokenProviderName string
	// AllowHTTP permits plaintext HTTP for trusted development platforms.
	AllowHTTP bool
	// TimeoutMillis bounds HTTP requests in milliseconds. Zero uses the SDK default.
	TimeoutMillis int64
	// KASPublicKeyPEM supplies the KAS public key for encryption instead of fetching it.
	KASPublicKeyPEM string
	// KID identifies the supplied KAS public key.
	KID string
	// KASAlgorithm selects the KAS key-wrapping algorithm.
	KASAlgorithm string
	// SessionAlgorithm selects the decryption response-session key algorithm.
	SessionAlgorithm string
	// AuthPrivateKeyPEM supplies the signing key used when DPoP is enabled.
	AuthPrivateKeyPEM string
	// AuthAlgorithm selects the DPoP signing algorithm when DPoP is enabled.
	AuthAlgorithm string
	// DPoP enables DPoP configuration for this call.
	DPoP bool
}

// EncryptOptions configures encryption through the deprecated facade.
type EncryptOptions struct {
	// PolicyBase64 is retained for compatibility. Nonempty values are unsupported.
	PolicyBase64 string
	// Attributes lists attribute-value FQNs to include in the policy.
	Attributes []string
	// Dissem is retained for compatibility. Nonempty lists are unsupported.
	Dissem []string
	// SegmentSize sets plaintext segment bytes, subject to the SDK's bounds.
	SegmentSize int64
	// HasSegmentSize makes an explicit zero SegmentSize select the minimum segment size.
	HasSegmentSize bool
	// SegmentHashAlgorithm accepts empty or GMAC. Other values are unsupported.
	SegmentHashAlgorithm string
	// MimeType sets the payload media type.
	MimeType string
	// Metadata contains the metadata bytes to encrypt.
	Metadata []byte
	// IncludeMetadata requests metadata handling even when Metadata is empty.
	IncludeMetadata bool
}

// Decrypted contains owned plaintext, decrypted metadata, and the decoded archive
// manifest.
type Decrypted struct {
	// Payload contains the verified plaintext.
	Payload []byte
	// Metadata contains decrypted metadata when HasMetadata is true.
	Metadata []byte
	// HasMetadata reports whether the archive included encrypted metadata.
	HasMetadata bool
	// ManifestJSON contains the manifest as read from the TDF.
	ManifestJSON []byte
}

// Failure is the façade's error record: Code is the sdk.Error kind.
type Failure struct {
	// Code is the stable SDK error kind.
	Code string
	// Operation identifies the failed operation.
	Operation string
	// HTTPStatus is the service response status when available.
	HTTPStatus int
	// ServerCode is the service's structured error code when available.
	ServerCode string
	// ServerMessage contains service-supplied diagnostic text and may be sensitive.
	ServerMessage string
	// RequiredObligations lists obligation FQNs returned by the service.
	RequiredObligations []string
	// CauseCategory classifies cancellation, deadline, or source failures.
	CauseCategory string
}

// Error returns the facade's operation and stable error code.
func (e *Failure) Error() string { return "sdk: " + e.Operation + ": " + e.Code }

// Version is the SDK version; TDFSpecVersion is the spec version new files
// target.
func Version() string { return sdk.Version }

// TDFSpecVersion returns the TDF specification version used for newly written files.
func TDFSpecVersion() string { return sdk.TDFSpecVersion }

func wrap(op string, err error) error {
	if err == nil {
		return nil
	}
	out := &Failure{Code: "failure", Operation: op, CauseCategory: "source"}
	if e, ok := err.(*sdk.Error); ok {
		out.Code = string(e.Kind)
		if e.Op != "" {
			out.Operation = e.Op
		}
		out.HTTPStatus = e.HTTPStatus
		out.ServerCode = e.ConnectCode
		out.ServerMessage = e.ServiceMessage
		out.RequiredObligations = append([]string(nil), e.RequiredObligations...)
		switch e.Kind {
		case sdk.KindCanceled:
			out.CauseCategory = "canceled"
		case sdk.KindDeadlineExceeded:
			out.CauseCategory = "deadline_exceeded"
		}
	} else if errors.Is(err, context.Canceled) {
		out.CauseCategory = "canceled"
	} else if errors.Is(err, context.DeadlineExceeded) {
		out.CauseCategory = "deadline_exceeded"
	}
	return out
}

func options(config Config) (sdk.Options, error) {
	opts := sdk.Options{
		ClientID:          config.ClientID,
		ClientSecret:      config.ClientSecret,
		TokenEndpoint:     config.TokenURL,
		TokenProviderName: config.TokenProviderName,
		InsecurePlaintext: config.AllowHTTP,
		TimeoutMillis:     config.TimeoutMillis,
		DPoPKeyPEM:        config.AuthPrivateKeyPEM,
	}
	if config.DPoP {
		if config.AuthAlgorithm != "" {
			opts.DPoPAlgorithm = sdk.SigningAlgorithm(config.AuthAlgorithm)
		} else {
			opts.DPoPAlgorithm = sdk.ES256
		}
	}
	// As the reference, a provided configuration skips OIDC discovery, so
	// IssuerURL stands in for the well-known document only together with an
	// explicit TokenURL; alone, the SDK discovers both from the platform.
	if config.IssuerURL != "" && config.TokenURL != "" {
		opts.PlatformConfiguration.Set("platform_issuer", config.IssuerURL)
		idp, err := json.Marshal(map[string]string{"issuer": config.IssuerURL})
		if err != nil {
			return sdk.Options{}, err
		}
		opts.PlatformConfiguration.Set("idp", string(idp))
	}
	return opts, nil
}

func kasInformation(config Config) []sdk.KASInfo {
	kasURL := config.KASURL
	if kasURL == "" {
		kasURL = strings.TrimSuffix(config.PlatformURL, "/") + "/kas"
	}
	return []sdk.KASInfo{{URL: kasURL, PublicKey: config.KASPublicKeyPEM, KID: config.KID, Algorithm: config.KASAlgorithm}}
}

func allowlist(config Config) []string {
	list := []string{strings.TrimSuffix(config.PlatformURL, "/")}
	if config.KASURL != "" {
		list = append(list, config.KASURL)
	}
	for _, r := range config.AllowedKAS {
		list = append(list, r.URL)
	}
	return list
}

// Encrypt encrypts input with one SDK constructed for this call.
// Deprecated: use SDK.EncryptBytes.
func Encrypt(ctx context.Context, config Config, input []byte, options EncryptOptions) ([]byte, error) {
	if options.PolicyBase64 != "" || len(options.Dissem) > 0 || options.SegmentHashAlgorithm != "" && !strings.EqualFold(options.SegmentHashAlgorithm, "GMAC") {
		return nil, wrap("create", errors.New("unsupported in this build: PolicyBase64, Dissem and SegmentHashAlgorithm"))
	}
	opts, err := sdkOptions(config)
	if err != nil {
		return nil, wrap("new", err)
	}
	client, err := sdk.New(ctx, config.PlatformURL, opts)
	if err != nil {
		return nil, wrap("new", err)
	}
	defer client.Close()
	tdfOpts := sdk.TDFOptions{
		Attributes:           options.Attributes,
		DisableAutoconfigure: true,
		KasInformation:       kasInformation(config),
		WrappingKeyAlgorithm: sdk.KeyType(config.KASAlgorithm),
		MimeType:             options.MimeType,
	}
	if options.HasSegmentSize || options.SegmentSize != 0 {
		tdfOpts.SegmentSize = options.SegmentSize
		if tdfOpts.SegmentSize == 0 {
			tdfOpts.SegmentSize = 1
		}
	}
	if options.IncludeMetadata || len(options.Metadata) > 0 {
		tdfOpts.Metadata = string(options.Metadata)
	}
	data, _, err := client.EncryptBytes(ctx, input, tdfOpts)
	if err != nil {
		return nil, wrap("create", err)
	}
	return data, nil
}

// Decrypt decrypts input with one SDK constructed for this call.
// Deprecated: use SDK.DecryptBytes.
func Decrypt(ctx context.Context, config Config, input []byte) (Decrypted, error) {
	opts, err := sdkOptions(config)
	if err != nil {
		return Decrypted{}, wrap("new", err)
	}
	client, err := sdk.New(ctx, config.PlatformURL, opts)
	if err != nil {
		return Decrypted{}, wrap("new", err)
	}
	defer client.Close()
	readerOpts := sdk.ReaderOptions{KasAllowlist: allowlist(config), SessionKeyType: sdk.KeyType(config.SessionAlgorithm)}
	reader, err := client.LoadTDF(ctx, bytes.NewReader(input), readerOpts)
	if err != nil {
		return Decrypted{}, wrap("decrypt", err)
	}
	defer reader.Close()
	manifest := reader.Manifest()
	manifestJSON, err := json.Marshal(manifest)
	if err != nil {
		return Decrypted{}, wrap("manifest", err)
	}
	hasMetadata := len(manifest.EncryptionInformation.KeyAccessObjs) > 0 && manifest.EncryptionInformation.KeyAccessObjs[0].EncryptedMetadata != ""
	var metadata []byte
	if hasMetadata {
		metadata, err = reader.UnencryptedMetadata()
		if err != nil {
			return Decrypted{}, wrap("decrypt", err)
		}
	}
	payload, err := client.DecryptBytes(ctx, input, readerOpts)
	if err != nil {
		return Decrypted{}, wrap("decrypt", err)
	}
	return Decrypted{Payload: payload, Metadata: metadata, HasMetadata: hasMetadata, ManifestJSON: manifestJSON}, nil
}

func sdkOptions(config Config) (sdk.Options, error) { return options(config) }
