package main

import (
	"github.com/eugenioenko/goalchemy/lib/clock"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
)

func main() {
	digest, err := crypto.SHA256([]byte("abc"))
	if err != nil {
		panic(err)
	}
	encoded, err := encoding.Base64URLEncode(digest)
	if err != nil {
		panic(err)
	}
	if encoded != "ungWv48Bz-pBQUDeXa4iI7ADYaOWF3qctBD_YfIAFa0" {
		panic("SHA256 vector")
	}
	key, err := crypto.GenerateP256()
	if err != nil {
		panic(err)
	}
	defer key.Close()
	signature, err := crypto.ES256Sign(key, []byte("probe"))
	if err != nil {
		panic(err)
	}
	ok, err := crypto.ES256Verify(key, []byte("probe"), signature)
	if err != nil || !ok {
		panic("signature")
	}
	fields, err := key.PublicJWK()
	if err != nil || fields[0] != "EC" || fields[1] != "P-256" {
		panic("JWK")
	}

	rsa, err := crypto.GenerateRSA2048()
	if err != nil {
		panic(err)
	}
	defer rsa.Close()
	publicPEM, err := rsa.PublicPEM()
	if err != nil {
		panic(err)
	}
	pub, err := crypto.ImportPEM(publicPEM)
	if err != nil {
		panic(err)
	}
	defer pub.Close()
	privatePEM, err := rsa.PrivatePEM()
	if err != nil {
		panic(err)
	}
	priv, err := crypto.ImportPEM(privatePEM)
	if err != nil {
		panic(err)
	}
	defer priv.Close()
	share, err := crypto.Random(32)
	if err != nil {
		panic(err)
	}
	ct, err := crypto.RSAOAEPEncrypt(pub, share)
	if err != nil {
		panic(err)
	}
	pt, err := crypto.RSAOAEPDecrypt(priv, ct)
	if err != nil || len(pt) != 32 {
		panic("RSA unwrap")
	}
	for i := 0; i < 32; i++ {
		if share[i] != pt[i] {
			panic("RSA plaintext")
		}
	}
	sig, err := crypto.RS256Sign(priv, share)
	if err != nil {
		panic(err)
	}
	valid, err := crypto.RS256Verify(pub, share, sig)
	if err != nil || !valid {
		panic("RS256")
	}
	mac, err := crypto.HMACSHA256(share, []byte("probe"))
	if err != nil || len(mac) != 32 {
		panic("HMAC")
	}
	derived, err := crypto.HKDFSHA256(share, nil, nil, 32)
	if err != nil {
		panic(err)
	}
	nonce := make([]byte, 12)
	encrypted, err := crypto.AES256GCMEncrypt(derived, nonce, share, nil)
	if err != nil {
		panic(err)
	}
	plain, err := crypto.AES256GCMDecrypt(derived, nonce, encrypted, nil)
	if err != nil || len(plain) != 32 {
		panic("AES")
	}
	for i := 0; i < 32; i++ {
		if share[i] != plain[i] {
			panic("AES plaintext")
		}
	}
	shared, err := crypto.ECDH(key, key)
	if err != nil || len(shared) != 32 {
		panic("ECDH")
	}
	standard, err := encoding.Base64Encode(share)
	if err != nil {
		panic(err)
	}
	decoded, err := encoding.Base64Decode(standard)
	if err != nil || len(decoded) != 32 {
		panic("base64")
	}
	urlEncoding, err := encoding.Base64URLEncode(share)
	if err != nil {
		panic(err)
	}
	urlDecoded, err := encoding.Base64URLDecode(urlEncoding)
	if err != nil || len(urlDecoded) != 32 {
		panic("base64url")
	}
	if clock.Unix() < 1700000000 {
		panic("wall clock")
	}
	println("native capability probe passed")
}
