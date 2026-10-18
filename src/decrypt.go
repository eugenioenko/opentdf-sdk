package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/bytes"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/io"
	"github.com/eugenioenko/goalchemy/std/os"
)

// maxDecryptBytesSize caps the plaintext size DecryptBytes will buffer into
// memory.
var maxDecryptBytesSize int64 = 1 << 30 // 1 GiB

// DecryptBytes decrypts a TDF payload held in memory and returns the
// plaintext. A thin wrapper over [SDK.LoadTDF] + [Reader.WriteTo]:
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

	out := bytes.NewBuffer(make([]byte, 0, reader.payloadSize))
	if _, err = reader.WriteTo(ctx, out); err != nil {
		return nil, wrapError(KindDecryptFailed, "DecryptBytes", ErrTDFDecryptFailed.Error(), err)
	}
	return out.Bytes(), nil
}

// DecryptTo decrypts the TDF on in and writes the plaintext to out, one
// segment at a time. A thin wrapper over [SDK.LoadTDF] + [Reader.WriteTo]
// for callers who have a destination writer (a file, standard output, an
// HTTP response) and do not need the plaintext in memory:
//
//	err := sdk.DecryptTo(ctx, os.Stdout, tdfFile, ReaderOptions{})
//
// The reference takes the ciphertext as bytes; here it is a seekable stream,
// so neither side of the operation is held whole. ctx governs the KAS rewrap
// request (the only network call this method makes) and is honored for
// cancellation/timeout.
func (s *SDK) DecryptTo(ctx context.Context, out io.Writer, in io.ReadSeeker, opts ReaderOptions) error {
	reader, err := s.LoadTDF(ctx, in, opts)
	if err != nil {
		return wrapError(KindNotDecryptable, "DecryptTo", ErrTDFNotDecryptable.Error(), err)
	}
	defer reader.Close()
	if err = reader.Init(ctx); err != nil {
		return wrapError(KindDecryptFailed, "DecryptTo", ErrTDFDecryptFailed.Error(), err)
	}
	if _, err = reader.WriteTo(ctx, out); err != nil {
		return wrapError(KindDecryptFailed, "DecryptTo", ErrTDFDecryptFailed.Error(), err)
	}
	return nil
}

// DecryptFile decrypts the TDF at inputPath and writes the plaintext to
// outputPath, streaming from disk to disk:
//
//	err := sdk.DecryptFile(ctx, "secret.tdf", "secret.txt", ReaderOptions{})
//
// The plaintext goes to a temporary file in outputPath's directory (mode
// 0600, since it holds decrypted content) that is renamed onto outputPath
// only once decryption has fully succeeded, so a pre-existing file there is
// never truncated or left half-written by a failure; the rename replaces an
// existing file as POSIX rename does. On any failure the temporary file is
// removed; if that removal fails too, both errors are reported. An existing
// directory at outputPath is rejected up front, as is an outputPath equal to
// inputPath. ctx governs the KAS rewrap request (the only network call this
// method makes) and is honored for cancellation/timeout.
func (s *SDK) DecryptFile(ctx context.Context, inputPath, outputPath string, opts ReaderOptions) error {
	if inputPath == outputPath {
		return newError(KindInvalidInput, "DecryptFile", fmt.Sprintf("inputPath and outputPath must not be the same file: %s", outputPath))
	}
	in, err := os.Open(inputPath)
	if err != nil {
		return wrapError(KindInvalidInput, "DecryptFile", fmt.Sprintf("failed to open input TDF %s", inputPath), err)
	}
	defer in.Close()

	outInfo, outStatErr := os.Stat(outputPath)
	if outStatErr == nil {
		if outInfo.IsDir() {
			return newError(KindInvalidInput, "DecryptFile", fmt.Sprintf("outputPath %s is a directory, not a file", outputPath))
		}
	} else if !errors.Is(outStatErr, os.ErrNotExist) {
		return wrapError(KindInvalidInput, "DecryptFile", fmt.Sprintf("failed to inspect output file %s", outputPath), outStatErr)
	}

	tmp, err := os.CreateTemp(parentDir(outputPath), ".decrypt-*.tmp")
	if err != nil {
		return wrapError(KindInvalidInput, "DecryptFile", fmt.Sprintf("failed to create output file %s", outputPath), err)
	}
	tmpPath := tmp.Name()

	reader, err := s.LoadTDF(ctx, in, opts)
	if err != nil {
		return cleanupTemp(wrapError(KindNotDecryptable, "DecryptFile", ErrTDFNotDecryptable.Error(), err), tmp, tmpPath)
	}
	defer reader.Close()
	if err = reader.Init(ctx); err != nil {
		return cleanupTemp(wrapError(KindDecryptFailed, "DecryptFile", ErrTDFDecryptFailed.Error(), err), tmp, tmpPath)
	}
	if _, err = reader.WriteTo(ctx, tmp); err != nil {
		return cleanupTemp(wrapError(KindDecryptFailed, "DecryptFile", ErrTDFDecryptFailed.Error(), err), tmp, tmpPath)
	}
	if err = tmp.Close(); err != nil {
		return cleanupTemp(wrapError(KindInvalidInput, "DecryptFile", "", err), nil, tmpPath)
	}
	if err = os.Rename(tmpPath, outputPath); err != nil {
		return cleanupTemp(wrapError(KindInvalidInput, "DecryptFile", fmt.Sprintf("failed to finalize output file %s", outputPath), err), nil, tmpPath)
	}
	return nil
}

// cleanupTemp closes (when tmp is given) and removes the temporary output
// after a failure, joining any removal error with err so a leftover file is
// reported rather than silent.
func cleanupTemp(err *Error, tmp *os.File, tmpPath string) error {
	errs := []error{err}
	if tmp != nil {
		if closeErr := tmp.Close(); closeErr != nil {
			errs = append(errs, closeErr)
		}
	}
	if removeErr := os.Remove(tmpPath); removeErr != nil {
		errs = append(errs, removeErr)
	}
	if len(errs) == 1 {
		return err
	}
	return joinError(err.Kind, "DecryptFile", errs)
}

// parentDir is filepath.Dir for '/'-separated paths: the directory holding
// path, "." when path has no separator.
func parentDir(path string) string {
	i := len(path) - 1
	for i >= 0 && path[i] != '/' {
		i--
	}
	if i < 0 {
		return "."
	}
	for i > 0 && path[i-1] == '/' {
		i--
	}
	if i == 0 {
		return "/"
	}
	return path[:i]
}
