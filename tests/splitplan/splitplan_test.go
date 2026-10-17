// Package splitplan compares the shared source's key split planning with the
// reference SDK: both encrypt the same policies against one fake platform,
// and their manifests must name the same KAS, kid, wrapping and split
// grouping for every key access object, in the same order.
package splitplan

import (
	"bytes"
	"context"
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/rand"
	"crypto/rsa"
	"crypto/x509"
	"encoding/pem"
	"fmt"
	"io"
	"log/slog"
	mrand "math/rand"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"testing"

	"connectrpc.com/connect"
	"github.com/opentdf/platform/protocol/go/policy"
	"github.com/opentdf/platform/protocol/go/policy/attributes"
	"github.com/opentdf/platform/protocol/go/policy/attributes/attributesconnect"
	"github.com/opentdf/platform/protocol/go/wellknownconfiguration"
	"github.com/opentdf/platform/protocol/go/wellknownconfiguration/wellknownconfigurationconnect"
	reference "github.com/opentdf/platform/sdk"
	"google.golang.org/protobuf/encoding/protojson"
	"google.golang.org/protobuf/proto"
	"google.golang.org/protobuf/types/known/structpb"
	gc "opentdf-local/sdk/src"
)

// The KAS entries of xtest's fixtures (kas_entry_default, _alpha, ...).
const (
	kasDefault = "http://localhost:8080/kas"
	kasAlpha   = "http://localhost:8181/kas"
	kasBeta    = "http://localhost:8282/kas"
	kasGamma   = "http://localhost:8383/kas"
	kasDelta   = "http://localhost:8484/kas"
	kasKM1     = "http://localhost:8585/kas"
	kasKM2     = "http://localhost:8686/kas"
)

var allKAS = []string{kasDefault, kasAlpha, kasBeta, kasGamma, kasDelta, kasKM1, kasKM2}

// assignment is a grant or key assignment of xtest's otdfctl calls:
// grant_assign_value/attr/ns or key_assign_value/attr/ns.
type assignment struct {
	level string // value, attr or ns
	value string // the value name, for level value
	kas   string
	kid   string // r1 and other r* kids are RSA-2048, e* kids EC P-256
}

type definition struct {
	ns, name string
	rule     policy.AttributeRuleTypeEnum
	values   []string
	assign   []assignment
}

type fixture struct {
	name    string
	defs    []definition
	encrypt []string // ns/name/value
	keyed   bool     // key management only (no legacy grant form)
}

const (
	anyOf     = policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF
	allOf     = policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ALL_OF
	hierarchy = policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_HIERARCHY
	noRule    = policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_UNSPECIFIED
)

func v(value, kas, kid string) assignment { return assignment{"value", value, kas, kid} }
func a(kas, kid string) assignment        { return assignment{"attr", "", kas, kid} }
func n(kas, kid string) assignment        { return assignment{"ns", "", kas, kid} }

