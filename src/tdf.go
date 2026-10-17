package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/std/encoding/hex"
	"github.com/eugenioenko/goalchemy/std/encoding/json"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/io"
	"github.com/eugenioenko/goalchemy/std/log/slog"
	"github.com/eugenioenko/goalchemy/std/sort"
	"github.com/eugenioenko/goalchemy/std/strconv"
	"github.com/eugenioenko/goalchemy/std/strings"
	"opentdf-local/sdk/src/protocol/kas"
	"opentdf-local/sdk/src/protocol/policy/kasregistry"
	"opentdf-local/sdk/src/tdf"
)

const (
	keyAccessSchemaVersion = "1.0"
	defaultMimeType        = "application/octet-stream"
	tdfAsZip               = "zip"
	gcmIvSize              = 12
	aesBlockSize           = 16
	hmacIntegrityAlgorithm = "HS256"
	gmacIntegrityAlgorithm = "GMAC"
	tdfZipReference        = "reference"
	kKeySize               = 32
	kWrapped               = "wrapped"
	kECWrapped             = "ec-wrapped"
	kHybridWrapped         = "hybrid-wrapped"
	kMLKEMWrapped          = "mlkem-wrapped"
	kKasProtocol           = "kas"
	kSplitKeyType          = "split"
	kGCMCipherAlgorithm    = "AES-256-GCM"
	kGMACPayloadLength     = 16
	hexSemverThreshold     = "4.3.0"

	// maxPayloadSegments caps the segment count a declared input size may imply.
	maxPayloadSegments = 2147483647

	// maxAESGCMSegmentsPerDEK is the most payload segment encryptions a writer
	// attempts under one DEK before failing with ErrAESGCMExhausted.
	maxAESGCMSegmentsPerDEK uint64 = 1<<32 - 1
)

// Loads and reads ZTDF files
type Reader struct {
	rewrapper           Rewrapper
	logger              *slog.Logger
	ctx                 context.Context
	manifest            Manifest
	unencryptedMetadata []byte
	payload             []byte
	cursor              int64
	payloadSize         int64
	payloadKey          []byte
	kasSessionKey       *crypto.Key
	kasSessionKeyType   KeyType
	ownsSessionKey      bool
	config              TDFReaderConfig
	requiredObligations *RequiredObligations
}

type RequiredObligations struct {
	FQNs []string
}

type TDFObject struct {
	manifest Manifest
	size     int64
}

func (t *TDFObject) Size() int64 {
	return t.size
}

func (t *TDFObject) Manifest() Manifest {
	return t.manifest
}

// CreateTDF encrypts the payload read from reader and writes a TDF to writer.
// This phase reads the payload whole and runs EncryptBytes; the streaming
// writer of Phase 7 replaces the byte path behind the same signature.
func (s *SDK) CreateTDF(ctx context.Context, writer io.Writer, reader io.Reader, opts TDFOptions) (*TDFObject, error) {
	plaintext, err := io.ReadAll(reader)
	if err != nil {
		return nil, wrapError(KindInvalidInput, "CreateTDF", "io.Reader.Read failed", err)
	}
	data, obj, err := s.EncryptBytes(ctx, plaintext, opts)
	if err != nil {
		return nil, err
	}
	n, err := writer.Write(data)
	if err != nil {
		return nil, wrapError(KindInvalidInput, "CreateTDF", "io.writer.Write failed", err)
	}
	if n != len(data) {
		return nil, sentinelError(KindInvalidInput, errWriteFailed)
	}
	return obj, nil
}

func (s *SDK) defaultKases(c *TDFConfig) []string {
	allk := make([]string, 0, len(c.kasInfoList))
	defk := make([]string, 0)
	for _, k := range c.kasInfoList {
		if k.Default {
			defk = append(defk, k.URL)
		} else if len(defk) == 0 {
			allk = append(allk, k.URL)
		}
	}
	if len(defk) == 0 {
		return allk
	}
	return defk
}

// uuidSplitIDGenerator returns a random UUID (version 4).
func uuidSplitIDGenerator() string {
	random, err := crypto.Random(16)
	if err != nil {
		return ""
	}
	random[6] = (random[6] & 15) | 64
	random[8] = (random[8] & 63) | 128
	h := hex.EncodeToString(random)
	return h[:8] + "-" + h[8:12] + "-" + h[12:16] + "-" + h[16:20] + "-" + h[20:]
}

// EncryptBytes encrypts plaintext into a TDF held in memory, subject to the
// given options: the reference's CreateTDFContext over bytes.
func (s *SDK) EncryptBytes(ctx context.Context, plaintext []byte, opts TDFOptions) ([]byte, *TDFObject, error) {
	if err := s.check("CreateTDF"); err != nil {
		return nil, nil, err
	}
	tdfConfig, err := newTDFConfig(opts)
	if err != nil {
		return nil, nil, wrapError(KindInvalidInput, "CreateTDF", "NewTDFConfig failed", err)
	}
	return s.encryptBytes(ctx, plaintext, tdfConfig)
}

func (s *SDK) encryptBytes(ctx context.Context, plaintext []byte, tdfConfig *TDFConfig) ([]byte, *TDFObject, error) {
	err := tdfConfig.initKAOTemplate(ctx, s)
	if err != nil {
		return nil, nil, wrapError(KindKASRequest, "CreateTDF", "", err)
	}

	segmentSize := tdfConfig.defaultSegmentSize
	if segmentSize > maxSegmentSize {
		return nil, nil, newError(KindInvalidInput, "CreateTDF", fmt.Sprintf("segment size too large: %d", segmentSize))
	} else if segmentSize < minSegmentSize {
		return nil, nil, newError(KindInvalidInput, "CreateTDF", fmt.Sprintf("segment size too small: %d", segmentSize))
	}

	// A declared length is exact: the input is bounded to it and a shorter
	// input fails rather than producing a TDF that is silently short.
	if tdfConfig.inputSize != inputSizeUnknown {
		if int64(len(plaintext)) < tdfConfig.inputSize {
			return nil, nil, wrapError(KindInvalidInput, "CreateTDF", "", fmt.Errorf("%w: read %d of %d bytes", errInputShorterThanDeclared, len(plaintext), tdfConfig.inputSize))
		}
		plaintext = plaintext[:tdfConfig.inputSize]
	}
	if _, err := segmentCount(int64(len(plaintext)), segmentSize); err != nil {
		return nil, nil, wrapError(KindInvalidInput, "CreateTDF", "", err)
	}

	w, err := s.newTDFWriter(ctx, tdfConfig)
	if err != nil {
		return nil, nil, wrapError(KindKASRequest, "CreateTDF", "", err)
	}

	for index := 0; ; index++ {
		end := int64(index+1) * segmentSize
		if end > int64(len(plaintext)) {
			end = int64(len(plaintext))
		}
		start := int64(index) * segmentSize
		// An empty payload still gets one empty segment.
		if start >= end && index > 0 {
			break
		}
		if err := w.writeSegment(plaintext[start:end]); err != nil {
			return nil, nil, wrapError(KindInvalidInput, "CreateTDF", "", err)
		}
		if end == int64(len(plaintext)) {
			break
		}
	}

	data, manifest, err := w.finalize(tdfConfig.mimeType)
	if err != nil {
		return nil, nil, wrapError(KindInvalidInput, "CreateTDF", "", err)
	}
	return data, &TDFObject{manifest: manifest, size: int64(len(data))}, nil
}

