package tdf

import (
	"github.com/eugenioenko/goalchemy/lib/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/io"
)

// The streaming archive of the reference's internal/zipstream: a Writer that
// emits the payload as it arrives (local header, data, data descriptor,
// manifest, central directory) and a Reader that locates the two entries
// from the central directory and reads payload ranges on demand.

const (
	sigLocalHeader     uint32 = 0x04034b50
	sigDataDescriptor  uint32 = 0x08074b50
	sigCentralHeader   uint32 = 0x02014b50
	sigEndRecord       uint32 = 0x06054b50
	sigZip64EndRecord  uint32 = 0x06064b50
	sigZip64Locator    uint32 = 0x07064b50
	zip64Marker        uint32 = 0xffffffff
	zip64Marker16             = 0xffff
	zipVersion45              = 45
	flagDataDescriptor        = 8
	zipDate19800101           = 33

	localHeaderSize   = 30
	centralHeaderSize = 46
	endRecordSize     = 22
	zip64EndSize      = 56
	zip64LocatorSize  = 20
	zip64LocalExtra   = 20
	zip64CentralExtra = 28

	// MaxNonZip64 is the largest size or offset written into a 32-bit field
	// before an entry switches to ZIP64: 2 GiB - 1 rather than 4 GiB - 1, as
	// in the reference, so readers that widen the fields with a signed read
	// still open archives in the 2-4 GiB band.
	MaxNonZip64 int64 = 2147483647
	maxInt64    int64 = 9223372036854775807
)

var errArchiveFinished = errors.New("zip: archive already finished")

func put64(b []byte, p int, v int64) {
	put32(b, p, uint32(v))
	put32(b, p+4, uint32(v>>32))
}

// Writer streams one TDF archive to out. WritePayload appends payload bytes
// as they arrive and Finish writes the manifest and the directory. With zip64
// set the payload's local header carries ZIP64 fields from the start, for a
// payload whose size is unknown or may exceed the 32-bit fields; otherwise
// ZIP64 structures appear only where a size or offset needs them.
type Writer struct {
	out      io.Writer
	zip64    bool
	started  bool
	finished bool
	size     int64
	crc      uint32
	written  int64
}

// NewWriter streams a ZIP archive to out. With zip64 set every entry is
// written in ZIP64 form from its first header, which is what an input of
// unknown size needs; otherwise entries start as ZIP32 and switch to ZIP64 only
// when they reach MaxNonZip64 (2 GiB).
func NewWriter(out io.Writer, zip64 bool) *Writer {
	return &Writer{out: out, zip64: zip64}
}

func (w *Writer) write(b []byte) error {
	n, err := w.out.Write(b)
	if err != nil {
		return err
	}
	if n != len(b) {
		return io.ErrShortWrite
	}
	w.written += int64(n)
	return nil
}

func (w *Writer) writePayloadHeader() error {
	h := make([]byte, localHeaderSize)
	put32(h, 0, sigLocalHeader)
	put16(h, 4, zipVersion45)
	put16(h, 6, flagDataDescriptor)
	put16(h, 12, zipDate19800101)
	put16(h, 26, len(PayloadEntry))
	if w.zip64 {
		put32(h, 18, zip64Marker)
		put32(h, 22, zip64Marker)
		put16(h, 28, zip64LocalExtra)
	}
	h = append(h, []byte(PayloadEntry)...)
	if w.zip64 {
		extra := make([]byte, zip64LocalExtra)
		put16(extra, 0, 1)
		put16(extra, 2, zip64LocalExtra-4)
		h = append(h, extra...)
	}
	w.started = true
	return w.write(h)
}

// WritePayload appends data to the payload entry.
func (w *Writer) WritePayload(data []byte) error {
	if w.finished {
		return errArchiveFinished
	}
	if !w.started {
		if err := w.writePayloadHeader(); err != nil {
			return err
		}
	}
	if len(data) == 0 {
		return nil
	}
	if int64(len(data)) > maxInt64-w.size {
		return errors.New("zip: payload too large")
	}
	w.crc = crc32Combine(w.crc, crc32Bytes(data), int64(len(data)))
	w.size += int64(len(data))
	return w.write(data)
}

