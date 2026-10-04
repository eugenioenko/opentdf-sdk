package tdf

import (
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/lib/errors"
	j "opentdf-local/sdk/src/tdf/json"
)

const SchemaVersion = "4.3.0"
const FrameOverhead = 28
const MaxSegmentBytes = 4 * 1024 * 1024
const MaxSegments = 10000

type Payload struct {
	Type        string
	URL         string
	Protocol    string
	MimeType    string
	IsEncrypted bool
}
type Method struct {
	Algorithm    string
	IV           string
	IsStreamable bool
}
type PolicyBinding struct {
	Algorithm    string
	Hash         string
	LegacyString bool
}
type KeyAccess struct {
	Type               string
	URL                string
	Protocol           string
	WrappedKey         string
	Binding            PolicyBinding
	EncryptedMetadata  string
	KID                string
	SID                string
	SchemaVersion      string
	EphemeralPublicKey string
}
type RootSignature struct {
	Algorithm string
	Signature string
}
type Segment struct {
	Hash             string
	Size             int
	EncryptedSize    int
	HasSize          bool
	HasEncryptedSize bool
}
type Integrity struct {
	Root                 RootSignature
	SegmentHashAlgorithm string
	DefaultSize          int
	DefaultEncryptedSize int
	Segments             []Segment
}
type EncryptionInformation struct {
	Type      string
	Policy    string
	KeyAccess []KeyAccess
	Method    Method
	Integrity Integrity
}

// Raw preserves the entire parsed document for inspection of unsupported fields.
// Parsing alone does not authorize decryption. Call ValidateModern before using it.
type Manifest struct {
	SchemaVersion string
	Payload       Payload
	Encryption    EncryptionInformation
	Assertions    []j.Value
	Raw           j.Value
}
type Attribute struct {
	Attribute   string
	DisplayName string
	IsDefault   bool
	PubKey      string
	KASURL      string
}
type Policy struct {
	UUID       string
	Attributes []Attribute
	Dissem     []string
	Raw        j.Value
}
type EncryptedMetadata struct {
	Ciphertext string
	IV         string
}

type decoder struct{ err error }

func (d *decoder) fail(s string) {
	if d.err == nil {
		d.err = errors.New(s)
	}
}
func (d *decoder) object(v j.Value) j.Value {
	if v.Kind != j.Object {
		d.fail("manifest: expected object")
	}
	return v
}
func (d *decoder) field(v j.Value, n string, required bool) (j.Value, bool) {
	x, ok := v.Get(n)
	if !ok && required {
		d.fail("manifest: missing " + n)
	}
	return x, ok
}
func (d *decoder) str(v j.Value, n string, required bool) string {
	x, ok := d.field(v, n, required)
	if !ok {
		return ""
	}
	if x.Kind != j.String {
		d.fail("manifest: expected string " + n)
	}
	return x.Text
}
func (d *decoder) boolean(v j.Value, n string, required bool) bool {
	x, ok := d.field(v, n, required)
	if !ok {
		return false
	}
	if x.Kind != j.Boolean {
		d.fail("manifest: expected boolean " + n)
	}
	return x.Bool
}
func (d *decoder) integer(v j.Value, n string, required bool) (int, bool) {
	x, ok := d.field(v, n, required)
	if !ok {
		return 0, false
	}
	i, e := x.Integer(128 * 1024 * 1024)
	if e != nil {
		d.fail("manifest: invalid integer " + n)
	}
	return i, true
}
func (d *decoder) obj(v j.Value, n string) j.Value { x, _ := d.field(v, n, true); return d.object(x) }
func (d *decoder) array(v j.Value, n string, required bool, max int) []j.Value {
	x, ok := d.field(v, n, required)
	if !ok {
		return nil
	}
	if x.Kind != j.Array || len(x.Children) > max {
		d.fail("manifest: array shape or limit " + n)
		return nil
	}
	return x.Children
}