// tdfWriter encrypts a payload segment by segment under one DEK and builds
// the manifest. This phase collects the ciphertext in memory; Phase 7 feeds
// the segments to the streaming archive writer instead.
type tdfWriter struct {
	dek           []byte
	segmentSize   int64
	policy        string
	keyAccess     []KeyAccess
	payload       []byte
	segments      []Segment
	aggregateHash []byte
	encryptions   uint64
}

// newTDFWriter resolves key access before any payload byte is encrypted, so
// that an unreachable KAS fails the call without a partial TDF.
func (s *SDK) newTDFWriter(ctx context.Context, tdfConfig *TDFConfig) (*tdfWriter, error) {
	dek, err := crypto.Random(kKeySize)
	if err != nil {
		return nil, fmt.Errorf("fail to create a new split key: %w", err)
	}

	base64Policy, kaos, err := s.resolveKeyAccess(ctx, tdfConfig, dek)
	if err != nil {
		return nil, fmt.Errorf("fail to create a new split key: %w", err)
	}

	return &tdfWriter{dek: dek, segmentSize: tdfConfig.defaultSegmentSize, policy: base64Policy, keyAccess: kaos}, nil
}

func (w *tdfWriter) writeSegment(data []byte) error {
	if w.encryptions >= maxAESGCMSegmentsPerDEK {
		return ErrAESGCMExhausted
	}
	w.encryptions++
	nonce, err := crypto.Random(gcmIvSize)
	if err != nil {
		return err
	}
	cipherText, err := crypto.AES256GCMEncrypt(w.dek, nonce, data, nil)
	if err != nil {
		return fmt.Errorf("ocrypto.AesGcm.encrypt failed:%w", err)
	}
	segment := append(nonce, cipherText...)
	payloadSig, err := segmentIntegrity(segment, w.dek, SegmentGMAC, false)
	if err != nil {
		return fmt.Errorf("splitKey.GetSignaturefailed: %w", err)
	}
	hash, err := encoding.Base64Encode([]byte(payloadSig))
	if err != nil {
		return err
	}
	w.segments = append(w.segments, Segment{Hash: hash, Size: int64(len(data)), EncryptedSize: int64(len(segment))})
	w.aggregateHash = append(w.aggregateHash, payloadSig...)
	w.payload = append(w.payload, segment...)
	return nil
}

func (w *tdfWriter) finalize(mimeType string) ([]byte, Manifest, error) {
	rootSig, err := rootIntegrity(w.aggregateHash, w.dek, RootHS256, false)
	if err != nil {
		return nil, Manifest{}, fmt.Errorf("splitkey.getSignature failed:%w", err)
	}
	sig, err := encoding.Base64Encode([]byte(rootSig))
	if err != nil {
		return nil, Manifest{}, err
	}
	if mimeType == "" {
		mimeType = defaultMimeType
	}
	manifest := Manifest{
		EncryptionInformation: EncryptionInformation{
			KeyAccessType: kSplitKeyType,
			Policy:        w.policy,
			KeyAccessObjs: w.keyAccess,
			Method:        Method{Algorithm: kGCMCipherAlgorithm, IsStreamable: true},
			IntegrityInformation: IntegrityInformation{
				RootSignature:           RootSignature{Algorithm: hmacIntegrityAlgorithm, Signature: sig},
				SegmentHashAlgorithm:    gmacIntegrityAlgorithm,
				DefaultSegmentSize:      w.segmentSize,
				DefaultEncryptedSegSize: w.segmentSize + gcmIvSize + aesBlockSize,
				Segments:                w.segments,
			},
		},
		Payload:    Payload{Type: tdfZipReference, URL: tdf.PayloadEntry, Protocol: tdfAsZip, MimeType: mimeType, IsEncrypted: true},
		TDFVersion: TDFSpecVersion,
	}
	manifestJSON, err := json.Marshal(manifest)
	if err != nil {
		return nil, Manifest{}, fmt.Errorf("json.Marshal failed: %w", err)
	}
	data, err := tdf.WriteArchive(w.payload, manifestJSON, tdf.DefaultArchiveLimits())
	if err != nil {
		return nil, Manifest{}, fmt.Errorf("archive write failed: %w", err)
	}
	return data, manifest, nil
}

// segmentCount returns the number of segments a payload of inputSize bytes occupies,
// and refuses a size that would need more segments than maxPayloadSegments.
func segmentCount(inputSize, segmentSize int64) (int, error) {
	if inputSize == 0 {
		// An empty payload still gets one empty segment.
		return 1, nil
	}
	count := inputSize / segmentSize
	if inputSize%segmentSize != 0 {
		count++
	}
	if count > maxPayloadSegments {
		return 0, fmt.Errorf("%w: %d bytes in %d-byte segments needs %d segments, limit is %d",
			errTooManySegments, inputSize, segmentSize, count, maxPayloadSegments)
	}
	return int(count), nil
}

