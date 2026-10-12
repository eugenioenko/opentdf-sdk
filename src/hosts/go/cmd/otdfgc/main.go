// Command otdfgc is the native Go CLI over the generated OpenTDF package. It
// implements the xtest CLI contract of docs/full-sdk-plan.md over the handle
// API: one SDK per invocation, created with --host and the client
// credentials; every flag this build cannot honour exits 2 as unsupported.
package main

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"time"

	g "goalchemyout"
)

const (
	exitSuccess = 0
	exitError   = 1
	exitUsage   = 2
	target      = "go"
	timeout     = 120 * time.Second
)

const usage = `Usage:
  otdfgc encrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--attr <fqn,...>] [--mime-type <t>] [--wrapping-key-algorithm rsa:2048|ec:secp256r1]
      [--target-mode 4.3.0] [--policy-mode encrypted] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc decrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--no-verify-assertions] [--session-key-algorithm rsa:2048|ec:secp256r1]
      [--kas-allowlist <url,...|*>] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc inspect <in>
  otdfgc supports <feature>
  otdfgc --version [--json]
`

func main() {
	os.Exit(run(os.Args[1:], os.Stdout, os.Stderr))
}

func run(args []string, stdout, stderr *os.File) int {
	if len(args) == 0 {
		fmt.Fprint(stderr, usage)
		return exitUsage
	}
	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()
	switch args[0] {
	case "--version", "version":
		return version(ctx, args[1:], stdout, stderr)
	case "supports":
		return supports(ctx, args[1:], stdout, stderr)
	case "inspect":
		return inspect(args[1:], stdout, stderr)
	case "encrypt":
		return encrypt(ctx, args[1:], stderr)
	case "decrypt":
		return decrypt(ctx, args[1:], stderr)
	case "help", "--help", "-h":
		fmt.Fprint(stdout, usage)
		return exitSuccess
	}
	fmt.Fprintf(stderr, "unknown command: %s\n%s", args[0], usage)
	return exitUsage
}

type versionInfo struct {
	SDKVersion        string   `json:"sdk_version"`
	SchemaVersion     string   `json:"schema_version"`
	SupportedFeatures []string `json:"supported_features"`
	Target            string   `json:"target"`
}

func versionInfoOf(ctx context.Context) (versionInfo, error) {
	v, err := g.Version(ctx)
	if err != nil {
		return versionInfo{}, err
	}
	s, err := g.TDFSpecVersion(ctx)
	if err != nil {
		return versionInfo{}, err
	}
	f, err := g.SupportedFeatures(ctx)
	if err != nil {
		return versionInfo{}, err
	}
	if f == nil {
		f = []string{}
	}
	return versionInfo{SDKVersion: v, SchemaVersion: s, SupportedFeatures: f, Target: target}, nil
}

func version(ctx context.Context, args []string, stdout, stderr *os.File) int {
	asJSON := false
	for _, a := range args {
		switch a {
		case "--json":
			asJSON = true
		default:
			fmt.Fprintf(stderr, "unknown flag: %s\n", a)
			return exitUsage
		}
	}
	info, err := versionInfoOf(ctx)
	if err != nil {
		fmt.Fprintf(stderr, "Failed to read version: %s\n", describe(err))
		return exitError
	}
	if asJSON {
		data, _ := json.Marshal(info)
		fmt.Fprintln(stdout, string(data))
		return exitSuccess
	}
	fmt.Fprintf(stdout, "otdfgc (%s) sdk %s tdf-spec %s features %s\n", target, info.SDKVersion, info.SchemaVersion, strings.Join(info.SupportedFeatures, ","))
	return exitSuccess
}

func supports(ctx context.Context, args []string, stdout, stderr *os.File) int {
	if len(args) != 1 || strings.HasPrefix(args[0], "-") {
		fmt.Fprintf(stderr, "usage: otdfgc supports <feature>\n")
		return exitUsage
	}
	features, err := g.SupportedFeatures(ctx)
	if err != nil {
		fmt.Fprintf(stderr, "Failed to read supported features: %s\n", describe(err))
		return exitError
	}
	for _, f := range features {
		if f == args[0] {
			return exitSuccess
		}
	}
	fmt.Fprintf(stdout, "%s unsupported\n", args[0])
	return exitError
}

func inspect(args []string, stdout, stderr *os.File) int {
	if len(args) != 1 || strings.HasPrefix(args[0], "-") {
		fmt.Fprintf(stderr, "usage: otdfgc inspect <in>\n")
		return exitUsage
	}
	manifest, err := readManifest(args[0])
	if err != nil {
		fmt.Fprintf(stderr, "not a valid TDF: %s\n", err)
		return exitError
	}
	var pretty json.RawMessage
	if err := json.Unmarshal(manifest, &pretty); err != nil {
		fmt.Fprintf(stderr, "failed to inspect TDF: %s\n", err)
		return exitError
	}
	data, err := json.MarshalIndent(pretty, "", "  ")
	if err != nil {
		fmt.Fprintf(stderr, "failed to inspect TDF: %s\n", err)
		return exitError
	}
	fmt.Fprintln(stdout, string(data))
	return exitSuccess
}

