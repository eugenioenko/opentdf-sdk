// Separate diagnostic profiler. These instrumented loops never become table samples.
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"runtime/pprof"
	"strconv"
	"time"

	"github.com/opentdf/platform/lib/ocrypto"
	r "github.com/opentdf/platform/sdk"
	g "goalchemyout"
	"golang.org/x/oauth2"
)

type configuration struct {
	Config  g.Config
	Token   string
	Expires int64
}

type phases struct {
	FullMS      float64 `json:"full_call_ms"`
	ConfigMS    float64 `json:"host_configuration_ms"`
	SetupMS     float64 `json:"original_client_setup_ms"`
	OperationMS float64 `json:"public_operation_ms"`
	CloseMS     float64 `json:"original_close_ms"`
}

func require(err error) {
	if err != nil {
		panic(err)
	}
}

func read(path string) []byte {
	value, err := os.ReadFile(path)
	require(err)
	return value
}

func milliseconds(start time.Time) float64 {
	return float64(time.Since(start).Nanoseconds()) / 1e6
}

func operation(raw configuration, input, archive []byte, target, op string) ([]byte, phases) {
	var result []byte
	var report phases
	full := time.Now()
	// Configuration/provider creation remains within the full public lifecycle.
	cfg := raw.Config
	var err error
	if target == "reference" {
		opts := []r.Option{
			r.WithOAuthAccessTokenSource(oauth2.StaticTokenSource(&oauth2.Token{
				AccessToken: raw.Token, TokenType: "Bearer", Expiry: time.Unix(raw.Expires, 0),
			})),
			r.WithPlatformConfiguration(r.PlatformConfiguration{}),
			r.WithTokenEndpoint("http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token"),
			r.WithInsecurePlaintextConn(),
			r.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil))),
		}
		report.ConfigMS = milliseconds(full)
		var client *r.SDK
		setup := time.Now()
		pprof.Do(context.Background(), pprof.Labels("target", target, "operation", op, "phase", "client_setup"), func(context.Context) {
			client, err = r.New("http://localhost:8080", opts...)
		})
		report.SetupMS = milliseconds(setup)
		require(err)
		start := time.Now()
		pprof.Do(context.Background(), pprof.Labels("target", target, "operation", op, "phase", "public_operation"), func(context.Context) {
			if op == "encrypt" {
				var output bytes.Buffer
				_, err = client.CreateTDF(&output, bytes.NewReader(input),
					r.WithAutoconfigure(false),
					r.WithKasInformation(r.KASInfo{URL: "http://localhost:8080/kas", Algorithm: "rsa:2048", PublicKey: cfg.KASPublicKeyPEM, KID: cfg.KID}),
					r.WithWrappingKeyAlg(ocrypto.RSA2048Key), r.WithSegmentSize(2<<20),
					r.WithDataAttributes("https://example.com/attr/attr1/value/value1"))
				result = output.Bytes()
			} else {
				var reader *r.Reader
				// Reuse the RSA2048 response-session key created by New.
				reader, err = client.LoadTDF(bytes.NewReader(archive), r.WithKasAllowlist([]string{"http://localhost:8080/kas"}))
				if err == nil {
					result, err = io.ReadAll(reader)
				}
			}
		})
		report.OperationMS = milliseconds(start)
		require(err)
		closeStart := time.Now()
		require(client.Close())
		report.CloseMS = milliseconds(closeStart)
	} else {
		callbacks := g.TokenCallbacks(func(context.Context) (g.AccessToken, error) {
			return g.AccessToken{Value: raw.Token, Scheme: "Bearer", ExpiresAt: raw.Expires}, nil
		})
		report.ConfigMS = milliseconds(full)
		start := time.Now()
		pprof.Do(context.Background(), pprof.Labels("target", target, "operation", op, "phase", "public_operation"), func(ctx context.Context) {
			if op == "encrypt" {
				result, err = g.Encrypt(ctx, cfg, input, g.EncryptOptions{
					Attributes:  []string{"https://example.com/attr/attr1/value/value1"},
					SegmentSize: 2 << 20, HasSegmentSize: true, SegmentHashAlgorithm: "GMAC",
				}, callbacks)
			} else {
				var output g.Decrypted
				output, err = g.Decrypt(ctx, cfg, archive, callbacks)
				result = output.Payload
			}
		})
		report.OperationMS = milliseconds(start)
		require(err)
	}
	report.FullMS = milliseconds(full)
	return result, report
}

func main() {
	if len(os.Args) != 6 {
		panic("usage: profiler campaign target operation seconds output-dir")
	}
	run, target, op, output := os.Args[1], os.Args[2], os.Args[3], os.Args[5]
	if target != "reference" && target != "go" {
		panic("unsupported target")
	}
	if op != "encrypt" && op != "decrypt" {
		panic("unsupported operation")
	}
	seconds, err := strconv.ParseFloat(os.Args[4], 64)
	require(err)
	if seconds <= 0 || seconds > 30 {
		panic("profile duration must be in (0,30] seconds")
	}
	require(os.MkdirAll(output, 0700))
	var raw configuration
	require(json.Unmarshal(read(filepath.Join(run, "private.json")), &raw))
	input := read(filepath.Join(run, "1.input"))
	archive := read(filepath.Join(run, "1.reference.tdf"))
	warmup, _ := operation(raw, input, archive, target, op)
	if op == "decrypt" {
		if !bytes.Equal(input, warmup) {
			panic("warmup plaintext mismatch")
		}
	} else {
		require(os.WriteFile(filepath.Join(output, "warmup.tdf"), warmup, 0600))
	}
	profile, err := os.Create(filepath.Join(output, "cpu.pprof"))
	require(err)
	require(pprof.StartCPUProfile(profile))
	started := time.Now()
	records := []phases{}
	var first, last []byte
	for time.Since(started).Seconds() < seconds {
		result, timing := operation(raw, input, archive, target, op)
		// Exact plaintext validation is outside each operation's wall timer.
		if op == "decrypt" {
			pprof.Do(context.Background(), pprof.Labels("phase", "plaintext_validation"), func(context.Context) {
				if !bytes.Equal(input, result) {
					panic("profile plaintext mismatch")
				}
			})
		} else {
			if first == nil {
				first = result
			}
			last = result
		}
		records = append(records, timing)
	}
	wall := milliseconds(started)
	pprof.StopCPUProfile()
	require(profile.Close())
	if op == "encrypt" {
		require(os.WriteFile(filepath.Join(output, "first.tdf"), first, 0600))
		require(os.WriteFile(filepath.Join(output, "last.tdf"), last, 0600))
	}
	summary := map[string]any{
		"target": target, "operation": op, "size_bytes": len(input), "profile_seconds_requested": seconds,
		"wall_loop_ms": wall, "operations": len(records), "warmups": 1,
		"correct": true, "phase_timings": records,
		"timing_scope":          "instrumented diagnostic loop; CPU sampling and labels active; not published benchmark samples",
		"generated_setup_scope": "internal generated client creation is inside the generated public API; original_client_setup_ms applies only to reference SDK.New",
		"kas_calls_expected": func() int {
			if op == "decrypt" {
				return len(records) + 1
			}
			return 0
		}(),
	}
	bytes, err := json.MarshalIndent(summary, "", "  ")
	require(err)
	require(os.WriteFile(filepath.Join(output, "instrumented.json"), append(bytes, '\n'), 0600))
	fmt.Println(string(bytes))
}
