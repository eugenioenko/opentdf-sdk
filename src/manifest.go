package sdk

import (
	"github.com/eugenioenko/goalchemy/std/encoding/json"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
)

// Segment describes one chunk of the payload.
//
// Size and EncryptedSize are optional in the wire format: a writer may omit
// either key whenever its value equals the corresponding manifest-level
// default. Since JSON can't distinguish an omitted key from an explicit 0,
// a zero EncryptedSize always means "omitted" -- ciphertext is never
// legitimately zero bytes -- but a zero Size is ambiguous; see
// resolveSegmentSizes for how it's disambiguated.
type Segment struct {
	// Hash is the base64-encoded integrity value for this encrypted segment.
	Hash string `json:"hash"`
	// Size is the plaintext length in bytes. Zero can mean an omitted manifest default.
	Size int64 `json:"segmentSize"`
	// EncryptedSize is the ciphertext length in bytes, including IV and tag. Zero uses
	// the manifest default.
	EncryptedSize int64 `json:"encryptedSegmentSize"`
}

// RootSignature authenticates the aggregate of the segment integrity values.
type RootSignature struct {
	// Algorithm names the root integrity algorithm, normally HS256.
	Algorithm string `json:"alg"`
	// Signature is the base64-encoded root signature.
	Signature string `json:"sig"`
}

// The reference embeds RootSignature; Goalchemy values cross the boundary
// only with named fields, so it is a named field with the same JSON tag.
type IntegrityInformation struct {
	// RootSignature authenticates the complete sequence of segment integrity values.
	RootSignature RootSignature `json:"rootSignature"`
	// SegmentHashAlgorithm names the integrity algorithm used by each segment.
	SegmentHashAlgorithm string `json:"segmentHashAlg"`
	// DefaultSegmentSize supplies the plaintext byte length for segments that omit it.
	DefaultSegmentSize int64 `json:"segmentSizeDefault"`
	// DefaultEncryptedSegSize supplies the ciphertext byte length for segments that omit
	// it.
	DefaultEncryptedSegSize int64 `json:"encryptedSegmentSizeDefault"`
	// Segments lists segment integrity values and lengths in payload order.
	Segments []Segment `json:"segments"`
}

// resolveSegmentSizes returns the plaintext and ciphertext sizes of seg in
// bytes, substituting the manifest-level default for whichever field the
// writer omitted, and validates that the resolved pair is internally
// consistent. See the reference for why Size alone is ambiguous.
func (i IntegrityInformation) resolveSegmentSizes(seg Segment) (int64, int64, error) {
	encryptedSize := seg.EncryptedSize
	if encryptedSize == 0 {
		encryptedSize = i.DefaultEncryptedSegSize
	}

	size := seg.Size
	if size == 0 && encryptedSize == i.DefaultEncryptedSegSize {
		size = i.DefaultSegmentSize
	}

	if size < 0 || encryptedSize <= 0 {
		return 0, 0, wrapError(KindSegmentSize, "", "", fmt.Errorf("%w: segmentSize=%d encryptedSegmentSize=%d", ErrSegSizeUnresolved, size, encryptedSize))
	}

	if encryptedSize < gcmIvSize+aesBlockSize || size != encryptedSize-(gcmIvSize+aesBlockSize) {
		return 0, 0, wrapError(KindSegmentSize, "", "", fmt.Errorf("%w: segment declares size %d with encrypted size %d", ErrSegSizeMismatch, size, encryptedSize))
	}

	return size, encryptedSize, nil
}

// KeyAccess is one key access object. PolicyBinding is a struct rather than
// the reference's interface{}: the wire form is an object or, in legacy
// files, a bare hash string, which PolicyBinding's JSON methods accept.
type KeyAccess struct {
	// KeyType identifies the key access scheme, such as wrapped or ec-wrapped.
	KeyType string `json:"type"`
	// KasURL identifies the KAS to contact for this key share.
	KasURL string `json:"url"`
	// Protocol names the KAS transport protocol recorded in the manifest.
	Protocol string `json:"protocol"`
	// WrappedKey is the base64-encoded encrypted key share.
	WrappedKey string `json:"wrappedKey"`
	// PolicyBinding authenticates the policy using this key share.
	PolicyBinding PolicyBinding `json:"policyBinding"`
	// EncryptedMetadata contains the encrypted metadata envelope, when present.
	EncryptedMetadata string `json:"encryptedMetadata,omitempty"`
	// KID identifies the KAS key used to wrap this share.
	KID string `json:"kid,omitempty"`
	// SplitID groups alternative KAS entries for the same required key share.
	SplitID string `json:"sid,omitempty"`
	// SchemaVersion records the key access object's schema version.
	SchemaVersion string `json:"schemaVersion,omitempty"`
	// EphemeralPublicKey contains the public key used for elliptic-curve key agreement.
	EphemeralPublicKey string `json:"ephemeralPublicKey,omitempty"`
}

