package tdf

import (
	"bytes"
	"testing"

	j "opentdf-local/sdk/src/tdf/json"
)

func corruptPreparedSnapshot(m *Manifest) {
	m.Encryption.KeyAccess[0].Binding.Hash = "corrupted"
	m.Encryption.Integrity.Segments[0].Hash = "corrupted"
	var corrupt func(*j.Value)
	corrupt = func(v *j.Value) {
		v.Text = "corrupted"
		for i := range v.Names {
			v.Names[i] = "corrupted"
		}
		for i := range v.Children {
			corrupt(&v.Children[i])
		}
	}
	corrupt(&m.Raw)
	for i := range m.Assertions {
		corrupt(&m.Assertions[i])
	}
}

func TestPreparedDecryptionOwnsInputAndResults(t *testing.T) {
	kas := engineKey(t, false)
	plain, metadata := bytes.Repeat([]byte{0, 255, 7}, MinSegmentBytes), []byte("owned metadata")
	data, err := Encrypt(plain, EncryptConfig{KASPublicKey: kas, KASURL: "https://kas.test", SegmentSize: MinSegmentBytes, Metadata: metadata})
	if err != nil {
		t.Fatal(err)
	}
	prepared, stage, err := PrepareDecryption(data)
	if err != nil || stage != "" {
		t.Fatalf("prepare: %s %v", stage, err)
	}
	manifest := prepared.Manifest()
	key := independentKey(t, kas, manifest)
	corruptPreparedSnapshot(&manifest)
	for i := range data {
		data[i] = 0
	}
	first, err := prepared.Decrypt(key)
	if err != nil || !bytes.Equal(first.Payload, plain) || !bytes.Equal(first.Metadata, metadata) {
		t.Fatalf("caller mutated cached input: %v", err)
	}
	first.Payload[0] ^= 1
	first.Metadata[0] ^= 1
	corruptPreparedSnapshot(&first.Manifest)
	second, err := prepared.Decrypt(key)
	if err != nil || !bytes.Equal(second.Payload, plain) || !bytes.Equal(second.Metadata, metadata) {
		t.Fatalf("returned output mutated cached state: %v", err)
	}
	if second.Manifest.Raw.Names[0] == "corrupted" {
		t.Fatal("raw manifest aliases a caller snapshot")
	}
}

func TestPreparedDecryptionStagesAndFailsClosed(t *testing.T) {
	kas := engineKey(t, false)
	data, err := Encrypt(bytes.Repeat([]byte{7}, MinSegmentBytes+1), EncryptConfig{KASPublicKey: kas, KASURL: "https://kas.test", SegmentSize: MinSegmentBytes})
	if err != nil {
		t.Fatal(err)
	}
	a, m := engineParts(t, data)
	key := independentKey(t, kas, m)
	crcBad := bytes.Clone(data)
	index := bytes.Index(crcBad, a.Payload)
	if index < 0 {
		t.Fatal("payload missing")
	}
	crcBad[index] ^= 1
	jsonBad, err := WriteArchive(a.Payload, []byte("{"), DefaultArchiveLimits())
	if err != nil {
		t.Fatal(err)
	}
	m.SchemaVersion = "4.2.2"
	unsupported := archiveChange(t, a, m)
	for _, tc := range []struct {
		name, stage string
		data        []byte
	}{{"CRC", "archive", crcBad}, {"JSON", "manifest", jsonBad}, {"profile", "unsupported_manifest", unsupported}} {
		t.Run(tc.name, func(t *testing.T) {
			p, stage, err := PrepareDecryption(tc.data)
			if err == nil || stage != tc.stage {
				t.Fatalf("stage %q: %v", stage, err)
			}
			out, err := p.Decrypt(key)
			if err == nil || out.Payload != nil || out.Metadata != nil || out.Manifest.SchemaVersion != "" {
				t.Fatal("failed constructor yielded usable plaintext")
			}
		})
	}
	for _, p := range []PreparedDecryption{{}, func() PreparedDecryption { p, _, _ := PrepareDecryption(data); return p }()} {
		for _, wrong := range [][]byte{nil, make([]byte, 31), make([]byte, 32), make([]byte, 33)} {
			out, err := p.Decrypt(wrong)
			if err == nil || out.Payload != nil || out.Metadata != nil || out.Manifest.SchemaVersion != "" {
				t.Fatal("invalid prepared archive/key returned partial data")
			}
		}
	}
}

func TestPreparedManifestAssertionsAreOwned(t *testing.T) {
	assertion := j.Obj([]string{"nested"}, []j.Value{j.Arr([]j.Value{j.Str("original")})})
	m := Manifest{Raw: j.Obj([]string{"assertions"}, []j.Value{j.Arr([]j.Value{assertion})}), Assertions: []j.Value{assertion}}
	copy := clonePreparedManifest(m)
	copy.Assertions[0].Names[0] = "changed"
	copy.Assertions[0].Children[0].Children[0].Text = "changed"
	copy.Raw.Children[0].Children[0].Names[0] = "changed"
	if m.Assertions[0].Names[0] != "nested" || m.Assertions[0].Children[0].Children[0].Text != "original" || m.Raw.Children[0].Children[0].Names[0] != "nested" {
		t.Fatal("nested assertion/raw slices alias the source")
	}
}
