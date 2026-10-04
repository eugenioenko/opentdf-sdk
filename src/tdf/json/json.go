// Package json implements bounded, strict JSON without host JSON dependencies.
package json

import "github.com/eugenioenko/goalchemy/lib/errors"

const (
	Null    = 0
	Object  = 1
	Array   = 2
	String  = 3
	Number  = 4
	Boolean = 5
)

// Value stores number lexemes exactly. Object Names correspond to Children.
// Object member order is retained; duplicate names are rejected by the codec.
type Value struct {
	Kind     int
	Text     string
	Bool     bool
	Names    []string
	Children []Value
}
type Limits struct {
	Bytes       int
	Depth       int
	Nodes       int
	StringBytes int
}

func DefaultLimits() Limits { return Limits{10 * 1024 * 1024, 32, 100000, 10 * 1024 * 1024} }
func validLimits(l Limits) bool {
	return l.Bytes > 0 && l.Depth > 0 && l.Depth <= 128 && l.Nodes > 0 && l.StringBytes > 0
}
func Str(s string) Value { return Value{Kind: String, Text: s} }
func Num(s string) Value { return Value{Kind: Number, Text: s} }
func Bool(b bool) Value  { return Value{Kind: Boolean, Bool: b} }
func Obj(names []string, values []Value) Value {
	return Value{Kind: Object, Names: names, Children: values}
}
func Arr(values []Value) Value { return Value{Kind: Array, Children: values} }
func (v Value) Get(name string) (Value, bool) {
	for i, n := range v.Names {
		if n == name && i < len(v.Children) {
			return v.Children[i], true
		}
	}
	return Value{}, false
}

type parser struct {
	data   []byte
	pos    int
	nodes  int
	limits Limits
}

func Parse(data []byte, l Limits) (Value, error) {
	if !validLimits(l) || len(data) > l.Bytes {
		return Value{}, errors.New("json: resource limit")
	}
	p := parser{data: data, limits: l}
	v, err := p.value(0)
	if err != nil {
		return Value{}, err
	}
	p.space()
	if p.pos != len(data) {
		return Value{}, errors.New("json: trailing input")
	}
	return v, nil
}
func (p *parser) space() {
	for p.pos < len(p.data) {
		b := p.data[p.pos]
		if b != ' ' && b != '\n' && b != '\r' && b != '\t' {
			break
		}
		p.pos++
	}
}
func (p *parser) value(depth int) (Value, error) {
	p.space()
	p.nodes++
	if depth > p.limits.Depth || p.nodes > p.limits.Nodes {
		return Value{}, errors.New("json: resource limit")
	}
	if p.pos >= len(p.data) {
		return Value{}, errors.New("json: truncated value")
	}
	b := p.data[p.pos]
	if b == '"' {
		s, e := p.str()
		return Str(s), e
	}
	if b == '{' || b == '[' {
		p.pos++
		v := Value{Kind: Array}
		close := byte(']')
		if b == '{' {
			v.Kind = Object
			close = '}'
		}
		p.space()
		if p.pos < len(p.data) && p.data[p.pos] == close {
			p.pos++
			return v, nil
		}
		seen := make(map[string]bool)
		for {
			if v.Kind == Object {
				p.space()
				if p.pos >= len(p.data) || p.data[p.pos] != '"' {
					return Value{}, errors.New("json: expected member")
				}
				n, e := p.str()
				if e != nil {
					return Value{}, e
				}
				if seen[n] {
					return Value{}, errors.New("json: duplicate member")
				}
				seen[n] = true
				v.Names = append(v.Names, n)
				p.space()
				if p.pos >= len(p.data) || p.data[p.pos] != ':' {
					return Value{}, errors.New("json: expected colon")
				}
				p.pos++
			}
			child, e := p.value(depth + 1)
			if e != nil {
				return Value{}, e
			}
			v.Children = append(v.Children, child)
			p.space()
			if p.pos >= len(p.data) {
				return Value{}, errors.New("json: truncated container")
			}
			c := p.data[p.pos]
			p.pos++
			if c == close {
				return v, nil
			}
			if c != ',' {
				return Value{}, errors.New("json: expected comma")
			}
		}
	}
	for _, word := range []string{"null", "true", "false"} {
		if len(p.data)-p.pos >= len(word) && string(p.data[p.pos:p.pos+len(word)]) == word {
			p.pos += len(word)
			if word == "null" {
				return Value{Kind: Null}, nil
			}
			return Bool(word == "true"), nil
		}
	}
	start := p.pos
	end := numberEnd(p.data, start)
	if end == start {
		return Value{}, errors.New("json: invalid value")
	}
	p.pos = end
	return Num(string(p.data[start:end])), nil
}
func numberEnd(b []byte, i int) int {
	start := i
	if i < len(b) && b[i] == '-' {
		i++
	}
	if i >= len(b) {
		return start
	}
	if b[i] == '0' {
		i++
	} else {
		if b[i] < '1' || b[i] > '9' {
			return start
		}
		for i < len(b) && b[i] >= '0' && b[i] <= '9' {
			i++
		}
	}
	if i < len(b) && b[i] == '.' {
		i++
		s := i
		for i < len(b) && b[i] >= '0' && b[i] <= '9' {
			i++
		}
		if i == s {
			return start
		}
	}
	if i < len(b) && (b[i] == 'e' || b[i] == 'E') {
		i++
		if i < len(b) && (b[i] == '+' || b[i] == '-') {
			i++
		}
		s := i
		for i < len(b) && b[i] >= '0' && b[i] <= '9' {
			i++
		}
		if s == i {
			return start
		}
	}
	return i
}
func hex(b byte) int {
	if b >= '0' && b <= '9' {
		return int(b - '0')
	}
	if b >= 'a' && b <= 'f' {
		return int(b-'a') + 10
	}
	if b >= 'A' && b <= 'F' {
		return int(b-'A') + 10
	}
	return -1
}
func (p *parser) hex4() (int, error) {
	if len(p.data)-p.pos < 4 {
		return 0, errors.New("json: truncated unicode")
	}
	n := 0
	for k := 0; k < 4; k++ {
		h := hex(p.data[p.pos])
		p.pos++
		if h < 0 {
			return 0, errors.New("json: invalid unicode escape")
		}
		n = n*16 + h
	}
	return n, nil
}
func addRune(b []byte, n int) []byte {
	if n < 128 {
		return append(b, byte(n))
	}
	if n < 2048 {
		return append(b, byte(192|(n>>6)), byte(128|(n&63)))
	}
	if n < 65536 {
		return append(b, byte(224|(n>>12)), byte(128|((n>>6)&63)), byte(128|(n&63)))
	}
	return append(b, byte(240|(n>>18)), byte(128|((n>>12)&63)), byte(128|((n>>6)&63)), byte(128|(n&63)))
}

