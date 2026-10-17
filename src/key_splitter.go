package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/std/encoding/json"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/io"
	"opentdf-local/sdk/src/protocol/policy"
)

// KeySplitter converts attribute values plus a DEK into one or more
// key splits, each addressed to one or more KAS servers. Injected on
// the chunked Writer so tests can substitute an identity splitter
// without touching real attribute grants.
//
// Implementations must be safe for concurrent use.
//
// Experimental: not part of the stable SDK API; may change or be removed.
type KeySplitter interface {
	// Split evaluates the ABAC policy expressed by attrs, produces N
	// splits of dek per the resulting boolean expression, and returns
	// each split alongside the KAS public keys it must be wrapped to.
	//
	// Split must not retain or modify attrs or dek, nor the elements of
	// attrs.
	Split(ctx context.Context, attrs []*policy.Value, dek []byte, defaultKAS *policy.SimpleKasKey) (*SplitResult, error)
}

// Split is one XOR share of the DEK bound to one or more KAS
// servers.
//
// Experimental: not part of the stable SDK API; may change or be removed.
type Split struct {
	// Data is this share of the DEK: the value that, XOR'd with every
	// other share in the result, reproduces the DEK. With a single
	// split -- what DefaultKeySplitter always produces -- there is
	// nothing to XOR against, so it is the DEK verbatim.
	Data []byte

	// ID uniquely identifies this split within a SplitResult. It is
	// what the reader groups shares on, so it is required once there
	// is more than one split and must be distinct across them. With a
	// single split there is nothing to distinguish, and it is
	// conventionally left empty.
	ID string

	// KASURLs lists every KAS that can unwrap this split. Multiple
	// URLs mean any one KAS is sufficient (OR semantics).
	KASURLs []string
}

// SplitResult is what KeySplitter.Split returns: the shares plus the
// KAS wrapping keys needed to encrypt each share into a KeyAccess
// object.
//
// Experimental: not part of the stable SDK API; may change or be removed.
type SplitResult struct {
	// KASPublicKeys maps KAS URL to the wrapping key to use for that
	// URL. Populated for every URL referenced by any split.
	KASPublicKeys map[string]KASPublicKey

	// Splits are the DEK shares in emission order.
	Splits []Split
}

// Validate reports whether the result can be turned into a key access
// array that a reader can actually reconstruct the DEK from. The
// chunked writer calls it before wrapping anything, and third-party
// KeySplitter implementations can call it as a self-check.
//
// Validate sees only the shares, so it is half the contract: it can
// confirm they agree with each other, but not that they agree with the
// key. VerifyReconstruction is the other half.
//
// Experimental: not part of the stable SDK API; may change or be removed.
func (r *SplitResult) Validate() error {
	if r == nil || len(r.Splits) == 0 {
		return errors.New("chunked: splitter returned no splits")
	}

	ids := make(map[string]struct{}, len(r.Splits))
	shareLen := len(r.Splits[0].Data)
	for i, split := range r.Splits {
		// A share is one XOR operand. A short or empty one silently
		// leaves part of the DEK unmasked, since the reader XORs only
		// as many bytes as the share carries.
		if len(split.Data) == 0 {
			return fmt.Errorf("chunked: split %d (id %q) has no share data", i, split.ID)
		}
		if len(split.Data) != shareLen {
			return fmt.Errorf("chunked: split %d (id %q) has a %d-byte share; split 0 has %d and all shares must agree",
				i, split.ID, len(split.Data), shareLen)
		}

		// A single split carries no sid, because there is nothing to
		// distinguish it from. With more than one, sid is what the
		// reader groups on: an empty or duplicated id collapses two
		// shares into one group, and the second is never XOR'd in.
		if len(r.Splits) > 1 && split.ID == "" {
			return fmt.Errorf("chunked: split %d has an empty id; ids are required when there is more than one split", i)
		}
		if _, dup := ids[split.ID]; dup {
			return fmt.Errorf("chunked: split id %q is used by more than one split", split.ID)
		}
		ids[split.ID] = struct{}{}

		if len(split.KASURLs) == 0 {
			return fmt.Errorf("chunked: split %d (id %q) names no KAS; its share could never be unwrapped", i, split.ID)
		}
		for _, url := range split.KASURLs {
			pk, ok := r.KASPublicKeys[url]
			if !ok {
				// The same sentinel the mainline writer uses for an
				// unresolved KAS, so callers can match one error whichever
				// path produced it.
				return fmt.Errorf("chunked: splitID:[%s], kas:[%s]: no entry in KASPublicKeys: %w", split.ID, url, errKasPubKeyMissing)
			}
			if err := pk.validate(url); err != nil {
				return err
			}
		}
	}
	return nil
}

