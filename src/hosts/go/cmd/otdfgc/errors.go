package main

import (
	"errors"
	"fmt"

	g "goalchemyout"
)

// describe renders an SDK failure: the Error carries the reference SDK's
// wording (docs/error-wording.md), and otdfctl's "required obligations"
// suffix is appended when the KAS reported any.
func describe(err error) string {
	var sdkErr *g.Error
	if !errors.As(err, &sdkErr) {
		return err.Error()
	}
	msg := sdkErr.Message
	if len(sdkErr.RequiredObligations) > 0 {
		msg += fmt.Sprintf("\nrequired obligations: %v", sdkErr.RequiredObligations)
	}
	return msg
}

// describeNew renders a failed SDK construction with otdfctl's wording for
// an unreachable platform and for rejected credentials, keeping the SDK's
// text so the cause stays visible.
func describeNew(err error, host string) string {
	var sdkErr *g.Error
	if errors.As(err, &sdkErr) {
		switch {
		case sdkErr.Kind == g.KindAuth:
			return "Failed to authenticate with flag-provided client credentials. (" + sdkErr.Message + ")"
		case errors.Is(err, g.ErrPlatformUnreachable), sdkErr.Kind == g.KindTransport, sdkErr.Kind == g.KindDeadlineExceeded:
			return fmt.Sprintf("Failed to connect to the platform. Is the platform accepting connections at '%s'? (%s)", host, sdkErr.Message)
		}
		return sdkErr.Message
	}
	return err.Error()
}
