# xtest coverage

Every test function in `../tests/xtest` (`test_*.py`, revision
`bc4ead452f4b` of `opentdf/tests`), read from the sources together with
`conftest.py`, `tdfs.py`, `fixtures/*.py` and `sdk/*/cli.sh`. For each test:
the features it gates on, the `XT_WITH_*` inputs and CLI flags it drives
through `cli.sh`, the stdout/stderr patterns it asserts, what it fans out
over, and the phase of [full-sdk-plan.md](full-sdk-plan.md) that makes it
pass for the generated SDKs (`gc*` types). The exact strings are collected in
[error-wording.md](error-wording.md); the per-target baseline is in
[xtest-scoreboard.md](xtest-scoreboard.md).

Nothing was run. The upstream workflow's CI history
(`gh run list -R opentdf/tests -w xtest.yml`) is behind the organisation's
SAML gate for this account, so the behaviour below comes from the code alone.

## How the harness drives an SDK

- `tdfs.SDK` wraps `sdk/<type>/dist/<version>/cli.sh`. `encrypt <in> <out> ztdf`
  and `decrypt <in> <out> ztdf` are the only positional forms; `ztdf-ecwrap`
  becomes `ztdf` plus `XT_WITH_ECWRAP=true`. `supports <feature>` must exit 0,
  1 or 2 (unknown).
- `tdfs.SDK.encrypt_command` sets `XT_WITH_MIME_TYPE` (always, default
  `text/plain` from `fixtures/encryption.py`), `XT_WITH_ATTRIBUTES`
  (comma-joined FQNs), `XT_WITH_ASSERTIONS` (file path or JSON),
  `XT_WITH_TARGET_MODE` (`4.2.2` or `4.3.0`), `XT_WITH_ECWRAP`.
  `decrypt_command` sets `XT_WITH_ASSERTION_VERIFICATION_KEYS`,
  `XT_WITH_ECWRAP`, `XT_WITH_VERIFY_ASSERTIONS=false`,
  `XT_WITH_KAS_ALLOWLIST`, `XT_WITH_IGNORE_KAS_ALLOWLIST=true`. The shims also
  read `XT_WITH_ECDSA_BINDING`, `XT_WITH_PLAINTEXT_POLICY`,
  `XT_WITH_KAS_ALLOW_LIST` (note the underscore: the harness sets
  `XT_WITH_KAS_ALLOWLIST`, which no shim reads; no current test passes an
  allowlist), and the js shim alone reads `XT_WITH_DPOP` and `XT_WITH_DPOP_KEY`.
- `sdk/go/cli.sh` maps them to otdfctl flags: `--mime-type`, `--attr`,
  `--with-assertions`, `--wrapping-key-algorithm ec:secp256r1`,
  `--ecdsa-binding`, `--policy-mode plaintext`, `--target-mode`,
  `--with-assertion-verification-keys`, `--session-key-algorithm ec:secp256r1`,
  `--no-verify-assertions`, `--kas-allowlist <list|*>`, always `--host
  $PLATFORMURL --tls-no-verify --log-level debug --with-client-creds '{...}'`,
  and renames `<out>.tdf` to `<out>` after encrypt. `supports` answers from
  `otdfctl --version --json` (`sdk_version`, `schema_version`,
  `supported_features`) and `help` output; `chunky`, `zip64-at-2gib` and
  `gmac_root_rejected` always answer no.
- `expect_error=True` runs decrypt with stderr folded into stdout and the
  tests grep `exc.output`; `test_abac.py` and `test_policytypes.py` grep
  stdout plus stderr. All greps are case-insensitive regexes.
- Platform features come from `PLATFORM_VERSION` (`tdfs.PlatformFeatureSet`),
  the km1 KAS log or a `PublicKey` probe for the PQ mechanisms, and
  `/.well-known/opentdf-configuration` for `dpop` and `dpop_nonce_challenge`.
  `XT_FORCE_SUPPORTS` and `XT_FORCE_PLATFORM_SUPPORTS` override the SDK and
  platform gates.
- Fan-out: `encrypt_sdk` and `decrypt_sdk` over `--sdks` (default every
  type, every installed version); `container` over `--containers` (default
  `ztdf ztdf-ecwrap`); `size` over `--sizes` (default `small` = 128 B;
  `chunky` 5 MiB, `medium` 2.1 GiB, `large` 5 GiB) for every test that takes
  `pt_file`; `in_focus` from `--focus`. `audit_logs` is on by default and
  fails setup when no KAS log is reachable (`--no-audit-logs` or
  `DISABLE_AUDIT_ASSERTIONS=1` to disable); `zip64`-marked tests are
  deselected unless a size reaches 2 GiB; `benchmark`-marked tests need
  `--bench`.
- Common skips: `skip_hexless_skew` (encrypt supports `hexless` but decrypt
  does not, unless encrypt supports `hexaflexible`), `select_target_version`
  (hexaflexible writers emit `4.2.2` for hexless-less readers, else `4.3.0`),
  `skip_connectrpc_skew` (no-op), `skip_chunky_skew` (reader lacks `chunky`
  and the file elides per-segment sizes), `dspx1153Fails` (`go@v0.15.0`),
  `skip_dspx2457` (java encrypt).
- The upstream workflow runs `test_legacy.py` with `--sdks-decrypt`,
  `test_tdfs.py test_policytypes.py` with `--sdks-encrypt`, and
  `test_abac.py test_pqc.py test_dpop.py` with `--sdks-encrypt` on multi-KAS
  platforms; `test_self.py test_audit_logs.py` validate the harness;
  `test_zip64.py` runs with `--sizes medium` and `test_benchmarks.py` with
  `--bench` in dedicated jobs.

Columns: Features = `sdk.supports`/`skip_if_unsupported` gates (SDK side)
and `pfs` = platform-side gates. Inputs = `XT_WITH_*` variables set for the
cell beyond the always-set `XT_WITH_MIME_TYPE`, and the otdfctl flags they
become. Asserts = output patterns; plaintext equality (`filecmp.cmp`) and
manifest assertions are summarised. Fan-out = parametrisation beyond
`encrypt_sdk`/`decrypt_sdk`.

