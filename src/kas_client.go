package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/lib/crypto"
	"github.com/eugenioenko/goalchemy/lib/encoding"
	"github.com/eugenioenko/goalchemy/lib/sync"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/encoding/protojson"
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/net"
	"github.com/eugenioenko/goalchemy/std/net/url"
	"opentdf-local/sdk/src/protocol/kas"
)

const (
	secondsPerMinute              = 60
	statusFail                    = "fail"
	statusPermit                  = "permit"
	additionalRewrapContextHeader = "X-Rewrap-Additional-Context"
	triggeredObligationsHeader    = "X-Required-Obligations"
)

// Rewrapper unwraps key access objects at a KAS: the reference's KASClient.
// New wires KASClient; tests supply a fake KAS.
type Rewrapper interface {
	// Unwrap requests and decrypts a KAS key share for the policy. ctx controls the
	// request; sessionKey is caller-owned.
	Unwrap(ctx context.Context, sessionKey *crypto.Key, sessionKeyType KeyType, requests []*kas.UnsignedRewrapRequest_WithPolicyRequest) (map[string][]kaoResult, error)
}

// KASClient is the default Rewrapper: the generated kas.AccessService
// client over the SDK's authenticated transport.
type KASClient struct {
	client *httpClient
	signer *dpopSigner
	clock  Clock

	// Set this to enable legacy, non-batch rewrap requests
	supportSingleRewrapEndpoint bool
	fulfillableObligations      []string
}

