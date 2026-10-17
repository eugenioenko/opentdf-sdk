package sdk

import (
	"context"
	"errors"
	"math/rand"
	"reflect"
	"regexp"
	"sort"
	"strconv"
	"strings"
	"sync"
	"testing"

	gslog "github.com/eugenioenko/goalchemy/std/log/slog"
	"opentdf-local/sdk/src/protocol/policy"
	"opentdf-local/sdk/src/protocol/policy/attributes"
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

// The FQN methods match the reference's regular expressions, including
// Prefix, which the reference derives with a regexp of its own.
func TestAttributeFQNMethodsMatchRegexp(t *testing.T) {
	prefix := regexp.MustCompile(`^(https?://[\w./-]+/attr/\S*)/value/\S*$`)
	for _, u := range []string{"http://e/attr/a/value/b", "https://e.com/x/attr/a/value/b", "http://e/attr/value/value/one", "https://e.com/attr/attr/value/value", "http://e/attr/a%20b/value/%F0%9F%98%81"} {
		v, err := NewAttributeValueFQN(u)
		if err != nil {
			t.Fatal(err)
		}
		if got, want := v.Prefix().String(), prefix.FindStringSubmatch(u)[1]; got != want {
			t.Fatalf("Prefix(%q) = %q, regexp %q", u, got, want)
		}
	}
}

// Ported from the reference's granter_test.go.
const (
	kasAu               = "https://kas.au/"
	kasCa               = "https://kas.ca/"
	kasUk               = "https://kas.uk/"
	kasNz               = "https://kas.nz/"
	kasUs               = "https://kas.us/"
	kasUsHCS            = "https://hcs.kas.us/"
	kasUsSA             = "https://si.kas.us/"
	specifiedKas        = "https://attr.kas.com/"
	evenMoreSpecificKas = "https://value.kas.com/"
	lessSpecificKas     = "https://namespace.kas.com/"
	obligationKas       = "https://obligation.kas.com/"

	// The granter only carries PEMs through; their contents are never parsed.
	mockRSAPublicKey1 = "mock-rsa-public-key-1"
	mockRSAPublicKey2 = "mock-rsa-public-key-2"
	mockRSAPublicKey3 = "mock-rsa-public-key-3"
	mockECPublicKey1  = "mock-ec-public-key-1"
	mockECPublicKey2  = "mock-ec-public-key-2"
	fakePem           = mockRSAPublicKey1
)

func mustNameFQN(u string) AttributeNameFQN {
	a, err := NewAttributeNameFQN(u)
	if err != nil {
		panic(err)
	}
	return a
}

func mustValueFQN(u string) AttributeValueFQN {
	a, err := NewAttributeValueFQN(u)
	if err != nil {
		panic(err)
	}
	return a
}

var (
	CLS = mustNameFQN("https://virtru.com/attr/Classification")
	N2K = mustNameFQN("https://virtru.com/attr/Need%20to%20Know")
	REL = mustNameFQN("https://virtru.com/attr/Releasable%20To")

	clsA  = mustValueFQN("https://virtru.com/attr/Classification/value/Allowed")
	clsS  = mustValueFQN("https://virtru.com/attr/Classification/value/Secret")
	clsTS = mustValueFQN("https://virtru.com/attr/Classification/value/Top%20Secret")

	n2kHCS = mustValueFQN("https://virtru.com/attr/Need%20to%20Know/value/HCS")
	n2kInt = mustValueFQN("https://virtru.com/attr/Need%20to%20Know/value/INT")
	n2kSI  = mustValueFQN("https://virtru.com/attr/Need%20to%20Know/value/SI")

	rel2aus = mustValueFQN("https://virtru.com/attr/Releasable%20To/value/AUS")
	rel2can = mustValueFQN("https://virtru.com/attr/Releasable%20To/value/CAN")
	rel2gbr = mustValueFQN("https://virtru.com/attr/Releasable%20To/value/GBR")
	rel2nzl = mustValueFQN("https://virtru.com/attr/Releasable%20To/value/NZL")
	rel2usa = mustValueFQN("https://virtru.com/attr/Releasable%20To/value/USA")

	// attributes to test specificity of kas grants
	UNSPECKED = mustNameFQN("https://other.com/attr/unspecified")
	SPECKED   = mustNameFQN("https://other.com/attr/specified")
	uns2uns   = mustValueFQN("https://other.com/attr/unspecified/value/unspecked")
	uns2spk   = mustValueFQN("https://other.com/attr/unspecified/value/specked")
	spk2uns   = mustValueFQN("https://other.com/attr/specified/value/unspecked")
	spk2spk   = mustValueFQN("https://other.com/attr/specified/value/specked")

	// with namsepace that has grants
	SPKUNSPECKED = mustNameFQN("https://hasgrants.com/attr/unspecified")
	SPKSPECKED   = mustNameFQN("https://hasgrants.com/attr/specified")
	spk2uns2uns  = mustValueFQN("https://hasgrants.com/attr/unspecified/value/unspecked")
	spk2uns2spk  = mustValueFQN("https://hasgrants.com/attr/unspecified/value/specked")
	spk2spk2uns  = mustValueFQN("https://hasgrants.com/attr/specified/value/unspecked")
	spk2spk2spk  = mustValueFQN("https://hasgrants.com/attr/specified/value/specked")

	// Attributes for testing specific key mapping
	MP  = mustNameFQN("https://virtru.com/attr/mapped")
	mpa = mustValueFQN("https://virtru.com/attr/mapped/value/a")
	mpb = mustValueFQN("https://virtru.com/attr/mapped/value/b")
	mpc = mustValueFQN("https://virtru.com/attr/mapped/value/c")
	mpd = mustValueFQN("https://virtru.com/attr/mapped/value/d")
	mpu = mustValueFQN("https://virtru.com/attr/mapped/value/unspecified")

	OBLIGATIONATTR = mustNameFQN("https://virtru.com/attr/obligation_test")
)

func spongeCase(s string) string {
	re := regexp.MustCompile(`^(https?://[\w./]+/attr/)([^/]*)(/value/)?(\S*)?$`)
	m := re.FindStringSubmatch(s)
	if m == nil {
		panic(ErrInvalid)
	}

	var sb strings.Builder
	sb.WriteString(m[1])
	n := m[2]
	for i := 0; i < len(n); i++ {
		sub := n[i : i+1]
		if i&1 == 1 {
			sb.WriteString(strings.ToUpper(sub))
		} else {
			sb.WriteString(sub)
		}
	}
	if len(m) > 3 {
		sb.WriteString(m[3])
		v := m[4]
		for i := 0; i < len(v); i++ {
			sub := v[i : i+1]
			if i&1 == 1 {
				sb.WriteString(sub)
			} else {
				sb.WriteString(strings.ToUpper(sub))
			}
		}
	}
	return sb.String()
}

func messUpV(t *testing.T, a AttributeValueFQN) AttributeValueFQN {
	n, err := NewAttributeValueFQN(spongeCase(a.String()))
	if err != nil {
		t.Fatal(err)
	}
	return n
}

func mockAttributeFor(fqn AttributeNameFQN) *policy.Attribute {
	nsOne := policy.Namespace{
		Id:   "v",
		Name: "virtru.com",
		Fqn:  "https://virtru.com",
	}
	nsTwo := policy.Namespace{
		Id:   "o",
		Name: "other.com",
		Fqn:  "https://other.com",
	}
	nsThree := policy.Namespace{
		Id:     "h",
		Name:   "hasgrants.com",
		Fqn:    "https://hasgrants.com",
		Grants: []*policy.KeyAccessServer{{Uri: lessSpecificKas}},
	}
	switch fqn.Key {
	case CLS.Key:
		return &policy.Attribute{
			Id:        "CLS",
			Namespace: &nsOne,
			Name:      "Classification",
			Rule:      policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_HIERARCHY,
			Fqn:       fqn.String(),
		}
	case MP.Key:
		g := make([]*policy.KeyAccessServer, 1)
		g[0] = mockGrant(specifiedKas, "r1")
		return &policy.Attribute{
			Id:        "MP",
			Namespace: &nsOne,
			Name:      "Mapped",
			Rule:      policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF,
			Fqn:       fqn.String(),
			Grants:    g,
			KasKeys:   []*policy.SimpleKasKey{mockSimpleKasKey(specifiedKas, "r1")},
		}
	case N2K.Key:
		return &policy.Attribute{
			Id:        "N2K",
			Namespace: &nsOne,
			Name:      "Need to Know",
			Rule:      policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ALL_OF,
			Fqn:       fqn.String(),
		}
	case REL.Key:
		return &policy.Attribute{
			Id:        "REL",
			Namespace: &nsOne,
			Name:      "Releasable To",
			Rule:      policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF,
			Fqn:       fqn.String(),
		}
	case SPECKED.Key:
		g := make([]*policy.KeyAccessServer, 1)
		g[0] = mockGrant(specifiedKas, "r1")
		return &policy.Attribute{
			Id:        "SPK",
			Namespace: &nsTwo,
			Name:      "specified",
			Rule:      policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF,
			Fqn:       fqn.String(),
			Grants:    g,
		}
	case UNSPECKED.Key:
		return &policy.Attribute{
			Id:        "UNS",
			Namespace: &nsTwo,
			Name:      "unspecified",
			Rule:      policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF,
			Fqn:       fqn.String(),
		}
	case SPKSPECKED.Key:
		g := make([]*policy.KeyAccessServer, 1)
		g[0] = mockGrant(specifiedKas, "r1")
		return &policy.Attribute{
			Id:        "SPK",
			Namespace: &nsThree,
			Name:      "specified",
			Rule:      policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF,
			Fqn:       fqn.String(),
			Grants:    g,
		}
	case SPKUNSPECKED.Key:
		return &policy.Attribute{
			Id:        "UNS",
			Namespace: &nsThree,
			Name:      "unspecified",
			Rule:      policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF,
			Fqn:       fqn.String(),
		}
	case OBLIGATIONATTR.Key:
		return &policy.Attribute{
			Id:        "OBL",
			Namespace: &nsOne,
			Name:      "obligation",
			Rule:      policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF,
			Fqn:       fqn.String(),
		}
	}
	return nil
}

func mockGrant(kas, kid string) *policy.KeyAccessServer {
	if kas == "" {
		panic("invalid kas URI")
	}
	if kid == "" {
		return mockGrant(kas, "r0")
	}

	var k policy.SimpleKasPublicKey
	switch kid {
	case "r0":
		k = policy.SimpleKasPublicKey{Algorithm: policy.Algorithm_ALGORITHM_RSA_2048, Kid: kid, Pem: fakePem}
	case "r1":
		k = policy.SimpleKasPublicKey{Algorithm: policy.Algorithm_ALGORITHM_RSA_2048, Kid: kid, Pem: mockRSAPublicKey1}
	case "r2":
		k = policy.SimpleKasPublicKey{Algorithm: policy.Algorithm_ALGORITHM_RSA_4096, Kid: kid, Pem: mockRSAPublicKey2}
	case "r3":
		k = policy.SimpleKasPublicKey{Algorithm: policy.Algorithm_ALGORITHM_RSA_2048, Kid: kid, Pem: mockRSAPublicKey3}
	case "e1":
		k = policy.SimpleKasPublicKey{Algorithm: policy.Algorithm_ALGORITHM_EC_P256, Kid: kid, Pem: mockECPublicKey1}
	case "e2":
		k = policy.SimpleKasPublicKey{Algorithm: policy.Algorithm_ALGORITHM_EC_P256, Kid: kid, Pem: mockECPublicKey2}
	default:
		panic("invalid kas kid: " + kid)
	}

	return &policy.KeyAccessServer{
		Uri:     kas,
		Id:      kas,
		KasKeys: []*policy.SimpleKasKey{{KasUri: kas, PublicKey: &k}},
	}
}

type fakeKeyInfoFetcher struct{}

func (f *fakeKeyInfoFetcher) getPublicKey(_ context.Context, kasurl, _, _ string) (*KASInfo, error) {
	if kasurl == "" {
		return nil, errors.New("kas URI is empty")
	}

	k := KASInfo{URL: kasurl}

	switch kasurl {
	case kasAu, kasCa, kasUk, kasNz, kasUs:
		k.PublicKey = mockRSAPublicKey1
		k.KID = "r1"
		k.Algorithm = "rsa:2048"
	case kasUsHCS, kasUsSA:
		k.PublicKey = mockRSAPublicKey2
		k.KID = "r2"
		k.Algorithm = "rsa:4096"
	case lessSpecificKas:
		k.PublicKey = mockRSAPublicKey3
		k.KID = "r3"
		k.Algorithm = "rsa:2048"
	default:
		return nil, errors.New("unexpected kas URL: " + kasurl)
	}

	return &k, nil
}

func mockSimpleKasKey(kas, kid string) *policy.SimpleKasKey {
	if kas == "" {
		panic("invalid kas URI")
	}
	if kid == "" {
		panic("invalid kas kid")
	}
	var alg policy.Algorithm
	var pem string
	switch kid {
	case "r0":
		alg, pem = policy.Algorithm_ALGORITHM_RSA_2048, fakePem
	case "r1":
		alg, pem = policy.Algorithm_ALGORITHM_RSA_2048, mockRSAPublicKey1
	case "r2":
		alg, pem = policy.Algorithm_ALGORITHM_RSA_4096, mockRSAPublicKey2
	case "r3":
		alg, pem = policy.Algorithm_ALGORITHM_RSA_2048, mockRSAPublicKey3
	case "e1":
		alg, pem = policy.Algorithm_ALGORITHM_EC_P256, mockECPublicKey1
	default:
		panic("invalid kas kid: " + kid)
	}
	return &policy.SimpleKasKey{
		KasUri:    kas,
		PublicKey: &policy.SimpleKasPublicKey{Algorithm: alg, Kid: kid, Pem: pem},
	}
}

func mockValueFor(fqn AttributeValueFQN) *policy.Value {
	an := fqn.Prefix()
	a := mockAttributeFor(an)
	v := fqn.Value()
	p := policy.Value{
		Id:        a.GetId() + ":" + v,
		Attribute: a,
		Value:     v,
		Fqn:       fqn.String(),
	}

	switch an.Key {
	case N2K.Key:
		switch strings.ToUpper(fqn.Value()) {
		case "INT":
			p.Grants = []*policy.KeyAccessServer{mockGrant(kasUk, "r1")}
		case "HCS":
			p.Grants = []*policy.KeyAccessServer{mockGrant(kasUsHCS, "r2")}
		case "SI":
			p.Grants = []*policy.KeyAccessServer{mockGrant(kasUsSA, "r2")}
		}
	case REL.Key:
		switch strings.ToUpper(fqn.Value()) {
		case "FVEY":
			p.Grants = []*policy.KeyAccessServer{mockGrant(kasAu, "r1"), mockGrant(kasCa, "r1"), mockGrant(kasUk, "r1"), mockGrant(kasNz, "r1"), mockGrant(kasUs, "r1")}
		case "AUS":
			p.Grants = []*policy.KeyAccessServer{mockGrant(kasAu, "r1")}
		case "CAN":
			p.Grants = []*policy.KeyAccessServer{mockGrant(kasCa, "r1")}
		case "GBR":
			p.Grants = []*policy.KeyAccessServer{mockGrant(kasUk, "r1")}
		case "NZL":
			p.Grants = []*policy.KeyAccessServer{mockGrant(kasNz, "r1")}
		case "USA":
			p.Grants = []*policy.KeyAccessServer{mockGrant(kasUs, "r1")}
		}
	case CLS.Key:
		// defaults only
	case MP.Key:
		switch strings.ToLower(fqn.Value()) {
		case "a":
			p.KasKeys = []*policy.SimpleKasKey{mockSimpleKasKey(evenMoreSpecificKas, "r2")}
		case "b":
			p.KasKeys = []*policy.SimpleKasKey{mockSimpleKasKey(evenMoreSpecificKas, "e1")}
		case "c":
			p.KasKeys = []*policy.SimpleKasKey{mockSimpleKasKey(evenMoreSpecificKas, "r0")}
		case "d":
			p.KasKeys = []*policy.SimpleKasKey{mockSimpleKasKey(evenMoreSpecificKas, "r3")}
		}
	case SPECKED.Key, UNSPECKED.Key, SPKSPECKED.Key, SPKUNSPECKED.Key:
		if strings.ToLower(fqn.Value()) == "specked" {
			p.Grants = []*policy.KeyAccessServer{mockGrant(evenMoreSpecificKas, "r1")}
		}
	case OBLIGATIONATTR.Key:
		switch strings.ToLower(fqn.Value()) {
		case "value1", "value2":
			p.KasKeys = []*policy.SimpleKasKey{mockSimpleKasKey(obligationKas, "r3")}
		case "value3":
			p.KasKeys = []*policy.SimpleKasKey{mockSimpleKasKey("https://d.kas/", "e1")}
		}
	}
	return &p
}

func TestAttributeFromURL(t *testing.T) {
	for _, tc := range []struct {
		n, u       string
		auth, name string
	}{
		{"letter", "http://e/attr/a", "http://e", "a"},
		{"number", "http://e/attr/1", "http://e", "1"},
		{"emoji", "http://e/attr/%F0%9F%98%81", "http://e", "😁"},
		{"dash", "http://a-b.com/attr/b-c", "http://a-b.com", "b-c"},
		{"nested", "https://e.com/x/attr/y/attr/a", "https://e.com/x/attr/y", "a"},
	} {
		t.Run(tc.n, func(t *testing.T) {
			a, err := NewAttributeNameFQN(tc.u)
			if err != nil || a.String() != tc.u {
				t.Fatal(a, err)
			}
			if a.Authority() != tc.auth || a.Name() != tc.name {
				t.Fatalf("%q %q", a.Authority(), a.Name())
			}
		})
	}
}

func TestAttributeFromMalformedURL(t *testing.T) {
	for _, tc := range []struct {
		n, u string
	}{
		{"no name", "http://e/attr"},
		{"invalid prefix 1", "hxxp://e/attr/a"},
		{"invalid prefix 2", "e/attr/a"},
		{"invalid prefix 3", "file://e/attr/a"},
		{"invalid prefix 4", "http:///attr/a"},
		{"bad encoding", "https://a/attr/%😁"},
		{"with value", "http://e/attr/a/value/b"},
		{"empty name", "http://e/attr/"},
		{"space", "http://e/attr/a b"},
	} {
		t.Run(tc.n, func(t *testing.T) {
			a, err := NewAttributeNameFQN(tc.u)
			if !errors.Is(err, ErrInvalid) || a.String() != "" {
				t.Fatal(a, err)
			}
		})
	}
}

func TestAttributeValueFromURL(t *testing.T) {
	for _, tc := range []struct {
		n, u              string
		auth, name, value string
	}{
		{"number", "http://e/attr/a/value/1", "http://e", "a", "1"},
		{"space", "http://e/attr/a/value/%20", "http://e", "a", " "},
		{"emoji", "http://e/attr/a/value/%F0%9F%98%81", "http://e", "a", "😁"},
		{"numberdef", "http://e/attr/1/value/one", "http://e", "1", "one"},
		{"valuevalue", "http://e/attr/value/value/one", "http://e", "value", "one"},
		{"dash", "http://a-b.com/attr/b-c/value/c-d", "http://a-b.com", "b-c", "c-d"},
	} {
		t.Run(tc.n, func(t *testing.T) {
			a, err := NewAttributeValueFQN(tc.u)
			if err != nil || a.String() != tc.u {
				t.Fatal(a, err)
			}
			if a.Authority() != tc.auth || a.Name() != tc.name || a.Value() != tc.value {
				t.Fatalf("%q %q %q", a.Authority(), a.Name(), a.Value())
			}
		})
	}
}

func TestAttributeValueFromMalformedURL(t *testing.T) {
	for _, tc := range []struct {
		n, u, want string
	}{
		{"no name", "http://e/attr/value/1", "invalid type: attribute regex fail for [http://e/attr/value/1]"},
		{"no value", "http://e/attr/who/value", "invalid type: attribute regex fail for [http://e/attr/who/value]"},
		{"invalid prefix 1", "hxxp://e/attr/a/value/1", "invalid type: attribute regex fail for [hxxp://e/attr/a/value/1]"},
		{"invalid prefix 2", "e/attr/a/a/value/1", "invalid type: attribute regex fail for [e/attr/a/a/value/1]"},
		{"bad encoding", "https://a/attr/emoji/value/%😁", "invalid type: error in attribute value [%😁]"},
		{"empty name", "http://e/attr//value/0", "invalid type: empty url path parts are not allowed"},
		{"slash name", "http://e/attr///value/0", "invalid type: slash not allowed in name or values"},
		{"slash in name", "http://0/attr/0/0/value/0", "invalid type: slash not allowed in name or values"},
		{"empty value", "http://e/attr/a/value/", "invalid type: empty url path parts are not allowed"},
	} {
		t.Run(tc.n, func(t *testing.T) {
			a, err := NewAttributeValueFQN(tc.u)
			if !errors.Is(err, ErrInvalid) || a.String() != "" || err.Error() != tc.want {
				t.Fatal(a, err)
			}
		})
	}
}

func TestAttributeNameSelect(t *testing.T) {
	v := REL.Select("New Zealand")
	if v.String() != "https://virtru.com/attr/Releasable%20To/value/New%20Zealand" || v.Value() != "New Zealand" || v.Prefix().Key != REL.Key {
		t.Fatal(v)
	}
}

func valuesToPolicy(p ...AttributeValueFQN) []*policy.Value {
	v := make([]*policy.Value, len(p))
	for i, ai := range p {
		v[i] = mockValueFor(ai)
	}
	return v
}

func policyToStringKeys(policy []AttributeValueFQN) []string {
	s := make([]string, len(policy))
	for i := 0; i < len(s); i++ {
		s[i] = policy[i].Key
	}
	return s
}

func testLogger() *gslog.Logger { return gslog.Default() }

func testKeyCache() *kasKeyCache { return newKasKeyCache(systemClock{}) }

func sortedCopy(s []string) []string {
	c := append([]string(nil), s...)
	sort.Strings(c)
	return c
}

func TestConfigurationServicePutGet(t *testing.T) {
	for _, tc := range []struct {
		n      string
		policy []AttributeValueFQN
		size   int
		kases  []string
	}{
		{"default", []AttributeValueFQN{clsA}, 1, []string{}},
		{"one-country", []AttributeValueFQN{rel2gbr}, 1, []string{kasUk}},
		{"two-country", []AttributeValueFQN{rel2gbr, rel2nzl}, 2, []string{kasUk, kasNz}},
		{"with-default", []AttributeValueFQN{clsA, rel2gbr}, 2, []string{kasUk}},
		{"need-to-know", []AttributeValueFQN{clsTS, rel2usa, n2kSI}, 3, []string{kasUs, kasUsSA}},
	} {
		t.Run(tc.n, func(t *testing.T) {
			v := valuesToPolicy(tc.policy...)
			grants, err := newGranterFromAttributes(testLogger(), testKeyCache(), v...)
			if err != nil {
				t.Fatal(err)
			}
			if len(grants.grantTable) != tc.size {
				t.Fatalf("grantTable %d", len(grants.grantTable))
			}
			keys := map[string]bool{}
			for _, k := range policyToStringKeys(tc.policy) {
				keys[k] = true
			}
			actualKases := map[string]bool{}
			for k, g := range grants.grantTable {
				if !keys[k] || g == nil {
					t.Fatalf("grant %q", k)
				}
				for _, kas := range g.kases {
					actualKases[kas] = true
				}
			}
			got := []string{}
			for k := range actualKases {
				got = append(got, k)
			}
			if !reflect.DeepEqual(sortedCopy(got), sortedCopy(tc.kases)) {
				t.Fatalf("kases %v, want %v", got, tc.kases)
			}
		})
	}
}

func counter() func() string {
	i := 0
	return func() string {
		i++
		return strconv.Itoa(i)
	}
}

func TestReasonerConstructAttributeBoolean(t *testing.T) {
	for _, tc := range []struct {
		n                   string
		policy              []AttributeValueFQN
		defaults            []string
		ats, keyed, reduced string
		plan                []keySplitStep
		tpl                 []kaoTpl
	}{
		{
			"one actual with default",
			[]AttributeValueFQN{clsS, rel2can},
			[]string{kasUs},
			"https://virtru.com/attr/Classification/value/Secret&https://virtru.com/attr/Releasable%20To/value/CAN",
			"[DEFAULT]&(https://kas.ca/)",
			"(https://kas.ca/)",
			[]keySplitStep{{kasCa, ""}},
			[]kaoTpl{{kasCa, "", "r1", mockRSAPublicKey1, RSA2048Key}},
		},
		{
			"one defaulted attr",
			[]AttributeValueFQN{clsS},
			[]string{kasUs},
			"https://virtru.com/attr/Classification/value/Secret",
			"[DEFAULT]",
			"",
			[]keySplitStep{{kasUs, ""}},
			[]kaoTpl{},
		},
		{
			"empty policy",
			[]AttributeValueFQN{},
			[]string{kasUs},
			"∅",
			"",
			"",
			[]keySplitStep{{kasUs, ""}},
			[]kaoTpl{},
		},
		{
			"old school splits",
			[]AttributeValueFQN{},
			[]string{kasAu, kasCa, kasUs},
			"∅",
			"",
			"",
			[]keySplitStep{{kasAu, "1"}, {kasCa, "2"}, {kasUs, "3"}},
			[]kaoTpl{},
		},
		{
			"simple with all three ops",
			[]AttributeValueFQN{clsS, rel2gbr, n2kInt},
			[]string{kasUs},
			"https://virtru.com/attr/Classification/value/Secret&https://virtru.com/attr/Releasable%20To/value/GBR&https://virtru.com/attr/Need%20to%20Know/value/INT",
			"[DEFAULT]&(https://kas.uk/)&(https://kas.uk/)",
			"(https://kas.uk/)",
			[]keySplitStep{{kasUk, ""}},
			[]kaoTpl{{kasUk, "", "r1", mockRSAPublicKey1, RSA2048Key}},
		},
		{
			"simple with namespace",
			[]AttributeValueFQN{clsS, spk2uns2uns},
			[]string{kasUs},
			"https://virtru.com/attr/Classification/value/Secret&https://hasgrants.com/attr/unspecified/value/unspecked",
			"[DEFAULT]&(https://namespace.kas.com/)",
			"(https://namespace.kas.com/)",
			[]keySplitStep{{lessSpecificKas, ""}},
			[]kaoTpl{{lessSpecificKas, "", "r3", mockRSAPublicKey3, RSA2048Key}},
		},
		{
			"compartments",
			[]AttributeValueFQN{clsS, rel2gbr, rel2usa, n2kHCS, n2kSI},
			[]string{kasUs},
			"https://virtru.com/attr/Classification/value/Secret&https://virtru.com/attr/Releasable%20To/value/{GBR,USA}&https://virtru.com/attr/Need%20to%20Know/value/{HCS,SI}",
			"[DEFAULT]&(https://kas.uk/⋁https://kas.us/)&(https://hcs.kas.us/⋀https://si.kas.us/)",
			"(https://kas.uk/⋁https://kas.us/)&(https://hcs.kas.us/)&(https://si.kas.us/)",
			[]keySplitStep{{kasUk, "1"}, {kasUs, "1"}, {kasUsHCS, "2"}, {kasUsSA, "3"}},
			[]kaoTpl{
				{kasUk, "1", "r1", mockRSAPublicKey1, RSA2048Key},
				{kasUs, "1", "r1", mockRSAPublicKey1, RSA2048Key},
				{kasUsHCS, "2", "r2", mockRSAPublicKey2, RSA4096Key},
				{kasUsSA, "3", "r2", mockRSAPublicKey2, RSA4096Key},
			},
		},
		{
			"compartments - case insensitive",
			[]AttributeValueFQN{messUpV(t, clsS), messUpV(t, rel2gbr), messUpV(t, rel2usa), messUpV(t, n2kHCS), messUpV(t, n2kSI)},
			[]string{kasUs},
			"https://virtru.com/attr/Classification/value/Secret&https://virtru.com/attr/Releasable%20To/value/{GBR,USA}&https://virtru.com/attr/Need%20to%20Know/value/{HCS,SI}",
			"[DEFAULT]&(https://kas.uk/⋁https://kas.us/)&(https://hcs.kas.us/⋀https://si.kas.us/)",
			"(https://kas.uk/⋁https://kas.us/)&(https://hcs.kas.us/)&(https://si.kas.us/)",
			[]keySplitStep{{kasUk, "1"}, {kasUs, "1"}, {kasUsHCS, "2"}, {kasUsSA, "3"}},
			[]kaoTpl{
				{kasUk, "1", "r1", mockRSAPublicKey1, RSA2048Key},
				{kasUs, "1", "r1", mockRSAPublicKey1, RSA2048Key},
				{kasUsHCS, "2", "r2", mockRSAPublicKey2, RSA4096Key},
				{kasUsSA, "3", "r2", mockRSAPublicKey2, RSA4096Key},
			},
		},
		{
			"mappings at attr value",
			[]AttributeValueFQN{mpa, mpb},
			[]string{emptyTerm},
			"https://virtru.com/attr/mapped/value/{a,b}",
			"(https://value.kas.com/⋁https://value.kas.com/)",
			"(https://value.kas.com/)",
			[]keySplitStep{{evenMoreSpecificKas, ""}},
			[]kaoTpl{
				{evenMoreSpecificKas, "1", "e1", mockECPublicKey1, EC256Key},
				{evenMoreSpecificKas, "1", "r2", mockRSAPublicKey2, RSA4096Key},
			},
		},
		{
			"mappings at attr definition",
			[]AttributeValueFQN{mpu},
			[]string{emptyTerm},
			"https://virtru.com/attr/mapped/value/unspecified",
			"(https://attr.kas.com/)",
			"(https://attr.kas.com/)",
			[]keySplitStep{{specifiedKas, ""}},
			[]kaoTpl{{specifiedKas, "", "r1", mockRSAPublicKey1, RSA2048Key}},
		},
		{
			"mappings all",
			[]AttributeValueFQN{mpa, mpb, mpu},
			[]string{emptyTerm},
			"https://virtru.com/attr/mapped/value/{a,b,unspecified}",
			"(https://value.kas.com/⋁https://value.kas.com/⋁https://attr.kas.com/)",
			"(https://attr.kas.com/⋁https://value.kas.com/)",
			[]keySplitStep{{specifiedKas, "1"}, {evenMoreSpecificKas, "1"}},
			[]kaoTpl{
				{specifiedKas, "1", "r1", mockRSAPublicKey1, RSA2048Key},
				{evenMoreSpecificKas, "1", "e1", mockECPublicKey1, EC256Key},
				{evenMoreSpecificKas, "1", "r2", mockRSAPublicKey2, RSA4096Key},
			},
		},
	} {
		t.Run(tc.n, func(t *testing.T) {
			reasoner, err := newGranterFromAttributes(testLogger(), testKeyCache(), valuesToPolicy(tc.policy...)...)
			if err != nil {
				t.Fatal(err)
			}
			reasoner.keyInfoFetcher = &fakeKeyInfoFetcher{}

			actualAB := reasoner.constructAttributeBoolean()
			if got := strings.ToLower(actualAB.String()); got != strings.ToLower(tc.ats) {
				t.Fatalf("boolean %q, want %q", got, tc.ats)
			}

			actualKeyed, err := reasoner.insertKeysForAttribute(*actualAB)
			if err != nil {
				t.Fatal(err)
			}
			if got := actualKeyed.String(); got != tc.keyed {
				t.Fatalf("keyed %q, want %q", got, tc.keyed)
			}

			if got := actualKeyed.reduce().String(); got != tc.reduced {
				t.Fatalf("reduced %q, want %q", got, tc.reduced)
			}

			plan, err := reasoner.plan(tc.defaults, counter())
			if err != nil {
				t.Fatal(err)
			}
			if !reflect.DeepEqual(plan, tc.plan) {
				t.Fatalf("plan %v, want %v", plan, tc.plan)
			}

			tpl, err := reasoner.resolveTemplate(t.Context(), "", counter())
			if err != nil {
				t.Fatal(err)
			}
			if !reflect.DeepEqual(tpl, tc.tpl) {
				t.Fatalf("template %v, want %v", tpl, tc.tpl)
			}
		})
	}
}

type mockAttributesClient struct{}

func (*mockAttributesClient) ListAttributes(context.Context, *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
	return &attributes.ListAttributesResponse{}, nil
}

func (*mockAttributesClient) GetAttribute(context.Context, *attributes.GetAttributeRequest) (*attributes.GetAttributeResponse, error) {
	return nil, errors.New("unexpected GetAttribute")
}

func (*mockAttributesClient) GetAttributeValuesByFqns(_ context.Context, req *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
	av := make(map[string]*attributes.GetAttributeValuesByFqnsResponse_AttributeAndValue)
	for _, v := range req.GetFqns() {
		vfqn, err := NewAttributeValueFQN(v)
		if err != nil {
			return nil, err
		}
		val := mockValueFor(vfqn)
		av[v] = &attributes.GetAttributeValuesByFqnsResponse_AttributeAndValue{
			Attribute: val.GetAttribute(),
			Value:     val,
		}
	}
	return &attributes.GetAttributeValuesByFqnsResponse{FqnAttributeValues: av}, nil
}

// effectiveMockKasKeys mirrors the server-side resolveEffectiveKasKeys: the
// most-specific mapped keys with value > definition > namespace precedence.
func effectiveMockKasKeys(v *policy.Value, a *policy.Attribute) []*policy.SimpleKasKey {
	if k := v.GetKasKeys(); len(k) > 0 {
		return k
	}
	if k := a.GetKasKeys(); len(k) > 0 {
		return k
	}
	if k := a.GetNamespace().GetKasKeys(); len(k) > 0 {
		return k
	}
	return nil
}

func (*mockAttributesClient) GetKeyMappingsByFqns(_ context.Context, req *attributes.GetKeyMappingsByFqnsRequest) (*attributes.GetKeyMappingsByFqnsResponse, error) {
	out := make(map[string]*attributes.GetKeyMappingsByFqnsResponse_AttributeKeyMapping)
	for _, v := range req.GetFqns() {
		vfqn, err := NewAttributeValueFQN(v)
		if err != nil {
			return nil, err
		}
		val := mockValueFor(vfqn)
		attr := val.GetAttribute()
		out[v] = &attributes.GetKeyMappingsByFqnsResponse_AttributeKeyMapping{
			Rule: attr.GetRule(),
			Keys: effectiveMockKasKeys(val, attr),
		}
	}
	return &attributes.GetKeyMappingsByFqnsResponse{FqnKeyMappings: out}, nil
}

// unimplementedKeyMappingsClient models an older platform that does not expose
// GetKeyMappingsByFqns; the granter must fall back to GetAttributeValuesByFqns.
type unimplementedKeyMappingsClient struct {
	mockAttributesClient
}

func (*unimplementedKeyMappingsClient) GetKeyMappingsByFqns(context.Context, *attributes.GetKeyMappingsByFqnsRequest) (*attributes.GetKeyMappingsByFqnsResponse, error) {
	return nil, &Error{Kind: KindTransport, Message: "unimplemented: not implemented", ConnectCode: "unimplemented"}
}

// emptyKeyMappingsClient models a server that succeeds but omits the requested
// FQNs from the response map; the granter must fall back per value rather than
// silently drop them.
type emptyKeyMappingsClient struct {
	mockAttributesClient
}

func (*emptyKeyMappingsClient) GetKeyMappingsByFqns(context.Context, *attributes.GetKeyMappingsByFqnsRequest) (*attributes.GetKeyMappingsByFqnsResponse, error) {
	return &attributes.GetKeyMappingsByFqnsResponse{}, nil
}

// failingKeyMappingsClient fails GetKeyMappingsByFqns with any other code,
// which the granter must not swallow.
type failingKeyMappingsClient struct {
	mockAttributesClient
}

func (*failingKeyMappingsClient) GetKeyMappingsByFqns(context.Context, *attributes.GetKeyMappingsByFqnsRequest) (*attributes.GetKeyMappingsByFqnsResponse, error) {
	return nil, &Error{Kind: KindTransport, Message: "permission_denied: no", ConnectCode: "permission_denied"}
}

func sameSteps(t *testing.T, got, want []keySplitStep) {
	t.Helper()
	key := func(s []keySplitStep) []string {
		out := make([]string, len(s))
		for i, step := range s {
			out[i] = step.KAS + "|" + step.SplitID
		}
		sort.Strings(out)
		return out
	}
	if !reflect.DeepEqual(key(got), key(want)) {
		t.Fatalf("plan %v, want %v", got, want)
	}
}

// TestReasonerKeyMappingFallback verifies that grant-only values and older
// platforms (no GetKeyMappingsByFqns) resolve to the same plan as the key-mapping
// path, via the GetAttributeValuesByFqns fallback.
func TestReasonerKeyMappingFallback(t *testing.T) {
	policy := []AttributeValueFQN{spk2spk}
	defaults := []string{kasUs}
	want := []keySplitStep{{evenMoreSpecificKas, ""}}
	noSplit := func() string { return "" }

	for _, tc := range []struct {
		name string
		as   attributesService
	}{
		{"per-value fallback (empty key set)", &mockAttributesClient{}},
		{"full fallback (RPC unimplemented)", &unimplementedKeyMappingsClient{}},
		{"per-value fallback (FQN omitted from response)", &emptyKeyMappingsClient{}},
	} {
		t.Run(tc.name, func(t *testing.T) {
			reasoner, err := newGranterFromService(t.Context(), testLogger(), testKeyCache(), tc.as, policy...)
			if err != nil {
				t.Fatal(err)
			}
			plan, err := reasoner.plan(defaults, noSplit)
			if err != nil {
				t.Fatal(err)
			}
			sameSteps(t, plan, want)
		})
	}
	if _, err := newGranterFromService(t.Context(), testLogger(), testKeyCache(), &failingKeyMappingsClient{}, policy...); connectCodeOf(err) != "permission_denied" {
		t.Fatal(err)
	}
}

// A single request that mixes mapped-key values (MP: mapped kas_keys) with
// grant-configured values (REL: legacy grants) must resolve every attribute and
// combine them into one plan.
func TestReasonerMixedMappingsAndGrants(t *testing.T) {
	policy := []AttributeValueFQN{mpa, mpb, rel2aus, rel2can}
	reasoner, err := newGranterFromService(t.Context(), testLogger(), testKeyCache(), &mockAttributesClient{}, policy...)
	if err != nil {
		t.Fatal(err)
	}
	plan, err := reasoner.plan([]string{kasUs}, counter())
	if err != nil {
		t.Fatal(err)
	}
	sameSteps(t, plan, []keySplitStep{
		{evenMoreSpecificKas, "1"},
		{kasAu, "2"},
		{kasCa, "2"},
	})
}

// Tests titles are written in the form [{attr}.{value}] => [{resulting kas boolean exp}]
func TestReasonerSpecificity(t *testing.T) {
	for _, tc := range []struct {
		n      string
		policy []AttributeValueFQN
		plan   []keySplitStep
	}{
		{"nogrant.nogrant => default", []AttributeValueFQN{uns2uns}, []keySplitStep{{kasUs, ""}}},
		{"nogrant.grant => valueSpecificKas", []AttributeValueFQN{uns2spk}, []keySplitStep{{evenMoreSpecificKas, ""}}},
		{"grant.noGrant => attrSpecificKas", []AttributeValueFQN{spk2uns}, []keySplitStep{{specifiedKas, ""}}},
		{"grant.grant => valueSpecificKas", []AttributeValueFQN{spk2spk}, []keySplitStep{{evenMoreSpecificKas, ""}}},
		{"grant.grant, grant.nogrant => valueSpecificKas || attrSpecificKas", []AttributeValueFQN{spk2spk, spk2uns}, []keySplitStep{{evenMoreSpecificKas, "1"}, {specifiedKas, "1"}}},
		{"grant.nogrant & grant.grant => valueSpecificKas || attrSpecificKas", []AttributeValueFQN{spk2spk, spk2uns}, []keySplitStep{{specifiedKas, "1"}, {evenMoreSpecificKas, "1"}}},
		{"nogrant.grant & nogrant.nogrant => valueSpecificKas", []AttributeValueFQN{uns2spk, uns2uns}, []keySplitStep{{evenMoreSpecificKas, ""}}},
		{"nogrant.nogrant & nogrant.grant => valueSpecificKas", []AttributeValueFQN{uns2spk, uns2uns}, []keySplitStep{{evenMoreSpecificKas, ""}}},
		{"nogrant.nogrant & grant.grant => valueSpecificKas", []AttributeValueFQN{uns2spk, uns2uns}, []keySplitStep{{evenMoreSpecificKas, ""}}},
	} {
		t.Run(tc.n, func(t *testing.T) {
			reasoner, err := newGranterFromService(t.Context(), testLogger(), testKeyCache(), &mockAttributesClient{}, tc.policy...)
			if err != nil {
				t.Fatal(err)
			}
			plan, err := reasoner.plan([]string{kasUs}, counter())
			if err != nil {
				t.Fatal(err)
			}
			sameSteps(t, plan, tc.plan)
		})
	}
}

// Tests titles are written in the form [{ns}.{attr}.{value}] => [{resulting kas boolean exp}]
func TestReasonerSpecificityWithNamespaces(t *testing.T) {
	for _, tc := range []struct {
		n      string
		policy []AttributeValueFQN
		plan   []keySplitStep
	}{
		{"nogrant.nogrant.nogrant => default", []AttributeValueFQN{uns2uns}, []keySplitStep{{kasUs, ""}}},
		{"grant.nogrant.nogrant => nsSpecificKas", []AttributeValueFQN{spk2uns2uns}, []keySplitStep{{lessSpecificKas, ""}}},
		{"grant.nogrant.grant => valueSpecificKas", []AttributeValueFQN{spk2uns2spk}, []keySplitStep{{evenMoreSpecificKas, ""}}},
		{"grant.grant.nogrant => attrSpecificKas", []AttributeValueFQN{spk2spk2uns}, []keySplitStep{{specifiedKas, ""}}},
		{"grant.grant.grant => valueSpecificKas", []AttributeValueFQN{spk2spk}, []keySplitStep{{evenMoreSpecificKas, ""}}},
		{"grant.grant.grant & grant.grant.nogrant => valueSpecificKas || attrSpecificKas", []AttributeValueFQN{spk2spk2spk, spk2spk2uns}, []keySplitStep{{evenMoreSpecificKas, "1"}, {specifiedKas, "1"}}},
		{"grant.grant.nogrant & grant.grant.grant => valueSpecificKas || attrSpecificKas", []AttributeValueFQN{spk2spk2uns, spk2spk2spk}, []keySplitStep{{evenMoreSpecificKas, "1"}, {specifiedKas, "1"}}},
		{"grant.grant.nogrant & grant.nogrant.nogrant => attrSpecificKas && nsSpecificKas", []AttributeValueFQN{spk2spk2uns, spk2uns2uns}, []keySplitStep{{specifiedKas, "1"}, {lessSpecificKas, "2"}}},
		{"grant.nogrant.nogrant & grant.grant.nogrant => attrSpecificKas && nsSpecificKas", []AttributeValueFQN{spk2uns2uns, spk2spk2uns}, []keySplitStep{{lessSpecificKas, "1"}, {specifiedKas, "2"}}},
		{"grant.nogrant.grant & grant.nogrant.nogrant => valueSpecificKas || nsSpecificKas", []AttributeValueFQN{spk2uns2spk, spk2uns2uns}, []keySplitStep{{evenMoreSpecificKas, "1"}, {lessSpecificKas, "1"}}},
		{"grant.nogrant.nogrant & grant.nogrant.grant => valueSpecificKas || nsSpecificKas", []AttributeValueFQN{spk2uns2uns, spk2uns2spk}, []keySplitStep{{evenMoreSpecificKas, "1"}, {lessSpecificKas, "1"}}},
		{"grant.nogrant.nogrant & nogrant.nogrant.nogrant => nsSpecificKas", []AttributeValueFQN{spk2uns2uns, uns2uns}, []keySplitStep{{lessSpecificKas, ""}}},
		{"nogrant.nogrant.nogrant & grant.nogrant.nogrant => nsSpecificKas", []AttributeValueFQN{uns2uns, spk2uns2uns}, []keySplitStep{{lessSpecificKas, ""}}},
		{"grant.nogrant.grant & nogrant.nogrant.nogrant => valueSpecificKas", []AttributeValueFQN{spk2uns2spk, uns2uns}, []keySplitStep{{evenMoreSpecificKas, ""}}},
		{"grant.grant.grant & nogrant.nogrant.nogrant => valueSpecificKas", []AttributeValueFQN{spk2spk2spk, uns2uns}, []keySplitStep{{evenMoreSpecificKas, ""}}},
	} {
		t.Run(tc.n, func(t *testing.T) {
			reasoner, err := newGranterFromService(t.Context(), testLogger(), testKeyCache(), &mockAttributesClient{}, tc.policy...)
			if err != nil {
				t.Fatal(err)
			}
			plan, err := reasoner.plan([]string{kasUs}, counter())
			if err != nil {
				t.Fatal(err)
			}
			sameSteps(t, plan, tc.plan)
		})
	}
}

// Every CreateTDF with attributes builds a granter against the SDK's shared
// kasKeyCache, and storeKeysToCache writes to it once per mapped key -- on
// every call, not just the first.
func TestGranterConcurrentBuildsShareKeyCache(t *testing.T) {
	const (
		workers    = 8
		iterations = 25
	)

	shared := testKeyCache()
	as := &mockAttributesClient{}
	fqns := []AttributeValueFQN{mpa, mpb, mpc, mpd}

	if _, err := newGranterFromService(t.Context(), testLogger(), shared, as, fqns...); err != nil {
		t.Fatal(err)
	}
	if len(shared.c) == 0 {
		t.Fatal("granter build should populate the shared key cache")
	}

	var wg sync.WaitGroup
	for range workers {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for range iterations {
				if _, err := newGranterFromService(t.Context(), testLogger(), shared, as, fqns...); err != nil {
					t.Error(err)
					return
				}
			}
		}()
	}
	wg.Wait()
}
