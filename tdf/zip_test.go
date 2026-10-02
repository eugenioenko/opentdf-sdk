package tdf

import (
	"archive/zip"
	"bytes"
	"encoding/binary"
	"hash/crc32"
	"io"
	"testing"
)

var le = binary.LittleEndian

func stdArchive(t *testing.T, names []string, data [][]byte, method uint16) []byte {
	t.Helper()
	var b bytes.Buffer
	w := zip.NewWriter(&b)
	for i, n := range names {
		x, e := w.CreateHeader(&zip.FileHeader{Name: n, Method: method})
		if e != nil {
			t.Fatal(e)
		}
		if _, e = x.Write(data[i]); e != nil {
			t.Fatal(e)
		}
	}
	if e := w.Close(); e != nil {
		t.Fatal(e)
	}
	return b.Bytes()
}
func checkStd(t *testing.T, b []byte, want map[string][]byte) {
	t.Helper()
	r, e := zip.NewReader(bytes.NewReader(b), int64(len(b)))
	if e != nil {
		t.Fatal(e)
	}
	for _, f := range r.File {
		x, e := f.Open()
		if e != nil {
			t.Fatal(e)
		}
		got, e := io.ReadAll(x)
		x.Close()
		if e != nil || !bytes.Equal(got, want[f.Name]) {
			t.Fatal(f.Name, e, got)
		}
	}
}
func TestWriterIndependentReader(t *testing.T) {
	for _, p := range [][]byte{nil, {}, []byte{0, 255, 128, 1, 0}, bytes.Repeat([]byte{7}, 1<<20)} {
		m := []byte(`{"schemaVersion":"4.3.0"}`)
		b, e := WriteArchive(p, m, DefaultArchiveLimits())
		if e != nil {
			t.Fatal(e)
		}
		checkStd(t, b, map[string][]byte{PayloadEntry: p, ManifestEntry: m})
		b2, _ := WriteArchive(p, m, DefaultArchiveLimits())
		if !bytes.Equal(b, b2) {
			t.Fatal("not deterministic")
		}
	}
}
func TestReaderIndependentWriter(t *testing.T) {
	for _, payload := range [][]byte{nil, []byte{0, 255, 128, 1, 0}, bytes.Repeat([]byte{7}, 1<<20)} {
		m := []byte("manifest\x00bytes")
		b := stdArchive(t, []string{PayloadEntry, ManifestEntry}, [][]byte{payload, m}, zip.Store)
		got, e := ReadArchive(b, DefaultArchiveLimits())
		if e != nil || !bytes.Equal(got.Payload, payload) || !bytes.Equal(got.Manifest, m) {
			t.Fatal(got, e)
		}
	}
}

