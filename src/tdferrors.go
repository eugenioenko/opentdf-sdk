package sdk

import (
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
)

var (
	errWriteFailed      = errors.New("tdf: io.writer fail to write all bytes")
	errInvalidKasInfo   = errors.New("tdf: kas information is missing")
	errKasPubKeyMissing = errors.New("tdf: kas public key is missing")

	// errInputShorterThanDeclared reports a payload reader that hit EOF before
	// producing the byte count promised by WithInputSize.
	errInputShorterThanDeclared = errors.New("tdf: payload shorter than the declared input size")

	// errTooManySegments rejects a declared input size that would split into more
	// segments than the archive writer can count.
	errTooManySegments = errors.New("tdf: input size needs more segments than the archive can address")

	// ErrAESGCMExhausted is returned by a TDF writer that refuses to encrypt
	// another segment under its data encryption key (DEK); see
	// maxAESGCMSegmentsPerDEK.
	ErrAESGCMExhausted = errors.New("tdf: AES-GCM IV collision limit reached for this data encryption key")

	// Exposed tamper detection errors, Catch all possible tamper errors with errors.Is(ErrTampered)
	ErrTampered                = errors.New("tamper detected")
	ErrRootSigValidation       = fmt.Errorf("[%w] tdf: failed integrity check on root signature", ErrTampered)
	ErrSegSizeMismatch         = fmt.Errorf("[%w] tdf: mismatch encrypted segment size in manifest", ErrTampered)
	ErrSegSizeUnresolved       = fmt.Errorf("[%w] tdf: segment size invalid or missing from manifest, with no default to fall back on", ErrTampered)
	ErrSegSigValidation        = fmt.Errorf("[%w] tdf: failed integrity check on segment hash", ErrTampered)
	ErrGMACSignatureFailed     = fmt.Errorf("[%w] tdf: ciphertext too short for a gmac signature", ErrTampered)
	ErrTDFPayloadReadFail      = fmt.Errorf("[%w] tdf: fail to read payload from tdf", ErrTampered)
	ErrTDFPayloadInvalidOffset = fmt.Errorf("[%w] sdk.Reader.ReadAt: negative offset", ErrTampered)
	ErrRootSignatureFailure    = fmt.Errorf("[%w] tdf: issue verifying root signature", ErrTampered)
	ErrRewrapBadRequest        = fmt.Errorf("[%w] tdf: rewrap request 400", ErrTampered)

	// ErrUnsupportedRootIntegrityAlgorithm rejects any root signature algorithm
	// other than HS256, on both the write and the read path.
	ErrUnsupportedRootIntegrityAlgorithm = errors.New("tdf: unsupported root integrity algorithm")

	// ErrUnsupportedSegmentIntegrityAlgorithm rejects a segment algorithm that
	// is neither HS256 nor GMAC.
	ErrUnsupportedSegmentIntegrityAlgorithm = errors.New("tdf: unsupported segment integrity algorithm")

	// kasGenericBadRequest is the substring the SDK looks for in serialized
	// KAS 400 errors to identify potential tamper. Must stay in sync with the
	// "bad request" message in service/kas/access/rewrap.go.
	kasGenericBadRequest = "desc = bad request"

	// KAS request errors — client/configuration issues, not integrity failures
	ErrKASRequestError = errors.New("tdf: KAS request error")
	ErrRewrapForbidden = errors.New("tdf: rewrap request 403")
)

// Custom error struct for Assertion errors
type ErrAssertionFailure struct {
	ID string
}

func (e ErrAssertionFailure) Error() string {
	return fmt.Errorf("[%w] tdf: issue verifying assertions, id: %s", ErrTampered, e.ID).Error()
}

func (e ErrAssertionFailure) Unwrap() error {
	return ErrTampered
}
