package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/strings"
)

// ErrorKind is the stable classification hosts switch on; Message carries the
// reference SDK's wording.
type ErrorKind string

const (
	// KindTampered reports a general integrity or tamper-detection failure.
	KindTampered ErrorKind = "tampered"
	// KindRootSignature reports a root signature verification failure.
	KindRootSignature ErrorKind = "root_signature"
	// KindSegmentSize reports invalid or inconsistent segment lengths.
	KindSegmentSize ErrorKind = "segment_size"
	// KindSegmentSignature reports a segment integrity verification failure.
	KindSegmentSignature ErrorKind = "segment_signature"
	// KindPayloadRead reports failure to read the encrypted payload.
	KindPayloadRead ErrorKind = "payload_read"
	// KindInvalidOffset reports an invalid payload offset.
	KindInvalidOffset ErrorKind = "invalid_offset"
	// KindRewrapBadRequest reports a rejected KAS rewrap request that may indicate
	// tampering.
	KindRewrapBadRequest ErrorKind = "rewrap_bad_request"
	// KindUnsupportedRootIntegrity reports an unsupported root integrity algorithm.
	KindUnsupportedRootIntegrity ErrorKind = "unsupported_root_integrity"
	// KindUnsupportedSegmentIntegrity reports an unsupported segment integrity algorithm.
	KindUnsupportedSegmentIntegrity ErrorKind = "unsupported_segment_integrity"
	// KindNotDecryptable reports an archive that cannot be loaded for decryption.
	KindNotDecryptable ErrorKind = "not_decryptable"
	// KindDecryptFailed reports a payload decryption or key-unwrapping failure.
	KindDecryptFailed ErrorKind = "decrypt_failed"
	// KindAESGCMExhausted reports that a writer reached its per-key encryption limit.
	KindAESGCMExhausted ErrorKind = "aes_gcm_exhausted"
	// KindKASRequest reports a KAS request or configuration failure.
	KindKASRequest ErrorKind = "kas_request"
	// KindRewrapForbidden reports a KAS authorization denial.
	KindRewrapForbidden ErrorKind = "rewrap_forbidden"
	// KindRewrapUnauthorized reports a KAS authentication failure.
	KindRewrapUnauthorized ErrorKind = "rewrap_unauthorized"
	// KindAssertionFailure reports an assertion validation or verification failure.
	KindAssertionFailure ErrorKind = "assertion_failure"
	// KindObligationsRequired reports obligations the caller must satisfy before access
	// is granted.
	KindObligationsRequired ErrorKind = "obligations_required"
	// KindBaseKeyNotFound reports a missing platform base key.
	KindBaseKeyNotFound ErrorKind = "base_key_not_found"
	// KindBaseKeyInvalid reports an invalid platform base-key record.
	KindBaseKeyInvalid ErrorKind = "base_key_invalid"
	// KindAllowlistRejected reports a manifest KAS endpoint outside the allowed origins.
	KindAllowlistRejected ErrorKind = "allowlist_rejected"
	// KindUnsupportedKeyAccess reports an unsupported key access scheme.
	KindUnsupportedKeyAccess ErrorKind = "unsupported_key_access"
	// KindUnsupportedFeature reports an option or operation not implemented by this
	// build.
	KindUnsupportedFeature ErrorKind = "unsupported_feature"
	// KindInvalidInput reports invalid arguments or configuration.
	KindInvalidInput ErrorKind = "invalid_input"
	// KindAuth reports token acquisition or authentication configuration failure.
	KindAuth ErrorKind = "auth"
	// KindTransport reports an HTTP, connection, or discovery failure.
	KindTransport ErrorKind = "transport"
	// KindCanceled reports cancellation of an operation.
	KindCanceled ErrorKind = "canceled"
	// KindDeadlineExceeded reports expiry of an operation's deadline.
	KindDeadlineExceeded ErrorKind = "deadline_exceeded"
	// KindBulk reserves the error classification for bulk operations.
	KindBulk ErrorKind = "bulk"
)

// Error is the one error type of the SDK. Message is the reference SDK's
// wording for the whole chain; errors.Is matches the reference sentinels
// through Is. ServiceMessage is bounded service text and is never logged by
// the SDK. The record holds only copied values so it crosses the Goalchemy
// boundary unchanged.
type Error struct {
	// Kind is the stable category to use for programmatic error handling.
	Kind ErrorKind
	// Op names the operation that failed.
	Op string
	// Message is the human-readable error text, including wrapped causes.
	Message string
	// HTTPStatus is the service's HTTP status, or zero when none is available.
	HTTPStatus int
	// ConnectCode is the Connect or OAuth error code when provided by the service.
	ConnectCode string
	// ServiceMessage contains bounded service-supplied diagnostic text. Handle it as
	// potentially sensitive.
	ServiceMessage string
	// KASURL identifies the KAS associated with the failure, when known.
	KASURL string
	// RequiredObligations lists obligation FQNs reported by the service.
	RequiredObligations []string
	// AssertionID identifies the assertion that failed, when known.
	AssertionID string
}