func ParseManifest(b []byte) (Manifest, error) {
	raw, e := j.Parse(b, j.DefaultLimits())
	if e != nil {
		return Manifest{}, e
	}
	d := decoder{}
	v := d.object(raw)
	m := Manifest{Raw: raw}
	m.SchemaVersion = d.str(v, "schemaVersion", false)
	p := d.obj(v, "payload")
	m.Payload = Payload{d.str(p, "type", true), d.str(p, "url", true), d.str(p, "protocol", true), d.str(p, "mimeType", false), d.boolean(p, "isEncrypted", true)}
	x := d.obj(v, "encryptionInformation")
	m.Encryption.Type = d.str(x, "type", true)
	m.Encryption.Policy = d.str(x, "policy", true)
	for _, a := range d.array(x, "keyAccess", true, 64) {
		a = d.object(a)
		k := KeyAccess{}
		k.Type = d.str(a, "type", true)
		k.URL = d.str(a, "url", true)
		k.Protocol = d.str(a, "protocol", true)
		k.WrappedKey = d.str(a, "wrappedKey", true)
		k.KID = d.str(a, "kid", false)
		k.SID = d.str(a, "sid", false)
		k.SchemaVersion = d.str(a, "schemaVersion", false)
		k.EphemeralPublicKey = d.str(a, "ephemeralPublicKey", false)
		k.EncryptedMetadata = d.str(a, "encryptedMetadata", false)
		binding, _ := d.field(a, "policyBinding", true)
		if binding.Kind == j.String {
			k.Binding = PolicyBinding{Hash: binding.Text, LegacyString: true}
		} else {
			binding = d.object(binding)
			k.Binding = PolicyBinding{Algorithm: d.str(binding, "alg", true), Hash: d.str(binding, "hash", true)}
		}
		m.Encryption.KeyAccess = append(m.Encryption.KeyAccess, k)
	}
	method := d.obj(x, "method")
	m.Encryption.Method = Method{d.str(method, "algorithm", true), d.str(method, "iv", false), d.boolean(method, "isStreamable", true)}
	in := d.obj(x, "integrityInformation")
	r := d.obj(in, "rootSignature")
	it := Integrity{Root: RootSignature{d.str(r, "alg", false), d.str(r, "sig", true)}, SegmentHashAlgorithm: d.str(in, "segmentHashAlg", false)}
	it.DefaultSize, _ = d.integer(in, "segmentSizeDefault", true)
	it.DefaultEncryptedSize, _ = d.integer(in, "encryptedSegmentSizeDefault", true)
	for _, s := range d.array(in, "segments", true, MaxSegments) {
		s = d.object(s)
		seg := Segment{Hash: d.str(s, "hash", true)}
		seg.Size, seg.HasSize = d.integer(s, "segmentSize", false)
		seg.EncryptedSize, seg.HasEncryptedSize = d.integer(s, "encryptedSegmentSize", false)
		it.Segments = append(it.Segments, seg)
	}
	m.Encryption.Integrity = it
	m.Assertions = d.array(v, "assertions", false, 1000)
	if d.err != nil {
		return Manifest{}, d.err
	}
	return m, nil
}
func ParsePolicy(b []byte) (Policy, error) {
	raw, e := j.Parse(b, j.DefaultLimits())
	if e != nil {
		return Policy{}, e
	}
	d := decoder{}
	v := d.object(raw)
	p := Policy{UUID: d.str(v, "uuid", true), Raw: raw}
	body := d.obj(v, "body")
	for _, a := range d.policyArray(body, "dataAttributes", 10000) {
		a = d.object(a)
		p.Attributes = append(p.Attributes, Attribute{d.str(a, "attribute", true), d.str(a, "displayName", false), d.boolean(a, "isDefault", false), d.str(a, "pubKey", false), d.str(a, "kasURL", false)})
	}
	for _, a := range d.policyArray(body, "dissem", 10000) {
		if a.Kind != j.String {
			d.fail("policy: expected dissemination string")
		}
		p.Dissem = append(p.Dissem, a.Text)
	}
	if d.err != nil {
		return Policy{}, d.err
	}
	return p, nil
}

