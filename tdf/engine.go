package tdf

import (
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/lib/errors"
	j "opentdf-local/sdk/tdf/json"
)

const DefaultSegmentBytes = 2 * 1024 * 1024
const MinSegmentBytes = 16 * 1024
const MaxMetadataBytes = 512 * 1024

// EncryptConfig supplies trusted wrapping information explicitly. No discovery,
// credentials or KAS requests are performed. Algorithm is rsa:2048 (default) or
// ec:secp256r1. Key handles remain owned by the caller.
// PolicyBase64, when nonempty, is validated and preserved byte-for-byte. Otherwise
// a policy with a secure UUIDv4 and the supplied attributes/dissemination is made.
// HasSegmentSize distinguishes an explicit zero (clamped to 16 KiB) from omission.
// Nonzero SegmentSize also selects an explicit size. Explicit sizes clamp like Go.
type EncryptConfig struct {
	KASPublicKey         *crypto.Key
	KASURL               string
	KID                  string
	Algorithm            string
	PolicyBase64         string
	Attributes           []string
	Dissem               []string
	SegmentSize          int
	HasSegmentSize       bool
	SegmentHashAlgorithm string
	MimeType             string
	Metadata             []byte
	IncludeMetadata      bool
}

// Decrypted is populated only after every required check succeeds. This engine
// authenticates a supplied recovered key; it does not authorize access to policy.
type Decrypted struct {
	Payload  []byte
	Metadata []byte
	Manifest Manifest
}

func hexLower(b []byte) string {
	const digits = "0123456789abcdef"
	out := make([]byte, len(b)*2)
	for i, c := range b {
		out[i*2] = digits[int(c)>>4]
		out[i*2+1] = digits[int(c)&15]
	}
	return string(out)
}
func hexDigest(s []byte) ([]byte, error) {
	if len(s) != 64 {
		return nil, errors.New("integrity: invalid policy binding length")
	}
	out := make([]byte, 32)
	for i := 0; i < 64; i++ {
		c := s[i]
		n := 0
		if c >= '0' && c <= '9' {
			n = int(c - '0')
		} else if c >= 'a' && c <= 'f' {
			n = int(c-'a') + 10
		} else {
			return nil, errors.New("integrity: invalid policy binding hex")
		}
		if i%2 == 0 {
			out[i/2] = byte(n * 16)
		} else {
			out[i/2] += byte(n)
		}
	}
	return out, nil
}

// NewPolicy creates a policy with a cryptographically random UUIDv4. It copies
// caller slices and validates the supported policy profile before returning.
func NewPolicy(attributes []string, dissem []string) (Policy, error) {
	if len(attributes) > 10000 || len(dissem) > 10000 {
		return Policy{}, errors.New("policy: array limit")
	}
	random, e := crypto.Random(16)
	if e != nil {
		return Policy{}, e
	}
	random[6] = (random[6] & 15) | 64
	random[8] = (random[8] & 63) | 128
	h := hexLower(random)
	p := Policy{UUID: h[:8] + "-" + h[8:12] + "-" + h[12:16] + "-" + h[16:20] + "-" + h[20:]}
	for _, a := range attributes {
		p.Attributes = append(p.Attributes, Attribute{Attribute: a})
	}
	for _, d := range dissem {
		p.Dissem = append(p.Dissem, d)
	}
	if e := p.ValidateModern(); e != nil {
		return Policy{}, e
	}
	return p, nil
}

