package sdk

import (
	"errors"
	"strings"
	"testing"

	"github.com/eugenioenko/goalchemy/lib/context"
)

func TestErrorChain(t *testing.T) {
	inner := wrapError(KindRewrapForbidden, "Rewrap", "kao unwrap failed", errors.New("forbidden"))
	inner.HTTPStatus = 403
	outer := wrapError(KindDecryptFailed, "DecryptBytes", ErrTDFDecryptFailed.Error(), inner)
	if outer.Kind != KindRewrapForbidden || outer.Op != "Rewrap" || outer.HTTPStatus != 403 || outer.Message != "tdf: decrypt failed: kao unwrap failed: forbidden" {
		t.Fatalf("%+v", outer)
	}
	if !errors.Is(outer, ErrTDFDecryptFailed) || errors.Is(outer, ErrTampered) || !errors.Is(outer, inner) {
		t.Fatal("Is")
	}
	joined := joinError(KindBulk, "Init", []error{errors.New("first"), sentinelError(KindSegmentSize, ErrSegSizeMismatch)})
	if joined.Kind != KindSegmentSize || joined.Message != "first\n"+ErrSegSizeMismatch.Error() || !errors.Is(joined, ErrTampered) {
		t.Fatalf("%+v", joined)
	}
	if e := wrapError(KindTransport, "x", "", context.Canceled); e.Kind != KindCanceled || e.Message != "context canceled" {
		t.Fatalf("%+v", e)
	}
	u := unsupported("CreateTDF", "Assertions")
	if u.Kind != KindUnsupportedFeature || u.Message != "unsupported in this build: Assertions" {
		t.Fatalf("%+v", u)
	}
	if !strings.HasPrefix(ErrRootSigValidation.Error(), "[tamper detected] ") || !errors.Is(ErrRootSigValidation, ErrTampered) {
		t.Fatal("sentinel wrapping")
	}
	var a ErrAssertionFailure = ErrAssertionFailure{ID: "x"}
	if a.Error() != "[tamper detected] tdf: issue verifying assertions, id: x" || !errors.Is(a, ErrTampered) {
		t.Fatal(a.Error())
	}
}