// utf8Width rejects overlong encodings, surrogate code points, and >U+10FFFF.
func utf8Width(b []byte, i int) int {
	c := b[i]
	if c < 128 {
		return 1
	}
	n := 0
	cp := 0
	min := 0
	if c >= 194 && c <= 223 {
		n = 2
		cp = int(c & 31)
		min = 128
	} else if c >= 224 && c <= 239 {
		n = 3
		cp = int(c & 15)
		min = 2048
	} else if c >= 240 && c <= 244 {
		n = 4
		cp = int(c & 7)
		min = 65536
	} else {
		return 0
	}
	if len(b)-i < n {
		return 0
	}
	for k := 1; k < n; k++ {
		d := b[i+k]
		if d < 128 || d > 191 {
			return 0
		}
		cp = cp*64 + int(d&63)
	}
	if cp < min || cp > 1114111 || (cp >= 55296 && cp <= 57343) {
		return 0
	}
	return n
}
func (p *parser) str() (string, error) {
	p.pos++
	out := []byte{}
	for p.pos < len(p.data) {
		c := p.data[p.pos]
		p.pos++
		if c == '"' {
			return string(out), nil
		}
		if c < 32 {
			return "", errors.New("json: control in string")
		}
		if c == '\\' {
			if p.pos >= len(p.data) {
				return "", errors.New("json: truncated escape")
			}
			c = p.data[p.pos]
			p.pos++
			switch c {
			case '"', '\\', '/':
				out = append(out, c)
			case 'b':
				out = append(out, 8)
			case 'f':
				out = append(out, 12)
			case 'n':
				out = append(out, 10)
			case 'r':
				out = append(out, 13)
			case 't':
				out = append(out, 9)
			case 'u':
				n, e := p.hex4()
				if e != nil {
					return "", e
				}
				if n >= 55296 && n <= 56319 {
					if len(p.data)-p.pos < 2 || p.data[p.pos] != '\\' || p.data[p.pos+1] != 'u' {
						return "", errors.New("json: missing low surrogate")
					}
					p.pos += 2
					low, e := p.hex4()
					if e != nil {
						return "", e
					}
					if low < 56320 || low > 57343 {
						return "", errors.New("json: invalid low surrogate")
					}
					n = 65536 + (n-55296)*1024 + low - 56320
				} else if n >= 56320 && n <= 57343 {
					return "", errors.New("json: lone low surrogate")
				}
				out = addRune(out, n)
			default:
				return "", errors.New("json: invalid escape")
			}
		} else {
			p.pos--
			n := utf8Width(p.data, p.pos)
			if n == 0 {
				return "", errors.New("json: invalid UTF-8")
			}
			out = append(out, p.data[p.pos:p.pos+n]...)
			p.pos += n
		}
		if len(out) > p.limits.StringBytes {
			return "", errors.New("json: string limit")
		}
	}
	return "", errors.New("json: unterminated string")
}

