package sdk

import (
	"bytes"
	"context"
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"errors"
	"strings"
	"testing"

	gbytes "github.com/eugenioenko/goalchemy/std/bytes"
)

const stanagValue = "{\"ocl\":{\"pol\":\"62c76c68-d73d-4628-8ccc-4c1e18118c22\",\"cls\":\"SECRET\",\"catl\":[{\"type\":\"P\",\"name\":\"Releasable To\",\"vals\":[\"usa\"]}],\"dcr\":\"2024-10-21T20:47:36Z\"},\"context\":{\"@base\":\"urn:nato:stanag:5636:A:1:elements:json\"}}"

func stanagAssertion(id string) AssertionConfig {
	return AssertionConfig{
		ID:             id,
		Type:           HandlingAssertion,
		Scope:          TrustedDataObjScope,
		AppliesToState: Encrypted,
		Statement:      Statement{Format: "json+stanag5636", Schema: "urn:nato:stanag:5636:A:1:elements:json", Value: stanagValue},
	}
}

func rsaPEMs(t *testing.T) (string, string) {
	t.Helper()
	key, err := rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		t.Fatal(err)
	}
	private, err := x509.MarshalPKCS8PrivateKey(key)
	if err != nil {
		t.Fatal(err)
	}
	public, err := x509.MarshalPKIXPublicKey(&key.PublicKey)
	if err != nil {
		t.Fatal(err)
	}
	return string(pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: private})), string(pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: public}))
}

// The reference's TestTDFWithAssertion hash vector.
func TestAssertionGetHash(t *testing.T) {
	c := stanagAssertion("424ff3a3-50ca-4f01-a2ae-ef851cd3cac0")
	a := Assertion{ID: c.ID, Type: c.Type, Scope: c.Scope, AppliesToState: c.AppliesToState, Statement: c.Statement}
	hash, err := a.GetHash()
	if err != nil {
		t.Fatal(err)
	}
	if string(hash) != "4a447a13c5a32730d20bdf7feecb9ffe16649bc731914b574d80035a3927f860" {
		t.Fatalf("hash %s", hash)
	}
	// The binding never takes part, and an empty appliesToState is omitted.
	a.Binding = Binding{Method: "jws", Signature: "x"}
	again, _ := a.GetHash()
	if string(again) != string(hash) {
		t.Fatal("binding changed the hash")
	}
	b := Assertion{ID: "ab43266781e64b51a4c52ffc44d6152c", Type: HandlingAssertion, Scope: PayloadScope, Statement: Statement{Format: "json-structured", Value: `{"a":1}`}}
	h1, _ := b.GetHash()
	b.AppliesToState = Unencrypted
	h2, _ := b.GetHash()
	if string(h1) == string(h2) || len(h1) != 64 {
		t.Fatal("appliesToState must take part when set")
	}
}

func TestAssertionSignVerify(t *testing.T) {
	private, public := rsaPEMs(t)
	secret := make([]byte, 32)
	rand.Read(secret)
	for _, tc := range []struct {
		name     string
		sign     AssertionKey
		verify   AssertionKey
		wantFail bool
	}{
		{"hs256", AssertionKey{AssertionKeyAlgHS256, string(secret)}, AssertionKey{AssertionKeyAlgHS256, string(secret)}, false},
		{"hs256-wrong-key", AssertionKey{AssertionKeyAlgHS256, string(secret)}, AssertionKey{AssertionKeyAlgHS256, "other"}, true},
		{"rs256-public", AssertionKey{AssertionKeyAlgRS256, private}, AssertionKey{AssertionKeyAlgRS256, public}, false},
		{"rs256-private", AssertionKey{AssertionKeyAlgRS256, private}, AssertionKey{AssertionKeyAlgRS256, private}, false},
		{"alg-mismatch", AssertionKey{AssertionKeyAlgHS256, string(secret)}, AssertionKey{AssertionKeyAlgRS256, public}, true},
	} {
		a := Assertion{ID: tc.name}
		if err := a.Sign("hash-value", "sig-value", tc.sign); err != nil {
			t.Fatalf("%s: sign %v", tc.name, err)
		}
		if a.Binding.Method != "jws" || strings.Count(a.Binding.Signature, ".") != 2 {
			t.Fatalf("%s: binding %+v", tc.name, a.Binding)
		}
		header, _ := base64.RawURLEncoding.DecodeString(strings.SplitN(a.Binding.Signature, ".", 2)[0])
		if string(header) != `{"alg":"`+string(tc.sign.Alg)+`","typ":"JWT"}` {
			t.Fatalf("%s: header %s", tc.name, header)
		}
		hash, sig, err := a.Verify(tc.verify)
		if tc.wantFail {
			if err == nil || !errors.Is(err, errAssertionVerifyKeyFailure) {
				t.Fatalf("%s: expected a key failure, got %v", tc.name, err)
			}
			continue
		}
		if err != nil || hash != "hash-value" || sig != "sig-value" {
			t.Fatalf("%s: verify %q %q %v", tc.name, hash, sig, err)
		}
	}
	if _, err := (AssertionKey{}).Algorithm(), (AssertionKey{}).IsEmpty(); !err {
		t.Fatal("empty key")
	}
}

