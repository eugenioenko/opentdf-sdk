package tdf

import "github.com/eugenioenko/goalchemy/lib/errors"

const PayloadEntry = "0.payload"
const ManifestEntry = "0.manifest.json"
const SpecManifestEntry = "manifest.json"

// ArchiveLimits are hard resource bounds applied before slicing or allocation.
// The initial implementation supports single-disk, stored, in-memory archives.
type ArchiveLimits struct {
	ArchiveBytes  int
	PayloadBytes  int
	ManifestBytes int
	Entries       int
}

func DefaultArchiveLimits() ArchiveLimits {
	return ArchiveLimits{128 * 1024 * 1024, 64 * 1024 * 1024, 10 * 1024 * 1024, 64}
}
func archiveLimitsOK(l ArchiveLimits) bool {
	return l.ArchiveBytes > 0 && l.ArchiveBytes <= 2147483647 && l.PayloadBytes >= 0 && l.ManifestBytes > 0 && l.Entries > 0 && l.Entries <= 65534
}

// Archive contains independent copies; CRC validation is not cryptographic integrity.
type Archive struct {
	Payload      []byte
	Manifest     []byte
	ManifestName string
}
type zipEntry struct {
	name   string
	flags  int
	crc    uint32
	size   int
	offset int
	end    int
	zip64  bool
	data   []byte
}

func span(b []byte, p int, n int) bool { return p >= 0 && n >= 0 && p <= len(b) && n <= len(b)-p }
func u16(b []byte, p int) int          { return int(b[p]) | int(b[p+1])<<8 }
func u32(b []byte, p int) uint32 {
	return uint32(b[p]) | uint32(b[p+1])<<8 | uint32(b[p+2])<<16 | uint32(b[p+3])<<24
}
func small64(b []byte, p int, max int) (int, error) {
	if !span(b, p, 8) || u32(b, p+4) != 0 || uint64(u32(b, p)) > uint64(max) {
		return 0, errors.New("zip: ZIP64 value exceeds bound")
	}
	return int(u32(b, p)), nil
}
func put16(b []byte, p int, n int) { b[p] = byte(n); b[p+1] = byte(n >> 8) }
func put32(b []byte, p int, n uint32) {
	b[p] = byte(n)
	b[p+1] = byte(n >> 8)
	b[p+2] = byte(n >> 16)
	b[p+3] = byte(n >> 24)
}
func crcTable() [256]uint32 {
	var t [256]uint32
	for i := 0; i < 256; i++ {
		c := uint32(i)
		for k := 0; k < 8; k++ {
			if c&1 != 0 {
				c = (c >> 1) ^ 0xedb88320
			} else {
				c >>= 1
			}
		}
		t[i] = c
	}
	return t
}

var zipCRC = crcTable()

func crc32Bytes(b []byte) uint32 {
	c := uint32(0xffffffff)
	for _, v := range b {
		c = (c >> 8) ^ zipCRC[byte(c)^v]
	}
	return ^c
}

// WriteArchive writes deterministic stored ZIP32 with the pinned legacy entry name.
func WriteArchive(payload []byte, manifest []byte, l ArchiveLimits) ([]byte, error) {
	if !archiveLimitsOK(l) || len(payload) > l.PayloadBytes || len(manifest) > l.ManifestBytes || l.Entries < 2 {
		return nil, errors.New("zip: resource limit")
	}
	overhead := 30 + len(PayloadEntry) + 30 + len(ManifestEntry) + 46 + len(PayloadEntry) + 46 + len(ManifestEntry) + 22
	if len(payload) > l.ArchiveBytes-overhead || len(manifest) > l.ArchiveBytes-overhead-len(payload) {
		return nil, errors.New("zip: archive limit")
	}
	entries := []zipEntry{{name: PayloadEntry, data: payload}, {name: ManifestEntry, data: manifest}}
	out := make([]byte, 0, len(payload)+len(manifest)+overhead)
	for i, e := range entries {
		e.offset = len(out)
		e.size = len(e.data)
		e.crc = crc32Bytes(e.data)
		h := make([]byte, 30)
		put32(h, 0, 0x04034b50)
		put16(h, 4, 20)
		put16(h, 6, 2048)
		put16(h, 12, 33)
		put32(h, 14, e.crc)
		put32(h, 18, uint32(e.size))
		put32(h, 22, uint32(e.size))
		put16(h, 26, len(e.name))
		out = append(out, h...)
		out = append(out, []byte(e.name)...)
		out = append(out, e.data...)
		entries[i] = e
	}
	cd := len(out)
	for _, e := range entries {
		h := make([]byte, 46)
		put32(h, 0, 0x02014b50)
		put16(h, 4, 20)
		put16(h, 6, 20)
		put16(h, 8, 2048)
		put16(h, 14, 33)
		put32(h, 16, e.crc)
		put32(h, 20, uint32(e.size))
		put32(h, 24, uint32(e.size))
		put16(h, 28, len(e.name))
		put32(h, 42, uint32(e.offset))
		out = append(out, h...)
		out = append(out, []byte(e.name)...)
	}
	h := make([]byte, 22)
	put32(h, 0, 0x06054b50)
	put16(h, 8, 2)
	put16(h, 10, 2)
	put32(h, 12, uint32(len(out)-cd))
	put32(h, 16, uint32(cd))
	out = append(out, h...)
	return out, nil
}