// Integer accepts only nonnegative integer lexemes, checking the caller's bound.
func (v Value) Integer(max int) (int, error) {
	if v.Kind != Number || len(v.Text) == 0 || max < 0 || (len(v.Text) > 1 && v.Text[0] == '0') {
		return 0, errors.New("json: expected integer")
	}
	n := 0
	for i := 0; i < len(v.Text); i++ {
		c := v.Text[i]
		if c < '0' || c > '9' {
			return 0, errors.New("json: expected nonnegative integer")
		}
		d := int(c - '0')
		if n > max/10 || (n == max/10 && d > max%10) {
			return 0, errors.New("json: integer limit")
		}
		n = n*10 + d
	}
	return n, nil
}
func Int(n int) Value {
	if n == 0 {
		return Num("0")
	}
	neg := n < 0
	digits := []byte{}
	for n != 0 {
		d := n % 10
		if d < 0 {
			d = -d
		}
		digits = append(digits, byte(d)+48)
		n /= 10
	}
	out := []byte{}
	if neg {
		out = append(out, '-')
	}
	for i := len(digits) - 1; i >= 0; i-- {
		out = append(out, digits[i])
	}
	return Num(string(out))
}

type writer struct {
	out    []byte
	limits Limits
	nodes  int
}

func Marshal(v Value, l Limits) ([]byte, error) {
	if !validLimits(l) {
		return nil, errors.New("json: resource limit")
	}
	w := writer{limits: l}
	err := w.value(v, 0)
	if err != nil {
		return nil, err
	}
	return w.out, nil
}
func (w *writer) emit(b []byte) error {
	if len(b) > w.limits.Bytes-len(w.out) {
		return errors.New("json: output limit")
	}
	w.out = append(w.out, b...)
	return nil
}
func (w *writer) str(s string) error {
	if len(s) > w.limits.StringBytes {
		return errors.New("json: string limit")
	}
	if e := w.emit([]byte{'"'}); e != nil {
		return e
	}
	b := []byte(s)
	for i := 0; i < len(b); {
		c := b[i]
		if c < 32 {
			h := "0123456789abcdef"
			if e := w.emit([]byte{'\\', 'u', '0', '0', h[c>>4], h[c&15]}); e != nil {
				return e
			}
			i++
		} else if c == '"' || c == '\\' {
			if e := w.emit([]byte{'\\', c}); e != nil {
				return e
			}
			i++
		} else {
			n := utf8Width(b, i)
			if n == 0 {
				return errors.New("json: invalid UTF-8")
			}
			if e := w.emit(b[i : i+n]); e != nil {
				return e
			}
			i += n
		}
	}
	return w.emit([]byte{'"'})
}
func (w *writer) value(v Value, depth int) error {
	w.nodes++
	if depth > w.limits.Depth || w.nodes > w.limits.Nodes {
		return errors.New("json: resource limit")
	}
	switch v.Kind {
	case Null:
		return w.emit([]byte("null"))
	case Boolean:
		if v.Bool {
			return w.emit([]byte("true"))
		}
		return w.emit([]byte("false"))
	case String:
		return w.str(v.Text)
	case Number:
		if len(v.Text) > w.limits.Bytes {
			return errors.New("json: number limit")
		}
		b := []byte(v.Text)
		if len(b) == 0 || numberEnd(b, 0) != len(b) {
			return errors.New("json: invalid number")
		}
		return w.emit(b)
	case Object, Array:
		if v.Kind == Object && len(v.Names) != len(v.Children) {
			return errors.New("json: object shape")
		}
		open := byte('[')
		close := byte(']')
		if v.Kind == Object {
			open = '{'
			close = '}'
		}
		if e := w.emit([]byte{open}); e != nil {
			return e
		}
		seen := make(map[string]bool)
		for i, c := range v.Children {
			if i > 0 {
				if e := w.emit([]byte{','}); e != nil {
					return e
				}
			}
			if v.Kind == Object {
				if seen[v.Names[i]] {
					return errors.New("json: duplicate member")
				}
				seen[v.Names[i]] = true
				if e := w.str(v.Names[i]); e != nil {
					return e
				}
				if e := w.emit([]byte{':'}); e != nil {
					return e
				}
			}
			if e := w.value(c, depth+1); e != nil {
				return e
			}
		}
		return w.emit([]byte{close})
	default:
		return errors.New("json: invalid kind")
	}
}
