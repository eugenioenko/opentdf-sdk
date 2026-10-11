// Command otdfgc is the native Go CLI over the generated TDF3 package. It
// implements the xtest CLI contract of docs/full-sdk-plan.md with the flags
// the current façade can honour; every other flag exits 2 as unsupported.
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
		fmt.Fprintf(stderr, "Failed to read version: %s\n", describe(err, ""))
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
		fmt.Fprintf(stderr, "Failed to read supported features: %s\n", describe(err, ""))
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
	cfg, code, err := p.config()
	if err != nil {
		fmt.Fprintln(stderr, err)
		return code
	}
	wrapping := p.value("wrapping-key-algorithm")
	if wrapping == "" {
		wrapping = "rsa:2048"
	}
	cfg.KASAlgorithm = wrapping
	input, err := os.ReadFile(p.positional[0])
	if err != nil {
		fmt.Fprintf(stderr, "Failed to read %s: %s\n", p.positional[0], err)
		return exitError
	}
	options := g.EncryptOptions{MimeType: p.value("mime-type")}
	if attr := p.value("attr"); attr != "" {
		options.Attributes = splitList(attr)
	}
	if err := discover(ctx, &cfg, p.flag("tls-no-verify")); err != nil {
		fmt.Fprintln(stderr, err)
		return exitError
	}
	data, err := g.Encrypt(ctx, cfg, input, options)
	if err != nil {
		fmt.Fprintf(stderr, "Failed to encrypt: %s\n", describe(err, ""))
		return exitError
	}
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
	cfg, code, err := p.config()
	if err != nil {
		fmt.Fprintln(stderr, err)
		return code
	}
	session := p.value("session-key-algorithm")
	if session == "" {
		session = "rsa:2048"
	}
	cfg.SessionAlgorithm = session
	input, err := os.ReadFile(p.positional[0])
	if err != nil {
		fmt.Fprintf(stderr, "Failed to read file: %s\n", err)
		return exitError
	}
	kasURL := ""
	if manifest, err := readManifestBytes(input); err == nil {
		kasURL = manifestKASURL(manifest)
	}
	cfg.AllowedKAS = allowedRoutes(cfg.PlatformURL, p.value("kas-allowlist"), kasURL)
	if err := discover(ctx, &cfg, p.flag("tls-no-verify")); err != nil {
		fmt.Fprintln(stderr, err)
		return exitError
	}
	result, err := g.Decrypt(ctx, cfg, input)
	if err != nil {
		fmt.Fprintf(stderr, "Failed to decrypt file: %s\n", describe(err, kasURL))
		return exitError
	}
	if err := writeFile(p.value("out"), result.Payload, 0o600); err != nil {
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
