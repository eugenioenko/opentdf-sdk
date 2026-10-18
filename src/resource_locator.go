package sdk

import (
	"github.com/eugenioenko/goalchemy/std/errors"
	"github.com/eugenioenko/goalchemy/std/fmt"
	"github.com/eugenioenko/goalchemy/std/io"
	"github.com/eugenioenko/goalchemy/std/strconv"
	"github.com/eugenioenko/goalchemy/std/strings"
)

// ============================================================================================================
// Support for serializing/deserializing URLs in the compact/binary encoding
//
// If an URL is specified as "https://some.site.com/endpoint"
// the storage format for this is to strip off the leading "https://" prefix and encode as 0 (or 1 for http)
// followed by the run-length-prefixed body value
// ============================================================================================================

// ResourceLocator - structure to contain a protocol + body comprising an URL
type ResourceLocator struct {
	protocol protocolHeader // See protocolHeader values below
	// body URL without protocol scheme
	body string
	// identifier unique to this URL
	identifier string
}

// protocolHeader - shorthand for protocol prefix on fully qualified url
// also specifies the optional resource identifier - current usage is a key identifier
type protocolHeader uint8

func (h protocolHeader) identifierLength() int {
	switch h & 0xF0 {
	case identifierNone, urlProtocolHTTPS:
		return identifierNoneLength
	case identifier2Byte:
		return identifier2ByteLength
	case identifier8Byte:
		return identifier8ByteLength
	case identifier32Byte:
		return identifier32ByteLength
	default:
		return 0
	}
}

const (
	kMaxBodyLen int = 255
	// kPrefixHTTPS identifier field is size of 0 bytes (not present)
	kPrefixHTTPS     string         = "https://"
	kPrefixHTTP      string         = "http://"
	urlProtocolHTTP  protocolHeader = 0x0
	urlProtocolHTTPS protocolHeader = 0x1
	// identifier
	identifierNone   protocolHeader = 0 << 4
	identifier2Byte  protocolHeader = 1 << 4
	identifier8Byte  protocolHeader = 2 << 4
	identifier32Byte protocolHeader = 3 << 4
	// length
	identifierNoneLength   int = 0
	identifier2ByteLength  int = 2
	identifier8ByteLength  int = 8
	identifier32ByteLength int = 32
)

// NewResourceLocator parses an HTTP or HTTPS URL into a compact resource locator.
// Invalid input returns KindInvalidInput.
func NewResourceLocator(url string) (*ResourceLocator, error) {
	rl, err := newResourceLocator(url)
	if err != nil {
		return nil, boundaryError(KindInvalidInput, "NewResourceLocator", err)
	}
	return rl, nil
}

func newResourceLocator(url string) (*ResourceLocator, error) {
	rl := &ResourceLocator{}

	err := rl.setURL(url)
	if err != nil {
		return nil, err
	}

	return rl, err
}

// NewResourceLocatorFromReader reads one binary resource locator from reader.
// Malformed or truncated input returns KindInvalidInput.
func NewResourceLocatorFromReader(reader io.Reader) (*ResourceLocator, error) {
	rl := &ResourceLocator{}
	err := rl.readResourceLocator(reader)
	if err != nil {
		return nil, boundaryError(KindInvalidInput, "NewResourceLocatorFromReader", err)
	}
	return rl, nil
}

// GetIdentifier - identifier is returned if the correct protocol enum is set else error
// padding is removed unlike rl.identifier direct access
func (rl ResourceLocator) GetIdentifier() (string, error) {
	// read the identifier if it exists
	switch rl.protocol & 0xf0 {
	case identifierNone, urlProtocolHTTPS:
		return "", fmt.Errorf("legacy resource locator identifer: %x", uint8(rl.protocol))
	case identifier2Byte, identifier8Byte, identifier32Byte:
		if rl.identifier == "" {
			return "", fmt.Errorf("no resource locator identifer: %d", uint8(rl.protocol))
		}
		// remove padding
		cleanedIdentifier := strings.TrimRight(rl.identifier, "\x00")
		return cleanedIdentifier, nil
	}
	return "", fmt.Errorf("unsupported identifer protocol: %x", uint8(rl.protocol))
}

