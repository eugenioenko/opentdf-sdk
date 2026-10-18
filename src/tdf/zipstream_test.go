package tdf

import (
	"archive/zip"
	"bytes"
	"hash/crc32"
	"io"
	"testing"
)

func streamArchive(t *testing.T, payload [][]byte, manifest []byte, zip64 bool) []byte {
	t.Helper()
	var out bytes.Buffer
	w := NewWriter(&out, zip64)
	for _, p := range payload {
		if e := w.WritePayload(p); e != nil {
			t.Fatal(e)
		}
	}
	size, e := w.Finish(manifest)
	if e != nil || size != int64(out.Len()) {
		t.Fatal(size, out.Len(), e)
	}
	if e := w.WritePayload([]byte{1}); e == nil {
		t.Fatal("write after finish")
	}
	return out.Bytes()
}

func TestStreamWriterReadByStandardLibrary(t *testing.T) {
	big := bytes.Repeat([]byte{7, 8, 9}, 1<<20)
	for _, zip64 := range []bool{false, true} {
		for _, chunks := range [][][]byte{nil, {{}}, {{0, 255}}, {[]byte("abc"), []byte("def"), {}, []byte("g")}, {big[:1000], big[1000:]}} {
			manifest := []byte(`{"schemaVersion":"4.3.0"}`)
			b := streamArchive(t, chunks, manifest, zip64)
			want := bytes.Join(chunks, nil)
			checkStd(t, b, map[string][]byte{PayloadEntry: want, ManifestEntry: manifest})
			r, e := zip.NewReader(bytes.NewReader(b), int64(len(b)))
			if e != nil || len(r.File) != 2 || r.File[0].Name != PayloadEntry || r.File[0].CRC32 != crc32.ChecksumIEEE(want) || r.File[0].UncompressedSize64 != uint64(len(want)) {
				t.Fatal("central directory", e)
			}
			// The whole-buffer reader and the streaming reader read it too.
			a, e := ReadArchive(b, DefaultArchiveLimits())
			if e != nil || !bytes.Equal(a.Payload, want) || !bytes.Equal(a.Manifest, manifest) {
				t.Fatal("ReadArchive", e)
			}
			s, e := NewReader(bytes.NewReader(b), DefaultArchiveLimits())
			if e != nil || !bytes.Equal(s.Manifest(), manifest) || s.ManifestName() != ManifestEntry || s.PayloadSize() != int64(len(want)) {
				t.Fatal("NewReader", e)
			}
			got, e := s.ReadPayload(0, s.PayloadSize())
			if e != nil || !bytes.Equal(got, want) {
				t.Fatal("ReadPayload", e)
			}
			if b2 := streamArchive(t, chunks, manifest, zip64); !bytes.Equal(b, b2) {
				t.Fatal("not deterministic")
			}
		}
	}
	b := streamArchive(t, [][]byte{{1}}, []byte("m"), true)
	if !bytes.Contains(b, []byte{0x50, 0x4b, 0x06, 0x06}) || !bytes.Contains(b, []byte{0x50, 0x4b, 0x06, 0x07}) {
		t.Fatal("forced ZIP64 writes the ZIP64 end record and locator")
	}
	if b := streamArchive(t, [][]byte{{1}}, []byte("m"), false); bytes.Contains(b, []byte{0x50, 0x4b, 0x06, 0x06}) {
		t.Fatal("ZIP32 archive carries ZIP64 records")
	}
}

