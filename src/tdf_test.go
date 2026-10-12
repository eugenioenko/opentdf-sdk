package sdk

import (
	"archive/zip"
	"bytes"
	"context"
	"crypto/sha256"
	"encoding/base64"
	"encoding/json"
	"errors"
	"io"
	"strings"
	"testing"

	gbytes "github.com/eugenioenko/goalchemy/std/bytes"
	gstrings "github.com/eugenioenko/goalchemy/std/strings"
)

type manifestEdit func(m map[string]any)

// rewriteTDF applies edit to the manifest JSON (and payloadEdit to the
// payload) of a TDF, like xtest's update_manifest.
func rewriteTDF(t *testing.T, data []byte, edit manifestEdit, payloadEdit func([]byte) []byte) []byte {
	t.Helper()
	r, err := zip.NewReader(bytes.NewReader(data), int64(len(data)))
	if err != nil {
		t.Fatal(err)
	}
	entries := map[string][]byte{}
	for _, f := range r.File {
		rc, _ := f.Open()
		entries[f.Name], _ = io.ReadAll(rc)
		rc.Close()
	}
	if edit != nil {
		var m map[string]any
		if err := json.Unmarshal(entries["0.manifest.json"], &m); err != nil {
			t.Fatal(err)
		}
		edit(m)
		entries["0.manifest.json"], _ = json.Marshal(m)
	}
	if payloadEdit != nil {
		entries["0.payload"] = payloadEdit(entries["0.payload"])
	}
	var out bytes.Buffer
	w := zip.NewWriter(&out)
	for _, name := range []string{"0.payload", "0.manifest.json"} {
		fw, _ := w.CreateHeader(&zip.FileHeader{Name: name, Method: zip.Store})
		fw.Write(entries[name])
	}
	w.Close()
	return out.Bytes()
}

func flipLastThree(b []byte) []byte {
	out := append([]byte(nil), b...)
	for i := len(out) - 3; i < len(out); i++ {
		if i >= 0 {
			out[i] ^= 0x55
		}
	}
	return out
}

func alterBase64(t *testing.T, s string) string {
	t.Helper()
	return base64.StdEncoding.EncodeToString(flipLastThree(mustDecode(t, s)))
}

func encryptionInformation(m map[string]any) map[string]any {
	return m["encryptionInformation"].(map[string]any)
}
func integrityInformation(m map[string]any) map[string]any {
	return encryptionInformation(m)["integrityInformation"].(map[string]any)
}
func firstKAO(m map[string]any) map[string]any {
	return encryptionInformation(m)["keyAccess"].([]any)[0].(map[string]any)
}

func kasOptions(f *fakePlatform, alg KeyType) TDFOptions {
	return TDFOptions{KasInformation: []KASInfo{{URL: f.kasURL()}}, WrappingKeyAlgorithm: alg, Attributes: []string{"https://example.com/attr/attr1/value/value1"}}
}

