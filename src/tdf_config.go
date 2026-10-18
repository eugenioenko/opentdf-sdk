package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/std/encoding/protojson"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/net"
	"github.com/eugenioenko/goalchemy/std/net/url"
	"github.com/eugenioenko/goalchemy/std/strings"
	"opentdf-local/sdk/src/protocol/policy"
)

const (
	tdf3KeySize            = 2048
	defaultMaxManifestSize = 10 * 1024 * 1024 // 10 MB
	defaultSegmentSize     = 2 * 1024 * 1024  // 2mb
	maxSegmentSize         = defaultSegmentSize * 2
	minSegmentSize         = 16 * 1024
	// DefaultRSAKeySize is the default RSA key length in bits.
	DefaultRSAKeySize = 2048
	// ECKeySize256 is the P-256 key length in bits.
	ECKeySize256 = 256
	// ECKeySize384 is the P-384 key length in bits.
	ECKeySize384 = 384
	// ECKeySize521 is the P-521 key length in bits.
	ECKeySize521 = 521

	// inputSizeUnknown marks a payload whose length cannot be established
	// before it is read.
	inputSizeUnknown = -1
)

const (
	schemeHTTPS     = "https"
	schemeSeperator = "://"
)

// KeyType names a KAS key algorithm, the reference's ocrypto.KeyType set.
type KeyType string

const (
	// RSA2048Key selects RSA-2048 key wrapping or response sessions.
	RSA2048Key KeyType = "rsa:2048"
	// RSA4096Key selects RSA-4096 key wrapping or response sessions.
	RSA4096Key KeyType = "rsa:4096"
	// EC256Key selects the P-256 elliptic curve.
	EC256Key KeyType = "ec:secp256r1"
	// EC384Key selects the P-384 elliptic curve.
	EC384Key KeyType = "ec:secp384r1"
	// EC521Key selects the P-521 elliptic curve.
	EC521Key KeyType = "ec:secp521r1"

	// Post-quantum key types are not planned; the names are kept so that
	// policy keys and KAOs that carry them are reported, not misread.
	mlkem768Key                 KeyType = "mlkem:768"
	mlkem1024Key                KeyType = "mlkem:1024"
	hybridXWingKey              KeyType = "hpqt:xwing"
	hybridSecp256r1MLKEM768Key  KeyType = "hpqt:secp256r1-mlkem768"
	hybridSecp384r1MLKEM1024Key KeyType = "hpqt:secp384r1-mlkem1024"
)

// IsECKeyType reports whether kt is an EC key type.
func IsECKeyType(kt KeyType) bool {
	return kt == EC256Key || kt == EC384Key || kt == EC521Key
}

// isPQKeyType reports whether kt is one of the post-quantum key types, which
// are not planned.
func isPQKeyType(kt KeyType) bool {
	switch kt {
	case mlkem768Key, mlkem1024Key, hybridXWingKey, hybridSecp256r1MLKEM768Key, hybridSecp384r1MLKEM1024Key:
		return true
	}
	return false
}

// IsRSAKeyType reports whether kt is an RSA key type.
func IsRSAKeyType(kt KeyType) bool {
	return kt == RSA2048Key || kt == RSA4096Key
}

// RootIntegrityAlg is the algorithm that signs a TDF's aggregate hash. HS256
// is the only value; see the reference for why GMAC cannot be one.
type RootIntegrityAlg int

// RootHS256 authenticates the aggregate segment hashes with HMAC-SHA256.
const RootHS256 RootIntegrityAlg = iota

// SegmentIntegrityAlg is the algorithm that authenticates a single segment's
// bytes.
type SegmentIntegrityAlg int

const (
	// SegmentHS256 authenticates each encrypted segment with HMAC-SHA256.
	SegmentHS256 SegmentIntegrityAlg = iota
	// SegmentGMAC uses each segment's AES-GCM authentication tag as its integrity value.
	SegmentGMAC
)

// String returns the manifest algorithm name, or a diagnostic name for an unknown
// value.
func (a RootIntegrityAlg) String() string {
	if a == RootHS256 {
		return hmacIntegrityAlgorithm
	}
	return fmt.Sprintf("RootIntegrityAlg(%d)", int(a))
}

// String returns the manifest algorithm name, or a diagnostic name for an unknown
// value.
func (a SegmentIntegrityAlg) String() string {
	switch a {
	case SegmentHS256:
		return hmacIntegrityAlgorithm
	case SegmentGMAC:
		return gmacIntegrityAlgorithm
	}
	return fmt.Sprintf("SegmentIntegrityAlg(%d)", int(a))
}