type entry struct {
	name   string
	offset int64
	size   int64
	crc    uint32
	flags  int
}

// Finish writes the payload's data descriptor, the manifest entry and the
// central directory, and returns the archive's total size.
func (w *Writer) Finish(manifest []byte) (int64, error) {
	if w.finished {
		return 0, errArchiveFinished
	}
	if !w.started {
		if err := w.writePayloadHeader(); err != nil {
			return 0, err
		}
	}
	wide := w.zip64 || w.size > MaxNonZip64
	var d []byte
	if wide {
		d = make([]byte, 24)
		put32(d, 0, sigDataDescriptor)
		put32(d, 4, w.crc)
		put64(d, 8, w.size)
		put64(d, 16, w.size)
	} else {
		d = make([]byte, 16)
		put32(d, 0, sigDataDescriptor)
		put32(d, 4, w.crc)
		put32(d, 8, uint32(w.size))
		put32(d, 12, uint32(w.size))
	}
	if err := w.write(d); err != nil {
		return 0, err
	}
	manifestEntry := entry{name: ManifestEntry, offset: w.written, size: int64(len(manifest)), crc: crc32Bytes(manifest)}
	h := make([]byte, localHeaderSize)
	put32(h, 0, sigLocalHeader)
	put16(h, 4, zipVersion45)
	put16(h, 12, zipDate19800101)
	put32(h, 14, manifestEntry.crc)
	put32(h, 18, uint32(manifestEntry.size))
	put32(h, 22, uint32(manifestEntry.size))
	put16(h, 26, len(ManifestEntry))
	h = append(h, []byte(ManifestEntry)...)
	if err := w.write(h); err != nil {
		return 0, err
	}
	if err := w.write(manifest); err != nil {
		return 0, err
	}
	entries := []entry{{name: PayloadEntry, offset: 0, size: w.size, crc: w.crc, flags: flagDataDescriptor}, manifestEntry}
	cdOffset := w.written
	cdZip64 := wide || cdOffset > MaxNonZip64
	for _, e := range entries {
		useZip64 := cdZip64 || e.size > MaxNonZip64 || e.offset > MaxNonZip64
		c := make([]byte, centralHeaderSize)
		put32(c, 0, sigCentralHeader)
		put16(c, 4, zipVersion45)
		put16(c, 6, zipVersion45)
		put16(c, 8, e.flags)
		put16(c, 14, zipDate19800101)
		put32(c, 16, e.crc)
		put16(c, 28, len(e.name))
		if useZip64 {
			put32(c, 20, zip64Marker)
			put32(c, 24, zip64Marker)
			put32(c, 42, zip64Marker)
			put16(c, 30, zip64CentralExtra)
		} else {
			put32(c, 20, uint32(e.size))
			put32(c, 24, uint32(e.size))
			put32(c, 42, uint32(e.offset))
		}
		c = append(c, []byte(e.name)...)
		if useZip64 {
			extra := make([]byte, zip64CentralExtra)
			put16(extra, 0, 1)
			put16(extra, 2, zip64CentralExtra-4)
			put64(extra, 4, e.size)
			put64(extra, 12, e.size)
			put64(extra, 20, e.offset)
			c = append(c, extra...)
		}
		if err := w.write(c); err != nil {
			return 0, err
		}
	}
	cdSize := w.written - cdOffset
	if cdZip64 {
		z := make([]byte, zip64EndSize)
		put32(z, 0, sigZip64EndRecord)
		put64(z, 4, zip64EndSize-12)
		put16(z, 12, zipVersion45)
		put16(z, 14, zipVersion45)
		put64(z, 24, int64(len(entries)))
		put64(z, 32, int64(len(entries)))
		put64(z, 40, cdSize)
		put64(z, 48, cdOffset)
		zip64EndOffset := w.written
		if err := w.write(z); err != nil {
			return 0, err
		}
		l := make([]byte, zip64LocatorSize)
		put32(l, 0, sigZip64Locator)
		put64(l, 8, zip64EndOffset)
		put32(l, 16, 1)
		if err := w.write(l); err != nil {
			return 0, err
		}
	}
	e := make([]byte, endRecordSize)
	put32(e, 0, sigEndRecord)
	if cdZip64 {
		put16(e, 8, zip64Marker16)
		put16(e, 10, zip64Marker16)
		put32(e, 12, zip64Marker)
		put32(e, 16, zip64Marker)
	} else {
		put16(e, 8, len(entries))
		put16(e, 10, len(entries))
		put32(e, 12, uint32(cdSize))
		put32(e, 16, uint32(cdOffset))
	}
	if err := w.write(e); err != nil {
		return 0, err
	}
	w.finished = true
	return w.written, nil
}

