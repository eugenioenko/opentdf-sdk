package sdk

import (
	"math/rand"
	"reflect"
	"regexp"
	"testing"
)

// The FQN parsers replace the reference's regular expressions; they must
// return the same submatches as Go's regexp for every input.
func TestAttributeFQNMatchesRegexp(t *testing.T) {
	name := regexp.MustCompile(`^(https?://[\w./-]+)/attr/([^/\s]*)$`)
	value := regexp.MustCompile(`^(https?://[\w./-]+)/attr/(\S*)/value/(\S*)$`)
	pieces := []string{"http://", "https://", "hxxp://", "e", "a-b.com", "/", "/attr/", "/attr", "/value/", "/value", "attr", "value", ".", "_", "-", " ", "\t", "%20", "%", "😁", "x", "1", ":"}
	r := rand.New(rand.NewSource(1))
	check := func(u string) {
		if got, want := matchAttributeNameFQN(u), name.FindStringSubmatch(u); !reflect.DeepEqual(got, want) {
			t.Fatalf("name %q: %q, regexp %q", u, got, want)
		}
		if got, want := matchAttributeValueFQN(u), value.FindStringSubmatch(u); !reflect.DeepEqual(got, want) {
			t.Fatalf("value %q: %q, regexp %q", u, got, want)
		}
	}
	for _, u := range []string{"", "http://e/attr/a", "http://e/attr/", "http:///attr/a", "http://e/attr/a/value/b/attr/c/value/d", "https://e/x/attr/a/attr/b/value/c/value/d", "http://e/attr/a b/value/c"} {
		check(u)
	}
	for i := 0; i < 200000; i++ {
		u := ""
		if r.Intn(4) > 0 {
			u = pieces[r.Intn(2)]
		}
		for n := r.Intn(9); n > 0; n-- {
			u += pieces[r.Intn(len(pieces))]
		}
		check(u)
	}
}