func TestSystemMetadataAssertionConfig(t *testing.T) {
	c, err := GetSystemMetadataAssertionConfig()
	if err != nil {
		t.Fatal(err)
	}
	if c.ID != SystemMetadataAssertionID || c.Type != BaseAssertion || c.Scope != PayloadScope || c.AppliesToState != Unencrypted || c.Statement.Format != "json" || c.Statement.Schema != SystemMetadataSchemaV1 {
		t.Fatalf("%+v", c)
	}
	var meta map[string]string
	if err := json.Unmarshal([]byte(c.Statement.Value), &meta); err != nil {
		t.Fatal(err)
	}
	if meta["tdf_spec_version"] != TDFSpecVersion || meta["sdk_version"] != "Goalchemy-"+Version || !strings.Contains(meta["creation_date"], "T") {
		t.Fatalf("%v", meta)
	}
}

// Test_TDFWithAssertion ports the reference's round trips: default-key
// signing, caller keys with HS256 and RS256, verification keys by ID and by
// default, a missing key, disabled verification, and a tampered statement.
func Test_TDFWithAssertion(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	private, public := rsaPEMs(t)
	secret := make([]byte, 32)
	rand.Read(secret)
	hs := AssertionKey{AssertionKeyAlgHS256, string(secret)}
	rs := AssertionKey{AssertionKeyAlgRS256, private}
	rsPublic := AssertionKey{AssertionKeyAlgRS256, public}
	plain := []byte("assertion payload")
	base64Assertion := AssertionConfig{ID: "assertion1", Type: BaseAssertion, Scope: TrustedDataObjScope, AppliesToState: Unencrypted, Statement: Statement{Format: "base64binary", Schema: "text", Value: "ICAgIDxlZGoOkVkaD4="}}
	jsonAssertion := AssertionConfig{ID: "assertion2", Type: BaseAssertion, Scope: TrustedDataObjScope, AppliesToState: Unencrypted, Statement: Statement{Format: "json", Schema: "urn:nato:stanag:5636:A:1:elements:json", Value: "{\"uuid\":\"f74efb60-4a9a-11ef-a6f1-8ee1a61c148a\",\"body\":{\"dataAttributes\":null,\"dissem\":null}}"}}
	signed := func(c AssertionConfig, k AssertionKey) AssertionConfig { c.SigningKey = k; return c }
	for _, tc := range []struct {
		name       string
		assertions []AssertionConfig
		reader     ReaderOptions
		wantIDs    []string
		wantErr    string
	}{
		{"default-key", []AssertionConfig{base64Assertion, jsonAssertion}, ReaderOptions{}, []string{"assertion1", "assertion2"}, ""},
		{"hs256-default-verifier", []AssertionConfig{signed(base64Assertion, hs), signed(jsonAssertion, hs)}, ReaderOptions{AssertionVerificationKeys: AssertionVerificationKeys{DefaultKey: hs}}, []string{"assertion1", "assertion2"}, ""},
		{"hs256-and-rs256-by-id", []AssertionConfig{signed(base64Assertion, hs), signed(jsonAssertion, rs)}, ReaderOptions{AssertionVerificationKeys: AssertionVerificationKeys{Keys: []AssertionKeyEntry{{"assertion1", hs.Alg, hs.Key}, {"assertion2", rsPublic.Alg, rsPublic.Key}}}}, []string{"assertion1", "assertion2"}, ""},
		{"missing-key", []AssertionConfig{signed(base64Assertion, hs), signed(jsonAssertion, rs)}, ReaderOptions{AssertionVerificationKeys: AssertionVerificationKeys{Keys: []AssertionKeyEntry{{"assertion1", hs.Alg, hs.Key}}}}, nil, "assertion verification failed: assertion: failed to verify with provided key"},
		{"wrong-default-key", []AssertionConfig{signed(base64Assertion, hs)}, ReaderOptions{}, nil, "assertion verification failed: assertion: failed to verify with provided key"},
		{"disabled", []AssertionConfig{signed(base64Assertion, hs)}, ReaderOptions{DisableAssertionVerification: true}, []string{"assertion1"}, ""},
		{"stanag", []AssertionConfig{stanagAssertion("424ff3a3-50ca-4f01-a2ae-ef851cd3cac0")}, ReaderOptions{}, []string{"424ff3a3-50ca-4f01-a2ae-ef851cd3cac0"}, ""},
	} {
		opts := kasOptions(f, RSA2048Key)
		opts.Assertions = tc.assertions
		data, obj, err := s.EncryptBytes(context.Background(), plain, opts)
		if err != nil {
			t.Fatalf("%s: encrypt %v", tc.name, err)
		}
		m := obj.Manifest()
		if len(m.Assertions) != len(tc.assertions) {
			t.Fatalf("%s: %d assertions in manifest", tc.name, len(m.Assertions))
		}
		for i, a := range m.Assertions {
			if a.ID != tc.assertions[i].ID || a.Binding.Method != "jws" || a.Binding.Signature == "" || a.Statement != tc.assertions[i].Statement {
				t.Fatalf("%s: assertion %+v", tc.name, a)
			}
		}
		out, err := s.DecryptBytes(context.Background(), data, tc.reader)
		if tc.wantErr != "" {
			var e *Error
			if err == nil || !errors.As(err, &e) || e.Kind != KindAssertionFailure || e.AssertionID == "" || !strings.Contains(e.Message, tc.wantErr) {
				t.Fatalf("%s: got %v, want %s", tc.name, err, tc.wantErr)
			}
			continue
		}
		if err != nil || !bytes.Equal(out, plain) {
			t.Fatalf("%s: decrypt %v", tc.name, err)
		}
		reader, err := s.LoadTDF(context.Background(), gbytes.NewReader(data), tc.reader)
		if err != nil {
			t.Fatal(err)
		}
		ids := []string{}
		for _, a := range reader.Manifest().Assertions {
			ids = append(ids, a.ID)
		}
		if strings.Join(ids, ",") != strings.Join(tc.wantIDs, ",") {
			t.Fatalf("%s: ids %v", tc.name, ids)
		}
		reader.Close()
	}

	// A tampered statement keeps a valid JWS whose hash claim no longer
	// matches: the reference's "assertion hash missmatch" tamper failure.
	opts := kasOptions(f, RSA2048Key)
	opts.Assertions = []AssertionConfig{base64Assertion}
	data, _, err := s.EncryptBytes(context.Background(), plain, opts)
	if err != nil {
		t.Fatal(err)
	}
	tampered := rewriteTDF(t, data, func(m map[string]any) {
		m["assertions"].([]any)[0].(map[string]any)["statement"].(map[string]any)["value"] = "tampered"
	}, nil)
	_, err = s.DecryptBytes(context.Background(), tampered, ReaderOptions{})
	var e *Error
	if err == nil || !errors.As(err, &e) || e.Kind != KindAssertionFailure || e.AssertionID != "assertion1" || !errors.Is(err, ErrTampered) || !strings.Contains(e.Message, "tdf: issue verifying assertions, id: assertion1: assertion hash missmatch") {
		t.Fatalf("tampered statement: %v", err)
	}
	if _, err := s.DecryptBytes(context.Background(), tampered, ReaderOptions{DisableAssertionVerification: true}); err != nil {
		t.Fatalf("disabled verification must accept the tampered assertion: %v", err)
	}
	// A manifest whose binding was moved to another TDF fails the signature
	// integrity check.
	other, _, err := s.EncryptBytes(context.Background(), []byte("other payload"), opts)
	if err != nil {
		t.Fatal(err)
	}
	var otherManifest map[string]any
	rewriteTDF(t, other, func(m map[string]any) { otherManifest = m }, nil)
	moved := rewriteTDF(t, data, func(m map[string]any) { m["assertions"] = otherManifest["assertions"] }, nil)
	_, err = s.DecryptBytes(context.Background(), moved, ReaderOptions{})
	if err == nil || !errors.As(err, &e) || e.Kind != KindAssertionFailure {
		t.Fatalf("moved assertion: %v", err)
	}
}