// DecodePolicy leaves the exact base64 string in Manifest.Encryption.Policy untouched.
func DecodePolicy(s string) (Policy, error) {
	if len(s) > 14*1024*1024 {
		return Policy{}, errors.New("policy: encoded size limit")
	}
	b, e := encoding.Base64Decode(s)
	if e != nil {
		return Policy{}, e
	}
	return ParsePolicy(b)
}
func ParseEncryptedMetadata(s string) (EncryptedMetadata, error) {
	if len(s) > 1024*1024 {
		return EncryptedMetadata{}, errors.New("metadata: encoded size limit")
	}
	b, e := encoding.Base64Decode(s)
	if e != nil {
		return EncryptedMetadata{}, e
	}
	v, e := j.Parse(b, j.DefaultLimits())
	if e != nil {
		return EncryptedMetadata{}, e
	}
	d := decoder{}
	v = d.object(v)
	m := EncryptedMetadata{d.str(v, "ciphertext", true), d.str(v, "iv", true)}
	if e := known(v, []string{"ciphertext", "iv"}); e != nil {
		return EncryptedMetadata{}, e
	}
	if d.err != nil {
		return EncryptedMetadata{}, d.err
	}
	frame, e := encoding.Base64Decode(m.Ciphertext)
	if e != nil || len(frame) < FrameOverhead {
		return EncryptedMetadata{}, errors.New("metadata: invalid frame")
	}
	iv, e := encoding.Base64Decode(m.IV)
	if e != nil || len(iv) != 12 {
		return EncryptedMetadata{}, errors.New("metadata: invalid IV")
	}
	for i := 0; i < 12; i++ {
		if iv[i] != frame[i] {
			return EncryptedMetadata{}, errors.New("metadata: IV/frame mismatch")
		}
	}
	return m, nil
}

func known(v j.Value, names []string) error {
	if v.Kind == j.Null {
		return nil
	}
	if v.Kind != j.Object {
		return errors.New("manifest: expected object")
	}
	for _, n := range v.Names {
		found := false
		for _, k := range names {
			if n == k {
				found = true
			}
		}
		if !found {
			return errors.New("manifest: unsupported field " + n)
		}
	}
	return nil
}
func manifestKnown(v j.Value) error {
	if v.Kind == j.Null {
		return nil
	}
	if e := known(v, []string{"schemaVersion", "payload", "encryptionInformation", "assertions"}); e != nil {
		return e
	}
	p, _ := v.Get("payload")
	if e := known(p, []string{"type", "url", "protocol", "mimeType", "isEncrypted"}); e != nil {
		return e
	}
	x, _ := v.Get("encryptionInformation")
	if e := known(x, []string{"type", "policy", "keyAccess", "method", "integrityInformation"}); e != nil {
		return e
	}
	a, _ := x.Get("keyAccess")
	for _, k := range a.Children {
		if e := known(k, []string{"type", "url", "protocol", "wrappedKey", "policyBinding", "kid", "sid", "schemaVersion", "ephemeralPublicKey", "encryptedMetadata"}); e != nil {
			return e
		}
		b, _ := k.Get("policyBinding")
		if b.Kind == j.Object {
			if e := known(b, []string{"alg", "hash"}); e != nil {
				return e
			}
		}
	}
	method, _ := x.Get("method")
	if e := known(method, []string{"algorithm", "iv", "isStreamable"}); e != nil {
		return e
	}
	in, _ := x.Get("integrityInformation")
	if e := known(in, []string{"rootSignature", "segmentHashAlg", "segmentSizeDefault", "encryptedSegmentSizeDefault", "segments"}); e != nil {
		return e
	}
	root, _ := in.Get("rootSignature")
	if e := known(root, []string{"alg", "sig"}); e != nil {
		return e
	}
	segs, _ := in.Get("segments")
	for _, s := range segs.Children {
		if e := known(s, []string{"hash", "segmentSize", "encryptedSegmentSize"}); e != nil {
			return e
		}
	}
	return nil
}
func policyKnown(v j.Value) error {
	if v.Kind == j.Null {
		return nil
	}
	if e := known(v, []string{"uuid", "body"}); e != nil {
		return e
	}
	b, _ := v.Get("body")
	if e := known(b, []string{"dataAttributes", "dissem"}); e != nil {
		return e
	}
	a, _ := b.Get("dataAttributes")
	for _, x := range a.Children {
		if e := known(x, []string{"attribute", "displayName", "isDefault", "pubKey", "kasURL"}); e != nil {
			return e
		}
	}
	return nil
}
func lower(s string) string {
	b := []byte(s)
	for i, c := range b {
		if c >= 'A' && c <= 'Z' {
			b[i] = c + 32
		}
	}
	return string(b)
}
func decodedLen(s string, n int) error {
	if len(s) > 4*((n+2)/3) {
		return errors.New("manifest: encoded value size")
	}
	b, e := encoding.Base64Decode(s)
	if e != nil || len(b) != n {
		return errors.New("manifest: invalid encoded value length")
	}
	return nil
}
func (in Integrity) ResolveSegment(s Segment) (int, int, error) {
	size := in.DefaultSize
	if s.HasSize {
		size = s.Size
	}
	encrypted := in.DefaultEncryptedSize
	if s.HasEncryptedSize {
		encrypted = s.EncryptedSize
	}
	if size < 0 || size > MaxSegmentBytes || encrypted < FrameOverhead || encrypted > MaxSegmentBytes+FrameOverhead || encrypted-FrameOverhead != size {
		return 0, 0, errors.New("manifest: inconsistent segment framing")
	}
	return size, encrypted, nil
}
func (m Manifest) ResolvedSegmentHashAlgorithm() string {
	a := m.Encryption.Integrity.SegmentHashAlgorithm
	if a == "" {
		a = m.Encryption.Integrity.Root.Algorithm
	}
	return lower(a)
}