## `test_tdfs.py` (19)

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_tdf_roundtrip` | encrypt and decrypt `ecwrap` for `ztdf-ecwrap`; pfs `ecwrap`; `hexless`/`hexaflexible` skew; second decrypt with `ecwrap=True` when decrypt supports `ecwrap` and pfs has it | `XT_WITH_ATTRIBUTES` (`attribute_default_rsa`, omitted for ecwrap), `XT_WITH_TARGET_MODE` per skew, `XT_WITH_ECWRAP` (`--wrapping-key-algorithm`/`--session-key-algorithm ec:secp256r1`) | manifest: `payload.isEncrypted`, exactly one KAO, `type` `wrapped`/`ec-wrapped`, `ephemeralPublicKey` presence, `looks_like_422`/`looks_like_430` (schemaVersion, hex-or-not integrity values, root alg HS256 or absent, 32-byte root, 16-byte GMAC or 32-byte HS256 segment hashes); plaintext equal; audit `rewrap` success since mark | `container` (ztdf, ztdf-ecwrap), `pt_file` sizes, `audit_logs` | 1 (gcgo, ztdf, small); 2 (every gc type both directions); 4 (`ztdf-ecwrap` advertised); 7 (`chunky`, `medium`, `large` sizes) |
| `test_chunky_roundtrip` | `hexless`/`hexaflexible` skew; `skip_chunky_skew` on decrypt (`chunky`) | `XT_WITH_ATTRIBUTES`, `XT_WITH_TARGET_MODE` | writer emitted more than one segment (assert, not skip); plaintext equal (5 MiB); audit rewrap success | `chunky_pt_file` (fixed 5 MiB), `audit_logs` | 6 (`chunky` advertised; js→gc) and 7 (bounded memory) |
| `test_tdf_spec_target_422` | pfs `hexaflexible`; encrypt `hexaflexible` | `XT_WITH_ATTRIBUTES`, `XT_WITH_TARGET_MODE=4.2.2` (`--target-mode 4.2.2`) | plaintext equal | `pt_file` | 6 |
| `test_tdf_spec_target_430` | encrypt `hexaflexible`; decrypt `hexless` | `XT_WITH_ATTRIBUTES`, `XT_WITH_TARGET_MODE=4.3.0` | plaintext equal | `pt_file` | 6 |
| `test_manifest_validity` | none | `XT_WITH_ATTRIBUTES` | manifest validates against `$SCHEMA_FILE` (`manifest.schema.json`) with `jsonschema` | encrypt only | 1 (gcgo); 2 (all) |
| `test_manifest_validity_with_assertions` | encrypt `assertions` | `XT_WITH_ATTRIBUTES`, `XT_WITH_ASSERTIONS` (`assertion_file_no_keys` path; `--with-assertions`) | schema validation | encrypt only | 5 |
| `test_tdf_assertions_unkeyed` | encrypt and decrypt `assertions`; skew | `XT_WITH_ASSERTIONS` (unsigned handling assertion), `XT_WITH_ATTRIBUTES`, `XT_WITH_TARGET_MODE` | plaintext equal (default-key verification on read) | `pt_file` | 5 |
| `test_tdf_assertions_with_keys` | encrypt `assertions`; decrypt `assertion_verification`; skew | `XT_WITH_ASSERTIONS` (HS256 and RS256 signed), `XT_WITH_ASSERTION_VERIFICATION_KEYS` (`--with-assertion-verification-keys`) | plaintext equal | `pt_file` | 5 |
| `test_tdf_assertions_422_format` | encrypt `hexaflexible` and `assertions`; decrypt `assertion_verification` | `XT_WITH_TARGET_MODE=4.2.2`, `XT_WITH_ASSERTIONS`, `XT_WITH_ASSERTION_VERIFICATION_KEYS`, `XT_WITH_ATTRIBUTES` | `looks_like_422`; plaintext equal | `pt_file` | 5 and 6 |
| `test_tdf_with_unbound_policy` | skew | `XT_WITH_ATTRIBUTES`, `XT_WITH_TARGET_MODE`; policy replaced in manifest, binding left | decrypt fails; `assert_kas_request_error`: `KAS request error\|rewrap request 400\|bad request\|InvalidArgument\|tamper\|InvalidFileError\|could not find policy in rewrap response` | none | 2 |
| `test_tdf_with_altered_policy_binding` | skew | `XT_WITH_ATTRIBUTES`; binding hash last three bytes changed | decrypt fails; `assert_kas_request_error` | none | 2 |
| `test_tdf_with_altered_root_sig` | skew | `XT_WITH_ATTRIBUTES`, `XT_WITH_TARGET_MODE`; root signature altered | decrypt fails; `root` in output; with `better-messages-2024`: `tamper\|IntegrityError`, else `integrity\|signature\|bad request` | none | 2 |
| `test_tdf_with_altered_seg_sig_wrong` | skew | as above; one segment hash altered; `XT_WITH_VERIFY_ASSERTIONS=false` | decrypt fails; `signature` in output; `tamper\|IntegrityError` | none | 2 |
| `test_tdf_with_altered_enc_seg_size` | skew | one `encryptedSegmentSize` decremented | decrypt fails; `tamper\|IntegrityError\|integrity check` | none | 2 |
| `test_tdf_with_altered_assertion_statement` | encrypt and decrypt `assertions`; skew | `XT_WITH_ASSERTIONS` (unsigned); statement value set to `tampered` | decrypt fails; `assertion` in output; `tamper\|IntegrityError\|integrity check` | none | 5 |
| `test_tdf_with_altered_assertion_with_keys` | encrypt `assertions`; decrypt `assertion_verification`; skew | `XT_WITH_ASSERTIONS` (signed), `XT_WITH_ASSERTION_VERIFICATION_KEYS`; statement altered | decrypt fails; `assertion`; `tamper\|IntegrityError\|integrity check` | none | 5 |
| `test_tdf_altered_payload_end` | skew; `dspx1153Fails` | last three payload bytes changed | decrypt fails; `segment` in output; `tamper\|IntegrityError\|integrity check` | none | 2 |
| `test_tdf_with_malicious_kao` | decrypt `kasallowlist`; skew | KAO url set to `http://localhost:9999/malicious/kas` | decrypt fails; `allowlist\|not allowed\|disallowed KASes\|AggregateError`; no audit assertion | `audit_logs` (mark only) | 2 |
| `test_tdf_with_duplicate_kao_same_kas` | decrypt `multikao`; skew | `XT_WITH_ATTRIBUTES`, `XT_WITH_TARGET_MODE`; first KAO duplicated | duplicate has same `url` and `sid`; plaintext equal | `pt_file` | 4 |

