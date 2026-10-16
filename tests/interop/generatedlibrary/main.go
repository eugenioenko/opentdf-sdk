// Independent native consumer of the actual generated package. This module
// imports no shared SDK or compiler code; besides the standard library it
// uses only the reference's protocol/go messages for the service clients. It
// drives the handle API: one SDK per run, EncryptBytes/DecryptBytes, LoadTDF
// readers and the platform service clients.
package main

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	"github.com/opentdf/platform/protocol/go/authorization"
	"github.com/opentdf/platform/protocol/go/policy"
	"github.com/opentdf/platform/protocol/go/policy/attributes"
	"github.com/opentdf/platform/protocol/go/policy/namespaces"
	g "goalchemyout"
)

func fail(label string, err error) {
	if err != nil {
		var sdk *g.Error
		var boundary *g.LibraryError
		if errors.As(err, &sdk) {
			panic(label + ": " + sdk.Error() + " kind=" + string(sdk.Kind))
		}
		if errors.As(err, &boundary) {
			panic(label + ": " + boundary.Error())
		}
		panic(label + ": native failure: " + err.Error())
	}
}
func read(path string) []byte     { b, e := os.ReadFile(path); fail("read", e); return b }
func write(path string, b []byte) { fail("write", os.WriteFile(path, b, 0600)) }

// consumerConfig is the run's config.json: the façade Config fields the
// delivery scripts write, mapped onto the handle API's options.
type consumerConfig = g.Config

func optionsOf(cfg consumerConfig) []g.Option {
	opts := []g.Option{g.WithTokenEndpoint(cfg.TokenURL)}
	if cfg.ClientID != "" {
		opts = append(opts, g.WithClientCredentials(cfg.ClientID, cfg.ClientSecret, nil))
	}
	if cfg.TokenProviderName != "" {
		opts = append(opts, g.WithTokenProvider())
	}
	if cfg.AllowHTTP {
		opts = append(opts, g.WithInsecurePlaintextConn())
	}
	if cfg.TimeoutMillis > 0 {
		opts = append(opts, g.WithTimeout(cfg.TimeoutMillis))
	}
	if cfg.DPoP {
		alg := g.SigningAlgorithm(cfg.AuthAlgorithm)
		if alg == "" {
			alg = "ES256"
		}
		opts = append(opts, g.WithDPoPAlgorithm(alg))
	}
	if cfg.AuthPrivateKeyPEM != "" {
		opts = append(opts, g.WithDPoPKeyPEM([]byte(cfg.AuthPrivateKeyPEM)))
	}
	return opts
}

func kasInformation(cfg consumerConfig) g.KASInfo {
	kasURL := cfg.KASURL
	if kasURL == "" {
		kasURL = strings.TrimSuffix(cfg.PlatformURL, "/") + "/kas"
	}
	return g.KASInfo{URL: kasURL, PublicKey: cfg.KASPublicKeyPEM, KID: cfg.KID, Algorithm: cfg.KASAlgorithm}
}

func tdfOptions(cfg consumerConfig, extra ...g.TDFOption) g.TDFOptions {
	opts := []g.TDFOption{g.WithAutoconfigure(false), g.WithKasInformation(kasInformation(cfg)), g.WithDataAttributes("https://example.com/attr/attr1/value/value1"), g.WithSegmentSize(16384)}
	if cfg.KASAlgorithm != "" {
		opts = append(opts, g.WithWrappingKeyAlg(g.KeyType(cfg.KASAlgorithm)))
	}
	o, err := g.TDFOptionsOf(append(opts, extra...)...)
	fail("tdf options", err)
	return o
}

func readerOptions(cfg consumerConfig) g.ReaderOptions {
	allow := []string{cfg.PlatformURL}
	if cfg.KASURL != "" {
		allow = append(allow, cfg.KASURL)
	}
	for _, r := range cfg.AllowedKAS {
		allow = append(allow, r.URL)
	}
	opts := []g.TDFReaderOption{g.WithKasAllowlist(allow)}
	if cfg.SessionAlgorithm != "" {
		opts = append(opts, g.WithSessionKeyType(g.KeyType(cfg.SessionAlgorithm)))
	}
	o, err := g.ReaderOptionsOf(opts...)
	fail("reader options", err)
	return o
}