// ValidateModern validates the deliberately narrow initial profile. It does not
// authenticate hashes, decrypt metadata, or validate/authorize any KAS URL.
func (m Manifest) ValidateModern(payloadBytes int) error {
	if e := manifestKnown(m.Raw); e != nil {
		return e
	}
	if m.SchemaVersion != SchemaVersion {
		return errors.New("manifest: unsupported schema version")
	}
	if len(m.Assertions) > 0 {
		return errors.New("manifest: assertions unsupported")
	}
	p := m.Payload
	if p.Type != "reference" || p.URL != PayloadEntry || p.Protocol != "zip" || !p.IsEncrypted {
		return errors.New("manifest: unsupported payload profile")
	}
	x := m.Encryption
	if x.Type != "split" || x.Method.Algorithm != "AES-256-GCM" || !x.Method.IsStreamable {
		return errors.New("manifest: unsupported encryption profile")
	}
	if len(x.KeyAccess) != 1 {
		return errors.New("manifest: multiple KAS/shares unsupported")
	}
	k := x.KeyAccess[0]
	if k.Type != "wrapped" && k.Type != "ec-wrapped" {
		return errors.New("manifest: unsupported key access type")
	}
	if k.Protocol != "kas" || k.URL == "" || (k.SchemaVersion != "" && k.SchemaVersion != "1.0") || k.Binding.LegacyString || lower(k.Binding.Algorithm) != "hs256" {
		return errors.New("manifest: unsupported KAS binding/profile")
	}
	if e := decodedLen(k.Binding.Hash, 64); e != nil {
		return e
	}
	hash, e := encoding.Base64Decode(k.Binding.Hash)
	if e != nil {
		return e
	}
	for _, c := range hash {
		if !((c >= '0' && c <= '9') || (c >= 'a' && c <= 'f')) {
			return errors.New("manifest: policy binding must encode lowercase hex")
		}
	}
	if k.Type == "wrapped" {
		if k.EphemeralPublicKey != "" {
			return errors.New("manifest: unexpected RSA ephemeral key")
		}
		if e := decodedLen(k.WrappedKey, 256); e != nil {
			return e
		}
	} else {
		if k.EphemeralPublicKey == "" {
			return errors.New("manifest: missing EC ephemeral key")
		}
		if e := decodedLen(k.WrappedKey, 60); e != nil {
			return e
		}
	}
	if k.EncryptedMetadata != "" {
		if _, e := ParseEncryptedMetadata(k.EncryptedMetadata); e != nil {
			return e
		}
	}
	if x.Method.IV != "" {
		if e := decodedLen(x.Method.IV, 12); e != nil {
			return e
		}
	}
	policy, e := DecodePolicy(x.Policy)
	if e != nil {
		return e
	}
	if e := policy.ValidateModern(); e != nil {
		return e
	}
	in := x.Integrity
	if lower(in.Root.Algorithm) != "hs256" {
		return errors.New("manifest: unsupported root signature algorithm")
	}
	if e := decodedLen(in.Root.Signature, 32); e != nil {
		return e
	}
	alg := m.ResolvedSegmentHashAlgorithm()
	if alg != "gmac" && alg != "hs256" {
		return errors.New("manifest: unsupported segment hash algorithm")
	}
	if in.DefaultSize <= 0 || in.DefaultSize > MaxSegmentBytes || in.DefaultEncryptedSize != in.DefaultSize+FrameOverhead || len(in.Segments) > MaxSegments {
		return errors.New("manifest: invalid segment defaults")
	}
	if payloadBytes < 0 || payloadBytes > 64*1024*1024 {
		return errors.New("manifest: payload limit")
	}
	total := 0
	for _, s := range in.Segments {
		_, n, e := in.ResolveSegment(s)
		if e != nil {
			return e
		}
		if n > payloadBytes-total {
			return errors.New("manifest: segment exceeds payload")
		}
		total += n
		hashSize := 32
		if alg == "gmac" {
			hashSize = 16
		}
		if e := decodedLen(s.Hash, hashSize); e != nil {
			return e
		}
	}
	if total != payloadBytes {
		return errors.New("manifest: segment/payload size mismatch")
	}
	return nil
}
func (p Policy) ValidateModern() error {
	if e := policyKnown(p.Raw); e != nil {
		return e
	}
	if p.UUID == "" || len(p.UUID) > 128 || len(p.Attributes) > 10000 || len(p.Dissem) > 10000 {
		return errors.New("policy: invalid identifier or bounds")
	}
	for _, a := range p.Attributes {
		if a.Attribute == "" {
			return errors.New("policy: empty attribute")
		}
	}
	return nil
}

