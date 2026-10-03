// End-to-end pinned stock SDK consumer; reusable client initialization is untimed.
package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"github.com/opentdf/platform/lib/ocrypto"
	r "github.com/opentdf/platform/sdk"
	"golang.org/x/oauth2"
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"strconv"
	"time"
)

func must(err error) {
	if err != nil {
		panic(err)
	}
}
func read(path string) []byte { value, err := os.ReadFile(path); must(err); return value }
func main() {
	run, op, size := os.Args[1], os.Args[2], os.Args[3]
	if op != "e2e" && op != "validate" {
		panic("reference supports e2e or validate")
	}
	n, err := strconv.Atoi(os.Args[4])
	must(err)
	var raw struct {
		Config  struct{ KASPublicKeyPEM, KID string }
		Token   string
		Expires int64
	}
	must(json.Unmarshal(read(filepath.Join(run, "private.json")), &raw))
	inputSize := size
	if op == "validate" {
		inputSize = os.Args[5]
	}
	input := read(filepath.Join(run, inputSize+".input"))
	// RSA2048 response-session and ES256 signer creation happen once, before warmup.
	client, err := r.New("http://localhost:8080",
		r.WithOAuthAccessTokenSource(oauth2.StaticTokenSource(&oauth2.Token{AccessToken: raw.Token, TokenType: "Bearer", Expiry: time.Unix(raw.Expires, 0)})),
		r.WithPlatformConfiguration(r.PlatformConfiguration{}),
		r.WithTokenEndpoint("http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token"),
		r.WithInsecurePlaintextConn(), r.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil))))
	must(err)
	defer func() { must(client.Close()) }()
	encryptOptions := []r.TDFOption{r.WithAutoconfigure(false),
		r.WithKasInformation(r.KASInfo{URL: "http://localhost:8080/kas", Algorithm: "rsa:2048", PublicKey: raw.Config.KASPublicKeyPEM, KID: raw.Config.KID}),
		r.WithWrappingKeyAlg(ocrypto.RSA2048Key), r.WithSegmentSize(2 << 20), r.WithDataAttributes("https://example.com/attr/attr1/value/value1")}
	// LoadTDF reuses New's default RSA2048 session. Do not generate a replacement.
	readerOptions := []r.TDFReaderOption{r.WithKasAllowlist([]string{"http://localhost:8080/kas"})}
	samples := []float64{}
	for i := -1; i < n; i++ {
		var archive []byte
		if op == "validate" {
			archive = read(size)
		}
		buffer := bytes.Buffer{}
		start := time.Now()
		if op == "e2e" {
			_, err = client.CreateTDF(&buffer, bytes.NewReader(input), encryptOptions...)
			must(err)
			archive = buffer.Bytes()
		}
		reader, err := client.LoadTDF(bytes.NewReader(archive), readerOptions...)
		must(err)
		output, err := io.ReadAll(reader)
		must(err)
		elapsed := float64(time.Since(start).Nanoseconds()) / 1e6
		if !bytes.Equal(input, output) {
			panic("plaintext mismatch")
		}
		if op == "e2e" {
			must(os.WriteFile(filepath.Join(run, fmt.Sprintf("reference-%s-%d.tdf", size, i)), archive, 0600))
		}
		if i >= 0 {
			samples = append(samples, elapsed)
		}
	}
	must(json.NewEncoder(os.Stdout).Encode(map[string]any{"samples_ms": samples, "correct": true, "kas_calls_expected": n + 1, "client_initializations": 1}))
}
