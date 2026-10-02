// Native-only EC/RSA interoperability: new readers use their own auth,
// discovery, SRT, rewrap and integrity checks. Reference dependencies stay here.
package main

import (
	"bytes"
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"log/slog"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"time"

	"github.com/opentdf/platform/lib/ocrypto"
	reference "github.com/opentdf/platform/sdk"
	client "opentdf-local/sdk"
	"opentdf-local/sdk/tdf"
)

const platform = "http://localhost:8080"
const kas = platform + "/kas"
const issuer = "http://localhost:8888/auth/realms/opentdf"
const allowed = "https://example.com/attr/attr1/value/value1"
const denied = "https://example.com/attr/attr1/value/value2"

var sdk, run string

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
func operationContext() (context.Context, context.CancelFunc) {
	return context.WithTimeout(context.Background(), 30*time.Second)
}
func remove(path string) {
	if e := os.Remove(path); e != nil && !os.IsNotExist(e) {
		panic(e)
	}
}
func gitOutput(repo string, args ...string) []byte {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	return must(exec.CommandContext(ctx, "git", append([]string{"-C", repo}, args...)...).Output())
}
func create(c *client.Client, payload []byte, options tdf.EncryptConfig) []byte {
	ctx, cancel := operationContext()
	defer cancel()
	return must(c.Create(ctx, payload, options))
}
func verifyReferences() {
	lock := struct {
		Repositories map[string]struct{ Path, Revision string }
	}{}
	check(json.Unmarshal(must(os.ReadFile(filepath.Join(sdk, "references.lock.json"))), &lock))
	for _, name := range []string{"platform", "web-sdk"} {
		pin := lock.Repositories[name]
		repo := filepath.Join(sdk, pin.Path)
		if pin.Path == "" || pin.Revision == "" || strings.TrimSpace(string(gitOutput(repo, "rev-parse", "HEAD"))) != pin.Revision {
			panic("reference pin: " + name)
		}
		if len(gitOutput(repo, "status", "--porcelain", "--untracked-files=no")) != 0 {
			panic("reference tracked edit: " + name)
		}
	}
}
func newClient(wrapping, session string) *client.Client {
	return must(client.New(client.Config{PlatformURL: platform, KASURL: kas, IssuerURL: issuer, ClientID: "opentdf-sdk", ClientSecret: "secret", AllowHTTP: true, KASAlgorithm: wrapping, SessionAlgorithm: session}))
}
func profile(data []byte, size int, integrity, wrapping, kid, label string) {
	a := must(tdf.ReadArchive(data, tdf.DefaultArchiveLimits()))
	m := must(tdf.ParseManifest(a.Manifest))
	check(m.ValidateModern(len(a.Payload)))
	k := m.Encryption.KeyAccess[0]
	typ := "wrapped"
	n := 256
	if wrapping == "ec:secp256r1" {
		typ = "ec-wrapped"
		n = 60
	}
	if k.Type != typ || k.KID != kid || len(must(base64.StdEncoding.DecodeString(k.WrappedKey))) != n || m.ResolvedSegmentHashAlgorithm() != strings.ToLower(integrity) {
		panic("actual manifest algorithm/kid/frame: " + label)
	}
	if (k.EphemeralPublicKey != "") != (typ == "ec-wrapped") {
		panic("KAO ephemeral: " + label)
	}
	if size == 0 {
		return
	}
	if len(m.Encryption.Integrity.Segments) != (size+16383)/16384 {
		panic("segment count: " + label)
	}
	remaining := size
	for _, s := range m.Encryption.Integrity.Segments {
		p, c, e := m.Encryption.Integrity.ResolveSegment(s)
		check(e)
		want := remaining
		if want > 16384 {
			want = 16384
		}
		if p != want || c != want+28 {
			panic("framing: " + label)
		}
		remaining -= p
	}
	if remaining != 0 {
		panic("segment total")
	}
}
func compare(c *client.Client, data, payload, metadata []byte, label string) {
	ctx, cancel := operationContext()
	defer cancel()
	d := must(c.Decrypt(ctx, data))
	if !bytes.Equal(d.Payload, payload) || !bytes.Equal(d.Metadata, metadata) {
		panic("new reader bytes/metadata: " + label)
	}
}
func zero(c *client.Client, data []byte, label, expected string) {
	ctx, cancel := operationContext()
	defer cancel()
	d, e := c.Decrypt(ctx, data)
	if e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 {
		panic("failed open: " + label)
	}
	var typed *client.Error
	if !errors.As(e, &typed) {
		panic("unstructured failure: " + label)
	}
	if expected == "denial" {
		diagnostic := strings.ToLower(typed.ServerMessage)
		if !(typed.Operation == "rewrap" && (typed.Code == "denied" && typed.HTTPStatus == 403 || typed.Code == "rewrap_failed" && (strings.Contains(diagnostic, "permission_denied") || strings.Contains(diagnostic, "permissiondenied")))) {
			panic("expected real PDP denial: " + label + " code=" + typed.Code)
		}
	} else if typed.Code != expected {
		panic("unexpected error class: " + label + " code=" + typed.Code)
	}
}
func cli(label, consumer, session string) {
	output := filepath.Join(run, label+"."+consumer+"."+session[:2]+".out")
	remove(output)
	ctx, cancel := context.WithTimeout(context.Background(), 45*time.Second)
	defer cancel()
	var cmd *exec.Cmd
	if consumer == "go" {
		cmd = exec.CommandContext(ctx, filepath.Join(sdk, ".local/bin/otdfctl"), "decrypt", filepath.Join(run, label+".tdf"), "--session-key-algorithm", session, "--kas-allowlist", kas, "--out", output, "--host", platform, "--with-client-creds-file", filepath.Join(sdk, ".local/interop/client-creds.json"))
	} else {
		cmd = exec.CommandContext(ctx, "node", filepath.Join(sdk, ".local/web-cli/bin/opentdf.mjs"), "decrypt", filepath.Join(run, label+".tdf"), "--rewrapKeyType", session, "--allowList", platform, "--output", output, "--platformUrl", platform, "--kasEndpoint", kas, "--oidcEndpoint", issuer, "--clientId", "opentdf-sdk", "--clientSecret", "secret", "--logLevel", "error")
	}
	if _, e := cmd.CombinedOutput(); e != nil {
		panic(fmt.Sprintf("stock %s CLI %s %s: %v", consumer, label, session, e))
	}
}
func webFixture(input, output, integrity, metadata, attribute, wrapping, kid string) {
	remove(output)
	ctx, cancel := context.WithTimeout(context.Background(), 45*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "node", filepath.Join(sdk, "tests/interop/ecclient/web-fixture.mjs"), sdk, input, output, integrity, metadata, attribute, wrapping, kid, filepath.Join(run, wrapping[:2]+"-public.pem"))
	if _, e := cmd.CombinedOutput(); e != nil {
		panic(fmt.Sprintf("stock Web writer %s: %v", filepath.Base(input), e))
	}
}
func main() {
	sdk = must(filepath.Abs("../../.."))
	verifyReferences()
	name := os.Getenv("TDF_ECCLIENT_INTEROP_NAME")
	if name == "" {
		name = "interop-ecclient"
	}
	for _, b := range []byte(name) {
		if !(b >= 'a' && b <= 'z' || b >= '0' && b <= '9' || b == '-') {
			panic("invalid directory")
		}
	}
	run = filepath.Join(sdk, ".local", name)
	check(os.MkdirAll(run, 0700))
	report := filepath.Join(run, "results.json")
	remove(report)
	s := must(reference.New(platform, reference.WithClientCredentials("opentdf-sdk", "secret", nil), reference.WithInsecurePlaintextConn(), reference.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil)))))
	defer s.Close()
	s.Conn().Client.Timeout = 15 * time.Second
	cases := []struct {
		name                string
		data                []byte
		integrity, metadata string
	}{{"small", []byte("native EC client\n"), "GMAC", ""}, {"empty", nil, "GMAC", ""}, {"binary", []byte{0, 255, 128, 1, 0, 17}, "GMAC", ""}, {"exact", bytes.Repeat([]byte{0, 255, 128, 7}, 4096), "GMAC", ""}, {"multiple", bytes.Repeat([]byte{0, 255, 128, 7}, 8193), "GMAC", ""}, {"hs256", bytes.Repeat([]byte{0, 255, 128, 7}, 8193), "HS256", ""}, {"metadata", []byte("metadata bytes"), "GMAC", `{"source":"independent metadata","count":7}`}}
	var writes, reads, metadataReads, negatives []string
	for _, wrapping := range []string{"rsa:2048", "ec:secp256r1"} {
		writer := newClient(wrapping, "rsa:2048")
		defer writer.Close()
		ctx, cancel := operationContext()
		public, kid, e := writer.PublicKey(ctx)
		cancel()
		check(e)
		pub := must(public.PublicPEM())
		check(os.WriteFile(filepath.Join(run, wrapping[:2]+"-public.pem"), []byte(pub), 0600))
		public.Close()
		wantKid := "profile-r1"
		prefix := "rsa"
		if wrapping == "ec:secp256r1" {
			wantKid = "profile-e1"
			prefix = "ec"
		}
		if kid != wantKid {
			panic("live wrapping discovery kid")
		}
		readers := map[string]*client.Client{}
		for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
			readers[session] = newClient(wrapping, session)
			defer readers[session].Close()
		}
		for _, v := range cases {
			label := prefix + "-" + v.name
			metadata := []byte(v.metadata)
			options := tdf.EncryptConfig{Algorithm: wrapping, Attributes: []string{allowed}, SegmentSize: 16384, SegmentHashAlgorithm: v.integrity, Metadata: metadata}
			ctx, cancel := operationContext()
			data := must(writer.Create(ctx, v.data, options))
			cancel()
			profile(data, len(v.data), v.integrity, wrapping, kid, label+".new")
			check(os.WriteFile(filepath.Join(run, label+".tdf"), data, 0600))
			for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
				for _, consumer := range []string{"go", "web"} {
					cli(label, consumer, session)
					if !bytes.Equal(must(os.ReadFile(filepath.Join(run, label+"."+consumer+"."+session[:2]+".out"))), v.data) {
						panic("CLI bytes")
					}
					writes = append(writes, label+"/"+consumer+"/"+session)
				}
				keyType := ocrypto.RSA2048Key
				if session == "ec:secp256r1" {
					keyType = ocrypto.EC256Key
				}
				r := must(s.LoadTDF(bytes.NewReader(data), reference.WithKasAllowlist([]string{kas}), reference.WithSessionKeyType(keyType)))
				if !bytes.Equal(must(io.ReadAll(r)), v.data) || !bytes.Equal(must(r.UnencryptedMetadata()), metadata) {
					panic("stock Go reader bytes/metadata")
				}
				metadataReads = append(metadataReads, label+"/go/"+session)
			}
			input := filepath.Join(run, label+".input")
			check(os.WriteFile(input, v.data, 0600))
			var out bytes.Buffer
			_, e := s.CreateTDF(&out, bytes.NewReader(v.data), reference.WithAutoconfigure(false), reference.WithKasInformation(reference.KASInfo{URL: kas, KID: kid, PublicKey: pub, Algorithm: wrapping}), reference.WithSegmentSize(16384), reference.WithMetaData(v.metadata), reference.WithDataAttributes(allowed))
			check(e)
			profile(out.Bytes(), len(v.data), "GMAC", wrapping, kid, label+".go")
			check(os.WriteFile(filepath.Join(run, label+".go.tdf"), out.Bytes(), 0600))
			webOutput := filepath.Join(run, label+".web.tdf")
			webFixture(input, webOutput, v.integrity, v.metadata, allowed, wrapping, kid)
			webData := must(os.ReadFile(webOutput))
			profile(webData, len(v.data), v.integrity, wrapping, kid, label+".web")
			for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
				for _, producer := range []string{"go", "web"} {
					referenceData := out.Bytes()
					if producer == "web" {
						referenceData = webData
					}
					compare(readers[session], referenceData, v.data, metadata, label+"/"+producer+"/"+session)
					reads = append(reads, label+"/"+producer+"/"+session)
				}
			}
			fmt.Println("PASS real KAS", label, "all producer/consumer/session pairs")
		}
		// Each producer makes a fresh actually-denied policy for each KAO algorithm.
		deniedNew := create(writer, []byte("denied"), tdf.EncryptConfig{Algorithm: wrapping, Attributes: []string{denied}, Metadata: []byte(`{"denied":true}`)})
		var deniedGo bytes.Buffer
		_, e = s.CreateTDF(&deniedGo, bytes.NewReader([]byte("denied")), reference.WithAutoconfigure(false), reference.WithKasInformation(reference.KASInfo{URL: kas, KID: kid, PublicKey: pub, Algorithm: wrapping}), reference.WithMetaData(`{"denied":true}`), reference.WithDataAttributes(denied))
		check(e)
		input := filepath.Join(run, prefix+"-denied.input")
		check(os.WriteFile(input, []byte("denied"), 0600))
		output := filepath.Join(run, prefix+"-denied.web.tdf")
		webFixture(input, output, "GMAC", `{"denied":true}`, denied, wrapping, kid)
		deniedWeb := must(os.ReadFile(output))
		allowedData := create(writer, []byte("negative"), tdf.EncryptConfig{Algorithm: wrapping, Attributes: []string{allowed}, Metadata: []byte(`{"negative":true}`)})
		a := must(tdf.ReadArchive(allowedData, tdf.DefaultArchiveLimits()))
		a.Payload[12] ^= 1
		tampered := must(tdf.WriteArchive(a.Payload, a.Manifest, tdf.DefaultArchiveLimits()))
		for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
			for _, producer := range []string{"new", "go", "web"} {
				data := deniedNew
				if producer == "go" {
					data = deniedGo.Bytes()
				}
				if producer == "web" {
					data = deniedWeb
				}
				zero(readers[session], data, "denied "+producer, "denial")
				negatives = append(negatives, prefix+"/denied/"+producer+"/"+session)
			}
			zero(readers[session], tampered, "payload tamper", "integrity")
			negatives = append(negatives, prefix+"/tamper/"+session)
			ctx, cancel := context.WithCancel(context.Background())
			cancel()
			d, e := readers[session].Decrypt(ctx, allowedData)
			if !errors.Is(e, context.Canceled) || len(d.Payload) != 0 || len(d.Metadata) != 0 {
				panic("cancellation failed open")
			}
			negatives = append(negatives, prefix+"/canceled/"+session)
			invalid := must(client.New(client.Config{PlatformURL: platform, KASURL: kas, AllowHTTP: true, SessionAlgorithm: session, TokenProvider: func(context.Context) (client.AccessToken, error) {
				return client.AccessToken{Value: "invalid-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}, nil
			}}))
			authCtx, authCancel := operationContext()
			d, e = invalid.Decrypt(authCtx, allowedData)
			authCancel()
			invalid.Close()
			var authError *client.Error
			if e == nil || len(d.Payload) != 0 || len(d.Metadata) != 0 || !errors.As(e, &authError) || authError.Code != "unauthenticated" || authError.HTTPStatus != 401 {
				panic("invalid Bearer did not produce HTTP401 zero output")
			}
			negatives = append(negatives, prefix+"/invalid-bearer/"+session)
		}
	}
	check(os.WriteFile(report, must(json.MarshalIndent(map[string]any{"scope": "native shared client ordinary Bearer RSA/P256 KAO and independent RSA/P256 response sessions", "client_to_cli": writes, "reference_to_client": reads, "stock_go_metadata_reads": metadataReads, "zero_output_cases": negatives, "go_hs256_case": "stock writer retains default GMAC; Web supplies independent HS256", "remaining": "DPoP/generated HTTP/all seven targets/actual browser/full Go SDK parity"}, "", "  ")), 0600))
	fmt.Printf("PASS native EC client: %d CLI pairs, %d reference reads, %d metadata reads, %d zero-output cases\n", len(writes), len(reads), len(metadataReads), len(negatives))
}