// KASInfo contains Key Access Server information.
type KASInfo struct {
	// URL of the KAS server
	URL string
	// Public key can be empty.
	// If it is empty, the public key will be fetched from the KAS server.
	PublicKey string
	// Key identifier associated with the given key, if present.
	KID string
	// The algorithm associated with this key
	Algorithm string
	// If this KAS should be used as the default for 'encrypt' calls
	Default bool
}

// AssertionConfig describes an assertion to add to a TDF: the reference's
// AssertionConfig. An empty SigningKey signs with HS256 over the payload key.
type AssertionConfig struct {
	// ID identifies this assertion in the manifest and verification-key lookup.
	ID string
	// Type classifies the assertion as handling instructions or another assertion.
	Type AssertionType
	// Scope identifies whether the assertion describes the TDF object or its payload.
	Scope Scope
	// AppliesToState identifies the encrypted or unencrypted state described by the
	// assertion.
	AppliesToState AppliesToState
	// Statement is the content to bind to the payload.
	Statement Statement
	// SigningKey selects an explicit signing key. Empty uses HS256 with the payload key.
	SigningKey AssertionKey
}

// TDFOptions configures CreateTDF and EncryptBytes. Zero values are the
// reference defaults; the Go host adapter's With* options fill it.
type TDFOptions struct {
	// Attributes lists attribute-value FQNs to attach to the policy. It cannot be
	// combined with AttributeValues.
	Attributes []string
	// AttributeValues are policy.Value messages in protobuf JSON
	// (WithDataAttributeValues): autoconfigure reads their grants and keys
	// instead of looking the attributes up. They replace Attributes.
	AttributeValues []string
	// DisableAutoconfigure uses the supplied KAS information instead of discovering
	// policy grants.
	DisableAutoconfigure bool
	// KasInformation supplies KAS endpoints and optional cached public keys for
	// encryption.
	KasInformation []KASInfo
	// WrappingKeyAlgorithm selects key wrapping. Empty leaves selection to the resolved
	// KAS configuration.
	WrappingKeyAlgorithm KeyType
	// SegmentSize sets plaintext bytes per segment. Zero uses 2 MiB; other values are
	// clamped to 16 KiB through 4 MiB.
	SegmentSize int64
	// InputSize declares plaintext length in bytes. Zero means unknown; negative values
	// are rejected.
	InputSize int64
	// MimeType sets the payload media type. Empty uses application/octet-stream.
	MimeType string
	// Metadata is text encrypted alongside the payload key and returned by
	// Reader.UnencryptedMetadata.
	Metadata string
	// Assertions lists assertions to sign and include in the manifest.
	Assertions []AssertionConfig
	// SystemMetadataAssertion adds the SDK's default system-metadata assertion when true.
	SystemMetadataAssertion bool
	// TargetMode selects format compatibility. Empty or 4.3.0 is accepted; other values
	// are unsupported.
	TargetMode string
}

// TDFConfig Internal config struct for building TDF options.
type TDFConfig struct {
	autoconfigure              bool
	defaultSegmentSize         int64
	inputSize                  int64
	metaData                   string
	mimeType                   string
	attributes                 []AttributeValueFQN
	attributeValues            []*policy.Value
	kasInfoList                []KASInfo
	kaoTemplate                []kaoTpl
	splitPlan                  []keySplitStep
	preferredKeyWrapAlg        KeyType
	useHex                     bool
	excludeVersionFromManifest bool
	assertions                 []AssertionConfig
	addDefaultAssertion        bool
}

