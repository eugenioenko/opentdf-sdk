package json

import (
	"bytes"
	"testing"

	"github.com/eugenioenko/goalchemy/lib/errors"
)

// The string reader and writer copy runs of plain bytes instead of one byte or
// rune at a time. These references are the previous per-rune implementations;
// the run-based ones must match them byte for byte, including which error is
// reported first.
func referenceWriterStr(w *writer, s string) error {
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

func referenceParserStr(p *parser) (string, error) {
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

// Pieces chosen to hit every branch: plain ASCII runs, quote and backslash,
// control bytes, valid 2/3/4-byte UTF-8, overlong/surrogate/out-of-range and
// truncated sequences, and (for the reader) every escape form.
var strPieces = []string{
	"a", "plain ascii run ", "~", " ", "\"", "\\", "\x00", "\x1f", "\n", "\x7f",
	"é", "中", "😃", "\xc0\xaf", "\xed\xa0\x80", "\xf4\x90\x80\x80", "\x80", "\xff", "\xe4\xb8",
	`\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `é`, `😃`, `\ud83d`, `\ude03`, `\u12`, `\x`,
}

func TestStringRunsMatchReference(t *testing.T) {
	seed := uint32(2463534242)
	next := func(n int) int {
		seed ^= seed << 13
		seed ^= seed >> 17
		seed ^= seed << 5
		return int(seed % uint32(n))
	}
	for iter := 0; iter < 20000; iter++ {
		var b bytes.Buffer
		for k := next(12); k >= 0; k-- {
			b.WriteString(strPieces[next(len(strPieces))])
		}
		s := b.String()
		l := Limits{Bytes: 1 + next(64), Depth: 8, Nodes: 64, StringBytes: 1 + next(48)}

		got, want := writer{limits: l}, writer{limits: l}
		ge, we := got.str(s), referenceWriterStr(&want, s)
		if !sameError(ge, we) || (we == nil && !bytes.Equal(got.out, want.out)) {
			t.Fatalf("writer %q limits %+v: got (%q, %v) want (%q, %v)", s, l, got.out, ge, want.out, we)
		}

		data := []byte("\"" + s + "\"")
		gp, wp := parser{data: data, limits: l}, parser{data: data, limits: l}
		gs, gpe := gp.str()
		ws, wpe := referenceParserStr(&wp)
		if !sameError(gpe, wpe) || gs != ws || (wpe == nil && gp.pos != wp.pos) {
			t.Fatalf("parser %q limits %+v: got (%q, %v, %d) want (%q, %v, %d)", data, l, gs, gpe, gp.pos, ws, wpe, wp.pos)
		}
	}
}

func sameError(a, b error) bool {
	if a == nil || b == nil {
		return a == nil && b == nil
	}
	return a.Error() == b.Error()
}