// VerifyReconstruction reports whether the shares actually rebuild
// dek. It is the half of the splitter's contract Validate cannot
// check: Validate sees only the shares, so it can confirm they agree
// with each other but not that they agree with the key.
//
// With a single split there is nothing to XOR against, so the share
// must equal dek verbatim; the loop below expresses that as the
// degenerate case rather than special-casing it.
//
// Experimental: not part of the stable SDK API; may change or be removed.
func (r *SplitResult) VerifyReconstruction(dek []byte) error {
	if r == nil || len(r.Splits) == 0 {
		return errors.New("chunked: splitter returned no splits")
	}
	if len(dek) == 0 {
		return errors.New("chunked: cannot verify split reconstruction against an empty DEK")
	}

	reconstructed := make([]byte, len(dek))
	for i, split := range r.Splits {
		// Length is checked against the DEK, not against Splits[0] as Validate
		// does: shares that agree with each other but are shorter than the key
		// leave its tail unmasked, and the reader XORs only as many bytes as
		// each share carries.
		if len(split.Data) != len(dek) {
			return fmt.Errorf("chunked: split %d (id %q) has a %d-byte share; every share must be %d bytes, the length of the DEK",
				i, split.ID, len(split.Data), len(dek))
		}
		for j, b := range split.Data {
			reconstructed[j] ^= b
		}
	}

	// Constant-time, and deliberately silent about where the mismatch is.
	if !constantTimeEqual(reconstructed, dek) {
		return errors.New("chunked: the splits do not XOR back to the DEK; a reader that collected every share would reconstruct the wrong key and report a root signature failure")
	}
	return nil
}

// constantTimeEqual is subtle.ConstantTimeCompare for equal-length inputs.
func constantTimeEqual(a, b []byte) bool {
	if len(a) != len(b) {
		return false
	}
	var v byte
	for i := range a {
		v |= a[i] ^ b[i]
	}
	return v == 0
}

// KASPublicKey is the wrapping key resolved for one KAS URL.
//
// Experimental: not part of the stable SDK API; may change or be removed.
type KASPublicKey struct {
	// Algorithm identifies the wrapping scheme. It must be one of the
	// key types the SDK parses, e.g. RSA2048Key or EC256Key -- use
	// PolicyAlgorithmToKeyType to derive it from a policy.Algorithm.
	Algorithm KeyType

	// KID identifies which key at that KAS to use.
	KID string

	// PEM is the wrapping key in PEM form.
	PEM string

	// URL of the KAS.
	URL string
}

// validate checks one resolved wrapping key. mapKey is the
// KASPublicKeys key it was found under, which is reported in errors
// and must match URL: the KAO is built from URL, so the two
// disagreeing means the split names one KAS and the manifest points a
// reader at another.
func (k KASPublicKey) validate(mapKey string) error {
	if k.PEM == "" {
		return fmt.Errorf("chunked: kas:[%s]: %w", mapKey, errKasPubKeyMissing)
	}
	if k.URL == "" {
		return fmt.Errorf("chunked: kas %q has an empty URL field; every key access object built from it would name no endpoint", mapKey)
	}
	if k.URL != mapKey {
		return fmt.Errorf("chunked: kas keyed under %q carries URL %q; key access objects are built from the URL field", mapKey, k.URL)
	}
	if _, err := parseKeyType(string(k.Algorithm)); err != nil {
		return fmt.Errorf("chunked: kas %q: %w", mapKey, err)
	}
	return nil
}

// ErrSplitterRequiresDefaultKAS is returned by the default key
// splitter when no default KAS was supplied. The default splitter is
// single-KAS only; multi-attribute splits require injecting a full
// splitter via WithChunkedKeySplitter.
var ErrSplitterRequiresDefaultKAS = errors.New("chunked: default splitter requires a default KAS; supply WithChunkedDefaultKAS or WithChunkedKeySplitter")

// ErrSplitterUnsupportedAlgorithm is returned by the default key
// splitter when the default KAS advertises a key algorithm this SDK
// has no wrapping scheme for.
var ErrSplitterUnsupportedAlgorithm = errors.New("chunked: unsupported KAS key algorithm")

// defaultKeySplitter is the reference's DefaultKeySplitter: a single-KAS
// single-split splitter. Attributes are ignored; the entire DEK is bound to
// the caller's default KAS. It is exported with the chunked writer that
// takes it (Phase 10): a KeySplitter interface does not cross the Goalchemy
// boundary.
func defaultKeySplitter() KeySplitter { return &singleKASSplitter{} }

// singleKASSplitter binds the full DEK to a single KAS. Attributes
// are ignored; splitting into multi-KAS OR-of-AND shares is beyond
// this default's scope.
type singleKASSplitter struct{}