// extra64 reads required fields in APPNOTE order and rejects ambiguous duplicate tags.
func extra64(extra []byte, needSize bool, needCompressed bool, needOffset bool, needDisk bool, max int) (int, int, int, error) {
	size := 0
	compressed := 0
	offset := 0
	seen := false
	for p := 0; p < len(extra); {
		if !span(extra, p, 4) {
			return 0, 0, 0, errors.New("zip: truncated extra")
		}
		tag := u16(extra, p)
		n := u16(extra, p+2)
		p += 4
		if !span(extra, p, n) {
			return 0, 0, 0, errors.New("zip: truncated extra")
		}
		if tag == 1 {
			if seen {
				return 0, 0, 0, errors.New("zip: duplicate ZIP64 extra")
			}
			seen = true
			q := p
			stop := p + n
			var err error
			if needSize {
				if q+8 > stop {
					return 0, 0, 0, errors.New("zip: missing ZIP64 size")
				}
				size, err = small64(extra, q, max)
				if err != nil {
					return 0, 0, 0, err
				}
				q += 8
			}
			if needCompressed {
				if q+8 > stop {
					return 0, 0, 0, errors.New("zip: missing ZIP64 compressed size")
				}
				compressed, err = small64(extra, q, max)
				if err != nil {
					return 0, 0, 0, err
				}
				q += 8
			}
			if needOffset {
				if q+8 > stop {
					return 0, 0, 0, errors.New("zip: missing ZIP64 offset")
				}
				offset, err = small64(extra, q, max)
				if err != nil {
					return 0, 0, 0, err
				}
				q += 8
			}
			if needDisk && (q+4 > stop || u32(extra, q) != 0) {
				return 0, 0, 0, errors.New("zip: unsupported disk")
			}
		}
		p += n
	}
	if (needSize || needCompressed || needOffset || needDisk) && !seen {
		return 0, 0, 0, errors.New("zip: missing ZIP64 extra")
	}
	return size, compressed, offset, nil
}
func safe32(v uint32, max int) (int, error) {
	if uint64(v) > uint64(max) {
		return 0, errors.New("zip: size or offset limit")
	}
	return int(v), nil
}
func cloneBytes(b []byte) []byte { out := make([]byte, len(b)); copy(out, b); return out }

