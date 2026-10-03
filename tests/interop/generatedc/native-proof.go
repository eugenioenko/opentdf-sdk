package main

import (
	"crypto"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/hmac"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha1"
	"crypto/sha256"
	"crypto/x509"
	"crypto/x509/pkix"
	"encoding/asn1"
	"encoding/json"
	"encoding/pem"
	"fmt"
	nativecrypto "github.com/eugenioenko/goalchemy/lib/crypto"
	"math/big"
	"os"
	"path/filepath"
	"time"
)

func main() {
	dir, mode := os.Args[1], os.Args[2]
	write := func(name string, b []byte) {
		if e := os.WriteFile(filepath.Join(dir, name), b, 0600); e != nil {
			panic(e)
		}
	}
	read := func(name string) []byte {
		b, e := os.ReadFile(filepath.Join(dir, name))
		if e != nil {
			panic(e)
		}
		return b
	}
	msg := []byte("native Go/C interoperability\x00\xff")
	digest := sha256.Sum256(msg)
	if mode == "produce" {
		os.MkdirAll(dir, 0700)
		r, e := rsa.GenerateKey(rand.Reader, 2048)
		if e != nil {
			panic(e)
		}
		p, e := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
		if e != nil {
			panic(e)
		}
		q, e := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
		if e != nil {
			panic(e)
		}
		for name, key := range map[string]any{"rsa": r, "ec": p, "peer": q} {
			der, e := x509.MarshalPKCS8PrivateKey(key)
			if e != nil {
				panic(e)
			}
			write(name+".private.pem", pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: der}))
			var pub any
			switch key := key.(type) {
			case *rsa.PrivateKey:
				pub = &key.PublicKey
			case *ecdsa.PrivateKey:
				pub = &key.PublicKey
			}
			der, e = x509.MarshalPKIXPublicKey(pub)
			if e != nil {
				panic(e)
			}
			write(name+".public.pem", pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: der}))
		}
		write("rsa.pkcs1-private.pem", pem.EncodeToMemory(&pem.Block{Type: "RSA PRIVATE KEY", Bytes: x509.MarshalPKCS1PrivateKey(r)}))
		write("rsa.pkcs1-public.pem", pem.EncodeToMemory(&pem.Block{Type: "RSA PUBLIC KEY", Bytes: x509.MarshalPKCS1PublicKey(&r.PublicKey)}))
		tmpl := &x509.Certificate{SerialNumber: big.NewInt(1), Subject: pkix.Name{CommonName: "test"}, NotBefore: time.Unix(0, 0), NotAfter: time.Now().Add(time.Hour), KeyUsage: x509.KeyUsageDigitalSignature}
		cert, e := x509.CreateCertificate(rand.Reader, tmpl, tmpl, &r.PublicKey, r)
		if e != nil {
			panic(e)
		}
		write("rsa.cert.pem", pem.EncodeToMemory(&pem.Block{Type: "CERTIFICATE", Bytes: cert}))
		sig, e := rsa.SignPKCS1v15(rand.Reader, r, crypto.SHA256, digest[:])
		if e != nil {
			panic(e)
		}
		write("rsa.go.sig", sig)
		a, b, e := ecdsa.Sign(rand.Reader, p, digest[:])
		if e != nil {
			panic(e)
		}
		sig = make([]byte, 64)
		a.FillBytes(sig[:32])
		b.FillBytes(sig[32:])
		write("ec.go.sig", sig)
		ct, e := rsa.EncryptOAEP(sha1.New(), rand.Reader, &r.PublicKey, msg, nil)
		if e != nil {
			panic(e)
		}
		write("rsa.go.oaep", ct)
		ep, _ := p.ECDH()
		eq, _ := q.PublicKey.ECDH()
		shared, e := ep.ECDH(eq)
		if e != nil {
			panic(e)
		}
		write("ec.go.ecdh", shared)
		write("message", msg)
		write("hmac.empty", hmac.New(sha256.New, nil).Sum(nil))
		keyFixtures(dir, r, p, q)
		fmt.Println("native Go production fixtures ready")
		return
	}
	block, _ := pem.Decode(read("rsa.private.pem"))
	raw, e := x509.ParsePKCS8PrivateKey(block.Bytes)
	if e != nil {
		panic(e)
	}
	r := raw.(*rsa.PrivateKey)
	if e := rsa.VerifyPKCS1v15(&r.PublicKey, crypto.SHA256, digest[:], read("rsa.c.sig")); e != nil {
		panic(e)
	}
	b, _ := pem.Decode(read("ec.private.pem"))
	raw, e = x509.ParsePKCS8PrivateKey(b.Bytes)
	if e != nil {
		panic(e)
	}
	p := raw.(*ecdsa.PrivateKey)
	sig := read("ec.c.sig")
	if len(sig) != 64 || !ecdsa.Verify(&p.PublicKey, digest[:], new(big.Int).SetBytes(sig[:32]), new(big.Int).SetBytes(sig[32:])) {
		panic("C ES256")
	}
	pt, e := rsa.DecryptOAEP(sha1.New(), rand.Reader, r, read("rsa.c.oaep"), nil)
	if e != nil || string(pt) != string(msg) {
		panic("C OAEP")
	}
	mac := read("hmac.c.empty")
	expected := hmac.New(sha256.New, nil).Sum(nil)
	if !hmac.Equal(expected, mac) {
		panic("C empty-key/empty-message HMAC")
	}
	for i := range mac {
		changed := append([]byte(nil), mac...)
		changed[i] ^= 1
		if hmac.Equal(expected, changed) {
			panic("C changed HMAC accepted")
		}
	}
	fmt.Println("native Go verified C RS256, ES256 P1363, SHA1/MGF1SHA1 OAEP and empty-key/empty-message HMAC including 32 rejected mismatches")
}

