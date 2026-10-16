package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/log/slog"
	"github.com/eugenioenko/goalchemy/std/net/url"
	"github.com/eugenioenko/goalchemy/std/sort"
	"github.com/eugenioenko/goalchemy/std/strings"
	"opentdf-local/sdk/src/protocol/policy"
	"opentdf-local/sdk/src/protocol/policy/attributes"
)

var ErrInvalid = errors.New("invalid type")

// Attribute rule types: operators!
const (
	hierarchy   = "hierarchy"
	allOf       = "allOf"
	anyOf       = "anyOf"
	unspecified = "unspecified"
	emptyTerm   = "DEFAULT"
)

// keySplitStep represents a which KAS a split with the associated ID should be shared with.
type keySplitStep struct {
	KAS, SplitID string
}

// Template for a KAS Access Object (KAO).
// It is filled in during manifest creation by splitting the key material (DEK) across each split ID,
// then wrapping each split with the KAS public key identified by the KID at each element.
type kaoTpl struct {
	KAS, SplitID string
	kid          string
	pem          string
	algorithm    KeyType
}

// AttributeNameFQN represents the FQN for an attribute. The reference's
// unexported url and key are exported so the value crosses the Goalchemy
// boundary; construct it with NewAttributeNameFQN.
type AttributeNameFQN struct {
	URL, Key string
}

