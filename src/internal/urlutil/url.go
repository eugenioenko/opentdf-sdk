// Package urlutil is the subset of Go's net/url that the reference SDK uses,
// with Go's parsing rules and error text. It is replaced by std/net/url
// (goalchemy#71) once that package is released.
package urlutil

import (
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/strings"
)

type URL struct {
	Scheme     string
	Opaque     string
	User       string
	Host       string
	Path       string
	RawPath    string
	OmitHost   bool
	ForceQuery bool
	RawQuery   string
	Fragment   string
}

type Error struct {
	Op  string
	URL string
	Err error
}

func (e *Error) Unwrap() error { return e.Err }
func (e *Error) Error() string { return e.Op + " " + quote(e.URL) + ": " + e.Err.Error() }

type escapeError string

func (e escapeError) Error() string { return "invalid URL escape " + quote(string(e)) }

type invalidHostError string

func (e invalidHostError) Error() string {
	return "invalid character " + quote(string(e)) + " in host name"
}

const (
	encodePath = iota + 1
	encodePathSegment
	encodeHost
	encodeUserPassword
	encodeQueryComponent
	encodeFragment
)

func quote(s string) string {
	out := []byte{'"'}
	for _, b := range []byte(s) {
		switch {
		case b == '"' || b == '\\':
			out = append(out, '\\', b)
		case b < 32 || b > 126:
			const hex = "0123456789abcdef"
			out = append(out, '\\', 'x', hex[b>>4], hex[b&15])
		default:
			out = append(out, b)
		}
	}
	return string(append(out, '"'))
}

func ishex(c byte) bool {
	return '0' <= c && c <= '9' || 'a' <= c && c <= 'f' || 'A' <= c && c <= 'F'
}

func unhex(c byte) byte {
	switch {
	case '0' <= c && c <= '9':
		return c - '0'
	case 'a' <= c && c <= 'f':
		return c - 'a' + 10
	case 'A' <= c && c <= 'F':
		return c - 'A' + 10
	}
	return 0
}

func shouldEscape(c byte, mode int) bool {
	if 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || '0' <= c && c <= '9' {
		return false
	}
	if mode == encodeHost {
		switch c {
		case '!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '=', ':', '[', ']', '<', '>', '"':
			return false
		}
	}
	switch c {
	case '-', '_', '.', '~':
		return false
	case '$', '&', '+', ',', '/', ':', ';', '=', '?', '@':
		switch mode {
		case encodePath:
			return c == '?'
		case encodePathSegment:
			return c == '/' || c == ';' || c == ',' || c == '?'
		case encodeUserPassword:
			return c == '@' || c == '/' || c == '?' || c == ':'
		case encodeQueryComponent:
			return true
		case encodeFragment:
			return false
		}
	}
	if mode == encodeFragment {
		switch c {
		case '!', '(', ')', '*':
			return false
		}
	}
	return true
}

func unescape(s string, mode int) (string, error) {
	n := 0
	hasPlus := false
	for i := 0; i < len(s); {
		switch s[i] {
		case '%':
			n++
			if i+2 >= len(s) || !ishex(s[i+1]) || !ishex(s[i+2]) {
				s = s[i:]
				if len(s) > 3 {
					s = s[:3]
				}
				return "", escapeError(s)
			}
			if mode == encodeHost && unhex(s[i+1]) < 8 && s[i:i+3] != "%25" {
				return "", escapeError(s[i : i+3])
			}
			i += 3
		case '+':
			hasPlus = mode == encodeQueryComponent
			i++
		default:
			if mode == encodeHost && shouldEscape(s[i], mode) {
				return "", invalidHostError(s[i : i+1])
			}
			i++
		}
	}
	if n == 0 && !hasPlus {
		return s, nil
	}
	t := make([]byte, 0, len(s)-2*n)
	for i := 0; i < len(s); i++ {
		switch s[i] {
		case '%':
			t = append(t, unhex(s[i+1])<<4|unhex(s[i+2]))
			i += 2
		case '+':
			if mode == encodeQueryComponent {
				t = append(t, ' ')
			} else {
				t = append(t, '+')
			}
		default:
			t = append(t, s[i])
		}
	}
	return string(t), nil
}