// initKAOTemplate initializes the KAO template, from either the split plan, kaoTemplate, or autoconfigure based on tags.
func (tdfConfig *TDFConfig) initKAOTemplate(ctx context.Context, s *SDK) error {
	// At most one of the following should be true:
	// - autoconfigure is true
	// - splitPlan is set
	// - kaoTemplate is set
	if len(tdfConfig.splitPlan) > 0 && len(tdfConfig.kaoTemplate) > 0 {
		return errors.New("cannot set both splitPlan and kaoTemplate")
	}
	if tdfConfig.autoconfigure && (len(tdfConfig.splitPlan) > 0 || len(tdfConfig.kaoTemplate) > 0) {
		return errors.New("cannot set autoconfigure and splitPlan or kaoTemplate")
	}

	// * Get base key before autoconfigure to condition off of.
	if tdfConfig.autoconfigure {
		g, err := s.newGranter(ctx, tdfConfig)
		if err != nil {
			return err
		}

		switch g.typ {
		case mappedFound:
			tdfConfig.kaoTemplate, err = g.resolveTemplate(ctx, string(tdfConfig.preferredKeyWrapAlg), uuidSplitIDGenerator)
		case grantsFound:
			tdfConfig.kaoTemplate = nil
			tdfConfig.splitPlan, err = g.plan(make([]string, 0), uuidSplitIDGenerator)
		case noKeysFound:
			var baseKey SimpleKasKey
			baseKey, err = s.GetBaseKey(ctx)
			if err == nil {
				err = populateKasInfoFromBaseKey(baseKey, tdfConfig, s.cfg.logger)
			} else {
				s.cfg.logger.Debug("cannot getting base key, falling back to default kas", slog.String("error", err.Error()))
				dk := s.defaultKases(tdfConfig)
				tdfConfig.kaoTemplate = nil
				tdfConfig.splitPlan, err = g.plan(dk, uuidSplitIDGenerator)
			}
		}
		if err != nil {
			return fmt.Errorf("failed generate plan: %w", err)
		}
	}

	switch {
	case len(tdfConfig.kaoTemplate) > 0:
		// use the kao template to create the key access objects
		// This is the preferred behavior; the following options upgrade deprecated behaviors
	case len(tdfConfig.splitPlan) > 0:
		// Seed anything passed in manually
		latestKASInfo := make(map[string]KASInfo)
		for _, kasInfo := range tdfConfig.kasInfoList {
			if kasInfo.PublicKey != "" {
				latestKASInfo[kasInfo.URL] = kasInfo
			}
		}
		// upgrade split plan to kao template
		tdfConfig.kaoTemplate = make([]kaoTpl, len(tdfConfig.splitPlan))
		for i, splitInfo := range tdfConfig.splitPlan {
			kasInfo, ok := latestKASInfo[splitInfo.KAS]
			if !ok {
				k, err := s.keyFetcher.getPublicKey(ctx, splitInfo.KAS, string(tdfConfig.preferredKeyWrapAlg), "")
				if err != nil {
					return fmt.Errorf("unable to retrieve public key from KAS at [%s]: %w", splitInfo.KAS, err)
				}
				kasInfo = *k
			}
			if kasInfo.PublicKey == "" {
				return fmt.Errorf("empty KAS key found for splitID:[%s], kas:[%s]: %w", splitInfo.SplitID, splitInfo.KAS, errKasPubKeyMissing)
			}
			tdfConfig.kaoTemplate[i] = kaoTpl{
				splitInfo.KAS,
				splitInfo.SplitID,
				kasInfo.KID,
				kasInfo.PublicKey,
				KeyType(kasInfo.Algorithm),
			}
		}
		tdfConfig.splitPlan = nil // clear split plan as we are using kaoTemplate now
	case len(tdfConfig.kasInfoList) > 0:
		// Default to split based on kasInfoList
		// To remove. This has been deprecated for some time.
		tdfConfig.kaoTemplate = createKaoTemplateFromKasInfo(tdfConfig.kasInfoList)
	}

	return nil
}

func (s *SDK) newGranter(ctx context.Context, tdfConfig *TDFConfig) (granter, error) {
	var g granter
	var err error
	if len(tdfConfig.attributeValues) > 0 {
		g, err = newGranterFromAttributes(s.cfg.logger, s.kasKeyCache, tdfConfig.attributeValues...)
	} else if len(tdfConfig.attributes) > 0 {
		g, err = newGranterFromService(ctx, s.cfg.logger, s.kasKeyCache, s.attributes, tdfConfig.attributes...)
	}
	if err != nil {
		return g, err
	}
	g.keyInfoFetcher = s.keyFetcher
	return g, nil
}

func (r *Reader) Manifest() Manifest {
	return r.manifest
}

// resolveKeyAccess builds the two manifest fields that bind the DEK to policy:
// the base64-encoded policy object and the key access objects.
func (s *SDK) resolveKeyAccess(ctx context.Context, tdfConfig *TDFConfig, dek []byte) (string, []KeyAccess, error) {
	shares, err := s.templateSplitShares(ctx, tdfConfig, dek)
	if err != nil {
		return "", nil, err
	}

	fqns := make([]string, 0, len(tdfConfig.attributes))
	for _, attribute := range tdfConfig.attributes {
		fqns = append(fqns, attribute.String())
	}
	return resolvePolicyAndKeyAccess(fqns, shares, tdfConfig.metaData)
}

// templateSplitShares groups the KAO template by split ID and divides the DEK across
// the resulting shares. KAS entries sharing a split ID form an OR-group: any one of
// them can unwrap that share. Public keys absent from the template are fetched here.
func (s *SDK) templateSplitShares(ctx context.Context, tdfConfig *TDFConfig, dek []byte) ([]splitShare, error) {
	if len(tdfConfig.kaoTemplate) == 0 {
		return nil, fmt.Errorf("no key access template specified or inferred in initKAOTemplate: %w", errInvalidKasInfo)
	}

	var shares []splitShare
	index := map[string]int{}
	for _, tpl := range tdfConfig.kaoTemplate {
		ki := KASInfo{
			URL:       tpl.KAS,
			KID:       tpl.kid,
			PublicKey: tpl.pem,
			Algorithm: string(tpl.algorithm),
		}
		if ki.PublicKey == "" {
			a := ki.Algorithm
			if a == "" {
				a = string(tdfConfig.preferredKeyWrapAlg)
			}
			k, err := s.keyFetcher.getPublicKey(ctx, tpl.KAS, a, tpl.kid)
			if err != nil {
				return nil, fmt.Errorf("unable to retrieve public key from KAS at [%s]: %w", tpl.KAS, err)
			}
			ki = *k
		}
		if i, ok := index[tpl.SplitID]; ok {
			shares[i].kases = append(shares[i].kases, ki)
			continue
		}
		index[tpl.SplitID] = len(shares)
		shares = append(shares, splitShare{id: tpl.SplitID, kases: []KASInfo{ki}})
	}

	// The shares XOR back to the DEK, so every one of them is needed to reconstruct
	// it — that is the AND half of the split semantics, with the OR half expressed by
	// the several KAS servers that may sit on a single share.
	data, err := splitDEK(dek, len(shares), randReader{})
	if err != nil {
		return nil, err
	}
	for i := range shares {
		shares[i].data = data[i]
	}
	return shares, nil
}

// manifestSegmentIntegrityAlg maps the manifest's segmentHashAlg string onto
// the algorithm used to verify each segment.
func manifestSegmentIntegrityAlg(alg string) SegmentIntegrityAlg {
	if strings.EqualFold(gmacIntegrityAlgorithm, alg) {
		return SegmentGMAC
	}
	return SegmentHS256
}

