package sdk

// endpoint accepts a deliberately bounded ASCII DNS/IPv4 URL profile. It rejects
// escaped paths, IPv6, IDNA, userinfo, queries, fragments, dot segments and
// ambiguous numeric hosts. Unsupported canonicalization always fails closed.
type endpoint struct {
	origin string
	path   string
}

func lower(s string) string {
	b := []byte(s)
	for i, c := range b {
		if c >= 'A' && c <= 'Z' {
			b[i] = c + 32
		}
	}
	return string(b)
}
func parseEndpoint(raw string, insecure bool) (endpoint, error) {
	if len(raw) == 0 || len(raw) > 4096 {
		return endpoint{}, failure("url", "invalid_destination", nil)
	}
	for _, c := range []byte(raw) {
		if c < 33 || c > 126 || c == '\\' || c == '%' || c == '?' || c == '#' || c == '@' {
			return endpoint{}, failure("url", "invalid_destination", nil)
		}
	}
	pos := -1
	for i := 0; i+2 < len(raw); i++ {
		if raw[i:i+3] == "://" {
			pos = i
			break
		}
	}
	if pos < 0 {
		return endpoint{}, failure("url", "invalid_destination", nil)
	}
	scheme := lower(raw[:pos])
	if scheme != "https" && (scheme != "http" || !insecure) {
		return endpoint{}, failure("url", "invalid_destination", nil)
	}
	rest := raw[pos+3:]
	end := len(rest)
	for i, c := range []byte(rest) {
		if c == '/' {
			end = i
			break
		}
	}
	authority := lower(rest[:end])
	path := rest[end:]
	host := authority
	port := ""
	for i, c := range []byte(authority) {
		if c == ':' {
			if port != "" {
				return endpoint{}, failure("url", "invalid_destination", nil)
			}
			host = authority[:i]
			port = authority[i+1:]
			if len(port) == 0 {
				return endpoint{}, failure("url", "invalid_destination", nil)
			}
			break
		}
	}
	if len(host) == 0 || len(host) > 253 || host[len(host)-1] == '.' {
		return endpoint{}, failure("url", "invalid_destination", nil)
	}
	numeric := true
	labels := 0
	start := 0
	for i := 0; i <= len(host); i++ {
		if i == len(host) || host[i] == '.' {
			label := host[start:i]
			if len(label) == 0 || len(label) > 63 || label[0] == '-' || label[len(label)-1] == '-' {
				return endpoint{}, failure("url", "invalid_destination", nil)
			}
			for _, c := range []byte(label) {
				if !(c >= 'a' && c <= 'z' || c >= '0' && c <= '9' || c == '-') {
					return endpoint{}, failure("url", "invalid_destination", nil)
				}
				if c < '0' || c > '9' {
					numeric = false
				}
			}
			labels++
			start = i + 1
		}
	}
	if !numeric {
		lastStart := 0
		for i, b := range []byte(host) {
			if b == '.' {
				lastStart = i + 1
			}
		}
		last := host[lastStart:]
		decimal := true
		for _, b := range []byte(last) {
			if b < '0' || b > '9' {
				decimal = false
			}
		}
		hexadecimal := len(last) >= 2 && last[:2] == "0x"
		if hexadecimal {
			for _, b := range []byte(last[2:]) {
				if !(b >= '0' && b <= '9' || b >= 'a' && b <= 'f') {
					hexadecimal = false
				}
			}
		}
		if decimal || hexadecimal {
			return endpoint{}, failure("url", "invalid_destination", nil)
		}
	}
	if numeric {
		if labels != 4 {
			return endpoint{}, failure("url", "invalid_destination", nil)
		}
		start = 0
		for i := 0; i <= len(host); i++ {
			if i == len(host) || host[i] == '.' {
				label := host[start:i]
				if len(label) > 3 {
					return endpoint{}, failure("url", "invalid_destination", nil)
				}
				n := 0
				if len(label) > 1 && label[0] == '0' {
					return endpoint{}, failure("url", "invalid_destination", nil)
				}
				for _, c := range []byte(label) {
					n = n*10 + int(c-'0')
				}
				if n > 255 {
					return endpoint{}, failure("url", "invalid_destination", nil)
				}
				start = i + 1
			}
		}
	}
	if port != "" {
		n := 0
		if len(port) > 5 || port[0] == '0' {
			return endpoint{}, failure("url", "invalid_destination", nil)
		}
		for _, c := range []byte(port) {
			if c < '0' || c > '9' {
				return endpoint{}, failure("url", "invalid_destination", nil)
			}
			n = n*10 + int(c-'0')
		}
		if n < 1 || n > 65535 {
			return endpoint{}, failure("url", "invalid_destination", nil)
		}
		if scheme == "https" && port == "443" || scheme == "http" && port == "80" {
			port = ""
		}
	}
	// Paths are unescaped RFC3986 unreserved characters and slash only.
	for _, c := range []byte(path) {
		if !(c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c >= '0' && c <= '9' || c == '/' || c == '-' || c == '_' || c == '.' || c == '~') {
			return endpoint{}, failure("url", "invalid_destination", nil)
		}
	}
	start = 0
	for i := 0; i <= len(path); i++ {
		if i == len(path) || path[i] == '/' {
			segment := path[start:i]
			if segment == "." || segment == ".." || i > 0 && segment == "" && i < len(path) {
				return endpoint{}, failure("url", "invalid_destination", nil)
			}
			start = i + 1
		}
	}
	if path == "/" {
		path = ""
	} else if len(path) > 0 && path[len(path)-1] == '/' {
		path = path[:len(path)-1]
	}
	origin := scheme + "://" + host
	if port != "" {
		origin += ":" + port
	}
	return endpoint{origin: origin, path: path}, nil
}
func (u endpoint) String() string { return u.origin + u.path }
