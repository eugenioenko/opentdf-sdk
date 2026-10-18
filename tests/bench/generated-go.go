// Generated Go handle-API benchmark: one SDK per process, as the reference
// consumer, with the pre-acquired token resolved by a provider callback.
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"time"

	g "goalchemyout"
)

func must(err error) {
	if err != nil {
		panic(err)
	}
}

func read(path string) []byte {
	b, err := os.ReadFile(path)
	must(err)
	return b
}

func main() {
	run, op, size := os.Args[1], os.Args[2], os.Args[3]
	if op != "e2e" {
		panic("generated Go consumer supports e2e only")
	}
	n, err := strconv.Atoi(os.Args[4])
	must(err)
	warmups, bulkWarmups := 1, 0
	if len(os.Args) > 5 {
		warmups, err = strconv.Atoi(os.Args[5])
		must(err)
	}
	if len(os.Args) > 6 {
		bulkWarmups, err = strconv.Atoi(os.Args[6])
		must(err)
	}
	if warmups < 1 || bulkWarmups < 0 {
		panic("invalid warmup count")
	}
	var raw struct {
		Config  g.Config
		Token   string
		Expires int64
	}
	must(json.Unmarshal(read(filepath.Join(run, "private.json")), &raw))
	input := read(filepath.Join(run, size+".input"))
	bulkInput := input
	if bulkWarmups > 0 && size != "50" {
		bulkInput = read(filepath.Join(run, "50.input"))
	}
	cfg := raw.Config
	ctx := context.Background()
	callbacks := g.TokenCallbacks(func(context.Context) (g.AccessToken, error) {
		return g.AccessToken{Value: raw.Token, Scheme: "Bearer", ExpiresAt: raw.Expires}, nil
	})
	client, err := g.New(ctx, cfg.PlatformURL, g.OptionsOf(g.WithTokenEndpoint(cfg.TokenURL), g.WithTokenProvider(), g.WithInsecurePlaintextConn()), callbacks)
	must(err)
	defer client.Close()
	kas := g.KASInfo{URL: cfg.KASURL, PublicKey: cfg.KASPublicKeyPEM, KID: cfg.KID, Algorithm: "rsa:2048"}
	tdfOptions, err := g.TDFOptionsOf(g.WithAutoconfigure(false), g.WithKasInformation(kas), g.WithWrappingKeyAlg(g.KeyType("rsa:2048")), g.WithSegmentSize(2<<20), g.WithDataAttributes("https://example.com/attr/attr1/value/value1"))
	must(err)
	// LoadTDF reuses New's default RSA2048 session. Do not generate a replacement.
	readerOptions, err := g.ReaderOptionsOf(g.WithKasAllowlist([]string{cfg.KASURL}))
	must(err)
	samples, warmupHistory, bulkHistory := []float64{}, []float64{}, []float64{}
	for i := -bulkWarmups - warmups; i < n; i++ {
		pairInput := input
		if i < -warmups {
			pairInput = bulkInput
		}
		start := time.Now()
		archive, obj, err := client.EncryptBytes(ctx, pairInput, tdfOptions, callbacks)
		must(err)
		if obj != nil {
			must(obj.Close())
		}
		result, err := client.DecryptBytes(ctx, archive, readerOptions, callbacks)
		must(err)
		elapsed := float64(time.Since(start).Nanoseconds()) / 1e6
		if !bytes.Equal(pairInput, result) {
			panic("plaintext mismatch")
		}
		if i == -1 || i >= 0 {
			must(os.WriteFile(filepath.Join(run, fmt.Sprintf("go-%s-%d.tdf", size, i)), archive, 0600))
		}
		if i >= 0 {
			samples = append(samples, elapsed)
		} else if i < -warmups {
			bulkHistory = append(bulkHistory, elapsed)
		} else {
			warmupHistory = append(warmupHistory, elapsed)
		}
	}
	must(json.NewEncoder(os.Stdout).Encode(map[string]any{"samples_ms": samples, "warmup_ms": warmupHistory, "bulk_warmup_ms": bulkHistory, "warmup_count": warmups, "bulk_warmup_count": bulkWarmups, "correct": true, "kas_calls_expected": n + warmups + bulkWarmups}))
}
