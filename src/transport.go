package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/clock"
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/http"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/net/url"
	"github.com/eugenioenko/goalchemy/std/strings"
)

const (
	defaultTimeoutMillis = 30000
	maxResponseBytes     = 16 << 20
	jsonContentType      = "application/json"
)

// Transport is one bounded HTTP exchange: the reference's *http.Client. The
// default runs on lib/http; tests and wrappers supply replacements.
type Transport interface {
	// Do performs a bounded HTTP exchange and returns status, headers, and body. Headers
	// are alternating name/value strings; ctx and timeoutMillis bound the request.
	Do(ctx context.Context, method, url string, headers []string, body []byte, maxResponseBytes int, timeoutMillis int64) (int, []string, []byte, error)
}

// Clock supplies Unix seconds for token expiry, JWT claims and the KAS key
// cache; the default reads the host clock.
type Clock interface {
	// Now returns the current UTC time in Unix seconds.
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
	return strings.TrimSpace(typ) == jsonContentType
}

// do sends one request with the client's bounds, adding a DPoP proof (and a
// nonce retry) when the client has a signing key. token is the access token
// bound by ath, or empty for token-endpoint requests.
func (c *httpClient) do(ctx context.Context, method, rawURL string, headers []string, body []byte, token string, proof bool) (int, []string, []byte, error) {
	return c.exchange(ctx, method, rawURL, headers, body, maxResponseBytes, c.timeoutMillis, token, proof)
}

func (c *httpClient) exchange(ctx context.Context, method, rawURL string, headers []string, body []byte, maxBytes int, timeoutMillis int64, token string, proof bool) (int, []string, []byte, error) {
	if !proof || c.dpop == nil {
		return c.transport.Do(ctx, method, rawURL, headers, body, maxBytes, timeoutMillis)
	}
	u, err := url.Parse(rawURL)
	if err != nil {
		return 0, nil, nil, err
	}
	htu := normalizeURI(u)
	origin := getOrigin(u)
	nonce := c.dpop.cachedNonce(origin)
	for attempt := 0; attempt < 2; attempt++ {
		jwt, err := c.dpop.proof(c.clock, method, htu, token, nonce)
		if err != nil {
			return 0, nil, nil, err
		}
		outgoing := make([]string, len(headers), len(headers)+2)
		copy(outgoing, headers)
		outgoing = append(outgoing, "DPoP", jwt)
		status, responseHeaders, data, err := c.transport.Do(ctx, method, rawURL, outgoing, body, maxBytes, timeoutMillis)
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

// Do sends an authenticated request to a platform or KAS service: the
// reference's token-adding interceptor and DPoP transport. It is the
// connect.HTTPClient every generated service client of the SDK uses.
func (c *httpClient) Do(ctx context.Context, method, url string, headers []string, body []byte, maxBytes int, timeoutMillis int64) (int, []string, []byte, error) {
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
	return c.exchange(ctx, method, url, headers, body, maxBytes, timeoutMillis, token, proof)
}

// normalizedHostPort returns the URL host lowercased with the scheme's default
// port (80 for http, 443 for https) removed. IPv6 literals keep their brackets.
func normalizedHostPort(u *url.URL) string {
	scheme := strings.ToLower(u.Scheme)
	host := strings.ToLower(u.Hostname())
	if strings.Contains(host, ":") {
		host = "[" + host + "]"
	}

	port := u.Port()
	if port == "" ||
		(scheme == "http" && port == "80") ||
		(scheme == "https" && port == "443") {
		return host
	}

	return host + ":" + port
}

// normalizeURI normalizes the URI per RFC 9449 HTTP URI Normalization:
// lowercase scheme and host, remove default ports, normalize an empty path to
// "/", strip query and fragment. The path keeps percent-encoded reserved bytes.
func normalizeURI(u *url.URL) string {
	escapedPath := u.EscapedPath()
	if escapedPath == "" {
		escapedPath = "/"
	}
	return fmt.Sprintf("%s://%s%s", strings.ToLower(u.Scheme), normalizedHostPort(u), escapedPath)
}

// getOrigin returns the origin (scheme://host:port) from a URL, normalized to
// lowercase with the scheme's default port removed.
func getOrigin(u *url.URL) string {
	return fmt.Sprintf("%s://%s", strings.ToLower(u.Scheme), normalizedHostPort(u))
}
