package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/connect"
	"github.com/eugenioenko/goalchemy/std/encoding/jsonvalue"
	"github.com/eugenioenko/goalchemy/std/encoding/protojson"
	"github.com/eugenioenko/goalchemy/std/strings"
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

// platformService reports whether service is one the reference SDK has a
// client for (sdkconnect) and whether it is reached through the entity
// resolution connection rather than the platform's.
func platformService(service string) (known bool, entityResolution bool) {
	switch service {
	case "policy.actions.ActionService",
		"policy.attributes.AttributesService",
		"authorization.AuthorizationService",
		"authorization.v2.AuthorizationService",
		"policy.kasregistry.KeyAccessServerRegistryService",
		"policy.keymanagement.KeyManagementService",
		"policy.namespaces.NamespaceService",
		"policy.obligations.Service",
		"policy.registeredresources.RegisteredResourcesService",
		"policy.resourcemapping.ResourceMappingService",
		"policy.subjectmapping.SubjectMappingService",
		"policy.dynamicvaluemapping.DynamicValueMappingService",
		"policy.unsafe.UnsafeService",
		"wellknownconfiguration.WellKnownService":
		return true, false
	case "entityresolution.EntityResolutionService",
		"entityresolution.v2.EntityResolutionService":
		return true, true
	}
	return false, false
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

// serviceEndpoint is the base URL of service's connection: the reference's
// platformConn, or ersConn for the entity resolution services.
func (s *SDK) serviceEndpoint(entityResolution bool) string {
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
	known, entityResolution := platformService(service)
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
	response, err := callJSON(ctx, s.client, s.serviceEndpoint(entityResolution), "/"+service+"/"+method, value, "Call")
	if err != nil {
		return "", err
	}
	out, err := jsonvalue.Encode(response)
	if err != nil {
		return "", boundaryError(KindTransport, "Call", err)
	}
	return string(out), nil
}