func Test_SystemMetadataAssertion(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	opts := kasOptions(f, RSA2048Key)
	opts.SystemMetadataAssertion = true
	opts.Assertions = []AssertionConfig{stanagAssertion("first")}
	data, obj, err := s.EncryptBytes(context.Background(), []byte("meta"), opts)
	if err != nil {
		t.Fatal(err)
	}
	m := obj.Manifest()
	if len(m.Assertions) != 2 || m.Assertions[0].ID != "first" || m.Assertions[1].ID != SystemMetadataAssertionID {
		t.Fatalf("%+v", m.Assertions)
	}
	var meta map[string]string
	if err := json.Unmarshal([]byte(m.Assertions[1].Statement.Value), &meta); err != nil || meta["tdf_spec_version"] != TDFSpecVersion {
		t.Fatalf("%v %v", meta, err)
	}
	out, err := s.DecryptBytes(context.Background(), data, ReaderOptions{})
	if err != nil || string(out) != "meta" {
		t.Fatalf("decrypt %v", err)
	}
}

// A manifest statement whose value is a JSON object parses to its compact
// text and hashes as the reference does; the parsed manifest round-trips.
func TestAssertionStatementObject(t *testing.T) {
	var a Assertion
	if err := json.Unmarshal([]byte(`{"id":"x","type":"other","scope":"payload","statement":{"format":"json","schema":"s","value":{"b":1,"a":"<x>"}},"binding":{"method":"jws","signature":"sig"}}`), &a); err != nil {
		t.Fatal(err)
	}
	escaped, _ := json.Marshal("<x>")
	if a.Statement.Value != `{"b":1,"a":`+string(escaped)+`}` {
		t.Fatalf("value %s", a.Statement.Value)
	}
	hash, err := a.GetHash()
	if err != nil || len(hash) != 64 {
		t.Fatal(err)
	}
}