// fixtures translates ../tests/xtest/fixtures/attributes.py and the
// key-assignment fixtures of fixtures/keys.py, with the value lists the
// test_abac.py cells encrypt with. Post-quantum and extended-mechanism keys
// are left out: the shared source does not plan them in this phase.
var fixtures = []fixture{
	{name: "attribute_single_kas_grant", defs: []definition{{"tmp.com", "letter", anyOf, []string{"a"}, []assignment{v("a", kasAlpha, "r1")}}}, encrypt: []string{"tmp.com/letter/a"}},
	{name: "attribute_two_kas_grant_or", defs: []definition{{"tmp.com", "letra", anyOf, []string{"alpha", "beta"}, []assignment{v("alpha", kasAlpha, "r1"), v("beta", kasBeta, "r1")}}}, encrypt: []string{"tmp.com/letra/alpha", "tmp.com/letra/beta"}},
	{name: "attribute_two_kas_grant_and", defs: []definition{{"tmp.com", "ot", allOf, []string{"alef", "bet", "gimmel"}, []assignment{v("alef", kasAlpha, "r1"), v("bet", kasBeta, "r1")}}}, encrypt: []string{"tmp.com/ot/alef", "tmp.com/ot/bet"}},
	{name: "attribute_two_kas_grant_and_all_values", defs: []definition{{"tmp.com", "ot", allOf, []string{"alef", "bet", "gimmel"}, []assignment{v("alef", kasAlpha, "r1"), v("bet", kasBeta, "r1")}}}, encrypt: []string{"tmp.com/ot/alef", "tmp.com/ot/bet", "tmp.com/ot/gimmel"}},
	{name: "one_attribute_attr_kas_grant", defs: []definition{{"tmp.com", "attrgrant", anyOf, []string{"alpha"}, []assignment{a(kasGamma, "r1")}}}, encrypt: []string{"tmp.com/attrgrant/alpha"}},
	{name: "attribute_with_or_type", defs: []definition{{"tmp.com", "or", anyOf, []string{"alpha", "beta"}, nil}}, encrypt: []string{"tmp.com/or/alpha", "tmp.com/or/beta"}},
	{name: "attribute_with_and_type", defs: []definition{{"tmp.com", "and", allOf, []string{"alpha", "beta"}, nil}}, encrypt: []string{"tmp.com/and/alpha", "tmp.com/and/beta"}},
	{name: "attribute_with_hierarchy_type", defs: []definition{{"tmp.com", "hierarchy", hierarchy, []string{"alpha", "beta", "gamma"}, nil}}, encrypt: []string{"tmp.com/hierarchy/alpha"}},
	{name: "attr_and_value_kas_grants_or", defs: []definition{{"tmp.com", "attrorvalgrant", anyOf, []string{"alpha", "beta"}, []assignment{a(kasGamma, "r1"), v("beta", kasAlpha, "r1")}}}, encrypt: []string{"tmp.com/attrorvalgrant/alpha", "tmp.com/attrorvalgrant/beta"}},
	{name: "attr_and_value_kas_grants_and", defs: []definition{{"tmp.com", "attrandvalgrant", allOf, []string{"alpha", "beta"}, []assignment{a(kasGamma, "r1"), v("beta", kasAlpha, "r1")}}}, encrypt: []string{"tmp.com/attrandvalgrant/alpha", "tmp.com/attrandvalgrant/beta"}},
	{name: "one_attribute_ns_kas_grant", defs: []definition{{"tmp.com", "nsgrant", anyOf, []string{"alpha"}, []assignment{n(kasDelta, "r1")}}}, encrypt: []string{"tmp.com/nsgrant/alpha"}},
	{name: "ns_and_value_kas_grants_or", defs: []definition{{"nsor.com", "nsorvalgrant", anyOf, []string{"alpha", "beta"}, []assignment{v("beta", kasAlpha, "r1"), n(kasDelta, "r1")}}}, encrypt: []string{"nsor.com/nsorvalgrant/alpha", "nsor.com/nsorvalgrant/beta"}},
	{name: "ns_and_value_kas_grants_and", defs: []definition{{"nsand.com", "nsandvalgrant", allOf, []string{"alpha", "beta"}, []assignment{v("beta", kasAlpha, "r1"), n(kasDelta, "r1")}}}, encrypt: []string{"nsand.com/nsandvalgrant/alpha", "nsand.com/nsandvalgrant/beta"}},
	{name: "attribute_default_rsa", defs: []definition{{"tmp.com", "defaultrsa", anyOf, []string{"wrapped"}, []assignment{v("wrapped", kasDefault, "r1")}}}, encrypt: []string{"tmp.com/defaultrsa/wrapped"}},
	{name: "attribute_with_different_kids", keyed: true, defs: []definition{{"tmp.com", "multikeys", allOf, []string{"r1", "e1"}, []assignment{v("r1", kasDefault, "r1"), v("e1", kasDefault, "e1")}}}, encrypt: []string{"tmp.com/multikeys/r1", "tmp.com/multikeys/e1"}},
	{name: "attribute_allof_with_two_managed_keys", keyed: true, defs: []definition{{"tmp.com", "kmallof", allOf, []string{"r1", "e1"}, []assignment{a(kasKM1, "km1-rsa"), a(kasKM2, "km2-ec")}}}, encrypt: []string{"tmp.com/kmallof/r1", "tmp.com/kmallof/e1"}},
	{name: "test_policytypes_or_and_hierarchy", defs: []definition{
		{"tmp.com", "or", anyOf, []string{"alpha", "beta"}, nil},
		{"tmp.com", "and", allOf, []string{"alpha", "beta"}, nil},
		{"tmp.com", "hierarchy", hierarchy, []string{"alpha", "beta", "gamma"}, nil},
	}, encrypt: []string{"tmp.com/or/alpha", "tmp.com/and/alpha", "tmp.com/hierarchy/gamma"}},
	{name: "two_kas_grant_and_with_single_grant", defs: []definition{
		{"tmp.com", "ot", allOf, []string{"alef", "bet", "gimmel"}, []assignment{v("alef", kasAlpha, "r1"), v("bet", kasBeta, "r1")}},
		{"tmp.com", "letter", anyOf, []string{"a"}, []assignment{v("a", kasAlpha, "r1")}},
	}, encrypt: []string{"tmp.com/ot/alef", "tmp.com/ot/bet", "tmp.com/letter/a"}},
}