// Independent ZIP64 fixture builder uses the standard binary/crc32 packages;
// Go archive/zip reads every entry to validate its structure and CRC first.
func zip64FixtureData(t *testing.T, signed bool, payload []byte) []byte {
	t.Helper()
	names := []string{PayloadEntry, ManifestEntry}
	data := [][]byte{payload, []byte(`{}`)}
	out := []byte{}
	offsets := []int{}
	for i, n := range names {
		offsets = append(offsets, len(out))
		h := make([]byte, 30)
		le.PutUint32(h, 0x04034b50)
		le.PutUint16(h[4:], 45)
		le.PutUint16(h[6:], 2056)
		le.PutUint32(h[18:], 0xffffffff)
		le.PutUint32(h[22:], 0xffffffff)
		le.PutUint16(h[26:], uint16(len(n)))
		le.PutUint16(h[28:], 20)
		extra := make([]byte, 20)
		le.PutUint16(extra, 1)
		le.PutUint16(extra[2:], 16)
		out = append(out, h...)
		out = append(out, n...)
		out = append(out, extra...)
		out = append(out, data[i]...)
		dd := make([]byte, 20)
		le.PutUint32(dd, crc32.ChecksumIEEE(data[i]))
		le.PutUint64(dd[4:], uint64(len(data[i])))
		le.PutUint64(dd[12:], uint64(len(data[i])))
		if signed {
			sig := make([]byte, 4)
			le.PutUint32(sig, 0x08074b50)
			out = append(out, sig...)
		}
		out = append(out, dd...)
	}
	cd := len(out)
	for i, n := range names {
		h := make([]byte, 46)
		le.PutUint32(h, 0x02014b50)
		le.PutUint16(h[4:], 45)
		le.PutUint16(h[6:], 45)
		le.PutUint16(h[8:], 2056)
		le.PutUint32(h[16:], crc32.ChecksumIEEE(data[i]))
		le.PutUint32(h[20:], 0xffffffff)
		le.PutUint32(h[24:], 0xffffffff)
		le.PutUint16(h[28:], uint16(len(n)))
		le.PutUint16(h[30:], 28)
		le.PutUint32(h[42:], 0xffffffff)
		extra := make([]byte, 28)
		le.PutUint16(extra, 1)
		le.PutUint16(extra[2:], 24)
		le.PutUint64(extra[4:], uint64(len(data[i])))
		le.PutUint64(extra[12:], uint64(len(data[i])))
		le.PutUint64(extra[20:], uint64(offsets[i]))
		out = append(out, h...)
		out = append(out, n...)
		out = append(out, extra...)
	}
	size := len(out) - cd
	z := len(out)
	h := make([]byte, 56)
	le.PutUint32(h, 0x06064b50)
	le.PutUint64(h[4:], 44)
	le.PutUint16(h[12:], 45)
	le.PutUint16(h[14:], 45)
	le.PutUint64(h[24:], 2)
	le.PutUint64(h[32:], 2)
	le.PutUint64(h[40:], uint64(size))
	le.PutUint64(h[48:], uint64(cd))
	out = append(out, h...)
	h = make([]byte, 20)
	le.PutUint32(h, 0x07064b50)
	le.PutUint64(h[8:], uint64(z))
	le.PutUint32(h[16:], 1)
	out = append(out, h...)
	h = make([]byte, 22)
	le.PutUint32(h, 0x06054b50)
	le.PutUint16(h[8:], 65535)
	le.PutUint16(h[10:], 65535)
	le.PutUint32(h[12:], 0xffffffff)
	le.PutUint32(h[16:], 0xffffffff)
	out = append(out, h...)
	if signed || crc32.ChecksumIEEE(payload) != 0x08074b50 {
		checkStd(t, out, map[string][]byte{PayloadEntry: data[0], ManifestEntry: data[1]})
	} else {
		// archive/zip's descriptor reader treats this CRC as a signature.
		r, e := zip.NewReader(bytes.NewReader(out), int64(len(out)))
		if e != nil {
			t.Fatal(e)
		}
		for i, f := range r.File {
			raw, e := f.OpenRaw()
			if e != nil {
				t.Fatal(e)
			}
			b, e := io.ReadAll(raw)
			if e != nil || !bytes.Equal(b, data[i]) || crc32.ChecksumIEEE(b) != f.CRC32 {
				t.Fatal("independent raw CRC", e)
			}
		}
	}
	return out
}
func TestZIP64AndDescriptors(t *testing.T) {
	for _, signed := range []bool{true, false} {
		b := zip64FixtureData(t, signed, []byte{0, 255, 1, 0})
		got, e := ReadArchive(b, DefaultArchiveLimits())
		if e != nil || !bytes.Equal(got.Payload, []byte{0, 255, 1, 0}) {
			t.Fatal(e, got)
		}
		bad := bytes.Clone(b)
		bad[len(b)-22-20+8+4] = 1
		if _, e = ReadArchive(bad, DefaultArchiveLimits()); e == nil {
			t.Fatal("accepted overflowing ZIP64 offset")
		}
	}
	b := stdArchive(t, []string{PayloadEntry, ManifestEntry}, [][]byte{[]byte("p"), []byte("m")}, zip.Store)
	if _, e := ReadArchive(b, DefaultArchiveLimits()); e != nil {
		t.Fatal(e)
	}
}
func TestReaderRejectsMalformed(t *testing.T) {
	b, e := WriteArchive([]byte("payload"), []byte("manifest"), DefaultArchiveLimits())
	if e != nil {
		t.Fatal(e)
	}
	end := len(b) - 22
	cd := int(le.Uint32(b[end+16:]))
	cases := map[string]func([]byte){"crc": func(x []byte) { x[30+len(PayloadEntry)] ^= 1 }, "offset": func(x []byte) { le.PutUint32(x[cd+42:], 1) }, "compressed": func(x []byte) { le.PutUint16(x[cd+10:], 8) }, "encrypted": func(x []byte) { le.PutUint16(x[cd+8:], 1) }, "localname": func(x []byte) { x[30] = 'x' }, "localsize": func(x []byte) { le.PutUint32(x[18:], 8) }, "count": func(x []byte) { le.PutUint16(x[end+8:], 3); le.PutUint16(x[end+10:], 3) }, "centralextent": func(x []byte) { le.PutUint32(x[end+12:], 1) }, "disk": func(x []byte) { le.PutUint16(x[end+4:], 1) }, "version": func(x []byte) { le.PutUint16(x[cd+6:], 99) }}
	for n, fn := range cases {
		t.Run(n, func(t *testing.T) {
			bad := bytes.Clone(b)
			fn(bad)
			if _, e := ReadArchive(bad, DefaultArchiveLimits()); e == nil {
				t.Fatal("accepted malformed")
			}
		})
	}
	for i := 0; i < len(b); i++ {
		if _, e := ReadArchive(b[:i], DefaultArchiveLimits()); e == nil {
			t.Fatalf("accepted truncation %d", i)
		}
	}
	dup := stdArchive(t, []string{PayloadEntry, ManifestEntry, ManifestEntry}, [][]byte{{1}, {2}, {3}}, zip.Store)
	if _, e := ReadArchive(dup, DefaultArchiveLimits()); e == nil {
		t.Fatal("duplicate accepted")
	}
	compressed := stdArchive(t, []string{PayloadEntry, ManifestEntry}, [][]byte{{1}, {2}}, zip.Deflate)
	if _, e := ReadArchive(compressed, DefaultArchiveLimits()); e == nil {
		t.Fatal("compression accepted")
	}
	streamed := stdArchive(t, []string{PayloadEntry, ManifestEntry}, [][]byte{[]byte("p"), []byte("m")}, zip.Store)
	bad := bytes.Clone(streamed)
	dd := 30 + len(PayloadEntry) + 1
	bad[dd+4] ^= 1
	if _, e := ReadArchive(bad, DefaultArchiveLimits()); e == nil {
		t.Fatal("bad descriptor accepted")
	}
}
func TestManifestSelectionAndLimits(t *testing.T) {
	b := stdArchive(t, []string{PayloadEntry, ManifestEntry, SpecManifestEntry}, [][]byte{{0}, []byte("legacy"), []byte("spec")}, zip.Store)
	a, e := ReadArchive(b, DefaultArchiveLimits())
	if e != nil || string(a.Manifest) != "spec" || a.ManifestName != SpecManifestEntry {
		t.Fatal(a, e)
	}
	l := DefaultArchiveLimits()
	l.ManifestBytes = 3
	if _, e = ReadArchive(b, l); e == nil {
		t.Fatal("oversized preferred entry fell back")
	}
	l = DefaultArchiveLimits()
	l.PayloadBytes = 0
	if _, e = ReadArchive(b, l); e == nil {
		t.Fatal("payload limit")
	}
	l = DefaultArchiveLimits()
	l.ArchiveBytes = 1
	if _, e = WriteArchive(nil, nil, l); e == nil {
		t.Fatal("writer limit")
	}
	l = DefaultArchiveLimits()
	l.Entries = 1
	if _, e = WriteArchive(nil, nil, l); e == nil {
		t.Fatal("writer entry limit")
	}
}
func FuzzReadArchive(f *testing.F) {
	b, _ := WriteArchive([]byte{0, 255}, []byte(`{}`), DefaultArchiveLimits())
	f.Add(b)
	f.Fuzz(func(t *testing.T, b []byte) {
		if len(b) > 4096 {
			return
		}
		l := ArchiveLimits{4096, 2048, 1024, 8}
		a, e := ReadArchive(b, l)
		if e == nil && (len(a.Payload) > 2048 || len(a.Manifest) > 1024) {
			t.Fatal("bound")
		}
	})
}

