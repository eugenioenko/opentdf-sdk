package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/lib/sync"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/net"
	"github.com/eugenioenko/goalchemy/std/net/url"
)

const (
	secondsPerMinute              = 60
	statusFail                    = "fail"
	statusPermit                  = "permit"
	additionalRewrapContextHeader = "X-Rewrap-Additional-Context"
	triggeredObligationsHeader    = "X-Required-Obligations"
	kasRewrapProcedure            = "/kas.AccessService/Rewrap"
	kasPublicKeyProcedure         = "/kas.AccessService/PublicKey"
)

// Rewrapper unwraps key access objects at a KAS: the reference's KASClient.
// New wires KASClient; tests supply a fake KAS.
type Rewrapper interface {
	Unwrap(ctx context.Context, sessionKey *crypto.Key, sessionKeyType KeyType, requests []*rewrapPolicyRequest) (map[string][]kaoResult, error)
}

// KASClient is the default Rewrapper: hand-written Connect JSON over the
// SDK's authenticated transport until Phase 3 generates the service client.
type KASClient struct {
	client *httpClient
	signer *dpopSigner
	clock  Clock

	// Set this to enable legacy, non-batch rewrap requests
	supportSingleRewrapEndpoint bool
	fulfillableObligations      []string
}

type kaoResult struct {
	SymmetricKey        []byte
	Error               error
	KeyAccessObjectID   string
	RequiredObligations []string
}

// The kas.UnsignedRewrapRequest message tree, with its protojson names.
type rewrapKeyAccess struct {
	KeyType            string
	KasURL             string
	KID                string
	Protocol           string
	PolicyBinding      PolicyBinding
	SplitID            string
	WrappedKey         []byte
	EphemeralPublicKey string
	EncryptedMetadata  string
}

type rewrapKeyAccessObject struct {
	KeyAccessObjectID string
	KeyAccessObject   rewrapKeyAccess
}

type rewrapPolicy struct {
	ID   string
	Body string
}

type rewrapPolicyRequest struct {
	Policy           rewrapPolicy
	KeyAccessObjects []*rewrapKeyAccessObject
	Algorithm        string
}

func newKASClient(client *httpClient, signer *dpopSigner, clock Clock, fulfillableObligations []string) *KASClient {
	return &KASClient{
		client:                      client,
		signer:                      signer,
		clock:                       clock,
		supportSingleRewrapEndpoint: true,
		fulfillableObligations:      fulfillableObligations,
	}
}

func keyAccessJSON(k rewrapKeyAccess) (jsonvalue.Value, error) {
	wrapped, err := encoding.Base64Encode(k.WrappedKey)
	if err != nil {
		return jsonvalue.Value{}, err
	}
	v := jsonvalue.Object("type", jsonvalue.String(k.KeyType), "url", jsonvalue.String(k.KasURL), "protocol", jsonvalue.String(k.Protocol), "wrappedKey", jsonvalue.String(wrapped),
		"policyBinding", jsonvalue.Object("alg", jsonvalue.String(k.PolicyBinding.Alg), "hash", jsonvalue.String(k.PolicyBinding.Hash)))
	if k.KID != "" {
		v = v.Set("kid", jsonvalue.String(k.KID))
	}
	if k.SplitID != "" {
		v = v.Set("sid", jsonvalue.String(k.SplitID))
	}
	if k.EphemeralPublicKey != "" {
		v = v.Set("ephemeralPublicKey", jsonvalue.String(k.EphemeralPublicKey))
	}
	if k.EncryptedMetadata != "" {
		v = v.Set("encryptedMetadata", jsonvalue.String(k.EncryptedMetadata))
	}
	return v, nil
}