// Lookup Table for KAS keys, indexed by ResourceLocator.
type rlKeyCache struct {
	c map[ResourceLocator]*policy.SimpleKasKey
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

// NewAttributeNameFQN parses an attribute FQN: https://<namespace>/attr/<name>.
func NewAttributeNameFQN(u string) (AttributeNameFQN, error) {
	a, err := newAttributeNameFQN(u)
	return a, boundaryError(KindInvalidInput, "NewAttributeNameFQN", err)
}

func newAttributeNameFQN(u string) (AttributeNameFQN, error) {
	m := matchAttributeNameFQN(u)
	if len(m) < 3 || len(m[0]) == 0 {
		return AttributeNameFQN{}, fmt.Errorf("%w: attribute regex fail", ErrInvalid)
	}

	_, err := url.PathUnescape(m[2])
	if err != nil {
		return AttributeNameFQN{}, fmt.Errorf("%w: error in attribute name [%s]", ErrInvalid, m[2])
	}

	if err := attributeURLPartsValid(m); err != nil {
		return AttributeNameFQN{}, err
	}

	return AttributeNameFQN{u, strings.ToLower(u)}, nil
}

func (a AttributeNameFQN) String() string {
	return a.URL
}

func (a AttributeNameFQN) Select(v string) AttributeValueFQN {
	u := fmt.Sprintf("%s/value/%s", a.URL, url.PathEscape(v))
	return AttributeValueFQN{u, strings.ToLower(u)}
}

func (a AttributeNameFQN) Prefix() string {
	return a.URL
}

func (a AttributeNameFQN) Authority() string {
	m := matchAttributeNameFQN(a.URL)
	if m == nil {
		panic(ErrInvalid)
	}
	return m[1]
}

func (a AttributeNameFQN) Name() string {
	m := matchAttributeNameFQN(a.URL)
	if m == nil {
		panic("invalid attribute")
	}
	v, err := url.PathUnescape(m[2])
	if err != nil {
		panic(ErrInvalid)
	}
	return v
}

// AttributeValueFQN is a utility type to represent an FQN for an attribute value.
// Construct it with NewAttributeValueFQN.
type AttributeValueFQN struct {
	URL, Key string
}

// NewAttributeValueFQN parses an attribute value FQN:
// https://<namespace>/attr/<name>/value/<value>.
func NewAttributeValueFQN(u string) (AttributeValueFQN, error) {
	a, err := newAttributeValueFQN(u)
	return a, boundaryError(KindInvalidInput, "NewAttributeValueFQN", err)
}

func newAttributeValueFQN(u string) (AttributeValueFQN, error) {
	m := matchAttributeValueFQN(u)
	if len(m) < 4 || len(m[0]) == 0 {
		return AttributeValueFQN{}, fmt.Errorf("%w: attribute regex fail for [%s]", ErrInvalid, u)
	}

	_, err := url.PathUnescape(m[2])
	if err != nil {
		return AttributeValueFQN{}, fmt.Errorf("%w: error in attribute name [%s]", ErrInvalid, m[2])
	}
	_, err = url.PathUnescape(m[3])
	if err != nil {
		return AttributeValueFQN{}, fmt.Errorf("%w: error in attribute value [%s]", ErrInvalid, m[3])
	}

	if err := attributeURLPartsValid(m); err != nil {
		return AttributeValueFQN{}, err
	}

	return AttributeValueFQN{u, strings.ToLower(u)}, nil
}

func (a AttributeValueFQN) String() string {
	return a.URL
}

func (a AttributeValueFQN) Authority() string {
	m := matchAttributeValueFQN(a.URL)
	if m == nil {
		panic(ErrInvalid)
	}
	return m[1]
}

func (a AttributeValueFQN) Prefix() AttributeNameFQN {
	m := matchAttributeValueFQN(a.URL)
	if m == nil {
		panic(ErrInvalid)
	}
	p, err := newAttributeNameFQN(m[1] + "/attr/" + m[2])
	if err != nil {
		panic(ErrInvalid)
	}
	return p
}

func (a AttributeValueFQN) Value() string {
	m := matchAttributeValueFQN(a.String())
	if m == nil {
		panic(ErrInvalid)
	}
	v, err := url.PathUnescape(m[3])
	if err != nil {
		panic(ErrInvalid)
	}
	return v
}

func (a AttributeValueFQN) Name() string {
	m := matchAttributeValueFQN(a.URL)
	if m == nil {
		panic("invalid attributeInstance")
	}
	v, err := url.PathUnescape(m[2])
	if err != nil {
		panic("invalid attributeInstance")
	}
	return v
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

// Structure capable of generating a split plan from a given set of data tags.
type granter struct {
	logger *slog.Logger

	// The data attributes (tags) that this granter is responsible for.
	tags []AttributeValueFQN

	// Older map of attribute values to associated KASes. Replaced by mapTable.
	grantTable map[string]*keyAccessGrant

	// Map from attribute values to KAS and key identifier.
	mapTable map[string][]*ResourceLocator

	// The types of grants or mapped keys found.
	typ grantType

	// The key cache to store KAS keys.
	keyCache *rlKeyCache

	// Key lookup for keys without a KID.
	keyInfoFetcher KASKeyFetcher
}

type keyAccessGrant struct {
	attr  *policy.Attribute
	kases []string
}

func (r *granter) addGrant(fqn AttributeValueFQN, kas string, attr *policy.Attribute) {
	if _, ok := r.grantTable[fqn.Key]; ok {
		r.grantTable[fqn.Key].kases = append(r.grantTable[fqn.Key].kases, kas)
	} else {
		r.grantTable[fqn.Key] = &keyAccessGrant{attr, []string{kas}}
	}
}

func (r *granter) addMappedKey(fqn AttributeValueFQN, sk *policy.SimpleKasKey) error {
	key := sk.GetPublicKey()
	if key == nil || key.GetKid() == "" || key.GetPem() == "" {
		r.logger.Debug("invalid cached key in policy service",
			slog.String("kas", sk.GetKasUri()),
			slog.String("value", fqn.String()),
		)
		return fmt.Errorf("invalid cached key in policy service associated with [%s]", fqn)
	}
	if r.mapTable == nil {
		r.mapTable = make(map[string][]*ResourceLocator)
	}
	rls := r.mapTable[fqn.Key]
	if rls == nil {
		rls = make([]*ResourceLocator, 0)
		r.mapTable[fqn.Key] = rls
	}

	rl, err := newResourceLocator(sk.GetKasUri())
	if err != nil {
		r.logger.Debug("invalid KAS URL in policy service",
			slog.String("kas", sk.GetKasUri()),
			slog.String("value", fqn.String()),
			slog.String("error", err.Error()),
		)
		return fmt.Errorf("invalid KAS URL in policy service associated with [%s]: %w", fqn, err)
	}
	rl.identifier = key.GetKid()
	r.logger.Debug("added mapped key",
		slog.String("fqn", fqn.String()),
		slog.String("kas", sk.GetKasUri()),
		slog.String("kid", key.GetKid()),
		slog.String("alg", algProto2String(policy.KasPublicKeyAlgEnum(key.GetAlgorithm()))),
	)
	rls = append(rls, rl)
	r.mapTable[fqn.Key] = rls
	r.keyCache.c[*rl] = sk
	return nil
}

func convertAlgEnum2Simple(a policy.KasPublicKeyAlgEnum) policy.Algorithm {
	switch a {
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP256R1:
		return policy.Algorithm_ALGORITHM_EC_P256
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP384R1:
		return policy.Algorithm_ALGORITHM_EC_P384
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP521R1:
		return policy.Algorithm_ALGORITHM_EC_P521
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_RSA_2048:
		return policy.Algorithm_ALGORITHM_RSA_2048
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_RSA_4096:
		return policy.Algorithm_ALGORITHM_RSA_4096
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_HPQT_XWING:
		return policy.Algorithm_ALGORITHM_HPQT_XWING
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP256R1_MLKEM768:
		return policy.Algorithm_ALGORITHM_HPQT_SECP256R1_MLKEM768
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP384R1_MLKEM1024:
		return policy.Algorithm_ALGORITHM_HPQT_SECP384R1_MLKEM1024
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_768:
		return policy.Algorithm_ALGORITHM_MLKEM_768
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_1024:
		return policy.Algorithm_ALGORITHM_MLKEM_1024
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_UNSPECIFIED:
		return policy.Algorithm_ALGORITHM_UNSPECIFIED
	default:
		return policy.Algorithm_ALGORITHM_UNSPECIFIED
	}
}

type grantType int

const (
	noKeysFound grantType = iota // No keys found
	grantsFound                  // Only grants found
	mappedFound                  // Only mapped keys found
)

type grantableObject interface {
	GetGrants() []*policy.KeyAccessServer
	GetKasKeys() []*policy.SimpleKasKey
}

// addAllGrants adds all grants from a list of KASes to the granter.
// It returns an enum value indicating what types of keys were found.
func (r *granter) addAllGrants(fqn AttributeValueFQN, ag grantableObject, attr *policy.Attribute) grantType {
	result := noKeysFound

	// Check for mapped keys
	for _, k := range ag.GetKasKeys() {
		if k == nil || k.GetKasUri() == "" {
			r.logger.Debug("invalid KAS key in policy service",
				slog.String("kas", k.GetKasUri()),
				slog.String("value", fqn.String()),
			)
			continue
		}
		kasURI := k.GetKasUri()
		r.typ = mappedFound
		result = r.typ
		err := r.addMappedKey(fqn, k)
		if err != nil {
			r.logger.Debug("failed to add mapped key",
				slog.String("fqn", fqn.String()),
				slog.String("kas", kasURI),
				slog.String("error", err.Error()),
			)
		}
		if _, present := r.grantTable[fqn.Key]; !present {
			r.grantTable[fqn.Key] = &keyAccessGrant{attr, []string{kasURI}}
		} else {
			r.grantTable[fqn.Key].kases = append(r.grantTable[fqn.Key].kases, kasURI)
		}
	}
	if result != noKeysFound {
		return result
	}

	for _, g := range ag.GetGrants() {
		if g != nil && g.GetUri() != "" {
			kasURI := g.GetUri()
			r.typ = grantsFound
			result = grantsFound
			r.addGrant(fqn, kasURI, attr)
			if len(g.GetKasKeys()) != 0 {
				for _, k := range g.GetKasKeys() {
					err := r.addMappedKey(fqn, k)
					if err != nil {
						r.logger.Warn("failed to add mapped key",
							slog.String("fqn", fqn.String()),
							slog.String("kas", kasURI),
							slog.String("error", err.Error()),
						)
					}
				}
				continue
			}
			ks := g.GetPublicKey().GetCached().GetKeys()
			if len(ks) == 0 {
				r.logger.Debug("no cached key in policy service",
					slog.String("kas", kasURI),
					slog.String("value", fqn.String()),
				)
				continue
			}
			for _, k := range ks {
				if k.GetKid() == "" || k.GetPem() == "" {
					r.logger.Debug("invalid cached key in policy service",
						slog.String("kas", kasURI),
						slog.String("value", fqn.String()),
						slog.String("kid", k.GetKid()),
					)
					continue
				}
				sk := &policy.SimpleKasKey{
					KasUri: kasURI,
					PublicKey: &policy.SimpleKasPublicKey{
						Algorithm: convertAlgEnum2Simple(k.GetAlg()),
						Pem:       k.GetPem(),
						Kid:       k.GetKid(),
					},
					KasId: g.GetId(),
				}
				err := r.addMappedKey(fqn, sk)
				if err != nil {
					r.logger.Warn("failed to add mapped key",
						slog.String("fqn", fqn.String()),
						slog.String("kas", kasURI),
						slog.String("error", err.Error()),
					)
				}
			}
		}
	}
	if result == noKeysFound {
		if _, present := r.grantTable[fqn.Key]; !present {
			r.grantTable[fqn.Key] = &keyAccessGrant{attr, []string{}}
		}
	}
	return result
}

func (r granter) byAttribute(fqn AttributeValueFQN) *keyAccessGrant {
	return r.grantTable[fqn.Key]
}

// Gets a list of directory of KAS grants for a list of attribute FQNs.
//
// Resolution uses GetKeyMappingsByFqns, which returns the effective mapped KAS
// keys per value (value > definition > namespace precedence, resolved
// server-side). Values that resolve to no mapped keys (e.g. configured only with
// legacy KAS grants) fall back to GetAttributeValuesByFqns so their grants still
// resolve.
func newGranterFromService(ctx context.Context, logger *slog.Logger, keyCache *kasKeyCache, as attributesService, fqns ...AttributeValueFQN) (granter, error) {
	fqnsStr := make([]string, len(fqns))
	for i, v := range fqns {
		fqnsStr[i] = v.String()
	}

	grants := granter{
		logger:     logger,
		tags:       fqns,
		grantTable: make(map[string]*keyAccessGrant),
		keyCache:   &rlKeyCache{c: make(map[ResourceLocator]*policy.SimpleKasKey)},
	}

	km, err := as.GetKeyMappingsByFqns(ctx, &attributes.GetKeyMappingsByFqnsRequest{Fqns: fqnsStr})
	if err != nil {
		// Older platforms do not expose GetKeyMappingsByFqns; fall back to the full
		// attribute lookup for all values.
		if connectCodeOf(err) == "unimplemented" {
			if e := grants.addGrantsFromAttributeValues(ctx, logger, keyCache, as, fqnsStr); e != nil {
				return granter{}, e
			}
			return grants, nil
		}
		return granter{}, err
	}

	// Iterate the requested FQNs (not the response map) so a value the server
	// omits still falls back rather than being silently dropped. The response is
	// keyed by normalized FQN, and the protobuf getters are nil-safe, so an absent
	// or nil entry yields empty keys and resolves via the fallback below.
	mappings := km.GetFqnKeyMappings()
	var fallback []string
	for _, fqn := range fqns {
		m := mappings[fqn.Key]
		keys := m.GetKeys()
		if len(keys) == 0 {
			// No mapped keys (legacy-grant-only, unconfigured, or omitted); resolve
			// via the full attribute lookup below.
			fallback = append(fallback, fqn.Key)
			continue
		}
		// Only the rule and FQN are read downstream (constructAttributeBoolean /
		// ruleToOperator), so a minimal attribute is sufficient.
		attr := &policy.Attribute{Fqn: fqn.Prefix().String(), Rule: m.GetRule()}
		for _, sk := range keys {
			if err := grants.addMappedKey(fqn, sk); err != nil {
				logger.Warn("failed to add mapped key",
					slog.String("fqn", fqn.String()),
					slog.String("kas", sk.GetKasUri()),
					slog.String("error", err.Error()),
				)
				continue
			}
			grants.typ = mappedFound
			if _, present := grants.grantTable[fqn.Key]; !present {
				grants.grantTable[fqn.Key] = &keyAccessGrant{attr, []string{sk.GetKasUri()}}
			} else {
				grants.grantTable[fqn.Key].kases = append(grants.grantTable[fqn.Key].kases, sk.GetKasUri())
			}
			// Populate the SDK key cache for wrapping (addMappedKey already filled
			// the granter's rlKeyCache).
			storeKeysToCache(logger, nil, []*policy.SimpleKasKey{sk}, keyCache, nil)
		}
	}

	if len(fallback) > 0 {
		if err := grants.addGrantsFromAttributeValues(ctx, logger, keyCache, as, fallback); err != nil {
			return granter{}, err
		}
	}

	return grants, nil
}

// addGrantsFromAttributeValues resolves grants for the given value FQNs via the
// full GetAttributeValuesByFqns lookup (value > definition > namespace), used as
// the fallback for values with no mapped keys.
func (r *granter) addGrantsFromAttributeValues(ctx context.Context, logger *slog.Logger, keyCache *kasKeyCache, as attributesService, fqns []string) error {
	av, err := as.GetAttributeValuesByFqns(ctx, &attributes.GetAttributeValuesByFqnsRequest{Fqns: fqns})
	if err != nil {
		return err
	}
	for fqnstr, pair := range av.GetFqnAttributeValues() {
		fqn, err := newAttributeValueFQN(fqnstr)
		if err != nil {
			return err
		}
		def := pair.GetAttribute()

		if def != nil {
			storeKeysToCache(logger, def.GetGrants(), def.GetKasKeys(), keyCache, r.keyCache)
		}
		v := pair.GetValue()
		gType := noKeysFound
		if v != nil {
			gType = r.addAllGrants(fqn, v, def)
			storeKeysToCache(logger, v.GetGrants(), v.GetKasKeys(), keyCache, r.keyCache)
		}

		// If no more specific grant was found, then add the value grants
		if gType == noKeysFound && def != nil {
			gType = r.addAllGrants(fqn, def, def)
			storeKeysToCache(logger, def.GetGrants(), def.GetKasKeys(), keyCache, r.keyCache)
		}
		if gType == noKeysFound && def.GetNamespace() != nil {
			r.addAllGrants(fqn, def.GetNamespace(), def)
			storeKeysToCache(logger, def.GetNamespace().GetGrants(), def.GetNamespace().GetKasKeys(), keyCache, r.keyCache)
		}
	}

	return nil
}

func algProto2String(e policy.KasPublicKeyAlgEnum) string {
	switch e {
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP256R1:
		return string(EC256Key)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP384R1:
		return string(EC384Key)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_EC_SECP521R1:
		return string(EC521Key)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_RSA_2048:
		return string(RSA2048Key)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_RSA_4096:
		return string(RSA4096Key)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_HPQT_XWING:
		return string(hybridXWingKey)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP256R1_MLKEM768:
		return string(hybridSecp256r1MLKEM768Key)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_HPQT_SECP384R1_MLKEM1024:
		return string(hybridSecp384r1MLKEM1024Key)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_768:
		return string(mlkem768Key)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_MLKEM_1024:
		return string(mlkem1024Key)
	case policy.KasPublicKeyAlgEnum_KAS_PUBLIC_KEY_ALG_ENUM_UNSPECIFIED:
		return ""
	}
	return ""
}

func storeKeysToCache(logger *slog.Logger, kases []*policy.KeyAccessServer, keys []*policy.SimpleKasKey, c *kasKeyCache, kc *rlKeyCache) {
	for _, kas := range kases {
		keys := kas.GetPublicKey().GetCached().GetKeys()
		if len(keys) == 0 {
			logger.Debug("no cached key in policy service", slog.String("kas", kas.GetUri()))
			continue
		}
		for _, ki := range keys {
			c.store(KASInfo{
				URL:       kas.GetUri(),
				KID:       ki.GetKid(),
				Algorithm: algProto2String(ki.GetAlg()),
				PublicKey: ki.GetPem(),
			})

			// Store in rlKeyCache if sufficient information is available
			// (KID and PEM must not be empty)
			if kc != nil && ki.GetKid() != "" && ki.GetPem() != "" {
				rl, err := newResourceLocator(kas.GetUri())
				if err != nil {
					logger.Debug("failed to create ResourceLocator",
						slog.String("kas", kas.GetUri()),
						slog.String("error", err.Error()),
					)
					continue
				}
				rl.identifier = ki.GetKid()
				kc.c[*rl] = &policy.SimpleKasKey{
					KasUri: kas.GetUri(),
					PublicKey: &policy.SimpleKasPublicKey{
						Algorithm: convertAlgEnum2Simple(ki.GetAlg()),
						Pem:       ki.GetPem(),
						Kid:       ki.GetKid(),
					},
					KasId: kas.GetId(),
				}
			}
		}
	}
	for _, key := range keys {
		alg, err := formatAlg(key.GetPublicKey().GetAlgorithm())
		if err != nil {
			continue
		}
		c.store(KASInfo{
			URL:       key.GetKasUri(),
			KID:       key.GetPublicKey().GetKid(),
			Algorithm: alg,
			PublicKey: key.GetPublicKey().GetPem(),
		})

		// Store in rlKeyCache if provided
		if kc != nil && key.GetPublicKey().GetKid() != "" && key.GetPublicKey().GetPem() != "" {
			rl, err := newResourceLocator(key.GetKasUri())
			if err != nil {
				logger.Debug("failed to create ResourceLocator",
					slog.String("kas", key.GetKasUri()),
					slog.String("error", err.Error()),
				)
				continue
			}
			rl.identifier = key.GetPublicKey().GetKid()
			kc.c[*rl] = key
		}
	}
}

// Given a policy (list of data attributes or tags),
// get a set of grants from attribute values to KASes.
// Unlike `newGranterFromService`, this works offline.
func newGranterFromAttributes(logger *slog.Logger, keyCache *kasKeyCache, attrs ...*policy.Value) (granter, error) {
	grants := granter{
		logger:     logger,
		grantTable: make(map[string]*keyAccessGrant),
		mapTable:   make(map[string][]*ResourceLocator),
		tags:       make([]AttributeValueFQN, len(attrs)),
		keyCache:   &rlKeyCache{c: make(map[ResourceLocator]*policy.SimpleKasKey)},
	}
	for i, v := range attrs {
		fqn, err := newAttributeValueFQN(v.GetFqn())
		if err != nil {
			return grants, err
		}
		grants.tags[i] = fqn
		def := v.GetAttribute()
		if def == nil {
			return granter{}, fmt.Errorf("no associated definition with value [%s]", fqn)
		}
		namespace := def.GetNamespace()
		if namespace == nil {
			return granter{}, fmt.Errorf("no associated namespace with definition [%s] from value [%s]", def.GetFqn(), fqn)
		}

		if grants.addAllGrants(fqn, v, def) != noKeysFound {
			storeKeysToCache(logger, v.GetGrants(), v.GetKasKeys(), keyCache, grants.keyCache)
			continue
		}
		// If no more specific grant was found, then add the attr grants
		if grants.addAllGrants(fqn, def, def) != noKeysFound {
			storeKeysToCache(logger, def.GetGrants(), def.GetKasKeys(), keyCache, grants.keyCache)
			continue
		}
		grants.addAllGrants(fqn, namespace, def)
		storeKeysToCache(logger, namespace.GetGrants(), namespace.GetKasKeys(), keyCache, grants.keyCache)
	}

	return grants, nil
}

type singleAttributeClause struct {
	def    *policy.Attribute
	values []AttributeValueFQN
}

type attributeBooleanExpression struct {
	must []singleAttributeClause
}

func (e attributeBooleanExpression) String() string {
	if len(e.must) == 0 {
		return "∅"
	}
	var sb strings.Builder
	for i, clause := range e.must {
		if i > 0 {
			sb.WriteString("&")
		}
		switch len(clause.values) {
		case 0:
			sb.WriteString(clause.def.GetFqn())
		case 1:
			sb.WriteString(clause.values[0].String())
		default:
			sb.WriteString(clause.def.GetFqn())
			sb.WriteString("/value/{")
			for j, v := range clause.values {
				if j > 0 {
					sb.WriteString(",")
				}
				sb.WriteString(v.Value())
			}
			sb.WriteString("}")
		}
	}
	return sb.String()
}

func (r granter) plan(defaultKas []string, genSplitID func() string) ([]keySplitStep, error) {
	b := r.constructAttributeBoolean()
	k, err := r.insertKeysForAttribute(*b)
	if err != nil {
		return nil, err
	}

	k = k.reduce()
	l := k.Len()
	if l == 0 {
		// default behavior: split key across all default kases
		switch len(defaultKas) {
		case 0:
			return nil, errors.New("no default KAS specified; required for grantless plans")
		case 1:
			return []keySplitStep{{KAS: defaultKas[0]}}, nil
		default:
			p := make([]keySplitStep, 0, len(defaultKas))
			for _, kas := range defaultKas {
				p = append(p, keySplitStep{KAS: kas, SplitID: genSplitID()})
			}
			return p, nil
		}
	}
	p := make([]keySplitStep, 0, l)
	for _, v := range k.values {
		splitID := ""
		if l > 1 {
			splitID = genSplitID()
		}
		for _, o := range v.values {
			p = append(p, keySplitStep{KAS: o.KASURI(), SplitID: splitID})
		}
	}
	return p, nil
}

func (r granter) resolveTemplate(ctx context.Context, kaoKeyAlg string, genSplitID func() string) ([]kaoTpl, error) {
	b := r.constructAttributeBoolean()
	k, err := r.assignKeysTo(*b)
	if err != nil {
		return nil, err
	}

	k = k.reduce()
	l := k.Len()
	if l == 0 {
		return []kaoTpl{}, nil
	}
	p := make([]kaoTpl, 0, l)
	for _, v := range k.values {
		splitID := ""
		if l > 1 {
			splitID = genSplitID()
		}
		for _, o := range v.values {
			if o.ID() == "" && r.keyInfoFetcher != nil {
				// No Key ID, guess what it should be.
				kpub, err := r.keyInfoFetcher.getPublicKey(ctx, o.KASURI(), kaoKeyAlg, "")
				if err != nil {
					return nil, fmt.Errorf("failed to fetch public key for resource locator [%s]: %w", o, err)
				}
				o.identifier = kpub.KID
				// Convert the string algorithm to the appropriate enum
				algEnum, err := getKasKeyAlg(strings.ToLower(kpub.Algorithm))
				if err != nil {
					return nil, fmt.Errorf("unsupported algorithm for kas [%s#%s]: %w", o.KASURI(), kpub.KID, err)
				}
				r.keyCache.c[*o] = &policy.SimpleKasKey{
					KasUri: o.KASURI(),
					PublicKey: &policy.SimpleKasPublicKey{
						Algorithm: algEnum,
						Pem:       kpub.PublicKey,
						Kid:       kpub.KID,
					},
				}
			}
			kpub, ok := r.keyCache.c[*o]
			if !ok || kpub.GetPublicKey() == nil || kpub.GetPublicKey().GetPem() == "" {
				return nil, fmt.Errorf("no key found for resource locator [%s]", o)
			}
			algorithm, err := policyAlgorithmToKeyType(kpub.GetPublicKey().GetAlgorithm())
			if err != nil {
				return nil, fmt.Errorf("invalid algorithm [%v] for kas %s with kid [%s]: %w", kpub.GetPublicKey().GetAlgorithm(), kpub.GetKasUri(), kpub.GetPublicKey().GetKid(), err)
			}
			p = append(p, kaoTpl{o.KASURI(), splitID, o.ID(), kpub.GetPublicKey().GetPem(), algorithm})
		}
	}
	return p, nil
}

func (r granter) constructAttributeBoolean() *attributeBooleanExpression {
	prefixes := make(map[string]*singleAttributeClause)
	sortedPrefixes := make([]string, 0)
	for _, aP := range r.tags {
		a := aP
		p := strings.ToLower(a.Prefix().String())
		if clause, ok := prefixes[p]; ok {
			clause.values = append(clause.values, a)
		} else if kag := r.byAttribute(a); kag != nil {
			prefixes[p] = &singleAttributeClause{kag.attr, []AttributeValueFQN{a}}
			sortedPrefixes = append(sortedPrefixes, p)
		}
	}
	must := make([]singleAttributeClause, 0, len(prefixes))[0:0]
	for _, p := range sortedPrefixes {
		must = append(must, *prefixes[p])
	}
	return &attributeBooleanExpression{must}
}

type keyClause struct {
	operator string
	values   []*ResourceLocator
}

func (e keyClause) String() string {
	if len(e.values) == 1 && e.values[0].KASURI() == "" {
		return "[" + emptyTerm + "]"
	}
	if len(e.values) == 1 {
		return "(" + e.values[0].String() + ")"
	}
	var sb strings.Builder
	sb.WriteString("(")
	op := "⋀"
	if e.operator == anyOf {
		op = "⋁"
	}
	for i, v := range e.values {
		if i > 0 {
			sb.WriteString(op)
		}
		sb.WriteString(v.String())
	}
	sb.WriteString(")")
	return sb.String()
}

type booleanKeyExpression struct {
	values []keyClause
}

func (e booleanKeyExpression) String() string {
	var sb strings.Builder
	for i, v := range e.values {
		if i > 0 {
			sb.WriteString("&")
		}
		sb.WriteString(v.String())
	}
	return sb.String()
}

func ruleToOperator(e policy.AttributeRuleTypeEnum) string {
	switch e {
	case policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ALL_OF:
		return allOf
	case policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF:
		return anyOf
	case policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_HIERARCHY:
		return hierarchy
	case policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_UNSPECIFIED:
		return unspecified
	}
	return ""
}

// Old version of plan, without KIDs in resource locators.
func (r *granter) insertKeysForAttribute(e attributeBooleanExpression) (booleanKeyExpression, error) {
	kcs := make([]keyClause, 0, len(e.must))
	for _, clause := range e.must {
		kcv := make([]*ResourceLocator, 0, len(clause.values))
		for _, term := range clause.values {
			grant := r.byAttribute(term)
			if grant == nil {
				return booleanKeyExpression{}, fmt.Errorf("no defintion or grant found for [%s]", term)
			}
			kases := grant.kases
			if len(kases) == 0 {
				kases = []string{emptyTerm}
			}
			for _, kas := range kases {
				var rl *ResourceLocator
				if kas == emptyTerm {
					// Default term, no KAS associated
					rl = &ResourceLocator{}
				} else {
					var err error
					rl, err = newResourceLocator(kas)
					if err != nil {
						r.logger.Warn("invalid KAS URL in policy service",
							slog.String("kas", kas),
							slog.String("value", term.String()),
							slog.String("error", err.Error()),
						)
						return booleanKeyExpression{}, fmt.Errorf("invalid KAS URL in policy service associated with [%s]: %w", term, err)
					}
				}
				kcv = append(kcv, rl)
			}
		}
		op := ruleToOperator(clause.def.GetRule())
		if op == unspecified {
			r.logger.Warn("unknown attribute rule type", slog.String("rule", clause.def.GetFqn()))
		}
		kc := keyClause{
			operator: op,
			values:   kcv,
		}
		kcs = append(kcs, kc)
	}
	return booleanKeyExpression{
		values: kcs,
	}, nil
}

func (r *granter) assignKeysTo(e attributeBooleanExpression) (booleanKeyExpression, error) {
	anyAssignmentsFound := false
	kcs := make([]keyClause, 0, len(e.must))
	for _, clause := range e.must {
		kcv := make([]*ResourceLocator, 0, len(clause.values))
		for _, term := range clause.values {
			mv, ok := r.mapTable[term.Key]
			if ok {
				for _, rl := range mv {
					kcv = append(kcv, rl)
					anyAssignmentsFound = true
				}
			}
		}
		op := ruleToOperator(clause.def.GetRule())
		if op == unspecified {
			r.logger.Warn("unknown attribute rule type", slog.String("rule", clause.def.GetFqn()))
		}
		kc := keyClause{
			operator: op,
			values:   kcv,
		}
		kcs = append(kcs, kc)
	}
	if !anyAssignmentsFound {
		// No assignments found, fall back to grants
		return r.insertKeysForAttribute(e)
	}

	return booleanKeyExpression{
		values: kcs,
	}, nil
}

// remove duplicates, sort a set of strings
func sortedNoDupes(l []*ResourceLocator) disjunction {
	set := map[string]bool{}
	list := make([]*ResourceLocator, 0)
	for _, e := range l {
		kas := e.String()
		if kas != "" && !set[kas] {
			set[kas] = true
			list = append(list, e)
		}
	}
	sort.Sort(disjunction(list))
	return list
}

type disjunction []*ResourceLocator

// Len, Swap and Less sort a disjunction as the reference's sort.Slice does;
// for the short lists here both run the same insertion sort.
func (l disjunction) Len() int           { return len(l) }
func (l disjunction) Swap(i, j int)      { l[i], l[j] = l[j], l[i] }
func (l disjunction) Less(i, j int) bool { return l[i].Less(l[j]) }

func (l disjunction) Equal(r disjunction) bool {
	if len(l) != len(r) {
		return false
	}
	for i, v := range l {
		if !v.Equals(*r[i]) {
			return false
		}
	}
	return true
}

func within(l []disjunction, e disjunction) bool {
	for _, v := range l {
		if e.Equal(v) {
			return true
		}
	}
	return false
}

func (e booleanKeyExpression) Len() int {
	c := 0
	for _, v := range e.values {
		c += len(v.values)
	}
	return c
}

func (e booleanKeyExpression) reduce() booleanKeyExpression {
	var conjunction []disjunction
	for _, v := range e.values {
		switch v.operator {
		case anyOf:
			terms := sortedNoDupes(v.values)
			if len(terms) > 0 && !within(conjunction, terms) {
				conjunction = append(conjunction, terms)
			}
		default:
			for _, k := range v.values {
				if k.KASURI() == "" {
					continue
				}
				terms := []*ResourceLocator{k}
				if !within(conjunction, terms) {
					conjunction = append(conjunction, terms)
				}
			}
		}
	}
	if len(conjunction) == 0 {
		return booleanKeyExpression{}
	}
	values := make([]keyClause, len(conjunction))
	for i, d := range conjunction {
		pki := make([]*ResourceLocator, len(d))
		copy(pki, d)
		values[i] = keyClause{
			operator: anyOf,
			values:   pki,
		}
	}
	return booleanKeyExpression{values}
}