func ReadArchive(b []byte, l ArchiveLimits) (Archive, error) {
	if !archiveLimitsOK(l) || len(b) > l.ArchiveBytes || len(b) < 22 {
		return Archive{}, errors.New("zip: resource limit or truncated archive")
	}
	end := -1
	bottom := len(b) - 22 - 65535
	if bottom < 0 {
		bottom = 0
	}
	for p := len(b) - 22; p >= bottom; p-- {
		if u32(b, p) == 0x06054b50 && span(b, p, 22) && u16(b, p+20) == len(b)-p-22 {
			end = p
			break
		}
	}
	if end < 0 {
		return Archive{}, errors.New("zip: missing end record")
	}
	if u16(b, end+4) != 0 || u16(b, end+6) != 0 || u16(b, end+8) != u16(b, end+10) {
		return Archive{}, errors.New("zip: unsupported multi-disk")
	}
	count := u16(b, end+10)
	size32 := u32(b, end+12)
	offset32 := u32(b, end+16)
	cdEnd := end
	cdSize := 0
	cd := 0
	var err error
	locator := end >= 20 && u32(b, end-20) == 0x07064b50
	if locator {
		loc := end - 20
		if u32(b, loc+4) != 0 || u32(b, loc+16) != 1 {
			return Archive{}, errors.New("zip: unsupported ZIP64 disks")
		}
		z, e := small64(b, loc+8, len(b))
		if e != nil {
			return Archive{}, e
		}
		if !span(b, z, 56) || u32(b, z) != 0x06064b50 {
			return Archive{}, errors.New("zip: truncated ZIP64 end")
		}
		record, e := small64(b, z+4, len(b))
		if e != nil || record < 44 || !span(b, z+12, record) || z+12+record != loc {
			return Archive{}, errors.New("zip: invalid ZIP64 end extent")
		}
		if u32(b, z+16) != 0 || u32(b, z+20) != 0 {
			return Archive{}, errors.New("zip: unsupported ZIP64 disks")
		}
		count, e = small64(b, z+32, l.Entries)
		if e != nil {
			return Archive{}, e
		}
		diskCount, e := small64(b, z+24, l.Entries)
		if e != nil || diskCount != count {
			return Archive{}, errors.New("zip: inconsistent ZIP64 counts")
		}
		cdSize, e = small64(b, z+40, len(b))
		if e != nil {
			return Archive{}, e
		}
		cd, e = small64(b, z+48, len(b))
		if e != nil {
			return Archive{}, e
		}
		cdEnd = z
		if u16(b, end+10) != 65535 && u16(b, end+10) != count {
			return Archive{}, errors.New("zip: inconsistent counts")
		}
		if size32 != 0xffffffff && int(size32) != cdSize {
			return Archive{}, errors.New("zip: inconsistent directory size")
		}
		if offset32 != 0xffffffff && int(offset32) != cd {
			return Archive{}, errors.New("zip: inconsistent directory offset")
		}
	} else {
		if count == 65535 || size32 == 0xffffffff || offset32 == 0xffffffff {
			return Archive{}, errors.New("zip: missing ZIP64 locator")
		}
		cdSize, err = safe32(size32, len(b))
		if err != nil {
			return Archive{}, err
		}
		cd, err = safe32(offset32, len(b))
		if err != nil {
			return Archive{}, err
		}
	}
	if count < 2 || count > l.Entries || !span(b, cd, cdSize) || cd+cdSize != cdEnd {
		return Archive{}, errors.New("zip: invalid directory bounds")
	}
	entries := []zipEntry{}
	names := make(map[string]bool)
	p := cd
	for i := 0; i < count; i++ {
		if !span(b, p, 46) || p+46 > cdEnd || u32(b, p) != 0x02014b50 {
			return Archive{}, errors.New("zip: truncated central header")
		}
		flags := u16(b, p+8)
		if flags & ^(8|2048) != 0 {
			return Archive{}, errors.New("zip: unsupported flags")
		}
		if u16(b, p+10) != 0 {
			return Archive{}, errors.New("zip: unsupported compression")
		}
		if u16(b, p+6) > 45 {
			return Archive{}, errors.New("zip: unsupported extraction version")
		}
		n := u16(b, p+28)
		x := u16(b, p+30)
		comment := u16(b, p+32)
		record := 46 + n + x + comment
		if !span(b, p, record) || record > cdEnd-p || n == 0 || n > 1024 {
			return Archive{}, errors.New("zip: invalid central extent")
		}
		name := string(b[p+46 : p+46+n])
		if names[name] {
			return Archive{}, errors.New("zip: duplicate entry")
		}
		names[name] = true
		for k := 0; k < len(name); k++ {
			c := name[k]
			if c < 32 || c > 126 || c == '\\' {
				return Archive{}, errors.New("zip: unsupported entry name")
			}
		}
		if name[0] == '/' {
			return Archive{}, errors.New("zip: unsupported absolute name")
		}
		s32 := u32(b, p+24)
		c32 := u32(b, p+20)
		o32 := u32(b, p+42)
		disk := u16(b, p+34)
		sz, cs, off, e := extra64(b[p+46+n:p+46+n+x], s32 == 0xffffffff, c32 == 0xffffffff, o32 == 0xffffffff, disk == 65535, len(b))
		if e != nil {
			return Archive{}, e
		}
		if s32 != 0xffffffff {
			sz, e = safe32(s32, len(b))
			if e != nil {
				return Archive{}, e
			}
		}
		if c32 != 0xffffffff {
			cs, e = safe32(c32, len(b))
			if e != nil {
				return Archive{}, e
			}
		}
		if o32 != 0xffffffff {
			off, e = safe32(o32, len(b))
			if e != nil {
				return Archive{}, e
			}
		}
		if sz != cs || (disk != 0 && disk != 65535) {
			return Archive{}, errors.New("zip: invalid stored size or disk")
		}
		if name == PayloadEntry && sz > l.PayloadBytes {
			return Archive{}, errors.New("zip: payload limit")
		}
		if (name == ManifestEntry || name == SpecManifestEntry) && sz > l.ManifestBytes {
			return Archive{}, errors.New("zip: manifest limit")
		}
		entries = append(entries, zipEntry{name: name, flags: flags, crc: u32(b, p+16), size: sz, offset: off, zip64: s32 == 0xffffffff || c32 == 0xffffffff})
		p += record
	}
	if p != cdEnd {
		return Archive{}, errors.New("zip: directory count mismatch")
	}
	// Index physical records: linear traversal rejects duplicate offsets,
	// overlaps, gaps, and central records that alias a local entry.
	physical := make(map[int]zipEntry)
	for _, e := range entries {
		if _, ok := physical[e.offset]; ok {
			return Archive{}, errors.New("zip: duplicate local offset")
		}
		physical[e.offset] = e
	}
	next := 0
	result := Archive{}
	for i := 0; i < len(entries); i++ {
		e, ok := physical[next]
		if !ok {
			return Archive{}, errors.New("zip: local section gap or overlap")
		}
		p = e.offset
		if p != next || !span(b, p, 30) || p+30 > cd || u32(b, p) != 0x04034b50 {
			return Archive{}, errors.New("zip: invalid local offset")
		}
		if u16(b, p+4) > 45 || u16(b, p+6) != e.flags || u16(b, p+8) != 0 {
			return Archive{}, errors.New("zip: local metadata mismatch")
		}
		n := u16(b, p+26)
		x := u16(b, p+28)
		start := p + 30 + n + x
		if !span(b, p, 30+n+x) || start > cd || string(b[p+30:p+30+n]) != e.name {
			return Archive{}, errors.New("zip: local name mismatch")
		}
		s32 := u32(b, p+22)
		c32 := u32(b, p+18)
		sz, cs, _, er := extra64(b[p+30+n:start], s32 == 0xffffffff, c32 == 0xffffffff, false, false, len(b))
		if er != nil {
			return Archive{}, er
		}
		if s32 != 0xffffffff {
			sz = int(s32)
		}
		if c32 != 0xffffffff {
			cs = int(c32)
		}
		streamed := e.flags&8 != 0
		if !streamed {
			if sz != e.size || cs != e.size || u32(b, p+14) != e.crc {
				return Archive{}, errors.New("zip: local size or CRC mismatch")
			}
		} else {
			if (sz != 0 && sz != e.size) || (cs != 0 && cs != e.size) || (u32(b, p+14) != 0 && u32(b, p+14) != e.crc) {
				return Archive{}, errors.New("zip: streamed local mismatch")
			}
		}
		if !span(b, start, e.size) || e.size > cd-start {
			return Archive{}, errors.New("zip: truncated entry")
		}
		stop := start + e.size
		if streamed {
			wide := e.zip64 || s32 == 0xffffffff || c32 == 0xffffffff
			matches := 0
			descriptorEnd := 0
			for _, signed := range []bool{false, true} {
				q := stop
				if signed {
					if !span(b, q, 4) || u32(b, q) != 0x08074b50 {
						continue
					}
					q += 4
				}
				candidate, ok := descriptor(b, q, wide, e.size, e.crc, cd)
				if !ok {
					continue
				}
				_, atRecord := physical[candidate]
				if candidate != cd && !atRecord {
					continue
				}
				matches++
				descriptorEnd = candidate
			}
			if matches != 1 {
				return Archive{}, errors.New("zip: invalid or ambiguous descriptor")
			}
			stop = descriptorEnd
		}

		data := b[start : start+e.size]
		if crc32Bytes(data) != e.crc {
			return Archive{}, errors.New("zip: CRC mismatch")
		}
		next = stop
		if e.name == PayloadEntry {
			result.Payload = cloneBytes(data)
		}
		if e.name == SpecManifestEntry || (e.name == ManifestEntry && result.ManifestName != SpecManifestEntry) {
			result.Manifest = cloneBytes(data)
			result.ManifestName = e.name
		}
	}
	if next != cd || !names[PayloadEntry] || result.ManifestName == "" {
		return Archive{}, errors.New("zip: missing TDF entries or invalid extent")
	}
	return result, nil
}

func descriptor(b []byte, q int, wide bool, size int, crc uint32, cd int) (int, bool) {
	need := 12
	if wide {
		need = 20
	}
	if !span(b, q, need) || q+need > cd || u32(b, q) != crc {
		return 0, false
	}
	if wide {
		a, e := small64(b, q+4, len(b))
		if e != nil || a != size {
			return 0, false
		}
		a, e = small64(b, q+12, len(b))
		if e != nil || a != size {
			return 0, false
		}
	} else if int(u32(b, q+4)) != size || int(u32(b, q+8)) != size {
		return 0, false
	}
	return q + need, true
}
