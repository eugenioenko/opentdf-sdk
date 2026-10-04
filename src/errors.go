// Package sdk provides a bounded in-memory OpenTDF client.
package sdk

// Error contains stable failure categories without including credentials or keys.
// ServerMessage preserves bounded service diagnostics for host inspection and
// is intentionally omitted from Error(). Do not log it without host review.
// RequiredObligations is populated when the service requires host fulfillment.
type Error struct {
	Code                string
	Operation           string
	HTTPStatus          int
	ServerCode          string
	ServerMessage       string
	RequiredObligations []string
	Cause               error
}

func (e *Error) Error() string { return "sdk: " + e.Operation + ": " + e.Code }
func (e *Error) Unwrap() error { return e.Cause }
func failure(op, code string, cause error) error {
	return &Error{Code: code, Operation: op, Cause: cause}
}
