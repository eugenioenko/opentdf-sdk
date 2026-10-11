package main

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"strings"

	g "goalchemyout"
)

// platformConfiguration mirrors platform/sdk PlatformConfiguration: the
// well-known document's "configuration" object, read for platform_issuer.
type platformConfiguration map[string]any

var (
	errPlatformUnreachable  = errors.New("platform unreachable or not responding")
	errPlatformConfigFailed = errors.New("failed to retrieve platform configuration")
	errPlatformIssuer       = errors.New("platform_issuer is not set, or is not a string")
)

func (c platformConfiguration) issuer() (string, error) {
	issuer, ok := c["platform_issuer"].(string)
	if !ok {
		return "", errPlatformIssuer
	}
	return issuer, nil
}

func getPlatformConfiguration(ctx context.Context, endpoint string) (platformConfiguration, error) {
	req, err := http.NewRequestWithContext(ctx, http.MethodGet, endpoint+"/.well-known/opentdf-configuration", nil)
	if err != nil {
		return nil, errors.Join(errPlatformConfigFailed, err)
	}
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return nil, errors.Join(errPlatformUnreachable, err)
	}
	defer resp.Body.Close()
	body, err := io.ReadAll(io.LimitReader(resp.Body, 1<<20))
	if err != nil {
		return nil, errors.Join(errPlatformUnreachable, err)
	}
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("%w: status %d", errPlatformConfigFailed, resp.StatusCode)
	}
	// The Connect RPC reply wraps the document in "configuration"; the HTTP
	// GET route serves the document itself.
	var doc platformConfiguration
	if err := json.Unmarshal(body, &doc); err != nil || doc == nil {
		return nil, errors.Join(errPlatformConfigFailed, err)
	}
	if inner, ok := doc["configuration"].(map[string]any); ok {
		return inner, nil
	}
	return doc, nil
}

// discover resolves the issuer the façade needs from the platform's well-known
// configuration, with otdfctl's wording when the platform cannot be reached.
func discover(ctx context.Context, cfg *g.Config, tlsNoVerify bool) error {
	pcfg, err := getPlatformConfiguration(ctx, cfg.PlatformURL)
	if err != nil {
		if errors.Is(err, errPlatformUnreachable) {
			return fmt.Errorf("Failed to connect to the platform. Is the platform accepting connections at '%s'?", cfg.PlatformURL)
		}
		return fmt.Errorf("Failed to get the platform configuration. Is the platform serving a well-known configuration at '%s'?", cfg.PlatformURL)
	}
	issuer, err := pcfg.issuer()
	if err != nil {
		return fmt.Errorf("Failed to get the platform configuration: %s", err)
	}
	cfg.IssuerURL = issuer
	return nil
}

// apiBase maps a KAS URL to the Connect API base the façade routes it to: the
// platform's embedded KAS is published as <platform>/kas, a standalone KAS as
// its own origin.
func apiBase(kasURL string) string {
	return strings.TrimSuffix(strings.TrimRight(kasURL, "/"), "/kas")
}

// allowedRoutes builds the façade's trusted KAS routes from --kas-allowlist:
// the platform's own KAS is always routable, an explicit list adds each
// entry, and "*" (otdfctl's ignore-allowlist spelling) adds the manifest's KAS.
func allowedRoutes(platform, allowlist, manifestKAS string) []g.KASRoute {
	routes := []g.KASRoute{{URL: platform + "/kas", APIBaseURL: platform}, {URL: platform, APIBaseURL: platform}}
	seen := map[string]bool{routes[0].URL: true, routes[1].URL: true}
	add := func(u string) {
		u = strings.TrimRight(u, "/")
		if u == "" || seen[u] {
			return
		}
		seen[u] = true
		routes = append(routes, g.KASRoute{URL: u, APIBaseURL: apiBase(u)})
	}
	for _, u := range splitList(allowlist) {
		if u == "*" {
			add(manifestKAS)
			continue
		}
		add(u)
	}
	return routes
}
