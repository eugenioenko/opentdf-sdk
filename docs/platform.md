# Local platform and reference interoperability

Run from `sdk/` on Linux with Docker Engine and Compose, Go, Node 24, npm, Python 3, OpenSSL, curl, ripgrep and cmp installed. Docker access is required. The tested host versions are Go 1.25.1 (the build selects and downloads Go 1.25.14), Node 24.15.0, npm 11.12.1, Docker Engine 29.2.1 and Compose 5.0.2. Builds need network access for locked Go/npm dependencies and digest-pinned images. This is a development configuration with HTTP and upstream fixture credentials; use it on a development host.

```sh
make platform-up       # init, build when needed, start infra, provision once, start monolith, check
make platform-ready    # health, token issuer/audience, unauthenticated rejection, RSA public key
make interop-smoke     # both reference CLIs in both directions; byte comparisons and policy denial
make platform-status
make platform-logs
make platform-down     # removes only this Compose project's containers; preserves local data
```

Individual steps are also available as `make platform-init`, `make platform-build` and `./scripts/platform.sh provision`. Build checks refuse a changed reference revision or tracked source changes. `platform-up` rebuilds when `.local/build-references.json` differs from `references.lock.json`; use `platform-build` explicitly after changing setup/build files or if an artifact is missing. `logs` accepts a service name, for example `./scripts/platform.sh logs platform`.

The only long-lived services are `tdf-sdk-platform-1`, `tdf-sdk-keycloak-1` and `tdf-sdk-postgres-1`, under Compose project `tdf-sdk`. Commands do not stop or recreate other Compose projects. Persistent database/Keycloak data, generated RSA keys/certificates, binaries, caches, CLI build metadata, profiles, fixture logs and smoke files are ignored beneath `.local/`. Keycloak initialization uses a short-lived digest-pinned runtime container to give only its workspace bind mount UID/GID 1000 ownership; the host user need not have that UID. The Go smoke commands supply explicit host and client-credential flags, selecting the CLI's in-memory profile; the Node CLI has no persistent profile. No host keyring, host file, global profile or certificate trust store is configured.

The setup uses Linux host networking. Reserve TCP ports 5432 (Postgres), 8888 (Keycloak), 9000 (Keycloak management) and 8080 (platform). Host clients and containers use the same URLs, so issuer comparison requires no DNS aliases:

| Purpose | URL |
| --- | --- |
| OAuth issuer | `http://localhost:8888/auth/realms/opentdf` |
| OAuth token | issuer + `/protocol/openid-connect/token` |
| Platform/audience | `http://localhost:8080` |
| Health | platform + `/healthz` |
| KAS manifest destination | `http://localhost:8080/kas` |
| Connect RSA public key | platform + `/kas.AccessService/PublicKey` |
| Connect rewrap | platform + `/kas.AccessService/Rewrap` |
| Discovery | platform + `/.well-known/opentdf-configuration` |

This networking profile is tested on Linux Docker Engine. It is not a verified Docker Desktop/macOS/Windows profile. Postgres and Keycloak startup readiness are checked before provisioning; platform readiness is checked before tests.

## Pinned builds and configuration

The service and Go CLI are built from the adjacent `../platform` revision in `references.lock.json`, using its Go workspace, module versions and sums. `dev/Dockerfile` packages that native binary in a digest-pinned Debian runtime. `dev/images.env` pins Postgres 15.17 Alpine and the runtime; Compose pins OpenTDF Keycloak 26.4.0 by digest. The host architecture must match the native binary's architecture. Go build/download caches use `.local/`.

