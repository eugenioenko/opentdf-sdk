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
	Hash          string `json:"hash"`
	Size          int64  `json:"segmentSize"`
	EncryptedSize int64  `json:"encryptedSegmentSize"`
}

type RootSignature struct {
	Algorithm string `json:"alg"`
	Signature string `json:"sig"`
}

// The reference embeds RootSignature; Goalchemy values cross the boundary
// only with named fields, so it is a named field with the same JSON tag.
type IntegrityInformation struct {
	RootSignature           RootSignature `json:"rootSignature"`
	SegmentHashAlgorithm    string        `json:"segmentHashAlg"`
	DefaultSegmentSize      int64         `json:"segmentSizeDefault"`
	DefaultEncryptedSegSize int64         `json:"encryptedSegmentSizeDefault"`
	Segments                []Segment     `json:"segments"`
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
	KeyType            string        `json:"type"`
	KasURL             string        `json:"url"`
	Protocol           string        `json:"protocol"`
	WrappedKey         string        `json:"wrappedKey"`
	PolicyBinding      PolicyBinding `json:"policyBinding"`
	EncryptedMetadata  string        `json:"encryptedMetadata,omitempty"`
	KID                string        `json:"kid,omitempty"`
	SplitID            string        `json:"sid,omitempty"`
	SchemaVersion      string        `json:"schemaVersion,omitempty"`
	EphemeralPublicKey string        `json:"ephemeralPublicKey,omitempty"`
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
	Alg  string `json:"alg"`
	Hash string `json:"hash"`
}

type policyBindingObject struct {
	Alg  string `json:"alg"`
	Hash string `json:"hash"`
}

func (p PolicyBinding) MarshalJSON() ([]byte, error) {
	if p.Alg == "" {
		return json.Marshal(p.Hash)
	}
	return json.Marshal(policyBindingObject{Alg: p.Alg, Hash: p.Hash})
}

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

type Method struct {
	Algorithm    string `json:"algorithm"`
	IV           string `json:"iv"`
	IsStreamable bool   `json:"isStreamable"`
}

type Payload struct {
	Type        string `json:"type"`
	URL         string `json:"url"`
	Protocol    string `json:"protocol"`
	MimeType    string `json:"mimeType"`
	IsEncrypted bool   `json:"isEncrypted"`
}

type EncryptionInformation struct {
	KeyAccessType        string               `json:"type"`
	Policy               string               `json:"policy"`
	KeyAccessObjs        []KeyAccess          `json:"keyAccess"`
	Method               Method               `json:"method"`
	IntegrityInformation IntegrityInformation `json:"integrityInformation"`
}

// Manifest is the TDF manifest. The reference embeds EncryptionInformation
// and Payload; they are named fields here with the same JSON tags.
type Manifest struct {
	EncryptionInformation EncryptionInformation `json:"encryptionInformation"`
	Payload               Payload               `json:"payload"`
	Assertions            []Assertion           `json:"assertions,omitempty"`
	TDFVersion            string                `json:"schemaVersion,omitempty"`
}

// AttributeObject is one entry of a policy's dataAttributes (the reference's
// unexported attributeObject; exported so the policy crosses the boundary).
type AttributeObject struct {
	Attribute   string `json:"attribute"`
	DisplayName string `json:"displayName"`
	IsDefault   bool   `json:"isDefault"`
	PubKey      string `json:"pubKey"`
	KasURL      string `json:"kasURL"`
}

// PolicyBody is the reference's anonymous PolicyObject.Body struct.
type PolicyBody struct {
	DataAttributes []AttributeObject `json:"dataAttributes"`
	Dissem         []string          `json:"dissem"`
}

type PolicyObject struct {
	UUID string     `json:"uuid"`
	Body PolicyBody `json:"body"`
}

type EncryptedMetadata struct {
	Cipher string `json:"ciphertext"`
	Iv     string `json:"iv"`
}

// Assertion types from the reference's assertion.go; signing and verification
// are in assertion.go.
type AssertionType string

type Scope string

type AppliesToState string

type BindingMethod string

type AssertionKeyAlg string

const (
	HandlingAssertion AssertionType = "handling"
	BaseAssertion     AssertionType = "other"

	TrustedDataObjScope Scope = "tdo"
	PayloadScope        Scope = "payload"

	Encrypted   AppliesToState = "encrypted"
	Unencrypted AppliesToState = "unencrypted"

	JWS BindingMethod = "jws"

	AssertionKeyAlgRS256 AssertionKeyAlg = "RS256"
	AssertionKeyAlgHS256 AssertionKeyAlg = "HS256"
)

type Assertion struct {
	ID             string         `json:"id"`
	Type           AssertionType  `json:"type"`
	Scope          Scope          `json:"scope"`
	AppliesToState AppliesToState `json:"appliesToState,omitempty"`
	Statement      Statement      `json:"statement"`
	Binding        Binding        `json:"binding,omitempty"`
}

type Statement struct {
	Format string `json:"format,omitempty"`
	Schema string `json:"schema,omitempty"`
	Value  string `json:"value,omitempty"`
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

type Binding struct {
	Method    string `json:"method,omitempty"`
	Signature string `json:"signature,omitempty"`
}