// How the platform reports the assignments, and how the writer reads them.
type dataMode string

const (
	legacyGrants dataMode = "grants"            // KeyAccessServer{uri}: keys fetched by URL
	grantKeys    dataMode = "grants-with-keys"  // KeyAccessServer{uri, kas_keys}
	cachedGrants dataMode = "grants-cached-key" // KeyAccessServer{uri, public_key.cached}
	mappedKeys   dataMode = "kas-keys"          // kas_keys on the value, definition or namespace
)

type path string

const (
	keyMappings path = "GetKeyMappingsByFqns"
	olderServer path = "GetAttributeValuesByFqns (GetKeyMappingsByFqns unimplemented)"
	offline     path = "WithDataAttributeValues"
)

type keyPair struct{ rsa, ec string }

func newKeys(t *testing.T) keyPair {
	t.Helper()
	r, err := rsa.GenerateKey(rand.Reader, 2048)
	if err != nil {
		t.Fatal(err)
	}
	e, err := ecdsa.GenerateKey(elliptic.P256(), rand.Reader)
	if err != nil {
		t.Fatal(err)
	}
	encode := func(pub any) string {
		der, err := x509.MarshalPKIXPublicKey(pub)
		if err != nil {
			t.Fatal(err)
		}
		return string(pem.EncodeToMemory(&pem.Block{Type: "PUBLIC KEY", Bytes: der}))
	}
	return keyPair{encode(&r.PublicKey), encode(&e.PublicKey)}
}

func (k keyPair) simple(kas, kid string) *policy.SimpleKasKey {
	if strings.HasPrefix(kid, "e") || strings.HasSuffix(kid, "-ec") {
		return &policy.SimpleKasKey{KasUri: kas, PublicKey: &policy.SimpleKasPublicKey{Algorithm: policy.Algorithm_ALGORITHM_EC_P256, Kid: kid, Pem: k.ec}}
	}
	return &policy.SimpleKasKey{KasUri: kas, PublicKey: &policy.SimpleKasPublicKey{Algorithm: policy.Algorithm_ALGORITHM_RSA_2048, Kid: kid, Pem: k.rsa}}
}

func (k keyPair) grant(mode dataMode, kas, kid string) *policy.KeyAccessServer {
	g := &policy.KeyAccessServer{Uri: kas, Id: kas}
	switch mode {
	case grantKeys:
		g.KasKeys = []*policy.SimpleKasKey{k.simple(kas, kid)}
	case cachedGrants:
		sk := k.simple(kas, kid)
		alg := policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_RSA_2048
		if sk.GetPublicKey().GetAlgorithm() == policy.Algorithm_ALGORITHM_EC_P256 {
			alg = policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP256R1
		}
		g.PublicKey = &policy.PublicKey{PublicKey: &policy.PublicKey_Cached{Cached: &policy.KasPublicKeySet{Keys: []*policy.KasPublicKey{{Pem: sk.GetPublicKey().GetPem(), Kid: kid, Alg: alg}}}}}
	}
	return g
}

// values builds the policy.Value of every attribute value of the fixture,
// each with its definition and namespace, as the platform returns them.
func (k keyPair) values(fx fixture, mode dataMode) map[string]*policy.Value {
	namespaces := map[string]*policy.Namespace{}
	for _, d := range fx.defs {
		if namespaces[d.ns] == nil {
			namespaces[d.ns] = &policy.Namespace{Id: d.ns, Name: d.ns, Fqn: "https://" + d.ns}
		}
	}
	add := func(grants *[]*policy.KeyAccessServer, keys *[]*policy.SimpleKasKey, as assignment) {
		if mode == mappedKeys {
			*keys = append(*keys, k.simple(as.kas, as.kid))
		} else {
			*grants = append(*grants, k.grant(mode, as.kas, as.kid))
		}
	}
	for _, d := range fx.defs {
		for _, as := range d.assign {
			if as.level == "ns" {
				ns := namespaces[d.ns]
				add(&ns.Grants, &ns.KasKeys, as)
			}
		}
	}
	out := map[string]*policy.Value{}
	for _, d := range fx.defs {
		attr := &policy.Attribute{Id: d.ns + "/" + d.name, Namespace: namespaces[d.ns], Name: d.name, Rule: d.rule, Fqn: "https://" + d.ns + "/attr/" + d.name}
		for _, as := range d.assign {
			if as.level == "attr" {
				add(&attr.Grants, &attr.KasKeys, as)
			}
		}
		for _, name := range d.values {
			val := &policy.Value{Id: attr.Id + "/" + name, Attribute: attr, Value: name, Fqn: attr.Fqn + "/value/" + name}
			for _, as := range d.assign {
				if as.level == "value" && as.value == name {
					add(&val.Grants, &val.KasKeys, as)
				}
			}
			out[strings.ToLower(val.Fqn)] = val
		}
	}
	return out
}