func escape(s string, mode int) string {
	const upperhex = "0123456789ABCDEF"
	hexCount := 0
	spaceCount := 0
	for _, c := range []byte(s) {
		if shouldEscape(c, mode) {
			if c == ' ' && mode == encodeQueryComponent {
				spaceCount++
			} else {
				hexCount++
			}
		}
	}
	if hexCount == 0 && spaceCount == 0 {
		return s
	}
	t := make([]byte, 0, len(s)+2*hexCount)
	for _, c := range []byte(s) {
		if c == ' ' && mode == encodeQueryComponent {
			t = append(t, '+')
		} else if shouldEscape(c, mode) {
			t = append(t, '%', upperhex[c>>4], upperhex[c&15])
		} else {
			t = append(t, c)
		}
	}
	return string(t)
}

// PathEscape escapes the string so it can be safely placed inside a URL path segment.
func PathEscape(s string) string { return escape(s, encodePathSegment) }

// QueryEscape escapes the string so it can be safely placed inside a URL query.
func QueryEscape(s string) string { return escape(s, encodeQueryComponent) }

func getScheme(rawURL string) (string, string, error) {
	for i := 0; i < len(rawURL); i++ {
		c := rawURL[i]
		switch {
		case 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z':
		case '0' <= c && c <= '9' || c == '+' || c == '-' || c == '.':
			if i == 0 {
				return "", rawURL, nil
			}
		case c == ':':
			if i == 0 {
				return "", "", errors.New("missing protocol scheme")
			}
			return rawURL[:i], rawURL[i+1:], nil
		default:
			return "", rawURL, nil
		}
	}
	return "", rawURL, nil
}

// Parse parses a raw url into a URL structure, as net/url.Parse does.
func Parse(rawURL string) (*URL, error) {
	u, frag, _ := strings.Cut(rawURL, "#")
	url, err := parse(u, false)
	if err != nil {
		return nil, &Error{"parse", u, err}
	}
	if frag == "" {
		return url, nil
	}
	if err = url.setFragment(frag); err != nil {
		return nil, &Error{"parse", rawURL, err}
	}
	return url, nil
}

// ParseRequestURI parses an absolute URI or absolute path, as
// net/url.ParseRequestURI does: the fragment is not stripped.
func ParseRequestURI(rawURL string) (*URL, error) {
	url, err := parse(rawURL, true)
	if err != nil {
		return nil, &Error{"parse", rawURL, err}
	}
	return url, nil
}

func stringContainsCTLByte(s string) bool {
	for _, b := range []byte(s) {
		if b < ' ' || b == 0x7f {
			return true
		}
	}
	return false
}

func parse(rawURL string, viaRequest bool) (*URL, error) {
	if stringContainsCTLByte(rawURL) {
		return nil, errors.New("net/url: invalid control character in URL")
	}
	if rawURL == "" && viaRequest {
		return nil, errors.New("empty url")
	}
	url := &URL{}
	if rawURL == "*" {
		url.Path = "*"
		return url, nil
	}
	scheme, rest, err := getScheme(rawURL)
	if err != nil {
		return nil, err
	}
	url.Scheme = strings.ToLower(scheme)
	if strings.HasSuffix(rest, "?") && strings.Count(rest, "?") == 1 {
		url.ForceQuery = true
		rest = rest[:len(rest)-1]
	} else {
		rest, url.RawQuery, _ = strings.Cut(rest, "?")
	}
	if !strings.HasPrefix(rest, "/") {
		if url.Scheme != "" {
			url.Opaque = rest
			return url, nil
		}
		if viaRequest {
			return nil, errors.New("invalid URI for request")
		}
		if segment, _, _ := strings.Cut(rest, "/"); strings.Contains(segment, ":") {
			return nil, errors.New("first path segment in URL cannot contain colon")
		}
	}
	if (url.Scheme != "" || !viaRequest && !strings.HasPrefix(rest, "///")) && strings.HasPrefix(rest, "//") {
		var authority string
		authority, rest = rest[2:], ""
		if i := strings.Index(authority, "/"); i >= 0 {
			authority, rest = authority[:i], authority[i:]
		}
		url.User, url.Host, err = parseAuthority(authority)
		if err != nil {
			return nil, err
		}
	} else if url.Scheme != "" && strings.HasPrefix(rest, "/") {
		url.OmitHost = true
	}
	if err := url.setPath(rest); err != nil {
		return nil, err
	}
	return url, nil
}