// there is no connection caching as of now
func (k *KASClient) makeRewrapRequest(ctx context.Context, requests []*rewrapPolicyRequest, pubKey string) (jsonvalue.Value, error) {
	rewrapRequest, err := k.getRewrapRequest(requests, pubKey)
	if err != nil {
		return jsonvalue.Value{}, err
	}
	kasURL := requests[0].KeyAccessObjects[0].KeyAccessObject.KasURL
	parsedURL, err := parseBaseURL(kasURL)
	if err != nil {
		return jsonvalue.Value{}, fmt.Errorf("cannot parse kas url(%s): %w", kasURL, err)
	}
	headers, err := k.rewrapContextHeaders()
	if err != nil {
		return jsonvalue.Value{}, fmt.Errorf("error creating rewrap request: %w", err)
	}
	response, err := k.client.connectCall(ctx, parsedURL, kasRewrapProcedure, rewrapRequest, headers, "Rewrap")
	if err != nil {
		return jsonvalue.Value{}, err
	}
	return upgradeRewrapResponseV1(response, requests), nil
}

// rewrapContextHeaders carries the fulfillable obligation FQNs, base64 JSON,
// as the reference's newConnectRewrapRequest does.
func (k *KASClient) rewrapContextHeaders() ([]string, error) {
	fqns := jsonvalue.Array()
	for _, f := range k.fulfillableObligations {
		fqns = fqns.Append(jsonvalue.String(f))
	}
	rewrapContext := jsonvalue.Object("obligations", jsonvalue.Object("fulfillableFQNs", fqns))
	rewrapContextJSON, err := jsonvalue.Encode(rewrapContext)
	if err != nil {
		return nil, fmt.Errorf("error marshaling additional rewrap context: %w", err)
	}
	rewrapContextBase64, err := encoding.Base64Encode(rewrapContextJSON)
	if err != nil {
		return nil, err
	}
	return []string{additionalRewrapContextHeader, rewrapContextBase64}, nil
}

// convert v1 responses to v2
func upgradeRewrapResponseV1(response jsonvalue.Value, requests []*rewrapPolicyRequest) jsonvalue.Value {
	if response.Get("responses").Len() > 0 {
		return response
	}
	wrapped := response.Get("entityWrappedKey").String()
	if wrapped == "" || len(requests) == 0 {
		return response
	}
	result := jsonvalue.Object("keyAccessObjectId", jsonvalue.String(requests[0].KeyAccessObjects[0].KeyAccessObjectID), "status", jsonvalue.String(statusPermit), "kasWrappedKey", jsonvalue.String(wrapped))
	policyResult := jsonvalue.Object("policyId", jsonvalue.String(requests[0].Policy.ID), "results", jsonvalue.Array(result))
	return response.Set("responses", jsonvalue.Array(policyResult))
}

// convert v1 errors to v2 responses: a failed request becomes a failed
// result for the single request's first KAO, keeping the typed error so its
// kind and service fields survive.
func upgradeRewrapErrorV1(err error, requests []*rewrapPolicyRequest) (map[string][]kaoResult, error) {
	if len(requests) != 1 {
		return nil, fmt.Errorf("error making rewrap request: %w", err)
	}
	return map[string][]kaoResult{requests[0].Policy.ID: {{KeyAccessObjectID: requests[0].KeyAccessObjects[0].KeyAccessObjectID, Error: err}}}, nil
}

// Unwrap sends the rewrap requests and returns the unwrapped keys per policy.
func (k *KASClient) Unwrap(ctx context.Context, sessionKey *crypto.Key, sessionKeyType KeyType, requests []*rewrapPolicyRequest) (map[string][]kaoResult, error) {
	if sessionKey == nil {
		return nil, errors.New("session key is nil")
	}
	pubKey, err := sessionKey.PublicPEM()
	if err != nil {
		return nil, fmt.Errorf("ocrypto.PublicKeyInPermFormat failed: %w", err)
	}
	response, err := k.makeRewrapRequest(ctx, requests, pubKey)
	if err != nil {
		results, upgradeErr := upgradeRewrapErrorV1(err, requests)
		if upgradeErr != nil {
			return nil, fmt.Errorf("error making rewrap request to kas: %w", upgradeErr)
		}
		return results, nil
	}
	return k.handleKeyResponse(response, sessionKey, sessionKeyType)
}

