package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/io"
	"github.com/eugenioenko/goalchemy/std/log/slog"
	"opentdf-local/sdk/src/protocol/authorization"
	"opentdf-local/sdk/src/protocol/policy"
)

// Client is the implemented public method set of SDK. Wrappers and test doubles
// can implement it without depending on the concrete client.
type Client interface {
	// Close releases the client's resources. See SDK.Close.
	Close() error
	// IsHealthy checks the platform health endpoint with ctx cancellation.
	IsHealthy(ctx context.Context) (bool, error)
	// PlatformConfiguration returns the discovered platform configuration.
	PlatformConfiguration() PlatformConfiguration
	// Logger returns the client's logger.
	Logger() *slog.Logger
	// CreateTDF encrypts reader into writer. The current implementation buffers the
	// payload.
	CreateTDF(ctx context.Context, writer io.Writer, reader io.Reader, opts TDFOptions) (*TDFObject, error)
	// EncryptBytes returns an encrypted archive and its manifest summary.
	EncryptBytes(ctx context.Context, plaintext []byte, opts TDFOptions) ([]byte, *TDFObject, error)
	// LoadTDF loads an archive into a reader without unwrapping its payload key yet.
	LoadTDF(ctx context.Context, reader io.ReadSeeker, opts ReaderOptions) (*Reader, error)
	// DecryptBytes returns verified plaintext from an in-memory archive.
	DecryptBytes(ctx context.Context, ciphertext []byte, opts ReaderOptions) ([]byte, error)
	// GetBaseKey returns the base key advertised by platform configuration.
	GetBaseKey(ctx context.Context) (SimpleKasKey, error)
	// StoreKASKeys updates cached public keys for a KAS endpoint.
	StoreKASKeys(url string, keys KasPublicKeySet) error
	// Call makes an authenticated unary service call using protobuf JSON.
	Call(ctx context.Context, service, method, request string) (string, error)
	// Conn returns the core service connection descriptor.
	Conn() ConnectRPCConnection
	// Auth returns the client's authentication helper.
	Auth() *AuthClient
	// ListAttributes lists policy attributes, optionally filtered by namespace.
	ListAttributes(ctx context.Context, namespace ...string) ([]*policy.Attribute, error)
	// GetEntityAttributes resolves the attributes associated with an entity.
	GetEntityAttributes(ctx context.Context, entity *authorization.Entity) ([]string, error)
	// ValidateAttributes checks that the supplied attribute FQNs exist.
	ValidateAttributes(ctx context.Context, fqns []string) error
	// AttributeExists reports whether an attribute definition exists.
	AttributeExists(ctx context.Context, attributeFqn string) (bool, error)
	// AttributeValueExists reports whether an attribute value exists.
	AttributeValueExists(ctx context.Context, fqn string) (bool, error)
}

// TDFReader is the implemented public method set of Reader. Payload streaming
// and bulk operations are not part of this interface.
type TDFReader interface {
	// Init unwraps the payload key through KAS using ctx for cancellation.
	Init(ctx context.Context) error
	// Manifest returns the parsed manifest without decrypting the payload.
	Manifest() Manifest
	// Policy decodes the manifest's policy without authorizing access.
	Policy() (PolicyObject, error)
	// DataAttributes returns the attribute FQNs recorded in the policy.
	DataAttributes() ([]string, error)
	// UnencryptedMetadata decrypts and returns metadata, initializing key access if
	// needed.
	UnencryptedMetadata() ([]byte, error)
	// Close releases reader-owned keys and buffered payload data.
	Close() error
}

var _ Client = (*SDK)(nil)

var _ TDFReader = (*Reader)(nil)
