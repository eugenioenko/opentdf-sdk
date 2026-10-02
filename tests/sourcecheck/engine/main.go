package main

import (
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"opentdf-local/sdk/tdf"
)

func probe(ec bool) {
	var key *crypto.Key
	var e error
	if ec {
		key, e = crypto.GenerateP256()
	} else {
		key, e = crypto.GenerateRSA2048()
	}
	if e != nil {
		panic(e)
	}
	config := tdf.EncryptConfig{KASPublicKey: key, KASURL: "https://kas.example", SegmentSize: 16384, Attributes: []string{"attribute"}, Metadata: []byte{0, 255, 128}, SegmentHashAlgorithm: "HS256"}
	if ec {
		config.Algorithm = "ec:secp256r1"
	}
	plain := make([]byte, 16385)
	for i := range plain {
		plain[i] = byte(i)
	}
	data, e := tdf.Encrypt(plain, config)
	if e != nil {
		panic(e)
	}
	a, e := tdf.ReadArchive(data, tdf.DefaultArchiveLimits())
	if e != nil {
		panic(e)
	}
	m, e := tdf.ParseManifest(a.Manifest)
	if e != nil {
		panic(e)
	}
	kao := m.Encryption.KeyAccess[0]
	wrapped, e := encoding.Base64Decode(kao.WrappedKey)
	if e != nil {
		panic(e)
	}
	var payloadKey []byte
	if ec {
		public, e := crypto.ImportPEM(kao.EphemeralPublicKey)
		if e != nil {
			panic(e)
		}
		secret, e := crypto.ECDH(key, public)
		public.Close()
		if e != nil {
			panic(e)
		}
		salt, e := crypto.SHA256([]byte("TDF"))
		if e != nil {
			panic(e)
		}
		wrapKey, e := crypto.HKDFSHA256(secret, salt, nil, 32)
		if e != nil {
			panic(e)
		}
		payloadKey, e = crypto.AES256GCMDecrypt(wrapKey, wrapped[:12], wrapped[12:], nil)
		if e != nil {
			panic(e)
		}
	} else {
		payloadKey, e = crypto.RSAOAEPDecrypt(key, wrapped)
		if e != nil {
			panic(e)
		}
	}
	key.Close()
	result, e := tdf.DecryptWithPayloadKey(data, payloadKey)
	if e != nil {
		panic(e)
	}
	if len(result.Payload) != len(plain) || len(result.Metadata) != 3 || result.Metadata[1] != 255 {
		panic("output shape")
	}
	for i := range plain {
		if result.Payload[i] != plain[i] {
			panic("plaintext")
		}
	}
	a.Payload[len(a.Payload)-17] ^= 1
	manifest, e := m.Marshal()
	if e != nil {
		panic(e)
	}
	bad, e := tdf.WriteArchive(a.Payload, manifest, tdf.DefaultArchiveLimits())
	if e != nil {
		panic(e)
	}
	failed, e := tdf.DecryptWithPayloadKey(bad, payloadKey)
	if e == nil || len(failed.Payload) != 0 || len(failed.Metadata) != 0 {
		panic("failed open")
	}
}
func main() { probe(false); probe(true); println("engine probe passed") }