## `test_policytypes.py` (3)

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_or_attributes_success` | `skip_rts_as_needed` (focus, connectrpc, hexless skew, `ecwrap` for `ztdf-ecwrap` on both sides and pfs) | `XT_WITH_ATTRIBUTES` (alpha; beta; alpha,beta of an ANY_OF attribute; client mapped to alpha), `XT_WITH_TARGET_MODE`, `XT_WITH_ECWRAP` | policy `dataAttributes` equal the FQNs, no `dissem`; decrypt succeeds for alpha and alpha+beta, fails for beta with `forbidden\|unable to reconstruct split key` | `container`, `pt_file`; three encrypt/decrypt pairs per cell | 2 |
| `test_and_attributes_success` | as above | ALL_OF attribute; alpha succeeds; beta and alpha+beta fail | same patterns | same | 2 |
| `test_hierarchy_attributes_success` | as above | HIERARCHY attribute (client has beta): alpha fails; beta and gamma succeed | same patterns | same | 2 |

## `test_abac.py` (24)

Every cell here needs the multi-KAS platform (`kas_entry_alpha..delta`,
`km1..km3`) and `OT_ROOT_KEY` for managed keys. "km" fixtures skip when the
platform lacks `key_management`.

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_key_mapping_multiple_mechanisms` | encrypt `key_management`, `autoconfigure` (pfs too); `skip_dspx2457`; decrypt `ecwrap`; skew | `XT_WITH_ATTRIBUTES` (values mapped to public keys `r1` and `e1` on the default KAS), `XT_WITH_TARGET_MODE` | KAO kids == {`r1`, `e1`}; first KAO url == `kas_url_default`; plaintext equal | `pt_file` | 4 |
| `test_key_mapping_extended_mechanisms` | pfs `key_management`, `autoconfigure`, `mechanism-ec-curves-384-521`; encrypt `key_management`, `autoconfigure`, `mechanism-rsa-4096`, `mechanism-ec-curves-384-521`; decrypt `ecwrap` if any KAO is `ec-wrapped` | `XT_WITH_ATTRIBUTES` (ALL_OF with e256, e384, e521, r2048, r4096 managed keys on km1/km2) | five KAOs; kids == managed key ids; urls within {km1, km2}; plaintext equal | `pt_file` | 4 |
| `test_key_mapping_extended_ec_mechanisms` | pfs and encrypt `key_management`, `autoconfigure`, `mechanism-ec-curves-384-521` | EC values only (`ec-secp3*`) | KAO count and kids == e384/e521 (`e3*`); urls within {km2}; plaintext equal | `pt_file` | 4 |
| `test_key_mapping_extended_rsa_mechanisms` | encrypt `key_management`, `autoconfigure`, `mechanism-rsa-4096` | RSA values only | kids == r2048/r4096; urls within {km1}; plaintext equal | `pt_file` | 4 |
| `test_autoconfigure_one_attribute_standard` | encrypt `autoconfigure`; decrypt `ecwrap` if ec-wrapped; skew | `XT_WITH_ATTRIBUTES` (value granted to alpha KAS) | one KAO at `kas_url_alpha`; plaintext equal; audit rewrap success with the attribute FQNs | `pt_file`, `audit_logs` | 4 |
| `test_autoconfigure_two_kas_or_standard` | encrypt `autoconfigure` | two ANY_OF values granted to alpha and beta | two KAOs, same `sid`, urls == {alpha, beta}; plaintext equal; at least one rewrap audit | `pt_file`, `audit_logs` | 4 |
| `test_autoconfigure_double_kas_and` | encrypt `autoconfigure` | two ALL_OF values granted to alpha and beta | two KAOs, different `sid`; plaintext equal; at least two rewrap audits | `pt_file`, `audit_logs` | 4 |
| `test_autoconfigure_one_attribute_attr_grant` | encrypt `autoconfigure` | attribute-level grant to gamma | one KAO at gamma; plaintext equal | `pt_file` | 4 |
| `test_autoconfigure_two_kas_or_attr_and_value_grant` | encrypt `autoconfigure` | attribute grant gamma plus value grant alpha, ANY_OF | two KAOs, same sid, {gamma, alpha} | `pt_file` | 4 |
| `test_autoconfigure_two_kas_and_attr_and_value_grant` | encrypt `autoconfigure` | same, ALL_OF | two KAOs, different sid | `pt_file` | 4 |
| `test_autoconfigure_one_attribute_ns_grant` | encrypt `autoconfigure`, `ns_grants` | namespace grant to delta | one KAO at delta | `pt_file` | 4 |
| `test_autoconfigure_two_kas_or_ns_and_value_grant` | encrypt `autoconfigure`, `ns_grants` | namespace grant delta plus value grant alpha, ANY_OF | two KAOs, same sid, {delta, alpha} | `pt_file` | 4 |
| `test_autoconfigure_two_kas_and_ns_and_value_grant` | encrypt `autoconfigure`, `ns_grants` | same, ALL_OF | two KAOs, different sid | `pt_file` | 4 |
| `test_obligations_not_entitled` | `skip_rts_as_needed`; decrypt `obligations` (pfs `obligations`) | `XT_WITH_ATTRIBUTES` (value with an unscoped obligation trigger, no subject mapping), `XT_WITH_ECWRAP` for the ecwrap container | decrypt fails with `tdf: rewrap request 403\|403 for \[https?://[^\]]+\]; rewrap permission denied` and without `required\s*obligations` | `container`, `pt_file` | 9 |
| `test_obligations_not_fulfillable` | decrypt `obligations` | value mapped to the client, unscoped trigger | decrypt fails with the obligation value FQN and the 403 pattern | `container`, `pt_file` | 9 |
| `test_obligations_client_not_scoped` | decrypt `obligations` | trigger scoped to `different-client` | decrypt succeeds; plaintext equal | `container`, `pt_file` | 9 |
| `test_obligations_client_scoped` | decrypt `obligations` | trigger scoped to client `opentdf` | decrypt fails with the obligation FQN and the 403 pattern | `container`, `pt_file` | 9 |
| `test_decrypt_uses_kao_kas_registration` | pfs `key_management`, `kas_uri_from_kao` (platform-only, force via `XT_FORCE_PLATFORM_SUPPORTS`); encrypt `key_management`, `autoconfigure`; decrypt `key_management`; skew | value keyed to a key registered at `km3/kas` only | KAO url and kid match the key; plaintext equal; audit rewrap with key id and FQNs; audit line `getting keyAccessServer by Key.*"key_id".*"kas_uri"` | `pt_file`, `audit_logs` | 4 |
| `test_decrypt_rejects_kao_kas_registration_when_disabled` | same gates | key registered at `km1/kas` while km1 ignores the KAO URI | decrypt fails with `bad request\|400`; audit line `"key_access_server_keys".*uri:.*kid:` | `audit_logs` | 4 |
| `test_decrypt_same_kid_in_different_registries_with_cache` | same gates | two keys sharing a kid on km3 and `km3/kas` | two ciphertexts decrypt in rounds A,B,A,B; audit rewrap per round; audit `found private key in cache.*"cache_key"` per key | `pt_file`, `audit_logs` | 4 |
| `test_autoconfigure_key_management_two_kas_two_keys` | encrypt `key_management`, `autoconfigure`; `skip_dspx2457`; decrypt `ecwrap` if needed; skew | ALL_OF attribute with km1 RSA and km2 EC managed keys assigned at attribute level | two KAOs with the managed kids and urls {km1, km2}; plaintext equal | `pt_file` | 4 |
| `test_encrypt_with_missing_value_uses_definition_key` | encrypt `key_management`, `autoconfigure`, `attribute_traversal` | `XT_WITH_ATTRIBUTES` = a value FQN that does not exist under an attribute with `allow_traversal` and a definition-level key | policy carries the missing FQN; one KAO at gamma with the definition key id | encrypt only | 4 |
| `test_import_legacy_golden_r1_key_and_decrypt_no_split` | decrypt `key_management`, `hexless` | golden `key-management-no-split-golden.tdf` after importing the legacy `golden-r1` key into km2 | plaintext == `hello` | decrypt only | 4 |
| `test_encrypt_decrypt_all_containers_with_base_key_e1` | encrypt and decrypt `key_management`; skew | no attributes; platform base key set to km1 `e1` (EC); `XT_WITH_ECWRAP` for the ecwrap container | plaintext equal (runs last: the base key cannot be unset) | `container`, `pt_file` | 4 |

## `test_legacy.py` (7)