// handleKeyResponse unwraps a rewrap response with the handler matching the
// session key type.
func (k *KASClient) handleKeyResponse(response jsonvalue.Value, sessionKey *crypto.Key, sessionKeyType KeyType) (map[string][]kaoResult, error) {
	if IsECKeyType(sessionKeyType) {
		return k.handleECKeyResponse(response, sessionKey)
	}
	return k.handleRSAKeyResponse(response, sessionKey)
}

func (k *KASClient) handleECKeyResponse(response jsonvalue.Value, sessionKey *crypto.Key) (map[string][]kaoResult, error) {
	kasEphemeralPublicKey := response.Get("sessionPublicKey").String()
	kasKey, err := crypto.ImportPEM(kasEphemeralPublicKey)
	if err != nil {
		return nil, fmt.Errorf("ocrypto.ComputeECDHKey failed: %w", err)
	}
	ecdhKey, err := crypto.ECDH(sessionKey, kasKey)
	kasKey.Close()
	if err != nil {
		return nil, fmt.Errorf("ocrypto.ComputeECDHKey failed: %w", err)
	}
	salt, err := tdfSalt()
	if err != nil {
		return nil, err
	}
	derived, err := crypto.HKDFSHA256(ecdhKey, salt, nil, kKeySize)
	wipe(ecdhKey)
	if err != nil {
		return nil, fmt.Errorf("ocrypto.CalculateHKDF failed: %w", err)
	}
	results, err := k.processResponse(response, func(wrapped []byte) ([]byte, error) {
		if len(wrapped) < gcmIvSize {
			return nil, errors.New("ciphertext too short")
		}
		return crypto.AES256GCMDecrypt(derived, wrapped[:gcmIvSize], wrapped[gcmIvSize:], nil)
	})
	wipe(derived)
	return results, err
}

/*
Metadata will be in the following form, per kao:

	{
	   "metadata": {
		  "X-Required-Obligations": [<Required obligation FQNs>]
	   }
	}
*/
func (k *KASClient) retrieveObligationsFromMetadata(metadata jsonvalue.Value) []string {
	var requiredObligations []string
	if metadata.Kind() != jsonvalue.KindObject {
		return requiredObligations
	}
	triggerOblsValue, ok := metadata.Lookup(triggeredObligationsHeader)
	if !ok {
		return requiredObligations
	}
	for i := 0; i < triggerOblsValue.Len(); i++ {
		v := triggerOblsValue.Index(i)
		if v.Kind() == jsonvalue.KindString {
			requiredObligations = append(requiredObligations, v.String())
		}
	}
	return requiredObligations
}

func (k *KASClient) handleRSAKeyResponse(response jsonvalue.Value, sessionKey *crypto.Key) (map[string][]kaoResult, error) {
	return k.processResponse(response, func(wrapped []byte) ([]byte, error) {
		return crypto.RSAOAEPDecrypt(sessionKey, wrapped)
	})
}

// processResponse unwraps each permitted KAO's key with decrypt, which is
// specific to the session key type (RSA or EC-derived AES-GCM).
func (k *KASClient) processResponse(response jsonvalue.Value, decrypt func([]byte) ([]byte, error)) (map[string][]kaoResult, error) {
	policyResults := make(map[string][]kaoResult)
	responses := response.Get("responses")
	for i := 0; i < responses.Len(); i++ {
		results := responses.Index(i)
		var kaoKeys []kaoResult
		kaos := results.Get("results")
		for j := 0; j < kaos.Len(); j++ {
			kao := kaos.Index(j)
			kaoID := kao.Get("keyAccessObjectId").String()
			requiredObligationsForKAO := k.retrieveObligationsFromMetadata(kao.Get("metadata"))
			if kao.Get("status").String() == statusPermit {
				wrapped, err := encoding.Base64Decode(kao.Get("kasWrappedKey").String())
				if err == nil {
					var key []byte
					key, err = decrypt(wrapped)
					if err == nil {
						kaoKeys = append(kaoKeys, kaoResult{KeyAccessObjectID: kaoID, SymmetricKey: key, RequiredObligations: requiredObligationsForKAO})
						continue
					}
				}
				kaoKeys = append(kaoKeys, kaoResult{KeyAccessObjectID: kaoID, Error: err, RequiredObligations: requiredObligationsForKAO})
			} else {
				kaoKeys = append(kaoKeys, kaoResult{KeyAccessObjectID: kaoID, Error: errors.New(kao.Get("error").String()), RequiredObligations: requiredObligationsForKAO})
			}
		}
		policyResults[results.Get("policyId").String()] = kaoKeys
	}
	return policyResults, nil
}

