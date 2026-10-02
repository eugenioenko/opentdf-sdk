# Phase 1 observed results

Verified on 2026-10-02 UTC (2026-10-01 America/Los_Angeles). This verifies the local development platform and pinned reference CLI interoperability; the new SDK and seven generated target libraries are not implemented by this phase.

References: platform `f2635158b681fa970aafce7eacf108a453521f63`, web SDK `55a0521b1499b392c75373e11ec5930c6a43f0c7`. Both tracked reference working trees remained clean. Service/Go CLI build used Go 1.25.14; Node CLI used Node 24.15.0/npm 11.12.1. Image identities are in `dev/images.env` and `dev/compose.yaml`.

| Check | Observed result |
| --- | --- |
| `./scripts/platform.sh build` | PASS: source-built monolith and Go CLI, local service image, source-built/packed TS SDK, isolated pinned Node CLI; exit 0 |
| Fresh persistent state `./scripts/platform.sh up` | PASS: automatic new RSA keys, fresh Postgres/Keycloak directories, original fixture provisioning, documented local KAS routing adaptation, readiness; exit 0 |
| Health `GET /healthz` | PASS, JSON healthy response |
| Unauthenticated `ListAttributes` policy RPC | PASS rejection: HTTP 401, `unauthenticated`, missing authorization header |
| OAuth client credentials | PASS Bearer token, exact issuer `http://localhost:8888/auth/realms/opentdf`, audience includes `http://localhost:8080` |
| KAS PublicKey | PASS real Connect RPC: RSA-2048 public PEM, kid `r1` |
| Go -> TypeScript, 35-byte text | PASS `cmp` |
| TypeScript -> Go, 35-byte text | PASS `cmp` |
| Go -> TypeScript, 4,110-byte arbitrary binary | PASS `cmp` |
| TypeScript -> Go, 4,110-byte arbitrary binary | PASS `cmp` |
| Go -> TypeScript, zero bytes | PASS `cmp` |
| TypeScript -> Go, zero bytes | PASS `cmp` |
| Go denied-policy file -> TypeScript decrypt | PASS failure with permission denial; no plaintext bytes |
| TypeScript denied-policy file -> Go decrypt | PASS failure with permission denial; no plaintext bytes |
| Project-scoped stop/start | PASS: only `tdf-sdk` containers removed/recreated; other projects preserved |
| Setup syntax and whitespace | PASS `bash -n`, Python compilation, `git diff --check` |

All six successful files contain `0.payload` and `0.manifest.json`, manifest `schemaVersion:4.3.0`, one wrapped RSA KAO with local URL `http://localhost:8080/kas`, kid `r1` and KAO version `1.0`. Decrypt uses real `/kas.AccessService/Rewrap`. Fresh service audit logs record successful rewrap for both `node` and `connect-go/1.20.0 (go1.25.14)` clients with algorithm `rsa:2048` and key ID `r1`; denied cases record authorization failure and KAS rewrap error. Plaintext SHA-256 hashes and per-file observed manifests are saved in `.local/interop/results.json`.

Allowed attribute: `https://example.com/attr/attr1/value/value1`. Denied attribute: `https://example.com/attr/attr1/value/value2`. Both are registered upstream fixtures. The `opentdf-sdk` client is mapped to the first and lacks entitlement to the second. Denial checks require an actual permission-denied/forbidden message and reject unrelated failures as evidence of enforcement.

The final fresh run used these commands from the workspace root, retaining previous project storage as ignored backups:

```sh
sdk/scripts/platform.sh down > sdk/.local/logs/fresh-down.log 2>&1
mv sdk/.local/postgres sdk/.local/postgres-initial
mv sdk/.local/keycloak sdk/.local/keycloak-initial
mv sdk/.local/keys sdk/.local/keys-initial
mv sdk/.local/provisioned sdk/.local/provisioned-initial
sdk/scripts/platform.sh up > sdk/.local/logs/fresh-up.log 2>&1
sdk/scripts/reference-smoke.sh > sdk/.local/logs/fresh-smoke.log 2>&1
```

The stack remains running for independent review: `tdf-sdk-platform-1`, `tdf-sdk-keycloak-1`, `tdf-sdk-postgres-1`. All logs, keys, data and output are under `.local/`; rerun `make platform-ready` and `make interop-smoke` from `sdk/`. The complete reproducible operator workflow is in [platform.md](platform.md).

Initial setup exposed and resolved three concrete issues. Docker auto-created an unwritable Keycloak bind directory before init; the final init explicitly sets only that directory's image UID/GID. Original policy fixtures route Go autoconfiguration to external KASes; original fixtures are now followed by `dev/local-kas.sql` to replace development routing, preserving authorization mappings. The CLI's checked-in local SDK tarball integrity did not match a new pinned-source pack; an isolated copy refreshes only that tarball's path/hash, preserving every other dependency lock entry. These changes are documented and do not alter tracked reference behavior. The Node reference also prints a recoverable missing-base-key discovery error; its original PublicKey RPC fallback succeeds. No auth adapter was needed.

Authentication is enabled. DPoP enforcement is false and runtime discovery reports `dpop_nonce_required:false`; successful Bearer tests do not establish enforced DPoP behavior. The service startup log lists RSA-2048/RSA-4096 trust mechanisms; only the generated RSA-2048 key is exercised. EC/hybrid preview and KAS key management are disabled. CORS intent includes readable `DPoP-Nonce`, but browser, HTTPS/public-client auth, EC, enforced DPoP, token expiry/cancellation, segment boundaries, large payloads and tamper/malformed-format suites remain later-phase requirements. No required Phase 1 smoke test was skipped.
