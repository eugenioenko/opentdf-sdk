// Native benchmark consumer of the accepted installed Go package.
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	g "goalchemyout"
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
		Config  g.Config
		Token   string
		Expires int64
	}
	must(json.Unmarshal(read(filepath.Join(run, "private.json")), &raw))
	input := read(filepath.Join(run, size+".input"))
	archive := []byte(nil)
	if op == "decrypt" {
		archive = read(filepath.Join(run, size+".reference.tdf"))
	}
	samples := []float64{}
	for i := -1; i < n; i++ {
		start := time.Now()
		cfg := raw.Config
		callbacks := g.TokenCallbacks(func(context.Context) (g.AccessToken, error) {
			return g.AccessToken{Value: raw.Token, Scheme: "Bearer", ExpiresAt: raw.Expires}, nil
		})
		var output []byte
		if op == "encrypt" {
			output, e = g.Encrypt(context.Background(), cfg, input, g.EncryptOptions{Attributes: []string{"https://example.com/attr/attr1/value/value1"}, SegmentSize: 2 << 20, HasSegmentSize: true, SegmentHashAlgorithm: "GMAC"}, callbacks)
		} else {
			var result g.Decrypted
			result, e = g.Decrypt(context.Background(), cfg, archive, callbacks)
			output = result.Payload
		}
		elapsed := float64(time.Since(start).Nanoseconds()) / 1e6
		must(e)
		if op == "decrypt" && !bytes.Equal(input, output) {
			panic("plaintext mismatch")
		}
		if op == "encrypt" {
			must(os.WriteFile(filepath.Join(run, fmt.Sprintf("go-%s-%d.tdf", size, i)), output, 0600))
		}
		if i >= 0 {
			samples = append(samples, elapsed)
		}
	}
	must(json.NewEncoder(os.Stdout).Encode(map[string]any{"samples_ms": samples, "correct": true}))
}