Decrypt-only golden files from `golden/`; `in_focus` on the decrypt SDK.

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_decrypt_small` | decrypt `hexless` | `small-java-4.3.0-e0f8caf.tdf` | 5 KiB of zero bytes | none | 6 |
| `test_decrypt_big` | decrypt `hexless` | `big-java-4.3.0-e0f8caf.tdf` | 10 MiB of zero bytes | none | 6 |
| `test_decrypt_SDKv0_7_5` | decrypt `hexless` | `xstext-java-v0.7.5-94b161d53-DSP2.0.2_and_2.0.3.tdf` | 102 bytes | none | 6 |
| `test_decrypt_SDKv0_7_8` | decrypt `hexless` | `xstext-java-v0.7.8-7f487c2-DSP2.0.4.tdf` | 92 bytes | none | 6 |
| `test_decrypt_SDKv0_9_0` | decrypt `hexless` | `xstext-java-v0.9.0-2de6a49-DSP2.0.5.1_and_2.0.6.tdf` | 92 bytes | none | 6 |
| `test_decrypt_no_splitid` | decrypt `hexless` | `no-splitids-java.tdf` (KAOs without `sid`) | 5 KiB of zero bytes | none | 6 |
| `test_decrypt_object_statement_value_json` | decrypt `assertion_verification` | `with-json-object-assertions-java.tdf`, `XT_WITH_VERIFY_ASSERTIONS=false` (`--no-verify-assertions`) | plaintext == `text` | none | 5 |

## `test_root_signature.py` (9 functions, 13 cells)

All use the 5 MiB `chunky_tdf`; `multi_segment_ct` asserts more than one
segment and applies `skip_chunky_skew`. `decrypt_failure` raises instead of
passing when the output contains an infrastructure marker (`failed to connect
to the platform`, `failed to authenticate`, `connection refused`,
`econnrefused`, `fetch failed`, `getaddrinfo`, `no such host`,
`unauthorized_client`, `invalid_client`, `connectexception`,
`unknownhostexception`) or is empty.

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_default_integrity_algorithms` | none (encrypt focus) | `XT_WITH_ATTRIBUTES` | root alg absent or `HS256`; `segmentHashAlg` `GMAC`; every segment hash decodes to 16 bytes | encrypt only | 2 |
| `test_root_signature_control_untouched_roundtrips` | `skip_hexless_skew` via fixture | untampered multi-segment file | plaintext equal | none | 6 |
| `test_root_signature_control_rewrite_roundtrips` | same | payload and manifest rewritten unchanged | plaintext equal | none | 6 |
| `test_root_signature_control_tamper_without_forgery` | same | `truncate` (drop last segment), `reverse` (segments and payload), `duplicate` (segment 0 and payload), `alg_only` (`rootSignature.alg=GMAC`, HS256 sig kept) | decrypt fails on the merits (any non-empty, non-infra output) | `scenario` x4 | 6 |
| `test_gmac_root_forged_signature_rejected` | `skip_unless_gmac_root_rejected` (probe: reader rejects a forged GMAC root; `gmac_root_rejected` is force-only) | `alg=GMAC`, sig = last 16 bytes of the aggregate hash | decrypt fails | none | 6 |
| `test_gmac_root_forged_truncation_rejected` | same | forged root plus truncation | decrypt fails | none | 6 |
| `test_gmac_root_forged_reorder_rejected` | same | forged root plus reversed segments | decrypt fails | none | 6 |
| `test_gmac_root_forged_duplicate_rejected` | same | forged root plus duplicated segment | decrypt fails | none | 6 |
| `test_gmac_root_casing_variants_rejected` | same | `alg` = `gmac`, `GMac` | decrypt fails | `alg` x2 | 6 |

## `test_zip64.py` (1)

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_zip64_band_roundtrip` | `zip64` marker (needs `--sizes medium` or `large`); `skip_hexless_skew`; encrypt `zip64-at-2gib` (asserted only when claimed); `skip_chunky_skew`; `zip64_reader_is_broken` (java < 0.19.0 must fail) | `XT_WITH_ATTRIBUTES`, `XT_WITH_TARGET_MODE`; 2.1 GiB plaintext | some central-directory offset at or above 2^31; offsets consistent; ZIP64 used above 4 GiB; if `zip64-at-2gib` claimed, no raw 32-bit value in [2^31, 2^32); plaintext equal (byte compare); output deleted | `size` (medium, large) | 7 |

## `test_dpop.py` (6)

`_dpop_client_env` sets `CLIENTID=opentdf-dpop` and `XT_WITH_DPOP=ES256` for
every test; only the js shim forwards `XT_WITH_DPOP` (`--dpop ES256`); the go
shim relies on otdfctl's default DPoP behaviour and answers `supports dpop`
from `supported_features`.

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_dpop_happy_path_roundtrip` | pfs `dpop`; encrypt and decrypt `dpop` | `XT_WITH_ATTRIBUTES` (`attribute_single_kas_grant`), `XT_WITH_TARGET_MODE`, `XT_WITH_DPOP` | plaintext equal (SDK must mint DPoP proofs for the token endpoint and KAS, send `Authorization: DPoP`) | `pt_file` | 8 |
| `test_dpop_server_issued_nonce_retry` | pfs `dpop`, `dpop_nonce_challenge`; both SDKs `dpop`, `dpop_nonce_challenge` | same | plaintext equal through a 401 + `DPoP-Nonce` retry | `pt_file` | 8 |
| `test_dpop_bearer_scheme_warns_but_accepted_for_dpop_token` | encrypt `dpop`; pfs `dpop` | encrypt only; direct HTTP rewrap with a hand-minted proof and `Authorization: Bearer` | 200 and audit line `DPoP-bound access token presented under Bearer Authorization scheme`; DPoP scheme also 200 | `audit_logs` | 8 (encrypt side) |
| `test_dpop_rejects_tampered_proof_htu` | encrypt `dpop`; pfs `dpop` | direct HTTP, wrong `htu` | 401 with `WWW-Authenticate: DPoP` | none | 8 (encrypt side) |
| `test_dpop_rejects_replayed_jti` | encrypt `dpop`; pfs `dpop` | direct HTTP, replayed `jti` | first 200, replays 401 DPoP | none | 8 (encrypt side) |
| `test_dpop_rejects_tampered_nonce` | encrypt `dpop`; pfs `dpop`, `dpop_nonce_challenge` | direct HTTP, tampered nonce | initial 401 + nonce; tampered 401 with a fresh `DPoP-Nonce` | none | 8 (encrypt side) |