// Solve the 32-bit affine CRC map for a four-byte payload, without using
// the implementation under test. This exercises signature/CRC ambiguity.
func payloadWithCRC(target uint32) []byte {
	zero := crc32.ChecksumIEEE(make([]byte, 4))
	var basis [32]uint32
	var masks [32]uint32
	for i := 0; i < 32; i++ {
		p := make([]byte, 4)
		p[i/8] = 1 << uint(i%8)
		v := crc32.ChecksumIEEE(p) ^ zero
		mask := uint32(1) << uint(i)
		for k := 31; k >= 0; k-- {
			if v&(uint32(1)<<uint(k)) == 0 {
				continue
			}
			if basis[k] != 0 {
				v ^= basis[k]
				mask ^= masks[k]
			} else {
				basis[k] = v
				masks[k] = mask
				break
			}
		}
	}
	v := target ^ zero
	mask := uint32(0)
	for k := 31; k >= 0; k-- {
		if v&(uint32(1)<<uint(k)) != 0 {
			v ^= basis[k]
			mask ^= masks[k]
		}
	}
	p := make([]byte, 4)
	le.PutUint32(p, mask)
	return p
}
func TestDescriptorCRCSameAsSignature(t *testing.T) {
	p := payloadWithCRC(0x08074b50)
	if crc32.ChecksumIEEE(p) != 0x08074b50 {
		t.Fatal("fixture")
	}
	for _, signed := range []bool{false, true} {
		b := zip64FixtureData(t, signed, p)
		a, e := ReadArchive(b, DefaultArchiveLimits())
		if e != nil || !bytes.Equal(a.Payload, p) {
			t.Fatal(signed, e)
		}
	}
}