// platform is the fake policy and well-known service both SDKs call.
type platform struct {
	attributesconnect.UnimplementedAttributesServiceHandler
	mu          sync.Mutex
	values      map[string]*policy.Value
	mode        dataMode
	unsupported bool
}

func (p *platform) set(values map[string]*policy.Value, mode dataMode, unsupported bool) {
	p.mu.Lock()
	defer p.mu.Unlock()
	p.values, p.mode, p.unsupported = values, mode, unsupported
}

func (p *platform) GetAttributeValuesByFqns(_ context.Context, req *connect.Request[attributes.GetAttributeValuesByFqnsRequest]) (*connect.Response[attributes.GetAttributeValuesByFqnsResponse], error) {
	p.mu.Lock()
	defer p.mu.Unlock()
	found := map[string]*attributes.GetAttributeValuesByFqnsResponse_AttributeAndValue{}
	for _, fqn := range req.Msg.GetFqns() {
		val, ok := p.values[strings.ToLower(fqn)]
		if !ok {
			return nil, connect.NewError(connect.CodeNotFound, fmt.Errorf("resource not found"))
		}
		found[fqn] = &attributes.GetAttributeValuesByFqnsResponse_AttributeAndValue{Attribute: val.GetAttribute(), Value: val}
	}
	return connect.NewResponse(&attributes.GetAttributeValuesByFqnsResponse{FqnAttributeValues: found}), nil
}

func (p *platform) GetKeyMappingsByFqns(_ context.Context, req *connect.Request[attributes.GetKeyMappingsByFqnsRequest]) (*connect.Response[attributes.GetKeyMappingsByFqnsResponse], error) {
	p.mu.Lock()
	defer p.mu.Unlock()
	if p.unsupported {
		return nil, connect.NewError(connect.CodeUnimplemented, fmt.Errorf("not implemented"))
	}
	out := map[string]*attributes.GetKeyMappingsByFqnsResponse_AttributeKeyMapping{}
	for _, fqn := range req.Msg.GetFqns() {
		val, ok := p.values[strings.ToLower(fqn)]
		if !ok {
			continue
		}
		mapping := &attributes.GetKeyMappingsByFqnsResponse_AttributeKeyMapping{Rule: val.GetAttribute().GetRule()}
		// Legacy grants are not used by this API; the effective kas_keys are
		// the value's, else the definition's, else the namespace's.
		for _, keys := range [][]*policy.SimpleKasKey{val.GetKasKeys(), val.GetAttribute().GetKasKeys(), val.GetAttribute().GetNamespace().GetKasKeys()} {
			if len(keys) > 0 {
				mapping.Keys = keys
				break
			}
		}
		out[strings.ToLower(fqn)] = mapping
	}
	return connect.NewResponse(&attributes.GetKeyMappingsByFqnsResponse{FqnKeyMappings: out}), nil
}

type wellKnown struct {
	wellknownconfigurationconnect.UnimplementedWellKnownServiceHandler
}

func (wellKnown) GetWellKnownConfiguration(context.Context, *connect.Request[wellknownconfiguration.GetWellKnownConfigurationRequest]) (*connect.Response[wellknownconfiguration.GetWellKnownConfigurationResponse], error) {
	cfg, err := structpb.NewStruct(map[string]any{"health": map[string]any{"endpoint": "/healthz"}})
	if err != nil {
		return nil, err
	}
	return connect.NewResponse(&wellknownconfiguration.GetWellKnownConfigurationResponse{Configuration: cfg}), nil
}

type kao struct{ URL, KID, Type, Split string }

