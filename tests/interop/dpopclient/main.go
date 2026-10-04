// Native-only enforced DPoP interoperability: shared readers use their own auth,
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
	client "opentdf-local/sdk/src"
	"opentdf-local/sdk/src/tdf"
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
func newClient(wrapping, session, auth string) *client.Client {
	return must(client.New(client.Config{PlatformURL: platform, KASURL: kas, IssuerURL: issuer, ClientID: "opentdf-sdk", ClientSecret: "secret", AllowHTTP: true, KASAlgorithm: wrapping, SessionAlgorithm: session, AuthAlgorithm: auth, DPoP: true}))
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

// Switches target only the tdf-sdk project. The runner restores basic on panic
// too; the documented outer shell trap also covers subprocess termination.
func switchProfile(name string) {
	ctx, cancel := context.WithTimeout(context.Background(), 180*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "make", "platform-profile-"+name)
	cmd.Dir = sdk
	out, e := cmd.CombinedOutput()
	if e != nil {
		panic(fmt.Sprintf("profile %s: %v\n%s", name, e, out))
	}
	fmt.Println("PROFILE", name)
}
func restoreBasic() {
	ctx, cancel := context.WithTimeout(context.Background(), 240*time.Second)
	defer cancel()
	cmd := exec.CommandContext(ctx, "make", "platform-profile-basic", "platform-ready")
	cmd.Dir = sdk
	out, e := cmd.CombinedOutput()
	if e != nil {
		fmt.Fprintf(os.Stderr, "RESTORE basic failed: %v\n%s\n", e, out)
		panic("basic restore failed")
	}
	fmt.Println("PROFILE basic restored and ready")
}
func main() {
	sdk = must(filepath.Abs("../../.."))
	verifyReferences()
	name := os.Getenv("TDF_DPOPCLIENT_INTEROP_NAME")
	if name == "" {
		name = "interop-dpopclient"
	}
	if name != "interop-dpopclient" && !strings.HasPrefix(name, "interop-dpopclient-") {
		panic("output name must begin interop-dpopclient")
	}
	if len(name) > 80 {
		panic("invalid output directory")
	}
	for _, b := range []byte(name) {
		if !(b >= 'a' && b <= 'z' || b >= '0' && b <= '9' || b == '-') {
			panic("invalid output directory")
		}
	}
	run = filepath.Join(sdk, ".local", name)
	check(os.MkdirAll(run, 0700))
	info := must(os.Lstat(run))
	if !info.IsDir() || info.Mode()&os.ModeSymlink != 0 {
		panic("output directory must be an ordinary directory")
	}
	entries := must(os.ReadDir(run))
	if len(entries) > 2048 {
		panic("output artifact limit")
	}
	for _, entry := range entries {
		if entry.Type().IsRegular() {
			remove(filepath.Join(run, entry.Name()))
		} else {
			panic("unexpected non-file output artifact")
		}
	}
	report := filepath.Join(run, "results.json")
	restored := false
	defer func() {
		if !restored {
			restoreBasic()
		}
	}()
	cases := []struct {
		name                string
		data                []byte
		integrity, metadata string
	}{
		{"small", []byte("native shared enforced DPoP\n"), "GMAC", ""}, {"empty", nil, "GMAC", ""},
		{"binary", []byte{0, 255, 128, 1, 0, 17}, "GMAC", ""}, {"exact", bytes.Repeat([]byte{0, 255, 128, 7}, 4096), "GMAC", ""},
		{"multiple", bytes.Repeat([]byte{0, 255, 128, 7}, 8193), "GMAC", ""}, {"hs256", bytes.Repeat([]byte{0, 255, 128, 7}, 8193), "HS256", ""},
		{"metadata", []byte("metadata bytes"), "GMAC", `{"source":"independent metadata","count":7}`},
	}
	var writes, reads, metadataReads, negatives, webFormat, webAuth []string
	// Stock Web does not authenticate on this nonce-enforced profile. Produce its
	// independent TDFs under Bearer first; persistent registry keys stay identical.
	switchProfile("ec")
	s := must(reference.New(platform, reference.WithClientCredentials("opentdf-sdk", "secret", nil), reference.WithInsecurePlaintextConn(), reference.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil)))))
	defer s.Close()
	s.Conn().Client.Timeout = 15 * time.Second
	pubs := map[string]string{}
	kids := map[string]string{}
	for _, wrapping := range []string{"rsa:2048", "ec:secp256r1"} {
		discovery := must(client.New(client.Config{PlatformURL: platform, KASURL: kas, IssuerURL: issuer, ClientID: "opentdf-sdk", ClientSecret: "secret", AllowHTTP: true, KASAlgorithm: wrapping}))
		ctx, cancel := operationContext()
		public, kid, e := discovery.PublicKey(ctx)
		cancel()
		check(e)
		pub := must(public.PublicPEM())
		public.Close()
		discovery.Close()
		pubs[wrapping] = pub
		kids[wrapping] = kid
		check(os.WriteFile(filepath.Join(run, wrapping[:2]+"-public.pem"), []byte(pub), 0600))
		want := "profile-r1"
		if wrapping == "ec:secp256r1" {
			want = "profile-e1"
		}
		if kid != want {
			panic("persistent profile key id")
		}
		for _, v := range cases {
			label := wrapping[:2] + "-" + v.name
			input := filepath.Join(run, label+".input")
			check(os.WriteFile(input, v.data, 0600))
			webFixture(input, filepath.Join(run, label+".web.tdf"), v.integrity, v.metadata, allowed, wrapping, kid)
			var out bytes.Buffer
			_, e := s.CreateTDF(&out, bytes.NewReader(v.data), reference.WithAutoconfigure(false), reference.WithKasInformation(reference.KASInfo{URL: kas, KID: kid, PublicKey: pub, Algorithm: wrapping}), reference.WithSegmentSize(16384), reference.WithMetaData(v.metadata), reference.WithDataAttributes(allowed))
			check(e)
			check(os.WriteFile(filepath.Join(run, label+".go.tdf"), out.Bytes(), 0600))
			profile(out.Bytes(), len(v.data), "GMAC", wrapping, kid, label+".go")
			profile(must(os.ReadFile(filepath.Join(run, label+".web.tdf"))), len(v.data), v.integrity, wrapping, kid, label+".web")
		}
		input := filepath.Join(run, wrapping[:2]+"-denied.input")
		check(os.WriteFile(input, []byte("denied"), 0600))
		webFixture(input, filepath.Join(run, wrapping[:2]+"-denied.web.tdf"), "GMAC", `{"denied":true}`, denied, wrapping, kid)
		var out bytes.Buffer
		_, e = s.CreateTDF(&out, bytes.NewReader([]byte("denied")), reference.WithAutoconfigure(false), reference.WithKasInformation(reference.KASInfo{URL: kas, KID: kid, PublicKey: pub, Algorithm: wrapping}), reference.WithMetaData(`{"denied":true}`), reference.WithDataAttributes(denied))
		check(e)
		check(os.WriteFile(filepath.Join(run, wrapping[:2]+"-denied.go.tdf"), out.Bytes(), 0600))
	}
	switchProfile("dpop")
	assertEnforcedDiscovery()
	// A fresh stock Go SDK instance carries genuine issuer-bound DPoP auth.
	enforced := must(reference.New(platform, reference.WithClientCredentials("opentdf-sdk", "secret", nil), reference.WithInsecurePlaintextConn(), reference.WithLogger(slog.New(slog.NewTextHandler(io.Discard, nil)))))
	defer enforced.Close()
	enforced.Conn().Client.Timeout = 15 * time.Second
	for _, wrapping := range []string{"rsa:2048", "ec:secp256r1"} {
		for _, auth := range []string{"ES256", "RS256"} {
			writer := newClient(wrapping, "rsa:2048", auth)
			defer writer.Close()
			readers := map[string]*client.Client{}
			for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
				readers[session] = newClient(wrapping, session, auth)
				defer readers[session].Close()
			}
			for _, v := range cases {
				label := wrapping[:2] + "-" + strings.ToLower(auth) + "-" + v.name
				data := create(writer, v.data, tdf.EncryptConfig{Attributes: []string{allowed}, SegmentSize: 16384, SegmentHashAlgorithm: v.integrity, Metadata: []byte(v.metadata)})
				profile(data, len(v.data), v.integrity, wrapping, kids[wrapping], label+".new")
				check(os.WriteFile(filepath.Join(run, label+".tdf"), data, 0600))
				for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
					cli(label, "go", session)
					if !bytes.Equal(must(os.ReadFile(filepath.Join(run, label+".go."+session[:2]+".out"))), v.data) {
						panic("stock Go CLI bytes")
					}
					writes = append(writes, label+"/go-dpop/"+session)
					kt := ocrypto.RSA2048Key
					if session == "ec:secp256r1" {
						kt = ocrypto.EC256Key
					}
					r := must(enforced.LoadTDF(bytes.NewReader(data), reference.WithKasAllowlist([]string{kas}), reference.WithSessionKeyType(kt)))
					if !bytes.Equal(must(io.ReadAll(r)), v.data) || !bytes.Equal(must(r.UnencryptedMetadata()), []byte(v.metadata)) {
						panic("stock Go DPoP bytes/metadata")
					}
					metadataReads = append(metadataReads, label+"/go-dpop/"+session)
					for _, producer := range []string{"go", "web"} {
						sourceLabel := wrapping[:2] + "-" + v.name
						referenceData := must(os.ReadFile(filepath.Join(run, sourceLabel+"."+producer+".tdf")))
						compare(readers[session], referenceData, v.data, []byte(v.metadata), label+"/"+producer+"/"+session)
						reads = append(reads, label+"/"+producer+"-format/own-dpop/"+session)
					}
				}
				fmt.Println("PASS real enforced KAS", label, "Go DPoP consumers and Go/Web producer formats")
			}
			allowedData := create(writer, []byte("negative"), tdf.EncryptConfig{Attributes: []string{allowed}, Metadata: []byte(`{"negative":true}`)})
			deniedNew := create(writer, []byte("denied"), tdf.EncryptConfig{Attributes: []string{denied}, Metadata: []byte(`{"denied":true}`)})
			archive := must(tdf.ReadArchive(allowedData, tdf.DefaultArchiveLimits()))
			archive.Payload[12] ^= 1
			tampered := must(tdf.WriteArchive(archive.Payload, archive.Manifest, tdf.DefaultArchiveLimits()))
			for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
				prefix := wrapping + "/" + auth + "/" + session
				for _, producer := range []string{"new", "go", "web"} {
					data := deniedNew
					if producer != "new" {
						data = must(os.ReadFile(filepath.Join(run, wrapping[:2]+"-denied."+producer+".tdf")))
					}
					zero(readers[session], data, prefix+"/denied/"+producer, "denial")
					negatives = append(negatives, prefix+"/denied/"+producer)
				}
				zero(readers[session], tampered, prefix+"/payload-tamper", "integrity")
				negatives = append(negatives, prefix+"/payload-tamper")
				ctx, cancel := context.WithCancel(context.Background())
				cancel()
				d, e := readers[session].Decrypt(ctx, allowedData)
				if !errors.Is(e, context.Canceled) || len(d.Payload) != 0 || len(d.Metadata) != 0 {
					panic("cancellation failed open")
				}
				negatives = append(negatives, prefix+"/canceled")
				manifestTamperNegatives(readers[session], allowedData, prefix, &negatives)
				protocolNegatives(wrapping, session, auth, allowedData, &negatives)
			}
		}
	}
	// Record actual stock Web DPoP HTTP401 incompatibility before the Bearer switch.
	webAuth = append(webAuth, stockWebDPoPFailure("rs-es256-small"))
	switchProfile("ec")
	for _, wrapping := range []string{"rsa:2048", "ec:secp256r1"} {
		for _, auth := range []string{"ES256", "RS256"} {
			for _, v := range cases {
				for _, session := range []string{"rsa:2048", "ec:secp256r1"} {
					label := wrapping[:2] + "-" + strings.ToLower(auth) + "-" + v.name
					cli(label, "web", session)
					if !bytes.Equal(must(os.ReadFile(filepath.Join(run, label+".web."+session[:2]+".out"))), v.data) {
						panic("Web Bearer format bytes")
					}
					webFormat = append(webFormat, label+"/stock-web-bearer-format/"+session)
				}
			}
		}
	}
	restoreBasic()
	restored = true
	check(os.WriteFile(report, must(json.MarshalIndent(map[string]any{"scope": "native shared client enforced DPoP; RSA/P256 KAO x RSA/P256 session x ES256/RS256 auth", "client_to_stock_go_dpop_cli": writes, "reference_formats_to_own_dpop": reads, "stock_go_dpop_metadata_reads": metadataReads, "stock_web_bearer_format_consumers": webFormat, "stock_web_dpop_outcome": webAuth, "zero_output_cases": negatives, "go_hs256_case": "stock Go writer retains GMAC; stock Web supplies independent HS256", "nonce_evidence": "independent native fixtures observe initial/rotating/success cache and retry counts; real platform enforces nonce and strict full htu", "restored_profile": "basic ready", "remaining": "generated HTTP/all seven targets/actual browser/full Go SDK parity"}, "", "  ")), 0600))
	fmt.Printf("PASS shared DPoP: %d Go CLI, %d producer reads, %d Go metadata, %d Web Bearer format, %d zero-output negatives; stock Web DPoP HTTP401 limitation recorded\n", len(writes), len(reads), len(metadataReads), len(webFormat), len(negatives))
}
