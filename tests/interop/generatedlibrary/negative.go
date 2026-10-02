package main

import (
	"context"
	"encoding/json"
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

// These endpoints only reject requests. Positive TDF/KAS interoperability is
// established separately against the real service, never by these fixtures.
func controlledNegatives(run string) {
	archive := read(filepath.Join(run, "binary.generated.tdf"))
	callbacks := g.TokenCallbacks(func(context.Context) (g.AccessToken, error) {
		return g.AccessToken{Value: "negative-fixture-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}, nil
	})
	var rows []map[string]any
	for _, name := range []string{"http401", "http403", "redirect", "content-type", "malformed-response", "oversized-response", "untrusted-tls", "caller-deadline", "source-deadline", "active-http-cancel"} {
		var requests, redirected atomic.Int32
		entered, released := make(chan struct{}, 1), make(chan struct{}, 1)
		handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			requests.Add(1)
			io.Copy(io.Discard, r.Body)
			r.Body.Close()
			if r.URL.Path == "/leak" {
				redirected.Add(1)
			}
			w.Header().Set("Content-Type", "application/json")
			switch name {
			case "http401", "http403":
				status := 401
				if name == "http403" {
					status = 403
				}
				w.WriteHeader(status)
				io.WriteString(w, `{"code":"fixture-rejected","message":"harmless fixture diagnostic"}`)
			case "redirect":
				w.Header().Set("Location", "/leak")
				w.WriteHeader(307)
			case "content-type":
				w.Header().Set("Content-Type", "text/html")
				io.WriteString(w, `{}`)
			case "malformed-response":
				io.WriteString(w, `{bad`)
			case "oversized-response":
				io.WriteString(w, strings.Repeat(" ", (1<<20)+1))
			case "caller-deadline", "source-deadline", "active-http-cancel":
				entered <- struct{}{}
				<-r.Context().Done()
				released <- struct{}{}
			default:
				panic("TLS verification should reject before HTTP")
			}
		})
		server := httptest.NewUnstartedServer(handler)
		server.Config.ErrorLog = log.New(io.Discard, "", 0)
		if name == "untrusted-tls" {
			server.StartTLS()
		} else {
			server.Start()
		}
		cfg := g.Config{PlatformURL: server.URL, KASURL: "http://localhost:8080/kas", AllowedKAS: []g.KASRoute{{URL: "http://localhost:8080/kas", APIBaseURL: server.URL}}, AllowHTTP: true, TokenProviderName: g.TokenProviderName}
		ctx, cancel := context.WithCancel(context.Background())
		if name == "caller-deadline" {
			cancel()
			ctx, cancel = context.WithTimeout(context.Background(), 100*time.Millisecond)
		}
		if name == "source-deadline" {
			cfg.TimeoutMillis = 100
		}
		if name == "active-http-cancel" {
			go func() { <-entered; cancel() }()
		}
		start := time.Now()
		result, err := g.Decrypt(ctx, cfg, archive, callbacks)
		cancel()
		elapsed := time.Since(start)
		if err == nil || result.Payload != nil || result.Metadata != nil || result.ManifestJSON != nil {
			panic("controlled negative successful output: " + name)
		}
		row := map[string]any{"case": name, "requests": requests.Load(), "zero_output": true}
		var failure *g.Failure
		var boundary *g.LibraryError
		if errors.As(err, &failure) {
			row["code"], row["operation"], row["httpStatus"], row["causeCategory"] = failure.Code, failure.Operation, failure.HTTPStatus, failure.CauseCategory
			want := map[string]string{"http401": "unauthenticated", "http403": "denied", "redirect": "http_status", "content-type": "invalid_content_type", "malformed-response": "invalid_json", "oversized-response": "transport", "untrusted-tls": "transport", "source-deadline": "transport"}[name]
			if failure.Code != want {
				panic("controlled wrong source category: " + name + ":" + failure.Code)
			}
			if name == "http401" || name == "http403" {
				if failure.ServerCode != "fixture-rejected" || failure.ServerMessage != "harmless fixture diagnostic" {
					panic("service fields dropped")
				}
			}
			if name == "source-deadline" && failure.CauseCategory != "deadline_exceeded" {
				panic("source deadline category dropped")
			}
		} else if errors.As(err, &boundary) {
			row["kind"] = boundary.Kind
			if name != "caller-deadline" && name != "active-http-cancel" {
				panic("unexpected boundary fault")
			}
			want := context.Canceled
			if name == "caller-deadline" {
				want = context.DeadlineExceeded
			}
			if boundary.Kind != "canceled" || !errors.Is(err, want) {
				panic("native cancellation identity dropped")
			}
		} else {
			panic("untyped controlled failure")
		}
		if name == "caller-deadline" || name == "source-deadline" || name == "active-http-cancel" {
			if elapsed > 2*time.Second {
				panic("deadline/cancel delayed")
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
		if redirected.Load() != 0 {
			panic("redirect followed")
		}
		server.Close()
		rows = append(rows, row)
	}
	encoded, err := json.MarshalIndent(map[string]any{"scope": "controlled rejecting endpoints through actual generated exports; no mock positive KAS", "negatives": rows}, "", "  ")
	fail("controlled report", err)
	write(filepath.Join(run, "controlled-negatives.json"), encoded)
}