func TestStreamReaderIndependentWriters(t *testing.T) {
	payload := bytes.Repeat([]byte{1, 2, 3, 4, 5}, 5000)
	manifest := []byte(`{"m":1}`)
	for name, b := range map[string][]byte{
		"std stored":     stdArchive(t, []string{PayloadEntry, ManifestEntry}, [][]byte{payload, manifest}, zip.Store),
		"std reversed":   stdArchive(t, []string{ManifestEntry, PayloadEntry}, [][]byte{manifest, payload}, zip.Store),
		"spec manifest":  stdArchive(t, []string{PayloadEntry, SpecManifestEntry}, [][]byte{payload, manifest}, zip.Store),
		"both manifests": stdArchive(t, []string{PayloadEntry, ManifestEntry, SpecManifestEntry}, [][]byte{payload, []byte("old"), manifest}, zip.Store),
		"whole buffer":   mustWrite(t, payload, manifest),
	} {
		r, e := NewReader(bytes.NewReader(b), DefaultArchiveLimits())
		if e != nil {
			t.Fatal(name, e)
		}
		if !bytes.Equal(r.Manifest(), manifest) || r.PayloadSize() != int64(len(payload)) {
			t.Fatal(name, "manifest or size")
		}
		for _, c := range []struct{ off, n int64 }{{0, 0}, {0, 1}, {7, 100}, {int64(len(payload)) - 3, 3}, {int64(len(payload)), 0}} {
			got, e := r.ReadPayload(c.off, c.n)
			if e != nil || !bytes.Equal(got, payload[c.off:c.off+c.n]) {
				t.Fatal(name, c, e)
			}
		}
		for _, c := range []struct{ off, n int64 }{{-1, 1}, {0, -1}, {int64(len(payload)), 1}, {1, int64(len(payload))}} {
			if _, e := r.ReadPayload(c.off, c.n); e == nil {
				t.Fatal(name, c, "accepted")
			}
		}
	}
	if _, e := NewReader(bytes.NewReader(stdArchive(t, []string{PayloadEntry, ManifestEntry}, [][]byte{payload, manifest}, zip.Deflate)), DefaultArchiveLimits()); e == nil {
		t.Fatal("compressed entries")
	}
	if _, e := NewReader(bytes.NewReader(stdArchive(t, []string{PayloadEntry}, [][]byte{payload}, zip.Store)), DefaultArchiveLimits()); e == nil {
		t.Fatal("missing manifest")
	}
	if _, e := NewReader(bytes.NewReader(stdArchive(t, []string{ManifestEntry}, [][]byte{manifest}, zip.Store)), DefaultArchiveLimits()); e == nil {
		t.Fatal("missing payload")
	}
	l := DefaultArchiveLimits()
	l.ManifestBytes = 3
	if _, e := NewReader(bytes.NewReader(mustWrite(t, payload, manifest)), l); e == nil {
		t.Fatal("manifest over the limit")
	}
	if _, e := NewReader(bytes.NewReader(nil), DefaultArchiveLimits()); e == nil {
		t.Fatal("empty input")
	}
	b := mustWrite(t, payload, manifest)
	for _, i := range []int{1, 10, len(b) - 1, len(b) - 30, len(b) - 100} {
		if _, e := NewReader(bytes.NewReader(b[:i]), DefaultArchiveLimits()); e == nil {
			t.Fatal("truncated at", i)
		}
	}
	// A central directory pointing past the archive is rejected.
	bad := append([]byte(nil), b...)
	put32(bad, len(bad)-6, 0x7fffffff)
	if _, e := NewReader(bytes.NewReader(bad), DefaultArchiveLimits()); e == nil {
		t.Fatal("directory past the end")
	}
	// The read position does not matter: the reader seeks to the records it needs.
	rs := bytes.NewReader(b)
	rs.Seek(5, io.SeekStart)
	if _, e := NewReader(rs, DefaultArchiveLimits()); e != nil {
		t.Fatal(e)
	}
}

func mustWrite(t *testing.T, payload, manifest []byte) []byte {
	t.Helper()
	b, e := WriteArchive(payload, manifest, DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	return b
}

func TestCRC32Combine(t *testing.T) {
	parts := [][]byte{nil, {}, {1}, bytes.Repeat([]byte{3, 4}, 7777), []byte("tail")}
	var crc uint32
	var all []byte
	for _, p := range parts {
		crc = crc32Combine(crc, crc32.ChecksumIEEE(p), int64(len(p)))
		all = append(all, p...)
		if crc != crc32.ChecksumIEEE(all) {
			t.Fatalf("combine %d: %x %x", len(all), crc, crc32.ChecksumIEEE(all))
		}
	}
}
