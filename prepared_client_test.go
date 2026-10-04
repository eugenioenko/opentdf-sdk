package sdk

import (
	"bytes"
	"context"
	"errors"
	"testing"

	"opentdf-local/sdk/tdf"
)

func TestPreparedDecryptRejectsBeforeCredentials(t *testing.T) {
	f := newFixture(t)
	data := f.encrypt(t)
	a, err := tdf.ReadArchive(data, tdf.DefaultArchiveLimits())
	if err != nil {
		t.Fatal(err)
	}
	m, err := tdf.ParseManifest(a.Manifest)
	if err != nil {
		t.Fatal(err)
	}
	crcBad := bytes.Clone(data)
	index := bytes.Index(crcBad, a.Payload)
	if index < 0 {
		t.Fatal("payload missing")
	}
	crcBad[index] ^= 1
	jsonBad, err := tdf.WriteArchive(a.Payload, []byte("{"), tdf.DefaultArchiveLimits())
	if err != nil {
		t.Fatal(err)
	}
	m.SchemaVersion = "4.2.2"
	manifest, err := m.Marshal()
	if err != nil {
		t.Fatal(err)
	}
	unsupported, err := tdf.WriteArchive(a.Payload, manifest, tdf.DefaultArchiveLimits())
	if err != nil {
		t.Fatal(err)
	}
	for _, tc := range []struct {
		name, operation string
		data            []byte
	}{{"CRC", "archive", crcBad}, {"JSON", "manifest", jsonBad}, {"profile", "unsupported_manifest", unsupported}} {
		t.Run(tc.name, func(t *testing.T) {
			out, err := f.client.Decrypt(context.Background(), tc.data)
			var typed *Error
			if !errors.As(err, &typed) || typed.Code != tc.operation || typed.Operation != "decrypt" || out.Payload != nil || out.Metadata != nil {
				t.Fatalf("lost preauth error stage: %v", err)
			}
			if f.tokenCalls.Load() != 0 || f.resourceCalls.Load() != 0 {
				t.Fatal("invalid archive requested credentials/KAS")
			}
		})
	}
}
