package sdk

import "github.com/eugenioenko/goalchemy/std/errors"

var (
	// ErrBaseKeyNotFound reports a missing base_key member in platform configuration.
	ErrBaseKeyNotFound = errors.New("base key not found in well-known configuration")
	// ErrBaseKeyInvalidFormat reports a base-key record with an invalid structure or
	// algorithm.
	ErrBaseKeyInvalidFormat = errors.New("base key has invalid format")
	// ErrBaseKeyEmpty reports an empty base-key value.
	ErrBaseKeyEmpty = errors.New("base key is empty or not provided")
	// ErrMarshalBaseKeyFailed reports failure to encode a base-key record.
	ErrMarshalBaseKeyFailed = errors.New("failed to marshal base key configuration")
	// ErrUnmarshalBaseKeyFailed reports failure to decode a base-key record.
	ErrUnmarshalBaseKeyFailed = errors.New("failed to unmarshal base key configuration")
)