// normalize names each split id by the position of its first key access
// object: the ids are random UUIDs, their grouping is the plan.
func normalize(kaos []kao) []kao {
	ids := map[string]string{}
	out := make([]kao, len(kaos))
	for i, k := range kaos {
		if k.Split != "" {
			if _, ok := ids[k.Split]; !ok {
				ids[k.Split] = fmt.Sprintf("split-%d", len(ids))
			}
			k.Split = ids[k.Split]
		}
		out[i] = k
	}
	return out
}

type writers struct {
	ref  *reference.SDK
	gc   *gc.SDK
	keys keyPair
}

func newWriters(t *testing.T, endpoint string, keys keyPair) writers {
	t.Helper()
	logger := slog.New(slog.NewTextHandler(io.Discard, nil))
	ref, err := reference.New(endpoint, reference.WithPlatformConfiguration(reference.PlatformConfiguration{"platform_issuer": endpoint}), reference.WithInsecurePlaintextConn(), reference.WithLogger(logger))
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { ref.Close() })
	g, err := gc.New(context.Background(), endpoint, gc.Options{PlatformConfiguration: gc.PlatformConfiguration{Entries: []gc.PlatformConfigurationEntry{{Key: "platform_issuer", Value: endpoint}}}, InsecurePlaintext: true, LogLevel: 8})
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { g.Close() })
	return writers{ref, g, keys}
}

// kasInformation is otdfctl's: the platform KAS is the default, and every
// KAS public key is known, so neither writer reaches a KAS.
func (w writers) kasInformation() ([]reference.KASInfo, []gc.KASInfo) {
	var r []reference.KASInfo
	var g []gc.KASInfo
	for _, url := range allKAS {
		r = append(r, reference.KASInfo{URL: url, PublicKey: w.keys.rsa, KID: "r1", Algorithm: "rsa:2048", Default: url == kasDefault})
		g = append(g, gc.KASInfo{URL: url, PublicKey: w.keys.rsa, KID: "r1", Algorithm: "rsa:2048", Default: url == kasDefault})
	}
	return r, g
}

type result struct {
	kaos  []kao
	attrs []string
	err   error
}

func (w writers) encrypt(t *testing.T, fqns []string, values []*policy.Value, via path) (result, result) {
	t.Helper()
	refKAS, gcKAS := w.kasInformation()
	refOpts := []reference.TDFOption{reference.WithKasInformation(refKAS...)}
	gcOpts := gc.TDFOptions{KasInformation: gcKAS}
	if via == offline {
		refOpts = append(refOpts, reference.WithDataAttributeValues(values...))
		for _, val := range values {
			data, err := protojson.Marshal(val)
			if err != nil {
				t.Fatal(err)
			}
			gcOpts.AttributeValues = append(gcOpts.AttributeValues, string(data))
		}
	} else {
		refOpts = append(refOpts, reference.WithDataAttributes(fqns...))
		gcOpts.Attributes = fqns
	}

	var refResult, gcResult result
	var out bytes.Buffer
	obj, err := w.ref.CreateTDF(&out, bytes.NewReader([]byte("plan")), refOpts...)
	if err != nil {
		refResult.err = err
	} else {
		for _, k := range obj.Manifest().EncryptionInformation.KeyAccessObjs {
			refResult.kaos = append(refResult.kaos, kao{k.KasURL, k.KID, k.KeyType, k.SplitID})
		}
	}
	_, gobj, err := w.gc.EncryptBytes(context.Background(), []byte("plan"), gcOpts)
	if err != nil {
		gcResult.err = err
	} else {
		for _, k := range gobj.Manifest().EncryptionInformation.KeyAccessObjs {
			gcResult.kaos = append(gcResult.kaos, kao{k.KasURL, k.KID, k.KeyType, k.SplitID})
		}
	}
	refResult.kaos, gcResult.kaos = normalize(refResult.kaos), normalize(gcResult.kaos)
	return refResult, gcResult
}

func compare(t *testing.T, label string, refResult, gcResult result) {
	t.Helper()
	if (refResult.err == nil) != (gcResult.err == nil) {
		t.Fatalf("%s: reference error %v, shared source error %v", label, refResult.err, gcResult.err)
	}
	if fmt.Sprint(refResult.kaos) != fmt.Sprint(gcResult.kaos) {
		t.Fatalf("%s:\nreference     %v\nshared source %v", label, refResult.kaos, gcResult.kaos)
	}
}

