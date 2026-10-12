// Independent stdlib-only native consumer of the actual generated package.
// This module imports no shared SDK or compiler code. It drives the handle
// API: one SDK per run, EncryptBytes/DecryptBytes, LoadTDF readers.
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	g "goalchemyout"
)

func fail(label string, err error) {
	if err != nil {
		var sdk *g.Error
		var boundary *g.LibraryError
		if errors.As(err, &sdk) {
			panic(label + ": " + sdk.Error() + " kind=" + string(sdk.Kind))
		}
		if errors.As(err, &boundary) {
			panic(label + ": " + boundary.Error())
		}
		panic(label + ": native failure: " + err.Error())
	}
}
func read(path string) []byte     { b, e := os.ReadFile(path); fail("read", e); return b }
func write(path string, b []byte) { fail("write", os.WriteFile(path, b, 0600)) }

// consumerConfig is the run's config.json: the façade Config fields the
// delivery scripts write, mapped onto the handle API's options.
type consumerConfig = g.Config

func optionsOf(cfg consumerConfig) []g.Option {
	opts := []g.Option{g.WithTokenEndpoint(cfg.TokenURL)}
	if cfg.ClientID != "" {
		opts = append(opts, g.WithClientCredentials(cfg.ClientID, cfg.ClientSecret, nil))
	}
	if cfg.TokenProviderName != "" {
		opts = append(opts, g.WithTokenProvider())
	}
	if cfg.AllowHTTP {
		opts = append(opts, g.WithInsecurePlaintextConn())
	}
	if cfg.TimeoutMillis > 0 {
		opts = append(opts, g.WithTimeout(cfg.TimeoutMillis))
	}
	if cfg.DPoP {
		alg := g.SigningAlgorithm(cfg.AuthAlgorithm)
		if alg == "" {
			alg = "ES256"
		}
		opts = append(opts, g.WithDPoPAlgorithm(alg))
	}
	if cfg.AuthPrivateKeyPEM != "" {
		opts = append(opts, g.WithDPoPKeyPEM([]byte(cfg.AuthPrivateKeyPEM)))
	}
	return opts
}

func kasInformation(cfg consumerConfig) g.KASInfo {
	kasURL := cfg.KASURL
	if kasURL == "" {
		kasURL = strings.TrimSuffix(cfg.PlatformURL, "/") + "/kas"
	}
	return g.KASInfo{URL: kasURL, PublicKey: cfg.KASPublicKeyPEM, KID: cfg.KID, Algorithm: cfg.KASAlgorithm}
}

func tdfOptions(cfg consumerConfig, extra ...g.TDFOption) g.TDFOptions {
	opts := []g.TDFOption{g.WithAutoconfigure(false), g.WithKasInformation(kasInformation(cfg)), g.WithDataAttributes("https://example.com/attr/attr1/value/value1"), g.WithSegmentSize(16384)}
	if cfg.KASAlgorithm != "" {
		opts = append(opts, g.WithWrappingKeyAlg(g.KeyType(cfg.KASAlgorithm)))
	}
	o, err := g.TDFOptionsOf(append(opts, extra...)...)
	fail("tdf options", err)
	return o
}

func readerOptions(cfg consumerConfig) g.ReaderOptions {
	allow := []string{cfg.PlatformURL}
	if cfg.KASURL != "" {
		allow = append(allow, cfg.KASURL)
	}
	for _, r := range cfg.AllowedKAS {
		allow = append(allow, r.URL)
	}
	opts := []g.TDFReaderOption{g.WithKasAllowlist(allow)}
	if cfg.SessionAlgorithm != "" {
		opts = append(opts, g.WithSessionKeyType(g.KeyType(cfg.SessionAlgorithm)))
	}
	o, err := g.ReaderOptionsOf(opts...)
	fail("reader options", err)
	return o
}