// createPolicyBinding binds a key split to the policy it unlocks, keyed on the split
// itself so that a KAS can verify the policy it is asked to enforce is the one the
// creator bound.
func createPolicyBinding(symKey []byte, base64Policy string) (PolicyBinding, error) {
	mac, err := crypto.HMACSHA256(symKey, []byte(base64Policy))
	if err != nil {
		return PolicyBinding{}, err
	}
	policyBindingHash := hex.EncodeToString(mac)
	hash, err := encoding.Base64Encode([]byte(policyBindingHash))
	if err != nil {
		return PolicyBinding{}, err
	}
	return PolicyBinding{
		Alg:  hmacIntegrityAlgorithm,
		Hash: hash,
	}, nil
}

func encryptMetadata(symKey []byte, metaData string) (string, error) {
	nonce, err := crypto.Random(gcmIvSize)
	if err != nil {
		return "", err
	}
	cipherText, err := crypto.AES256GCMEncrypt(symKey, nonce, []byte(metaData), nil)
	if err != nil {
		return "", fmt.Errorf("ocrypto.AesGcm.encrypt failed:%w", err)
	}
	emb := append(nonce, cipherText...)

	cipher, err := encoding.Base64Encode(emb)
	if err != nil {
		return "", err
	}
	iv, err := encoding.Base64Encode(nonce)
	if err != nil {
		return "", err
	}
	metadata := EncryptedMetadata{
		Cipher: cipher,
		Iv:     iv,
	}

	metadataJSON, err := json.Marshal(metadata)
	if err != nil {
		return "", fmt.Errorf(" json.Marshal failed:%w", err)
	}
	return encoding.Base64Encode(metadataJSON)
}

func createKeyAccess(kasInfo KASInfo, symKey []byte, policyBinding PolicyBinding, encryptedMetadata, splitID string) (KeyAccess, error) {
	keyAccess := KeyAccess{
		KeyType:           kWrapped,
		KasURL:            kasInfo.URL,
		KID:               kasInfo.KID,
		Protocol:          kKasProtocol,
		PolicyBinding:     policyBinding,
		EncryptedMetadata: encryptedMetadata,
		SplitID:           splitID,
		SchemaVersion:     keyAccessSchemaVersion,
	}

	ktype := KeyType(kasInfo.Algorithm)
	switch {
	case isPQKeyType(ktype):
		return KeyAccess{}, notPlannedPQ("CreateTDF", "wrapping key algorithm "+string(ktype))
	case IsECKeyType(ktype):
		wrappedKeyInfo, err := generateWrapKeyWithEC(ktype, kasInfo.PublicKey, symKey)
		if err != nil {
			return KeyAccess{}, err
		}
		keyAccess.KeyType = kECWrapped
		keyAccess.WrappedKey = wrappedKeyInfo.wrappedKey
		keyAccess.EphemeralPublicKey = wrappedKeyInfo.publicKey
	default:
		wrappedKey, err := generateWrapKeyWithRSA(kasInfo.PublicKey, symKey)
		if err != nil {
			return KeyAccess{}, err
		}
		keyAccess.WrappedKey = wrappedKey
	}

	return keyAccess, nil
}

func tdfSalt() ([]byte, error) {
	return crypto.SHA256([]byte("TDF"))
}

type ecKeyWrappedKeyInfo struct {
	publicKey  string
	wrappedKey string
}

func generateWrapKeyWithEC(ktype KeyType, kasPublicKey string, symKey []byte) (ecKeyWrappedKeyInfo, error) {
	ecKeyPair, err := newKeyPair(ktype)
	if err != nil {
		return ecKeyWrappedKeyInfo{}, fmt.Errorf("ocrypto.NewECKeyPair failed:%w", err)
	}
	defer ecKeyPair.Close()

	emphermalPublicKey, err := ecKeyPair.PublicPEM()
	if err != nil {
		return ecKeyWrappedKeyInfo{}, fmt.Errorf("generateWrapKeyWithEC: failed to get EC public key: %w", err)
	}

	kasKey, err := crypto.ImportPEM(kasPublicKey)
	if err != nil {
		return ecKeyWrappedKeyInfo{}, fmt.Errorf("generateWrapKeyWithEC: ocrypto.ComputeECDHKey failed:%w", err)
	}
	ecdhKey, err := crypto.ECDH(ecKeyPair, kasKey)
	kasKey.Close()
	if err != nil {
		return ecKeyWrappedKeyInfo{}, fmt.Errorf("generateWrapKeyWithEC: ocrypto.ComputeECDHKey failed:%w", err)
	}

	salt, err := tdfSalt()
	if err != nil {
		return ecKeyWrappedKeyInfo{}, err
	}
	sessionKey, err := crypto.HKDFSHA256(ecdhKey, salt, nil, kKeySize)
	wipe(ecdhKey)
	if err != nil {
		return ecKeyWrappedKeyInfo{}, fmt.Errorf("generateWrapKeyWithEC: ocrypto.CalculateHKDF failed:%w", err)
	}

	nonce, err := crypto.Random(gcmIvSize)
	if err != nil {
		return ecKeyWrappedKeyInfo{}, err
	}
	cipherText, err := crypto.AES256GCMEncrypt(sessionKey, nonce, symKey, nil)
	wipe(sessionKey)
	if err != nil {
		return ecKeyWrappedKeyInfo{}, fmt.Errorf("generateWrapKeyWithEC: ocrypto.AESGcm.Encrypt failed:%w", err)
	}
	wrappedKey, err := encoding.Base64Encode(append(nonce, cipherText...))
	if err != nil {
		return ecKeyWrappedKeyInfo{}, err
	}

	return ecKeyWrappedKeyInfo{
		publicKey:  emphermalPublicKey,
		wrappedKey: wrappedKey,
	}, nil
}

func generateWrapKeyWithRSA(publicKey string, symKey []byte) (string, error) {
	key, err := crypto.ImportPEM(publicKey)
	if err != nil {
		return "", fmt.Errorf("generateWrapKeyWithRSA: ocrypto.FromPublicPEM failed:%w", err)
	}
	defer key.Close()

	wrappedKey, err := crypto.RSAOAEPEncrypt(key, symKey)
	if err != nil {
		return "", fmt.Errorf("generateWrapKeyWithRSA: ocrypto.AsymEncryption.encrypt failed:%w", err)
	}

	return encoding.Base64Encode(wrappedKey)
}

// createPolicyObjectFromFQNs builds the TDF policy document from attribute value FQNs.
func createPolicyObjectFromFQNs(fqns []string) (PolicyObject, error) {
	policyObj := PolicyObject{}
	policyObj.UUID = uuidSplitIDGenerator()
	if policyObj.UUID == "" {
		return PolicyObject{}, errors.New("uuid.NewUUID failed: no random source")
	}

	for i, fqn := range fqns {
		// An empty FQN would write {"attribute": ""} into the bound policy: the
		// PDP denies it, so the TDF fails closed, but at decrypt with nothing
		// naming the cause.
		if fqn == "" {
			return PolicyObject{}, fmt.Errorf("attribute at index %d has an empty FQN; the policy it produces names no attribute and no KAS could evaluate it", i)
		}
		attributeObj := AttributeObject{}
		attributeObj.Attribute = fqn
		policyObj.Body.DataAttributes = append(policyObj.Body.DataAttributes, attributeObj)
		policyObj.Body.Dissem = make([]string, 0)
	}

	return policyObj, nil
}

