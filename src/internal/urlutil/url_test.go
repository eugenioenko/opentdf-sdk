package urlutil

import (
	"net"
	"net/url"
	"testing"
)

var parseCases = []string{
	"https://example.com", "https://example.com:8080", "https://example.com/path", "https://:8080", "example.com",
	"https://exa mple.com", "https://example:com", "http://localhost:8080/kas", "http://localhost:8080/kas/",
	"https://kas.example.org/path?x=1#frag", "HTTPS://Example.COM/A%20B/c", "https://example.com:443", "example.com:8080",
	"http://:8080", "", "https://[::1]:8443/kas", "http://user:pass@host.example/p", "mailto:someone@example.com",
	"/relative/path", "relative/path", "https://example.com/a/../b/./c", "http://example.com?",
	"http://example.com/%zz", "https://example.com/a%2Fb", "http://exam%41ple.com/", "https://example.com/#",
	"https://example.com/path with space", "*", "foo:bar:baz", ":no-scheme", "1http://example.com",
	"https://example.com:08080/", "https://example.com:port/", "http://[::1", "http://x/a?b=c&d=e#f%20g",
}

func TestParseMatchesNetURL(t *testing.T) {
	for _, raw := range parseCases {
		want, wantErr := url.Parse(raw)
		got, gotErr := Parse(raw)
		if (wantErr == nil) != (gotErr == nil) {
			t.Fatalf("Parse(%q): err %v vs %v", raw, gotErr, wantErr)
		}
		if wantErr != nil {
			if gotErr.Error() != wantErr.Error() {
				t.Fatalf("Parse(%q): error text %q vs %q", raw, gotErr.Error(), wantErr.Error())
			}
			continue
		}
		user := ""
		if want.User != nil {
			user = want.User.String()
		}
		if got.Scheme != want.Scheme || got.Opaque != want.Opaque || got.User != user || got.Host != want.Host || got.Path != want.Path || got.RawPath != want.RawPath || got.RawQuery != want.RawQuery || got.Fragment != want.Fragment || got.ForceQuery != want.ForceQuery {
			t.Fatalf("Parse(%q): %+v vs %+v", raw, *got, *want)
		}
		if got.Hostname() != want.Hostname() || got.Port() != want.Port() || got.String() != want.String() || got.IsAbs() != want.IsAbs() {
			t.Fatalf("Parse(%q): hostname/port/string %q %q %q vs %q %q %q", raw, got.Hostname(), got.Port(), got.String(), want.Hostname(), want.Port(), want.String())
		}
	}
}

func TestParseRequestURIMatchesNetURL(t *testing.T) {
	for _, raw := range parseCases {
		want, wantErr := url.ParseRequestURI(raw)
		got, gotErr := ParseRequestURI(raw)
		if (wantErr == nil) != (gotErr == nil) {
			t.Fatalf("ParseRequestURI(%q): err %v vs %v", raw, gotErr, wantErr)
		}
		if wantErr != nil {
			if gotErr.Error() != wantErr.Error() {
				t.Fatalf("ParseRequestURI(%q): error text %q vs %q", raw, gotErr.Error(), wantErr.Error())
			}
			continue
		}
		if got.Hostname() != want.Hostname() || got.String() != want.String() || got.Path != want.Path {
			t.Fatalf("ParseRequestURI(%q): %q %q vs %q %q", raw, got.Hostname(), got.String(), want.Hostname(), want.String())
		}
	}
}

func TestJoinPathMatchesNetURL(t *testing.T) {
	cases := []struct {
		base string
		elem []string
	}{
		{"http://localhost:8080", []string{"grpc.health.v1.Health", "Check"}},
		{"http://localhost:8080/", []string{"grpc.health.v1.Health", "Check"}},
		{"http://localhost:8080/api", []string{".well-known/openid-configuration"}},
		{"http://localhost:8888/auth/realms/opentdf", []string{".well-known/openid-configuration"}},
		{"http://localhost:8888/auth/realms/opentdf/", []string{"/.well-known/openid-configuration"}},
		{"https://example.com/a/", []string{"../b", "c/"}},
		{"https://example.com", []string{}},
		{"https://example.com/x?q=1#f", []string{"y"}},
		{"relative", []string{"y"}},
	}
	for _, c := range cases {
		want, wantErr := url.JoinPath(c.base, c.elem...)
		got, gotErr := JoinPath(c.base, c.elem...)
		if (wantErr == nil) != (gotErr == nil) || got != want {
			t.Fatalf("JoinPath(%q, %v): %q %v vs %q %v", c.base, c.elem, got, gotErr, want, wantErr)
		}
	}
}

func TestJoinHostPortMatchesNet(t *testing.T) {
	for _, c := range [][2]string{{"example.com", "443"}, {"::1", "8443"}, {"localhost", "8080"}} {
		if got, want := JoinHostPort(c[0], c[1]), net.JoinHostPort(c[0], c[1]); got != want {
			t.Fatalf("JoinHostPort(%q, %q): %q vs %q", c[0], c[1], got, want)
		}
	}
}

func TestEscapesMatchNetURL(t *testing.T) {
	for _, s := range []string{"a b", "a/b", "x?y=z&w", "ünïcode", "safe-._~", "!*'()", "100%"} {
		if got, want := PathEscape(s), url.PathEscape(s); got != want {
			t.Fatalf("PathEscape(%q): %q vs %q", s, got, want)
		}
		if got, want := QueryEscape(s), url.QueryEscape(s); got != want {
			t.Fatalf("QueryEscape(%q): %q vs %q", s, got, want)
		}
	}
}