// Binding returns the policy binding, failing when the manifest carried
// none.
func (k KeyAccess) Binding() (PolicyBinding, error) {
	if k.PolicyBinding.Hash == "" {
		return PolicyBinding{}, errors.New("invalid policy object: missing policy binding hash")
	}
	return k.PolicyBinding, nil
}

// PolicyBinding is the HMAC that binds a key split to the policy. A legacy
// manifest stores only the hash as a string; Alg is then empty.
type PolicyBinding struct {
	// Alg names the binding algorithm. Empty preserves the legacy string representation.
	Alg string `json:"alg"`
	// Hash is the base64-encoded policy authentication value.
	Hash string `json:"hash"`
}

type policyBindingObject struct {
	Alg  string `json:"alg"`
	Hash string `json:"hash"`
}

// MarshalJSON writes a legacy hash string when Alg is empty, otherwise an
// algorithm/hash object.
func (p PolicyBinding) MarshalJSON() ([]byte, error) {
	if p.Alg == "" {
		return json.Marshal(p.Hash)
	}
	return json.Marshal(policyBindingObject{Alg: p.Alg, Hash: p.Hash})
}

// UnmarshalJSON accepts a legacy hash string, an algorithm/hash object, or null. Other
// shapes return an error.
func (p *PolicyBinding) UnmarshalJSON(data []byte) error {
	v, err := jsonvalue.Parse(data)
	if err != nil {
		return err
	}
	switch v.Kind() {
	case jsonvalue.KindString:
		p.Alg = ""
		p.Hash = v.String()
		return nil
	case jsonvalue.KindObject:
		hash, ok := v.Get("hash").AsString()
		if !ok {
			return errors.New("invalid policy object: hash is not a string")
		}
		alg, ok := v.Get("alg").AsString()
		if !ok {
			return errors.New("invalid policy object: alg is not a string")
		}
		p.Alg, p.Hash = alg, hash
		return nil
	case jsonvalue.KindNull:
		p.Alg, p.Hash = "", ""
		return nil
	}
	return errors.New("invalid policy object: " + v.String())
}

// Method describes the payload encryption method recorded in a manifest.
type Method struct {
	// Algorithm names the payload cipher.
	Algorithm string `json:"algorithm"`
	// IV contains the manifest-level initialization vector when the format uses one.
	IV string `json:"iv"`
	// IsStreamable records whether the encryption format supports segmented reading.
	IsStreamable bool `json:"isStreamable"`
}

// Payload describes where the encrypted content is stored and its media type.
type Payload struct {
	// Type identifies the payload location type.
	Type string `json:"type"`
	// URL names the payload entry or location.
	URL string `json:"url"`
	// Protocol identifies how the payload location is resolved.
	Protocol string `json:"protocol"`
	// MimeType records the plaintext media type.
	MimeType string `json:"mimeType"`
	// IsEncrypted reports whether the stored payload is encrypted.
	IsEncrypted bool `json:"isEncrypted"`
}

// EncryptionInformation groups key access, policy, cipher, and integrity information.
type EncryptionInformation struct {
	// KeyAccessType identifies how the payload key is made available.
	KeyAccessType string `json:"type"`
	// Policy is the base64-encoded policy JSON.
	Policy string `json:"policy"`
	// KeyAccessObjs lists wrapped key shares and their KAS endpoints.
	KeyAccessObjs []KeyAccess `json:"keyAccess"`
	// Method describes the payload cipher.
	Method Method `json:"method"`
	// IntegrityInformation describes root and segment authentication.
	IntegrityInformation IntegrityInformation `json:"integrityInformation"`
}

// Manifest is the TDF manifest. The reference embeds EncryptionInformation
// and Payload; they are named fields here with the same JSON tags.
type Manifest struct {
	// EncryptionInformation contains the policy, key access, and integrity records.
	EncryptionInformation EncryptionInformation `json:"encryptionInformation"`
	// Payload describes the encrypted payload entry.
	Payload Payload `json:"payload"`
	// Assertions contains the statements bound to this payload.
	Assertions []Assertion `json:"assertions,omitempty"`
	// TDFVersion records the manifest schema version when present.
	TDFVersion string `json:"schemaVersion,omitempty"`
}

// AttributeObject is one entry of a policy's dataAttributes (the reference's
// unexported attributeObject; exported so the policy crosses the boundary).
type AttributeObject struct {
	// Attribute is the attribute-value FQN required by the policy.
	Attribute string `json:"attribute"`
	// DisplayName is the optional human-readable attribute label.
	DisplayName string `json:"displayName"`
	// IsDefault marks a default attribute entry in the policy.
	IsDefault bool `json:"isDefault"`
	// PubKey contains a public key recorded with this attribute, if present.
	PubKey string `json:"pubKey"`
	// KasURL identifies a KAS recorded with this attribute, if present.
	KasURL string `json:"kasURL"`
}

