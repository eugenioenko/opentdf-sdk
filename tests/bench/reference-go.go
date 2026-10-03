// Pinned original OpenTDF Go SDK, full fresh-client public operation boundary.
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

func must(e error) {
	if e != nil {
		panic(e)
	}
}
func read(p string) []byte { b, e := os.ReadFile(p); must(e); return b }
func main() {
	run, op, size := os.Args[1], os.Args[2], os.Args[3]
	n, e := strconv.Atoi(os.Args[4])
	must(e)
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
	var archive []byte
	if op == "decrypt" {
		archive = read(filepath.Join(run, size+".reference.tdf"))
	}
	samples := []float64{}
	for i := -1; i < n; i++ {
		data := archive
		if op == "validate" {
			data = read(size)
		}
		start := time.Now()
		client, e := r.New("http://localhost:8080", r.WithOAuthAccessTokenSource(oauth2.StaticTokenSource(&oauth2.Token{AccessToken: raw.Token, TokenType: "Bearer", Expiry: time.Unix(raw.Expires, 0)})), r.WithPlatformConfiguration(r.PlatformConfiguration{}), r.WithTokenEndpoint("http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token"), r.WithInsecurePlaintextConn(), r.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil))))
		must(e)
		var output []byte
		if op == "encrypt" {
			var out bytes.Buffer
			_, e = client.CreateTDF(&out, bytes.NewReader(input), r.WithAutoconfigure(false), r.WithKasInformation(r.KASInfo{URL: "http://localhost:8080/kas", Algorithm: "rsa:2048", PublicKey: raw.Config.KASPublicKeyPEM, KID: raw.Config.KID}), r.WithWrappingKeyAlg(ocrypto.RSA2048Key), r.WithSegmentSize(2<<20), r.WithDataAttributes("https://example.com/attr/attr1/value/value1"))
			output = out.Bytes()
		} else {
			var reader *r.Reader
			reader, e = client.LoadTDF(bytes.NewReader(data), r.WithKasAllowlist([]string{"http://localhost:8080/kas"}), r.WithSessionKeyType(ocrypto.RSA2048Key))
			if e == nil {
				output, e = io.ReadAll(reader)
			}
		}
		must(e)
		must(client.Close())
		elapsed := float64(time.Since(start).Nanoseconds()) / 1e6
		if op != "encrypt" {
			if op == "validate" {
				input = read(filepath.Join(run, os.Args[5]+".input"))
			}
			if !bytes.Equal(input, output) {
				panic("plaintext mismatch")
			}
		} else {
			must(os.WriteFile(filepath.Join(run, fmt.Sprintf("reference-%s-%d.tdf", size, i)), output, 0600))
			if i == -1 {
				must(os.WriteFile(filepath.Join(run, size+".reference.tdf"), output, 0600))
			}
		}
		if i >= 0 {
			samples = append(samples, elapsed)
		}
	}
	must(json.NewEncoder(os.Stdout).Encode(map[string]any{"samples_ms": samples, "correct": true}))
}