func startPlatform(t *testing.T) (*platform, string) {
	t.Helper()
	p := &platform{}
	mux := http.NewServeMux()
	mux.Handle(attributesconnect.NewAttributesServiceHandler(p))
	mux.Handle(wellknownconfigurationconnect.NewWellKnownServiceHandler(wellKnown{}))
	server := httptest.NewServer(mux)
	t.Cleanup(server.Close)
	return p, server.URL
}

func run(t *testing.T, p *platform, w writers, label string, fx fixture) int {
	t.Helper()
	cases := 0
	modes := []dataMode{legacyGrants, grantKeys, cachedGrants, mappedKeys}
	if fx.keyed {
		modes = []dataMode{mappedKeys}
	}
	for _, mode := range modes {
		values := w.keys.values(fx, mode)
		var fqns []string
		var selected []*policy.Value
		for _, e := range fx.encrypt {
			parts := strings.SplitN(e, "/", 3)
			fqn := "https://" + parts[0] + "/attr/" + parts[1] + "/value/" + parts[2]
			fqns = append(fqns, fqn)
			selected = append(selected, proto.Clone(values[strings.ToLower(fqn)]).(*policy.Value))
		}
		for _, via := range []path{keyMappings, olderServer, offline} {
			p.set(values, mode, via == olderServer)
			refResult, gcResult := w.encrypt(t, fqns, selected, via)
			compare(t, fmt.Sprintf("%s [%s, %s]", label, mode, via), refResult, gcResult)
			if refResult.err == nil && len(refResult.kaos) == 0 {
				t.Fatalf("%s: no key access objects", label)
			}
			cases++
		}
	}
	return cases
}

// TestSplitPlanMatchesReference runs the xtest attribute fixtures through
// both writers, for every way the platform can report the grants and keys.
func TestSplitPlanMatchesReference(t *testing.T) {
	p, endpoint := startPlatform(t)
	w := newWriters(t, endpoint, newKeys(t))
	cases := 0
	for _, fx := range fixtures {
		cases += run(t, p, w, fx.name, fx)
	}
	t.Logf("%d fixture cases, identical key access objects", cases)
}

// TestSplitPlanMatchesReferenceRandom compares random policies: up to three
// definitions over two namespaces with every rule, assignments at every
// level, and random value selections.
func TestSplitPlanMatchesReferenceRandom(t *testing.T) {
	p, endpoint := startPlatform(t)
	w := newWriters(t, endpoint, newKeys(t))
	r := mrand.New(mrand.NewSource(4))
	rules := []policy.AttributeRuleTypeEnum{anyOf, allOf, hierarchy, noRule}
	kids := []string{"r1", "e1", "r2"}
	pick := func() assignment { return assignment{kas: allKAS[r.Intn(len(allKAS))], kid: kids[r.Intn(len(kids))]} }
	cases := 0
	for i := 0; i < 150; i++ {
		fx := fixture{name: fmt.Sprintf("random-%d", i)}
		nsAssign := map[string][]assignment{}
		for _, ns := range []string{"a.com", "b.com"} {
			for j := r.Intn(4) - 2; j > 0; j-- {
				as := pick()
				as.level = "ns"
				nsAssign[ns] = append(nsAssign[ns], as)
			}
		}
		var all []string
		for d := 0; d <= r.Intn(3); d++ {
			ns := []string{"a.com", "b.com"}[r.Intn(2)]
			def := definition{ns: ns, name: fmt.Sprintf("d%d", d), rule: rules[r.Intn(len(rules))]}
			def.assign = append(def.assign, nsAssign[ns]...)
			for j := r.Intn(4) - 2; j > 0; j-- {
				as := pick()
				as.level = "attr"
				def.assign = append(def.assign, as)
			}
			for k := 0; k <= r.Intn(3); k++ {
				name := fmt.Sprintf("v%d", k)
				def.values = append(def.values, name)
				all = append(all, ns+"/"+def.name+"/"+name)
				for j := r.Intn(4) - 1; j > 0; j-- {
					as := pick()
					as.level, as.value = "value", name
					def.assign = append(def.assign, as)
				}
			}
			fx.defs = append(fx.defs, def)
		}
		r.Shuffle(len(all), func(x, y int) { all[x], all[y] = all[y], all[x] })
		fx.encrypt = all[:1+r.Intn(len(all))]
		cases += run(t, p, w, fx.name, fx)
	}
	t.Logf("%d random cases, identical key access objects", cases)
}