// Test_SimpleTDF, over the fake KAS: RSA and EC wrapping, RSA and EC
// sessions, metadata, several segments and an empty payload round-trip.
func Test_SimpleTDF(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	plain := bytes.Repeat([]byte("0123456789abcdef"), 5000)
	for _, tc := range []struct {
		name     string
		wrapping KeyType
		session  KeyType
		input    []byte
		metadata string
		segment  int64
	}{
		{"rsa", RSA2048Key, "", plain, "meta data", 16 * 1024},
		{"rsa-ec-session", RSA2048Key, EC256Key, plain, "", 0},
		{"ec", EC256Key, "", []byte("hello"), `{"a":1}`, 0},
		{"ec-ec-session", EC256Key, EC256Key, plain, "", 32 * 1024},
		{"empty", "", RSA2048Key, []byte{}, "", 0},
		{"exact-multiple", RSA2048Key, "", bytes.Repeat([]byte("x"), 32*1024), "", 16 * 1024},
	} {
		opts := kasOptions(f, tc.wrapping)
		opts.Metadata = tc.metadata
		opts.SegmentSize = tc.segment
		opts.MimeType = "text/plain"
		data, obj, err := s.EncryptBytes(context.Background(), tc.input, opts)
		if err != nil {
			t.Fatalf("%s: encrypt %v", tc.name, err)
		}
		if obj.Size() != int64(len(data)) {
			t.Fatalf("%s: size", tc.name)
		}
		m := obj.Manifest()
		kao := m.EncryptionInformation.KeyAccessObjs[0]
		if m.TDFVersion != TDFSpecVersion || m.Payload.MimeType != "text/plain" || kao.KasURL != f.kasURL() || kao.SchemaVersion != "1.0" || kao.Protocol != "kas" || m.EncryptionInformation.IntegrityInformation.SegmentHashAlgorithm != "GMAC" || m.EncryptionInformation.IntegrityInformation.RootSignature.Algorithm != "HS256" {
			t.Fatalf("%s: manifest %+v", tc.name, m)
		}
		if tc.wrapping == EC256Key {
			if kao.KeyType != "ec-wrapped" || kao.EphemeralPublicKey == "" || kao.KID != "e1" {
				t.Fatalf("%s: ec kao %+v", tc.name, kao)
			}
		} else if kao.KeyType != "wrapped" || kao.EphemeralPublicKey != "" || kao.KID != "r1" {
			t.Fatalf("%s: rsa kao %+v", tc.name, kao)
		}
		if (tc.metadata != "") != (kao.EncryptedMetadata != "") {
			t.Fatalf("%s: metadata presence", tc.name)
		}
		expectedSegments := (len(tc.input) + int(m.EncryptionInformation.IntegrityInformation.DefaultSegmentSize) - 1) / int(m.EncryptionInformation.IntegrityInformation.DefaultSegmentSize)
		if expectedSegments == 0 {
			expectedSegments = 1
		}
		if len(m.EncryptionInformation.IntegrityInformation.Segments) != expectedSegments {
			t.Fatalf("%s: %d segments, want %d", tc.name, len(m.EncryptionInformation.IntegrityInformation.Segments), expectedSegments)
		}
		valid, err := IsValidTdf(gbytes.NewReader(data))
		if err != nil || !valid {
			t.Fatalf("%s: valid %v", tc.name, err)
		}
		readerOpts := ReaderOptions{SessionKeyType: tc.session}
		before := f.rewrapCalls.Load()
		out, err := s.DecryptBytes(context.Background(), data, readerOpts)
		if err != nil || !bytes.Equal(out, tc.input) {
			t.Fatalf("%s: decrypt %v", tc.name, err)
		}
		if f.rewrapCalls.Load() != before+1 {
			t.Fatalf("%s: one rewrap per decrypt", tc.name)
		}
		reader, err := s.LoadTDF(context.Background(), gbytes.NewReader(data), readerOpts)
		if err != nil {
			t.Fatal(err)
		}
		attrs, err := reader.DataAttributes()
		if err != nil || len(attrs) != 1 || attrs[0] != "https://example.com/attr/attr1/value/value1" {
			t.Fatalf("%s: attributes %v %v", tc.name, attrs, err)
		}
		policy, err := reader.Policy()
		if err != nil || policy.UUID == "" || len(policy.Body.DataAttributes) != 1 || policy.Body.Dissem == nil {
			t.Fatalf("%s: policy %+v %v", tc.name, policy, err)
		}
		if err := reader.Init(context.Background()); err != nil {
			t.Fatalf("%s: init %v", tc.name, err)
		}
		meta, err := reader.UnencryptedMetadata()
		if err != nil || string(meta) != tc.metadata {
			t.Fatalf("%s: metadata %q %v", tc.name, meta, err)
		}
		if reader.Manifest().EncryptionInformation.Policy != m.EncryptionInformation.Policy {
			t.Fatalf("%s: manifest round trip", tc.name)
		}
		reader.Close()
	}
	// One fetch per (URL, algorithm) key: rsa:2048, ec:secp256r1 and the
	// unspecified algorithm of the empty case, as the reference caches them.
	if f.publicKeyCalls.Load() != 3 {
		t.Fatal("KAS keys fetched once per algorithm and cached", f.publicKeyCalls.Load())
	}
	if f.registryCalls.Load() == 0 {
		t.Fatal("allowlist fetched from the registry")
	}
	f.mu.Lock()
	auth := f.lastAuth
	f.mu.Unlock()
	if auth != "Bearer access-token-Bearer" {
		t.Fatal("bearer token on rewrap", auth)
	}
}

