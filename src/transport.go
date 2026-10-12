package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/clock"
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/http"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/strings"
	"opentdf-local/sdk/src/internal/urlutil"
)

const (
	defaultTimeoutMillis = 30000
	maxResponseBytes     = 16 << 20
	connectProtocol      = "application/json"
)

// Transport is one bounded HTTP exchange: the reference's *http.Client. The
// default runs on lib/http; tests and wrappers supply replacements.
type Transport interface {
	Do(ctx context.Context, method, url string, headers []string, body []byte, maxResponseBytes int, timeoutMillis int64) (int, []string, []byte, error)
}

// Clock supplies Unix seconds for token expiry, JWT claims and the KAS key
// cache; the default reads the host clock.
type Clock interface {
	Now() int64
}

type systemClock struct{}

func (systemClock) Now() int64 { return clock.Unix() }

type libTransport struct {
	options []string
}

func (t *libTransport) Do(ctx context.Context, method, url string, headers []string, body []byte, maxBytes int, timeoutMillis int64) (int, []string, []byte, error) {
	if len(t.options) == 0 {
		return http.Do(ctx, method, url, headers, body, maxBytes, timeoutMillis)
	}
	return http.DoWith(ctx, method, url, headers, body, maxBytes, timeoutMillis, t.options)
}

// httpClient is the SDK's authenticated request layer: the reference's
// http.Client wrapped by the DPoP transport and the token interceptor.
// Resource requests carry Authorization and, for DPoP-bound tokens, a proof
// with the ath claim; token-endpoint requests carry a proof without ath.
type httpClient struct {
	transport     Transport
	clock         Clock
	timeoutMillis int64
	tokenSource   TokenSource
	dpop          *dpopSigner
	tokenEndpoint string
}

func headerValue(headers []string, name string) string {
	for i := 0; i+1 < len(headers); i += 2 {
		if strings.EqualFold(headers[i], name) {
			return headers[i+1]
		}
	}
	return ""
}

func isJSONContentType(headers []string) bool {
	typ := strings.ToLower(headerValue(headers, "content-type"))
	if i := strings.Index(typ, ";"); i >= 0 {
		typ = typ[:i]
	}
	return strings.TrimSpace(typ) == connectProtocol
}

// do sends one request, adding a DPoP proof (and a nonce retry) when the
// client has a signing key. token is the access token bound by ath, or empty
// for token-endpoint requests.
func (c *httpClient) do(ctx context.Context, method, url string, headers []string, body []byte, token string, proof bool) (int, []string, []byte, error) {
	if !proof || c.dpop == nil {
		return c.transport.Do(ctx, method, url, headers, body, maxResponseBytes, c.timeoutMillis)
	}
	htu, origin, err := normalizeURI(url)
	if err != nil {
		return 0, nil, nil, err
	}
	nonce := c.dpop.cachedNonce(origin)
	for attempt := 0; attempt < 2; attempt++ {
		jwt, err := c.dpop.proof(c.clock, method, htu, token, nonce)
		if err != nil {
			return 0, nil, nil, err
		}
		outgoing := make([]string, len(headers), len(headers)+2)
		copy(outgoing, headers)
		outgoing = append(outgoing, "DPoP", jwt)
		status, responseHeaders, data, err := c.transport.Do(ctx, method, url, outgoing, body, maxResponseBytes, c.timeoutMillis)
		if err != nil {
			return 0, nil, nil, err
		}
		received := headerValue(responseHeaders, "DPoP-Nonce")
		challenge := status == 401 || status == 400
		if received != "" && (challenge || status >= 200 && status < 300) {
			c.dpop.cacheNonce(origin, received)
		}
		// Retry once when the server supplies a nonce that differs from the
		// one sent (RFC 9449 §8); the retry is returned as is.
		if attempt == 0 && challenge && received != "" && received != nonce {
			nonce = received
			continue
		}
		return status, responseHeaders, data, nil
	}
	return 0, nil, nil, newError(KindTransport, "dpop", "dpop nonce retry limit reached")
}

