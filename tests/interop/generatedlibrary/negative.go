package main

import (
	"context"
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/json"
	"encoding/pem"
	"errors"
	"io"
	"log"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"sync/atomic"
	"time"

	g "goalchemyout"
)

// A platform that only discovers and rejects: no KAS. Positive TDF/KAS
// interoperability is established separately against the real service.
func fakePlatform(mode string, requests *atomic.Int32, entered, released chan struct{}) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		requests.Add(1)
		io.Copy(io.Discard, r.Body)
		r.Body.Close()
		w.Header().Set("Content-Type", "application/json")
		issuer := "http://" + r.Host + "/issuer"
		switch {
		case mode == "wellknown-500" && r.URL.Path == "/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration":
			w.WriteHeader(500)
			io.WriteString(w, `{"code":"internal","message":"harmless fixture diagnostic"}`)
		case mode == "redirect" && r.URL.Path == "/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration":
			w.Header().Set("Location", "/leak")
			w.WriteHeader(307)
		case mode == "content-type" && r.URL.Path == "/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration":
			w.Header().Set("Content-Type", "text/html")
			io.WriteString(w, `{}`)
		case mode == "malformed-response" && r.URL.Path == "/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration":
			io.WriteString(w, `{bad`)
		case (mode == "caller-deadline" || mode == "source-deadline") && r.URL.Path == "/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration":
			entered <- struct{}{}
			<-r.Context().Done()
			released <- struct{}{}
		case r.URL.Path == "/wellknownconfiguration.WellKnownService/GetWellKnownConfiguration":
			json.NewEncoder(w).Encode(map[string]any{"configuration": map[string]any{"platform_issuer": issuer, "idp": map[string]any{"issuer": issuer, "token_endpoint": issuer + "/protocol/openid-connect/token"}}})
		case r.URL.Path == "/issuer/.well-known/openid-configuration":
			json.NewEncoder(w).Encode(map[string]any{"token_endpoint": issuer + "/protocol/openid-connect/token"})
		case r.URL.Path == "/issuer/protocol/openid-connect/token":
			w.WriteHeader(401)
			io.WriteString(w, `{"error":"invalid_client","error_description":"harmless fixture diagnostic"}`)
		case r.URL.Path == "/grpc.health.v1.Health/Check":
			io.WriteString(w, `{"status":"SERVING"}`)
		case r.URL.Path == "/policy.kasregistry.KeyAccessServerRegistryService/ListKeyAccessServers":
			io.WriteString(w, `{"keyAccessServers":[]}`)
		default:
			w.WriteHeader(404)
			io.WriteString(w, `{"code":"unimplemented","message":"`+r.URL.Path+`"}`)
		}
	}
}

func offlineKASKey() (string, *rsa.PrivateKey) {
	key, err := rsa.GenerateKey(rand.Reader, 2048)
	fail("kas key", err)
	der, err := x509.MarshalPKIXPublicKey(&key.PublicKey)
	fail("kas key der", err)
	return string(pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: der})), key
}

func kind(err error) (string, map[string]any) {
	var failure *g.Error
	var boundary *g.LibraryError
	if errors.As(err, &failure) {
		return string(failure.Kind), map[string]any{"kind": failure.Kind, "op": failure.Op, "httpStatus": failure.HTTPStatus, "connectCode": failure.ConnectCode, "message": failure.Message}
	}
	if errors.As(err, &boundary) {
		return "boundary:" + boundary.Kind, map[string]any{"boundary": boundary.Kind}
	}
	return "untyped", map[string]any{"error": err.Error()}
}