// Test_DefaultTDF: with no KAS information the writer uses the base key,
// and without one the platform KAS from the default list.
func Test_DefaultTDF(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	f.baseKey.Store("ec")
	data, obj, err := s.EncryptBytes(context.Background(), []byte("base key"), TDFOptions{KasInformation: []KASInfo{{URL: "https://ignored.example"}}})
	if err != nil {
		t.Fatal(err)
	}
	kao := obj.Manifest().EncryptionInformation.KeyAccessObjs[0]
	if kao.KeyType != "ec-wrapped" || kao.KID != "e1" || kao.KasURL != f.kasURL() {
		t.Fatal("base key overrides kas information", kao)
	}
	out, err := s.DecryptBytes(context.Background(), data, ReaderOptions{})
	if err != nil || string(out) != "base key" {
		t.Fatal(err)
	}
	f.baseKey.Store("")
	if _, _, err := s.EncryptBytes(context.Background(), []byte("x"), TDFOptions{}); err == nil || !strings.Contains(err.Error(), "no default KAS specified") {
		t.Fatal("no base key and no KAS information", err)
	}
	if _, _, err := s.EncryptBytes(context.Background(), []byte("x"), TDFOptions{DisableAutoconfigure: true}); err == nil || !strings.Contains(err.Error(), errInvalidKasInfo.Error()) {
		t.Fatal("no template without autoconfigure", err)
	}
	data, _, err = s.EncryptBytes(context.Background(), []byte("explicit"), TDFOptions{DisableAutoconfigure: true, KasInformation: []KASInfo{{URL: f.kasURL(), KID: "r1", PublicKey: publicPEM(t, &f.kasRSA.PublicKey), Algorithm: "rsa:2048"}}})
	if err != nil {
		t.Fatal(err)
	}
	if out, err := s.DecryptBytes(context.Background(), data, ReaderOptions{}); err != nil || string(out) != "explicit" {
		t.Fatal("explicit kas information bypasses key lookup", err)
	}
}

// CreateTDF over host streams reads the payload whole and writes the TDF.
func Test_CreateTDF_Streams(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	var out bytes.Buffer
	obj, err := s.CreateTDF(context.Background(), &out, gstrings.NewReader("streamed"), kasOptions(f, RSA2048Key))
	if err != nil || obj.Size() != int64(out.Len()) {
		t.Fatal(err)
	}
	plain, err := s.DecryptBytes(context.Background(), out.Bytes(), ReaderOptions{})
	if err != nil || string(plain) != "streamed" {
		t.Fatal(err)
	}
	opts := kasOptions(f, RSA2048Key)
	opts.InputSize = 3
	data, _, err := s.EncryptBytes(context.Background(), []byte("abcdef"), opts)
	if err != nil {
		t.Fatal(err)
	}
	if plain, err := s.DecryptBytes(context.Background(), data, ReaderOptions{}); err != nil || string(plain) != "abc" {
		t.Fatal("declared size bounds the input", err)
	}
	opts.InputSize = 10
	if _, _, err := s.EncryptBytes(context.Background(), []byte("abcdef"), opts); !errors.Is(err, errInputShorterThanDeclared) {
		t.Fatal("short input", err)
	}
}

