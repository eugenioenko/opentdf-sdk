# Live EC and enforced DPoP development profiles

These profiles extend the [basic platform setup](platform.md) at the pinned [reference revisions](../references.lock.json). They are local development configurations. They use the existing `tdf-sdk` Docker project, infrastructure, fixture credentials and ports: platform 8080, Postgres 5432, Keycloak 8888 with management on 9000. Switching stops and recreates only `tdf-sdk-platform-1`. No reference source, unrelated Docker project, global CLI profile or host keyring changes.

From `sdk/`, after the initial `make platform-up`:

```sh
make platform-profile-ec      # generate/provision profile keys, switch, verify discovery/auth/key IDs
make interop-profiles         # real EC rewrap, both response sessions, plaintext and PDP denial
make platform-profile-dpop    # same EC registry; enforce DPoP, native nonce and full-origin htu
make interop-profiles         # above plus signed-request-token negative cases and CLI evidence
make platform-ready          # checks whichever profile is selected
make platform-profile-basic  # remove only profile registry rows; restore original r1 config
make platform-ready
make interop-smoke            # original RSA/Bearer stock Go/Web cross-consumer smoke
```

`./scripts/platform.sh profile {ec|dpop|basic|check|smoke}` provides the same entry points. `make platform-up` retains a successfully selected profile across down/up; its artifact comparison considers the platform/Web pins, avoiding an unnecessary reference rebuild when only Goalchemy's pin changes. Restore basic before explicit fixture reprovisioning. The commands require Linux host networking, GNU `timeout`, Docker Compose and the toolchains listed in [platform.md](platform.md). Profile operations bound Go runners to 180 seconds, Docker calls to 90 seconds, runner HTTP calls to 15 seconds and each reference CLI operation to 45 seconds. Fresh runs remove prior success reports and CLI plaintext outputs.

## Effective keys and registry

| Setting | basic | ec | dpop |
| --- | --- | --- | --- |
| KAS key management | false | true, native BasicManager | true, native BasicManager |
| EC TDF preview | false | true | true |
| RSA key ID | r1 | profile-r1 | profile-r1 |
| P256 key ID | absent | profile-e1 | profile-e1 |
| Default/base wrapping key | RSA PublicKey fallback | profile-e1 | profile-e1 |
| DPoP enforce / require_nonce / strict_htu | false / false / false | false / false / false | true / true / true |
| Native nonce expiration | disabled | disabled | 5m |
| Issuer | `http://localhost:8888/auth/realms/opentdf` | same | same |
| Registered KAS URI / manifest destination | `http://localhost:8080/kas` | same | same |

[platform-profile.sh](../scripts/platform-profile.sh), the [Compose override](../dev/compose.profiles.yaml) and the [native helper](../tests/interop/profiles/main.go) generate `.local/profiles/effective.yaml`. It derives from the basic config, sets `registered_kas_uri` to the **exact registry URI ending in `/kas`**, enables EC preview/key management and supplies a generated 32-byte root KEK as hex. The dpop profile additionally sets `server.auth.dpop.enforce`, `require_nonce`, `strict_htu` and `nonce_expiration`. The original basic config and r1 private key/certificate remain intact. EC/RSA rewrap session keys and DPoP authentication keys are independent ephemeral keys.

The helper persists one generated P256 private key and one random KEK beneath ignored, permission-restricted `.local/profiles/`, reusing both across switches. `profile-r1` uses the existing RSA-2048 private key with a distinct registry ID. Both registry entries are ACTIVE with `KEY_MODE_CONFIG_ROOT_KEY`; public contexts hold base64 SPKI PEM, private contexts hold base64 `nonce12 || AES-256-GCM(private PEM) || tag16` under that KEK. This matches pinned [BasicManager unwrap](../../platform/service/internal/security/basic_manager.go). Private PEM/KEK/SQL/config must stay together when backing up local profile storage. No private keys, tokens or proofs appear in result reports.

The provisioned base key and value-level grants both select **profile-e1**. Both `attr1/value/value1` and `value2` receive the EC grant, while subject mappings remain unchanged: `opentdf-sdk` is entitled to value1 and denied value2. This avoids falsely treating an EC PublicKey lookup against an RSA default registry as EC encryption. The smoke reads actual emitted manifests and requires one `ec-wrapped` KAO with kid `profile-e1` from the pinned Go SDK's default/grant lookup. Live discovery must independently name the same base key, KAS URI, algorithm and issuer; live PublicKey responses must parse as P256 and RSA-2048 respectively. The SQL sets a transaction-local `opentdf_policy, public` search path because the pinned base-key trigger uses unqualified table names.