func binding(key []byte, policy string) (PolicyBinding, error) {
	digest, e := crypto.HMACSHA256(key, []byte(policy))
	if e != nil {
		return PolicyBinding{}, e
	}
	hash, e := encoding.Base64Encode([]byte(hexLower(digest)))
	if e != nil {
		return PolicyBinding{}, e
	}
	return PolicyBinding{Algorithm: "HS256", Hash: hash}, nil
}
func encryptFrame(key, plain []byte) ([]byte, error) {
	nonce, e := crypto.Random(12)
	if e != nil {
		return nil, e
	}
	cipher, e := crypto.AES256GCMEncrypt(key, nonce, plain, nil)
	if e != nil {
		return nil, e
	}
	return append(nonce, cipher...), nil
}
func decryptFrame(key, frame []byte) ([]byte, error) {
	if len(frame) < FrameOverhead {
		return nil, errors.New("encryption: truncated AES-GCM frame")
	}
	return crypto.AES256GCMDecrypt(key, frame[:12], frame[12:], nil)
}
func ecWrapKey(private, public *crypto.Key) ([]byte, error) {
	secret, e := crypto.ECDH(private, public)
	if e != nil {
		return nil, e
	}
	salt, e := crypto.SHA256([]byte("TDF"))
	if e != nil {
		return nil, e
	}
	return crypto.HKDFSHA256(secret, salt, nil, 32)
}
func wrap(key []byte, c EncryptConfig) (KeyAccess, error) {
	k := KeyAccess{URL: c.KASURL, Protocol: "kas", KID: c.KID, SchemaVersion: "1.0"}
	var wrapped []byte
	var e error
	if c.Algorithm == "" || c.Algorithm == "rsa:2048" {
		k.Type = "wrapped"
		wrapped, e = crypto.RSAOAEPEncrypt(c.KASPublicKey, key)
	} else if c.Algorithm == "ec:secp256r1" {
		k.Type = "ec-wrapped"
		ephemeral, err := crypto.GenerateP256()
		if err != nil {
			return KeyAccess{}, err
		}
		// Explicit closure on every exit keeps this compatible with subset Go.
		wrapKey, err := ecWrapKey(ephemeral, c.KASPublicKey)
		if err != nil {
			ephemeral.Close()
			return KeyAccess{}, err
		}
		k.EphemeralPublicKey, err = ephemeral.PublicPEM()
		ephemeral.Close()
		if err != nil {
			return KeyAccess{}, err
		}
		wrapped, e = encryptFrame(wrapKey, key)
	} else {
		return KeyAccess{}, errors.New("encryption: unsupported wrapping algorithm")
	}
	if e != nil {
		return KeyAccess{}, e
	}
	k.WrappedKey, e = encoding.Base64Encode(wrapped)
	if e != nil {
		return KeyAccess{}, e
	}
	return k, nil
}
func encryptMetadata(key, data []byte) (string, error) {
	if len(data) > MaxMetadataBytes {
		return "", errors.New("metadata: plaintext limit")
	}
	frame, e := encryptFrame(key, data)
	if e != nil {
		return "", e
	}
	cipher, e := encoding.Base64Encode(frame)
	if e != nil {
		return "", e
	}
	iv, e := encoding.Base64Encode(frame[:12])
	if e != nil {
		return "", e
	}
	b, e := j.Marshal(obj([]string{"ciphertext", "iv"}, []j.Value{j.Str(cipher), j.Str(iv)}), j.DefaultLimits())
	if e != nil {
		return "", e
	}
	return encoding.Base64Encode(b)
}