func keyFixtures(dir string, r *rsa.PrivateKey, p, q *ecdsa.PrivateKey) {
	rows := []map[string]any{}
	fixture := func(name, label string, der []byte, valid bool) {
		data := pem.EncodeToMemory(&pem.Block{Type: label, Bytes: der})
		if e := os.WriteFile(filepath.Join(dir, name), data, 0600); e != nil {
			panic(e)
		}
		key, e := nativecrypto.ImportPEM(string(data))
		if (e == nil) != valid || (key != nil) != valid {
			panic("native Go fixture validity: " + name)
		}
		if key != nil {
			key.Close()
		}
		message := ""
		if e != nil {
			message = e.Error()
			if message != "crypto: invalid input or key" {
				panic(message)
			}
		}
		rows = append(rows, map[string]any{"fixture": name, "valid": valid, "nilKey": key == nil, "error": message})
	}
	marshal := func(v any) []byte {
		b, e := asn1.Marshal(v)
		if e != nil {
			panic(e)
		}
		return b
	}
	for name, exponent := range map[string]*big.Int{"rsa.exponent-one.pem": big.NewInt(1), "rsa.exponent-even.pem": big.NewInt(2), "rsa.exponent-overflow.pem": new(big.Int).Add(new(big.Int).Lsh(big.NewInt(1), 63), big.NewInt(1)), "rsa.exponent-three.pem": big.NewInt(3)} {
		fixture(name, "RSA PUBLIC KEY", marshal(struct{ N, E *big.Int }{r.N, exponent}), name == "rsa.exponent-three.pem")
	}
	rsaDER := x509.MarshalPKCS1PrivateKey(r)
	var rsaFields struct {
		Version               int
		N                     *big.Int
		E                     int
		D, P, Q, DP, DQ, QInv *big.Int
	}
	if _, e := asn1.Unmarshal(rsaDER, &rsaFields); e != nil {
		panic(e)
	}
	badD := rsaFields
	badD.D = new(big.Int).Add(rsaFields.D, big.NewInt(2))
	badD.DP = new(big.Int).Mod(badD.D, new(big.Int).Sub(rsaFields.P, big.NewInt(1)))
	badD.DQ = new(big.Int).Mod(badD.D, new(big.Int).Sub(rsaFields.Q, big.NewInt(1)))
	fixture("rsa.inconsistent-d.pem", "RSA PRIVATE KEY", marshal(badD), false)
	badP := rsaFields
	badP.P = new(big.Int).Add(rsaFields.P, big.NewInt(2))
	fixture("rsa.inconsistent-prime.pem", "RSA PRIVATE KEY", marshal(badP), false)
	pkcs8, e := x509.MarshalPKCS8PrivateKey(p)
	if e != nil {
		panic(e)
	}
	var outer struct {
		Version    int
		Algorithm  pkix.AlgorithmIdentifier
		PrivateKey []byte
	}
	if _, e = asn1.Unmarshal(pkcs8, &outer); e != nil {
		panic(e)
	}
	var inner struct {
		Version    int
		PrivateKey []byte
		NamedCurve asn1.ObjectIdentifier `asn1:"optional,explicit,tag:0"`
		PublicKey  asn1.BitString        `asn1:"optional,explicit,tag:1"`
	}
	if _, e = asn1.Unmarshal(outer.PrivateKey, &inner); e != nil {
		panic(e)
	}
	badEC := inner
	badEC.PrivateKey = q.D.FillBytes(make([]byte, 32))
	badOuter := outer
	badOuter.PrivateKey = marshal(badEC)
	scalarX, scalarY := elliptic.P256().ScalarBaseMult(q.D.Bytes())
	if scalarX.Cmp(q.X) != 0 || scalarY.Cmp(q.Y) != 0 || (p.X.Cmp(scalarX) == 0 && p.Y.Cmp(scalarY) == 0) {
		panic("expected independent EC points")
	}
	fixture("ec.inconsistent-point.pem", "PRIVATE KEY", marshal(badOuter), true)
	normalized, e := nativecrypto.ImportPEM(string(pem.EncodeToMemory(&pem.Block{Type: "PRIVATE KEY", Bytes: marshal(badOuter)})))
	if e != nil {
		panic(e)
	}
	normalizedPublic, e := normalized.PublicPEM()
	normalized.Close()
	if e != nil {
		panic(e)
	}
	expectedPublic, e := x509.MarshalPKIXPublicKey(&q.PublicKey)
	if e != nil {
		panic(e)
	}
	if normalizedPublic != string(pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: expectedPublic})) {
		panic("Go did not reconstruct Q from D")
	}
	rows[len(rows)-1]["encodedPointMismatch"] = true
	rows[len(rows)-1]["nativeGoPublicPointReconstructed"] = true
	rows[len(rows)-1]["independentScalarBaseMultMismatchProof"] = true
	rows[len(rows)-1]["nativeGoNormalization"] = "Go reconstructs public point from private scalar; C rejects inconsistent encoded point"
	zeroEC := inner
	zeroEC.PrivateKey = make([]byte, 32)
	zeroOuter := outer
	zeroOuter.PrivateKey = marshal(zeroEC)
	fixture("ec.zero-scalar.pem", "PRIVATE KEY", marshal(zeroOuter), false)
	inner.PublicKey = asn1.BitString{}
	outer.PrivateKey = marshal(inner)
	fixture("ec.omitted-point.pem", "PRIVATE KEY", marshal(outer), true)
	for name, key := range map[string]any{"rsa.valid-private.pem": r, "ec.valid-private.pem": p} {
		der, e := x509.MarshalPKCS8PrivateKey(key)
		if e != nil {
			panic(e)
		}
		fixture(name, "PRIVATE KEY", der, true)
	}
	b, e := json.MarshalIndent(rows, "", "  ")
	if e != nil {
		panic(e)
	}
	if e = os.WriteFile(filepath.Join(dir, "native-go-key-validity.json"), b, 0600); e != nil {
		panic(e)
	}
	fmt.Println("native Go proved six malformed key rejections, four ordinary valid imports, and D/Q decoder normalization")
}
