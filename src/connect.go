package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/connect"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/encoding/protojson"
	"github.com/eugenioenko/goalchemy/std/strings"
	"opentdf-local/sdk/src/protocol/authorization"
	"opentdf-local/sdk/src/protocol/policy/attributes"
)

const healthProcedure = "/grpc.health.v1.Health/Check"

// serviceCall is one Connect call's view of the SDK's transport. It records
// the HTTP status and the transport's own failure, which a connect.Error does
// not carry, so the SDK's Error keeps them.
type serviceCall struct {
	client *httpClient
	status int
	err    error
}

func (c *serviceCall) Do(ctx context.Context, method, url string, headers []string, body []byte, maxBytes int, timeoutMillis int64) (int, []string, []byte, error) {
	status, responseHeaders, data, err := c.client.Do(ctx, method, url, headers, body, maxBytes, timeoutMillis)
	c.status, c.err = status, err
	return status, responseHeaders, data, err
}

// newServiceClient returns a Connect client for the server at baseURL whose
// calls go through the SDK's authenticated transport, as the reference's
// service clients share its http.Client and interceptors. headers are sent
// with every call of the client.
func (c *httpClient) newServiceClient(baseURL string, headers []string) (*connect.Client, *serviceCall) {
	call := &serviceCall{client: c}
	return &connect.Client{
		BaseURL:          strings.TrimSuffix(baseURL, "/"),
		Headers:          headers,
		TimeoutMillis:    c.timeoutMillis,
		MaxResponseBytes: maxResponseBytes,
		HTTPClient:       call,
	}, call
}

// serviceError is the SDK's Error for a failed call: the transport's own
// Error unchanged, a transport failure with its text, or the service's
// Connect error worded as connect-go's ("<code>: <message>").
func (c *serviceCall) serviceError(op string, err error) *Error {
	if c.err != nil {
		if e, ok := c.err.(*Error); ok {
			if e.Op == "" {
				out := *e
				out.Op = op
				return &out
			}
			return e
		}
		return wrapError(KindTransport, op, "", c.err)
	}
	ce, ok := err.(*connect.Error)
	if !ok {
		return wrapError(KindTransport, op, "", err)
	}
	return &Error{Kind: serviceErrorKind(ce.Code, op, c.status), Op: op, Message: ce.Error(), HTTPStatus: c.status, ConnectCode: ce.Code.String(), ServiceMessage: ce.Message}
}

// serviceErrorKind classifies a Connect code. A reply the client could not
// read (status 200, or no reply) is a transport failure whatever its code;
// KAS calls keep the kinds the reader reports for rewrap failures, and other
// services get the generic ones.
func serviceErrorKind(code connect.Code, op string, status int) ErrorKind {
	kas := op == "Rewrap" || op == "PublicKey"
	switch {
	case code == connect.CodeCanceled:
		return KindCanceled
	case code == connect.CodeDeadlineExceeded:
		return KindDeadlineExceeded
	case status == 0 || status == 200:
		return KindTransport
	}
	switch code {
	case connect.CodeUnauthenticated:
		return KindAuth
	case connect.CodePermissionDenied:
		if kas {
			return KindRewrapForbidden
		}
		return KindAuth
	case connect.CodeInvalidArgument:
		if kas {
			return KindKASRequest
		}
		return KindInvalidInput
	case connect.CodeNotFound, connect.CodeAlreadyExists, connect.CodeFailedPrecondition, connect.CodeOutOfRange:
		if kas {
			return KindKASRequest
		}
		return KindInvalidInput
	}
	return KindTransport
}

// jsonMessage is any protojson object: the request and response of a call
// the shared source does not type, which the host decodes with its own
// generated messages.
type jsonMessage struct {
	value jsonvalue.Value
}

func (m *jsonMessage) EncodeProtoJSON(e *protojson.Encoder) { e.Struct(m.value) }

func (m *jsonMessage) DecodeProtoJSON(d *protojson.Decoder, v jsonvalue.Value) {
	m.value = d.Struct(v, "")
}

