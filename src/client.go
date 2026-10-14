package sdk

import (
	"github.com/eugenioenko/goalchemy/lib/context"
	"github.com/eugenioenko/goalchemy/std/io"
	"github.com/eugenioenko/goalchemy/std/log/slog"
)

// Client is the public composition point: the method set of *SDK that this
// phase implements. Wrappers and test doubles implement it; Call carries
// every platform service client, and the streaming and bulk methods join it
// in Phases 7 and 9.
type Client interface {
	Close() error
	IsHealthy(ctx context.Context) (bool, error)
	PlatformConfiguration() PlatformConfiguration
	Logger() *slog.Logger
	CreateTDF(ctx context.Context, writer io.Writer, reader io.Reader, opts TDFOptions) (*TDFObject, error)
	EncryptBytes(ctx context.Context, plaintext []byte, opts TDFOptions) ([]byte, *TDFObject, error)
	LoadTDF(ctx context.Context, reader io.ReadSeeker, opts ReaderOptions) (*Reader, error)
	DecryptBytes(ctx context.Context, ciphertext []byte, opts ReaderOptions) ([]byte, error)
	GetBaseKey(ctx context.Context) (SimpleKasKey, error)
	StoreKASKeys(url string, keys KasPublicKeySet) error
	Call(ctx context.Context, service, method, request string) (string, error)
}

// TDFReader is the method set of *Reader that this phase implements; the
// stream methods join it in Phase 7 and Obligations in Phase 9.
type TDFReader interface {
	Init(ctx context.Context) error
	Manifest() Manifest
	Policy() (PolicyObject, error)
	DataAttributes() ([]string, error)
	UnencryptedMetadata() ([]byte, error)
	Close() error
}

var _ Client = (*SDK)(nil)

var _ TDFReader = (*Reader)(nil)