// Tamper cells of test_tdfs.py against the fake KAS, with the reference
// wording of docs/error-wording.md.
func Test_TDFReaderFail(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	data, _, err := s.EncryptBytes(context.Background(), bytes.Repeat([]byte("payload "), 8*1024), kasOptions(f, RSA2048Key))
	if err != nil {
		t.Fatal(err)
	}
	for _, tc := range []struct {
		name     string
		edit     manifestEdit
		payload  func([]byte) []byte
		kind     ErrorKind
		sentinel error
		text     string
		stage    error
	}{
		{"altered_root_sig", func(m map[string]any) {
			rs := integrityInformation(m)["rootSignature"].(map[string]any)
			rs["sig"] = alterBase64(t, rs["sig"].(string))
		}, nil, KindRootSignature, ErrRootSigValidation, "tdf: issue verifying root signature: [tamper detected] tdf: failed integrity check on root signature", ErrTDFDecryptFailed},
		{"altered_seg_sig", func(m map[string]any) {
			seg := integrityInformation(m)["segments"].([]any)[0].(map[string]any)
			seg["hash"] = alterBase64(t, seg["hash"].(string))
		}, nil, KindRootSignature, ErrRootSignatureFailure, "root signature", ErrTDFDecryptFailed},
		{"altered_enc_seg_size", func(m map[string]any) {
			seg := integrityInformation(m)["segments"].([]any)[0].(map[string]any)
			seg["encryptedSegmentSize"] = seg["encryptedSegmentSize"].(float64) + 1
		}, nil, KindSegmentSize, ErrSegSizeMismatch, "mismatch encrypted segment size", ErrTDFNotDecryptable},
		{"altered_payload_end", nil, flipLastThree, KindSegmentSignature, ErrSegSigValidation, "failed integrity check on segment hash", ErrTDFDecryptFailed},
		{"altered_policy_binding", func(m map[string]any) {
			pb := firstKAO(m)["policyBinding"].(map[string]any)
			pb["hash"] = alterBase64(t, pb["hash"].(string))
		}, nil, KindRewrapBadRequest, ErrRewrapBadRequest, "tdf: rewrap request 400", ErrTDFDecryptFailed},
		{"unbound_policy", func(m map[string]any) {
			encryptionInformation(m)["policy"] = base64.StdEncoding.EncodeToString([]byte(`{"uuid":"x","body":{"dataAttributes":[],"dissem":["yves@dropp.er"]}}`))
		}, nil, KindRewrapBadRequest, ErrRewrapBadRequest, "desc = bad request", ErrTDFDecryptFailed},
		{"unsupported_root_alg", func(m map[string]any) {
			integrityInformation(m)["rootSignature"].(map[string]any)["alg"] = "GMAC"
		}, nil, KindUnsupportedRootIntegrity, ErrUnsupportedRootIntegrityAlgorithm, `tdf: unsupported root integrity algorithm: "GMAC"`, ErrTDFDecryptFailed},
	} {
		tampered := rewriteTDF(t, data, tc.edit, tc.payload)
		_, err := s.DecryptBytes(context.Background(), tampered, ReaderOptions{})
		var e *Error
		if !errors.As(err, &e) {
			t.Fatalf("%s: %v", tc.name, err)
		}
		if e.Kind != tc.kind || !errors.Is(err, tc.sentinel) || !errors.Is(err, tc.stage) || !strings.Contains(err.Error(), tc.text) || (tc.kind != KindUnsupportedRootIntegrity && !errors.Is(err, ErrTampered)) {
			t.Fatalf("%s: kind %s, %v", tc.name, e.Kind, err)
		}
	}
	if _, err := s.DecryptBytes(context.Background(), []byte("not a tdf"), ReaderOptions{}); !errors.Is(err, ErrTDFNotDecryptable) {
		t.Fatal("invalid ciphertext", err)
	}
	if _, err := s.DecryptBytes(context.Background(), data, ReaderOptions{MaxManifestSize: -1}); !errors.Is(err, ErrTDFNotDecryptable) {
		t.Fatal("option error surfaces as not decryptable", err)
	}
	if _, err := s.DecryptBytes(context.Background(), data, ReaderOptions{MaxManifestSize: 100}); !errors.Is(err, ErrTDFNotDecryptable) {
		t.Fatal("manifest over the limit", err)
	}
	f.mode.Store("deny")
	_, err = s.DecryptBytes(context.Background(), data, ReaderOptions{})
	var e *Error
	if !errors.As(err, &e) || e.Kind != KindRewrapForbidden || !errors.Is(err, ErrRewrapForbidden) || !strings.Contains(err.Error(), "unable to reconstruct split key") || !strings.Contains(err.Error(), "forbidden") {
		t.Fatal("forbidden", err)
	}
	f.mode.Store("misconfigured")
	if _, err := s.DecryptBytes(context.Background(), data, ReaderOptions{}); !errors.Is(err, ErrKASRequestError) || errors.Is(err, ErrTampered) {
		t.Fatal("misconfiguration is not tamper", err)
	}
	f.mode.Store("no-policy")
	if _, err := s.DecryptBytes(context.Background(), data, ReaderOptions{}); err == nil || !strings.Contains(err.Error(), "could not find policy in rewrap response") {
		t.Fatal("missing policy", err)
	}
	f.mode.Store("rewrap-unauthenticated")
	if _, err := s.DecryptBytes(context.Background(), data, ReaderOptions{}); err == nil || !strings.Contains(err.Error(), "unauthenticated: invalid token") {
		t.Fatal("connect error text", err)
	}
	f.mode.Store("obligations")
	reader, err := s.LoadTDF(context.Background(), gbytes.NewReader(data), ReaderOptions{})
	if err != nil || reader.Init(context.Background()) != nil {
		t.Fatal(err)
	}
	if reader.requiredObligations == nil || len(reader.requiredObligations.FQNs) != 1 {
		t.Fatal("obligations recorded from rewrap metadata")
	}
	f.mode.Store("")
	withAssertions := rewriteTDF(t, data, func(m map[string]any) {
		m["assertions"] = []any{map[string]any{"id": "a", "type": "other", "scope": "payload", "statement": map[string]any{"format": "json", "value": map[string]any{"k": "v"}}}}
	}, nil)
	_, err = s.DecryptBytes(context.Background(), withAssertions, ReaderOptions{})
	if !errors.As(err, &e) || e.Kind != KindUnsupportedFeature {
		t.Fatal("assertions unsupported until Phase 5", err)
	}
	if out, err := s.DecryptBytes(context.Background(), withAssertions, ReaderOptions{DisableAssertionVerification: true}); err != nil || len(out) == 0 {
		t.Fatal("verification disabled", err)
	}
}