func allowListFromKASRegistry(ctx context.Context, logger *slog.Logger, kasRegistryClient *kasregistry.KeyAccessServerRegistryServiceClient, call *serviceCall, platformURL string) (AllowList, error) {
	kases, err := kasRegistryClient.ListKeyAccessServers(ctx, &kasregistry.ListKeyAccessServersRequest{})
	if err != nil {
		return nil, fmt.Errorf("kasregistry.ListKeyAccessServers failed: %w", call.serviceError("LoadTDF", err))
	}
	kasAllowlist := AllowList{}
	for _, kas := range kases.GetKeyAccessServers() {
		err = kasAllowlist.Add(kas.GetUri())
		if err != nil {
			return nil, fmt.Errorf("kasAllowlist.Add failed: %w", err)
		}
	}
	// grpc target does not have a scheme
	logger.Debug("adding platform URL to KAS allowlist", slog.String("platform_url", platformURL))
	err = kasAllowlist.Add(platformURL)
	if err != nil {
		return nil, fmt.Errorf("kasAllowlist.Add failed: %w", err)
	}
	return kasAllowlist, nil
}

// LoadTDF loads the tdf and prepare for reading the payload from TDF
func (s *SDK) LoadTDF(ctx context.Context, reader io.ReadSeeker, opts ReaderOptions) (*Reader, error) {
	if err := s.check("LoadTDF"); err != nil {
		return nil, err
	}
	config, err := newTDFReaderConfig(s.cfg.kasSessionKey, opts)
	if err != nil {
		return nil, wrapError(KindInvalidInput, "LoadTDF", "newTDFReaderConfig failed", err)
	}

	archive, err := readArchive(reader, config.maxManifestSize)
	if err != nil {
		config.closeSessionKey()
		return nil, wrapError(KindNotDecryptable, "LoadTDF", "zipstream.NewTDFReader failed", err)
	}
	useGlobalFulfillableObligations := len(config.fulfillableObligationFQNs) == 0 && len(s.cfg.fulfillableObligationFQNs) > 0
	if useGlobalFulfillableObligations {
		config.fulfillableObligationFQNs = s.cfg.fulfillableObligationFQNs
	}

	config.kasAllowlist, err = getKasAllowList(ctx, config.kasAllowlist, s, config.ignoreAllowList)
	if err != nil {
		config.closeSessionKey()
		return nil, wrapError(KindTransport, "LoadTDF", "", err)
	}

	manifest := string(archive.Manifest)

	manifestObj := &Manifest{}
	err = json.Unmarshal([]byte(manifest), manifestObj)
	if err != nil {
		config.closeSessionKey()
		return nil, wrapError(KindInvalidInput, "LoadTDF", "json.Unmarshal failed", err)
	}

	var payloadSize int64
	for _, seg := range manifestObj.EncryptionInformation.IntegrityInformation.Segments {
		size, _, err := manifestObj.EncryptionInformation.IntegrityInformation.resolveSegmentSizes(seg)
		if err != nil {
			config.closeSessionKey()
			return nil, wrapError(KindSegmentSize, "LoadTDF", "", err)
		}
		payloadSize += size
	}

	return &Reader{
		rewrapper:         s.rewrapper,
		logger:            s.cfg.logger,
		ctx:               ctx,
		payload:           archive.Payload,
		manifest:          *manifestObj,
		kasSessionKey:     config.kasSessionKey,
		kasSessionKeyType: config.kasSessionKeyType,
		ownsSessionKey:    config.ownsSessionKey,
		config:            *config,
		payloadSize:       payloadSize,
	}, nil
}

func (c *TDFReaderConfig) closeSessionKey() {
	if c.ownsSessionKey && c.kasSessionKey != nil {
		c.kasSessionKey.Close()
		c.kasSessionKey = nil
	}
}

// Do any network based operations required.
// This allows making the requests cancellable
func (r *Reader) Init(ctx context.Context) error {
	if r.payloadKey != nil {
		return nil
	}
	return boundaryError(KindDecryptFailed, "Init", r.doPayloadKeyUnwrap(ctx))
}

// Close releases the reader's session key and payload key.
func (r *Reader) Close() error {
	if r == nil {
		return nil
	}
	if r.ownsSessionKey && r.kasSessionKey != nil {
		r.kasSessionKey.Close()
	}
	r.kasSessionKey = nil
	wipe(r.payloadKey)
	r.payloadKey = nil
	r.payload = nil
	return nil
}

// callContext is the context for a lazily run Init: the LoadTDF context
// while it lives, as in the reference. A generated host's call context ends
// with its call, so a later lazy operation falls back to the background
// context; hosts that need cancellation call Init first.
func (r *Reader) callContext() context.Context {
	if r.ctx != nil && r.ctx.Err() == nil {
		return r.ctx
	}
	return context.Background()
}

// readAll decrypts the whole payload, verifying every segment: the
// reference's WriteTo from cursor 0, over the in-memory payload.
func (r *Reader) readAll() ([]byte, error) {
	if r.payloadKey == nil {
		err := r.doPayloadKeyUnwrap(r.callContext())
		if err != nil {
			return nil, fmt.Errorf("reader.WriteTo failed: %w", err)
		}
	}

	isLegacyTDF := r.manifest.TDFVersion == ""
	integrity := r.manifest.EncryptionInformation.IntegrityInformation

	out := make([]byte, 0, r.payloadSize)
	var payloadReadOffset int64
	for _, seg := range integrity.Segments {
		_, encryptedSegSize, err := integrity.resolveSegmentSizes(seg)
		if err != nil {
			return nil, err
		}

		if payloadReadOffset+encryptedSegSize > int64(len(r.payload)) {
			return nil, sentinelError(KindSegmentSize, ErrSegSizeMismatch)
		}
		readBuf := r.payload[payloadReadOffset : payloadReadOffset+encryptedSegSize]

		sigAlg := manifestSegmentIntegrityAlg(integrity.SegmentHashAlgorithm)
		payloadSig, err := segmentIntegrity(readBuf, r.payloadKey, sigAlg, isLegacyTDF)
		if err != nil {
			return nil, fmt.Errorf("splitKey.GetSignaturefailed: %w", err)
		}

		encoded, err := encoding.Base64Encode([]byte(payloadSig))
		if err != nil {
			return nil, err
		}
		if seg.Hash != encoded {
			return nil, sentinelError(KindSegmentSignature, ErrSegSigValidation)
		}

		writeBuf, err := crypto.AES256GCMDecrypt(r.payloadKey, readBuf[:gcmIvSize], readBuf[gcmIvSize:], nil)
		if err != nil {
			return nil, wrapError(KindDecryptFailed, "", "splitKey.decrypt failed", err)
		}

		out = append(out, writeBuf...)
		payloadReadOffset += encryptedSegSize
	}

	return out, nil
}