// PolicyBody is the reference's anonymous PolicyObject.Body struct.
type PolicyBody struct {
	// DataAttributes lists the policy's attribute requirements.
	DataAttributes []AttributeObject `json:"dataAttributes"`
	// Dissem lists dissemination identities recorded in the policy.
	Dissem []string `json:"dissem"`
}

// PolicyObject is the decoded policy carried by a TDF manifest.
type PolicyObject struct {
	// UUID identifies the policy instance.
	UUID string `json:"uuid"`
	// Body contains attribute requirements and dissemination identities.
	Body PolicyBody `json:"body"`
}

// EncryptedMetadata is the JSON envelope for encrypted metadata and its IV.
type EncryptedMetadata struct {
	// Cipher is the base64-encoded metadata ciphertext.
	Cipher string `json:"ciphertext"`
	// Iv is the base64-encoded metadata initialization vector.
	Iv string `json:"iv"`
}

// AssertionType classifies a manifest assertion as handling instructions or other.
type AssertionType string

// Scope identifies the part of a TDF described by an assertion.
type Scope string

// AppliesToState identifies whether an assertion describes encrypted or unencrypted
// content.
type AppliesToState string

// BindingMethod names how an assertion is cryptographically bound to its payload.
type BindingMethod string

// AssertionKeyAlg names an assertion signing or verification algorithm.
type AssertionKeyAlg string

const (
	// HandlingAssertion identifies a handling-instructions assertion.
	HandlingAssertion AssertionType = "handling"
	// BaseAssertion identifies an assertion of type other.
	BaseAssertion AssertionType = "other"

	// TrustedDataObjScope applies an assertion to the complete TDF object.
	TrustedDataObjScope Scope = "tdo"
	// PayloadScope applies an assertion to the payload.
	PayloadScope Scope = "payload"

	// Encrypted applies an assertion to encrypted content.
	Encrypted AppliesToState = "encrypted"
	// Unencrypted applies an assertion to plaintext content.
	Unencrypted AppliesToState = "unencrypted"

	// JWS identifies a JSON Web Signature assertion binding.
	JWS BindingMethod = "jws"

	// AssertionKeyAlgRS256 uses an RSA key with SHA-256 for assertion signatures.
	AssertionKeyAlgRS256 AssertionKeyAlg = "RS256"
	// AssertionKeyAlgHS256 uses HMAC-SHA256 for assertion signatures.
	AssertionKeyAlgHS256 AssertionKeyAlg = "HS256"
)

// Assertion is a manifest statement with an optional cryptographic binding.
type Assertion struct {
	// ID identifies the assertion for verification and error reporting.
	ID string `json:"id"`
	// Type classifies the assertion.
	Type AssertionType `json:"type"`
	// Scope identifies the TDF object or payload described by the statement.
	Scope Scope `json:"scope"`
	// AppliesToState identifies the content state described by the statement.
	AppliesToState AppliesToState `json:"appliesToState,omitempty"`
	// Statement contains the assertion's format, schema, and value.
	Statement Statement `json:"statement"`
	// Binding contains the signature binding this statement to the payload.
	Binding Binding `json:"binding,omitempty"`
}

// Statement holds an assertion's content and optional format and schema identifiers.
type Statement struct {
	// Format identifies the statement's encoding or media type.
	Format string `json:"format,omitempty"`
	// Schema identifies the schema used to interpret Value.
	Schema string `json:"schema,omitempty"`
	// Value holds statement text or the compact JSON text of an object value.
	Value string `json:"value,omitempty"`
}

// UnmarshalJSON accepts an object statement value and keeps its compact JSON
// text, as the reference does.
func (s *Statement) UnmarshalJSON(data []byte) error {
	v, err := jsonvalue.Parse(data)
	if err != nil {
		return err
	}
	if v.Kind() != jsonvalue.KindObject {
		return errors.New("statement is not an object")
	}
	s.Format = v.Get("format").String()
	s.Schema = v.Get("schema").String()
	value, ok := v.Lookup("value")
	if !ok || value.IsNull() {
		s.Value = ""
		return nil
	}
	switch value.Kind() {
	case jsonvalue.KindObject:
		text, err := jsonvalue.Encode(value)
		if err != nil {
			return err
		}
		s.Value = string(text)
	case jsonvalue.KindString:
		s.Value = value.String()
	default:
		return fmt.Errorf("value is neither a valid JSON object nor a string: %s", value.String())
	}
	return nil
}

// Binding holds an assertion's binding method and signature.
type Binding struct {
	// Method names the binding method, normally jws.
	Method string `json:"method,omitempty"`
	// Signature contains the assertion's serialized signature.
	Signature string `json:"signature,omitempty"`
}
