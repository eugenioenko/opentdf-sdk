package tdf

import (
	"archive/zip"
	"bytes"
	"crypto/aes"
	"crypto/cipher"
	"crypto/ecdsa"
	"crypto/hkdf"
	"crypto/hmac"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/hex"
	"encoding/pem"
	"io"
	"strings"
	"testing"

	capcrypto "github.com/eugenioenko/goalchemy/lib/crypto"
)

func engineKey(t *testing.T, ec bool) *capcrypto.Key {
	t.Helper()
	var k *capcrypto.Key
	var e error
	if ec {
		k, e = capcrypto.GenerateP256()
	} else {
		k, e = capcrypto.GenerateRSA2048()
	}
	if e != nil {
		t.Fatal(e)
	}
	t.Cleanup(k.Close)
	return k
}
func engineParts(t *testing.T, data []byte) (Archive, Manifest) {
	t.Helper()
	a, e := ReadArchive(data, DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	m, e := ParseManifest(a.Manifest)
	if e != nil {
		t.Fatal(e)
	}
	return a, m
}
func independentKey(t *testing.T, k *capcrypto.Key, m Manifest) []byte {
	t.Helper()
	p, e := k.PrivatePEM()
	if e != nil {
		t.Fatal(e)
	}
	b, _ := pem.Decode([]byte(p))
	v, e := x509.ParsePKCS8PrivateKey(b.Bytes)
	if e != nil {
		t.Fatal(e)
	}
	kao := m.Encryption.KeyAccess[0]
	wrapped, e := base64.StdEncoding.DecodeString(kao.WrappedKey)
	if e != nil {
		t.Fatal(e)
	}
	if kao.Type == "wrapped" {
		out, e := rsa.DecryptOAEP(sha1.New(), rand.Reader, v.(*rsa.PrivateKey), wrapped, nil)
		if e != nil {
			t.Fatal(e)
		}
		return out
	}
	ep, _ := pem.Decode([]byte(kao.EphemeralPublicKey))
	pub, e := x509.ParsePKIXPublicKey(ep.Bytes)
	if e != nil {
		t.Fatal(e)
	}
	priv, e := v.(*ecdsa.PrivateKey).ECDH()
	if e != nil {
		t.Fatal(e)
	}
	peer, e := pub.(*ecdsa.PublicKey).ECDH()
	if e != nil {
		t.Fatal(e)
	}
	secret, e := priv.ECDH(peer)
	if e != nil {
		t.Fatal(e)
	}
	salt := sha256.Sum256([]byte("TDF"))
	key, e := hkdf.Key(sha256.New, secret, salt[:], "", 32)
	if e != nil {
		t.Fatal(e)
	}
	return independentOpen(t, key, wrapped)
}
func independentOpen(t *testing.T, key, frame []byte) []byte {
	t.Helper()
	block, e := aes.NewCipher(key)
	if e != nil {
		t.Fatal(e)
	}
	g, e := cipher.NewGCM(block)
	if e != nil {
		t.Fatal(e)
	}
	out, e := g.Open(nil, frame[:12], frame[12:], nil)
	if e != nil {
		t.Fatal(e)
	}
	return out
}
func independentMAC(key, data []byte) []byte {
	h := hmac.New(sha256.New, key)
	h.Write(data)
	return h.Sum(nil)
}
func archiveChange(t testing.TB, a Archive, m Manifest) []byte {
	t.Helper()
	b, e := m.Marshal()
	if e != nil {
		t.Fatal(e)
	}
	out, e := WriteArchive(a.Payload, b, DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	return out
}
func TestEngineIndependentWrite(t *testing.T) {
	for _, ec := range []bool{false, true} {
		for _, alg := range []string{"GMAC", "HS256"} {
			for _, n := range []int{0, MinSegmentBytes, MinSegmentBytes + 1, MinSegmentBytes * 3} {
				t.Run(string(rune('a'+n%26))+alg+map[bool]string{false: "RSA", true: "EC"}[ec], func(t *testing.T) {
					k := engineKey(t, ec)
					c := EncryptConfig{KASPublicKey: k, KASURL: "https://kas.test", KID: "key", Attributes: []string{"https://example.test/attr/a/value/b"}, SegmentSize: MinSegmentBytes, SegmentHashAlgorithm: alg, Metadata: []byte{0, 255, 128, 7}}
					if ec {
						c.Algorithm = "ec:secp256r1"
					}
					plain := make([]byte, n)
					for i := range plain {
						plain[i] = byte(i)
					}
					before := bytes.Clone(plain)
					data, e := Encrypt(plain, c)
					if e != nil {
						t.Fatal(e)
					}
					if !bytes.Equal(plain, before) {
						t.Fatal("input mutation")
					}
					// Independently read stored archive with standard library.
					zr, e := zip.NewReader(bytes.NewReader(data), int64(len(data)))
					if e != nil {
						t.Fatal(e)
					}
					if len(zr.File) != 2 {
						t.Fatal("entries")
					}
					for _, f := range zr.File {
						r, e := f.Open()
						if e != nil {
							t.Fatal(e)
						}
						_, e = io.ReadAll(r)
						r.Close()
						if e != nil {
							t.Fatal(e)
						}
					}
					a, m := engineParts(t, data)
					key := independentKey(t, k, m)
					policy, e := base64.StdEncoding.DecodeString(m.Encryption.Policy)
					if e != nil {
						t.Fatal(e)
					}
					p, e := ParsePolicy(policy)
					if e != nil {
						t.Fatal(e)
					}
					if len(p.UUID) != 36 || p.UUID[14] != '4' || !strings.Contains("89ab", p.UUID[19:20]) {
						t.Fatal("UUIDv4")
					}
					bind := base64.StdEncoding.EncodeToString([]byte(hex.EncodeToString(independentMAC(key, []byte(m.Encryption.Policy)))))
					if bind != m.Encryption.KeyAccess[0].Binding.Hash {
						t.Fatal("binding exact encoding")
					}
					var got, aggregate []byte
					pos := 0
					for _, s := range m.Encryption.Integrity.Segments {
						frame := a.Payload[pos : pos+s.EncryptedSize]
						got = append(got, independentOpen(t, key, frame)...)
						hash := frame[len(frame)-16:]
						if alg == "HS256" {
							hash = independentMAC(key, frame)
						}
						if s.Hash != base64.StdEncoding.EncodeToString(hash) {
							t.Fatal("segment hash")
						}
						aggregate = append(aggregate, hash...)
						pos += len(frame)
					}
					if m.Encryption.Integrity.Root.Signature != base64.StdEncoding.EncodeToString(independentMAC(key, aggregate)) {
						t.Fatal("root hash")
					}
					if !bytes.Equal(got, plain) {
						t.Fatal("independent plaintext")
					}
					result, e := DecryptWithPayloadKey(data, key)
					if e != nil || !bytes.Equal(result.Payload, plain) || !bytes.Equal(result.Metadata, c.Metadata) {
						t.Fatalf("engine: %v", e)
					}
					if n == 0 && (len(m.Encryption.Integrity.Segments) != 1 || len(a.Payload) != 28) {
						t.Fatal("Go empty frame")
					}
				})
			}
		}
	}
}

func TestEngineTamperFailsClosed(t *testing.T) {
	for _, alg := range []string{"GMAC", "HS256"} {
		k := engineKey(t, false)
		data, e := Encrypt(bytes.Repeat([]byte{7}, MinSegmentBytes+1), EncryptConfig{KASPublicKey: k, KASURL: "https://kas.test", SegmentSize: MinSegmentBytes, SegmentHashAlgorithm: alg, Metadata: []byte("metadata")})
		if e != nil {
			t.Fatal(e)
		}
		_, m := engineParts(t, data)
		key := independentKey(t, k, m)
		changes := []struct {
			name   string
			mutate func(*Archive, *Manifest)
		}{
			{"last ciphertext", func(a *Archive, m *Manifest) { a.Payload[len(a.Payload)-17] ^= 1 }},
			{"nonce", func(a *Archive, m *Manifest) { a.Payload[0] ^= 1 }},
			{"tag", func(a *Archive, m *Manifest) { a.Payload[len(a.Payload)-1] ^= 1 }},
			{"root", func(a *Archive, m *Manifest) {
				m.Encryption.Integrity.Root.Signature = base64.StdEncoding.EncodeToString(make([]byte, 32))
			}},
			{"binding", func(a *Archive, m *Manifest) {
				m.Encryption.KeyAccess[0].Binding.Hash = base64.StdEncoding.EncodeToString(bytes.Repeat([]byte{'0'}, 64))
			}},
			{"policy exact whitespace", func(a *Archive, m *Manifest) {
				p, _ := base64.StdEncoding.DecodeString(m.Encryption.Policy)
				m.Encryption.Policy = base64.StdEncoding.EncodeToString(append(p, ' '))
			}},
			{"size", func(a *Archive, m *Manifest) { m.Encryption.Integrity.Segments[0].Size-- }},
			{"algorithm", func(a *Archive, m *Manifest) { m.Encryption.Integrity.SegmentHashAlgorithm = "unknown" }},
			{"unsupported version", func(a *Archive, m *Manifest) { m.SchemaVersion = "4.2.2" }},
			{"truncated payload", func(a *Archive, m *Manifest) { a.Payload = a.Payload[:len(a.Payload)-1] }},
			{"extra payload", func(a *Archive, m *Manifest) { a.Payload = append(a.Payload, 1) }},
			{"metadata auth", func(a *Archive, m *Manifest) {
				md, _ := ParseEncryptedMetadata(m.Encryption.KeyAccess[0].EncryptedMetadata)
				b, _ := base64.StdEncoding.DecodeString(md.Ciphertext)
				b[len(b)-1] ^= 1
				md.Ciphertext = base64.StdEncoding.EncodeToString(b)
				j := []byte(`{"ciphertext":"` + md.Ciphertext + `","iv":"` + md.IV + `"}`)
				m.Encryption.KeyAccess[0].EncryptedMetadata = base64.StdEncoding.EncodeToString(j)
			}},
		}
		for _, c := range changes {
			t.Run(alg+"/"+c.name, func(t *testing.T) {
				a, m := engineParts(t, data)
				c.mutate(&a, &m)
				bad := archiveChange(t, a, m)
				before := bytes.Clone(bad)
				r, e := DecryptWithPayloadKey(bad, key)
				if e == nil || r.Payload != nil || r.Metadata != nil || r.Manifest.SchemaVersion != "" {
					t.Fatal("failed open")
				}
				if !bytes.Equal(before, bad) {
					t.Fatal("decrypt mutated input")
				}
			})
		}
		for _, wrong := range [][]byte{nil, make([]byte, 31), make([]byte, 32), make([]byte, 33)} {
			r, e := DecryptWithPayloadKey(data, wrong)
			if e == nil || r.Payload != nil || r.Metadata != nil {
				t.Fatal("wrong key accepted")
			}
		}
	}
}

func TestEngineConfigAndDefaults(t *testing.T) {
	k := engineKey(t, false)
	for _, test := range []struct {
		size int
		set  bool
		want int
	}{{0, false, DefaultSegmentBytes}, {0, true, MinSegmentBytes}, {-1, false, MinSegmentBytes}, {1, false, MinSegmentBytes}, {MaxSegmentBytes + 1, false, MaxSegmentBytes}} {
		data, e := Encrypt(nil, EncryptConfig{KASPublicKey: k, KASURL: "https://kas.test", SegmentSize: test.size, HasSegmentSize: test.set, IncludeMetadata: true})
		if e != nil {
			t.Fatal(e)
		}
		_, m := engineParts(t, data)
		if m.Encryption.Integrity.DefaultSize != test.want {
			t.Fatal("defaults")
		}
		key := independentKey(t, k, m)
		r, e := DecryptWithPayloadKey(data, key)
		if e != nil || len(r.Metadata) != 0 {
			t.Fatal("empty metadata")
		}
	}
	for _, c := range []EncryptConfig{{}, {KASPublicKey: k}, {KASURL: "url"}, {KASPublicKey: k, KASURL: "url", Algorithm: "bad"}, {KASPublicKey: k, KASURL: "url", SegmentHashAlgorithm: "bad"}, {KASPublicKey: k, KASURL: "url", Attributes: []string{""}}, {KASPublicKey: k, KASURL: "url", Metadata: make([]byte, MaxMetadataBytes+1)}, {KASPublicKey: k, KASURL: "url", PolicyBase64: "bad"}} {
		if out, e := Encrypt(nil, c); e == nil || out != nil {
			t.Fatal("invalid config accepted")
		}
	}
	ec := engineKey(t, true)
	if out, e := Encrypt(nil, EncryptConfig{KASPublicKey: ec, KASURL: "url"}); e == nil || out != nil {
		t.Fatal("EC as RSA accepted")
	}
	for _, handle := range []*capcrypto.Key{nil, k} {
		if out, e := Encrypt(nil, EncryptConfig{KASPublicKey: handle, KASURL: "url", Algorithm: "ec:secp256r1"}); e == nil || out != nil {
			t.Fatal("invalid EC handle accepted")
		}
	}
	closedEC := engineKey(t, true)
	closedEC.Close()
	if out, e := Encrypt(nil, EncryptConfig{KASPublicKey: closedEC, KASURL: "url", Algorithm: "ec:secp256r1"}); e == nil || out != nil {
		t.Fatal("closed EC handle accepted")
	}
	// Explicit policy text is never canonicalized: byte whitespace is part of binding.
	supplied := base64.StdEncoding.EncodeToString([]byte(" " + `{"uuid":"exact","body":{"dataAttributes":[],"dissem":[]}}` + " "))
	data, e := Encrypt(nil, EncryptConfig{KASPublicKey: k, KASURL: "url", PolicyBase64: supplied})
	if e != nil {
		t.Fatal(e)
	}
	_, m := engineParts(t, data)
	if m.Encryption.Policy != supplied {
		t.Fatal("policy changed")
	}
	if out, e := Encrypt(nil, EncryptConfig{KASPublicKey: k, KASURL: "url", PolicyBase64: supplied, Attributes: []string{"a"}}); e == nil || out != nil {
		t.Fatal("conflicting policy accepted")
	}

	closed := engineKey(t, false)
	closed.Close()
	if out, e := Encrypt(nil, EncryptConfig{KASPublicKey: closed, KASURL: "url"}); e == nil || out != nil {
		t.Fatal("closed key accepted")
	}
}

func independentEmptyFixture(t testing.TB) ([]byte, []byte, Manifest) {
	// NIST zero key / zero nonce / empty plaintext GCM authentication vector.
	key := make([]byte, 32)
	frame := make([]byte, 12)
	tag, _ := hex.DecodeString("530f8afbc74536b9a963b4f1c4cb738b")
	frame = append(frame, tag...)
	policy := base64.StdEncoding.EncodeToString([]byte(`{"uuid":"independent","body":{"dataAttributes":[],"dissem":[]}}`))
	b := base64.StdEncoding.EncodeToString([]byte(hex.EncodeToString(independentMAC(key, []byte(policy)))))
	m := Manifest{SchemaVersion: SchemaVersion, Payload: Payload{Type: "reference", URL: PayloadEntry, Protocol: "zip", IsEncrypted: true}, Encryption: EncryptionInformation{Type: "split", Policy: policy, Method: Method{Algorithm: "AES-256-GCM", IsStreamable: true}, KeyAccess: []KeyAccess{{Type: "wrapped", URL: "url", Protocol: "kas", WrappedKey: base64.StdEncoding.EncodeToString(make([]byte, 256)), Binding: PolicyBinding{Algorithm: "HS256", Hash: b}}}, Integrity: Integrity{Root: RootSignature{Algorithm: "HS256", Signature: base64.StdEncoding.EncodeToString(independentMAC(key, tag))}, SegmentHashAlgorithm: "GMAC", DefaultSize: DefaultSegmentBytes, DefaultEncryptedSize: DefaultSegmentBytes + 28, Segments: []Segment{{Hash: base64.StdEncoding.EncodeToString(tag), Size: 0, EncryptedSize: 28, HasSize: true, HasEncryptedSize: true}}}}}
	return archiveChange(t, Archive{Payload: frame}, m), key, m
}

func TestEngineIndependentReadVector(t *testing.T) {
	data, key, m := independentEmptyFixture(t)
	r, e := DecryptWithPayloadKey(data, key)
	if e != nil || len(r.Payload) != 0 {
		t.Fatalf("independent vector: %v", e)
	}
	// Web's empty convention: no segments, HMAC over the empty aggregate.
	m.Encryption.Integrity.Segments = nil
	m.Encryption.Integrity.Root.Signature = base64.StdEncoding.EncodeToString(independentMAC(key, nil))
	data = archiveChange(t, Archive{}, m)
	r, e = DecryptWithPayloadKey(data, key)
	if e != nil || len(r.Payload) != 0 {
		t.Fatalf("web empty: %v", e)
	}
	// Independent HS256 framing with both segment sizes omitted. The omitted
	// algorithm falls back to the HS256 root. All hashes use the original frame.
	block, _ := aes.NewCipher(key)
	g, _ := cipher.NewGCM(block)
	payload := []byte{0, 255, 128, 7}
	nonce := make([]byte, 12)
	frame := append(nonce, g.Seal(nil, nonce, payload, nil)...)
	digest := independentMAC(key, frame)
	in := &m.Encryption.Integrity
	in.DefaultSize = len(payload)
	in.DefaultEncryptedSize = len(frame)
	in.SegmentHashAlgorithm = ""
	in.Segments = []Segment{{Hash: base64.StdEncoding.EncodeToString(digest)}}
	in.Root.Signature = base64.StdEncoding.EncodeToString(independentMAC(key, digest))
	data = archiveChange(t, Archive{Payload: frame}, m)
	r, e = DecryptWithPayloadKey(data, key)
	if e != nil || !bytes.Equal(r.Payload, payload) {
		t.Fatalf("independent HS256 omitted defaults: %v", e)
	}

}

func FuzzEngineDecrypt(f *testing.F) {
	valid, key, _ := independentEmptyFixture(f)
	f.Add(valid, key)
	f.Add([]byte{}, make([]byte, 32))
	f.Add([]byte("PK\x03\x04"), []byte{1})
	f.Fuzz(func(t *testing.T, archive, key []byte) {
		if len(archive) > 1<<20 || len(key) > 64 {
			t.Skip()
		}
		r, e := DecryptWithPayloadKey(archive, key)
		if e != nil && (r.Payload != nil || r.Metadata != nil || r.Manifest.SchemaVersion != "") {
			t.Fatal("partial plaintext on error")
		}
	})
}