// UnencryptedMetadata return decrypted metadata in manifest.
func (r *Reader) UnencryptedMetadata() ([]byte, error) {
	if r.payloadKey == nil {
		err := r.doPayloadKeyUnwrap(r.callContext())
		if err != nil {
			return nil, wrapError(KindDecryptFailed, "UnencryptedMetadata", "reader.UnencryptedMetadata failed", err)
		}
	}

	return r.unencryptedMetadata, nil
}

// Policy returns a copy of the policy object in manifest, if it is valid.
// Otherwise, returns an error.
func (r *Reader) Policy() (PolicyObject, error) {
	policyObj := PolicyObject{}
	policy, err := encoding.Base64Decode(r.manifest.EncryptionInformation.Policy)
	if err != nil {
		return policyObj, wrapError(KindInvalidInput, "Policy", "ocrypto.Base64Decode failed", err)
	}

	err = json.Unmarshal(policy, &policyObj)
	if err != nil {
		return policyObj, wrapError(KindInvalidInput, "Policy", "json.Unmarshal failed", err)
	}

	return policyObj, nil
}

// DataAttributes return the data attributes present in tdf.
func (r *Reader) DataAttributes() ([]string, error) {
	policy, err := encoding.Base64Decode(r.manifest.EncryptionInformation.Policy)
	if err != nil {
		return nil, wrapError(KindInvalidInput, "DataAttributes", "ocrypto.Base64Decode failed", err)
	}

	policyObj := PolicyObject{}
	err = json.Unmarshal(policy, &policyObj)
	if err != nil {
		return nil, wrapError(KindInvalidInput, "DataAttributes", "json.Unmarshal failed", err)
	}

	attributes := make([]string, 0)
	attributeObjs := policyObj.Body.DataAttributes
	for _, attributeObj := range attributeObjs {
		attributes = append(attributes, attributeObj.Attribute)
	}

	return attributes, nil
}

// rewrapRequests is the reference's map[string]*WithPolicyRequest keyed by
// KAS URL, with insertion order kept for deterministic requests. rejected
// holds the post-quantum KAOs, which are never sent: each fails its split
// as an unwrap failure would, so an alternative KAO can still satisfy it.
type rewrapRequests struct {
	urls     []string
	requests map[string]*kas.UnsignedRewrapRequest_WithPolicyRequest
	rejected []kaoResult
}

func createRewrapRequest(_ context.Context, r *Reader) (*rewrapRequests, error) {
	kasReqs := &rewrapRequests{requests: make(map[string]*kas.UnsignedRewrapRequest_WithPolicyRequest)}
	for i, kao := range r.manifest.EncryptionInformation.KeyAccessObjs {
		kaoID := fmt.Sprintf("kao-%d", i)
		if kao.KeyType == kHybridWrapped || kao.KeyType == kMLKEMWrapped {
			kasReqs.rejected = append(kasReqs.rejected, kaoResult{
				KeyAccessObjectID: kaoID,
				Error:             newError(KindUnsupportedKeyAccess, "Init", "unsupported key access type: "+kao.KeyType+": "+postQuantumNotPlanned),
			})
			continue
		}

		key, err := encoding.Base64Decode(kao.WrappedKey)
		if err != nil {
			return nil, fmt.Errorf("could not decode wrapper key: %w", err)
		}
		binding, err := kao.Binding()
		if err != nil {
			return nil, err
		}
		kaoReq := &kas.UnsignedRewrapRequest_WithKeyAccessObject{
			KeyAccessObjectId: kaoID,
			KeyAccessObject: &kas.KeyAccess{
				KeyType:  kao.KeyType,
				KasUrl:   kao.KasURL,
				Kid:      kao.KID,
				Protocol: kao.Protocol,
				PolicyBinding: &kas.PolicyBinding{
					Hash:      binding.Hash,
					Algorithm: binding.Alg,
				},
				SplitId:            kao.SplitID,
				WrappedKey:         key,
				EphemeralPublicKey: kao.EphemeralPublicKey,
			},
		}
		if req, ok := kasReqs.requests[kao.KasURL]; ok {
			req.KeyAccessObjects = append(req.KeyAccessObjects, kaoReq)
		} else {
			rewrapReq := kas.UnsignedRewrapRequest_WithPolicyRequest{
				Policy: &kas.UnsignedRewrapRequest_WithPolicy{
					Body: r.manifest.EncryptionInformation.Policy,
					Id:   "policy",
				},
				KeyAccessObjects: []*kas.UnsignedRewrapRequest_WithKeyAccessObject{kaoReq},
			}
			kasReqs.requests[kao.KasURL] = &rewrapReq
			kasReqs.urls = append(kasReqs.urls, kao.KasURL)
		}
	}

	return kasReqs, nil
}

func getIdx(kaoID string) int {
	idx, _ := strconv.Atoi(strings.Split(kaoID, "-")[1])
	return idx
}

// skippedSplits is the reference's map[keySplitStep]error, rendered the way
// fmt prints it (sorted keys) in the unable-to-reconstruct message.
type skippedSplit struct {
	step keySplitStep
	err  error
}

type skippedSplitList []skippedSplit

func (s skippedSplitList) Len() int      { return len(s) }
func (s skippedSplitList) Swap(i, j int) { s[i], s[j] = s[j], s[i] }
func (s skippedSplitList) Less(i, j int) bool {
	if s[i].step.KAS != s[j].step.KAS {
		return s[i].step.KAS < s[j].step.KAS
	}
	return s[i].step.SplitID < s[j].step.SplitID
}

func (s skippedSplitList) String() string {
	sort.Sort(s)
	parts := make([]string, 0, len(s))
	for _, e := range s {
		parts = append(parts, "{"+e.step.KAS+" "+e.step.SplitID+"}:"+e.err.Error())
	}
	return "map[" + strings.Join(parts, " ") + "]"
}

