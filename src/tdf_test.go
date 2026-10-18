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

// Test_SimpleTDF, over the fake KAS: RSA and EC wrapping and sessions of
// every non-PQ key type, metadata, several segments and an empty payload
// round-trip.
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
		{"rsa-4096", RSA4096Key, RSA4096Key, []byte("rsa 4096"), "meta", 0},
		{"ec-p384", EC384Key, EC384Key, plain, "meta", 32 * 1024},
		{"ec-p521", EC521Key, EC521Key, []byte("ec p521"), "", 0},
		{"ec-p521-rsa-session", EC521Key, RSA2048Key, []byte("mixed"), "", 0},
		{"rsa-ec-p384-session", RSA2048Key, EC384Key, []byte("mixed"), "", 0},
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
		if m.TDFVersion != TDFSpecVersion() || m.Payload.MimeType != "text/plain" || kao.KasURL != f.kasURL() || kao.SchemaVersion != "1.0" || kao.Protocol != "kas" || m.EncryptionInformation.IntegrityInformation.SegmentHashAlgorithm != "GMAC" || m.EncryptionInformation.IntegrityInformation.RootSignature.Algorithm != "HS256" {
			t.Fatalf("%s: manifest %+v", tc.name, m)
		}
		if IsECKeyType(tc.wrapping) {
			if kao.KeyType != "ec-wrapped" || kao.EphemeralPublicKey == "" || kao.KID != fakeKID(string(tc.wrapping)) {
				t.Fatalf("%s: ec kao %+v", tc.name, kao)
			}
		} else if kao.KeyType != "wrapped" || kao.EphemeralPublicKey != "" || kao.KID != fakeKID(string(tc.wrapping)) {
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
	// One fetch per (URL, algorithm) key: the five key types and the
	// unspecified algorithm of the empty case, as the reference caches them.
	if f.publicKeyCalls.Load() != 6 {
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
	if !errors.As(err, &e) || e.Kind != KindAssertionFailure || e.AssertionID != "a" || !strings.Contains(e.Message, "assertion verification failed") {
		t.Fatal("an unsigned assertion fails verification", err)
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

// fakeValue is a policy.Value in protobuf JSON, with its definition and
// namespace, as GetAttributeValuesByFqns returns it. at holds the grants or
// kasKeys of the value, the definition and the namespace.
func fakeValue(ns, name, rule, value string, at map[string]map[string]any) map[string]any {
	namespace := map[string]any{"name": ns, "fqn": "https://" + ns}
	attr := map[string]any{"name": name, "rule": "ATTRIBUTE_RULE_TYPE_ENUM_" + rule, "fqn": "https://" + ns + "/attr/" + name, "namespace": namespace}
	v := map[string]any{"value": value, "fqn": "https://" + ns + "/attr/" + name + "/value/" + value, "attribute": attr}
	for k, x := range at["value"] {
		v[k] = x
	}
	for k, x := range at["attr"] {
		attr[k] = x
	}
	for k, x := range at["ns"] {
		namespace[k] = x
	}
	return v
}

func grants(urls ...string) map[string]any {
	g := []any{}
	for _, u := range urls {
		g = append(g, map[string]any{"uri": u})
	}
	return map[string]any{"grants": g}
}

func kasKeys(keys ...map[string]any) map[string]any {
	k := []any{}
	for _, key := range keys {
		k = append(k, key)
	}
	return map[string]any{"kasKeys": k}
}

func (f *fakePlatform) simpleKey(url, kid string) map[string]any {
	alg := ""
	for _, k := range fakeKASKeys {
		if k.kid == kid {
			alg = k.policyAlg
		}
	}
	return map[string]any{"kasUri": url, "publicKey": map[string]any{"algorithm": alg, "kid": kid, "pem": f.kasPublicPEM(kid)}}
}

func (f *fakePlatform) defineAttributes(values ...map[string]any) []string {
	defs := map[string]map[string]any{}
	fqns := []string{}
	for _, v := range values {
		fqn := v["fqn"].(string)
		defs[strings.ToLower(fqn)] = v
		fqns = append(fqns, fqn)
	}
	f.attributes.Store(defs)
	return fqns
}

// Test_Autoconfigure, over the fake platform: the granter's plan decides the
// key access objects (KAS, split and kid), and the reader rebuilds the DEK
// from them, as the reference's Test_Autoconfigure and Test_KeySplits do.
func Test_Autoconfigure(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	kasA, kasB := f.server.URL+"/kas-a", f.server.URL+"/kas-b"
	type kao struct{ url, kid, typ string }
	for _, tc := range []struct {
		name     string
		mode     string
		values   []map[string]any
		offline  bool
		kaos     []kao
		sameSID  bool
		lookups  bool
		unsplits bool
	}{
		{name: "default kas", values: []map[string]any{fakeValue("a.com", "c", "HIERARCHY", "s", nil)}, kaos: []kao{{f.kasURL(), "r1", "wrapped"}}, unsplits: true},
		{name: "any of two value grants", values: []map[string]any{fakeValue("a.com", "rel", "ANY_OF", "a", map[string]map[string]any{"value": grants(kasA)}), fakeValue("a.com", "rel", "ANY_OF", "b", map[string]map[string]any{"value": grants(kasB)})}, kaos: []kao{{kasA, "r1", "wrapped"}, {kasB, "r1", "wrapped"}}, sameSID: true},
		{name: "all of two value grants", values: []map[string]any{fakeValue("a.com", "n2k", "ALL_OF", "a", map[string]map[string]any{"value": grants(kasA)}), fakeValue("a.com", "n2k", "ALL_OF", "b", map[string]map[string]any{"value": grants(kasB)})}, kaos: []kao{{kasA, "r1", "wrapped"}, {kasB, "r1", "wrapped"}}},
		{name: "attribute grant", values: []map[string]any{fakeValue("a.com", "g", "ANY_OF", "a", map[string]map[string]any{"attr": grants(kasB)})}, kaos: []kao{{kasB, "r1", "wrapped"}}, unsplits: true},
		{name: "namespace grant", values: []map[string]any{fakeValue("ns.com", "g", "ANY_OF", "a", map[string]map[string]any{"ns": grants(kasA)})}, kaos: []kao{{kasA, "r1", "wrapped"}}, unsplits: true},
		{name: "mapped keys of two kinds", mode: "key-mappings", values: []map[string]any{fakeValue("a.com", "mk", "ALL_OF", "r1", map[string]map[string]any{"value": kasKeys(f.simpleKey(f.kasURL(), "r1"))}), fakeValue("a.com", "mk", "ALL_OF", "e1", map[string]map[string]any{"value": kasKeys(f.simpleKey(f.kasURL(), "e1"))})}, kaos: []kao{{f.kasURL(), "r1", "wrapped"}, {f.kasURL(), "e1", "ec-wrapped"}}},
		{name: "mapped extended mechanisms", mode: "key-mappings", values: []map[string]any{
			fakeValue("a.com", "xm", "ALL_OF", "r1", map[string]map[string]any{"value": kasKeys(f.simpleKey(kasA, "r1"))}),
			fakeValue("a.com", "xm", "ALL_OF", "r2", map[string]map[string]any{"value": kasKeys(f.simpleKey(kasA, "r2"))}),
			fakeValue("a.com", "xm", "ALL_OF", "e1", map[string]map[string]any{"value": kasKeys(f.simpleKey(kasB, "e1"))}),
			fakeValue("a.com", "xm", "ALL_OF", "e2", map[string]map[string]any{"value": kasKeys(f.simpleKey(kasB, "e2"))}),
			fakeValue("a.com", "xm", "ALL_OF", "e3", map[string]map[string]any{"value": kasKeys(f.simpleKey(kasB, "e3"))}),
		}, kaos: []kao{{kasA, "r1", "wrapped"}, {kasA, "r2", "wrapped"}, {kasB, "e1", "ec-wrapped"}, {kasB, "e2", "ec-wrapped"}, {kasB, "e3", "ec-wrapped"}}},
		{name: "mapped definition key", mode: "key-mappings", values: []map[string]any{fakeValue("a.com", "dk", "ANY_OF", "x", map[string]map[string]any{"attr": kasKeys(f.simpleKey(kasA, "e1"))})}, kaos: []kao{{kasA, "e1", "ec-wrapped"}}, unsplits: true},
		{name: "offline values", offline: true, values: []map[string]any{fakeValue("a.com", "n2k", "ALL_OF", "a", map[string]map[string]any{"value": grants(kasA)}), fakeValue("a.com", "n2k", "ALL_OF", "b", map[string]map[string]any{"value": kasKeys(f.simpleKey(kasB, "e1"))})}, kaos: []kao{{kasB, "e1", "ec-wrapped"}}, unsplits: true},
	} {
		t.Run(tc.name, func(t *testing.T) {
			f.mode.Store(tc.mode)
			defer f.mode.Store("")
			fqns := f.defineAttributes(tc.values...)
			opts := TDFOptions{KasInformation: []KASInfo{{URL: f.kasURL(), Default: true}}}
			if tc.offline {
				f.attributes.Store(map[string]map[string]any(nil))
				for _, v := range tc.values {
					data, _ := json.Marshal(v)
					opts.AttributeValues = append(opts.AttributeValues, string(data))
				}
			} else {
				opts.Attributes = fqns
			}
			f.served()
			data, obj, err := s.EncryptBytes(context.Background(), []byte("autoconfigured"), opts)
			if err != nil {
				t.Fatal(err)
			}
			for _, path := range f.served() {
				if tc.offline && strings.HasPrefix(path, "/policy.attributes.") {
					t.Fatal("offline values looked up", path)
				}
			}
			kaos := obj.Manifest().EncryptionInformation.KeyAccessObjs
			if len(kaos) != len(tc.kaos) {
				t.Fatalf("%d KAOs: %+v", len(kaos), kaos)
			}
			sids := map[string]bool{}
			for i, want := range tc.kaos {
				if kaos[i].KasURL != want.url || kaos[i].KID != want.kid || kaos[i].KeyType != want.typ {
					t.Fatalf("KAO %d: %+v, want %+v", i, kaos[i], want)
				}
				sids[kaos[i].SplitID] = true
			}
			switch {
			case tc.unsplits:
				if !sids[""] || len(sids) != 1 {
					t.Fatal("one share has no split id", sids)
				}
			case tc.sameSID:
				if len(sids) != 1 || sids[""] {
					t.Fatal("an OR group shares one split id", sids)
				}
			default:
				if len(sids) != len(kaos) || sids[""] {
					t.Fatal("an AND split has one split id per share", sids)
				}
			}
			out, err := s.DecryptBytes(context.Background(), data, ReaderOptions{})
			if err != nil || string(out) != "autoconfigured" {
				t.Fatal(err)
			}
		})
	}
	f.defineAttributes(fakeValue("a.com", "rel", "ANY_OF", "a", nil))
	if _, _, err := s.EncryptBytes(context.Background(), []byte("x"), TDFOptions{Attributes: []string{"https://a.com/attr/rel/value/a"}, AttributeValues: []string{"{}"}}); err == nil {
		t.Fatal("Attributes and AttributeValues together")
	}
	if _, _, err := s.EncryptBytes(context.Background(), []byte("x"), TDFOptions{AttributeValues: []string{`{"fqn":"https://a.com/attr/rel/value/a"}`}, KasInformation: []KASInfo{{URL: f.kasURL()}}}); err == nil || !strings.Contains(err.Error(), "no associated definition with value [https://a.com/attr/rel/value/a]") {
		t.Fatal("an offline value needs its definition", err)
	}
	if _, _, err := s.EncryptBytes(context.Background(), []byte("x"), TDFOptions{Attributes: []string{"https://a.com/attr/rel"}}); err == nil || !strings.Contains(err.Error(), "invalid type: attribute regex fail") {
		t.Fatal("attributes are value FQNs", err)
	}
}

// encryptWithSplitPlan is the reference's withSplitPlan option: the plan's
// keys are fetched from each KAS and the DEK is split across its split ids.
func encryptWithSplitPlan(t *testing.T, s *SDK, plaintext []byte, plan []keySplitStep) ([]byte, *TDFObject) {
	t.Helper()
	tdfConfig, err := newTDFConfig(TDFOptions{DisableAutoconfigure: true})
	if err != nil {
		t.Fatal(err)
	}
	tdfConfig.splitPlan = plan
	data, obj, err := s.encryptBytes(context.Background(), plaintext, tdfConfig)
	if err != nil {
		t.Fatal(err)
	}
	return data, obj
}

// Ported from Test_KeySplits: shared, split and mixed plans over three KAS
// rebuild the DEK by XOR of one share per split id, with one rewrap request
// per KAS; a denied KAS fails only the splits it alone holds.
func Test_KeySplits(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	kasA, kasB, kasC := f.server.URL+"/kas-a", f.server.URL+"/kas-b", f.server.URL+"/kas-c"
	for _, tc := range []struct {
		name      string
		plan      []keySplitStep
		requests  int32
		deniedB   bool
		reconcile bool
	}{
		{name: "shared", plan: []keySplitStep{{kasA, "a"}, {kasB, "a"}, {kasC, "a"}}, requests: 3, reconcile: true},
		{name: "split", plan: []keySplitStep{{kasA, "a"}, {kasB, "b"}, {kasC, "c"}}, requests: 3},
		{name: "mixture", plan: []keySplitStep{{kasA, "a"}, {kasB, "a"}, {kasB, "b"}, {kasC, "b"}}, requests: 3, reconcile: true},
	} {
		t.Run(tc.name, func(t *testing.T) {
			data, obj := encryptWithSplitPlan(t, s, []byte("hello"), tc.plan)
			kaos := obj.Manifest().EncryptionInformation.KeyAccessObjs
			if len(kaos) != len(tc.plan) || kaos[0].KasURL != tc.plan[0].KAS {
				t.Fatalf("KAOs %+v", kaos)
			}
			for i, step := range tc.plan {
				if kaos[i].KasURL != step.KAS || kaos[i].SplitID != step.SplitID {
					t.Fatalf("KAO %d: %+v, want %+v", i, kaos[i], step)
				}
			}
			before := f.rewrapCalls.Load()
			out, err := s.DecryptBytes(context.Background(), data, ReaderOptions{})
			if err != nil || string(out) != "hello" {
				t.Fatal(err)
			}
			if got := f.rewrapCalls.Load() - before; got != tc.requests {
				t.Fatalf("%d rewrap requests, want one per KAS", got)
			}
			f.mu.Lock()
			for _, kao := range f.lastRewrap["requests"].([]any)[0].(map[string]any)["keyAccessObjects"].([]any) {
				if url := kao.(map[string]any)["keyAccessObject"].(map[string]any)["url"]; url != kasC {
					t.Fatal("the last request carries only kas-c's KAOs", url)
				}
			}
			f.mu.Unlock()

			f.mode.Store("deny:" + kasB)
			defer f.mode.Store("")
			out, err = s.DecryptBytes(context.Background(), data, ReaderOptions{})
			if tc.reconcile {
				if err != nil || string(out) != "hello" {
					t.Fatal("an alternative KAS holds every split kas-b holds", err)
				}
				return
			}
			var e *Error
			if !errors.As(err, &e) || e.Kind != KindRewrapForbidden || !errors.Is(err, ErrRewrapForbidden) || !errors.Is(err, ErrTDFDecryptFailed) {
				t.Fatal("denied split", err)
			}
			for _, text := range []string{
				"splitKey.unable to reconstruct split key: map[{" + kasB + " b}:",
				"tdf: rewrap request 403\nkao unwrap failed for split {" + kasB + " b}: rpc error: code = PermissionDenied desc = forbidden",
			} {
				if !strings.Contains(err.Error(), text) {
					t.Fatalf("missing %q in %v", text, err)
				}
			}
			if strings.Contains(err.Error(), "{"+kasA) || strings.Contains(err.Error(), "{"+kasC) {
				t.Fatal("only the failed split is reported", err)
			}
		})
	}
}

// Ported from Test_KeySplit_SameKas_SameAlgorithm and DSPX-3379
// (test_tdf_with_duplicate_kao_same_kas): KAOs on one KAS travel in one
// rewrap request, and a duplicate of an already satisfied split is skipped.
func Test_KeySplit_SameKas(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	data, obj := encryptWithSplitPlan(t, s, []byte("same kas"), []keySplitStep{{f.kasURL(), "a"}, {f.kasURL(), "b"}})
	if kaos := obj.Manifest().EncryptionInformation.KeyAccessObjs; len(kaos) != 2 || kaos[0].KasURL != kaos[1].KasURL || kaos[0].SplitID == kaos[1].SplitID {
		t.Fatalf("KAOs %+v", kaos)
	}
	duplicated := rewriteTDF(t, data, func(m map[string]any) {
		info := encryptionInformation(m)
		kaos := info["keyAccess"].([]any)
		copied := map[string]any{}
		for k, v := range firstKAO(m) {
			copied[k] = v
		}
		info["keyAccess"] = append(kaos, copied)
	}, nil)
	for _, tc := range []struct {
		name string
		data []byte
		kaos int
	}{{"same kas", data, 2}, {"duplicate kao", duplicated, 3}} {
		before := f.rewrapCalls.Load()
		out, err := s.DecryptBytes(context.Background(), tc.data, ReaderOptions{})
		if err != nil || string(out) != "same kas" {
			t.Fatalf("%s: %v", tc.name, err)
		}
		if f.rewrapCalls.Load() != before+1 {
			t.Fatalf("%s: one rewrap request for the KAS", tc.name)
		}
		f.mu.Lock()
		requests := f.lastRewrap["requests"].([]any)
		f.mu.Unlock()
		if len(requests) != 1 || len(requests[0].(map[string]any)["keyAccessObjects"].([]any)) != tc.kaos {
			t.Fatalf("%s: request %v", tc.name, requests)
		}
	}
}

// Post-quantum KAOs are not planned: each fails its split without being
// sent, so an alternative KAO still opens the TDF.
func Test_PostQuantumKAO(t *testing.T) {
	f := newFakePlatform(t)
	s := f.newSDK(t, f.options())
	for _, keyType := range []string{"hybrid-wrapped", "mlkem-wrapped"} {
		data, _, err := s.EncryptBytes(context.Background(), []byte("pq"), kasOptions(f, RSA2048Key))
		if err != nil {
			t.Fatal(err)
		}
		pq := rewriteTDF(t, data, func(m map[string]any) { firstKAO(m)["type"] = keyType }, nil)
		before := f.rewrapCalls.Load()
		_, err = s.DecryptBytes(context.Background(), pq, ReaderOptions{})
		var e *Error
		if !errors.As(err, &e) || e.Kind != KindUnsupportedKeyAccess || !strings.Contains(err.Error(), "unsupported key access type: "+keyType+": post-quantum mechanisms are not planned") || !strings.Contains(err.Error(), "splitKey.unable to reconstruct split key") {
			t.Fatal(keyType, err)
		}
		if f.rewrapCalls.Load() != before {
			t.Fatal("a post-quantum KAO is not sent", keyType)
		}

		shared, _ := encryptWithSplitPlan(t, s, []byte("pq or rsa"), []keySplitStep{{f.kasURL(), "a"}, {f.server.URL + "/kas-b", "a"}})
		alternative := rewriteTDF(t, shared, func(m map[string]any) { firstKAO(m)["type"] = keyType }, nil)
		if out, err := s.DecryptBytes(context.Background(), alternative, ReaderOptions{}); err != nil || string(out) != "pq or rsa" {
			t.Fatal("the other alternative opens the split", keyType, err)
		}
	}
}

// WithWrappingKeyAlg and WithSessionKeyType take every non-PQ key type; a
// post-quantum one fails with the not-planned reason, anything else as the
// reference's ocrypto does.
func TestKeyTypeOptions(t *testing.T) {
	for _, kt := range []KeyType{RSA2048Key, RSA4096Key, EC256Key, EC384Key, EC521Key} {
		if c, err := newTDFConfig(TDFOptions{WrappingKeyAlgorithm: kt}); err != nil || c.preferredKeyWrapAlg != kt {
			t.Fatal(kt, err)
		}
		c, err := newTDFReaderConfig(nil, ReaderOptions{SessionKeyType: kt})
		if err != nil || c.kasSessionKeyType != kt || c.kasSessionKey == nil {
			t.Fatal(kt, err)
		}
		c.closeSessionKey()
	}
	for _, kt := range []KeyType{mlkem768Key, mlkem1024Key, hybridXWingKey, hybridSecp256r1MLKEM768Key, hybridSecp384r1MLKEM1024Key} {
		_, err := newTDFConfig(TDFOptions{WrappingKeyAlgorithm: kt})
		var e *Error
		if !errors.As(err, &e) || e.Kind != KindUnsupportedFeature || e.Message != "unsupported in this build: WrappingKeyAlgorithm "+string(kt)+": post-quantum mechanisms are not planned" {
			t.Fatal(kt, err)
		}
		_, err = newTDFReaderConfig(nil, ReaderOptions{SessionKeyType: kt})
		if !errors.As(err, &e) || e.Kind != KindUnsupportedFeature || e.Message != "unsupported in this build: SessionKeyType "+string(kt)+": post-quantum mechanisms are not planned" {
			t.Fatal(kt, err)
		}
		if _, err := createKeyAccess(KASInfo{URL: "https://kas.example", Algorithm: string(kt)}, make([]byte, 32), PolicyBinding{}, "", ""); !errors.As(err, &e) || e.Kind != KindUnsupportedFeature || !strings.Contains(err.Error(), "post-quantum mechanisms are not planned") {
			t.Fatal(kt, err)
		}
	}
	if _, err := newTDFConfig(TDFOptions{WrappingKeyAlgorithm: "rsa:1024"}); err == nil || err.Error() != "unsupported key type: rsa:1024" {
		t.Fatal(err)
	}
	if _, err := newTDFReaderConfig(nil, ReaderOptions{SessionKeyType: "rsa:1024"}); err == nil || !strings.Contains(err.Error(), "failed to create rsa:1024 key pair") {
		t.Fatal(err)
	}
}