## `test_pqc.py` (6)

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_xwing_roundtrip` | pfs and encrypt `key_management`, `autoconfigure`, `mechanism-xwing`; `skip_pqc_hybrid_format_skew` | value keyed to an `hpqt:xwing` key on km1 | one KAO with the kid at km1; `wrappedKey` > 1120 bytes, no `ephemeralPublicKey`; registry key DER >= 1216 bytes; plaintext equal | `pt_file` | not planned (`mechanism-xwing`; skips while a gc type encrypts, see note) |
| `test_xwing_with_ec_roundtrip` | same plus decrypt `ecwrap` if ec-wrapped | X-Wing on km1 plus EC on km2 | two KAOs; sizes as above; plaintext equal | `pt_file` | not planned |
| `test_secpmlkem_3_roundtrip` | pfs and encrypt `key_management`, `autoconfigure`, `mechanism-secpmlkem`; hybrid skew | `hpqt:secp256r1-mlkem768` key | one KAO; `wrappedKey` > 1120, DER >= 1216; plaintext equal | `pt_file` | not planned |
| `test_secpmlkem_5_roundtrip` | same | `hpqt:secp384r1-mlkem1024` key | same | `pt_file` | not planned |
| `test_mlkem_768_roundtrip` | pfs and encrypt `key_management`, `autoconfigure`, `mechanism-mlkem`; decrypt side not gated | `mlkem:768` key | KAO type `wrapped` or `mlkem-wrapped`, `wrappedKey` > 1088, DER >= 1184; plaintext equal | `pt_file` | not planned (decrypt-side gc cells fail rather than skip; see note) |
| `test_mlkem_1024_roundtrip` | same | `mlkem:1024` key | `wrappedKey` > 1568, DER >= 1568; plaintext equal | `pt_file` | not planned (same note) |

Note: the `mechanism-*` gates apply to the encrypt SDK only. When a reference
SDK encrypts with a PQ key and a gc type decrypts, `test_mlkem_*` cells run
and fail with the reader's `unsupported key access type` error; `test_xwing_*`
and `test_secpmlkem_*` behave the same unless the decrypting SDK is gated.
The plan's "skip: not planned" outcome for these cells needs a decrypt-side
gate in `tdfs.py` (a harness change) or a `--sdks-decrypt` selection that
excludes them.

## `test_audit_logs_integration.py` (9)

Autouse `skip_if_audit_disabled` skips the module without audit logs.

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `TestRewrapAudit.test_rewrap_success_fields` | skew | `XT_WITH_ATTRIBUTES` (default RSA) | plaintext equal; rewrap audit event: result `success`, type `rewrap`, object `key_object`, policy id present, platform `kas`, key id or algorithm | `pt_file`, `audit_logs` | 2 |
| `TestRewrapAudit.test_rewrap_success_with_attributes` | encrypt `autoconfigure`; skew | `attribute_single_kas_grant` | rewrap audit with the attribute FQNs | `pt_file`, `audit_logs` | 4 |
| `TestRewrapAudit.test_multiple_kao_rewrap_audit` | encrypt `autoconfigure`; decrypt `ecwrap` if ec-wrapped | ALL_OF two-KAS attribute | at least two rewrap audits | `pt_file`, `audit_logs` | 4 |
| `TestPolicyCRUDAudit.test_namespace_create_audit` | none | otdfctl only | policy create audit for the namespace | `audit_logs` | n/a: drives otdfctl, not the SDK under test |
| `TestPolicyCRUDAudit.test_attribute_create_audit` | none | otdfctl only | namespace and attribute_definition audits with two values | `audit_logs` | n/a: otdfctl |
| `TestPolicyCRUDAudit.test_subject_condition_set_audit` | none | otdfctl only | condition_set create audit | `audit_logs` | n/a: otdfctl |
| `TestDecisionAudit.test_decision_on_successful_access` | encrypt `autoconfigure`; skew | `attribute_single_kas_grant` | plaintext equal; rewrap audit; `"msg":\s*"decision"` best effort | `pt_file`, `audit_logs` | 4 |
| `TestEdgeCases.test_audit_logs_on_tampered_file` | skew | policy binding hash prefixed with `tampered` | decrypt fails (no pattern); rewrap error audit | `pt_file`, `audit_logs` | 2 |
| `TestEdgeCases.test_audit_under_sequential_load` | `slow` marker; skew | five sequential decrypts | five rewrap success audits within 10 s | `pt_file`, `audit_logs` | 2 |

## `test_self.py` (4)

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_namespaces_list` | none | otdfctl | at least four namespaces | none | n/a: otdfctl harness check |
| `test_namespace_create` | none | otdfctl | namespace id; policy create audit | `audit_logs` | n/a |
| `test_attribute_create` | none | otdfctl | two attributes differ; audits with at least six values | `audit_logs` | n/a |
| `test_scs_create` | none | otdfctl | one subject set; condition_set audit | `audit_logs` | n/a |

## `test_benchmarks.py` (1)

| Test | Features | Inputs | Asserts | Fan-out | Phase |
| --- | --- | --- | --- | --- | --- |
| `test_sdk_performance` | `benchmark` marker (`--bench`); arms resolved per SDK name from the newest final release (baseline) and the branch build (candidate); `comparability_problem` | encrypt and decrypt commands from `tdfs.SDK.*_command` with `attribute_default_rsa`; sizes from `fixtures/bench.py` | records samples only; the session gate in `conftest.py` decides | `bench_cell` (operation x payload x SDK) | 11 (needs a released gc version as baseline) |

## Harness unit tests (no SDK under test)

These modules test the harness itself (`tdfs.py`, `sizes.py`, `zipinspect.py`,
`audit_logs.py`, `perf/`) with stubs and never execute a `cli.sh`. They are
listed so that every test function has a row; their phase is `n/a`.
`test_bench_measure.py` is skipped on platforms without `/proc`.