The [reset SQL](../dev/secure-profile.sql) removes only profile-r1/profile-e1's base entry, value grants and key rows. It preserves basic KAS routing, original r1 material, attributes and authorization fixtures. The `active` marker is invalidated before switching and written only after live verification succeeds. A failed switch can leave the platform stopped or running a partially checked profile; recover with `make platform-profile-basic`. A missing marker never constitutes successful restoration; `platform-up`, `platform-ready` and reprovisioning reject unverified selection after a profile attempt. Down retains local state. Do not delete the KEK/private key files while preserving their wrapped-key registry rows; switch basic before resetting profile storage.

## Observed live evidence

The profile runner is an isolated native module. Authentication and rewrap use the **pinned Go SDK**, not the new shared SDK client. The runner also produces an EC KAO with the accepted new engine and consumes it through the pinned reader. This is platform/profile and engine interoperability evidence; the shared client still requires a separate EC/DPoP implementation assignment. It is not seven-target or browser parity evidence.

`make interop-profiles` verifies 36,018 binary plaintext bytes with both P256 and RSA-2048 response sessions for the pinned Go SDK's EC file and the new engine's independently constructed EC file. The pinned reader must also return real PDP permission denial and zero plaintext for the EC file tagged with value2. Stock Go CLI independently decrypts through the same real KAS with both response sessions. In the ec profile, stock Web CLI also decrypts successfully with both sessions and ordinary Bearer auth.

The dpop profile performs these live protected policy RPC checks, using a real Keycloak-issued DPoP access token whose `cnf.jkt` equals the proof signing key's SHA-256 JWK thumbprint:

| Case | HTTP status |
| --- | --- |
| Ordinary unbound Bearer token | 401 |
| Bound token with missing resource proof | 401 |
| Correct proof without nonce | 401 with native `DPoP-Nonce` and `WWW-Authenticate: DPoP error="use_dpop_nonce"` |
| Fresh correctly signed proof with returned nonce | 200 |
| Replayed identical proof/jti | 401 |
| Same nonce with fresh jti | 200 |
| Wrong nonce/key/signature, expired proof, missing/wrong ath, wrong htm/htu, path-only htu | 401 |
| Bound token sent under Bearer with otherwise valid proof | **200** |

The last row is an observed pinned-service limitation: enforcement rejects ordinary unbound Bearer tokens but does not reject the Bearer scheme when a correctly bound token and valid DPoP proof accompany it. Do not equate this with strict authorization-scheme enforcement. Nonces are reusable; [native nonce state](../../platform/service/internal/auth/authn.go) accepts current and previous nonces during rotation, while [replay protection](../../platform/service/internal/auth/dpop_replay.go) rejects proof/jti reuse. The runner does not wait for nonce rotation or claim an expired-nonce live test. Expired **proof** and arbitrary incorrect nonce checks are distinct. The live service issues its own nonce; no proxy challenge adapter or mock is involved. Valid requests/rewraps exercise the pinned Go transport's native challenge retry.

Real KAS Rewrap also receives authenticated DPoP requests with a valid signed request token (SRT), an SRT signed with a different authentication key, a corrupted SRT signature and an expired SRT. Results are respectively 200, 401, 401 and 401. This proves [server SRT verification](../../platform/service/kas/access/rewrap.go) binds its signing key to authenticated DPoP and checks signature/expiry. The explicit negative probe uses the supported legacy single-KAO `requestBody` shape. The pinned reader additionally exercises its normal grouped rewrap flow. The next shared-client assignment must test that client's modern grouped SRT under enforcement rather than infer it from these negative probes.

Stock Web CLI `--dpop` fails authentication with HTTP401/unauthenticated on both sessions under the native enforced-nonce profile, with no successful plaintext. This is recorded as an expected **reference incompatibility**, never as successful web DPoP interoperability. The pinned [Connect interceptor](../../web-sdk/lib/src/auth/interceptors.ts) generates a resource proof without passing access-token hash/nonce arguments; the [OIDC legacy path](../../web-sdk/lib/src/auth/oidc.ts) passes the token hash but supplies no nonce and uses Bearer. These source limitations explain why stock web auth cannot be presumed equivalent to the pinned Go transport; the CLI failure diagnostics are retained only in memory and reports contain whitelisted auth markers. No insecure auth bypass or reference-source fix is used.

Nonsecret evidence lives under `.local/profiles/`: `check-{ec,dpop}.json`, `smoke-{ec,dpop}.json`, `cli-{ec,dpop}.json`, `srt-dpop.json` and allowed/denied archives. Check reports distinguish configured key management/EC preview from verified live discovery, parsed public keys, and successful rewrap in smoke. Initial/basic checks remain `.local/platform-check.json`; the original smoke writes `.local/interop/results.json`. The source pin/cleanliness check runs before every profile helper invocation and switching also verifies cached platform/Web artifact revisions.