// Encrypt writes bounded modern TDF3 with one wrapped KAO and no assertions.
// Payload keys/nonces are always freshly generated; the payload key is not exposed.
func Encrypt(data []byte, c EncryptConfig) ([]byte, error) {
	if len(data) > 64*1024*1024 || len(c.Metadata) > MaxMetadataBytes {
		return nil, errors.New("encryption: input limit")
	}
	if c.KASURL == "" || len(c.KASURL) > 8192 || len(c.KID) > 1024 {
		return nil, errors.New("encryption: invalid KAS configuration")
	}
	if c.PolicyBase64 != "" && (len(c.Attributes) > 0 || len(c.Dissem) > 0) {
		return nil, errors.New("encryption: conflicting policy configuration")
	}
	alg := c.SegmentHashAlgorithm
	if alg == "" {
		alg = "GMAC"
	}
	if lower(alg) != "gmac" && lower(alg) != "hs256" {
		return nil, errors.New("encryption: unsupported segment hash algorithm")
	}
	size := DefaultSegmentBytes
	if c.HasSegmentSize || c.SegmentSize != 0 {
		size = c.SegmentSize
		if size < MinSegmentBytes {
			size = MinSegmentBytes
		}
		if size > MaxSegmentBytes {
			size = MaxSegmentBytes
		}
	}
	count := len(data) / size
	if len(data)%size != 0 {
		count++
	}
	if count == 0 {
		count = 1
	}
	if count > MaxSegments || len(data) > 64*1024*1024-count*FrameOverhead {
		return nil, errors.New("encryption: encrypted payload limit")
	}
	policy := c.PolicyBase64
	if policy == "" {
		p, e := NewPolicy(c.Attributes, c.Dissem)
		if e != nil {
			return nil, e
		}
		b, e := p.Marshal()
		if e != nil {
			return nil, e
		}
		policy, e = encoding.Base64Encode(b)
		if e != nil {
			return nil, e
		}
	} else {
		p, e := DecodePolicy(policy)
		if e != nil {
			return nil, e
		}
		if e := p.ValidateModern(); e != nil {
			return nil, e
		}
	}
	key, e := crypto.Random(32)
	if e != nil {
		return nil, e
	}
	k, e := wrap(key, c)
	if e != nil {
		return nil, e
	}
	k.Binding, e = binding(key, policy)
	if e != nil {
		return nil, e
	}
	if c.IncludeMetadata || len(c.Metadata) > 0 {
		k.EncryptedMetadata, e = encryptMetadata(key, c.Metadata)
		if e != nil {
			return nil, e
		}
	}
	mime := c.MimeType
	if mime == "" {
		mime = "application/octet-stream"
	}
	m := Manifest{SchemaVersion: SchemaVersion, Payload: Payload{Type: "reference", URL: PayloadEntry, Protocol: "zip", MimeType: mime, IsEncrypted: true}}
	in := Integrity{SegmentHashAlgorithm: alg, DefaultSize: size, DefaultEncryptedSize: size + FrameOverhead}
	var payload []byte
	var aggregate []byte
	pos := 0
	for i := 0; i < count; i++ {
		end := pos + size
		if end > len(data) {
			end = len(data)
		}
		frame, e := encryptFrame(key, data[pos:end])
		if e != nil {
			return nil, e
		}
		hash := frame[len(frame)-16:]
		if lower(alg) == "hs256" {
			hash, e = crypto.HMACSHA256(key, frame)
			if e != nil {
				return nil, e
			}
		}
		encoded, e := encoding.Base64Encode(hash)
		if e != nil {
			return nil, e
		}
		in.Segments = append(in.Segments, Segment{Hash: encoded, Size: end - pos, EncryptedSize: len(frame), HasSize: true, HasEncryptedSize: true})
		aggregate = append(aggregate, hash...)
		payload = append(payload, frame...)
		pos = end
	}
	root, e := crypto.HMACSHA256(key, aggregate)
	if e != nil {
		return nil, e
	}
	sig, e := encoding.Base64Encode(root)
	if e != nil {
		return nil, e
	}
	in.Root = RootSignature{Algorithm: "HS256", Signature: sig}
	m.Encryption = EncryptionInformation{Type: "split", Policy: policy, KeyAccess: []KeyAccess{k}, Method: Method{Algorithm: "AES-256-GCM", IsStreamable: true}, Integrity: in}
	if e := m.ValidateModern(len(payload)); e != nil {
		return nil, e
	}
	b, e := m.Marshal()
	if e != nil {
		return nil, e
	}
	return WriteArchive(payload, b, DefaultArchiveLimits())
}

func verifyMAC(key, data, mac []byte, label string) error {
	ok, e := crypto.HMACSHA256Verify(key, data, mac)
	if e != nil {
		return e
	}
	if !ok {
		return errors.New("integrity: " + label + " verification failed")
	}
	return nil
}

// PreparedDecryption owns a CRC-checked archive and its validated manifest.
// Its zero value cannot decrypt. Mutable state is never exposed to callers.
type PreparedDecryption struct {
	archive  Archive
	manifest Manifest
	ready    bool
}

// PrepareDecryption checks ZIP CRCs, parses the manifest and validates its
// supported profile once. Stage preserves the client's pre-rewrap errors.
func PrepareDecryption(data []byte) (PreparedDecryption, string, error) {
	archive, e := ReadArchive(data, DefaultArchiveLimits())
	if e != nil {
		return PreparedDecryption{}, "archive", e
	}
	m, e := ParseManifest(archive.Manifest)
	if e != nil {
		return PreparedDecryption{}, "manifest", e
	}
	if e := m.ValidateModern(len(archive.Payload)); e != nil {
		return PreparedDecryption{}, "unsupported_manifest", e
	}
	return PreparedDecryption{archive: archive, manifest: m, ready: true}, "", nil
}

func clonePreparedJSON(v j.Value) j.Value {
	r := v
	r.Names = append([]string(nil), v.Names...)
	r.Children = nil
	for _, child := range v.Children {
		r.Children = append(r.Children, clonePreparedJSON(child))
	}
	return r
}