func obj(names []string, values []j.Value) j.Value { return j.Obj(names, values) }
func appendString(v j.Value, n string, s string) j.Value {
	if s != "" {
		v.Names = append(v.Names, n)
		v.Children = append(v.Children, j.Str(s))
	}
	return v
}
func (m Manifest) Marshal() ([]byte, error) {
	if len(m.Encryption.KeyAccess) > 64 || len(m.Encryption.Integrity.Segments) > MaxSegments || len(m.Assertions) > 1000 {
		return nil, errors.New("manifest: serialization array limit")
	}
	if e := manifestKnown(m.Raw); e != nil {
		return nil, e
	}
	p := m.Payload
	payload := obj([]string{"type", "url", "protocol", "isEncrypted", "mimeType"}, []j.Value{j.Str(p.Type), j.Str(p.URL), j.Str(p.Protocol), j.Bool(p.IsEncrypted), j.Str(p.MimeType)})
	x := m.Encryption
	kaos := []j.Value{}
	for _, k := range x.KeyAccess {
		binding := obj([]string{"alg", "hash"}, []j.Value{j.Str(k.Binding.Algorithm), j.Str(k.Binding.Hash)})
		if k.Binding.LegacyString {
			binding = j.Str(k.Binding.Hash)
		}
		v := obj([]string{"type", "url", "protocol", "wrappedKey", "policyBinding"}, []j.Value{j.Str(k.Type), j.Str(k.URL), j.Str(k.Protocol), j.Str(k.WrappedKey), binding})
		v = appendString(v, "kid", k.KID)
		v = appendString(v, "sid", k.SID)
		v = appendString(v, "schemaVersion", k.SchemaVersion)
		v = appendString(v, "ephemeralPublicKey", k.EphemeralPublicKey)
		v = appendString(v, "encryptedMetadata", k.EncryptedMetadata)
		kaos = append(kaos, v)
	}
	method := obj([]string{"algorithm", "iv", "isStreamable"}, []j.Value{j.Str(x.Method.Algorithm), j.Str(x.Method.IV), j.Bool(x.Method.IsStreamable)})
	in := x.Integrity
	segs := []j.Value{}
	for _, s := range in.Segments {
		v := obj([]string{"hash"}, []j.Value{j.Str(s.Hash)})
		if s.HasSize {
			v.Names = append(v.Names, "segmentSize")
			v.Children = append(v.Children, j.Int(s.Size))
		}
		if s.HasEncryptedSize {
			v.Names = append(v.Names, "encryptedSegmentSize")
			v.Children = append(v.Children, j.Int(s.EncryptedSize))
		}
		segs = append(segs, v)
	}
	root := obj([]string{"alg", "sig"}, []j.Value{j.Str(in.Root.Algorithm), j.Str(in.Root.Signature)})
	integrity := obj([]string{"rootSignature", "segmentHashAlg", "segmentSizeDefault", "encryptedSegmentSizeDefault", "segments"}, []j.Value{root, j.Str(in.SegmentHashAlgorithm), j.Int(in.DefaultSize), j.Int(in.DefaultEncryptedSize), j.Arr(segs)})
	enc := obj([]string{"type", "policy", "keyAccess", "method", "integrityInformation"}, []j.Value{j.Str(x.Type), j.Str(x.Policy), j.Arr(kaos), method, integrity})
	v := obj([]string{"schemaVersion", "payload", "encryptionInformation"}, []j.Value{j.Str(m.SchemaVersion), payload, enc})
	if len(m.Assertions) > 0 {
		v.Names = append(v.Names, "assertions")
		v.Children = append(v.Children, j.Arr(m.Assertions))
	}
	return j.Marshal(v, j.DefaultLimits())
}
func (p Policy) Marshal() ([]byte, error) {
	if len(p.Attributes) > 10000 || len(p.Dissem) > 10000 {
		return nil, errors.New("policy: serialization array limit")
	}
	if e := policyKnown(p.Raw); e != nil {
		return nil, e
	}
	attrs := []j.Value{}
	for _, a := range p.Attributes {
		v := obj([]string{"attribute"}, []j.Value{j.Str(a.Attribute)})
		v = appendString(v, "displayName", a.DisplayName)
		v = appendString(v, "pubKey", a.PubKey)
		v = appendString(v, "kasURL", a.KASURL)
		if a.IsDefault {
			v.Names = append(v.Names, "isDefault")
			v.Children = append(v.Children, j.Bool(true))
		}
		attrs = append(attrs, v)
	}
	dissem := []j.Value{}
	for _, s := range p.Dissem {
		dissem = append(dissem, j.Str(s))
	}
	body := obj([]string{"dataAttributes", "dissem"}, []j.Value{j.Arr(attrs), j.Arr(dissem)})
	v := obj([]string{"uuid", "body"}, []j.Value{j.Str(p.UUID), body})
	return j.Marshal(v, j.DefaultLimits())
}

// The pinned Go writer emits null arrays for a policy with zero attributes.
func (d *decoder) policyArray(v j.Value, n string, max int) []j.Value {
	x, ok := d.field(v, n, true)
	if !ok || x.Kind == j.Null {
		return nil
	}
	return d.array(v, n, true, max)
}
