package main

import (
	"github.com/eugenioenko/goalchemy/lib/clock"
	"github.com/eugenioenko/goalchemy/lib/context"
	"opentdf-local/sdk/src"
	"opentdf-local/sdk/src/tdf"
)

func main() {
	client, e := sdk.New(sdk.Config{PlatformURL: "https://platform.example", DPoP: true, KASAlgorithm: "ec:secp256r1", SessionAlgorithm: "ec:secp256r1", TokenProvider: func(ctx context.Context) (sdk.AccessToken, error) {
		return sdk.AccessToken{Value: "host-token", Scheme: "DPoP", ConfirmationJKT: "host-attested-thumbprint", ExpiresAt: clock.Unix() + 60}, nil
	}})
	if e != nil {
		panic(e)
	}
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	_, e = client.Create(ctx, []byte{0, 255}, tdf.EncryptConfig{})
	if e == nil {
		panic("canceled create")
	}
	client.Close()
	println("client probe passed")
}
