package main

import (
	"errors"
	"fmt"
	"strings"

	g "goalchemyout"
)

// describe renders a façade failure with the reference SDK's wording
// (docs/error-wording.md) where the façade's bounded error record carries
// enough to choose it, and the record's own text otherwise. kasURL is the
// manifest's KAS, needed for the allowlist message.
func describe(err error, kasURL string) string {
	var failure *g.Failure
	if !errors.As(err, &failure) {
		return err.Error()
	}
	msg := failureMessage(failure, kasURL)
	if len(failure.RequiredObligations) > 0 {
		msg += fmt.Sprintf("\nrequired obligations: %v", failure.RequiredObligations)
	}
	return msg
}

func failureMessage(f *g.Failure, kasURL string) string {
	server := strings.TrimSpace(f.ServerMessage)
	switch {
	case f.CauseCategory == "canceled":
		return "context canceled"
	case f.CauseCategory == "deadline_exceeded":
		return "context deadline exceeded"
	case f.Code == "kas_not_allowed":
		return fmt.Sprintf("KasAllowlist: kas url %s is not allowed", kasURL)
	case f.Operation == "rewrap" && f.HTTPStatus == 403:
		return join("tdf: rewrap request 403", server)
	case f.Operation == "rewrap" && f.HTTPStatus == 400:
		return join("[tamper detected] tdf: rewrap request 400", server)
	case f.Operation == "rewrap" && f.HTTPStatus != 0:
		return join(fmt.Sprintf("tdf: KAS request error: rewrap request %d", f.HTTPStatus), server)
	case f.Operation == "auth" || f.Operation == "oauth" || f.Operation == "discovery" || f.Code == "unauthenticated":
		return "Failed to authenticate with flag-provided client credentials. (" + f.Error() + ")"
	case f.Code == "transport":
		return join("failed to connect to the platform", f.Error())
	case f.Operation == "decrypt" && f.Code == "integrity":
		return "[tamper detected] tdf: failed integrity check (policy binding, root signature or segment hash)"
	case f.Operation == "decrypt" && (f.Code == "archive" || f.Code == "manifest" || f.Code == "unsupported_manifest"):
		return join("tdf: input is not decryptable", f.Error())
	}
	return f.Error()
}

func join(message, detail string) string {
	if detail == "" {
		return message
	}
	return message + ": " + detail
}