// newSDK constructs the SDK from --host and the credentials, with otdfctl's
// wording when the platform cannot be reached or the credentials fail.
func newSDK(ctx context.Context, p *parsed, stderr *os.File) (*g.SDK, string, int) {
	host, opts, code, err := p.options()
	if err != nil {
		fmt.Fprintln(stderr, err)
		return nil, "", code
	}
	sdk, err := g.NewWithOptions(ctx, host, opts...)
	if err != nil {
		fmt.Fprintln(stderr, describeNew(err, host))
		return nil, "", exitError
	}
	return sdk, host, exitSuccess
}

func encrypt(ctx context.Context, args []string, stderr *os.File) int {
	p, err := parseFlags(args, encryptFlags)
	if err != nil {
		fmt.Fprintf(stderr, "%s\n%s", err, usage)
		return exitUsage
	}
	if reason := p.unsupported(encryptUnsupported); reason != "" {
		fmt.Fprintf(stderr, "unsupported in this build: %s\n", reason)
		return exitUsage
	}
	if len(p.positional) != 1 || p.value("out") == "" {
		fmt.Fprintf(stderr, "encrypt needs exactly one input file and --out\n%s", usage)
		return exitUsage
	}
	input, err := os.ReadFile(p.positional[0])
	if err != nil {
		fmt.Fprintf(stderr, "Failed to read %s: %s\n", p.positional[0], err)
		return exitError
	}
	sdk, host, code := newSDK(ctx, p, stderr)
	if code != exitSuccess {
		return code
	}
	defer sdk.Close()
	// As otdfctl: the platform's KAS is the default key access server and
	// attributes drive autoconfigure.
	tdfOpts := []g.TDFOption{g.WithKasInformation(g.KASInfo{URL: host + "/kas"}), g.WithMimeType(p.value("mime-type"))}
	if attr := p.value("attr"); attr != "" {
		tdfOpts = append(tdfOpts, g.WithDataAttributes(splitList(attr)...))
	}
	if wrapping := p.value("wrapping-key-algorithm"); wrapping != "" {
		tdfOpts = append(tdfOpts, g.WithWrappingKeyAlg(g.KeyType(wrapping)))
	}
	if mode := p.value("target-mode"); mode != "" {
		tdfOpts = append(tdfOpts, g.WithTargetMode(mode))
	}
	options, err := g.TDFOptionsOf(tdfOpts...)
	if err != nil {
		fmt.Fprintf(stderr, "Failed to encrypt: %s\n", err)
		return exitError
	}
	data, obj, err := sdk.EncryptBytes(ctx, input, options)
	if err != nil {
		fmt.Fprintf(stderr, "Failed to encrypt: %s\n", describe(err))
		return exitError
	}
	obj.Close()
	if err := writeFile(p.value("out"), data, 0o644); err != nil {
		fmt.Fprintf(stderr, "Failed to write encrypted file %s: %s\n", p.value("out"), err)
		return exitError
	}
	return exitSuccess
}

func decrypt(ctx context.Context, args []string, stderr *os.File) int {
	p, err := parseFlags(args, decryptFlags)
	if err != nil {
		fmt.Fprintf(stderr, "%s\n%s", err, usage)
		return exitUsage
	}
	if reason := p.unsupported(decryptUnsupported); reason != "" {
		fmt.Fprintf(stderr, "unsupported in this build: %s\n", reason)
		return exitUsage
	}
	if len(p.positional) != 1 || p.value("out") == "" {
		fmt.Fprintf(stderr, "decrypt needs exactly one input file and --out\n%s", usage)
		return exitUsage
	}
	input, err := os.ReadFile(p.positional[0])
	if err != nil {
		fmt.Fprintf(stderr, "Failed to read file: %s\n", err)
		return exitError
	}
	sdk, _, code := newSDK(ctx, p, stderr)
	if code != exitSuccess {
		return code
	}
	defer sdk.Close()
	readerOpts := []g.TDFReaderOption{g.WithDisableAssertionVerification(p.flag("no-verify-assertions"))}
	if session := p.value("session-key-algorithm"); session != "" {
		readerOpts = append(readerOpts, g.WithSessionKeyType(g.KeyType(session)))
	}
	// otdfctl: "*" alone ignores the allowlist; any other list is explicit.
	if allowlist := splitList(p.value("kas-allowlist")); len(allowlist) == 1 && allowlist[0] == "*" {
		readerOpts = append(readerOpts, g.WithIgnoreAllowlist(true))
	} else if len(allowlist) > 0 {
		readerOpts = append(readerOpts, g.WithKasAllowlist(allowlist))
	}
	options, err := g.ReaderOptionsOf(readerOpts...)
	if err != nil {
		fmt.Fprintf(stderr, "Failed to decrypt file: %s\n", err)
		return exitError
	}
	plaintext, err := sdk.DecryptBytes(ctx, input, options)
	if err != nil {
		fmt.Fprintf(stderr, "Failed to decrypt file: %s\n", describe(err))
		return exitError
	}
	if err := writeFile(p.value("out"), plaintext, 0o600); err != nil {
		fmt.Fprintf(stderr, "Failed to write decrypted data to file: %s\n", err)
		return exitError
	}
	return exitSuccess
}

func writeFile(path string, data []byte, mode os.FileMode) error {
	tmp := path + ".tmp"
	if err := os.WriteFile(tmp, data, mode); err != nil {
		return err
	}
	if err := os.Rename(tmp, path); err != nil {
		os.Remove(tmp)
		return err
	}
	return nil
}

func splitList(s string) []string {
	var out []string
	for _, part := range strings.Split(s, ",") {
		if part = strings.TrimSpace(part); part != "" {
			out = append(out, part)
		}
	}
	return out
}