// kaoResult holds one key-access outcome returned by a Rewrapper.
type kaoResult struct {
	// SymmetricKey is the unwrapped key share. Treat it as sensitive material.
	SymmetricKey []byte
	// Error reports failure for this key access object, if any.
	Error error
	// KeyAccessObjectID identifies the requested key access object.
	KeyAccessObjectID string
	// RequiredObligations lists obligations returned by the KAS.
	RequiredObligations []string
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

// there is no connection caching as of now
func (k *KASClient) makeRewrapRequest(ctx context.Context, requests []*kas.UnsignedRewrapRequest_WithPolicyRequest, pubKey string) (*kas.RewrapResponse, error) {
	rewrapRequest, err := k.getRewrapRequest(requests, pubKey)
	if err != nil {
		return nil, err
	}
	kasURL := requests[0].GetKeyAccessObjects()[0].GetKeyAccessObject().GetKasUrl()
	parsedURL, err := parseBaseURL(kasURL)
	if err != nil {
		return nil, fmt.Errorf("cannot parse kas url(%s): %w", kasURL, err)
	}

	headers, err := k.newConnectRewrapRequestHeaders()
	if err != nil {
		return nil, fmt.Errorf("error creating rewrap request: %w", err)
	}
	conn, call := k.client.newServiceClient(parsedURL, headers)
	serviceClient := kas.NewAccessServiceClient(conn)

	response, err := serviceClient.Rewrap(ctx, rewrapRequest)
	if err != nil {
		return nil, call.serviceError("Rewrap", err)
	}

	upgradeRewrapResponseV1(response, requests)

	return response, nil
}

// newConnectRewrapRequestHeaders carries the fulfillable obligation FQNs,
// base64 JSON, as the reference's newConnectRewrapRequest does.
func (k *KASClient) newConnectRewrapRequestHeaders() ([]string, error) {
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
func upgradeRewrapResponseV1(response *kas.RewrapResponse, requests []*kas.UnsignedRewrapRequest_WithPolicyRequest) {
	if len(response.GetResponses()) > 0 {
		return
	}
	if len(response.GetEntityWrappedKey()) == 0 {
		return
	}
	if len(requests) == 0 {
		return
	}
	response.Responses = []*kas.PolicyRewrapResult{
		{
			PolicyId: requests[0].GetPolicy().GetId(),
			Results: []*kas.KeyAccessRewrapResult{
				{
					KeyAccessObjectId: requests[0].GetKeyAccessObjects()[0].GetKeyAccessObjectId(),
					Status:            statusPermit,
					Result: &kas.KeyAccessRewrapResult_KasWrappedKey{
						KasWrappedKey: response.GetEntityWrappedKey(),
					},
				},
			},
		},
	}
}

// convert v1 errors to v2 responses: a failed request becomes a failed
// result for the single request's first KAO, keeping the typed error so its
// kind and service fields survive.
func upgradeRewrapErrorV1(err error, requests []*kas.UnsignedRewrapRequest_WithPolicyRequest) (map[string][]kaoResult, error) {
	if len(requests) != 1 {
		return nil, fmt.Errorf("error making rewrap request: %w", err)
	}
	return map[string][]kaoResult{requests[0].GetPolicy().GetId(): {{KeyAccessObjectID: requests[0].GetKeyAccessObjects()[0].GetKeyAccessObjectId(), Error: err}}}, nil
}

// Unwrap sends the rewrap requests and returns the unwrapped keys per policy.
func (k *KASClient) Unwrap(ctx context.Context, sessionKey *crypto.Key, sessionKeyType KeyType, requests []*kas.UnsignedRewrapRequest_WithPolicyRequest) (map[string][]kaoResult, error) {
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
func (k *KASClient) handleKeyResponse(response *kas.RewrapResponse, sessionKey *crypto.Key, sessionKeyType KeyType) (map[string][]kaoResult, error) {
	if IsECKeyType(sessionKeyType) {
		return k.handleECKeyResponse(response, sessionKey)
	}
	return k.handleRSAKeyResponse(response, sessionKey)
}

func (k *KASClient) handleECKeyResponse(response *kas.RewrapResponse, sessionKey *crypto.Key) (map[string][]kaoResult, error) {
	kasEphemeralPublicKey := response.GetSessionPublicKey()
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
func (k *KASClient) retrieveObligationsFromMetadata(metadata map[string]jsonvalue.Value) []string {
	var requiredObligations []string

	if metadata == nil {
		return requiredObligations
	}

	triggerOblsValue, ok := metadata[triggeredObligationsHeader]
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

func (k *KASClient) handleRSAKeyResponse(response *kas.RewrapResponse, sessionKey *crypto.Key) (map[string][]kaoResult, error) {
	return k.processResponse(response, func(wrapped []byte) ([]byte, error) {
		return crypto.RSAOAEPDecrypt(sessionKey, wrapped)
	})
}

// processResponse unwraps each permitted KAO's key with decrypt, which is
// specific to the session key type (RSA or EC-derived AES-GCM).
func (k *KASClient) processResponse(response *kas.RewrapResponse, decrypt func([]byte) ([]byte, error)) (map[string][]kaoResult, error) {
	policyResults := make(map[string][]kaoResult)
	for _, results := range response.GetResponses() {
		var kaoKeys []kaoResult
		for _, kao := range results.GetResults() {
			requiredObligationsForKAO := k.retrieveObligationsFromMetadata(kao.GetMetadata())
			if kao.GetStatus() == statusPermit {
				key, err := decrypt(kao.GetKasWrappedKey())
				if err != nil {
					kaoKeys = append(kaoKeys, kaoResult{KeyAccessObjectID: kao.GetKeyAccessObjectId(), Error: err, RequiredObligations: requiredObligationsForKAO})
				} else {
					kaoKeys = append(kaoKeys, kaoResult{KeyAccessObjectID: kao.GetKeyAccessObjectId(), SymmetricKey: key, RequiredObligations: requiredObligationsForKAO})
				}
			} else {
				kaoKeys = append(kaoKeys, kaoResult{KeyAccessObjectID: kao.GetKeyAccessObjectId(), Error: errors.New(kao.GetError()), RequiredObligations: requiredObligationsForKAO})
			}
		}
		policyResults[results.GetPolicyId()] = kaoKeys
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
func (k *KASClient) getRewrapRequest(reqs []*kas.UnsignedRewrapRequest_WithPolicyRequest, pubKey string) (*kas.RewrapRequest, error) {
	if len(reqs) == 0 {
		return nil, errors.New("no requests provided")
	}
	if k.signer == nil {
		return nil, ErrNoAccessTokenSource
	}
	requestBody := &kas.UnsignedRewrapRequest{
		ClientPublicKey: pubKey,
		Requests:        reqs,
	}
	if len(reqs) == 1 && len(reqs[0].GetKeyAccessObjects()) == 1 && k.supportSingleRewrapEndpoint {
		requestBody.KeyAccess = reqs[0].GetKeyAccessObjects()[0].GetKeyAccessObject()
		requestBody.Policy = reqs[0].GetPolicy().GetBody()
		requestBody.Algorithm = reqs[0].GetAlgorithm()
	}

	requestBodyJSON, err := protojson.Marshal(requestBody)
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

	rewrapRequest := kas.RewrapRequest{
		SignedRequestToken: signedToken,
	}
	return &rewrapRequest, nil
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

	conn, call := s.client.newServiceClient(parsedURL, nil)
	serviceClient := kas.NewAccessServiceClient(conn)

	req := kas.PublicKeyRequest{
		Algorithm: algorithm,
	}
	if s.cfg.tdfFeatures.noKID {
		req.V = "1"
	}
	resp, err := serviceClient.PublicKey(ctx, &req)
	if err != nil {
		return nil, fmt.Errorf("error making request to KAS: %w", call.serviceError("PublicKey", err))
	}

	kid := resp.GetKid()
	if s.cfg.tdfFeatures.noKID {
		kid = ""
	}

	ki := KASInfo{
		URL:       kasurl,
		Algorithm: algorithm,
		KID:       kid,
		PublicKey: resp.GetPublicKey(),
	}
	if s.kasKeyCache != nil {
		s.kasKeyCache.store(ki)
	}
	return &ki, nil
}

func wipe(data []byte) {
	clear(data)
}
