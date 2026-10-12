package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/bytes"
	"github.com/eugenioenko/goalchemy/std/fmt"
)

// maxDecryptBytesSize caps the plaintext size DecryptBytes will buffer into
// memory.
var maxDecryptBytesSize int64 = 1 << 30 // 1 GiB

// DecryptBytes decrypts a TDF payload held in memory and returns the
// plaintext. A thin wrapper over [SDK.LoadTDF] + the reader's payload read:
//
//	plaintext, err := sdk.DecryptBytes(ctx, ciphertext, ReaderOptions{})
//
// ctx governs the KAS rewrap request (the only network call this method
// makes) and is honored for cancellation/timeout.
func (s *SDK) DecryptBytes(ctx context.Context, ciphertext []byte, opts ReaderOptions) ([]byte, error) {
	reader, err := s.LoadTDF(ctx, bytes.NewReader(ciphertext), opts)
	if err != nil {
		return nil, wrapError(KindNotDecryptable, "DecryptBytes", ErrTDFNotDecryptable.Error(), err)
	}
	defer reader.Close()
	if err = reader.Init(ctx); err != nil {
		return nil, wrapError(KindDecryptFailed, "DecryptBytes", ErrTDFDecryptFailed.Error(), err)
	}

	if reader.payloadSize > maxDecryptBytesSize {
		return nil, newError(KindNotDecryptable, "DecryptBytes", fmt.Sprintf("%v: plaintext is %d bytes, exceeds the %d byte limit for DecryptBytes; use DecryptTo, DecryptFile, or LoadTDF directly for large payloads", ErrTDFNotDecryptable, reader.payloadSize, maxDecryptBytesSize))
	}

	plaintext, err := reader.readAll()
	if err != nil {
		return nil, wrapError(KindDecryptFailed, "DecryptBytes", ErrTDFDecryptFailed.Error(), err)
	}
	return plaintext, nil
}