| Module | Test | Phase |
| --- | --- | --- |
| `test_audit_logs.py` | `TestLogEntry.test_create_log_entry` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogCollector.test_collector_initialization` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogCollector.test_mark_and_get_mark` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogAsserter.test_assert_contains_success` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogAsserter.test_assert_contains_with_mark` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogAsserter.test_asserter_with_disabled_collector` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogAsserter.test_asserter_with_none_collector` | n/a: harness unit test |
| `test_audit_logs.py` | `TestParsedAuditEvent.test_parse_rewrap_log` | n/a: harness unit test |
| `test_audit_logs.py` | `TestParsedAuditEvent.test_parse_policy_crud_log` | n/a: harness unit test |
| `test_audit_logs.py` | `TestParsedAuditEvent.test_parse_decision_v2_log` | n/a: harness unit test |
| `test_audit_logs.py` | `TestParsedAuditEvent.test_parse_non_audit_log_returns_none` | n/a: harness unit test |
| `test_audit_logs.py` | `TestParsedAuditEvent.test_parse_invalid_json_returns_none` | n/a: harness unit test |
| `test_audit_logs.py` | `TestParsedAuditEvent.test_matches_rewrap_basic` | n/a: harness unit test |
| `test_audit_logs.py` | `TestParsedAuditEvent.test_matches_rewrap_with_attrs` | n/a: harness unit test |
| `test_audit_logs.py` | `TestParsedAuditEvent.test_matches_policy_crud` | n/a: harness unit test |
| `test_audit_logs.py` | `TestParsedAuditEvent.test_matches_decision` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogAsserterEnhanced.test_assert_rewrap_success_finds_match` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogAsserterEnhanced.test_assert_rewrap_error_finds_match` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogAsserterEnhanced.test_assert_policy_create_finds_match` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogAsserterEnhanced.test_assert_rewrap_with_disabled_collector` | n/a: harness unit test |
| `test_audit_logs.py` | `TestAuditLogAsserterEnhanced.test_assert_policy_crud_with_object_id` | n/a: harness unit test |
| `test_audit_logs.py` | `TestClockSkewEstimation.test_parse_rfc3339_basic` | n/a: harness unit test |
| `test_audit_logs.py` | `TestClockSkewEstimation.test_clock_skew_estimate_properties` | n/a: harness unit test |
| `test_audit_logs.py` | `TestClockSkewEstimation.test_clock_skew_estimate_negative_skew` | n/a: harness unit test |
| `test_audit_logs.py` | `TestClockSkewEstimation.test_clock_skew_estimator_record_and_retrieve` | n/a: harness unit test |
| `test_audit_logs.py` | `TestClockSkewEstimation.test_parsed_audit_event_skew_properties` | n/a: harness unit test |
| `test_audit_logs.py` | `TestClockSkewEstimation.test_asserter_skew_methods` | n/a: harness unit test |
| `test_audit_logs.py` | `TestClockSkewEstimation.test_asserter_skew_methods_disabled` | n/a: harness unit test |
| `test_audit_logs.py` | `TestClockSkewEstimation.test_skew_recorded_on_parse` | n/a: harness unit test |
| `test_audit_logs.py` | `TestClockSkewEstimation.test_resolve_since_applies_skew_adjustment` | n/a: harness unit test |
| `test_bench_arms.py` | `TestFinalRelease.test_accepts_a_plain_tag` | n/a: harness unit test |
| `test_bench_arms.py` | `TestFinalRelease.test_rejects_anything_else` | n/a: harness unit test |
| `test_bench_arms.py` | `TestBaselineSelection.test_picks_the_newest_final_release` | n/a: harness unit test |
| `test_bench_arms.py` | `TestBaselineSelection.test_a_release_candidate_never_becomes_the_baseline` | n/a: harness unit test |
| `test_bench_arms.py` | `TestBaselineSelection.test_no_final_release_is_a_clear_refusal` | n/a: harness unit test |
| `test_bench_arms.py` | `TestBaselineSelection.test_no_branch_build_is_a_clear_refusal` | n/a: harness unit test |
| `test_bench_arms.py` | `TestBaselineSelection.test_explicit_specs_win` | n/a: harness unit test |
| `test_bench_arms.py` | `TestBaselineSelection.test_refuses_to_compare_a_build_against_itself` | n/a: harness unit test |
| `test_bench_arms.py` | `TestPayloads.test_every_size_is_generated` | n/a: harness unit test |
| `test_bench_arms.py` | `TestPayloads.test_a_seed_reproduces_the_bytes` | n/a: harness unit test |
| `test_bench_arms.py` | `TestPayloads.test_a_different_seed_changes_them` | n/a: harness unit test |
| `test_bench_arms.py` | `TestPayloads.test_a_partial_cache_still_reproduces_the_bytes` | n/a: harness unit test |
| `test_bench_arms.py` | `TestPayloads.test_a_truncated_cache_entry_is_regenerated` | n/a: harness unit test |
| `test_bench_measure.py` | `TestWallClock.test_tracks_sleep_duration` | n/a: harness unit test |
| `test_bench_measure.py` | `TestWallClock.test_wall_s_matches_wall_ns` | n/a: harness unit test |
| `test_bench_measure.py` | `TestCpuTime.test_sleeping_child_burns_almost_no_cpu` | n/a: harness unit test |
| `test_bench_measure.py` | `TestCpuTime.test_busy_child_burns_cpu_close_to_wall_time` | n/a: harness unit test |
| `test_bench_measure.py` | `TestCpuTime.test_includes_grandchild_cpu` | n/a: harness unit test |
| `test_bench_measure.py` | `TestPeakRss.test_reports_ballast_in_bytes` | n/a: harness unit test |
| `test_bench_measure.py` | `TestPeakRss.test_larger_allocation_reports_larger_peak` | n/a: harness unit test |
| `test_bench_measure.py` | `TestPeakRss.test_a_fat_measuring_process_does_not_inflate_a_small_child` | n/a: harness unit test |
| `test_bench_measure.py` | `TestPeakRss.test_reports_the_floor_under_the_reading` | n/a: harness unit test |
| `test_bench_measure.py` | `TestPeakRss.test_a_reading_at_the_floor_is_marked_censored` | n/a: harness unit test |
| `test_bench_measure.py` | `TestPeakRss.test_includes_grandchild_memory` | n/a: harness unit test |
| `test_bench_measure.py` | `TestFailureHandling.test_non_zero_exit_raises_by_default` | n/a: harness unit test |
| `test_bench_measure.py` | `TestFailureHandling.test_error_includes_child_stderr` | n/a: harness unit test |
| `test_bench_measure.py` | `TestFailureHandling.test_check_false_returns_the_failing_sample` | n/a: harness unit test |
| `test_bench_measure.py` | `TestFailureHandling.test_missing_executable_raises` | n/a: harness unit test |
| `test_bench_measure.py` | `TestFailureHandling.test_timeout_kills_and_raises` | n/a: harness unit test |
| `test_bench_measure.py` | `TestFailureHandling.test_large_output_does_not_deadlock` | n/a: harness unit test |
| `test_bench_measure.py` | `TestMetricAccess.test_metric_lookup_matches_fields` | n/a: harness unit test |
| `test_bench_measure.py` | `TestMetricAccess.test_unknown_metric_raises` | n/a: harness unit test |
| `test_bench_measure.py` | `TestMetricAccess.test_every_declared_metric_is_retrievable` | n/a: harness unit test |
| `test_bench_measure.py` | `TestMetricAccess.test_formatting` | n/a: harness unit test |
| `test_bench_measure.py` | `TestMetricAccess.test_formatting_rejects_unknown_metric` | n/a: harness unit test |
| `test_bench_runner.py` | `TestRoundLoop.test_arms_are_paired_every_round` | n/a: harness unit test |
| `test_bench_runner.py` | `TestRoundLoop.test_order_within_rounds_is_shuffled` | n/a: harness unit test |
| `test_bench_runner.py` | `TestRoundLoop.test_warmup_rounds_are_discarded` | n/a: harness unit test |
| `test_bench_runner.py` | `TestRoundLoop.test_interleaving_is_reproducible_for_a_seed` | n/a: harness unit test |
| `test_bench_runner.py` | `TestRoundLoop.test_cells_do_not_share_an_interleaving` | n/a: harness unit test |
| `test_bench_runner.py` | `TestRoundLoop.test_output_is_removed_before_each_run` | n/a: harness unit test |
| `test_bench_runner.py` | `TestRoundLoop.test_samples_are_collected_for_every_metric` | n/a: harness unit test |
| `test_bench_runner.py` | `TestStopping.test_stops_early_on_precision_when_quiet` | n/a: harness unit test |
| `test_bench_runner.py` | `TestStopping.test_runs_to_max_rounds_when_noisy` | n/a: harness unit test |
| `test_bench_runner.py` | `TestStopping.test_never_stops_before_min_rounds` | n/a: harness unit test |
| `test_bench_runner.py` | `TestStopping.test_deadline_stops_the_loop` | n/a: harness unit test |
| `test_bench_runner.py` | `TestStopping.test_warmup_gives_up_when_the_budget_runs_out` | n/a: harness unit test |
| `test_bench_runner.py` | `TestStopping.test_budget_below_min_usable_rounds_refuses_a_verdict` | n/a: harness unit test |
| `test_bench_runner.py` | `TestStopping.test_budget_below_configured_minimum_refuses_a_verdict` | n/a: harness unit test |
| `test_bench_runner.py` | `TestStopping.test_deadline_caps_invocations_and_discards_a_partial_round` | n/a: harness unit test |
| `test_bench_runner.py` | `TestBenchConfigValidation.test_rejects_min_rounds_below_the_usable_floor` | n/a: harness unit test |
| `test_bench_runner.py` | `TestBenchConfigValidation.test_rejects_max_below_min` | n/a: harness unit test |
| `test_bench_runner.py` | `TestBenchConfigValidation.test_rejects_a_threshold_that_is_not_a_ratio` | n/a: harness unit test |
| `test_bench_runner.py` | `TestBenchConfigValidation.test_rejects_unknown_gated_metrics` | n/a: harness unit test |
| `test_bench_runner.py` | `TestBenchConfigValidation.test_target_half_width_is_a_third_of_the_log_threshold` | n/a: harness unit test |
| `test_bench_runner.py` | `TestBudget.test_divides_remaining_time_evenly` | n/a: harness unit test |
| `test_bench_runner.py` | `TestBudget.test_unused_time_flows_to_later_cells` | n/a: harness unit test |
| `test_bench_runner.py` | `TestBudget.test_never_hands_out_a_deadline_in_the_past` | n/a: harness unit test |
| `test_bench_runner.py` | `TestBudget.test_rejects_a_budget_with_no_cells` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_planted_25_percent_slowdown_is_caught` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_planted_3_percent_slowdown_is_ignored` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_no_effect_does_not_fire` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_planted_speedup_is_reported_not_failed` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_the_control_cell_never_fails_the_build` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_control_only_run_measured_nothing_about_the_candidate` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_control_plus_candidate_counts_as_measured` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_censored_control_only_run_still_measured_nothing` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_a_regression_in_an_ungated_metric_does_not_fail` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_rss_pinned_to_the_measurement_floor_cannot_report_pass` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_rss_above_the_floor_is_still_gated` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_each_sdk_is_judged_against_its_own_control` | n/a: harness unit test |
| `test_bench_runner.py` | `TestGateOnPlantedEffects.test_a_run_with_no_control_cannot_report_pass` | n/a: harness unit test |
| `test_bench_stats.py` | `TestLogRatios.test_recovers_exact_ratio` | n/a: harness unit test |
| `test_bench_stats.py` | `TestLogRatios.test_rejects_mismatched_lengths` | n/a: harness unit test |
| `test_bench_stats.py` | `TestLogRatios.test_rejects_non_positive` | n/a: harness unit test |
| `test_bench_stats.py` | `TestLogRatios.test_rejects_non_finite` | n/a: harness unit test |
| `test_bench_stats.py` | `TestLogRatios.test_empty_is_empty` | n/a: harness unit test |
| `test_bench_stats.py` | `TestCompare.test_point_estimate_tracks_true_ratio` | n/a: harness unit test |
| `test_bench_stats.py` | `TestCompare.test_interval_covers_truth` | n/a: harness unit test |
| `test_bench_stats.py` | `TestCompare.test_too_few_rounds_yields_no_interval` | n/a: harness unit test |
| `test_bench_stats.py` | `TestCompare.test_identical_inputs_give_unit_ratio_and_no_significance` | n/a: harness unit test |
| `test_bench_stats.py` | `TestCompare.test_constant_offset_has_degenerate_interval` | n/a: harness unit test |
| `test_bench_stats.py` | `TestCompare.test_single_outlier_round_does_not_dominate` | n/a: harness unit test |
| `test_bench_stats.py` | `TestDecisionRule.test_planted_regression_is_detected` | n/a: harness unit test |
| `test_bench_stats.py` | `TestDecisionRule.test_trivial_but_real_slowdown_does_not_fire` | n/a: harness unit test |
| `test_bench_stats.py` | `TestDecisionRule.test_planted_speedup_is_reported_but_never_fails` | n/a: harness unit test |
| `test_bench_stats.py` | `TestDecisionRule.test_borderline_effect_without_power_is_inconclusive_not_pass` | n/a: harness unit test |
| `test_bench_stats.py` | `TestDecisionRule.test_pure_noise_false_positive_rate_is_controlled` | n/a: harness unit test |
| `test_bench_stats.py` | `TestDecisionRule.test_detects_regression_across_realistic_noise` | n/a: harness unit test |
| `test_bench_stats.py` | `TestFasterTail.test_compare_reports_both_tails` | n/a: harness unit test |
| `test_bench_stats.py` | `TestFasterTail.test_identical_inputs_have_no_evidence_in_either_direction` | n/a: harness unit test |
| `test_bench_stats.py` | `TestFasterTail.test_too_few_rounds_leaves_both_tails_unset` | n/a: harness unit test |
| `test_bench_stats.py` | `TestFasterTail.test_improvement_is_decided_by_the_adjusted_faster_tail` | n/a: harness unit test |
| `test_bench_stats.py` | `TestFasterTail.test_faster_tail_is_bh_adjusted_within_its_family` | n/a: harness unit test |
| `test_bench_stats.py` | `TestFasterTail.test_adjusted_slower_tail_is_not_read_as_evidence_of_improvement` | n/a: harness unit test |
| `test_bench_stats.py` | `TestFasterTail.test_controls_and_censored_keys_enter_neither_family` | n/a: harness unit test |
| `test_bench_stats.py` | `TestFasterTail.test_a_control_reads_its_own_unadjusted_tails` | n/a: harness unit test |
| `test_bench_stats.py` | `TestNoiseFloor.test_clean_control_is_trusted` | n/a: harness unit test |
| `test_bench_stats.py` | `TestNoiseFloor.test_missing_control_is_underpowered` | n/a: harness unit test |
| `test_bench_stats.py` | `TestNoiseFloor.test_wide_control_marks_run_underpowered` | n/a: harness unit test |
| `test_bench_stats.py` | `TestNoiseFloor.test_biased_control_disables_the_gate` | n/a: harness unit test |
| `test_bench_stats.py` | `TestNoiseFloor.test_underpowered_run_cannot_report_pass` | n/a: harness unit test |
| `test_bench_stats.py` | `TestMultiplicityControl.test_bh_is_monotone_and_bounded` | n/a: harness unit test |
| `test_bench_stats.py` | `TestMultiplicityControl.test_bh_passes_nan_through` | n/a: harness unit test |
| `test_bench_stats.py` | `TestMultiplicityControl.test_correction_suppresses_lone_lucky_cell` | n/a: harness unit test |
| `test_bench_stats.py` | `TestMultiplicityControl.test_control_is_excluded_from_the_gate` | n/a: harness unit test |
| `test_bench_stats.py` | `TestMultiplicityControl.test_ungated_metric_is_reported_but_cannot_fail` | n/a: harness unit test |
| `test_bench_stats.py` | `TestMultiplicityControl.test_empty_run_reports_no_regressions` | n/a: harness unit test |
| `test_bench_stats.py` | `TestMultiplicityControl.test_empty_run_says_it_measured_nothing` | n/a: harness unit test |
| `test_bench_stats.py` | `TestMultiplicityControl.test_a_run_with_comparisons_measured_something` | n/a: harness unit test |
| `test_encryption_units.py` | `test_cache_distinguishes_plaintexts` | n/a: harness unit test |
| `test_encryption_units.py` | `test_cache_still_shares_identical_plaintext_and_parameters` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestEncryptCommand.test_positional_arguments` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestEncryptCommand.test_mime_type_defaults_on` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestEncryptCommand.test_empty_mime_type_omits_the_variable` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestEncryptCommand.test_attributes_are_comma_joined` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestEncryptCommand.test_empty_attribute_list_omits_the_variable` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestEncryptCommand.test_assertions` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestEncryptCommand.test_target_mode` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestEncryptCommand.test_ecwrap_container_maps_to_ztdf_plus_a_flag` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestEncryptCommand.test_target_mode_survives_ecwrap` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestDecryptCommand.test_positional_arguments` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestDecryptCommand.test_assertion_verification_keys` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestDecryptCommand.test_verify_assertions_only_set_when_disabled` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestDecryptCommand.test_ecwrap_flag` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestDecryptCommand.test_kas_allowlist` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestDecryptCommand.test_ecwrap_container_maps_to_ztdf` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestDeterminism.test_builders_are_pure` | n/a: harness unit test |
| `test_sdk_commands.py` | `TestDeterminism.test_no_side_effects_on_the_filesystem` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizes.test_medium_is_inside_the_broken_window` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizes.test_medium_has_margin_below_the_low_edge` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizes.test_small_and_large_sit_outside_the_window` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizes.test_which_sizes_select_the_zip64_tests` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizes.test_chunky_clears_every_sdk_default_segment` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizes.test_chunky_stays_cheap` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizeOrder.test_every_size_is_ordered` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizeOrder.test_ordering_is_cheapest_first` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizeOrder.test_a_size_missing_from_the_order_is_rejected_not_dropped` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizesOptionParsing.test_dedups_and_orders_cheapest_first` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizesOptionParsing.test_single_name` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizesOptionParsing.test_unknown_name_raises` | n/a: harness unit test |
| `test_sizes_units.py` | `TestSizesOptionParsing.test_empty_string_raises` | n/a: harness unit test |
| `test_sizes_units.py` | `TestResolveSizes.test_sizes_option_passes_through` | n/a: harness unit test |
| `test_sizes_units.py` | `TestResolveSizes.test_no_options_defaults_to_small` | n/a: harness unit test |
| `test_sizes_units.py` | `TestResolveSizes.test_large_resolves_to_small_and_large_with_warning` | n/a: harness unit test |
| `test_sizes_units.py` | `TestResolveSizes.test_large_and_sizes_together_is_a_usage_error` | n/a: harness unit test |
| `test_sizes_units.py` | `TestResolveSizes.test_result_is_cached_on_the_config` | n/a: harness unit test |
| `test_sizes_units.py` | `TestBulkPlaintext.test_partial_final_block_has_the_exact_requested_length` | n/a: harness unit test |
| `test_sizes_units.py` | `TestBulkPlaintext.test_generation_is_deterministic` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestParseForcedSupports.test_parses_comma_and_whitespace_separated_names` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestParseForcedSupports.test_empty_string_yields_empty_set` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestParseForcedSupports.test_unknown_name_raises` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestParseForcedSupports.test_platform_only_name_is_rejected_by_the_sdk_override` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestParseForcedSupports.test_platform_only_name_is_accepted_by_the_platform_override` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestParseForcedSupports.test_both_sided_features_stay_forceable_from_either_side` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestParseForcedSupports.test_a_typo_is_reported_as_unknown_not_as_misdirected` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestParseForcedSupports.test_the_sdk_valid_list_does_not_advertise_platform_only_names` | n/a: harness unit test |
| `test_tdfs_units.py` | `test_platform_forced_supports` | n/a: harness unit test |
| `test_tdfs_units.py` | `test_platform_forced_supports_do_not_leak` | n/a: harness unit test |
| `test_tdfs_units.py` | `test_platform_override_does_not_override_sdk` | n/a: harness unit test |
| `test_tdfs_units.py` | `test_unknown_platform_override_names_source` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestZip64ReaderIsBroken.test_pre_fix_java_is_broken` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestZip64ReaderIsBroken.test_the_fix_release_itself_is_not` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestZip64ReaderIsBroken.test_a_later_java_is_not` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestZip64ReaderIsBroken.test_another_sdk_is_never_broken` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestZip64ReaderIsBroken.test_a_branch_build_is_not` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestElidesSegmentSizes.test_true_when_any_segment_omits_a_size` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestElidesSegmentSizes.test_false_when_every_segment_is_fully_sized` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestSkipChunkySkew.test_skips_when_needed_and_unsupported` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestSkipChunkySkew.test_does_not_skip_when_reader_supports_chunky` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestSkipChunkySkew.test_does_not_skip_when_container_does_not_elide` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestForgeGMACRootSignature.test_sig_is_the_trailing_16_bytes_of_the_aggregate` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestForgeGMACRootSignature.test_decodes_segment_hashes_exactly_once` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestForgeGMACRootSignature.test_legacy_container_hex_encodes_before_base64` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestForgeGMACRootSignature.test_casing_is_preserved` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestForgeGMACRootSignature.test_too_short_an_aggregate_raises` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestEncryptedSegmentSizes.test_falls_back_to_the_manifest_default` | n/a: harness unit test |
| `test_tdfs_units.py` | `TestEncryptedSegmentSizes.test_uses_the_per_segment_override` | n/a: harness unit test |
| `test_zip64_units.py` | `TestZip64Selection.test_mixed_session_uses_each_items_size` | n/a: harness unit test |
| `test_zip64_units.py` | `TestZip64Selection.test_item_without_size_uses_session_selection` | n/a: harness unit test |
| `test_zip64_units.py` | `TestZip64Deselection.test_a_default_run_drops_the_zip64_cells` | n/a: harness unit test |
| `test_zip64_units.py` | `TestZip64Deselection.test_a_medium_run_keeps_them` | n/a: harness unit test |
| `test_zip64_units.py` | `TestZip64Deselection.test_a_mixed_run_keeps_only_the_cells_that_reach_the_window` | n/a: harness unit test |
| `test_zip64_units.py` | `TestZip64Deselection.test_benchmarks_need_their_own_opt_in` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_reads_a_real_zip` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_local_header_zip64_is_not_mistaken_for_central_directory_zip64` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_reads_a_zip64_end_of_central_directory` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_rejects_a_locator_pointing_at_nothing` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_raw_value_in_the_window_is_preserved` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_signed_read_of_a_windowed_offset_goes_negative` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_signed_read_is_harmless_below_the_window` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_sentinel_resolves_through_the_extra_field` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_each_zip64_value_lands_in_its_own_field` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_skips_a_foreign_extra_field_record` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_rejects_a_file_with_no_eocd` | n/a: harness unit test |
| `test_zip64_units.py` | `TestCentralDirectory.test_rejects_a_sentinel_with_no_zip64_extra` | n/a: harness unit test |
| `test_zip64_units.py` | `TestMalformedExtraField.test_rejects_more_values_than_the_sentinels_call_for` | n/a: harness unit test |
| `test_zip64_units.py` | `TestMalformedExtraField.test_rejects_fewer_values_than_the_sentinels_call_for` | n/a: harness unit test |
| `test_zip64_units.py` | `TestMalformedExtraField.test_rejects_an_empty_zip64_record` | n/a: harness unit test |
| `test_zip64_units.py` | `TestMalformedExtraField.test_rejects_a_record_claiming_more_than_remains` | n/a: harness unit test |
| `test_zip64_units.py` | `TestMalformedExtraField.test_tolerates_the_trailing_disk_start_field` | n/a: harness unit test |
| `test_zip64_units.py` | `TestTruncatedContainers.test_truncated_eocd` | n/a: harness unit test |
| `test_zip64_units.py` | `TestTruncatedContainers.test_truncated_central_directory_record` | n/a: harness unit test |
| `test_zip64_units.py` | `TestTruncatedContainers.test_central_directory_record_with_a_name_past_the_end` | n/a: harness unit test |
| `test_zip64_units.py` | `TestTruncatedContainers.test_zip64_locator_pointing_past_the_end_of_the_file` | n/a: harness unit test |
| `test_zip64_units.py` | `TestTruncatedContainers.test_central_directory_extending_past_the_end_of_the_file` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_a_container_past_4gib_must_use_zip64` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_a_container_past_4gib_with_zip64_passes` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_a_container_below_4gib_is_not_required_to_use_zip64` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_an_offset_truncated_mod_2_32_is_caught` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_an_entry_at_or_after_the_central_directory_is_caught` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_two_entries_claiming_one_offset_are_caught` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_a_well_formed_container_is_consistent` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_window_entries_are_reported_for_either_encoding` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_a_compressed_size_in_the_window_is_reported` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_only_raw_window_values_exercise_signed_read` | n/a: harness unit test |
| `test_zip64_units.py` | `TestConformanceAssertions.test_describe_includes_the_numbers_needed_to_debug` | n/a: harness unit test |
