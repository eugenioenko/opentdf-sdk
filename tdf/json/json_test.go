package json

import (
	"bytes"
	std "encoding/json"
	"fmt"
	"reflect"
	"testing"
)

func TestIndependentRoundTrips(t *testing.T) {
	vectors := []string{`null`, `true`, `false`, `-42`, `1.234e-05`, `{"nul":"\u0000","pair":"\ud83d\ude03","raw":"é中😃","esc":"\"\\\/\b\f\n\r\t","array":[1,{},[],false,null]}`}
	for _, s := range vectors {
		v, e := Parse([]byte(s), DefaultLimits())
		if e != nil {
			t.Fatal(e)
		}
		b, e := Marshal(v, DefaultLimits())
		if e != nil {
			t.Fatal(e)
		}
		var want, got any
		d := std.NewDecoder(bytes.NewBufferString(s))
		d.UseNumber()
		if e = d.Decode(&want); e != nil {
			t.Fatal(e)
		}
		d = std.NewDecoder(bytes.NewReader(b))
		d.UseNumber()
		if e = d.Decode(&got); e != nil {
			t.Fatal(e)
		}
		if !reflect.DeepEqual(want, got) {
			t.Fatalf("%s => %s", s, b)
		}
		b2, e := Marshal(v, DefaultLimits())
		if e != nil || !bytes.Equal(b, b2) {
			t.Fatal("nondeterministic")
		}
	}
	// Stdlib writer supplies every control byte, quotes, Unicode, and HTML chars.
	s := ""
	for i := 0; i < 32; i++ {
		s += string(rune(i))
	}
	s += "é中😃\"\\<>&"
	b, _ := std.Marshal(s)
	v, e := Parse(b, DefaultLimits())
	if e != nil || v.Text != s {
		t.Fatal(v, e)
	}
	b, e = Marshal(Str(s), DefaultLimits())
	var got string
	if e != nil || std.Unmarshal(b, &got) != nil || got != s {
		t.Fatal(string(b), e)
	}
}
func TestRejectMalformed(t *testing.T) {
	bad := []string{"", `{`, `[1,]`, `{"a":1,}`, `{"a":1,"\u0061":2}`, `"\ud800"`, `"\udc00"`, `"\ud800\u1234"`, `"\uZZZZ"`, `"\x00"`, `"x`, "\"\x00\"", "\"\xc0\x80\"", "\"\xed\xa0\x80\"", "\"\xf4\x90\x80\x80\"", "\"\xe2\x82\"", `00`, `-01`, `1.`, `1e`, `+1`, `true false`, `{}x`, `[}`}
	for _, s := range bad {
		if _, e := Parse([]byte(s), DefaultLimits()); e == nil {
			t.Errorf("accepted %q", s)
		}
	}
	for _, v := range []Value{Num("00"), Num("1e"), Str("\xff"), Obj([]string{"a", "a"}, []Value{Bool(true), Bool(false)}), Obj([]string{"a"}, nil), {Kind: 99}} {
		if _, e := Marshal(v, DefaultLimits()); e == nil {
			t.Fatal("writer accepted invalid value", v)
		}
	}
}
func TestBounds(t *testing.T) {
	l := Limits{Bytes: 20, Depth: 1, Nodes: 3, StringBytes: 2}
	for _, s := range []string{`[[[]]]`, `[1,2,3]`, `"abc"`, `"abcdefghijklmnopqrst"`} {
		if _, e := Parse([]byte(s), l); e == nil {
			t.Fatal(s)
		}
	}
	for _, v := range []Value{Str("abc"), Arr([]Value{Int(1), Int(2), Int(3)}), Arr([]Value{Arr([]Value{Arr(nil)})})} {
		if _, e := Marshal(v, l); e == nil {
			t.Fatal(v)
		}
	}
	if _, e := Marshal(Str("a"), Limits{Bytes: 2, Depth: 1, Nodes: 1, StringBytes: 1}); e == nil {
		t.Fatal("output bound")
	}
	for _, s := range []string{"00", "-1", "1e2", "1.0", "9999999999999999999999999"} {
		if _, e := Num(s).Integer(100); e == nil {
			t.Fatal(s)
		}
	}
	if n, e := Num("100").Integer(100); e != nil || n != 100 {
		t.Fatal(n, e)
	}
}
func TestLargeUniqueObject(t *testing.T) {
	var b bytes.Buffer
	b.WriteByte('{')
	for i := 0; i < 20000; i++ {
		if i > 0 {
			b.WriteByte(',')
		}
		fmt.Fprintf(&b, "\"k%d\":%d", i, i)
	}
	b.WriteByte('}')
	v, e := Parse(b.Bytes(), DefaultLimits())
	if e != nil || len(v.Names) != 20000 {
		t.Fatal(e)
	}
	out, e := Marshal(v, DefaultLimits())
	if e != nil || !bytes.Equal(b.Bytes(), out) {
		t.Fatal(e)
	}
}
func FuzzParse(f *testing.F) {
	for _, s := range []string{`{"a":[]}`, `"\ud83d\ude03"`, `true`, `[1,null]`} {
		f.Add([]byte(s))
	}
	f.Fuzz(func(t *testing.T, b []byte) {
		l := Limits{Bytes: 4096, Depth: 8, Nodes: 256, StringBytes: 4096}
		v, e := Parse(b, l)
		if e != nil {
			return
		}
		outputLimits := l
		outputLimits.Bytes = 6 * l.Bytes
		out, e := Marshal(v, outputLimits)
		if e != nil {
			t.Fatal(e)
		}
		if !std.Valid(out) {
			t.Fatalf("invalid emitted JSON %q", out)
		}
	})
}