func main() {
	if len(os.Args) != 5 {
		panic("usage: consumer sdk-root run-dir mode case")
	}
	sdk, run, mode, name := os.Args[1], os.Args[2], os.Args[3], os.Args[4]
	if mode == "controlled" {
		controlledNegatives(run)
		fmt.Println("PASS generated native library", mode, name)
		return
	}
	var cfg consumerConfig
	fail("config", json.Unmarshal(read(filepath.Join(run, "config.json")), &cfg))
	callbacks := g.Callbacks{}
	if cfg.TokenProviderName != "" {
		if cfg.TokenProviderName != g.TokenProviderName {
			panic("unsupported test provider name")
		}
		callbacks = g.TokenCallbacks(func(ctx context.Context) (g.AccessToken, error) {
			if cfg.DPoP {
				return dpopProvider(ctx, cfg, name == "mismatched-provider")
			}
			if name == "invalid-token" {
				return g.AccessToken{Value: "invalid-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}, nil
			}
			if name == "expired-token" {
				return g.AccessToken{Value: "expired-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() - 1}, nil
			}
			if name == "provider-reject" {
				return g.AccessToken{}, errors.New("provider rejected")
			}
			form := url.Values{"grant_type": {"client_credentials"}, "client_id": {"opentdf-sdk"}, "client_secret": {"secret"}}
			request, e := http.NewRequestWithContext(ctx, "POST", "http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token", bytes.NewBufferString(form.Encode()))
			if e != nil {
				return g.AccessToken{}, e
			}
			request.Header.Set("Content-Type", "application/x-www-form-urlencoded")
			response, e := (&http.Client{Timeout: 10 * time.Second}).Do(request)
			if e != nil {
				return g.AccessToken{}, e
			}
			body, e := io.ReadAll(io.LimitReader(response.Body, 128*1024))
			response.Body.Close()
			if e != nil {
				return g.AccessToken{}, e
			}
			if response.StatusCode != 200 {
				return g.AccessToken{}, errors.New("provider endpoint")
			}
			var token struct {
				Value   string `json:"access_token"`
				Scheme  string `json:"token_type"`
				Expires int64  `json:"expires_in"`
			}
			if e = json.Unmarshal(body, &token); e != nil {
				return g.AccessToken{}, e
			}
			return g.AccessToken{Value: token.Value, Scheme: "Bearer", ExpiresAt: time.Now().Unix() + token.Expires}, nil
		})
	}
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()
	client, e := g.New(ctx, cfg.PlatformURL, g.OptionsOf(optionsOf(cfg)...), callbacks)
	fail("new", e)
	defer client.Close()
	switch mode {
	case "encrypt":
		var extra []g.TDFOption
		if (name == "metadata" || strings.HasSuffix(name, "-metadata")) && name != "empty-metadata" && !strings.HasSuffix(name, "-empty-metadata") {
			extra = append(extra, g.WithMetaData(`{"source":"independent metadata","count":7}`))
		}
		if name == "denied" || strings.HasSuffix(name, "-denied") {
			extra = append(extra, g.WithDataAttributes("https://example.com/attr/attr1/value/value2"))
		}
		data, _, e := client.EncryptBytes(ctx, read(filepath.Join(run, name+".input")), tdfOptions(cfg, extra...), callbacks)
		fail("encrypt", e)
		write(filepath.Join(run, name+".generated.tdf"), data)
	case "decrypt":
		input := read(filepath.Join(run, name+".tdf"))
		reader, e := client.LoadTDF(ctx, bytes.NewReader(input), readerOptions(cfg), callbacks)
		fail("load", e)
		manifest, e := reader.Manifest(ctx)
		fail("manifest", e)
		manifestJSON, e := json.Marshal(manifest)
		fail("manifest json", e)
		hasMetadata := len(manifest.EncryptionInformation.KeyAccessObjs) > 0 && manifest.EncryptionInformation.KeyAccessObjs[0].EncryptedMetadata != ""
		var metadata []byte
		if hasMetadata {
			metadata, e = reader.UnencryptedMetadata(ctx, callbacks)
			fail("metadata", e)
		}
		fail("reader close", reader.Close())
		payload, e := client.DecryptBytes(ctx, input, readerOptions(cfg), callbacks)
		fail("decrypt", e)
		write(filepath.Join(run, name+".out"), payload)
		write(filepath.Join(run, name+".metadata"), metadata)
		write(filepath.Join(run, name+".manifest"), manifestJSON)
		write(filepath.Join(run, name+".presence"), []byte(strconv.FormatBool(hasMetadata)))
	case "repeat":
		repeat(ctx, client, cfg, callbacks)
	case "negative":
		payload, e := client.DecryptBytes(ctx, read(filepath.Join(run, name+".tdf")), readerOptions(cfg), callbacks)
		if e == nil || payload != nil {
			panic("negative returned successful output")
		}
		var failure *g.Error
		var boundary *g.LibraryError
		report := map[string]any{}
		if errors.As(e, &failure) {
			report["kind"] = failure.Kind
			report["op"] = failure.Op
			report["httpStatus"] = failure.HTTPStatus
			report["message"] = failure.Message
		} else if errors.As(e, &boundary) {
			report["kind"] = boundary.Kind
		} else {
			panic("untyped error")
		}
		reportBytes, e := json.Marshal(report)
		fail("report", e)
		write(filepath.Join(run, name+".error.json"), reportBytes)
	default:
		panic("unknown mode")
	}
	_ = sdk
	fmt.Println("PASS generated native library", mode, name)
}

// repeat runs 100 encrypt and decrypt calls on one SDK, keeps the inputs and
// results independent, closes every reader handle, then closes the SDK and
// checks that later calls fail.
func repeat(ctx context.Context, client *g.SDK, cfg consumerConfig, callbacks g.Callbacks) {
	original := []byte{0, 255, 128, 1}
	for i := 0; i < 100; i++ {
		input := append(append([]byte(nil), original...), byte(i))
		opts := tdfOptions(cfg, g.WithMetaData(fmt.Sprintf("round %d", i)))
		archive, obj, e := client.EncryptBytes(ctx, input, opts, callbacks)
		fail("repeat create", e)
		size, e := obj.Size(ctx)
		fail("repeat size", e)
		if size != int64(len(archive)) {
			panic("size")
		}
		fail("object close", obj.Close())
		reader, e := client.LoadTDF(ctx, bytes.NewReader(archive), readerOptions(cfg), callbacks)
		fail("repeat load", e)
		attrs, e := reader.DataAttributes(ctx)
		fail("repeat attributes", e)
		if len(attrs) != 1 {
			panic("attributes")
		}
		fail("repeat reader close", reader.Close())
		plain, e := client.DecryptBytes(ctx, archive, readerOptions(cfg), callbacks)
		fail("repeat decrypt", e)
		if !bytes.Equal(plain, input) {
			panic("round trip")
		}
		plain[0] = 1
		if input[0] != 0 {
			panic("result alias")
		}
	}
	fail("close", client.Close())
	if _, e := client.DecryptBytes(ctx, original, readerOptions(cfg), callbacks); e == nil {
		panic("closed handle accepted a call")
	}
}