// Test_TDF_KAS_Allowlist: a KAO naming a KAS outside the allowlist is
// rejected before any request, with the reference wording.
func Test_TDF_KAS_Allowlist(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	data, _, err := s.EncryptBytes(context.Background(), []byte("allow"), kasOptions(f, RSA2048Key))
	if err != nil {
		t.Fatal(err)
	}
	malicious := rewriteTDF(t, data, func(m map[string]any) { firstKAO(m)["url"] = "http://localhost:9999/malicious/kas" }, nil)
	before := f.rewrapCalls.Load()
	_, err = s.DecryptBytes(context.Background(), malicious, ReaderOptions{})
	var e *Error
	if !errors.As(err, &e) || e.Kind != KindAllowlistRejected || !strings.Contains(err.Error(), "KasAllowlist: kas url http://localhost:9999/malicious/kas is not allowed") || e.KASURL != "http://localhost:9999/malicious/kas" {
		t.Fatal("allowlist", err)
	}
	if f.rewrapCalls.Load() != before {
		t.Fatal("rejected before any request")
	}
	if _, err := s.DecryptBytes(context.Background(), data, ReaderOptions{KasAllowlist: []string{"https://other.example"}}); !errors.As(err, &e) || e.Kind != KindAllowlistRejected {
		t.Fatal("explicit allowlist", err)
	}
	registry := f.registryCalls.Load()
	if out, err := s.DecryptBytes(context.Background(), data, ReaderOptions{KasAllowlist: []string{f.kasURL()}}); err != nil || string(out) != "allow" || f.registryCalls.Load() != registry {
		t.Fatal("explicit allowlist skips the registry", err)
	}
	if out, err := s.DecryptBytes(context.Background(), data, ReaderOptions{IgnoreAllowlist: true}); err != nil || string(out) != "allow" || f.registryCalls.Load() != registry {
		t.Fatal("ignore allowlist", err)
	}
	f.mode.Store("registry-error")
	if _, err := s.DecryptBytes(context.Background(), data, ReaderOptions{}); err == nil || !strings.Contains(err.Error(), "allowListFromKASRegistry failed") {
		t.Fatal("registry failure", err)
	}
}

