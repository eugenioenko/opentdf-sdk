// Native-only engine acceptance runner. Reference auth/rewrap is deliberately
// confined to this module. Keys and tokens remain in memory and are never printed.
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"log/slog"
	"net/http"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	capcrypto "github.com/eugenioenko/goalchemy/lib/crypto"
	reference "github.com/opentdf/platform/sdk"
	"opentdf-local/sdk/src/tdf"
)

const platform = "http://localhost:8080"
const kas = platform + "/kas"
const issuer = "http://localhost:8888/auth/realms/opentdf"
const allowed = "https://example.com/attr/attr1/value/value1"

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
func request(client *http.Client, path, contentType, token string, body []byte) []byte {
	req := must(http.NewRequest("POST", path, bytes.NewReader(body)))
	req.Header.Set("Content-Type", contentType)
	if token != "" {
		req.Header.Set("Authorization", "Bearer "+token)
		req.Header.Set("Connect-Protocol-Version", "1")
	}
	response := must(client.Do(req))
	defer response.Body.Close()
	if response.StatusCode != 200 {
		panic(fmt.Sprintf("native interop HTTP status %d", response.StatusCode))
	}
	return must(io.ReadAll(io.LimitReader(response.Body, 1<<20)))
}
func cli(sdk, consumer, name string) {
	run := filepath.Join(sdk, ".local", "interop-engine")
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
func referenceRead(s *reference.SDK, data, expected, metadata []byte, label string) {
	r := must(s.LoadTDF(bytes.NewReader(data), reference.WithKasAllowlist([]string{kas})))
	// The pinned SDK independently authorizes and obtains a key from real KAS.
	key := must(r.UnsafePayloadKeyRetrieval())
	if !bytes.Equal(must(r.UnencryptedMetadata()), metadata) {
		panic("reference metadata bytes: " + label)
	}
	result := must(tdf.DecryptWithPayloadKey(data, key))
	if !bytes.Equal(result.Payload, expected) || !bytes.Equal(result.Metadata, metadata) {
		panic("reference-read bytes: " + label)
	}
	fmt.Println("PASS reference -> engine:", label)
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

func main() {
	sdk := must(filepath.Abs("../../.."))
	verifyReferences(sdk)
	run := filepath.Join(sdk, ".local", "interop-engine")
	check(os.MkdirAll(run, 0700))
	client := &http.Client{Timeout: 15 * time.Second}
	token := struct {
		AccessToken string `json:"access_token"`
		Type        string `json:"token_type"`
	}{}
	check(json.Unmarshal(request(client, issuer+"/protocol/openid-connect/token", "application/x-www-form-urlencoded", "", []byte("grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret")), &token))
	if token.AccessToken == "" || token.Type != "Bearer" {
		panic("expected Bearer token")
	}
	public := struct {
		PublicKey string `json:"publicKey"`
		KID       string `json:"kid"`
	}{}
	check(json.Unmarshal(request(client, platform+"/kas.AccessService/PublicKey", "application/json", token.AccessToken, []byte(`{"algorithm":"rsa:2048"}`)), &public))
	if public.KID != "r1" {
		panic("expected KAS r1")
	}
	key := must(capcrypto.ImportPEM(public.PublicKey))
	defer key.Close()
	s := must(reference.New(platform, reference.WithClientCredentials("opentdf-sdk", "secret", nil), reference.WithInsecurePlaintextConn(), reference.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil)))))
	defer s.Close()
	names := []string{"small", "binary", "empty"}
	for _, name := range names {
		expected := must(os.ReadFile(filepath.Join(sdk, ".local", "interop", name+".input")))
		for _, producer := range []string{"go", "web"} {
			data := must(os.ReadFile(filepath.Join(sdk, ".local", "interop", name+"."+producer+".tdf")))
			referenceRead(s, data, expected, nil, name+"."+producer)
		}
	}
	noattrs := must(os.ReadFile(filepath.Join(sdk, ".local", "interop", "noattrs.go.tdf")))
	r := must(s.LoadTDF(bytes.NewReader(noattrs), reference.WithKasAllowlist([]string{kas})))
	expected := must(io.ReadAll(r))
	referenceRead(s, noattrs, expected, nil, "noattrs.go")
	cases := []struct {
		name     string
		data     []byte
		size     int
		set      bool
		alg      string
		metadata []byte
	}{
		{"engine-small", []byte("bounded engine real KAS\n"), 0, false, "GMAC", nil},
		{"engine-empty", nil, 0, false, "GMAC", nil},
		{"engine-exact", bytes.Repeat([]byte{0, 255, 128, 7}, 4096), 16384, true, "GMAC", []byte("metadata")},
		{"engine-multiple", bytes.Repeat([]byte{0, 255, 128, 7}, 8193), 16384, true, "GMAC", nil},
		{"engine-hs256", bytes.Repeat([]byte{0, 255, 128, 7}, 8193), 16384, true, "HS256", nil},
		{"engine-explicit-zero", []byte{0, 255, 128}, 0, true, "GMAC", nil},
	}
	var results []string
	for _, c := range cases {
		data := must(tdf.Encrypt(c.data, tdf.EncryptConfig{KASPublicKey: key, KASURL: kas, KID: public.KID, Attributes: []string{allowed}, SegmentSize: c.size, HasSegmentSize: c.set, SegmentHashAlgorithm: c.alg, Metadata: c.metadata}))
		check(os.WriteFile(filepath.Join(run, c.name+".tdf"), data, 0600))
		for _, consumer := range []string{"go", "web"} {
			cli(sdk, consumer, c.name)
			got := must(os.ReadFile(filepath.Join(run, c.name+"."+consumer+".out")))
			if !bytes.Equal(got, c.data) {
				panic("CLI plaintext bytes: " + c.name + "/" + consumer)
			}
			fmt.Println("PASS engine -> reference:", c.name, consumer)
			results = append(results, c.name+"/"+consumer)
		}
		referenceRead(s, data, c.data, c.metadata, c.name+" recovered by Go SDK")
	}
	// Additional independent Go writer: actual metadata plus multiple segments.
	raw := bytes.Repeat([]byte("\x00\xff\x80go\n"), 6000)
	var written bytes.Buffer
	_, e := s.CreateTDF(&written, bytes.NewReader(raw), reference.WithDataAttributes(allowed), reference.WithAutoconfigure(false), reference.WithKasInformation(reference.KASInfo{URL: kas, KID: public.KID, PublicKey: public.PublicKey, Algorithm: "rsa:2048"}), reference.WithSegmentSize(16384), reference.WithMetaData("reference metadata"))
	check(e)
	referenceRead(s, written.Bytes(), raw, []byte("reference metadata"), "Go multiple segments+metadata")
	report := map[string]any{"scope": "bounded native encryption/integrity engine; reference SDK owns auth/rewrap", "engine_writes": results, "reference_reads": []string{"small.go", "small.web", "binary.go", "binary.web", "empty.go", "empty.web", "noattrs.go", "Go multiple segments+metadata"}, "kas": "real RSA r1", "ec": "independent native vectors only"}
	check(os.WriteFile(filepath.Join(run, "results.json"), must(json.MarshalIndent(report, "", "  ")), 0600))
	fmt.Println("PASS bounded native engine interoperability:", strings.Join(results, ", "))
}
