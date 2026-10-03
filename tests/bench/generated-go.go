// End-to-end native consumer of the accepted installed Go package.
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

func must(err error) {
	if err != nil {
		panic(err)
	}
}
func read(path string) []byte { value, err := os.ReadFile(path); must(err); return value }
func main() {
	run, op, size := os.Args[1], os.Args[2], os.Args[3]
	if op != "e2e" {
		panic("generated Go consumer supports e2e only")
	}
	n, err := strconv.Atoi(os.Args[4])
	must(err)
	var raw struct {
		Config  g.Config
		Token   string
		Expires int64
	}
	must(json.Unmarshal(read(filepath.Join(run, "private.json")), &raw))
	input := read(filepath.Join(run, size+".input"))
	cfg := raw.Config
	callbacks := g.TokenCallbacks(func(context.Context) (g.AccessToken, error) {
		return g.AccessToken{Value: raw.Token, Scheme: "Bearer", ExpiresAt: raw.Expires}, nil
	})
	options := g.EncryptOptions{Attributes: []string{"https://example.com/attr/attr1/value/value1"}, SegmentSize: 2 << 20, HasSegmentSize: true, SegmentHashAlgorithm: "GMAC"}
	samples := []float64{}
	for i := -1; i < n; i++ {
		start := time.Now()
		archive, err := g.Encrypt(context.Background(), cfg, input, options, callbacks)
		must(err)
		result, err := g.Decrypt(context.Background(), cfg, archive, callbacks)
		must(err)
		elapsed := float64(time.Since(start).Nanoseconds()) / 1e6
		if !bytes.Equal(input, result.Payload) {
			panic("plaintext mismatch")
		}
		must(os.WriteFile(filepath.Join(run, fmt.Sprintf("go-%s-%d.tdf", size, i)), archive, 0600))
		if i >= 0 {
			samples = append(samples, elapsed)
		}
	}
	must(json.NewEncoder(os.Stdout).Encode(map[string]any{"samples_ms": samples, "correct": true, "kas_calls_expected": n + 1}))
}
