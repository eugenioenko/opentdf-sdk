// Pinned stock Go SDK producer/metadata reader, independent of shared SDK IR.
package main

import (
	"bytes"
	"fmt"
	"io"
	"log/slog"
	"os"
	"path/filepath"
	"strings"

	"github.com/opentdf/platform/lib/ocrypto"
	reference "github.com/opentdf/platform/sdk"
)

func check(err error) {
	if err != nil {
		panic("reference operation failed")
	}
}
func main() {
	if len(os.Args) != 5 {
		panic("usage: reference run-dir mode case algorithm")
	}
	run, mode, name, algorithm := os.Args[1], os.Args[2], os.Args[3], os.Args[4]
	client, err := reference.New("http://localhost:8080", reference.WithClientCredentials("opentdf-sdk", "secret", nil), reference.WithInsecurePlaintextConn(), reference.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil))))
	check(err)
	defer client.Close()
	kas := "http://localhost:8080/kas"
	if mode == "encrypt" {
		input, err := os.ReadFile(filepath.Join(run, name+".input"))
		check(err)
		key := reference.KASInfo{URL: kas, Algorithm: algorithm}
		options := []reference.TDFOption{reference.WithAutoconfigure(false), reference.WithKasInformation(key), reference.WithWrappingKeyAlg(ocrypto.KeyType(algorithm)), reference.WithSegmentSize(16384), reference.WithDataAttributes("https://example.com/attr/attr1/value/value1")}
		if (name == "metadata" || strings.HasSuffix(name, "-metadata")) && name != "empty-metadata" && !strings.HasSuffix(name, "-empty-metadata") {
			options = append(options, reference.WithMetaData(`{"source":"independent metadata","count":7}`))
		}
		var out bytes.Buffer
		_, err = client.CreateTDF(&out, bytes.NewReader(input), options...)
		check(err)
		check(os.WriteFile(filepath.Join(run, name+".go.tdf"), out.Bytes(), 0600))
	} else if mode == "decrypt" {
		archive, err := os.ReadFile(filepath.Join(run, name+".generated.tdf"))
		check(err)
		reader, err := client.LoadTDF(bytes.NewReader(archive), reference.WithKasAllowlist([]string{kas}), reference.WithSessionKeyType(ocrypto.KeyType(algorithm)))
		check(err)
		payload, err := io.ReadAll(reader)
		check(err)
		metadata, err := reader.UnencryptedMetadata()
		check(err)
		check(os.WriteFile(filepath.Join(run, name+".stock-go.out"), payload, 0600))
		check(os.WriteFile(filepath.Join(run, name+".stock-go.metadata"), metadata, 0600))
	} else {
		panic("unknown mode")
	}
	fmt.Println("PASS stock Go", mode, name)
}
