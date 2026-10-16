package sdk

import (
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/net/url"
	"github.com/eugenioenko/goalchemy/std/strings"
)

var ErrInvalid = errors.New("invalid type")

// attributeNameFQN represents the FQN for an attribute. The exported
// AttributeNameFQN and its methods arrive with the granter in Phase 4.
type attributeNameFQN struct {
	url, key string
}

func attributeURLPartsValid(parts []string) error {
	for i, part := range parts {
		if part == "" {
			return fmt.Errorf("%w: empty url path parts are not allowed", ErrInvalid)
		} else if i > 1 && // skip first two parts as they will have protocol slashes
			strings.Contains(part, "/") {
			return fmt.Errorf("%w: slash not allowed in name or values", ErrInvalid)
		}
	}
	return nil
}

func newAttributeNameFQN(u string) (attributeNameFQN, error) {
	m := matchAttributeNameFQN(u)
	if len(m) < 3 || len(m[0]) == 0 {
		return attributeNameFQN{}, fmt.Errorf("%w: attribute regex fail", ErrInvalid)
	}

	_, err := url.PathUnescape(m[2])
	if err != nil {
		return attributeNameFQN{}, fmt.Errorf("%w: error in attribute name [%s]", ErrInvalid, m[2])
	}

	if err := attributeURLPartsValid(m); err != nil {
		return attributeNameFQN{}, err
	}

	return attributeNameFQN{u, strings.ToLower(u)}, nil
}

func (a attributeNameFQN) String() string {
	return a.url
}

// attributeValueFQN is a utility type to represent an FQN for an attribute value.
type attributeValueFQN struct {
	url, key string
}

func newAttributeValueFQN(u string) (attributeValueFQN, error) {
	m := matchAttributeValueFQN(u)
	if len(m) < 4 || len(m[0]) == 0 {
		return attributeValueFQN{}, fmt.Errorf("%w: attribute regex fail for [%s]", ErrInvalid, u)
	}

	_, err := url.PathUnescape(m[2])
	if err != nil {
		return attributeValueFQN{}, fmt.Errorf("%w: error in attribute name [%s]", ErrInvalid, m[2])
	}
	_, err = url.PathUnescape(m[3])
	if err != nil {
		return attributeValueFQN{}, fmt.Errorf("%w: error in attribute value [%s]", ErrInvalid, m[3])
	}

	if err := attributeURLPartsValid(m); err != nil {
		return attributeValueFQN{}, err
	}

	return attributeValueFQN{u, strings.ToLower(u)}, nil
}

func (a attributeValueFQN) String() string {
	return a.url
}

// matchAttributeNameFQN is the reference's ^(https?://[\w./-]+)/attr/([^/\s]*)$
// as a parser: the submatches, or nil. The name holds no slash, so its
// "/attr/" ends at the last slash.
func matchAttributeNameFQN(u string) []string {
	slash := strings.LastIndex(u, "/")
	if slash < len("/attr") || u[slash-len("/attr"):slash] != "/attr" {
		return nil
	}
	authority, name := u[:slash-len("/attr")], u[slash+1:]
	if !isAttributeAuthority(authority) || strings.IndexAny(name, regexpSpace) >= 0 {
		return nil
	}
	return []string{u, authority, name}
}

// matchAttributeValueFQN is the reference's
// ^(https?://[\w./-]+)/attr/(\S*)/value/(\S*)$ as a parser, with the
// regexp's greedy choice: the last "/attr/" the authority can reach, then
// the last "/value/".
func matchAttributeValueFQN(u string) []string {
	if strings.IndexAny(u, regexpSpace) >= 0 {
		return nil
	}
	for end := strings.LastIndex(u, "/attr/"); end >= 0; end = strings.LastIndex(u[:end+len("/attr")], "/attr/") {
		rest := u[end+len("/attr/"):]
		value := strings.LastIndex(rest, "/value/")
		if value >= 0 && isAttributeAuthority(u[:end]) {
			return []string{u, u[:end], rest[:value], rest[value+len("/value/"):]}
		}
	}
	return nil
}

// regexpSpace is RE2's \s.
const regexpSpace = "\t\n\f\r "

// isAttributeAuthority matches https?://[\w./-]+ in full.
func isAttributeAuthority(s string) bool {
	body := ""
	switch {
	case strings.HasPrefix(s, "https://"):
		body = s[len("https://"):]
	case strings.HasPrefix(s, "http://"):
		body = s[len("http://"):]
	default:
		return false
	}
	if body == "" {
		return false
	}
	for i := 0; i < len(body); i++ {
		c := body[i]
		if !(c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c >= '0' && c <= '9' || c == '_' || c == '.' || c == '/' || c == '-') {
			return false
		}
	}
	return true
}