// crc32Combine returns the CRC-32 of two concatenated parts from the parts'
// CRCs and the second part's length (zlib's crc32_combine).
func crc32Combine(crc1 uint32, crc2 uint32, len2 int64) uint32 {
	if len2 <= 0 {
		return crc1
	}
	even := make([]uint32, 32)
	odd := make([]uint32, 32)
	odd[0] = 0xedb88320
	row := uint32(1)
	for n := 1; n < 32; n++ {
		odd[n] = row
		row <<= 1
	}
	gf2MatrixSquare(even, odd)
	gf2MatrixSquare(odd, even)
	for {
		gf2MatrixSquare(even, odd)
		if len2&1 != 0 {
			crc1 = gf2MatrixTimes(even, crc1)
		}
		len2 >>= 1
		if len2 == 0 {
			break
		}
		gf2MatrixSquare(odd, even)
		if len2&1 != 0 {
			crc1 = gf2MatrixTimes(odd, crc1)
		}
		len2 >>= 1
		if len2 == 0 {
			break
		}
	}
	return crc1 ^ crc2
}

func gf2MatrixTimes(mat []uint32, vec uint32) uint32 {
	var sum uint32
	i := 0
	for vec != 0 {
		if vec&1 != 0 {
			sum ^= mat[i]
		}
		vec >>= 1
		i++
	}
	return sum
}

func gf2MatrixSquare(square []uint32, mat []uint32) {
	for n := 0; n < 32; n++ {
		square[n] = gf2MatrixTimes(mat, mat[n])
	}
}

// Reader locates the payload and manifest entries of a TDF archive from its
// central directory and reads payload ranges from the underlying stream on
// demand. The manifest is read once, bounded by the limits; the payload is
// never held whole.
type Reader struct {
	src          io.ReadSeeker
	payloadStart int64
	payloadSize  int64
	manifest     []byte
	manifestName string
}

var errZipFormat = errors.New("zip: not a valid zip file")

func zipFormatError(what string) error { return fmt.Errorf("%w: %s", errZipFormat, what) }

func readAt(src io.ReadSeeker, offset int64, size int64) ([]byte, error) {
	if offset < 0 || size < 0 {
		return nil, zipFormatError("negative offset or size")
	}
	if _, err := src.Seek(offset, io.SeekStart); err != nil {
		return nil, fmt.Errorf("readSeeker.Seek failed: %w", err)
	}
	buf := make([]byte, size)
	if _, err := io.ReadFull(src, buf); err != nil {
		if errors.Is(err, io.EOF) {
			err = io.ErrUnexpectedEOF
		}
		return nil, fmt.Errorf("reading %d bytes at %d failed: %w", size, offset, err)
	}
	return buf, nil
}

// get64 reads a little-endian 64-bit field that must fit a non-negative int64.
func get64(b []byte, p int) (int64, error) {
	hi := u32(b, p+4)
	if hi&0x80000000 != 0 {
		return 0, zipFormatError("ZIP64 value exceeds the supported range")
	}
	return int64(hi)<<32 | int64(u32(b, p)), nil
}