func parseBaseURL(rawURL string) (string, error) {
	u, err := url.Parse(rawURL)
	if err != nil {
		return "", err
	}

	host := u.Hostname()
	port := u.Port()

	// Add port only if it's present
	addr := host
	if port != "" {
		addr = net.JoinHostPort(host, port)
	}

	return fmt.Sprintf("%s://%s", u.Scheme, addr), nil
}

// getRewrapRequest builds the signed request token over the protojson
// UnsignedRewrapRequest, signed with the DPoP key as the reference does.
func (k *KASClient) getRewrapRequest(reqs []*rewrapPolicyRequest, pubKey string) ([]byte, error) {
	if len(reqs) == 0 {
		return nil, errors.New("no requests provided")
	}
	if k.signer == nil {
		return nil, ErrNoAccessTokenSource
	}
	requests := jsonvalue.Array()
	for _, req := range reqs {
		kaos := jsonvalue.Array()
		for _, kao := range req.KeyAccessObjects {
			ka, err := keyAccessJSON(kao.KeyAccessObject)
			if err != nil {
				return nil, err
			}
			kaos = kaos.Append(jsonvalue.Object("keyAccessObjectId", jsonvalue.String(kao.KeyAccessObjectID), "keyAccessObject", ka))
		}
		entry := jsonvalue.Object("keyAccessObjects", kaos, "policy", jsonvalue.Object("id", jsonvalue.String(req.Policy.ID), "body", jsonvalue.String(req.Policy.Body)))
		if req.Algorithm != "" {
			entry = entry.Set("algorithm", jsonvalue.String(req.Algorithm))
		}
		requests = requests.Append(entry)
	}
	requestBody := jsonvalue.Object("clientPublicKey", jsonvalue.String(pubKey), "requests", requests)
	if len(reqs) == 1 && len(reqs[0].KeyAccessObjects) == 1 && k.supportSingleRewrapEndpoint {
		ka, err := keyAccessJSON(reqs[0].KeyAccessObjects[0].KeyAccessObject)
		if err != nil {
			return nil, err
		}
		requestBody = requestBody.Set("keyAccess", ka).Set("policy", jsonvalue.String(reqs[0].Policy.Body))
		if reqs[0].Algorithm != "" {
			requestBody = requestBody.Set("algorithm", jsonvalue.String(reqs[0].Algorithm))
		}
	}
	requestBodyJSON, err := jsonvalue.Encode(requestBody)
	if err != nil {
		return nil, fmt.Errorf("error marshaling request body: %w", err)
	}

	now := k.clock.Now()
	claims := jsonvalue.Object("requestBody", jsonvalue.String(string(requestBodyJSON)), "iat", jsonvalue.Int(now), "exp", jsonvalue.Int(now+secondsPerMinute))
	header := jsonvalue.Object("alg", jsonvalue.String(string(k.signer.algorithm)), "typ", jsonvalue.String("JWT"))
	signedToken, err := signJWT(k.signer.key, k.signer.algorithm, header, claims)
	if err != nil {
		return nil, fmt.Errorf("failed to sign the token: %w", err)
	}
	return jsonvalue.Encode(jsonvalue.Object("signedRequestToken", jsonvalue.String(signedToken)))
}

type kasKeyRequest struct {
	url, algorithm, kid string
}

type timeStampedKASInfo struct {
	KASInfo
	Time int64
}

// How long a cached KAS key stays usable before get() evicts it, in seconds.
const kasKeyCacheTTL = 5 * 60