func (r *Reader) buildKey(_ context.Context, results []kaoResult) error {
	var unencryptedMetadata []byte
	payloadKey := make([]byte, kKeySize)
	knownSplits := make(map[string]bool)
	foundSplits := make(map[string]bool)
	skipped := skippedSplitList{}

	for _, kaoRes := range results {
		idx := getIdx(kaoRes.KeyAccessObjectID)
		keyAccessObj := r.manifest.EncryptionInformation.KeyAccessObjs[idx]
		ss := keySplitStep{KAS: keyAccessObj.KasURL, SplitID: keyAccessObj.SplitID}

		wrappedKey := kaoRes.SymmetricKey
		err := kaoRes.Error
		knownSplits[ss.SplitID] = true
		if foundSplits[ss.SplitID] {
			// already found
			continue
		}

		if err != nil {
			errToReturn := wrapError("", "", "kao unwrap failed for split {"+ss.KAS+" "+ss.SplitID+"}", err)
			skipped = append(skipped, skippedSplit{step: ss, err: getKasErrorToReturn(err, errToReturn)})
			continue
		}

		for keyByteIndex, keyByte := range wrappedKey {
			if keyByteIndex < len(payloadKey) {
				payloadKey[keyByteIndex] ^= keyByte
			}
		}
		foundSplits[ss.SplitID] = true

		if len(keyAccessObj.EncryptedMetadata) != 0 {
			decodedMetaData, err := encoding.Base64Decode(keyAccessObj.EncryptedMetadata)
			if err != nil {
				return fmt.Errorf("ocrypto.Base64Decode failed:%w", err)
			}

			metadata := EncryptedMetadata{}
			err = json.Unmarshal(decodedMetaData, &metadata)
			if err != nil {
				return fmt.Errorf("json.Unmarshal failed:%w", err)
			}

			cipherText, _ := encoding.Base64Decode(metadata.Cipher)
			if len(cipherText) < gcmIvSize {
				return errors.New("ocrypto.AesGcm.encrypt failed:ciphertext too short")
			}
			metaData, err := crypto.AES256GCMDecrypt(wrappedKey, cipherText[:gcmIvSize], cipherText[gcmIvSize:], nil)
			if err != nil {
				return fmt.Errorf("ocrypto.AesGcm.encrypt failed:%w", err)
			}

			unencryptedMetadata = metaData
		}
	}

	if len(knownSplits) > len(foundSplits) {
		v := make([]error, 1, len(skipped)+1)
		v[0] = errors.New("splitKey.unable to reconstruct split key: " + skipped.String())
		kind := KindNotDecryptable
		for _, e := range skipped {
			v = append(v, e.err)
		}
		return joinError(kind, "Init", v)
	}

	aggregateHash := []byte{}
	for _, segment := range r.manifest.EncryptionInformation.IntegrityInformation.Segments {
		decodedHash, err := encoding.Base64Decode(segment.Hash)
		if err != nil {
			return fmt.Errorf("ocrypto.Base64Decode failed:%w", err)
		}

		aggregateHash = append(aggregateHash, decodedHash...)
	}

	res, err := validateRootSignature(r.manifest, aggregateHash, payloadKey)
	if err != nil {
		return wrapError(KindRootSignature, "Init", ErrRootSignatureFailure.Error()+": splitKey.validateRootSignature failed", err)
	}

	if !res {
		return wrapError(KindRootSignature, "Init", ErrRootSignatureFailure.Error(), ErrRootSigValidation)
	}

	segSize := r.manifest.EncryptionInformation.IntegrityInformation.DefaultSegmentSize
	encryptedSegSize := r.manifest.EncryptionInformation.IntegrityInformation.DefaultEncryptedSegSize

	if segSize != encryptedSegSize-(gcmIvSize+aesBlockSize) {
		return sentinelError(KindSegmentSize, ErrSegSizeMismatch)
	}

	// Validate assertions
	if len(r.manifest.Assertions) > 0 && !r.config.disableAssertionVerification {
		return unsupported("Init", "assertion verification")
	}

	r.unencryptedMetadata = unencryptedMetadata
	r.payloadKey = payloadKey

	return nil
}

// Unwraps the payload key, if possible, using the access service
func (r *Reader) doPayloadKeyUnwrap(ctx context.Context) error {
	var kaoResults []kaoResult
	reqFail := func(err error, req *kas.UnsignedRewrapRequest_WithPolicyRequest) {
		for _, kao := range req.GetKeyAccessObjects() {
			kaoResults = append(kaoResults, kaoResult{
				KeyAccessObjectID: kao.GetKeyAccessObjectId(),
				Error:             err,
			})
		}
	}

	reqs, err := createRewrapRequest(ctx, r)
	if err != nil {
		return err
	}
	kaoResults = append(kaoResults, reqs.rejected...)
	for _, kasurl := range reqs.urls {
		req := reqs.requests[kasurl]
		// if ignoreing allowlist then warn
		// if kas url is not allowed then return error
		if r.config.ignoreAllowList {
			r.logger.WarnContext(ctx, "kasAllowlist is ignored, kas url is allowed", slog.String("kas_url", kasurl))
		} else if !r.config.kasAllowlist.IsAllowed(kasurl) {
			reqFail(&Error{Kind: KindAllowlistRejected, Op: "Init", Message: fmt.Sprintf("KasAllowlist: kas url %s is not allowed", kasurl), KASURL: kasurl}, req)
			continue
		}

		// if allowed then unwrap
		policyRes, err := r.rewrapper.Unwrap(ctx, r.kasSessionKey, r.kasSessionKeyType, []*kas.UnsignedRewrapRequest_WithPolicyRequest{req})
		if err != nil {
			reqFail(err, req)
		} else {
			result, ok := policyRes["policy"]
			if !ok {
				err = errors.New("could not find policy in rewrap response")
				reqFail(err, req)
			}
			kaoResults = append(kaoResults, result...)
		}
	}
	// Deduplicate obligations for all kao results
	r.requiredObligations = &RequiredObligations{FQNs: dedupRequiredObligations(kaoResults)}

	return r.buildKey(ctx, kaoResults)
}

// hmacIntegrity computes an HMAC-SHA256 over data, keyed by the payload key.
//
// Legacy (pre-4.3.0) TDFs hex-encode the digest before the caller base64s it;
// isLegacyTDF preserves that wire format.
func hmacIntegrity(data, key []byte, isLegacyTDF bool) (string, error) {
	hmac, err := crypto.HMACSHA256(key, data)
	if err != nil {
		return "", err
	}
	if isLegacyTDF {
		return hex.EncodeToString(hmac), nil
	}
	return string(hmac), nil
}

// readAEADTag returns the trailing authentication tag of an AES-GCM ciphertext.
func readAEADTag(ciphertext []byte, isLegacyTDF bool) (string, error) {
	if kGMACPayloadLength > len(ciphertext) {
		return "", wrapError(KindSegmentSignature, "", "", fmt.Errorf("%w: ciphertext length=%d", ErrGMACSignatureFailed, len(ciphertext)))
	}

	tag := ciphertext[len(ciphertext)-kGMACPayloadLength:]
	if isLegacyTDF {
		return hex.EncodeToString(tag), nil
	}
	return string(tag), nil
}

