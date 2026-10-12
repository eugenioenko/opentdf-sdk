package main

import (
	"archive/zip"
	"bytes"
	"encoding/json"
	"errors"
	"io"
	"os"
)

const manifestEntry = "0.manifest.json"

func readManifest(path string) ([]byte, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}
	return readManifestBytes(data)
}

func readManifestBytes(data []byte) ([]byte, error) {
	r, err := zip.NewReader(bytes.NewReader(data), int64(len(data)))
	if err != nil {
		return nil, err
	}
	for _, f := range r.File {
		if f.Name != manifestEntry {
			continue
		}
		rc, err := f.Open()
		if err != nil {
			return nil, err
		}
		defer rc.Close()
		return io.ReadAll(io.LimitReader(rc, 64<<20))
	}
	return nil, errors.New("manifest entry not found")
}

func manifestKASURL(manifest []byte) string {
	var m struct {
		EncryptionInformation struct {
			KeyAccess []struct {
				URL string `json:"url"`
			} `json:"keyAccess"`
		} `json:"encryptionInformation"`
	}
	if json.Unmarshal(manifest, &m) != nil || len(m.EncryptionInformation.KeyAccess) == 0 {
		return ""
	}
	return m.EncryptionInformation.KeyAccess[0].URL
}
