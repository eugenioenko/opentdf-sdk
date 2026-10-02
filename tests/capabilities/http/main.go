package main

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/http"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	status, headers, body, err := http.Do(ctx, "GET", "http://localhost:8080/healthz", nil, nil, 65536, 5000)
	if err != nil {
		panic(err)
	}
	if status != 200 || len(body) == 0 || len(headers) == 0 {
		panic("real platform health response")
	}
	println("native HTTP real platform health probe passed")
}
