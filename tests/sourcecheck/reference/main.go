// Native-only runner: independent checks against existing Phase 1 live fixtures.
package main

import (
	"archive/zip"
	"bytes"
	"fmt"
	"io"
	"opentdf-local/sdk/tdf"
	"os"
)

func main() {
	dir := ".local/interop"
	if len(os.Args) > 1 {
		dir = os.Args[1]
	}
	names := []string{"small.go", "small.web", "binary.go", "binary.web", "empty.go", "empty.web"}
	if len(os.Args) > 2 {
		names = append(names, os.Args[2:]...)
	}
	for _, name := range names {
		path := dir + "/" + name + ".tdf"
		b, e := os.ReadFile(path)
		if e != nil {
			panic(e)
		}
		a, e := tdf.ReadArchive(b, tdf.DefaultArchiveLimits())
		if e != nil {
			panic(fmt.Sprintf("%s: %v", path, e))
		}
		r, e := zip.NewReader(bytes.NewReader(b), int64(len(b)))
		if e != nil {
			panic(e)
		}
		for _, f := range r.File {
			x, e := f.Open()
			if e != nil {
				panic(e)
			}
			data, e := io.ReadAll(x)
			x.Close()
			if e != nil {
				panic(e)
			}
			if f.Name == tdf.PayloadEntry && !bytes.Equal(data, a.Payload) {
				panic("payload mismatch")
			}
			if f.Name == a.ManifestName && !bytes.Equal(data, a.Manifest) {
				panic("manifest mismatch")
			}
		}
		m, e := tdf.ParseManifest(a.Manifest)
		if e != nil {
			panic(e)
		}
		if e = m.ValidateModern(len(a.Payload)); e != nil {
			panic(fmt.Sprintf("%s: %v", path, e))
		}
		serialized, e := m.Marshal()
		if e != nil {
			panic(e)
		}
		again, e := tdf.ParseManifest(serialized)
		if e != nil || again.ValidateModern(len(a.Payload)) != nil || again.Encryption.Policy != m.Encryption.Policy {
			panic("manifest roundtrip")
		}
		fmt.Printf("%s: ZIP, CRC, modern manifest valid; payload=%d segments=%d\n", name, len(a.Payload), len(m.Encryption.Integrity.Segments))
	}
}