// Ported from TestGetKasErrorToReturn.
func TestGetKasErrorToReturn(t *testing.T) {
	defaultError := errors.New("default KAS error")
	result := getKasErrorToReturn(errors.New("rpc error: code = InvalidArgument desc = bad request"), defaultError)
	if !errors.Is(result, ErrRewrapBadRequest) || !errors.Is(result, ErrTampered) || errors.Is(result, ErrKASRequestError) || !errors.Is(result, defaultError) {
		t.Fatal("generic bad request", result)
	}
	for _, msg := range []string{"unsupported key type", "key access object is nil", "ec-wrapped not enabled", "wrapped key is empty", "invalid ephemeral public key", "no legacy key IDs found", "failed to get additional rewrap context"} {
		result := getKasErrorToReturn(errors.New("rpc error: code = InvalidArgument desc = "+msg), defaultError)
		if !errors.Is(result, ErrKASRequestError) || errors.Is(result, ErrTampered) || !errors.Is(result, defaultError) {
			t.Fatal(msg, result)
		}
	}
	result = getKasErrorToReturn(errors.New("rpc error: code = InvalidArgument desc = bad request body"), defaultError)
	if !errors.Is(result, ErrRewrapBadRequest) || !errors.Is(result, ErrTampered) {
		t.Fatal("substring match", result)
	}
	result = getKasErrorToReturn(errors.New("rpc error: code = InvalidArgument bad request: unsupported key type"), defaultError)
	if !errors.Is(result, ErrKASRequestError) || errors.Is(result, ErrTampered) {
		t.Fatal("unanchored bad request", result)
	}
	result = getKasErrorToReturn(errors.New("rpc error: code = PermissionDenied desc = access denied"), defaultError)
	if !errors.Is(result, ErrRewrapForbidden) || errors.Is(result, ErrKASRequestError) || !errors.Is(result, defaultError) {
		t.Fatal("permission denied", result)
	}
	other := errors.New("rpc error: code = Internal desc = internal server error")
	if result := getKasErrorToReturn(other, defaultError); result != defaultError {
		t.Fatal("other error returns default", result)
	}
}

// Ported from Test_SegmentCount and Test_SegmentCountTooManySegments.
func Test_SegmentCount(t *testing.T) {
	const segmentSize = 1024
	for _, tc := range []struct {
		inputSize, segmentSize int64
		expected               int
	}{
		{0, segmentSize, 1}, {1, segmentSize, 1}, {segmentSize, segmentSize, 1}, {segmentSize + 1, segmentSize, 2}, {3 * segmentSize, segmentSize, 3},
		{9223372036854775807, 1 << 33, 1 << 30},
	} {
		count, err := segmentCount(tc.inputSize, tc.segmentSize)
		if err != nil || count != tc.expected {
			t.Fatalf("segmentCount(%d, %d) = %d, %v", tc.inputSize, tc.segmentSize, count, err)
		}
	}
	if count, err := segmentCount(9223372036854775807, minSegmentSize); !errors.Is(err, errTooManySegments) || count != 0 {
		t.Fatal("too many segments", count, err)
	}
}