// segmentIntegrity computes the integrity value recorded in (and checked
// against) a segment's manifest entry, over that segment's AES-GCM ciphertext.
func segmentIntegrity(ciphertext, key []byte, alg SegmentIntegrityAlg, isLegacyTDF bool) (string, error) {
	switch alg {
	case SegmentHS256:
		return hmacIntegrity(ciphertext, key, isLegacyTDF)
	case SegmentGMAC:
		return readAEADTag(ciphertext, isLegacyTDF)
	}
	return "", wrapError(KindUnsupportedSegmentIntegrity, "", "", fmt.Errorf("%w: %s", ErrUnsupportedSegmentIntegrityAlgorithm, alg.String()))
}

// rootIntegrity computes the root signature over the aggregate hash: the
// concatenation of every segment's decoded hash, in manifest order. HS256 only.
func rootIntegrity(aggregateHash, key []byte, alg RootIntegrityAlg, isLegacyTDF bool) (string, error) {
	if alg != RootHS256 {
		return "", wrapError(KindUnsupportedRootIntegrity, "", "", fmt.Errorf("%w: %s", ErrUnsupportedRootIntegrityAlgorithm, alg.String()))
	}
	return hmacIntegrity(aggregateHash, key, isLegacyTDF)
}

// validate the root signature
func validateRootSignature(manifest Manifest, aggregateHash, secret []byte) (bool, error) {
	rootSigAlg := manifest.EncryptionInformation.IntegrityInformation.RootSignature.Algorithm
	rootSigValue := manifest.EncryptionInformation.IntegrityInformation.RootSignature.Signature
	isLegacyTDF := manifest.TDFVersion == ""

	// Allowlist, not "anything that is not GMAC means HS256". An absent or
	// empty alg keeps its historical meaning of HS256.
	if rootSigAlg != "" && !strings.EqualFold(hmacIntegrityAlgorithm, rootSigAlg) {
		return false, wrapError(KindUnsupportedRootIntegrity, "", "", fmt.Errorf("%w: %q", ErrUnsupportedRootIntegrityAlgorithm, rootSigAlg))
	}

	sig, err := rootIntegrity(aggregateHash, secret, RootHS256, isLegacyTDF)
	if err != nil {
		return false, fmt.Errorf("splitkey.getSignature failed:%w", err)
	}

	encoded, err := encoding.Base64Encode([]byte(sig))
	if err != nil {
		return false, err
	}
	if rootSigValue == encoded {
		return true, nil
	}

	return false, nil
}

func populateKasInfoFromBaseKey(key SimpleKasKey, tdfConfig *TDFConfig, logger *slog.Logger) error {
	algoString, err := formatAlg(policyAlgorithmOf(key.PublicKey.Algorithm))
	if err != nil {
		return fmt.Errorf("formatAlg failed: %w", err)
	}

	if tdfConfig.preferredKeyWrapAlg != KeyType(algoString) {
		logger.Warn("base key is enabled, setting key type", slog.String("key_type", algoString))
	}
	tdfConfig.preferredKeyWrapAlg = KeyType(algoString)
	tdfConfig.splitPlan = nil
	if len(tdfConfig.kasInfoList) > 0 {
		logger.Warn("base key is enabled, overwriting kasInfoList with base key info")
	}
	tdfConfig.kasInfoList = []KASInfo{
		{
			URL:       key.KasURI,
			PublicKey: key.PublicKey.Pem,
			KID:       key.PublicKey.Kid,
			Algorithm: algoString,
		},
	}
	return nil
}

func createKaoTemplateFromKasInfo(kasInfoArr []KASInfo) []kaoTpl {
	kaoTemplate := make([]kaoTpl, len(kasInfoArr))
	for i, kasInfo := range kasInfoArr {
		splitID := ""
		if len(kasInfoArr) > 1 {
			splitID = fmt.Sprintf("s-%d", i)
		}
		kaoTemplate[i] = kaoTpl{
			KAS:       kasInfo.URL,
			SplitID:   splitID,
			kid:       kasInfo.KID,
			pem:       kasInfo.PublicKey,
			algorithm: KeyType(kasInfo.Algorithm),
		}
	}

	return kaoTemplate
}

// getKasErrorToReturn classifies KAS rewrap errors. KAS intentionally uses a
// generic "bad request" for policy binding and DEK failures to avoid leaking
// information about secret key material. Descriptive messages indicate
// client/configuration issues that are safe to surface as non-tamper errors.
func getKasErrorToReturn(err error, defaultError error) error {
	errToReturn := defaultError
	errStr := err.Error()

	switch {
	case strings.Contains(errStr, "InvalidArgument"):
		if strings.Contains(errStr, kasGenericBadRequest) {
			errToReturn = joinError(KindRewrapBadRequest, "", []error{ErrRewrapBadRequest, errToReturn})
		} else {
			errToReturn = joinError(KindKASRequest, "", []error{ErrKASRequestError, errToReturn})
		}
	case strings.Contains(errStr, "PermissionDenied"):
		errToReturn = joinError(KindRewrapForbidden, "", []error{ErrRewrapForbidden, errToReturn})
	}

	return errToReturn
}

func getKasAllowList(ctx context.Context, kasAllowList AllowList, s *SDK, ignoreAllowList bool) (AllowList, error) {
	allowList := kasAllowList
	if len(allowList) == 0 && !ignoreAllowList {
		// retrieve the registered kases if not provided
		platformEndpoint, err := s.cfg.PlatformConfiguration.platformEndpoint()
		if err != nil {
			return nil, fmt.Errorf("retrieving platformEndpoint failed: %w", err)
		}
		conn, call := s.client.newServiceClient(s.serviceEndpoint(policyServices), nil)
		allowList, err = allowListFromKASRegistry(ctx, s.cfg.logger, kasregistry.NewKeyAccessServerRegistryServiceClient(conn), call, platformEndpoint)
		if err != nil {
			return nil, fmt.Errorf("allowListFromKASRegistry failed: %w", err)
		}
	}

	return allowList, nil
}

func dedupRequiredObligations(kaoResults []kaoResult) []string {
	seen := make(map[string]bool)
	dedupedOblgs := make([]string, 0)
	for _, kao := range kaoResults {
		for _, oblg := range kao.RequiredObligations {
			normalizedOblg := strings.TrimSpace(strings.ToLower(oblg))
			if len(normalizedOblg) == 0 {
				continue
			}
			if _, ok := seen[normalizedOblg]; !ok {
				seen[normalizedOblg] = true
				dedupedOblgs = append(dedupedOblgs, normalizedOblg)
			}
		}
	}

	return dedupedOblgs
}