func newTDFConfig(opts TDFOptions) (*TDFConfig, error) {
	c := &TDFConfig{
		autoconfigure:       !opts.DisableAutoconfigure,
		defaultSegmentSize:  defaultSegmentSize,
		inputSize:           inputSizeUnknown,
		addDefaultAssertion: false,
	}
	if len(opts.AttributeValues) > 0 && len(opts.Attributes) > 0 {
		return nil, errors.New("Attributes and AttributeValues are exclusive: AttributeValues replaces the attributes")
	}
	for _, a := range opts.Attributes {
		v, err := newAttributeValueFQN(a)
		if err != nil {
			return nil, err
		}
		c.attributes = append(c.attributes, v)
	}
	for i, data := range opts.AttributeValues {
		a := &policy.Value{}
		if err := protojson.Unmarshal([]byte(data), a); err != nil {
			return nil, fmt.Errorf("attribute value at index %d is not a policy.Value in protobuf JSON: %w", i, err)
		}
		afqn, err := newAttributeValueFQN(a.GetFqn())
		if err != nil {
			return nil, err
		}
		c.attributeValues = append(c.attributeValues, a)
		c.attributes = append(c.attributes, afqn)
	}
	c.kasInfoList = append(make([]KASInfo, 0), opts.KasInformation...)
	c.metaData = opts.Metadata
	c.mimeType = opts.MimeType
	if opts.SegmentSize != 0 {
		size := opts.SegmentSize
		if size > maxSegmentSize {
			size = maxSegmentSize
		} else if size < minSegmentSize {
			size = minSegmentSize
		}
		c.defaultSegmentSize = size
	}
	if opts.InputSize < 0 {
		return nil, fmt.Errorf("WithInputSize: size must not be negative, got %d", opts.InputSize)
	}
	if opts.InputSize > 0 {
		c.inputSize = opts.InputSize
	}
	wrapAlg := opts.WrappingKeyAlgorithm
	switch {
	case wrapAlg == "" || IsRSAKeyType(wrapAlg) || IsECKeyType(wrapAlg):
		c.preferredKeyWrapAlg = wrapAlg
	case isPQKeyType(wrapAlg):
		return nil, notPlannedPQ("CreateTDF", "WrappingKeyAlgorithm "+string(wrapAlg))
	default:
		return nil, fmt.Errorf("unsupported key type: %s", string(opts.WrappingKeyAlgorithm))
	}
	c.assertions = append(c.assertions, opts.Assertions...)
	c.addDefaultAssertion = opts.SystemMetadataAssertion
	if opts.TargetMode != "" && opts.TargetMode != hexSemverThreshold {
		return nil, unsupported("CreateTDF", "TargetMode "+opts.TargetMode)
	}
	return c, nil
}

// Schema Validation where 0 = none (skip), 1 = lax (allowing novel entries, 'falsy' values for unkowns), 2 = strict (rejecting novel entries, strict match to manifest schema)
type SchemaValidationIntensity int

const (
	// Skip disables manifest schema validation.
	Skip SchemaValidationIntensity = iota
	// Lax names permissive schema validation. ReaderOptions currently rejects this mode.
	Lax
	// Strict names strict schema validation. ReaderOptions currently rejects this mode.
	Strict
	unreasonable = 100
)

// ReaderOptions configures LoadTDF and DecryptBytes. Zero values are the
// reference defaults: assertions verified, the allowlist fetched from the KAS
// registry, 10 MiB manifests, RSA-2048 session keys.
type ReaderOptions struct {
	// DisableAssertionVerification skips assertion verification when true. The default
	// verifies assertions.
	DisableAssertionVerification bool
	// AssertionVerificationKeys provides keys for explicitly signed assertions. Payload-
	// key assertions need no extra key.
	AssertionVerificationKeys AssertionVerificationKeys
	// KasAllowlist lists allowed KAS origins. Empty discovers them from the platform
	// registry.
	KasAllowlist []string
	// IgnoreAllowlist allows manifest KAS endpoints without allowlist checks. Keep false
	// for untrusted files.
	IgnoreAllowlist bool
	// MaxManifestSize bounds manifest bytes. Zero uses 10 MiB; the archive reader's hard
	// limit still applies.
	MaxManifestSize int64
	// SchemaValidation selects schema validation. Only Skip is currently supported.
	SchemaValidation SchemaValidationIntensity
	// SessionKeyType selects the response-session key algorithm. Empty reuses the SDK's
	// session key.
	SessionKeyType KeyType
	// FulfillableObligationFQNs declares obligations the caller can fulfill. Empty
	// inherits the SDK-level list.
	FulfillableObligationFQNs []string
}

// TDFReaderConfig holds validated reader settings internally. Consumers configure
// readers with ReaderOptions.
type TDFReaderConfig struct {
	disableAssertionVerification bool
	verifiers                    AssertionVerificationKeys

	schemaValidationIntensity SchemaValidationIntensity
	kasSessionKey             *crypto.Key
	kasSessionKeyType         KeyType
	ownsSessionKey            bool
	kasAllowlist              AllowList // KAS URLs that are allowed to be used for reading TDFs
	ignoreAllowList           bool      // If true, the kasAllowlist will be ignored, and all KAS URLs will be allowed
	fulfillableObligationFQNs []string
	maxManifestSize           int64
}

// AllowList stores normalized KAS origins. Paths are ignored and HTTPS defaults to
// port 443.
type AllowList map[string]bool

func getKasAddress(kasURL string) (string, error) {
	// default to https if no scheme is provided
	if !strings.Contains(kasURL, schemeSeperator) {
		kasURL = schemeHTTPS + schemeSeperator + kasURL
	}
	parsedURL, err := url.Parse(kasURL)
	if err != nil {
		return "", fmt.Errorf("cannot parse kas url(%s): %w", kasURL, err)
	}
	if parsedURL.Hostname() == "" {
		return "", fmt.Errorf("no host parsed from url: %s", kasURL)
	}

	// Default to port 443 if scheme is https and no port is provided
	if parsedURL.Port() != "" {
		parsedURL.Host = net.JoinHostPort(parsedURL.Hostname(), parsedURL.Port())
	} else if parsedURL.Scheme == schemeHTTPS {
		parsedURL.Host = net.JoinHostPort(parsedURL.Hostname(), "443")
	}

	// Reconstruct the URL with only the scheme, host, and port
	parsedURL.Path = ""
	parsedURL.RawQuery = ""
	parsedURL.Fragment = ""

	return parsedURL.String(), nil
}