// GetURL - Retrieve a fully qualified protocol+body URL string from a ResourceLocator struct
func (rl ResourceLocator) GetURL() (string, error) {
	switch rl.protocol & 0xF { // use bitwise AND to get first 4 bits
	case urlProtocolHTTPS, identifier2Byte, identifier8Byte, identifier32Byte:
		return kPrefixHTTPS + rl.body, nil
	case urlProtocolHTTP:
		return kPrefixHTTP + rl.body, nil
	default:
		return "", fmt.Errorf("unsupported protocol: %x", uint8(rl.protocol))
	}
}

// Less compares locators for the SDK's deterministic key ordering. y must not be nil.
func (rl ResourceLocator) Less(y *ResourceLocator) bool {
	pa := rl.protocol & 0xF
	pb := y.protocol & 0xF
	if pa != pb {
		return pa < pb
	}
	if rl.body < y.body {
		return true
	}
	if rl.identifier < y.identifier {
		return true
	}
	return false
}

// KASURI returns the URL without its key identifier, or an empty string for an empty
// locator.
func (rl ResourceLocator) KASURI() string {
	if rl.body == "" {
		return ""
	}
	switch rl.protocol & 0xF {
	case urlProtocolHTTPS, identifier2Byte, identifier8Byte, identifier32Byte:
		return kPrefixHTTPS + rl.body
	case urlProtocolHTTP:
		return kPrefixHTTP + rl.body
	default:
		return "unspecified://" + rl.body
	}
}

// String returns the KAS URL followed by # and the key identifier when present.
func (rl ResourceLocator) String() string {
	url := rl.KASURI()
	if rl.identifier == "" {
		return url
	}
	return fmt.Sprintf("%s#%s", url, rl.identifier)
}

// Equals compares the protocol, URL body, and key identifier.
func (rl ResourceLocator) Equals(r2 ResourceLocator) bool {
	if rl.protocol != r2.protocol {
		return false
	}
	if rl.body != r2.body {
		return false
	}
	if rl.identifier != r2.identifier {
		return false
	}
	return true
}

// ID returns the stored key identifier, including any binary-format padding.
func (rl ResourceLocator) ID() string {
	return rl.identifier
}

const protocolSharedRes = 0x4

// readResourceLocator - read the encoded protocol and body string into a ResourceLocator
func (rl *ResourceLocator) readResourceLocator(reader io.Reader) error {
	header := make([]byte, 1)
	if _, err := io.ReadFull(reader, header); err != nil {
		return errors.Join(errors.New("Error reading ResourceLocator protocol value"), err)
	}
	rl.protocol = protocolHeader(header[0])
	if (rl.protocol&0x0f != urlProtocolHTTP) && (rl.protocol&0x0f != urlProtocolHTTPS) {
		return errors.New("Unsupported protocol: " + strconv.Itoa(int(rl.protocol)))
	}
	lengthBody := make([]byte, 1)
	if _, err := io.ReadFull(reader, lengthBody); err != nil {
		return errors.Join(errors.New("Error reading ResourceLocator body length value"), err)
	}
	body := make([]byte, lengthBody[0])
	if _, err := io.ReadFull(reader, body); err != nil {
		return errors.Join(errors.New("Error reading ResourceLocator body value"), err)
	}
	rl.body = string(body)
	// read the identifier if it exists
	switch rl.protocol & 0xf0 {
	case identifierNone, urlProtocolHTTPS:
		// noop
	case identifier2Byte:
		identifier := make([]byte, identifier2ByteLength)
		if _, err := io.ReadFull(reader, identifier); err != nil {
			return errors.New("Error reading ResourceLocator identifier value: " + err.Error())
		}
		rl.identifier = string(identifier)
	case identifier8Byte:
		identifier := make([]byte, identifier8ByteLength)
		if _, err := io.ReadFull(reader, identifier); err != nil {
			return errors.New("Error reading ResourceLocator identifier value: " + err.Error())
		}
		rl.identifier = string(identifier)
	case identifier32Byte:
		identifier := make([]byte, identifier32ByteLength)
		if _, err := io.ReadFull(reader, identifier); err != nil {
			return errors.New("Error reading ResourceLocator identifier value: " + err.Error())
		}
		rl.identifier = string(identifier)
	case protocolSharedRes:
		// noop for legacy relative file references
	default:
		return errors.New("unsupported identifier protocol: " + strconv.Itoa(int(rl.protocol)))
	}
	return nil
}

