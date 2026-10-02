package main

import "github.com/eugenioenko/goalchemy/lib/crypto"

// A type-only capability reference still needs a target implementation.
func main() { var key *crypto.Key; println(key == nil) }