// callJSON sends one Connect unary call of procedure with a JSON object
// request through the SDK's transport and returns the JSON object response.
func callJSON(ctx context.Context, client *httpClient, baseURL, procedure string, request jsonvalue.Value, op string) (jsonvalue.Value, error) {
	conn, call := client.newServiceClient(baseURL, nil)
	response := &jsonMessage{}
	if _, err := conn.CallUnary(ctx, procedure, &jsonMessage{value: request}, response, nil); err != nil {
		return jsonvalue.Value{}, call.serviceError(op, err)
	}
	return response.value, nil
}

// The connections the reference's services share: the core (platform)
// connection, and the per-service overrides of WithCustom*Connection.
const (
	coreServices             = ""
	policyServices           = "policy"
	authorizationServices    = "authorization"
	entityResolutionServices = "entityresolution"
	wellknownServices        = "wellknownconfiguration"
)

// platformService reports whether service is one the reference SDK has a
// client for (sdkconnect) and which connection reaches it.
func platformService(service string) (connection string, known bool) {
	switch service {
	case "policy.actions.ActionService",
		"policy.attributes.AttributesService",
		"policy.kasregistry.KeyAccessServerRegistryService",
		"policy.keymanagement.KeyManagementService",
		"policy.namespaces.NamespaceService",
		"policy.obligations.Service",
		"policy.registeredresources.RegisteredResourcesService",
		"policy.resourcemapping.ResourceMappingService",
		"policy.subjectmapping.SubjectMappingService",
		"policy.dynamicvaluemapping.DynamicValueMappingService",
		"policy.unsafe.UnsafeService":
		return policyServices, true
	case "authorization.AuthorizationService",
		"authorization.v2.AuthorizationService":
		return authorizationServices, true
	case "entityresolution.EntityResolutionService",
		"entityresolution.v2.EntityResolutionService":
		return entityResolutionServices, true
	case "wellknownconfiguration.WellKnownService":
		return wellknownServices, true
	}
	return "", false
}

func isMethodName(method string) bool {
	if method == "" {
		return false
	}
	for i := 0; i < len(method); i++ {
		c := method[i]
		if !(c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_' || i > 0 && c >= '0' && c <= '9') {
			return false
		}
	}
	return true
}

// serviceEndpoint is the base URL of a connection: its WithCustom*Connection
// override, else the core connection's, else the platform endpoint, as the
// reference's ersConn falls back to its platformConn.
func (s *SDK) serviceEndpoint(connection string) string {
	override := ""
	switch connection {
	case policyServices:
		override = s.cfg.policyEndpoint
	case authorizationServices:
		override = s.cfg.authorizationEndpoint
	case entityResolutionServices:
		override = s.cfg.entityResolutionEndpoint
	case wellknownServices:
		override = s.cfg.wellknownEndpoint
	}
	if override != "" {
		return override
	}
	if s.cfg.coreEndpoint != "" {
		return s.cfg.coreEndpoint
	}
	return s.platformEndpoint
}

// Call sends one unary request to a method of a platform service, as the
// reference's sdkconnect clients do, through the SDK's authenticated
// transport. service is the protobuf service name, such as
// "policy.attributes.AttributesService"; request and the result are the
// messages in protobuf JSON. Host adapters type both with their own
// generated protobuf messages. A failure the service reports is an Error
// with its ConnectCode, ServiceMessage and HTTPStatus.
func (s *SDK) Call(ctx context.Context, service, method, request string) (string, error) {
	if err := s.check("Call"); err != nil {
		return "", err
	}
	connection, known := platformService(service)
	if !known {
		return "", newError(KindInvalidInput, "Call", "unknown service: "+service)
	}
	if !isMethodName(method) {
		return "", newError(KindInvalidInput, "Call", "invalid method name: "+method)
	}
	body := strings.TrimSpace(request)
	if body == "" {
		body = "{}"
	}
	value, err := jsonvalue.Parse([]byte(body))
	if err != nil || value.Kind() != jsonvalue.KindObject {
		return "", newError(KindInvalidInput, "Call", "request is not a JSON object")
	}
	response, err := callJSON(ctx, s.client, s.serviceEndpoint(connection), "/"+service+"/"+method, value, "Call")
	if err != nil {
		return "", err
	}
	out, err := jsonvalue.Encode(response)
	if err != nil {
		return "", boundaryError(KindTransport, "Call", err)
	}
	return string(out), nil
}

