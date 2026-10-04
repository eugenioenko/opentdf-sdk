// Native-only independent SDK interoperability runner. No reference imports
// enter the shared SDK dependency graph. Secrets and tokens remain in memory.
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	reference "github.com/opentdf/platform/sdk"
	"io"
	"log/slog"
	client "opentdf-local/sdk/src"
	"opentdf-local/sdk/src/tdf"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"
)

const platform = "http://localhost:8080"
const kas = platform + "/kas"
const issuer = "http://localhost:8888/auth/realms/opentdf"
const allowed = "https://example.com/attr/attr1/value/value1"
const denied = "https://example.com/attr/attr1/value/value2"

func must[T any](v T, e error) T {
	if e != nil {
		panic(e)
	}
	return v
}
func check(e error) {
	if e != nil {
		panic(e)
	}
}
func cli(sdk, run, consumer, name string) {
	output := filepath.Join(run, name+"."+consumer+".out")
	if e := os.Remove(output); e != nil && !os.IsNotExist(e) {
		panic(e)
	}
	ctx, cancel := context.WithTimeout(context.Background(), 45*time.Second)
	defer cancel()
	var cmd *exec.Cmd
	if consumer == "go" {
		cmd = exec.CommandContext(ctx, filepath.Join(sdk, ".local", "bin", "otdfctl"), "decrypt", filepath.Join(run, name+".tdf"), "--session-key-algorithm", "rsa:2048", "--kas-allowlist", kas, "--out", filepath.Join(run, name+".go.out"), "--host", platform, "--with-client-creds-file", filepath.Join(sdk, ".local", "interop", "client-creds.json"))
	} else {
		cmd = exec.CommandContext(ctx, "node", filepath.Join(sdk, ".local", "web-cli", "bin", "opentdf.mjs"), "decrypt", filepath.Join(run, name+".tdf"), "--rewrapKeyType", "rsa:2048", "--allowList", platform, "--output", filepath.Join(run, name+".web.out"), "--platformUrl", platform, "--kasEndpoint", kas, "--oidcEndpoint", issuer, "--clientId", "opentdf-sdk", "--clientSecret", "secret", "--logLevel", "error")
	}
	// Capture logs without persisting or displaying possible sensitive diagnostics.
	if _, e := cmd.CombinedOutput(); e != nil {
		panic(fmt.Sprintf("reference %s decrypt %s: %v", consumer, name, e))
	}
}
func verifyReferences(sdk string) {
	lock := struct {
		Repositories map[string]struct {
			Path     string `json:"path"`
			Revision string `json:"revision"`
		} `json:"repositories"`
	}{}
	check(json.Unmarshal(must(os.ReadFile(filepath.Join(sdk, "references.lock.json"))), &lock))
	for _, name := range []string{"platform", "web-sdk"} {
		pin, ok := lock.Repositories[name]
		if !ok || pin.Path == "" || pin.Revision == "" {
			panic("missing reference lock: " + name)
		}
		repo := filepath.Join(sdk, pin.Path)
		revision := must(exec.Command("git", "-C", repo, "rev-parse", "HEAD").Output())
		if strings.TrimSpace(string(revision)) != pin.Revision {
			panic("reference revision mismatch: " + name)
		}
		changed := must(exec.Command("git", "-C", repo, "status", "--porcelain", "--untracked-files=no").Output())
		if len(changed) != 0 {
			panic("reference tracked source is changed: " + name)
		}
	}
}
func freshCommand(sdk string, args ...string) {
	ctx, cancel := context.WithTimeout(context.Background(), 45*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "node", append([]string{filepath.Join(sdk, "tests", "interop", "client", "web-fixture.mjs"), sdk}, args...)...)
	if _, e := cmd.CombinedOutput(); e != nil {
		panic(fmt.Sprintf("independent Web SDK fixture %s: %v", args[0], e))
	}
}
func compare(c *client.Client, data, expected, metadata []byte, label string) {
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	d := must(c.Decrypt(ctx, data))
	if !bytes.Equal(d.Payload, expected) || !bytes.Equal(d.Metadata, metadata) {
		panic("shared client plaintext/metadata: " + label)
	}
	fmt.Println("PASS real KAS reference -> client:", label)
}
func profile(data []byte, size int, alg, label string) {
	a := must(tdf.ReadArchive(data, tdf.DefaultArchiveLimits()))
	m := must(tdf.ParseManifest(a.Manifest))
	check(m.ValidateModern(len(a.Payload)))
	if m.ResolvedSegmentHashAlgorithm() != strings.ToLower(alg) {
		panic("actual segment algorithm: " + label)
	}
	if size == 0 {
		return
	} // Empty writer layouts intentionally differ (Go one, Web zero).
	expected := (size + 16383) / 16384
	if len(m.Encryption.Integrity.Segments) != expected {
		panic("actual segment count: " + label)
	}
	remaining := size
	for _, segment := range m.Encryption.Integrity.Segments {
		plain, cipher, e := m.Encryption.Integrity.ResolveSegment(segment)
		check(e)
		wanted := remaining
		if wanted > 16384 {
			wanted = 16384
		}
		if plain != wanted || cipher != wanted+28 {
			panic("actual segment boundary: " + label)
		}
		remaining -= plain
	}
	if remaining != 0 {
		panic("actual segment total: " + label)
	}
}
func noOutput(c *client.Client, data []byte, label string) {
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()
	d, e := c.Decrypt(ctx, data)
	if e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 {
		panic("failed open: " + label)
	}
	fmt.Println("PASS zero output:", label)
}
func main() {
	sdk := must(filepath.Abs("../../.."))
	verifyReferences(sdk)
	runName := os.Getenv("TDF_CLIENT_INTEROP_NAME")
	if runName == "" {
		runName = "interop-client"
	}
	for _, b := range []byte(runName) {
		if !(b >= 'a' && b <= 'z' || b >= '0' && b <= '9' || b == '-') {
			panic("invalid ignored interop directory name")
		}
	}
	run := filepath.Join(sdk, ".local", runName)
	check(os.MkdirAll(run, 0700))
	reportFile := filepath.Join(run, "results.json")
	if e := os.Remove(reportFile); e != nil && !os.IsNotExist(e) {
		panic(e)
	}
	c := must(client.New(client.Config{PlatformURL: platform, KASURL: kas, IssuerURL: issuer, ClientID: "opentdf-sdk", ClientSecret: "secret", AllowHTTP: true}))
	defer c.Close()
	public, kid, e := c.PublicKey(context.Background())
	check(e)
	if kid != "r1" {
		panic("unexpected real KAS key")
	}
	pub := must(public.PublicPEM())
	public.Close()
	s := must(reference.New(platform, reference.WithClientCredentials("opentdf-sdk", "secret", nil), reference.WithInsecurePlaintextConn(), reference.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil)))))
	defer s.Close()
	var pairs, reads []string
	for _, name := range []string{"small", "binary", "empty"} {
		expected := must(os.ReadFile(filepath.Join(sdk, ".local", "interop", name+".input")))
		for _, producer := range []string{"go", "web"} {
			label := name + "." + producer
			compare(c, must(os.ReadFile(filepath.Join(sdk, ".local", "interop", label+".tdf"))), expected, nil, label)
			reads = append(reads, label)
		}
	}
	cases := []struct {
		name     string
		data     []byte
		alg      string
		metadata string
		noattrs  bool
	}{
		{"client-small", []byte("shared client real KAS\n"), "GMAC", "", false},
		{"client-empty", nil, "GMAC", "", false},
		{"client-binary", []byte{0, 255, 128, 1, 0, 17}, "GMAC", "", false},
		{"client-exact", bytes.Repeat([]byte{0, 255, 128, 7}, 4096), "GMAC", "", false},
		{"client-multiple", bytes.Repeat([]byte{0, 255, 128, 7}, 8193), "GMAC", "", false},
		{"client-hs256", bytes.Repeat([]byte{0, 255, 128, 7}, 8193), "HS256", "", false},
		{"client-metadata", []byte("metadata case"), "GMAC", `{"source":"independent metadata","count":7}`, false},
		{"client-noattrs", []byte("no attributes\n"), "GMAC", "", true},
	}
	for _, v := range cases {
		attributes := []string{allowed}
		if v.noattrs {
			attributes = nil
		}
		options := tdf.EncryptConfig{Attributes: attributes, SegmentSize: 16384, SegmentHashAlgorithm: v.alg, Metadata: []byte(v.metadata)}
		if v.metadata == "" {
			options.Metadata = nil
		}
		data := must(c.Create(context.Background(), v.data, options))
		profile(data, len(v.data), v.alg, v.name+".client")
		check(os.WriteFile(filepath.Join(run, v.name+".tdf"), data, 0600))
		for _, consumer := range []string{"go", "web"} {
			cli(sdk, run, consumer, v.name)
			output := must(os.ReadFile(filepath.Join(run, v.name+"."+consumer+".out")))
			if !bytes.Equal(output, v.data) {
				panic("reference consumer bytes: " + v.name + consumer)
			}
			pairs = append(pairs, v.name+"/"+consumer)
			fmt.Println("PASS real KAS client -> reference:", v.name, consumer)
		}
		// Independent reference reader also authenticates nonempty metadata bytes.
		r := must(s.LoadTDF(bytes.NewReader(data), reference.WithKasAllowlist([]string{kas})))
		if !bytes.Equal(must(io.ReadAll(r)), v.data) || !bytes.Equal(must(r.UnencryptedMetadata()), options.Metadata) {
			panic("independent Go reader metadata")
		}
		input := filepath.Join(run, v.name+".input")
		check(os.WriteFile(input, v.data, 0600))
		// Fresh stock Go SDK writer, preserving normal native reference defaults.
		var written bytes.Buffer
		opts := []reference.TDFOption{reference.WithAutoconfigure(false), reference.WithKasInformation(reference.KASInfo{URL: kas, KID: kid, PublicKey: pub, Algorithm: "rsa:2048"}), reference.WithSegmentSize(16384), reference.WithMetaData(v.metadata)}
		if !v.noattrs {
			opts = append(opts, reference.WithDataAttributes(allowed))
		}
		_, e := s.CreateTDF(&written, bytes.NewReader(v.data), opts...)
		check(e)
		goLabel := v.name + ".go"
		if v.alg == "HS256" {
			goLabel += "-default-gmac"
		}
		profile(written.Bytes(), len(v.data), "GMAC", goLabel)
		compare(c, written.Bytes(), v.data, options.Metadata, goLabel)
		reads = append(reads, goLabel)
		// Fresh stock Web SDK writer supports exact segment and metadata fixtures.
		output := filepath.Join(run, v.name+".web.tdf")
		if e := os.Remove(output); e != nil && !os.IsNotExist(e) {
			panic(e)
		}
		attrs := allowed
		if v.noattrs {
			attrs = ""
		}
		freshCommand(sdk, input, output, v.alg, v.metadata, attrs)
		webData := must(os.ReadFile(output))
		profile(webData, len(v.data), v.alg, v.name+".web")
		compare(c, webData, v.data, options.Metadata, v.name+".web")
		reads = append(reads, v.name+".web")
	}
	deniedData := must(c.Create(context.Background(), []byte("denied"), tdf.EncryptConfig{Attributes: []string{denied}}))
	noOutput(c, deniedData, "client denied policy")
	for _, producer := range []string{"go", "web"} {
		noOutput(c, must(os.ReadFile(filepath.Join(sdk, ".local", "interop", "denied."+producer+".tdf"))), producer+" denied policy")
	}
	tampered := must(c.Create(context.Background(), []byte("tamper"), tdf.EncryptConfig{Attributes: []string{allowed}}))
	a := must(tdf.ReadArchive(tampered, tdf.DefaultArchiveLimits()))
	a.Payload[12] ^= 1
	tampered = must(tdf.WriteArchive(a.Payload, a.Manifest, tdf.DefaultArchiveLimits()))
	noOutput(c, tampered, "payload tamper")
	invalid := must(client.New(client.Config{PlatformURL: platform, KASURL: kas, AllowHTTP: true, TokenProvider: func(context.Context) (client.AccessToken, error) {
		return client.AccessToken{Value: "invalid-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}, nil
	}}))
	allowedData := must(c.Create(context.Background(), []byte("allowed token negative fixture"), tdf.EncryptConfig{Attributes: []string{allowed}}))
	invalidResult, invalidError := invalid.Decrypt(context.Background(), allowedData)
	var authError *client.Error
	if invalidError == nil || len(invalidResult.Payload) != 0 || len(invalidResult.Metadata) != 0 || !errors.As(invalidError, &authError) || authError.Code != "unauthenticated" || authError.HTTPStatus != 401 {
		panic("invalid Bearer failed to produce authenticated HTTP401 zero output")
	}
	fmt.Println("PASS zero output: invalid Bearer HTTP401 on allowed policy")
	invalid.Close()
	ctx, cancel := context.WithCancel(context.Background())
	cancel()
	d, e := c.Decrypt(ctx, allowedData)
	if !errors.Is(e, context.Canceled) || len(d.Payload) != 0 || len(d.Metadata) != 0 {
		panic("canceled decrypt")
	}
	report := map[string]any{"scope": "bounded shared native SDK ordinary Bearer discovery and RSA rewrap", "client_writes": pairs, "reference_reads": reads, "real_denials": 3, "tamper": "zero output", "auth": "ordinary Bearer", "kas": "RSA r1", "session": "fresh RSA2048 OAEP SHA1", "remaining": "EC/DPoP and generated HTTP/all targets"}
	check(os.WriteFile(reportFile, must(json.MarshalIndent(report, "", "  ")), 0600))
	fmt.Println("PASS bounded native shared SDK interoperability")
}
