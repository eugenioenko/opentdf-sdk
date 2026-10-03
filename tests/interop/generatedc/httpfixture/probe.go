// SPDX-License-Identifier: Apache-2.0
package httpprobe

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/http"
)

type Reply struct {
	Status  int64
	Headers []string
	Body    []byte
}

func Probe(ctx context.Context, url string, headers []string, limit int64) (Reply, error) {
	status, h, b, e := http.Do(ctx, "GET", url, headers, nil, int(limit), 1000)
	if e != nil {
		return Reply{}, e
	}
	return Reply{int64(status), h, b}, nil
}
