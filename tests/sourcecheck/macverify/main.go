package main

import "github.com/eugenioenko/goalchemy/lib/crypto"

func main() {
	ok, e := crypto.HMACSHA256Verify(nil, nil, make([]byte, 32))
	if e != nil || ok {
		panic("HMAC verification mismatch contract")
	}
}
