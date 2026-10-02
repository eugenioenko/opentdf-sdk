package sdk

import (
	"bytes"
	"context"
	"crypto/aes"
	"crypto/cipher"
	"crypto/hkdf"
	"crypto/rand"
	"crypto/sha256"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"encoding/pem"
	"errors"
	"sync"
	"testing"
	"time"

	capcrypto "github.com/eugenioenko/goalchemy/lib/crypto"
	"opentdf-local/sdk/tdf"
	j "opentdf-local/sdk/tdf/json"
)

func testSPKI(t *testing.T, public any) string {
	t.Helper()
	der, e := x509.MarshalPKIXPublicKey(public)
	if e != nil {
		t.Fatal(e)
	}
	return string(pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: der}))
}
func testPrivatePEM(t *testing.T, private any) string {
	t.Helper()
	der, e := x509.MarshalPKCS8PrivateKey(private)
	if e != nil {
		t.Fatal(e)
	}
	return string(pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: der}))
}
func testHKDF(t *testing.T, secret []byte) []byte {
	t.Helper()
	salt := sha256.Sum256([]byte("TDF"))
	key, e := hkdf.Key(sha256.New, secret, salt[:], "", 32)
	if e != nil {
		t.Fatal(e)
	}
	return key
}
func testGCMSeal(t *testing.T, key, plain []byte) []byte {
	t.Helper()
	block, e := aes.NewCipher(key)
	if e != nil {
		t.Fatal(e)
	}
	gcm, e := cipher.NewGCM(block)
	if e != nil {
		t.Fatal(e)
	}
	nonce := make([]byte, 12)
	if _, e := rand.Read(nonce); e != nil {
		t.Fatal(e)
	}
	return gcm.Seal(nonce, nonce, plain, nil)
}
func testGCMOpen(key, frame []byte) ([]byte, error) {
	block, e := aes.NewCipher(key)
	if e != nil {
		return nil, e
	}
	gcm, e := cipher.NewGCM(block)
	if e != nil {
		return nil, e
	}
	if len(frame) < 28 {
		return nil, errors.New("frame")
	}
	return gcm.Open(nil, frame[:12], frame[12:], nil)
}
func configureFixture(t *testing.T, f *fixture, session, auth string, explicit bool) {
	t.Helper()
	f.client.Close()
	config := Config{PlatformURL: f.server.URL, KASURL: f.server.URL + "/kas", AllowedKAS: []KASRoute{{URL: f.server.URL + "/kas", APIBaseURL: f.server.URL + "/rpc"}}, AllowHTTP: true, KASAlgorithm: f.algorithm, SessionAlgorithm: session, AuthAlgorithm: auth, TokenProvider: func(context.Context) (AccessToken, error) {
		return AccessToken{Value: "valid-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}, nil
	}}
	if explicit {
		config.KASPublicKey = f.public
		config.KID = "r1"
	}
	c, e := New(config)
	if e != nil {
		t.Fatal(e)
	}
	f.client = c
}
func TestIndependentWrappingAndResponseSessionMatrix(t *testing.T) {
	for _, wrapping := range []string{"rsa:2048", "ec:secp256r1"} {
		for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
			for _, auth := range []string{"RS256", "ES256"} {
				t.Run(wrapping+"/"+session+"/"+auth, func(t *testing.T) {
					f := newAlgorithmFixture(t, wrapping)
					configureFixture(t, f, session, auth, false)
					policy, e := tdf.NewPolicy([]string{"allowed"}, nil)
					if e != nil {
						t.Fatal(e)
					}
					raw, e := policy.Marshal()
					if e != nil {
						t.Fatal(e)
					}
					var pretty bytes.Buffer
					if e := json.Indent(&pretty, raw, "", " "); e != nil {
						t.Fatal(e)
					}
					bound := base64.StdEncoding.EncodeToString(pretty.Bytes())
					payload := bytes.Repeat([]byte{0, 255, 128, 7}, 8193)
					metadata := []byte(`{"independent":true}`)
					data, e := f.client.Create(context.Background(), payload, tdf.EncryptConfig{Algorithm: wrapping, PolicyBase64: bound, SegmentSize: 16384, SegmentHashAlgorithm: "HS256", Metadata: metadata})
					if e != nil {
						t.Fatal(e)
					}
					a, e := tdf.ReadArchive(data, tdf.DefaultArchiveLimits())
					if e != nil {
						t.Fatal(e)
					}
					m, e := tdf.ParseManifest(a.Manifest)
					if e != nil {
						t.Fatal(e)
					}
					m.Encryption.KeyAccess[0].SID = "ec-share-1"
					manifest, e := m.Marshal()
					if e != nil {
						t.Fatal(e)
					}
					data, e = tdf.WriteArchive(a.Payload, manifest, tdf.DefaultArchiveLimits())
					if e != nil {
						t.Fatal(e)
					}
					request, e := rewrapBody(m, "session")
					if e != nil {
						t.Fatal(e)
					}
					var b wireBody
					if e = json.Unmarshal(request, &b); e != nil {
						t.Fatal(e)
					}
					kao := b.Requests[0].KeyAccessObjects[0].KeyAccessObject
					if b.Requests[0].Policy.Body != bound || kao.Sid != "ec-share-1" || kao.EncryptedMetadata != m.Encryption.KeyAccess[0].EncryptedMetadata || kao.EphemeralPublicKey != m.Encryption.KeyAccess[0].EphemeralPublicKey || bytes.Contains(request, []byte("schemaVersion")) {
						t.Fatal("KAO/policy fields changed")
					}
					for i := 0; i < 2; i++ {
						d, e := f.client.Decrypt(context.Background(), data)
						if e != nil || !bytes.Equal(d.Payload, payload) || !bytes.Equal(d.Metadata, metadata) {
							t.Fatal("matrix decrypt", e)
						}
					}
					f.client.Close()
					if _, e := f.public.PublicPEM(); e != nil {
						t.Fatal("caller wrapping key closed", e)
					}
				})
			}
		}
	}
}
func TestECSessionFailuresReturnZeroOutput(t *testing.T) {
	f := newAlgorithmFixture(t, "ec:secp256r1")
	configureFixture(t, f, "ec:secp256r1", "ES256", true)
	data := f.encrypt(t)
	for _, mode := range []string{"session-missing", "session-empty", "session-malformed", "session-type", "session-private", "session-curve", "session-tamper", "session-json-type", "frame-short", "frame-long", "nonce-tamper", "tag-tamper", "recovered-length", "denied", "obligation", "malformed-obligation", "both", "policy", "id", "status", "duplicate", "missing"} {
		t.Run(mode, func(t *testing.T) {
			f.mode.Store(mode)
			d, e := f.client.Decrypt(context.Background(), data)
			if e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 {
				t.Fatal("failed open", e)
			}
			if mode == "obligation" {
				var v *Error
				if !errors.As(e, &v) || len(v.RequiredObligations) != 1 {
					t.Fatal("obligations lost")
				}
			}
		})
	}
	f.mode.Store("")
	a, e := tdf.ReadArchive(data, tdf.DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	a.Payload[len(a.Payload)-1] ^= 1
	data, e = tdf.WriteArchive(a.Payload, a.Manifest, tdf.DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	if d, e := f.client.Decrypt(context.Background(), data); e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 {
		t.Fatal("payload tamper failed open")
	}
}
func TestWrappingConfigurationAndDiscoveryRejectMismatch(t *testing.T) {
	for _, wrapping := range []string{"rsa:2048", "ec:secp256r1"} {
		t.Run(wrapping, func(t *testing.T) {
			f := newAlgorithmFixture(t, wrapping)
			wrong := "ec:secp256r1"
			if wrapping == wrong {
				wrong = "rsa:2048"
			}
			config := f.client.config
			config.KASPublicKey = f.public
			config.KASAlgorithm = wrong
			if c, e := New(config); e == nil {
				c.Close()
				t.Fatal("explicit wrong type accepted")
			}
			for _, mode := range []string{"discovery-mismatch", "discovery-private", "discovery-curve"} {
				f.mode.Store(mode)
				if key, _, e := f.client.PublicKey(context.Background()); e == nil {
					key.Close()
					t.Fatal("discovery accepted", mode)
				}
			}
			f.mode.Store("")
			for _, options := range []tdf.EncryptConfig{{Algorithm: wrong}, {Algorithm: "unsupported"}, {KASPublicKey: f.public}, {KID: "other"}, {KASURL: f.server.URL + "/other"}} {
				if data, e := f.client.Create(context.Background(), nil, options); e == nil || len(data) != 0 {
					t.Fatal("conflicting option accepted")
				}
			}
		})
	}
	for _, config := range []Config{{KASAlgorithm: "ec:P384"}, {SessionAlgorithm: "ES256"}, {KID: "without-key"}, {DPoP: true}} {
		if c, e := New(config); e == nil {
			c.Close()
			t.Fatal("unsupported config")
		}
	}
	v, e := j.Parse([]byte(`{}`), jsonLimits())
	if e != nil {
		t.Fatal(e)
	}
	if _, e := parseRewrap(v, "unknown"); e == nil {
		t.Fatal("unknown session")
	}
	f := newFixture(t)
	f.mode.Store("rsa-session-empty")
	if _, e := f.client.Decrypt(context.Background(), f.encrypt(t)); e != nil {
		t.Fatal("protobuf empty RSA sessionPublicKey regression", e)
	}
	f.mode.Store("recovered-length")
	if d, e := f.client.Decrypt(context.Background(), f.encrypt(t)); e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 {
		t.Fatal("RSA recovered wrong share length accepted")
	}
	for _, mode := range []string{"session-json-type", "session-type", "session-malformed", "session-private"} {
		f.mode.Store(mode)
		if d, e := f.client.Decrypt(context.Background(), f.encrypt(t)); e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 {
			t.Fatal("RSA invalid session key accepted", mode)
		}
	}
}
func TestECConcurrentCancellationAndKeyLifetime(t *testing.T) {
	f := newAlgorithmFixture(t, "ec:secp256r1")
	configureFixture(t, f, "ec:secp256r1", "ES256", true)
	data := f.encrypt(t)
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	if d, e := f.client.Decrypt(ctx, data); !errors.Is(e, context.Canceled) || len(d.Payload) != 0 || len(d.Metadata) != 0 {
		t.Fatal("cancellation", e)
	}
	var wg sync.WaitGroup
	for i := 0; i < 8; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			if _, e := f.client.Decrypt(context.Background(), data); e != nil {
				t.Error(e)
			}
		}()
	}
	wg.Wait()
	key, e := capcrypto.GenerateP256()
	if e != nil {
		t.Fatal(e)
	}
	defer key.Close()
	f.client.Close()
	config := f.client.config
	config.AuthKey = key
	c, e := New(config)
	if e != nil {
		t.Fatal(e)
	}
	c.Close()
	if _, e := key.PublicPEM(); e != nil {
		t.Fatal("caller auth key closed", e)
	}
	if _, e := f.public.PublicPEM(); e != nil {
		t.Fatal("caller KAS key closed", e)
	}
	validData := f.encrypt(t)
	c, e = New(config)
	if e != nil {
		t.Fatal(e)
	}
	defer c.Close()
	f.public.Close()
	if data, e := c.Create(context.Background(), nil, tdf.EncryptConfig{}); e == nil || len(data) != 0 {
		t.Fatal("closed handles accepted")
	}
	key.Close()
	if d, e := c.Decrypt(context.Background(), validData); e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 {
		t.Fatal("closed caller auth key accepted")
	}
}