func unsigned32(t *testing.T, payload []byte) []byte {
	b := stdArchive(t, []string{PayloadEntry, ManifestEntry}, [][]byte{payload, []byte(`{}`)}, zip.Store)
	cd := int(le.Uint32(b[len(b)-22+16:]))
	first := 30 + len(PayloadEntry) + len(payload)
	secondOffset := first + 16
	second := secondOffset + 30 + len(ManifestEntry) + 2
	if le.Uint32(b[first:]) != 0x08074b50 || le.Uint32(b[second:]) != 0x08074b50 {
		t.Fatal("descriptor positions")
	}
	out := append([]byte{}, b[:first]...)
	out = append(out, b[first+4:second]...)
	out = append(out, b[second+4:]...)
	cd -= 8
	secondCentral := cd + 46 + len(PayloadEntry)
	le.PutUint32(out[secondCentral+42:], uint32(secondOffset-4))
	le.PutUint32(out[len(out)-22+16:], uint32(cd))
	if crc32.ChecksumIEEE(payload) != 0x08074b50 {
		checkStd(t, out, map[string][]byte{PayloadEntry: payload, ManifestEntry: []byte(`{}`)})
	}
	return out
}
func TestUnsignedZIP32(t *testing.T) {
	for _, p := range [][]byte{{0, 255}, payloadWithCRC(0x08074b50)} {
		b := unsigned32(t, p)
		a, e := ReadArchive(b, DefaultArchiveLimits())
		if e != nil || !bytes.Equal(a.Payload, p) {
			t.Fatal(e)
		}
	}
}
func TestZIP64MalformedMetadata(t *testing.T) {
	b := zip64FixtureData(t, true, []byte{0, 255, 1, 0})
	z := len(b) - 22 - 20 - 56
	cd := int(le.Uint64(b[z+48:]))
	extra := 30 + len(PayloadEntry)
	centralExtra := cd + 46 + len(PayloadEntry)
	cases := map[string]func([]byte){"extraTruncation": func(x []byte) { le.PutUint16(x[extra+2:], 15) }, "hugeSize": func(x []byte) { x[centralExtra+4+4] = 1 }, "hugeCD": func(x []byte) { x[z+48+4] = 1 }, "missingLocator": func(x []byte) { x[len(x)-22-20] = 0 }, "disk": func(x []byte) { le.PutUint32(x[z+16:], 1) }, "endExtent": func(x []byte) { le.PutUint64(x[z+4:], 45) }, "countBound": func(x []byte) { le.PutUint64(x[z+24:], 65); le.PutUint64(x[z+32:], 65) }, "duplicateOffset": func(x []byte) {
		second := cd + 46 + len(PayloadEntry) + 28
		secondExtra := second + 46 + len(ManifestEntry)
		le.PutUint64(x[secondExtra+20:], 0)
	}}
	for name, fn := range cases {
		t.Run(name, func(t *testing.T) {
			bad := bytes.Clone(b)
			fn(bad)
			if _, e := ReadArchive(bad, DefaultArchiveLimits()); e == nil {
				t.Fatal("accepted")
			}
		})
	}
	for i := 0; i < len(b); i++ {
		if _, e := ReadArchive(b[:i], DefaultArchiveLimits()); e == nil {
			t.Fatalf("accepted ZIP64 truncation %d", i)
		}
	}
}

func TestCentralOrderIndependent(t *testing.T) {
	names := []string{PayloadEntry, SpecManifestEntry, ManifestEntry}
	data := [][]byte{{0, 255}, []byte("spec"), []byte("legacy")}
	b := stdArchive(t, names, data, zip.Store)
	cd := int(le.Uint32(b[len(b)-22+16:]))
	p := cd
	records := [][]byte{}
	for range names {
		n := 46 + int(le.Uint16(b[p+28:])) + int(le.Uint16(b[p+30:])) + int(le.Uint16(b[p+32:]))
		records = append(records, bytes.Clone(b[p:p+n]))
		p += n
	}
	reverse := append([]byte{}, b[:cd]...)
	for i := len(records) - 1; i >= 0; i-- {
		reverse = append(reverse, records[i]...)
	}
	reverse = append(reverse, b[p:]...)
	checkStd(t, reverse, map[string][]byte{PayloadEntry: data[0], SpecManifestEntry: data[1], ManifestEntry: data[2]})
	a, e := ReadArchive(reverse, DefaultArchiveLimits())
	if e != nil || string(a.Manifest) != "spec" || !bytes.Equal(a.Payload, data[0]) {
		t.Fatal(a, e)
	}
}
