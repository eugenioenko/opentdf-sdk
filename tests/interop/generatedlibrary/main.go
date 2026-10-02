// Independent stdlib-only native consumer of the actual generated package.
// This module imports no shared SDK or compiler code.
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
		var sdk *g.Failure
		var boundary *g.LibraryError
		if errors.As(err, &sdk) {
			panic(label + ": " + sdk.Error() + " cause=" + sdk.CauseCategory)
		}
		if errors.As(err, &boundary) {
			panic(label + ": " + boundary.Error())
		}
		panic(label + ": native failure")
	}
}
func read(path string) []byte     { b, e := os.ReadFile(path); fail("read", e); return b }
func write(path string, b []byte) { fail("write", os.WriteFile(path, b, 0600)) }
func main() {
	if len(os.Args) != 5 {
		panic("usage: consumer sdk-root run-dir mode case")
	}
	sdk, run, mode, name := os.Args[1], os.Args[2], os.Args[3], os.Args[4]
	var cfg g.Config
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
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	switch mode {
	case "controlled":
		controlledNegatives(run)
	case "encrypt":
		options := g.EncryptOptions{Attributes: []string{"https://example.com/attr/attr1/value/value1"}, SegmentSize: 16384, HasSegmentSize: true}
		if (name == "metadata" || strings.HasSuffix(name, "-metadata")) && name != "empty-metadata" && !strings.HasSuffix(name, "-empty-metadata") {
			options.Metadata = []byte(`{"source":"independent metadata","count":7}`)
			options.IncludeMetadata = true
		}
		if name == "empty-metadata" || strings.HasSuffix(name, "-empty-metadata") {
			options.Metadata = []byte{}
			options.IncludeMetadata = true
		}
		if name == "hs256" || strings.HasSuffix(name, "-hs256") {
			options.SegmentHashAlgorithm = "HS256"
		}
		if name == "denied" || strings.HasSuffix(name, "-denied") {
			options.Attributes = []string{"https://example.com/attr/attr1/value/value2"}
		}
		data, e := g.Encrypt(ctx, cfg, read(filepath.Join(run, name+".input")), options, callbacks)
		fail("encrypt", e)
		write(filepath.Join(run, name+".generated.tdf"), data)
	case "decrypt":
		result, e := g.Decrypt(ctx, cfg, read(filepath.Join(run, name+".tdf")), callbacks)
		fail("decrypt", e)
		write(filepath.Join(run, name+".out"), result.Payload)
		write(filepath.Join(run, name+".metadata"), result.Metadata)
		write(filepath.Join(run, name+".manifest"), result.ManifestJSON)
		write(filepath.Join(run, name+".presence"), []byte(strconv.FormatBool(result.HasMetadata)))
	case "repeat":
		original := []byte{0, 255, 128, 1}
		input := append([]byte(nil), original...)
		options := g.EncryptOptions{Attributes: []string{"https://example.com/attr/attr1/value/value1"}, Metadata: []byte{0, 255}, IncludeMetadata: true}
		bad := cfg
		bad.ClientSecret = ""
		if _, e := g.Encrypt(ctx, bad, input, options); e == nil {
			panic("invalid constructor accepted")
		}
		archive, e := g.Encrypt(ctx, cfg, input, options, callbacks)
		fail("repeat create", e)
		if !bytes.Equal(input, original) {
			panic("input mutated")
		}
		first, e := g.Decrypt(ctx, cfg, archive, callbacks)
		fail("repeat first", e)
		saved := append([]byte(nil), first.Payload...)
		metadata := append([]byte(nil), first.Metadata...)
		options.Metadata[0] = 1
		options.Attributes[0] = "https://example.com/attr/attr1/value/value2"
		second, e := g.Decrypt(ctx, cfg, archive, callbacks)
		fail("repeat second", e)
		if !bytes.Equal(first.Payload, saved) || !bytes.Equal(second.Payload, original) || !bytes.Equal(first.Metadata, metadata) || !bytes.Equal(second.Metadata, []byte{0, 255}) {
			panic("results/config isolation")
		}
		second.Payload[0] = 1
		if first.Payload[0] != 0 {
			panic("result alias")
		}
		providerCfg := cfg
		providerCfg.ClientID = ""
		providerCfg.ClientSecret = ""
		providerCfg.TokenProviderName = g.TokenProviderName
		started := make(chan struct{})
		cancelCtx, stop := context.WithCancel(context.Background())
		finished := make(chan error, 1)
		waiting := g.TokenCallbacks(func(ctx context.Context) (g.AccessToken, error) {
			close(started)
			<-ctx.Done()
			return g.AccessToken{}, ctx.Err()
		})
		go func() {
			data, e := g.Encrypt(cancelCtx, providerCfg, input, g.EncryptOptions{}, waiting)
			if data != nil {
				panic("canceled plaintext result")
			}
			finished <- e
		}()
		<-started
		stop()
		if !errors.Is(<-finished, context.Canceled) {
			panic("SDK active provider cancellation")
		}
		_, e = g.Decrypt(ctx, cfg, archive, callbacks)
		fail("after canceled call", e)
	case "negative":
		result, e := g.Decrypt(ctx, cfg, read(filepath.Join(run, name+".tdf")), callbacks)
		if e == nil || result.Payload != nil || result.Metadata != nil || result.ManifestJSON != nil {
			panic("negative returned successful output")
		}
		var failure *g.Failure
		var boundary *g.LibraryError
		report := map[string]any{}
		if errors.As(e, &failure) {
			report["code"] = failure.Code
			report["operation"] = failure.Operation
			report["httpStatus"] = failure.HTTPStatus
			report["causeCategory"] = failure.CauseCategory
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