func clonePreparedManifest(m Manifest) Manifest {
	r := m
	r.Encryption.KeyAccess = append([]KeyAccess(nil), m.Encryption.KeyAccess...)
	r.Encryption.Integrity.Segments = append([]Segment(nil), m.Encryption.Integrity.Segments...)
	r.Raw = clonePreparedJSON(m.Raw)
	r.Assertions = nil
	for _, assertion := range m.Assertions {
		r.Assertions = append(r.Assertions, clonePreparedJSON(assertion))
	}
	return r
}

// Manifest returns an owned snapshot for KAS routing and inspection.
func (p PreparedDecryption) Manifest() Manifest { return clonePreparedManifest(p.manifest) }

// DecryptWithPayloadKey authenticates and decrypts a complete bounded archive
// given an independently recovered 32-byte payload key. It performs no rewrap.
// Failure always returns a zero Decrypted, including failures in the last segment
// or metadata; callers never receive partial plaintext.
func DecryptWithPayloadKey(data, key []byte) (Decrypted, error) {
	if len(key) != 32 {
		return Decrypted{}, errors.New("decryption: payload key must be 32 bytes")
	}
	p, _, e := PrepareDecryption(data)
	if e != nil {
		return Decrypted{}, e
	}
	return p.Decrypt(key)
}

// Decrypt authenticates policy, root, every segment and metadata before
// publishing any plaintext. It retains no caller-visible mutable aliases.
func (p PreparedDecryption) Decrypt(key []byte) (Decrypted, error) {
	if !p.ready || len(key) != 32 {
		return Decrypted{}, errors.New("decryption: invalid prepared archive or payload key")
	}
	archive, m := p.archive, p.manifest
	k := m.Encryption.KeyAccess[0]
	bh, e := encoding.Base64Decode(k.Binding.Hash)
	if e != nil {
		return Decrypted{}, e
	}
	mac, e := hexDigest(bh)
	if e != nil {
		return Decrypted{}, e
	}
	if e := verifyMAC(key, []byte(m.Encryption.Policy), mac, "policy binding"); e != nil {
		return Decrypted{}, e
	}
	in := m.Encryption.Integrity
	var aggregate []byte
	for _, s := range in.Segments {
		hash, e := encoding.Base64Decode(s.Hash)
		if e != nil {
			return Decrypted{}, e
		}
		aggregate = append(aggregate, hash...)
	}
	root, e := encoding.Base64Decode(in.Root.Signature)
	if e != nil {
		return Decrypted{}, e
	}
	if e := verifyMAC(key, aggregate, root, "root signature"); e != nil {
		return Decrypted{}, e
	}
	var plain []byte
	pos := 0
	for _, s := range in.Segments {
		size, n, e := in.ResolveSegment(s)
		if e != nil {
			return Decrypted{}, e
		}
		frame := archive.Payload[pos : pos+n]
		hash, e := encoding.Base64Decode(s.Hash)
		if e != nil {
			return Decrypted{}, e
		}
		if m.ResolvedSegmentHashAlgorithm() == "hs256" {
			if e := verifyMAC(key, frame, hash, "segment"); e != nil {
				return Decrypted{}, e
			}
		} else {
			// These are public authenticated manifest/tag bytes, not secret MACs.
			for i := 0; i < 16; i++ {
				if hash[i] != frame[len(frame)-16+i] {
					return Decrypted{}, errors.New("integrity: segment tag mismatch")
				}
			}
		}
		b, e := decryptFrame(key, frame)
		if e != nil {
			return Decrypted{}, e
		}
		if len(b) != size {
			return Decrypted{}, errors.New("decryption: segment plaintext size mismatch")
		}
		plain = append(plain, b...)
		pos += n
	}
	var metadata []byte
	if k.EncryptedMetadata != "" {
		md, e := ParseEncryptedMetadata(k.EncryptedMetadata)
		if e != nil {
			return Decrypted{}, e
		}
		frame, e := encoding.Base64Decode(md.Ciphertext)
		if e != nil {
			return Decrypted{}, e
		}
		if len(frame) > MaxMetadataBytes+FrameOverhead {
			return Decrypted{}, errors.New("metadata: plaintext limit")
		}
		metadata, e = decryptFrame(key, frame)
		if e != nil {
			return Decrypted{}, e
		}
	}
	return Decrypted{Payload: plain, Metadata: metadata, Manifest: clonePreparedManifest(m)}, nil
}