// Caches the most recent key info for a given KAS URL and algorithm.
//
// A single cache is shared by every operation on an SDK instance, so all
// access holds cacheMu.
type kasKeyCache struct {
	cacheMu sync.Mutex
	clock   Clock
	c       map[kasKeyRequest]timeStampedKASInfo
}

func newKasKeyCache(clock Clock) *kasKeyCache {
	return &kasKeyCache{clock: clock, c: make(map[kasKeyRequest]timeStampedKASInfo)}
}

func (c *kasKeyCache) clear() {
	c.cacheMu.Lock()
	defer c.cacheMu.Unlock()
	c.c = make(map[kasKeyRequest]timeStampedKASInfo)
}

func (c *kasKeyCache) get(url, algorithm, kid string) *KASInfo {
	cacheKey := kasKeyRequest{url, algorithm, kid}
	now := c.clock.Now()

	c.cacheMu.Lock()
	cv, ok := c.c[cacheKey]
	if !ok && kid == "" {
		for k, v := range c.c {
			if k.url == url && k.algorithm == algorithm {
				cv = v
				cacheKey = k
				ok = true
				break
			}
		}
	}
	c.cacheMu.Unlock()

	if !ok {
		return nil
	}
	ago := now - kasKeyCacheTTL
	if ago > cv.Time {
		c.evictIfUnchanged(cacheKey, cv.Time)
		return nil
	}
	// cv is a copy, so this pointer is not shared with any other caller.
	info := cv.KASInfo
	return &info
}

// evictIfUnchanged drops an expired entry, unless another caller refreshed
// it after the lock was released.
func (c *kasKeyCache) evictIfUnchanged(cacheKey kasKeyRequest, staleAt int64) {
	c.cacheMu.Lock()
	defer c.cacheMu.Unlock()
	if current, ok := c.c[cacheKey]; ok && current.Time == staleAt {
		delete(c.c, cacheKey)
	}
}

func (c *kasKeyCache) store(ki KASInfo) {
	cacheKey := kasKeyRequest{ki.URL, ki.Algorithm, ki.KID}
	c.cacheMu.Lock()
	defer c.cacheMu.Unlock()
	c.c[cacheKey] = timeStampedKASInfo{ki, c.clock.Now()}
}

// KASKeyFetcher resolves a KAS public key for a URL, algorithm and kid: the
// reference's interface of the same name. New wires the SDK itself (with its
// cache); tests supply replacements.
type KASKeyFetcher interface {
	getPublicKey(ctx context.Context, kasurl, algorithm, kidToFind string) (*KASInfo, error)
}

func (s *SDK) getPublicKey(ctx context.Context, kasurl, algorithm, kidToFind string) (*KASInfo, error) {
	if s.kasKeyCache != nil {
		if cachedValue := s.kasKeyCache.get(kasurl, algorithm, kidToFind); nil != cachedValue {
			return cachedValue, nil
		}
	}
	parsedURL, err := parseBaseURL(kasurl)
	if err != nil {
		return nil, fmt.Errorf("cannot parse kas url(%s): %w", kasurl, err)
	}

	req := jsonvalue.Object()
	if algorithm != "" {
		req = req.Set("algorithm", jsonvalue.String(algorithm))
	}
	if s.cfg.tdfFeatures.noKID {
		req = req.Set("v", jsonvalue.String("1"))
	}
	body, err := jsonvalue.Encode(req)
	if err != nil {
		return nil, err
	}
	resp, err := s.client.connectCall(ctx, parsedURL, kasPublicKeyProcedure, body, nil, "PublicKey")
	if err != nil {
		return nil, fmt.Errorf("error making request to KAS: %w", err)
	}

	kid := resp.Get("kid").String()
	if s.cfg.tdfFeatures.noKID {
		kid = ""
	}

	ki := KASInfo{
		URL:       kasurl,
		Algorithm: algorithm,
		KID:       kid,
		PublicKey: resp.Get("publicKey").String(),
	}
	if s.kasKeyCache != nil {
		s.kasKeyCache.store(ki)
	}
	return &ki, nil
}

func wipe(data []byte) {
	for i := range data {
		data[i] = 0
	}
}