func parseAuthority(authority string) (string, string, error) {
	i := strings.LastIndex(authority, "@")
	var host string
	var err error
	if i < 0 {
		host, err = parseHost(authority)
	} else {
		host, err = parseHost(authority[i+1:])
	}
	if err != nil {
		return "", "", err
	}
	if i < 0 {
		return "", host, nil
	}
	userinfo := authority[:i]
	if !validUserinfo(userinfo) {
		return "", "", errors.New("net/url: invalid userinfo")
	}
	return userinfo, host, nil
}

func parseHost(host string) (string, error) {
	if strings.HasPrefix(host, "[") {
		i := strings.LastIndex(host, "]")
		if i < 0 {
			return "", errors.New("missing ']' in host")
		}
		colonPort := host[i+1:]
		if !validOptionalPort(colonPort) {
			return "", errors.New("invalid port " + quote(colonPort) + " after host")
		}
		zone := strings.Index(host[:i], "%25")
		if zone >= 0 {
			host1, err := unescape(host[:zone], encodeHost)
			if err != nil {
				return "", err
			}
			host2, err := unescape(host[zone:i], encodeZone)
			if err != nil {
				return "", err
			}
			host3, err := unescape(host[i:], encodeHost)
			if err != nil {
				return "", err
			}
			return host1 + host2 + host3, nil
		}
	} else if i := strings.LastIndex(host, ":"); i != -1 {
		colonPort := host[i:]
		if !validOptionalPort(colonPort) {
			return "", errors.New("invalid port " + quote(colonPort) + " after host")
		}
	}
	var err error
	if host, err = unescape(host, encodeHost); err != nil {
		return "", err
	}
	return host, nil
}

const encodeZone = encodeFragment + 1

func validUserinfo(s string) bool {
	for _, r := range []byte(s) {
		if 'A' <= r && r <= 'Z' || 'a' <= r && r <= 'z' || '0' <= r && r <= '9' {
			continue
		}
		switch r {
		case '-', '.', '_', ':', '~', '!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '=', '%', '@':
			continue
		default:
			return false
		}
	}
	return true
}

func validOptionalPort(port string) bool {
	if port == "" {
		return true
	}
	if port[0] != ':' {
		return false
	}
	for _, b := range []byte(port[1:]) {
		if b < '0' || b > '9' {
			return false
		}
	}
	return true
}

func (u *URL) setPath(p string) error {
	path, err := unescape(p, encodePath)
	if err != nil {
		return err
	}
	u.Path = path
	if escp := escape(path, encodePath); p == escp {
		u.RawPath = ""
	} else {
		u.RawPath = p
	}
	return nil
}

func (u *URL) setFragment(f string) error {
	frag, err := unescape(f, encodeFragment)
	if err != nil {
		return err
	}
	u.Fragment = frag
	return nil
}

// EscapedPath returns the escaped form of u.Path.
func (u *URL) EscapedPath() string {
	if u.RawPath != "" && validEncoded(u.RawPath, encodePath) {
		p, err := unescape(u.RawPath, encodePath)
		if err == nil && p == u.Path {
			return u.RawPath
		}
	}
	if u.Path == "*" {
		return "*"
	}
	return escape(u.Path, encodePath)
}

func validEncoded(s string, mode int) bool {
	for i := 0; i < len(s); i++ {
		switch s[i] {
		case '!', '$', '&', '\'', '(', ')', '*', '+', ',', ';', '=', ':', '@':
		case '[', ']':
		case '%':
		default:
			if shouldEscape(s[i], mode) {
				return false
			}
		}
	}
	return true
}

// String reassembles the URL into a valid URL string, as net/url does.
func (u *URL) String() string {
	var buf strings.Builder
	if u.Scheme != "" {
		buf.WriteString(u.Scheme)
		buf.WriteByte(':')
	}
	if u.Opaque != "" {
		buf.WriteString(u.Opaque)
	} else {
		if u.Scheme != "" || u.Host != "" || u.User != "" {
			if u.OmitHost && u.Host == "" && u.User == "" {
			} else {
				if u.Host != "" || u.Path != "" || u.User != "" {
					buf.WriteString("//")
				}
				if u.User != "" {
					buf.WriteString(u.User)
					buf.WriteByte('@')
				}
				if h := u.Host; h != "" {
					buf.WriteString(escape(h, encodeHost))
				}
			}
		}
		path := u.EscapedPath()
		if path != "" && path[0] != '/' && u.Host != "" {
			buf.WriteByte('/')
		}
		if buf.Len() == 0 {
			if segment, _, _ := strings.Cut(path, "/"); strings.Contains(segment, ":") {
				buf.WriteString("./")
			}
		}
		buf.WriteString(path)
	}
	if u.ForceQuery || u.RawQuery != "" {
		buf.WriteByte('?')
		buf.WriteString(u.RawQuery)
	}
	if u.Fragment != "" {
		buf.WriteByte('#')
		buf.WriteString(escape(u.Fragment, encodeFragment))
	}
	return buf.String()
}

