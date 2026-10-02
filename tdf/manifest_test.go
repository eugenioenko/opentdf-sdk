package tdf

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	j "opentdf-local/sdk/tdf/json"
	"testing"
)

func b64(n int) string { return base64.StdEncoding.EncodeToString(make([]byte, n)) }
func exampleManifest(t *testing.T) Manifest {
	t.Helper()
	policy := `{ "uuid": "policy-id", "body": { "dataAttributes": [{"attribute":"https://example.test/a/value/b"}], "dissem": [] } }`
	binding := base64.StdEncoding.EncodeToString(bytes.Repeat([]byte{'0'}, 64))
	return Manifest{SchemaVersion: SchemaVersion, Payload: Payload{"reference", PayloadEntry, "zip", "application/octet-stream", true}, Encryption: EncryptionInformation{Type: "split", Policy: base64.StdEncoding.EncodeToString([]byte(policy)), KeyAccess: []KeyAccess{{Type: "wrapped", URL: "https://kas.test/kas", Protocol: "kas", WrappedKey: b64(256), Binding: PolicyBinding{Algorithm: "HS256", Hash: binding}, KID: "r1", SID: "s1", SchemaVersion: "1.0"}}, Method: Method{"AES-256-GCM", "", true}, Integrity: Integrity{Root: RootSignature{"HS256", b64(32)}, SegmentHashAlgorithm: "GMAC", DefaultSize: 1048576, DefaultEncryptedSize: 1048604, Segments: []Segment{{Hash: b64(16), Size: 3, EncryptedSize: 31, HasSize: true, HasEncryptedSize: true}}}}}
}
func TestManifestIndependentVectors(t *testing.T) {
	m := exampleManifest(t)
	if e := m.ValidateModern(31); e != nil {
		t.Fatal(e)
	}
	b, e := m.Marshal()
	if e != nil {
		t.Fatal(e)
	}
	var wire map[string]any
	if e = json.Unmarshal(b, &wire); e != nil {
		t.Fatal(e)
	}
	if wire["schemaVersion"] != "4.3.0" {
		t.Fatal(wire)
	}
	en := wire["encryptionInformation"].(map[string]any)
	if en["policy"] != m.Encryption.Policy {
		t.Fatal("policy changed")
	}
	kao := en["keyAccess"].([]any)[0].(map[string]any)
	if kao["sid"] != "s1" || kao["kid"] != "r1" {
		t.Fatal(kao)
	}
	parsed, e := ParseManifest(b)
	if e != nil || parsed.ValidateModern(31) != nil {
		t.Fatal(e)
	}
	if parsed.Encryption.Policy != m.Encryption.Policy || !parsed.Encryption.Integrity.Segments[0].HasSize {
		t.Fatal(parsed)
	}
	again, e := parsed.Marshal()
	if e != nil || !bytes.Equal(b, again) {
		t.Fatal(e)
	}
	// Let the independent stdlib writer omit size members separately.
	in := en["integrityInformation"].(map[string]any)
	segments := in["segments"].([]any)
	seg := segments[0].(map[string]any)
	delete(seg, "segmentSize")
	seg["encryptedSegmentSize"] = 1048604
	b, _ = json.Marshal(wire)
	parsed, e = ParseManifest(b)
	if e != nil || parsed.Encryption.Integrity.Segments[0].HasSize {
		t.Fatal(e)
	}
	if e = parsed.ValidateModern(1048604); e != nil {
		t.Fatal(e)
	}
	seg["segmentSize"] = 1048576
	delete(seg, "encryptedSegmentSize")
	b, _ = json.Marshal(wire)
	parsed, e = ParseManifest(b)
	if e != nil || parsed.ValidateModern(1048604) != nil {
		t.Fatal(e)
	}
}
func TestEmptyAndExplicitZeroSizes(t *testing.T) {
	m := exampleManifest(t)
	m.Encryption.Integrity.Segments = nil
	if e := m.ValidateModern(0); e != nil {
		t.Fatal("web empty", e)
	}
	m.Encryption.Integrity.Segments = []Segment{{Hash: b64(16), Size: 0, EncryptedSize: 28, HasSize: true, HasEncryptedSize: true}}
	if e := m.ValidateModern(28); e != nil {
		t.Fatal("Go empty", e)
	}
	b, _ := m.Marshal()
	parsed, e := ParseManifest(b)
	if e != nil || !parsed.Encryption.Integrity.Segments[0].HasSize || parsed.ValidateModern(28) != nil {
		t.Fatal(e)
	}
	m.Encryption.Integrity.Segments[0].HasSize = false
	if e := m.ValidateModern(28); e == nil {
		t.Fatal("omitted default contradicts ciphertext")
	}
}
func TestUnsupportedProfiles(t *testing.T) {
	cases := map[string]func(*Manifest){"version": func(m *Manifest) { m.SchemaVersion = "4.2.2" }, "assertions": func(m *Manifest) { m.Assertions = []j.Value{j.Obj([]string{"type"}, []j.Value{j.Str("handling")})} }, "multi": func(m *Manifest) { m.Encryption.KeyAccess = append(m.Encryption.KeyAccess, m.Encryption.KeyAccess[0]) }, "hybrid": func(m *Manifest) { m.Encryption.KeyAccess[0].Type = "hybrid-wrapped" }, "remote": func(m *Manifest) { m.Encryption.KeyAccess[0].Type = "remote" }, "bindinglegacy": func(m *Manifest) { m.Encryption.KeyAccess[0].Binding.LegacyString = true }, "rootgmac": func(m *Manifest) { m.Encryption.Integrity.Root.Algorithm = "GMAC" }, "unknownhash": func(m *Manifest) { m.Encryption.Integrity.SegmentHashAlgorithm = "unknown" }, "negative": func(m *Manifest) { m.Encryption.Integrity.Segments[0].Size = -1 }, "tooBig": func(m *Manifest) { m.Encryption.Integrity.DefaultSize = MaxSegmentBytes + 1 }, "unknownfield": func(m *Manifest) { m.Raw = j.Obj([]string{"obligations"}, []j.Value{j.Arr(nil)}) }, "bindingbytes": func(m *Manifest) { m.Encryption.KeyAccess[0].Binding.Hash = b64(32) }, "nonhex": func(m *Manifest) {
		m.Encryption.KeyAccess[0].Binding.Hash = base64.StdEncoding.EncodeToString(bytes.Repeat([]byte{'X'}, 64))
	}, "badroot": func(m *Manifest) { m.Encryption.Integrity.Root.Signature = "!!!" }, "badhash": func(m *Manifest) { m.Encryption.Integrity.Segments[0].Hash = b64(32) }}
	for name, change := range cases {
		t.Run(name, func(t *testing.T) {
			m := exampleManifest(t)
			change(&m)
			if e := m.ValidateModern(31); e == nil {
				t.Fatal("unsupported accepted")
			}
		})
	}
	m := exampleManifest(t)
	m.Encryption.Integrity.SegmentHashAlgorithm = ""
	m.Encryption.Integrity.Segments[0].Hash = b64(32)
	if e := m.ValidateModern(31); e != nil {
		t.Fatal("missing segment alg fallback", e)
	}
	m.Encryption.Integrity.Root.Algorithm = ""
	if e := m.ValidateModern(31); e == nil {
		t.Fatal("missing root alg supported")
	}
	for _, n := range []int{0, 30, 32, -1, 65 * 1024 * 1024} {
		if e := exampleManifest(t).ValidateModern(n); e == nil {
			t.Fatal("size", n)
		}
	}
}
func TestUnknownFieldsRetainedAndRejected(t *testing.T) {
	b, _ := exampleManifest(t).Marshal()
	var v map[string]any
	json.Unmarshal(b, &v)
	v["encryptionInformation"].(map[string]any)["obligations"] = map[string]any{"required": true}
	b, _ = json.Marshal(v)
	m, e := ParseManifest(b)
	if e != nil {
		t.Fatal(e)
	}
	raw, _ := m.Raw.Get("encryptionInformation")
	if _, ok := raw.Get("obligations"); !ok {
		t.Fatal("lost required feature")
	}
	if m.ValidateModern(31) == nil {
		t.Fatal("unknown feature accepted")
	}
	if _, e = m.Marshal(); e == nil {
		t.Fatal("serialization discarded feature")
	}
	delete(v, "schemaVersion")
	b, _ = json.Marshal(v)
	m, e = ParseManifest(b)
	if e != nil || m.ValidateModern(31) == nil {
		t.Fatal(e)
	}
}
func TestPolicyAndMetadata(t *testing.T) {
	p, e := DecodePolicy(exampleManifest(t).Encryption.Policy)
	if e != nil || p.ValidateModern() != nil {
		t.Fatal(e)
	}
	p.Dissem = []string{"a@example.test"}
	p.Attributes[0].DisplayName = "é\x00😃"
	b, e := p.Marshal()
	var independent map[string]any
	if e != nil || json.Unmarshal(b, &independent) != nil {
		t.Fatal(e)
	}
	round, e := ParsePolicy(b)
	if e != nil || round.Attributes[0].DisplayName != p.Attributes[0].DisplayName {
		t.Fatal(e)
	}
	policyWithFeature := `{"uuid":"p","body":{"dataAttributes":[],"dissem":[],"obligations":[]}}`
	p, e = ParsePolicy([]byte(policyWithFeature))
	if e != nil || p.ValidateModern() == nil {
		t.Fatal(e)
	}
	if _, e = p.Marshal(); e == nil {
		t.Fatal("policy feature discarded")
	}
	metadata := map[string]string{"ciphertext": b64(28), "iv": b64(12)}
	b, _ = json.Marshal(metadata)
	encoded := base64.StdEncoding.EncodeToString(b)
	md, e := ParseEncryptedMetadata(encoded)
	if e != nil || md.IV != b64(12) {
		t.Fatal(md, e)
	}
	m := exampleManifest(t)
	m.Encryption.KeyAccess[0].EncryptedMetadata = encoded
	m.Encryption.Method.IV = b64(12)
	if e = m.ValidateModern(31); e != nil {
		t.Fatal(e)
	}
	metadata["iv"] = base64.StdEncoding.EncodeToString(bytes.Repeat([]byte{1}, 12))
	b, _ = json.Marshal(metadata)
	if _, e = ParseEncryptedMetadata(base64.StdEncoding.EncodeToString(b)); e == nil {
		t.Fatal("IV mismatch")
	}
	for _, s := range []string{"!", base64.StdEncoding.EncodeToString([]byte(`{"ciphertext":"","iv":""}`)), base64.StdEncoding.EncodeToString([]byte(`{"ciphertext":0,"iv":""}`))} {
		if _, e = ParseEncryptedMetadata(s); e == nil {
			t.Fatal(s)
		}
	}
}
func TestMalformedManifestTypes(t *testing.T) {
	b, _ := exampleManifest(t).Marshal()
	for _, replace := range []string{`"segmentSize":-1`, `"segmentSize":1.5`, `"segmentSize":1e2`, `"segmentSize":"3"`, `"segmentSize":999999999999999999999`, `"segmentSize":3,"segmentSize":3`} {
		bad := bytes.Replace(b, []byte(`"segmentSize":3`), []byte(replace), 1)
		if _, e := ParseManifest(bad); e == nil {
			t.Fatal(replace)
		}
	}
	for _, s := range []string{`{}`, `[]`, `{"payload":null}`, `{"schemaVersion":null}`} {
		if _, e := ParseManifest([]byte(s)); e == nil {
			t.Fatal(s)
		}
	}
}

func TestGoNullPolicyArrays(t *testing.T) {
	b, e := json.Marshal(map[string]any{"uuid": "empty-policy", "body": map[string]any{"dataAttributes": nil, "dissem": nil}})
	if e != nil {
		t.Fatal(e)
	}
	p, e := ParsePolicy(b)
	if e != nil || p.ValidateModern() != nil || len(p.Attributes) != 0 || len(p.Dissem) != 0 {
		t.Fatal(p, e)
	}
	original := base64.StdEncoding.EncodeToString(b)
	m := exampleManifest(t)
	m.Encryption.Policy = original
	if e = m.ValidateModern(31); e != nil {
		t.Fatal(e)
	}
	out, e := m.Marshal()
	if e != nil {
		t.Fatal(e)
	}
	again, e := ParseManifest(out)
	if e != nil || again.Encryption.Policy != original {
		t.Fatal("policy base64 modified", e)
	}
}