func main() {
	if len(os.Args) != 5 {
		panic("usage: consumer sdk-root run-dir mode case")
	}
	sdk, run, mode, name := os.Args[1], os.Args[2], os.Args[3], os.Args[4]
	if mode == "controlled" {
		controlledNegatives(run)
		fmt.Println("PASS generated native library", mode, name)
		return
	}
	var cfg consumerConfig
	fail("config", json.Unmarshal(read(filepath.Join(run, "config.json")), &cfg))
	callbacks := g.Callbacks{}
	if cfg.TokenProviderName != "" {
		if cfg.TokenProviderName != g.TokenProviderName {
			panic("unsupported test provider name")
		}
		callbacks = g.TokenCallbacks(func(ctx context.Context) (g.AccessToken, error) {
			if cfg.DPoP {
				return dpopProvider(ctx, cfg, name == "mismatched-provider")
			}
			if name == "invalid-token" {
				return g.AccessToken{Value: "invalid-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() + 300}, nil
			}
			if name == "expired-token" {
				return g.AccessToken{Value: "expired-token", Scheme: "Bearer", ExpiresAt: time.Now().Unix() - 1}, nil
			}
			if name == "provider-reject" {
				return g.AccessToken{}, errors.New("provider rejected")
			}
			form := url.Values{"grant_type": {"client_credentials"}, "client_id": {"opentdf-sdk"}, "client_secret": {"secret"}}
			request, e := http.NewRequestWithContext(ctx, "POST", "http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token", bytes.NewBufferString(form.Encode()))
			if e != nil {
				return g.AccessToken{}, e
			}
			request.Header.Set("Content-Type", "application/x-www-form-urlencoded")
			response, e := (&http.Client{Timeout: 10 * time.Second}).Do(request)
			if e != nil {
				return g.AccessToken{}, e
			}
			body, e := io.ReadAll(io.LimitReader(response.Body, 128*1024))
			response.Body.Close()
			if e != nil {
				return g.AccessToken{}, e
			}
			if response.StatusCode != 200 {
				return g.AccessToken{}, errors.New("provider endpoint")
			}
			var token struct {
				Value   string `json:"access_token"`
				Scheme  string `json:"token_type"`
				Expires int64  `json:"expires_in"`
			}
			if e = json.Unmarshal(body, &token); e != nil {
				return g.AccessToken{}, e
			}
			return g.AccessToken{Value: token.Value, Scheme: "Bearer", ExpiresAt: time.Now().Unix() + token.Expires}, nil
		})
	}
	ctx, cancel := context.WithTimeout(context.Background(), 60*time.Second)
	defer cancel()
	client, e := g.New(ctx, cfg.PlatformURL, g.OptionsOf(optionsOf(cfg)...), callbacks)
	fail("new", e)
	defer client.Close()
	switch mode {
	case "encrypt":
		var extra []g.TDFOption
		if (name == "metadata" || strings.HasSuffix(name, "-metadata")) && name != "empty-metadata" && !strings.HasSuffix(name, "-empty-metadata") {
			extra = append(extra, g.WithMetaData(`{"source":"independent metadata","count":7}`))
		}
		if name == "denied" || strings.HasSuffix(name, "-denied") {
			extra = append(extra, g.WithDataAttributes("https://example.com/attr/attr1/value/value2"))
		}
		data, _, e := client.EncryptBytes(ctx, read(filepath.Join(run, name+".input")), tdfOptions(cfg, extra...), callbacks)
		fail("encrypt", e)
		write(filepath.Join(run, name+".generated.tdf"), data)
	case "decrypt":
		input := read(filepath.Join(run, name+".tdf"))
		reader, e := client.LoadTDF(ctx, bytes.NewReader(input), readerOptions(cfg), callbacks)
		fail("load", e)
		manifest, e := reader.Manifest(ctx)
		fail("manifest", e)
		manifestJSON, e := json.Marshal(manifest)
		fail("manifest json", e)
		hasMetadata := len(manifest.EncryptionInformation.KeyAccessObjs) > 0 && manifest.EncryptionInformation.KeyAccessObjs[0].EncryptedMetadata != ""
		var metadata []byte
		if hasMetadata {
			fail("init", reader.Init(ctx, callbacks))
			metadata, e = reader.UnencryptedMetadata(ctx, callbacks)
			fail("metadata", e)
		}
		fail("reader close", reader.Close())
		payload, e := client.DecryptBytes(ctx, input, readerOptions(cfg), callbacks)
		fail("decrypt", e)
		write(filepath.Join(run, name+".out"), payload)
		write(filepath.Join(run, name+".metadata"), metadata)
		write(filepath.Join(run, name+".manifest"), manifestJSON)
		write(filepath.Join(run, name+".presence"), []byte(strconv.FormatBool(hasMetadata)))
	case "repeat":
		repeat(ctx, client, cfg, callbacks)
	case "services":
		services(ctx, client, cfg, callbacks, run)
	case "negative":
		payload, e := client.DecryptBytes(ctx, read(filepath.Join(run, name+".tdf")), readerOptions(cfg), callbacks)
		if e == nil || payload != nil {
			panic("negative returned successful output")
		}
		var failure *g.Error
		var boundary *g.LibraryError
		report := map[string]any{}
		if errors.As(e, &failure) {
			report["kind"] = failure.Kind
			report["op"] = failure.Op
			report["httpStatus"] = failure.HTTPStatus
			report["message"] = failure.Message
		} else if errors.As(e, &boundary) {
			report["kind"] = boundary.Kind
		} else {
			panic("untyped error")
		}
		reportBytes, e := json.Marshal(report)
		fail("report", e)
		write(filepath.Join(run, name+".error.json"), reportBytes)
	default:
		panic("unknown mode")
	}
	_ = sdk
	fmt.Println("PASS generated native library", mode, name)
}

// repeat runs 100 encrypt and decrypt calls on one SDK, keeps the inputs and
// results independent, closes every reader handle, then closes the SDK and
// checks that later calls fail.
func repeat(ctx context.Context, client *g.SDK, cfg consumerConfig, callbacks g.Callbacks) {
	original := []byte{0, 255, 128, 1}
	for i := 0; i < 100; i++ {
		input := append(append([]byte(nil), original...), byte(i))
		opts := tdfOptions(cfg, g.WithMetaData(fmt.Sprintf("round %d", i)))
		archive, obj, e := client.EncryptBytes(ctx, input, opts, callbacks)
		fail("repeat create", e)
		size, e := obj.Size(ctx)
		fail("repeat size", e)
		if size != int64(len(archive)) {
			panic("size")
		}
		fail("object close", obj.Close())
		reader, e := client.LoadTDF(ctx, bytes.NewReader(archive), readerOptions(cfg), callbacks)
		fail("repeat load", e)
		attrs, e := reader.DataAttributes(ctx)
		fail("repeat attributes", e)
		if len(attrs) != 1 {
			panic("attributes")
		}
		fail("repeat reader close", reader.Close())
		plain, e := client.DecryptBytes(ctx, archive, readerOptions(cfg), callbacks)
		fail("repeat decrypt", e)
		if !bytes.Equal(plain, input) {
			panic("round trip")
		}
		plain[0] = 1
		if input[0] != 0 {
			panic("result alias")
		}
	}
	fail("close", client.Close())
	if _, e := client.DecryptBytes(ctx, original, readerOptions(cfg), callbacks); e == nil {
		panic("closed handle accepted a call")
	}
}

// services is xtest's test_self.py through the generated service clients:
// list namespaces, create a namespace and an attribute, read the attribute
// back by id and in the namespace's list, and report a missing attribute's
// Connect error. The attribute helpers, the access token and a core
// endpoint override are reported with it.
func services(ctx context.Context, client *g.SDK, cfg consumerConfig, callbacks g.Callbacks, run string) {
	name := "gcgo" + strconv.FormatInt(time.Now().UnixNano(), 36) + ".com"
	before, e := client.Namespaces(callbacks).ListNamespaces(ctx, &namespaces.ListNamespacesRequest{})
	fail("list namespaces", e)
	created, e := client.Namespaces(callbacks).CreateNamespace(ctx, &namespaces.CreateNamespaceRequest{Name: name})
	fail("create namespace", e)
	ns := created.GetNamespace()
	if ns.GetId() == "" || ns.GetName() != name {
		panic("created namespace")
	}
	after, e := client.Namespaces(callbacks).ListNamespaces(ctx, &namespaces.ListNamespacesRequest{})
	fail("list namespaces again", e)
	listed := false
	for _, n := range after.GetNamespaces() {
		listed = listed || n.GetId() == ns.GetId()
	}
	if !listed || len(after.GetNamespaces()) != len(before.GetNamespaces())+1 {
		panic("namespace not listed")
	}
	attr, e := client.Attributes(callbacks).CreateAttribute(ctx, &attributes.CreateAttributeRequest{
		NamespaceId: ns.GetId(), Name: "free", Rule: policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF, Values: []string{"1", "2", "3"},
	})
	fail("create attribute", e)
	got, e := client.Attributes(callbacks).GetAttribute(ctx, &attributes.GetAttributeRequest{
		Identifier: &attributes.GetAttributeRequest_AttributeId{AttributeId: attr.GetAttribute().GetId()},
	})
	fail("get attribute", e)
	a := got.GetAttribute()
	var values []string
	for _, v := range a.GetValues() {
		values = append(values, v.GetValue())
	}
	if a.GetFqn() != "https://"+name+"/attr/free" || a.GetRule() != policy.AttributeRuleTypeEnum_ATTRIBUTE_RULE_TYPE_ENUM_ANY_OF || strings.Join(values, ",") != "1,2,3" || a.GetNamespace().GetName() != name {
		panic("attribute read back: " + a.GetFqn())
	}
	inNamespace, e := client.Attributes(callbacks).ListAttributes(ctx, &attributes.ListAttributesRequest{Namespace: name})
	fail("list attributes", e)
	if len(inNamespace.GetAttributes()) != 1 || inNamespace.GetAttributes()[0].GetId() != a.GetId() {
		panic("attribute not listed")
	}
	_, e = client.Attributes(callbacks).GetAttribute(ctx, &attributes.GetAttributeRequest{
		Identifier: &attributes.GetAttributeRequest_AttributeId{AttributeId: "00000000-0000-0000-0000-000000000000"},
	})
	var missing *g.Error
	if !errors.As(e, &missing) || missing.ConnectCode != "not_found" || missing.HTTPStatus != 404 {
		panic(fmt.Sprintf("missing attribute error: %v", e))
	}
	report, e := json.Marshal(map[string]any{
		"namespace": name, "namespaces_before": len(before.GetNamespaces()), "namespaces_after": len(after.GetNamespaces()),
		"attribute_fqn": a.GetFqn(), "values": values,
		"not_found": map[string]any{"kind": missing.Kind, "connectCode": missing.ConnectCode, "httpStatus": missing.HTTPStatus, "message": missing.Message},
		"helpers":   helpers(ctx, client, cfg, callbacks, a.GetFqn()),
	})
	fail("report", e)
	write(filepath.Join(run, "services.json"), report)
}

// helpers runs the reference's attribute helpers against the attribute
// services created, SDK.Auth, and an SDK whose platform endpoint is
// unreachable but whose core connection is the platform.
func helpers(ctx context.Context, client *g.SDK, cfg consumerConfig, callbacks g.Callbacks, fqn string) map[string]any {
	report := map[string]any{}
	check := func(label string, got bool, e error) {
		fail(label, e)
		report[label] = got
	}
	ok, e := client.AttributeExists(ctx, fqn, callbacks)
	check("attribute_exists", ok, e)
	ok, e = client.AttributeExists(ctx, fqn+"-missing", callbacks)
	check("attribute_missing", ok, e)
	ok, e = client.AttributeValueExists(ctx, fqn+"/value/1", callbacks)
	check("value_exists", ok, e)
	ok, e = client.AttributeValueExists(ctx, fqn+"/value/9", callbacks)
	check("value_missing", ok, e)
	attrs, e := client.ListAttributes(ctx, []string{strings.TrimPrefix(strings.TrimSuffix(fqn, "/attr/free"), "https://")}, callbacks)
	fail("list attributes helper", e)
	listed := []string{}
	for _, attr := range attrs {
		listed = append(listed, attr.GetFqn())
	}
	report["listed"] = listed
	fail("validate attributes", client.ValidateAttributes(ctx, []string{fqn + "/value/1", fqn + "/value/2"}, callbacks))
	report["validate_ok"] = true
	e = client.ValidateAttributes(ctx, []string{fqn + "/value/1", fqn + "/value/9"}, callbacks)
	var invalid *g.Error
	if !errors.As(e, &invalid) {
		panic(fmt.Sprintf("validate missing: %v", e))
	}
	report["validate_missing"] = map[string]any{"kind": invalid.Kind, "connectCode": invalid.ConnectCode, "message": invalid.Message}
	entitled, e := client.GetEntityAttributes(ctx, &authorization.Entity{Id: "e1", EntityType: &authorization.Entity_ClientId{ClientId: "opentdf-sdk"}}, callbacks)
	fail("entity attributes", e)
	report["entity_attributes"] = entitled
	auth, e := client.Auth(ctx, callbacks)
	fail("auth", e)
	token, e := auth.AccessToken(ctx, callbacks)
	fail("access token", e)
	fail("auth close", auth.Close())
	report["access_token"] = map[string]any{"present": token.Value != "", "scheme": token.Scheme}
	override, e := g.New(ctx, "http://127.0.0.1:9", g.OptionsOf(append(optionsOf(cfg), g.WithCustomCoreConnection(&g.ConnectRPCConnection{Endpoint: cfg.PlatformURL}))...), callbacks)
	fail("new with core connection", e)
	defer override.Close()
	conn, e := override.Conn(ctx, callbacks)
	fail("conn", e)
	ok, e = override.AttributeExists(ctx, fqn, callbacks)
	fail("override attribute exists", e)
	report["override"] = map[string]any{"endpoint": conn.Endpoint, "attribute_exists": ok}
	return report
}