// centralExtra64 returns the size and local header offset of a central
// directory entry whose 32-bit fields carry the ZIP64 marker, from the ZIP64
// extended information field in its extra area.
func centralExtra64(extra []byte, size32 uint32, compressed32 uint32, offset32 uint32) (int64, int64, int64, error) {
	size := int64(size32)
	compressed := int64(compressed32)
	offset := int64(offset32)
	needed := size32 == zip64Marker || compressed32 == zip64Marker || offset32 == zip64Marker
	if !needed {
		return size, compressed, offset, nil
	}
	for p := 0; p+4 <= len(extra); {
		tag := u16(extra, p)
		n := u16(extra, p+2)
		p += 4
		if n > len(extra)-p {
			return 0, 0, 0, zipFormatError("truncated extra field")
		}
		if tag != 1 {
			p += n
			continue
		}
		q := p
		stop := p + n
		var err error
		if size32 == zip64Marker {
			if q+8 > stop {
				return 0, 0, 0, zipFormatError("missing ZIP64 size")
			}
			if size, err = get64(extra, q); err != nil {
				return 0, 0, 0, err
			}
			q += 8
		}
		if compressed32 == zip64Marker {
			if q+8 > stop {
				return 0, 0, 0, zipFormatError("missing ZIP64 compressed size")
			}
			if compressed, err = get64(extra, q); err != nil {
				return 0, 0, 0, err
			}
			q += 8
		}
		if offset32 == zip64Marker {
			if q+8 > stop {
				return 0, 0, 0, zipFormatError("missing ZIP64 offset")
			}
			if offset, err = get64(extra, q); err != nil {
				return 0, 0, 0, err
			}
		}
		return size, compressed, offset, nil
	}
	return 0, 0, 0, zipFormatError("missing ZIP64 extra field")
}