// attributesService is the part of the reference's
// sdkconnect.AttributesServiceClient the SDK's helpers call.
type attributesService interface {
	ListAttributes(ctx context.Context, req *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error)
	GetAttribute(ctx context.Context, req *attributes.GetAttributeRequest) (*attributes.GetAttributeResponse, error)
	GetAttributeValuesByFqns(ctx context.Context, req *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error)
	GetKeyMappingsByFqns(ctx context.Context, req *attributes.GetKeyMappingsByFqnsRequest) (*attributes.GetKeyMappingsByFqnsResponse, error)
}

// authorizationService is the part of the reference's
// sdkconnect.AuthorizationServiceClient the SDK's helpers call.
type authorizationService interface {
	GetEntitlements(ctx context.Context, req *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error)
}

// serviceConnection reaches the services at endpoint through the SDK's
// authenticated transport with the generated clients, one Connect client
// per call; a failure is the SDK's Error with the service's Connect code.
type serviceConnection struct {
	client   *httpClient
	endpoint string
}

func (c serviceConnection) ListAttributes(ctx context.Context, req *attributes.ListAttributesRequest) (*attributes.ListAttributesResponse, error) {
	conn, call := c.client.newServiceClient(c.endpoint, nil)
	resp, err := attributes.NewAttributesServiceClient(conn).ListAttributes(ctx, req)
	if err != nil {
		return nil, call.serviceError("ListAttributes", err)
	}
	return resp, nil
}

func (c serviceConnection) GetAttribute(ctx context.Context, req *attributes.GetAttributeRequest) (*attributes.GetAttributeResponse, error) {
	conn, call := c.client.newServiceClient(c.endpoint, nil)
	resp, err := attributes.NewAttributesServiceClient(conn).GetAttribute(ctx, req)
	if err != nil {
		return nil, call.serviceError("GetAttribute", err)
	}
	return resp, nil
}

func (c serviceConnection) GetAttributeValuesByFqns(ctx context.Context, req *attributes.GetAttributeValuesByFqnsRequest) (*attributes.GetAttributeValuesByFqnsResponse, error) {
	conn, call := c.client.newServiceClient(c.endpoint, nil)
	resp, err := attributes.NewAttributesServiceClient(conn).GetAttributeValuesByFqns(ctx, req)
	if err != nil {
		return nil, call.serviceError("GetAttributeValuesByFqns", err)
	}
	return resp, nil
}

func (c serviceConnection) GetKeyMappingsByFqns(ctx context.Context, req *attributes.GetKeyMappingsByFqnsRequest) (*attributes.GetKeyMappingsByFqnsResponse, error) {
	conn, call := c.client.newServiceClient(c.endpoint, nil)
	resp, err := attributes.NewAttributesServiceClient(conn).GetKeyMappingsByFqns(ctx, req)
	if err != nil {
		return nil, call.serviceError("GetKeyMappingsByFqns", err)
	}
	return resp, nil
}

func (c serviceConnection) GetEntitlements(ctx context.Context, req *authorization.GetEntitlementsRequest) (*authorization.GetEntitlementsResponse, error) {
	conn, call := c.client.newServiceClient(c.endpoint, nil)
	resp, err := authorization.NewAuthorizationServiceClient(conn).GetEntitlements(ctx, req)
	if err != nil {
		return nil, call.serviceError("GetEntitlements", err)
	}
	return resp, nil
}

// connectCodeOf is connect.CodeOf for the SDK's errors: the Connect code
// name the service replied with, or "".
func connectCodeOf(err error) string {
	if e, ok := asError(err); ok {
		return e.ConnectCode
	}
	return ""
}