// IsAbs reports whether the URL is absolute.
func (u *URL) IsAbs() bool { return u.Scheme != "" }

// Hostname returns u.Host, stripping any valid port number if present.
func (u *URL) Hostname() string {
	host, _ := splitHostPort(u.Host)
	return host
}

// Port returns the port part of u.Host, without the leading colon.
func (u *URL) Port() string {
	_, port := splitHostPort(u.Host)
	return port
}

func splitHostPort(hostPort string) (string, string) {
	host := hostPort
	port := ""
	colon := strings.LastIndex(host, ":")
	if colon != -1 && validOptionalPort(host[colon:]) {
		host, port = host[:colon], host[colon+1:]
	}
	if strings.HasPrefix(host, "[") && strings.HasSuffix(host, "]") {
		host = host[1 : len(host)-1]
	}
	return host, port
}

// JoinPath returns a new URL with the provided path elements joined to any
// existing path and the resulting path cleaned of any ./ or ../ elements.
func (u *URL) JoinPath(elem ...string) *URL {
	elem = append([]string{u.EscapedPath()}, elem...)
	var p string
	if !strings.HasPrefix(elem[0], "/") {
		elem[0] = "/" + elem[0]
		p = pathJoin(elem)[1:]
	} else {
		p = pathJoin(elem)
	}
	if strings.HasSuffix(elem[len(elem)-1], "/") && !strings.HasSuffix(p, "/") {
		p += "/"
	}
	url := *u
	url.setPath(p)
	return &url
}

// JoinPath returns a URL string with the provided path elements joined to
// the existing path of base and the resulting path cleaned of any ./ or ../ elements.
func JoinPath(base string, elem ...string) (string, error) {
	u, err := Parse(base)
	if err != nil {
		return "", err
	}
	return u.JoinPath(elem...).String(), nil
}

// JoinHostPort combines host and port into a network address of the form
// "host:port", bracketing an IPv6 literal, as net.JoinHostPort does.
func JoinHostPort(host, port string) string {
	if strings.Index(host, ":") >= 0 {
		return "[" + host + "]:" + port
	}
	return host + ":" + port
}

func pathJoin(elem []string) string {
	size := 0
	for _, e := range elem {
		size += len(e)
	}
	if size == 0 {
		return ""
	}
	buf := make([]byte, 0, size+len(elem)-1)
	for _, e := range elem {
		if len(buf) > 0 || e != "" {
			if len(buf) > 0 {
				buf = append(buf, '/')
			}
			buf = append(buf, e...)
		}
	}
	return pathClean(string(buf))
}

func pathClean(path string) string {
	if path == "" {
		return "."
	}
	rooted := path[0] == '/'
	n := len(path)
	out := make([]byte, 0, n)
	r, dotdot := 0, 0
	if rooted {
		out = append(out, '/')
		r, dotdot = 1, 1
	}
	for r < n {
		switch {
		case path[r] == '/':
			r++
		case path[r] == '.' && (r+1 == n || path[r+1] == '/'):
			r++
		case path[r] == '.' && path[r+1] == '.' && (r+2 == n || path[r+2] == '/'):
			r += 2
			switch {
			case len(out) > dotdot:
				w := len(out) - 1
				for w > dotdot && out[w] != '/' {
					w--
				}
				out = out[:w]
			case !rooted:
				if len(out) > 0 {
					out = append(out, '/')
				}
				out = append(out, '.', '.')
				dotdot = len(out)
			}
		default:
			if rooted && len(out) != 1 || !rooted && len(out) != 0 {
				out = append(out, '/')
			}
			for ; r < n && path[r] != '/'; r++ {
				out = append(out, path[r])
			}
		}
	}
	if len(out) == 0 {
		return "."
	}
	return string(out)
}