// NewReader reads the central directory of the archive on src and locates
// its TDF entries. l.Entries bounds the directory and l.ManifestBytes the
// manifest; the payload may be any size.
func NewReader(src io.ReadSeeker, l ArchiveLimits) (*Reader, error) {
	if !archiveLimitsOK(l) {
		return nil, errors.New("zip: resource limit")
	}
	endOffset, err := src.Seek(-endRecordSize, io.SeekEnd)
	if err != nil {
		return nil, fmt.Errorf("readSeeker.Seek failed: %w", err)
	}
	if endOffset < 0 {
		return nil, zipFormatError("truncated archive")
	}
	end, err := readAt(src, endOffset, endRecordSize)
	if err != nil {
		return nil, err
	}
	if u32(end, 0) != sigEndRecord {
		return nil, zipFormatError("missing end of central directory record")
	}
	if u16(end, 4) != 0 || u16(end, 6) != 0 || u16(end, 8) != u16(end, 10) {
		return nil, zipFormatError("multi-disk archives are not supported")
	}
	count := int64(u16(end, 10))
	cdSize := int64(u32(end, 12))
	cdOffset := int64(u32(end, 16))
	directoryEnd := endOffset
	if count == zip64Marker16 || u32(end, 12) == zip64Marker || u32(end, 16) == zip64Marker {
		locatorOffset := endOffset - zip64LocatorSize
		if locatorOffset < 0 {
			return nil, zipFormatError("missing ZIP64 locator")
		}
		loc, err := readAt(src, locatorOffset, zip64LocatorSize)
		if err != nil {
			return nil, err
		}
		if u32(loc, 0) != sigZip64Locator {
			return nil, zipFormatError("missing ZIP64 locator")
		}
		if u32(loc, 4) != 0 {
			return nil, zipFormatError("multi-disk archives are not supported")
		}
		zip64End, err := get64(loc, 8)
		if err != nil {
			return nil, err
		}
		if zip64End > locatorOffset-zip64EndSize {
			return nil, zipFormatError("ZIP64 end record lies past the locator")
		}
		z, err := readAt(src, zip64End, zip64EndSize)
		if err != nil {
			return nil, err
		}
		if u32(z, 0) != sigZip64EndRecord {
			return nil, zipFormatError("missing ZIP64 end of central directory record")
		}
		if u32(z, 16) != 0 || u32(z, 20) != 0 {
			return nil, zipFormatError("multi-disk archives are not supported")
		}
		if count, err = get64(z, 24); err != nil {
			return nil, err
		}
		total, err := get64(z, 32)
		if err != nil {
			return nil, err
		}
		if total != count {
			return nil, zipFormatError("inconsistent entry counts")
		}
		if cdSize, err = get64(z, 40); err != nil {
			return nil, err
		}
		if cdOffset, err = get64(z, 48); err != nil {
			return nil, err
		}
		directoryEnd = zip64End
	}
	if count < 2 || count > int64(l.Entries) {
		return nil, zipFormatError("unexpected entry count")
	}
	if cdOffset > directoryEnd || cdSize > directoryEnd-cdOffset {
		return nil, zipFormatError("central directory lies past the archive")
	}
	r := &Reader{src: src, payloadSize: -1}
	next := cdOffset
	for i := int64(0); i < count; i++ {
		if next > cdOffset+cdSize-centralHeaderSize {
			return nil, zipFormatError("truncated central directory")
		}
		c, err := readAt(src, next, centralHeaderSize)
		if err != nil {
			return nil, err
		}
		if u32(c, 0) != sigCentralHeader {
			return nil, zipFormatError("invalid central directory header")
		}
		if u16(c, 10) != 0 {
			return nil, errors.New("zip: unsupported compression")
		}
		nameLen := u16(c, 28)
		extraLen := u16(c, 30)
		commentLen := u16(c, 32)
		record := int64(centralHeaderSize + nameLen + extraLen + commentLen)
		if nameLen == 0 || record > cdOffset+cdSize-next {
			return nil, zipFormatError("invalid central directory entry")
		}
		nameAndExtra, err := readAt(src, next+centralHeaderSize, int64(nameLen+extraLen))
		if err != nil {
			return nil, err
		}
		name := string(nameAndExtra[:nameLen])
		size, compressed, offset, err := centralExtra64(nameAndExtra[nameLen:], u32(c, 24), u32(c, 20), u32(c, 42))
		if err != nil {
			return nil, err
		}
		if size != compressed {
			return nil, zipFormatError("stored entry sizes differ")
		}
		next += record
		isPayload := name == PayloadEntry
		isManifest := name == SpecManifestEntry || (name == ManifestEntry && r.manifestName != SpecManifestEntry)
		if !isPayload && !isManifest {
			continue
		}
		if offset > cdOffset-localHeaderSize {
			return nil, zipFormatError("local header lies past the central directory")
		}
		h, err := readAt(src, offset, localHeaderSize)
		if err != nil {
			return nil, err
		}
		if u32(h, 0) != sigLocalHeader {
			return nil, errors.New("zip: unable to read local file header")
		}
		dataStart := offset + localHeaderSize + int64(u16(h, 26)) + int64(u16(h, 28))
		if dataStart > cdOffset || size > cdOffset-dataStart {
			return nil, zipFormatError("entry data lies past the central directory")
		}
		if isPayload {
			r.payloadStart = dataStart
			r.payloadSize = size
		} else {
			if size > int64(l.ManifestBytes) {
				return nil, fmt.Errorf("%s size too large: %d KiB", name, size/1024)
			}
			if r.manifest, err = readAt(src, dataStart, size); err != nil {
				return nil, err
			}
			r.manifestName = name
		}
	}
	if r.payloadSize < 0 {
		return nil, errors.New("zip: no " + PayloadEntry + " entry")
	}
	if r.manifestName == "" {
		return nil, errors.New("zip: no " + SpecManifestEntry + " or " + ManifestEntry + " entry")
	}
	return r, nil
}

// Manifest returns the manifest entry's bytes.
func (r *Reader) Manifest() []byte { return r.manifest }

// ManifestName returns the name of the manifest entry that was read.
func (r *Reader) ManifestName() string { return r.manifestName }

// PayloadSize returns the stored size of the payload entry.
func (r *Reader) PayloadSize() int64 { return r.payloadSize }

// ReadPayload returns length payload bytes from offset.
func (r *Reader) ReadPayload(offset int64, length int64) ([]byte, error) {
	if length < 0 || length > r.payloadSize || offset < 0 || offset > r.payloadSize-length {
		return nil, fmt.Errorf("zip: not a valid file size: %s: read of %d bytes at %d does not fit an entry of %d bytes", PayloadEntry, length, offset, r.payloadSize)
	}
	return readAt(r.src, r.payloadStart+offset, length)
}
