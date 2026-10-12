package sdk

import (
	"testing"

	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
)

func parseJSON(t *testing.T, s string) (jsonvalue.Value, error) {
	t.Helper()
	return jsonvalue.Parse([]byte(s))
}
