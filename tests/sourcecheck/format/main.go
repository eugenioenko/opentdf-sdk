package main

import (
	"opentdf-local/sdk/tdf"
	j "opentdf-local/sdk/tdf/json"
)

func main() {
	v, e := j.Parse([]byte(`{"unicode":"\ud83d\ude03é","bytes":"\u0000","n":12,"array":[false,null]}`), j.DefaultLimits())
	if e != nil {
		panic(e)
	}
	s, ok := v.Get("unicode")
	if !ok || s.Text != "😃é" {
		panic("unicode")
	}
	out, e := j.Marshal(v, j.DefaultLimits())
	if e != nil {
		panic(e)
	}
	v, e = j.Parse(out, j.DefaultLimits())
	if e != nil {
		panic(e)
	}
	s, _ = v.Get("bytes")
	if s.Text != "\x00" {
		panic("NUL")
	}
	payload := []byte{0, 255, 128, 7}
	archive, e := tdf.WriteArchive(payload, out, tdf.DefaultArchiveLimits())
	if e != nil {
		panic(e)
	}
	read, e := tdf.ReadArchive(archive, tdf.DefaultArchiveLimits())
	if e != nil || len(read.Payload) != 4 || read.Payload[1] != 255 {
		panic("ZIP")
	}
	if _, e = j.Parse([]byte(`{"a":1,"a":2}`), j.DefaultLimits()); e == nil {
		panic("duplicate")
	}
	p, e := tdf.ParsePolicy([]byte(`{"uuid":"p","body":{"dataAttributes":[{"attribute":"a"}],"dissem":[]}}`))
	if e != nil || p.ValidateModern() != nil {
		panic("policy")
	}
	out, e = p.Marshal()
	if e != nil {
		panic(e)
	}
	p, e = tdf.ParsePolicy(out)
	if e != nil || p.UUID != "p" {
		panic("policy roundtrip")
	}
	m, e := tdf.ParseManifest([]byte(`{"schemaVersion":"4.3.0","payload":{"type":"reference","url":"0.payload","protocol":"zip","isEncrypted":true},"encryptionInformation":{"type":"split","policy":"exact original","keyAccess":[{"type":"wrapped","url":"https://kas.test","protocol":"kas","wrappedKey":"w","policyBinding":{"alg":"HS256","hash":"h"},"sid":"s"}],"method":{"algorithm":"AES-256-GCM","iv":"","isStreamable":true},"integrityInformation":{"rootSignature":{"alg":"HS256","sig":"s"},"segmentHashAlg":"GMAC","segmentSizeDefault":1024,"encryptedSegmentSizeDefault":1052,"segments":[{"hash":"h","segmentSize":0,"encryptedSegmentSize":28}]}}}`))
	if e != nil {
		panic(e)
	}
	out, e = m.Marshal()
	if e != nil {
		panic(e)
	}
	m, e = tdf.ParseManifest(out)
	if e != nil || m.Encryption.Policy != "exact original" || m.Encryption.KeyAccess[0].SID != "s" {
		panic("manifest")
	}
	plain, encrypted, e := m.Encryption.Integrity.ResolveSegment(m.Encryption.Integrity.Segments[0])
	if e != nil || plain != 0 || encrypted != 28 {
		panic("segment")
	}
	println("format probe passed")
}