func newAllowList(kasList []string) (AllowList, error) {
	allowList := make(AllowList, len(kasList))
	for _, kasURL := range kasList {
		err := allowList.Add(kasURL)
		if err != nil {
			return nil, fmt.Errorf("error adding kas url(%s) to allowlist: %w", kasURL, err)
		}
	}
	return allowList, nil
}

// IsAllowed reports whether the normalized KAS origin is present. Nil lists and
// malformed URLs return false.
func (a AllowList) IsAllowed(kasURL string) bool {
	if a == nil {
		return false // No allowlist, so no URLs are allowed
	}
	kasAddress, err := getKasAddress(kasURL)
	if err != nil {
		return false // If we can't parse the URL, we can't allow it
	}
	_, ok := a[kasAddress]
	return ok
}

// Add adds a normalized KAS origin or returns a URL error. Initialize the map before
// calling it.
func (a AllowList) Add(kasURL string) error {
	if a == nil {
		a = make(AllowList)
	}
	kasAddress, err := getKasAddress(kasURL)
	if err != nil {
		// If we can't parse the URL, we can't add it to the allowlist
		return fmt.Errorf("error parsing kas url(%s): %w", kasURL, err)
	} else if kasAddress == "" {
		return fmt.Errorf("kas url(%s) not parsed", kasURL)
	}
	a[kasAddress] = true
	return nil
}

func newTDFReaderConfig(sessionKey *crypto.Key, opts ReaderOptions) (*TDFReaderConfig, error) {
	c := &TDFReaderConfig{
		disableAssertionVerification: opts.DisableAssertionVerification,
		verifiers:                    opts.AssertionVerificationKeys,
		maxManifestSize:              defaultMaxManifestSize,
		kasSessionKey:                sessionKey,
		kasSessionKeyType:            RSA2048Key,
	}
	if opts.MaxManifestSize < 0 {
		return nil, errors.New("max manifest size must be greater than 0")
	}
	if opts.MaxManifestSize > 0 {
		c.maxManifestSize = opts.MaxManifestSize
	}
	if opts.SchemaValidation != Skip {
		return nil, unsupported("LoadTDF", "SchemaValidation")
	}
	if len(opts.KasAllowlist) > 0 {
		allowlist, err := newAllowList(opts.KasAllowlist)
		if err != nil {
			return nil, fmt.Errorf("failed to create kas allowlist: %w", err)
		}
		c.kasAllowlist = allowlist
	}
	c.ignoreAllowList = opts.IgnoreAllowlist
	c.fulfillableObligationFQNs = append([]string(nil), opts.FulfillableObligationFQNs...)
	sessionAlg := opts.SessionKeyType
	switch {
	case sessionAlg == "":
	case IsRSAKeyType(sessionAlg) || IsECKeyType(sessionAlg):
		key, err := newKeyPair(sessionAlg)
		if err != nil {
			return nil, fmt.Errorf("failed to create %v key pair: %w", string(sessionAlg), err)
		}
		c.kasSessionKey = key
		c.kasSessionKeyType = sessionAlg
		c.ownsSessionKey = true
	case isPQKeyType(sessionAlg):
		return nil, notPlannedPQ("LoadTDF", "SessionKeyType "+string(sessionAlg))
	default:
		return nil, fmt.Errorf("failed to create %v key pair: unsupported key type", string(opts.SessionKeyType))
	}
	if c.kasSessionKey == nil {
		key, err := newKeyPair(RSA2048Key)
		if err != nil {
			return nil, fmt.Errorf("failed to create %v key pair: %w", string(RSA2048Key), err)
		}
		c.kasSessionKey = key
		c.ownsSessionKey = true
	}
	return c, nil
}

// newKeyPair generates a key pair for the given key type.
func newKeyPair(keyType KeyType) (*crypto.Key, error) {
	switch keyType {
	case RSA2048Key:
		return crypto.GenerateRSA2048()
	case RSA4096Key:
		return crypto.GenerateRSA4096()
	case EC256Key:
		return crypto.GenerateP256()
	case EC384Key:
		return crypto.GenerateP384()
	case EC521Key:
		return crypto.GenerateP521()
	}
	return nil, fmt.Errorf("unsupported key type: %s", string(keyType))
}