// setURL - Store a fully qualified protocol+body string into a ResourceLocator as a protocol value and a body string
func (rl *ResourceLocator) setURL(url string) error {
	lowerURL := strings.ToLower(url)
	if strings.HasPrefix(lowerURL, kPrefixHTTPS) {
		urlBody := url[len(kPrefixHTTPS):]
		if len(urlBody) > kMaxBodyLen {
			return errors.New("URL too long")
		}
		rl.protocol = urlProtocolHTTPS
		rl.body = urlBody
		return nil
	}
	if strings.HasPrefix(lowerURL, kPrefixHTTP) {
		urlBody := url[len(kPrefixHTTP):]
		if len(urlBody) > kMaxBodyLen {
			return errors.New("URL too long")
		}
		rl.protocol = urlProtocolHTTP
		rl.body = urlBody
		return nil
	}
	return errors.New("unsupported protocol: " + url)
}

// writeResourceLocator - writes the content of the resource locator to the supplied writer
func (rl ResourceLocator) writeResourceLocator(writer io.Writer) error {
	if _, err := writer.Write([]byte{byte(rl.protocol)}); err != nil {
		return err
	}

	if _, err := writer.Write([]byte{byte(len(rl.body))}); err != nil {
		return err
	}

	if _, err := writer.Write([]byte(rl.body)); err != nil {
		return err
	}
	// identifier
	if len(rl.identifier) > 0 {
		if _, err := writer.Write([]byte(rl.identifier)); err != nil {
			return err
		}
	}

	return nil
}

// setURLWithIdentifier - Store a fully qualified protocol+body string and an identifier into a ResourceLocator.
func (rl *ResourceLocator) setURLWithIdentifier(url string, identifier string) error {
	if identifier == "" {
		return errors.New("identifier is empty")
	}
	lowerURL := strings.ToLower(url)

	if strings.HasPrefix(lowerURL, kPrefixHTTPS) {
		return rl.setURLParts(url[len(kPrefixHTTPS):], identifier, urlProtocolHTTPS)
	}
	if strings.HasPrefix(lowerURL, kPrefixHTTP) {
		return rl.setURLParts(url[len(kPrefixHTTP):], identifier, urlProtocolHTTP)
	}
	return errors.New("unsupported protocol with identifier: " + url)
}

func (rl *ResourceLocator) setURLParts(urlBody, identifier string, baseProtocol protocolHeader) error {
	if len(urlBody) > kMaxBodyLen {
		return errors.New("URL too long")
	}

	identifierLen := len(identifier)
	var idProtocol protocolHeader
	var paddingLen int

	switch {
	case identifierLen <= identifier2ByteLength:
		idProtocol = identifier2Byte
		paddingLen = identifier2ByteLength - identifierLen
	case identifierLen > identifier2ByteLength && identifierLen <= identifier8ByteLength:
		idProtocol = identifier8Byte
		paddingLen = identifier8ByteLength - identifierLen
	case identifierLen > identifier8ByteLength && identifierLen <= identifier32ByteLength:
		idProtocol = identifier32Byte
		paddingLen = identifier32ByteLength - identifierLen
	default:
		return fmt.Errorf("unsupported identifier length: %d", identifierLen)
	}

	rl.protocol = baseProtocol | idProtocol
	rl.body = urlBody
	rl.identifier = identifier + strings.Repeat("\x00", paddingLen)
	return nil
}