The [official complete quickstart](https://opentdf.io/quickstart/docker-compose.yaml) supplied the service ordering, key generation and provisioning pattern. Its `nightly` service image, `main` downloads, Caddy and host/trust changes are replaced by local pinned source, digest-pinned images, checked-in configuration and shared loopback URLs. The adjacent platform's default Compose file supplies only infrastructure and is not used to start this stack.

`dev/opentdf.yaml` adapts the pinned development configuration: authentication remains enabled, client credentials use ordinary Bearer tokens, one RSA-2048 key `r1` is generated locally, KAS key management and EC/hybrid preview are disabled, and DPoP enforcement is disabled. Runtime discovery advertises DPoP support and `dpop_nonce_required:false`; these are separate from enforcement. Actual RSA PublicKey and rewrap are checked, rather than inferring algorithm usability from discovery. The KAS startup log advertises trust mechanisms `rsa:2048` and `rsa:4096`; only RSA-2048 is provisioned and exercised here.

Keycloak realms/clients/roles and policy attributes/subject mappings are provisioned by the pinned platform binary from its original checked-in YAML fixtures. `dev/local-kas.sql` then replaces the external fixture KAS registry/keys/grants with one local KAS. This changes development routing; it preserves attribute definitions and subject authorization mappings. Without this adaptation, Go's default autoconfiguration selects external fixture KAS URLs. The `opentdf-sdk` client with fixture secret `secret` is entitled to `https://example.com/attr/attr1/value/value1`; it is denied `.../value/value2`. The admin and entity-resolution credentials are the original development fixture values.

The TypeScript library is built/packed from pinned `../web-sdk/lib` using `npm ci`. The pinned CLI's lockfile stores the hash of a prior SDK tarball; a new source-built tarball has a different integrity hash. `scripts/build-web-cli.py` copies the CLI source/build metadata into `.local/web-cli` and changes only `@opentdf/sdk`'s local file path and SHA-512 integrity in the copied manifest/lock. All other locked dependencies remain unchanged. No tracked reference source or lockfile is modified. No auth adapter is used: both smoke directions run the stock compiled CLI auth with DPoP disabled. The Go CLI uses its documented native test-build linker flag; operational commands still use the reference SDK/auth implementation.

CORS allows `http://localhost:5173`, the required request headers and readable `DPoP-Nonce`. This records setup intent; Phase 1 does not claim a browser, trusted HTTPS, public-client/token-provider flow, EC rewrap or enforced DPoP test. Those require separate later configurations and validation.

## Evidence and lifecycle

`platform-ready` writes `.local/platform-check.json` containing health, issuer/audience, rejection status (unauthenticated policy RPC returns 401) and RSA key `r1`; access tokens are not saved. `interop-smoke` saves actual CLI help and per-operation logs under `.local/interop`, then writes `results.json` only after every required smoke check passes. Small text, binary and empty inputs cross both ways with `cmp`. Registered-but-unentitled policies must fail in both consumers with permission denial and no successful plaintext. The Node reference prints a recoverable missing-base-key discovery error before falling back to its PublicKey RPC; successful byte comparisons establish the actual outcome.

`platform-down` preserves `.local/postgres`, `.local/keycloak`, keys and the provisioning marker. Repeated `platform-up` uses that initialized state. To retain a clean test run without deleting prior data, stop this stack, move its `postgres` and `keycloak` directories and `.local/provisioned` marker to backup names inside `.local/`, then start it again; init creates fresh directories and provisioning runs again. Do not reuse a provisioning marker with an empty database. If database fixtures need reprovisioning, stop the platform container first and run the explicit provision command; it replaces development KAS routing again. Do not use this command against production data.

The current verified outcomes and remaining scope are in [phase1-results.md](phase1-results.md).

## EC and enforced DPoP profiles

[secure-profiles.md](secure-profiles.md) documents reproducible switches to native key-managed P256 EC and enforced DPoP with native nonce challenges. These reuse the same infrastructure/ports and recreate only this project's platform service. Use `make platform-profile-ec` or `make platform-profile-dpop`, followed by `make interop-profiles`; restore the original RSA/Bearer fixtures with `make platform-profile-basic`. `platform-up` and `platform-ready` retain/check the selected profile. The profile evidence includes real EC KAO/plaintext, EC and RSA response sessions, policy denial, DPoP rejection cases and SRT signature/key binding. Stock Web CLI works in the EC/Bearer profile but fails the enforced DPoP profile; this is an explicit reference limitation. Those runners establish setup and reference/engine interoperability, not new shared-client EC/DPoP or generated-target parity.