// Error returns the human-readable Message.
func (e *Error) Error() string { return e.Message }

// Is reports whether target is one of the sentinels in this error's chain. The
// chain is recorded as text (the reference joins its sentinels with %w), so a
// sentinel matches when its text appears in Message.
func (e *Error) Is(target error) bool {
	if other, ok := target.(*Error); ok {
		return other == e || other.Kind == e.Kind && strings.Contains(e.Message, other.Message)
	}
	if target == nil {
		return false
	}
	text := target.Error()
	return text != "" && strings.Contains(e.Message, text)
}

func newError(kind ErrorKind, op, message string) *Error {
	return &Error{Kind: kind, Op: op, Message: message}
}

// sentinelError returns the reference sentinel's text as an Error of kind.
func sentinelError(kind ErrorKind, sentinel error) *Error {
	return &Error{Kind: kind, Message: sentinel.Error()}
}

// wrapError is the reference's fmt.Errorf("%s: %w", prefix, cause). The
// cause's kind and service fields win when the cause is already an Error, so
// the most specific classification survives the chain.
func wrapError(kind ErrorKind, op, prefix string, cause error) *Error {
	e := &Error{Kind: kind, Op: op, Message: prefix}
	if cause == nil {
		return e
	}
	if c, ok := asError(cause); ok {
		e = &Error{Kind: c.Kind, Op: c.Op, HTTPStatus: c.HTTPStatus, ConnectCode: c.ConnectCode, ServiceMessage: c.ServiceMessage, KASURL: c.KASURL, RequiredObligations: append([]string(nil), c.RequiredObligations...), AssertionID: c.AssertionID}
		// A generic invalid-input cause takes the more specific outer kind.
		if e.Kind == "" || e.Kind == KindInvalidInput && kind != "" {
			e.Kind = kind
		}
		if e.Op == "" {
			e.Op = op
		}
	} else {
		e.Kind = contextKind(cause, kind)
	}
	e.Message = cause.Error()
	if prefix != "" {
		e.Message = prefix + ": " + e.Message
	}
	return e
}

// joinError is the reference's errors.Join: the texts separated by newlines.
// The first Error in errs supplies the record's fields.
func joinError(kind ErrorKind, op string, errs []error) *Error {
	var first *Error
	texts := []string{}
	for _, err := range errs {
		if err == nil {
			continue
		}
		if c, ok := asError(err); ok && first == nil {
			first = c
		}
		texts = append(texts, err.Error())
	}
	e := &Error{Kind: kind, Op: op}
	if first != nil {
		e = wrapError(kind, op, "", first)
	}
	e.Message = strings.Join(texts, "\n")
	return e
}

// asError finds the Error in err's chain, through fmt.Errorf and
// errors.Join wrappers, so a wrapped failure keeps its kind and fields.
func asError(err error) (*Error, bool) {
	for err != nil {
		if e, ok := err.(*Error); ok {
			return e, e != nil
		}
		switch u := err.(type) {
		case interface{ Unwrap() error }:
			err = u.Unwrap()
		case interface{ Unwrap() []error }:
			for _, inner := range u.Unwrap() {
				if e, ok := asError(inner); ok {
					return e, true
				}
			}
			return nil, false
		default:
			return nil, false
		}
	}
	return nil, false
}

// contextKind classifies a cancellation or expiry: the context sentinels
// natively, and the host transport's wording ("http: deadline exceeded")
// where a target's capability reports the deadline as text.
func contextKind(err error, fallback ErrorKind) ErrorKind {
	if errors.Is(err, context.Canceled) {
		return KindCanceled
	}
	if errors.Is(err, context.DeadlineExceeded) || strings.Contains(err.Error(), "deadline exceeded") {
		return KindDeadlineExceeded
	}
	return fallback
}

// boundaryError makes err an *Error for an exported method's return: an
// *Error in the chain is kept (its fields win), anything else becomes an
// Error of kind with the chain's text, so every failure crosses the
// Goalchemy boundary as the one error type of api-spec.md.
func boundaryError(kind ErrorKind, op string, err error) error {
	if err == nil {
		return nil
	}
	if e, ok := err.(*Error); ok {
		return e
	}
	if _, ok := asError(err); ok {
		return wrapError(kind, op, "", err)
	}
	return &Error{Kind: contextKind(err, kind), Op: op, Message: err.Error()}
}

// unsupported is the error for an option or input this build does not
// implement yet; the feature name is the api-spec option or feature name.
func unsupported(op, feature string) *Error {
	return newError(KindUnsupportedFeature, op, "unsupported in this build: "+feature)
}

// postQuantumNotPlanned is why post-quantum key types and key access objects
// fail: Goalchemy has no post-quantum primitives (goalchemy#21).
const postQuantumNotPlanned = "post-quantum mechanisms are not planned"

func notPlannedPQ(op, feature string) *Error {
	return unsupported(op, feature+": "+postQuantumNotPlanned)
}
