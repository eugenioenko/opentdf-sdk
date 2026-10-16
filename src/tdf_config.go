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
	DefaultRSAKeySize      = 2048
	ECKeySize256           = 256
	ECKeySize384           = 384
	ECKeySize521           = 521

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
	RSA2048Key KeyType = "rsa:2048"
	RSA4096Key KeyType = "rsa:4096"
	EC256Key   KeyType = "ec:secp256r1"
	EC384Key   KeyType = "ec:secp384r1"
	EC521Key   KeyType = "ec:secp521r1"

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

const RootHS256 RootIntegrityAlg = iota

// SegmentIntegrityAlg is the algorithm that authenticates a single segment's
// bytes.
type SegmentIntegrityAlg int

const (
	SegmentHS256 SegmentIntegrityAlg = iota
	SegmentGMAC
)

func (a RootIntegrityAlg) String() string {
	if a == RootHS256 {
		return hmacIntegrityAlgorithm
	}
	return fmt.Sprintf("RootIntegrityAlg(%d)", int(a))
}

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

// AssertionConfig describes an assertion to add to a TDF (reference
// AssertionConfig without its signing key; both arrive in Phase 5).
type AssertionConfig struct {
	ID             string
	Type           AssertionType
	Scope          Scope
	AppliesToState AppliesToState
	Statement      Statement
}

// TDFOptions configures CreateTDF and EncryptBytes. Zero values are the
// reference defaults; the Go host adapter's With* options fill it.
type TDFOptions struct {
	Attributes []string
	// AttributeValues are policy.Value messages in protobuf JSON
	// (WithDataAttributeValues): autoconfigure reads their grants and keys
	// instead of looking the attributes up. They replace Attributes.
	AttributeValues         []string
	DisableAutoconfigure    bool
	KasInformation          []KASInfo
	WrappingKeyAlgorithm    KeyType
	SegmentSize             int64
	InputSize               int64
	MimeType                string
	Metadata                string
	Assertions              []AssertionConfig
	SystemMetadataAssertion bool
	TargetMode              string
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
		v, err := NewAttributeValueFQN(a)
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
		afqn, err := NewAttributeValueFQN(a.GetFqn())
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
	switch opts.WrappingKeyAlgorithm {
	case "", RSA2048Key, EC256Key:
		c.preferredKeyWrapAlg = opts.WrappingKeyAlgorithm
	case RSA4096Key, EC384Key, EC521Key:
		return nil, unsupported("CreateTDF", "WrappingKeyAlgorithm "+string(opts.WrappingKeyAlgorithm))
	default:
		return nil, fmt.Errorf("unsupported key type: %s", string(opts.WrappingKeyAlgorithm))
	}
	if len(opts.Assertions) > 0 {
		return nil, unsupported("CreateTDF", "Assertions")
	}
	if opts.SystemMetadataAssertion {
		return nil, unsupported("CreateTDF", "SystemMetadataAssertion")
	}
	if opts.TargetMode != "" && opts.TargetMode != hexSemverThreshold {
		return nil, unsupported("CreateTDF", "TargetMode "+opts.TargetMode)
	}
	return c, nil
}

// Schema Validation where 0 = none (skip), 1 = lax (allowing novel entries, 'falsy' values for unkowns), 2 = strict (rejecting novel entries, strict match to manifest schema)
type SchemaValidationIntensity int

const (
	Skip SchemaValidationIntensity = iota
	Lax
	Strict
	unreasonable = 100
)

// ReaderOptions configures LoadTDF and DecryptBytes. Zero values are the
// reference defaults: assertions verified, the allowlist fetched from the KAS
// registry, 10 MiB manifests, RSA-2048 session keys.
type ReaderOptions struct {
	DisableAssertionVerification bool
	KasAllowlist                 []string
	IgnoreAllowlist              bool
	MaxManifestSize              int64
	SchemaValidation             SchemaValidationIntensity
	SessionKeyType               KeyType
	FulfillableObligationFQNs    []string
}

type TDFReaderConfig struct {
	disableAssertionVerification bool

	schemaValidationIntensity SchemaValidationIntensity
	kasSessionKey             *crypto.Key
	kasSessionKeyType         KeyType
	ownsSessionKey            bool
	kasAllowlist              AllowList // KAS URLs that are allowed to be used for reading TDFs
	ignoreAllowList           bool      // If true, the kasAllowlist will be ignored, and all KAS URLs will be allowed
	fulfillableObligationFQNs []string
	maxManifestSize           int64
}

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
	switch opts.SessionKeyType {
	case "":
	case RSA2048Key, EC256Key:
		key, err := newKeyPair(opts.SessionKeyType)
		if err != nil {
			return nil, fmt.Errorf("failed to create %v key pair: %w", string(opts.SessionKeyType), err)
		}
		c.kasSessionKey = key
		c.kasSessionKeyType = opts.SessionKeyType
		c.ownsSessionKey = true
	case RSA4096Key, EC384Key, EC521Key:
		return nil, unsupported("LoadTDF", "SessionKeyType "+string(opts.SessionKeyType))
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