// resource sends an authenticated request to a platform or KAS service.
func (c *httpClient) resource(ctx context.Context, method, url string, headers []string, body []byte) (int, []string, []byte, error) {
	token := ""
	proof := false
	if c.tokenSource != nil {
		t, err := c.tokenSource.Token(ctx)
		if err != nil {
			return 0, nil, nil, wrapError(KindAuth, "", "failed to get access token", err)
		}
		if t.Scheme == "DPoP" && c.dpop != nil {
			headers = append(append([]string(nil), headers...), "Authorization", "DPoP "+t.Value)
			token = t.Value
			proof = true
		} else {
			headers = append(append([]string(nil), headers...), "Authorization", "Bearer "+t.Value)
		}
	}
	return c.do(ctx, method, url, headers, body, token, proof)
}

// connectCall is a Connect unary call with the JSON codec: a POST of the
// request message to baseURL+procedure. A non-2xx reply becomes an Error
// carrying the Connect code and message, worded like connect-go's Error.
func (c *httpClient) connectCall(ctx context.Context, baseURL, procedure string, request []byte, extraHeaders []string, op string) (jsonvalue.Value, error) {
	headers := []string{"Content-Type", connectProtocol, "Connect-Protocol-Version", "1", "Accept", connectProtocol}
	headers = append(headers, extraHeaders...)
	url := strings.TrimSuffix(baseURL, "/") + procedure
	status, responseHeaders, data, err := c.resource(ctx, "POST", url, headers, request)
	if err != nil {
		if e, ok := err.(*Error); ok {
			return jsonvalue.Value{}, e
		}
		return jsonvalue.Value{}, wrapError(KindTransport, op, "", err)
	}
	if status < 200 || status >= 300 {
		return jsonvalue.Value{}, connectError(status, data, op)
	}
	if !isJSONContentType(responseHeaders) {
		return jsonvalue.Value{}, newError(KindTransport, op, "unexpected content type: "+headerValue(responseHeaders, "content-type"))
	}
	v, err := jsonvalue.Parse(data)
	if err != nil || v.Kind() != jsonvalue.KindObject {
		return jsonvalue.Value{}, newError(KindTransport, op, "invalid JSON response")
	}
	return v, nil
}

// connectError maps an HTTP status and a Connect error body to an Error. The
// message is connect-go's "<code>: <message>" so that KAS error text reads
// as it does in the reference.
func connectError(status int, data []byte, op string) *Error {
	code := connectCodeForHTTPStatus(status)
	message := ""
	if v, err := jsonvalue.Parse(data); err == nil && v.Kind() == jsonvalue.KindObject {
		if c, ok := v.Get("code").AsString(); ok && c != "" {
			code = c
		}
		message, _ = v.Get("message").AsString()
	}
	kind := KindTransport
	switch code {
	case "unauthenticated":
		kind = KindAuth
	case "permission_denied":
		kind = KindRewrapForbidden
	case "invalid_argument":
		kind = KindKASRequest
	case "canceled":
		kind = KindCanceled
	case "deadline_exceeded":
		kind = KindDeadlineExceeded
	}
	text := code
	if message != "" {
		text += ": " + message
	}
	return &Error{Kind: kind, Op: op, Message: text, HTTPStatus: status, ConnectCode: code, ServiceMessage: message}
}

func connectCodeForHTTPStatus(status int) string {
	switch status {
	case 400:
		return "invalid_argument"
	case 401:
		return "unauthenticated"
	case 403:
		return "permission_denied"
	case 404:
		return "unimplemented"
	case 408:
		return "deadline_exceeded"
	case 409:
		return "aborted"
	case 412:
		return "failed_precondition"
	case 413:
		return "resource_exhausted"
	case 415:
		return "internal"
	case 429:
		return "unavailable"
	case 431:
		return "resource_exhausted"
	case 502, 503, 504:
		return "unavailable"
	}
	return "unknown"
}

// normalizeURI returns the DPoP htu for a request URL (scheme and host
// lower-cased, default port dropped, query and fragment removed) and the
// origin the nonce cache is keyed on, as the reference's DPoP transport does.
func normalizeURI(rawURL string) (string, string, error) {
	u, err := urlutil.Parse(rawURL)
	if err != nil {
		return "", "", err
	}
	scheme := strings.ToLower(u.Scheme)
	host := strings.ToLower(u.Hostname())
	port := u.Port()
	if port == "443" && scheme == "https" || port == "80" && scheme == "http" {
		port = ""
	}
	if port != "" {
		host = urlutil.JoinHostPort(host, port)
	} else if strings.Contains(host, ":") {
		host = "[" + host + "]"
	}
	origin := scheme + "://" + host
	path := u.EscapedPath()
	if path == "" {
		path = "/"
	}
	return origin + path, origin, nil
}