// Ported from TestDedupRequiredObligations.
func TestDedupRequiredObligations(t *testing.T) {
	for _, tc := range []struct {
		name     string
		results  []kaoResult
		expected []string
	}{
		{"empty", []kaoResult{}, []string{}},
		{"none", []kaoResult{{KeyAccessObjectID: "kao-1"}}, []string{}},
		{"single", []kaoResult{{RequiredObligations: []string{"https://demo.com/obl/test/value/watermark"}}}, []string{"https://demo.com/obl/test/value/watermark"}},
		{"dedupe and normalise", []kaoResult{{RequiredObligations: []string{"https://demo.com/obl/test/value/Watermark", " https://demo.com/obl/test/value/geofence "}}, {RequiredObligations: []string{"https://demo.com/obl/test/value/watermark", ""}}}, []string{"https://demo.com/obl/test/value/watermark", "https://demo.com/obl/test/value/geofence"}},
	} {
		got := dedupRequiredObligations(tc.results)
		if len(got) != len(tc.expected) {
			t.Fatalf("%s: %v", tc.name, got)
		}
		for i := range got {
			if got[i] != tc.expected[i] {
				t.Fatalf("%s: %v", tc.name, got)
			}
		}
	}
}

// Ported from TestResolveSegmentSizes.
func TestResolveSegmentSizes(t *testing.T) {
	defaults := IntegrityInformation{DefaultSegmentSize: 1024, DefaultEncryptedSegSize: 1052}
	for _, tc := range []struct {
		name          string
		integrity     IntegrityInformation
		segment       Segment
		wantSize      int64
		wantEncrypted int64
		wantErr       error
	}{
		{"explicit sizes win", defaults, Segment{Size: 7, EncryptedSize: 35}, 7, 35, nil},
		{"both omitted fall back", defaults, Segment{}, 1024, 1052, nil},
		{"Size explicit, EncryptedSize omitted", defaults, Segment{Size: 1024}, 1024, 1052, nil},
		{"explicit zero-length segment", defaults, Segment{Size: 0, EncryptedSize: 28}, 0, 28, nil},
		{"Size omitted, EncryptedSize equal to default", defaults, Segment{EncryptedSize: 1052}, 1024, 1052, nil},
		{"no value and no default", IntegrityInformation{}, Segment{}, 0, 0, ErrSegSizeUnresolved},
		{"negative default", IntegrityInformation{DefaultSegmentSize: -1, DefaultEncryptedSegSize: -1}, Segment{}, 0, 0, ErrSegSizeUnresolved},
		{"inconsistent pair", defaults, Segment{Size: 10, EncryptedSize: 35}, 0, 0, ErrSegSizeMismatch},
		{"encrypted below framing", defaults, Segment{Size: 0, EncryptedSize: 20}, 0, 0, ErrSegSizeMismatch},
	} {
		size, encrypted, err := tc.integrity.resolveSegmentSizes(tc.segment)
		if tc.wantErr != nil {
			if !errors.Is(err, tc.wantErr) || !errors.Is(err, ErrTampered) {
				t.Fatalf("%s: %v", tc.name, err)
			}
			continue
		}
		if err != nil || size != tc.wantSize || encrypted != tc.wantEncrypted {
			t.Fatalf("%s: %d %d %v", tc.name, size, encrypted, err)
		}
	}
}

