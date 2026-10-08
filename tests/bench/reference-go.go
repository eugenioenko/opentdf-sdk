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
	warmups, bulkWarmups := 1, 0
	if op == "e2e" && len(os.Args) > 5 {
		warmups, err = strconv.Atoi(os.Args[5])
		must(err)
	}
	if op == "e2e" && len(os.Args) > 6 {
		bulkWarmups, err = strconv.Atoi(os.Args[6])
		must(err)
	}
	if warmups < 1 || bulkWarmups < 0 {
		panic("invalid warmup count")
	}
	var raw struct {
		Config  struct{ KASPublicKeyPEM, KID, PlatformURL, KASURL, TokenURL string }
		Token   string
		Expires int64
	}
	must(json.Unmarshal(read(filepath.Join(run, "private.json")), &raw))
	inputSize := size
	if op == "validate" {
		inputSize = os.Args[5]
	}
	input := read(filepath.Join(run, inputSize+".input"))
	bulkInput := input
	if bulkWarmups > 0 && size != "50" {
		bulkInput = read(filepath.Join(run, "50.input"))
	}
	accessToken := &oauth2.Token{AccessToken: raw.Token, TokenType: "Bearer", Expiry: time.Unix(raw.Expires, 0)}
	platformURL, kasURL, tokenURL := raw.Config.PlatformURL, raw.Config.KASURL, raw.Config.TokenURL
	if platformURL == "" {
		platformURL = "http://localhost:8080"
	}
	if kasURL == "" {
		kasURL = platformURL + "/kas"
	}
	if tokenURL == "" {
		tokenURL = "http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token"
	}
	// RSA2048 response-session and ES256 signer creation happen once, before warmup.
	client, err := r.New(platformURL,
		r.WithOAuthAccessTokenSource(oauth2.StaticTokenSource(accessToken)),
		r.WithPlatformConfiguration(r.PlatformConfiguration{}),
		r.WithTokenEndpoint(tokenURL),
		r.WithInsecurePlaintextConn(), r.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil))))
	must(err)
	defer func() { must(client.Close()) }()
	encryptOptions := []r.TDFOption{r.WithAutoconfigure(false),
		r.WithKasInformation(r.KASInfo{URL: kasURL, Algorithm: "rsa:2048", PublicKey: raw.Config.KASPublicKeyPEM, KID: raw.Config.KID}),
		r.WithWrappingKeyAlg(ocrypto.RSA2048Key), r.WithSegmentSize(2 << 20), r.WithDataAttributes("https://example.com/attr/attr1/value/value1")}
	// LoadTDF reuses New's default RSA2048 session. Do not generate a replacement.
	readerOptions := []r.TDFReaderOption{r.WithKasAllowlist([]string{kasURL})}
	samples, warmupHistory, bulkHistory := []float64{}, []float64{}, []float64{}
	for i := -bulkWarmups - warmups; i < n; i++ {
		pairInput := input
		if i < -warmups {
			pairInput = bulkInput
		}
		var archive []byte
		if op == "validate" {
			archive = read(size)
		}
		buffer := bytes.Buffer{}
		start := time.Now()
		if op == "e2e" {
			_, err = client.CreateTDF(&buffer, bytes.NewReader(pairInput), encryptOptions...)
			must(err)
			archive = buffer.Bytes()
		}
		reader, err := client.LoadTDF(bytes.NewReader(archive), readerOptions...)
		must(err)
		output, err := io.ReadAll(reader)
		must(err)
		elapsed := float64(time.Since(start).Nanoseconds()) / 1e6
		if !bytes.Equal(pairInput, output) {
			panic("plaintext mismatch")
		}
		if op == "e2e" && (i == -1 || i >= 0) {
			must(os.WriteFile(filepath.Join(run, fmt.Sprintf("reference-%s-%d.tdf", size, i)), archive, 0600))
		}
		if i >= 0 {
			samples = append(samples, elapsed)
		} else if i < -warmups {
			bulkHistory = append(bulkHistory, elapsed)
		} else {
			warmupHistory = append(warmupHistory, elapsed)
		}
	}
	must(json.NewEncoder(os.Stdout).Encode(map[string]any{"samples_ms": samples, "warmup_ms": warmupHistory, "bulk_warmup_ms": bulkHistory, "warmup_count": warmups, "bulk_warmup_count": bulkWarmups, "correct": true, "kas_calls_expected": n + warmups + bulkWarmups, "client_initializations": 1}))
}