// Split returns one split covering the full DEK, addressed to
// defaultKAS. Errors when defaultKAS is nil, has no public key or
// URI, or names an algorithm this SDK cannot wrap for.
func (s *singleKASSplitter) Split(_ context.Context, _ []*policy.Value, dek []byte, defaultKAS *policy.SimpleKasKey) (*SplitResult, error) {
	if defaultKAS == nil || defaultKAS.GetPublicKey() == nil || defaultKAS.GetPublicKey().GetPem() == "" {
		return nil, ErrSplitterRequiresDefaultKAS
	}
	url := defaultKAS.GetKasUri()
	if url == "" {
		// An empty URI would still produce a PEM-valid split, but the
		// resulting KeyAccess.KasURL leaves a reader with no endpoint to
		// send a rewrap request to.
		return nil, fmt.Errorf("%w: kas uri is empty", ErrSplitterRequiresDefaultKAS)
	}

	// Reject an unmappable algorithm here rather than letting the empty
	// string formatAlg returns on failure reach createKeyAccess, which
	// would wrap with RSA and produce a TDF nothing can decrypt.
	alg, err := policyAlgorithmToKeyType(defaultKAS.GetPublicKey().GetAlgorithm())
	if err != nil {
		return nil, fmt.Errorf("%w: kas %s: %w", ErrSplitterUnsupportedAlgorithm, url, err)
	}

	share := make([]byte, len(dek))
	copy(share, dek)
	return &SplitResult{
		KASPublicKeys: map[string]KASPublicKey{
			url: {
				Algorithm: alg,
				KID:       defaultKAS.GetPublicKey().GetKid(),
				PEM:       defaultKAS.GetPublicKey().GetPem(),
				URL:       url,
			},
		},
		Splits: []Split{{
			Data:    share,
			KASURLs: []string{url},
		}},
	}, nil
}

// splitShare is one XOR share of the DEK together with every KAS able
// to unwrap it. Several KAS entries on one share mean any of them
// suffices (OR semantics); several shares mean all are required (AND).
type splitShare struct {
	// id names the share in the manifest ("sid"). Empty when the TDF
	// has a single share.
	id string

	// data is the share itself.
	data []byte

	// kases are the wrapping targets for this share.
	kases []KASInfo
}

// resolvePolicyAndKeyAccess builds the policy document the DEK is bound to and wraps
// every share to its KAS targets, returning the two manifest fields that bind a DEK to
// policy.
func resolvePolicyAndKeyAccess(fqns []string, shares []splitShare, metadata string) (string, []KeyAccess, error) {
	policyObj, err := createPolicyObjectFromFQNs(fqns)
	if err != nil {
		return "", nil, fmt.Errorf("fail to create policy object:%w", err)
	}
	policyObjectAsStr, err := json.Marshal(policyObj)
	if err != nil {
		return "", nil, fmt.Errorf("json.Marshal failed:%w", err)
	}
	base64Policy, err := encoding.Base64Encode(policyObjectAsStr)
	if err != nil {
		return "", nil, err
	}

	kaos, err := buildKeyAccessObjects(shares, base64Policy, metadata)
	if err != nil {
		return "", nil, err
	}
	return base64Policy, kaos, nil
}

// buildKeyAccessObjects wraps every share to each of its KAS targets,
// emitting the manifest's keyAccess array in share order.
func buildKeyAccessObjects(shares []splitShare, base64Policy, metadata string) ([]KeyAccess, error) {
	var kaos []KeyAccess
	for _, share := range shares {
		// A share with no KAS target contributes no key access object,
		// so nothing in the manifest ever hands its bytes back.
		if len(share.kases) == 0 {
			return nil, fmt.Errorf("splitID:[%s]: share names no KAS; its bytes could never be unwrapped", share.id)
		}

		// Policy binding and metadata are keyed on the split share, not
		// on the KAS, so compute them once per share rather than once
		// per KAS URL in an OR-group.
		policyBinding, err := createPolicyBinding(share.data, base64Policy)
		if err != nil {
			return nil, err
		}

		var encryptedMetadata string
		if metadata != "" {
			encryptedMetadata, err = encryptMetadata(share.data, metadata)
			if err != nil {
				return nil, err
			}
		}

		for _, kasInfo := range share.kases {
			if kasInfo.PublicKey == "" {
				return nil, fmt.Errorf("splitID:[%s], kas:[%s]: %w", share.id, kasInfo.URL, errKasPubKeyMissing)
			}
			keyAccess, err := createKeyAccess(kasInfo, share.data, policyBinding, encryptedMetadata, share.id)
			if err != nil {
				return nil, err
			}
			kaos = append(kaos, keyAccess)
		}
	}
	if len(kaos) == 0 {
		return nil, errors.New("no key access objects generated")
	}
	return kaos, nil
}

// splitDEK returns count XOR shares of dek. Every share but the last
// is random; the last absorbs the parity so the shares XOR back to dek.
func splitDEK(dek []byte, count int, rand io.Reader) ([][]byte, error) {
	if count <= 0 {
		return nil, errors.New("no key splits requested")
	}
	shares := make([][]byte, count)
	parity := make([]byte, len(dek))
	copy(parity, dek)
	for i := 0; i < count-1; i++ {
		share := make([]byte, len(dek))
		if _, err := io.ReadFull(rand, share); err != nil {
			return nil, fmt.Errorf("generate key split failed: %w", err)
		}
		for j, b := range share {
			parity[j] ^= b
		}
		shares[i] = share
	}
	shares[count-1] = parity
	return shares, nil
}

// randReader is crypto/rand.Reader over the host's random source.
type randReader struct{}

func (randReader) Read(p []byte) (int, error) {
	b, err := crypto.Random(len(p))
	if err != nil {
		return 0, err
	}
	return copy(p, b), nil
}