// Ported from key_access_test.go: createKeyAccess shape and error paths,
// encryptMetadata and tdfSalt.
func TestCreateKeyAccess(t *testing.T) {
	f := newFakePlatform(t)
	symKey := make([]byte, kKeySize)
	for i := range symKey {
		symKey[i] = byte(i)
	}
	kao, err := createKeyAccess(KASInfo{URL: "https://kas1.example.com/", PublicKey: publicPEM(t, &f.kasRSA.PublicKey), KID: "test-kid", Algorithm: "rsa:2048"}, symKey, PolicyBinding{Alg: "HS256", Hash: "test-binding"}, "encrypted-metadata", "split-1")
	if err != nil {
		t.Fatal(err)
	}
	if kao.KeyType != kWrapped || kao.KasURL != "https://kas1.example.com/" || kao.KID != "test-kid" || kao.Protocol != kKasProtocol || kao.SplitID != "split-1" || kao.SchemaVersion != keyAccessSchemaVersion || kao.EncryptedMetadata != "encrypted-metadata" || kao.PolicyBinding != (PolicyBinding{Alg: "HS256", Hash: "test-binding"}) || kao.WrappedKey == "" || kao.EphemeralPublicKey != "" {
		t.Fatalf("rsa kao %+v", kao)
	}
	if unwrapped, err := f.unwrap(fakeKAO{Type: kao.KeyType, WrappedKey: kao.WrappedKey}); err != nil || !bytes.Equal(unwrapped, symKey) {
		t.Fatal("rsa unwrap", err)
	}
	for _, pub := range []string{"", "invalid-pem-data"} {
		if _, err := createKeyAccess(KASInfo{URL: "https://kas1.example.com/", PublicKey: pub, Algorithm: "rsa:2048"}, symKey, PolicyBinding{}, "", "split-1"); err == nil {
			t.Fatal("bad public key accepted")
		}
	}
	ecKao, err := createKeyAccess(KASInfo{URL: "https://kas1.example.com/", PublicKey: publicPEM(t, &f.kasEC.PublicKey), Algorithm: "ec:secp256r1"}, symKey, PolicyBinding{}, "", "")
	if err != nil || ecKao.KeyType != kECWrapped || ecKao.EphemeralPublicKey == "" {
		t.Fatal("ec kao", err)
	}
	if unwrapped, err := f.unwrap(fakeKAO{Type: ecKao.KeyType, WrappedKey: ecKao.WrappedKey, EphemeralPublicKey: ecKao.EphemeralPublicKey}); err != nil || !bytes.Equal(unwrapped, symKey) {
		t.Fatal("ec unwrap the way the KAS does", err)
	}
	encrypted, err := encryptMetadata(symKey, "test metadata content")
	if err != nil {
		t.Fatal(err)
	}
	var em EncryptedMetadata
	if err := json.Unmarshal(mustDecode(t, encrypted), &em); err != nil || em.Cipher == "" || em.Iv == "" {
		t.Fatal("metadata shape", err)
	}
	frame := mustDecode(t, em.Cipher)
	if !bytes.Equal(mustDecode(t, em.Iv), frame[:12]) {
		t.Fatal("iv prefixes the ciphertext")
	}
	if plain, err := gcmOpen(symKey, frame); err != nil || string(plain) != "test metadata content" {
		t.Fatal("metadata decrypts", err)
	}
	salt, err := tdfSalt()
	expected := sha256.Sum256([]byte("TDF"))
	if err != nil || !bytes.Equal(salt, expected[:]) {
		t.Fatal("salt is SHA-256 of TDF", err)
	}
}

func TestPolicyBindingJSON(t *testing.T) {
	var k KeyAccess
	if err := json.Unmarshal([]byte(`{"type":"wrapped","url":"u","protocol":"kas","wrappedKey":"w","policyBinding":"legacyhash"}`), &k); err != nil || k.PolicyBinding != (PolicyBinding{Hash: "legacyhash"}) {
		t.Fatal("legacy string binding", k, err)
	}
	out, _ := json.Marshal(k)
	if !strings.Contains(string(out), `"policyBinding":"legacyhash"`) {
		t.Fatal("legacy round trip", string(out))
	}
	if err := json.Unmarshal([]byte(`{"policyBinding":{"alg":"HS256","hash":"h"}}`), &k); err != nil || k.PolicyBinding != (PolicyBinding{Alg: "HS256", Hash: "h"}) {
		t.Fatal("object binding", k, err)
	}
	out, _ = json.Marshal(k)
	if !strings.Contains(string(out), `"policyBinding":{"alg":"HS256","hash":"h"}`) {
		t.Fatal("object round trip", string(out))
	}
	if err := json.Unmarshal([]byte(`{"policyBinding":{"alg":"HS256"}}`), &k); err == nil {
		t.Fatal("missing hash")
	}
	var st Statement
	if err := json.Unmarshal([]byte(`{"format":"json","value":{"b":1,"a":2}}`), &st); err != nil || st.Value != `{"b":1,"a":2}` {
		t.Fatal("object statement", st, err)
	}
}