func controlledNegatives(run string) {
	var rows []map[string]any
	ctx := context.Background()
	kasPEM, _ := offlineKASKey()
	closed := httptest.NewServer(http.NotFoundHandler())
	closedURL := closed.URL
	closed.Close()

	// A TDF written offline against an explicit KAS key: it needs no platform
	// to create, so the reader's allowlist, option and input checks can be
	// driven without a KAS.
	plain := httptest.NewServer(fakePlatform("", new(atomic.Int32), nil, nil))
	sdk, err := g.NewWithOptions(ctx, plain.URL, g.WithClientCredentials("opentdf", "secret", nil))
	fail("offline sdk", err)
	tdfOpts, err := g.TDFOptionsOf(g.WithAutoconfigure(false), g.WithKasInformation(g.KASInfo{URL: "http://localhost:9999/malicious/kas", PublicKey: kasPEM, KID: "k1", Algorithm: "rsa:2048"}), g.WithDataAttributes("https://example.com/attr/a/value/b"))
	fail("tdf options", err)
	archive, obj, err := sdk.EncryptBytes(ctx, []byte("offline payload"), tdfOpts)
	fail("offline encrypt", err)
	fail("object close", obj.Close())

	check := func(name string, err error, want string, extra map[string]any) {
		got, row := kind(err)
		if got != want {
			panic("controlled wrong kind: " + name + ": " + got + " (" + err.Error() + ")")
		}
		row["case"] = name
		for k, v := range extra {
			row[k] = v
		}
		rows = append(rows, row)
	}

	_, err = g.NewWithOptions(ctx, closedURL, g.WithConnectionValidation())
	if !errors.Is(err, g.ErrPlatformUnreachable) {
		panic("unreachable platform sentinel")
	}
	check("unreachable", err, "transport", nil)

	for _, name := range []string{"wellknown-500", "redirect", "content-type", "malformed-response", "untrusted-tls", "caller-deadline", "source-deadline", "token-401"} {
		var requests atomic.Int32
		entered, released := make(chan struct{}, 1), make(chan struct{}, 1)
		server := httptest.NewUnstartedServer(fakePlatform(name, &requests, entered, released))
		server.Config.ErrorLog = log.New(io.Discard, "", 0)
		if name == "untrusted-tls" {
			server.StartTLS()
		} else {
			server.Start()
		}
		callCtx, cancel := context.WithCancel(ctx)
		opts := []g.Option{g.WithClientCredentials("opentdf", "secret", nil)}
		if name == "caller-deadline" {
			cancel()
			callCtx, cancel = context.WithTimeout(ctx, 100*time.Millisecond)
		}
		if name == "source-deadline" {
			opts = append(opts, g.WithTimeout(100))
		}
		start := time.Now()
		s, err := g.NewWithOptions(callCtx, server.URL, opts...)
		if name == "token-401" {
			fail("token-401 new", err)
			_, err = s.DecryptBytes(ctx, archive, g.ReaderOptions{IgnoreAllowlist: true})
			s.Close()
		}
		elapsed := time.Since(start)
		if err == nil {
			panic("controlled negative succeeded: " + name)
		}
		want := map[string]string{"wellknown-500": "transport", "redirect": "transport", "content-type": "transport", "malformed-response": "transport", "untrusted-tls": "transport", "caller-deadline": "boundary:canceled", "source-deadline": "deadline_exceeded", "token-401": "auth"}[name]
		check(name, err, want, map[string]any{"requests": requests.Load(), "elapsed_ms": elapsed.Milliseconds()})
		if name != "caller-deadline" && name != "token-401" && !errors.Is(err, g.ErrPlatformConfigFailed) && name != "untrusted-tls" {
			panic("discovery failures carry ErrPlatformConfigFailed: " + name)
		}
		if name == "token-401" {
			var e *g.Error
			if !errors.As(err, &e) || e.HTTPStatus != 401 || e.ConnectCode != "invalid_client" || !strings.Contains(e.Message, "tdf: decrypt failed") {
				panic("token failure fields")
			}
		}
		if name == "caller-deadline" || name == "source-deadline" {
			if elapsed > 2*time.Second {
				panic("deadline delayed")
			}
			select {
			case <-released:
			case <-time.After(time.Second):
				panic("HTTP request cleanup not observed: " + name)
			}
		}
		if name == "untrusted-tls" && requests.Load() != 0 {
			panic("untrusted TLS reached handler")
		}
		cancel()
		server.Close()
	}

	_, err = sdk.DecryptBytes(ctx, archive, g.ReaderOptions{KasAllowlist: []string{plain.URL}})
	if !errors.Is(err, g.ErrTDFDecryptFailed) {
		panic("allowlist rejection is a decrypt failure")
	}
	check("allowlist", err, "allowlist_rejected", nil)
	_, err = sdk.DecryptBytes(ctx, []byte("not a tdf"), g.ReaderOptions{})
	if !errors.Is(err, g.ErrTDFNotDecryptable) {
		panic("invalid input sentinel")
	}
	check("invalid-tdf", err, "not_decryptable", nil)
	badOpts, _ := g.TDFOptionsOf(g.WithTargetMode("4.2.2"))
	_, _, err = sdk.EncryptBytes(ctx, []byte("x"), badOpts)
	check("unsupported-option", err, "unsupported_feature", nil)

	// 100 reader handles on one SDK, every one closed; then the SDK.
	for i := 0; i < 100; i++ {
		reader, err := sdk.LoadTDF(ctx, &readSeeker{data: archive}, g.ReaderOptions{IgnoreAllowlist: true})
		fail("load", err)
		manifest, err := reader.Manifest(ctx)
		fail("manifest", err)
		if manifest.EncryptionInformation.KeyAccessObjs[0].KID != "k1" {
			panic("manifest")
		}
		fail("reader close", reader.Close())
	}
	fail("sdk close", sdk.Close())
	_, err = sdk.DecryptBytes(ctx, archive, g.ReaderOptions{})
	if err == nil {
		panic("closed sdk")
	}
	check("closed-handle", err, "boundary:closed", nil)
	rows = append(rows, map[string]any{"case": "handles", "readers": 100, "closed": true})
	plain.Close()

	encoded, err := json.MarshalIndent(map[string]any{"scope": "controlled discovery, transport, allowlist, option and handle checks through actual generated exports; no mock positive KAS", "negatives": rows}, "", "  ")
	fail("controlled report", err)
	write(filepath.Join(run, "controlled-negatives.json"), encoded)
}

type readSeeker struct {
	data []byte
	pos  int64
}

func (r *readSeeker) Read(p []byte) (int, error) {
	if r.pos >= int64(len(r.data)) {
		return 0, io.EOF
	}
	n := copy(p, r.data[r.pos:])
	r.pos += int64(n)
	return n, nil
}

func (r *readSeeker) Seek(offset int64, whence int) (int64, error) {
	switch whence {
	case io.SeekStart:
		r.pos = offset
	case io.SeekCurrent:
		r.pos += offset
	case io.SeekEnd:
		r.pos = int64(len(r.data)) + offset
	}
	return r.pos, nil
}
