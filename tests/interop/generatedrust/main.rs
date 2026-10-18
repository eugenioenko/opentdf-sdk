//! Independent native consumer of the installed crate's handle API: one SDK
//! per run, create_tdf/decrypt_bytes, load_tdf readers, 100 round trips, the
//! platform service clients, and the controlled checks of
//! tests/interop/delivery/controlled_platform.py.
//! Host OAuth/DPoP is native Rust.
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use opentdf_tdf3::protocol::policy::{attributes, namespaces, AttributeRuleTypeEnum};
use opentdf_tdf3::{self as tdf, AccessToken, AssertionConfig, AssertionKey, AssertionVerificationKeys, Error, KasInfo, Options, ProviderError, ProviderRequest, ReaderOptions, Sdk, Statement, TdfOptions};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    future::Future,
    path::Path,
    pin::pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::{Context, Poll, Wake, Waker},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const ATTRIBUTE: &str = "https://example.com/attr/attr1/value/value1";

fn text(v: &Value, k: &str) -> String {
    v[k].as_str().unwrap_or_default().to_owned()
}
fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64
}
fn b64(b: impl AsRef<[u8]>) -> String {
    URL_SAFE_NO_PAD.encode(b)
}
fn check(ok: bool, label: &str) {
    if !ok {
        panic!("ASSERT: {label}")
    }
}

// The host token source: native OAuth client credentials with a DPoP proof when the profile asks for one.
fn token(cfg: &Value, name: &str, r: ProviderRequest) -> Result<AccessToken, ProviderError> {
    if name == "invalid-token" || name == "expired-token" {
        return Ok(AccessToken { value: name.into(), scheme: "Bearer".into(), expires_at: now() + if name == "expired-token" { -1 } else { 300 }, confirmation_jkt: String::new() });
    }
    if name == "provider-reject" {
        return Err(ProviderError::Rejected("provider rejected".into()));
    }
    let endpoint = "http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token";
    let dpop = cfg["DPoP"].as_bool().unwrap_or(false);
    let key = if dpop {
        Some(if name == "mismatched-provider" {
            openssl::pkey::PKey::from_ec_key(openssl::ec::EcKey::generate(&openssl::ec::EcGroup::from_curve_name(openssl::nid::Nid::X9_62_PRIME256V1).unwrap()).unwrap()).unwrap()
        } else {
            openssl::pkey::PKey::private_key_from_pem(text(cfg, "AuthPrivateKeyPEM").as_bytes()).unwrap()
        })
    } else {
        None
    };
    let jwk=key.as_ref().map(|p|{if p.id()==openssl::pkey::Id::RSA{let k=p.rsa().unwrap();json!({"e":b64(k.e().to_vec()),"kty":"RSA","n":b64(k.n().to_vec())})}else{let k=p.ec_key().unwrap();let mut x=openssl::bn::BigNum::new().unwrap();let mut y=openssl::bn::BigNum::new().unwrap();k.public_key().affine_coordinates_gfp(k.group(),&mut x,&mut y,&mut openssl::bn::BigNumContext::new().unwrap()).unwrap();json!({"crv":"P-256","kty":"EC","x":b64(x.to_vec_padded(32).unwrap()),"y":b64(y.to_vec_padded(32).unwrap())})}});
    let thumb = jwk.as_ref().map(|j| b64(openssl::sha::sha256(j.to_string().as_bytes()))).unwrap_or_default();
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    let result=runtime.block_on(async{
 let client=reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).pool_max_idle_per_host(0).timeout(Duration::from_secs(15)).build().unwrap();let mut nonce=String::new();
 for _ in 0..3{if r.cancellation.is_canceled(){return Err(ProviderError::Rejected("provider stopped".into()))};let mut request=client.post(endpoint).header("Content-Type","application/x-www-form-urlencoded");
 if let Some(key)=&key{let mut random=[0;16];openssl::rand::rand_bytes(&mut random).unwrap();let mut claims=json!({"htu":endpoint,"htm":"POST","iat":now(),"jti":b64(random)});if !nonce.is_empty(){claims["nonce"]=json!(nonce)};let data=format!("{}.{}",b64(json!({"typ":"dpop+jwt","alg":if key.id()==openssl::pkey::Id::RSA{"RS256"}else{"ES256"},"jwk":jwk}).to_string()),b64(claims.to_string()));let mut signer=openssl::sign::Signer::new(openssl::hash::MessageDigest::sha256(),key).unwrap();signer.update(data.as_bytes()).unwrap();let mut sig=signer.sign_to_vec().unwrap();if key.id()!=openssl::pkey::Id::RSA{let s=openssl::ecdsa::EcdsaSig::from_der(&sig).unwrap();sig=s.r().to_vec_padded(32).unwrap();sig.extend(s.s().to_vec_padded(32).unwrap())};request=request.header("DPoP",format!("{}.{}",data,b64(sig)))}
 let response=request.body("grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret").send().await.map_err(|_|ProviderError::Rejected("provider transport".into()))?;let status=response.status();let next=response.headers().get("DPoP-Nonce").and_then(|s|s.to_str().ok()).unwrap_or_default().to_owned();let body=response.bytes().await.map_err(|_|ProviderError::Rejected("provider body".into()))?;if (status.as_u16()==400||status.as_u16()==401)&&!next.is_empty()&&next!=nonce{nonce=next;continue};if status.as_u16()!=200||body.len()>128*1024{return Err(ProviderError::Rejected("provider status".into()))};let v:Value=serde_json::from_slice(&body).map_err(|_|ProviderError::Rejected("provider JSON".into()))?;return Ok(AccessToken{value:text(&v,"access_token"),scheme:if dpop{"DPoP"}else{"Bearer"}.into(),expires_at:now()+v["expires_in"].as_i64().unwrap(),confirmation_jkt:thumb.clone()})};Err(ProviderError::Rejected("provider nonce retries".into()))
 });
    drop(runtime);
    result
}

// config.json carries the façade Config fields the delivery scripts write; they map onto the handle API's options as the Go consumer maps them.
fn options_of(cfg: &Value, name: &str, credentials: bool) -> Options {
    let provider_cfg = cfg.clone();
    let provider_name = name.to_owned();
    Options {
        insecure_plaintext: cfg["AllowHTTP"].as_bool().unwrap_or(false),
        token_endpoint: text(cfg, "TokenURL"),
        client_id: if credentials { text(cfg, "ClientID") } else { String::new() },
        client_secret: if credentials { text(cfg, "ClientSecret") } else { String::new() },
        token_provider: if text(cfg, "TokenProviderName").is_empty() { None } else { Some(tdf::token_provider(move |r| token(&provider_cfg, &provider_name, r))) },
        timeout_millis: cfg["TimeoutMillis"].as_i64().unwrap_or(0),
        dpop_algorithm: if cfg["DPoP"].as_bool().unwrap_or(false) { if text(cfg, "AuthAlgorithm").is_empty() { "ES256".into() } else { text(cfg, "AuthAlgorithm") } } else { String::new() },
        dpop_key_pem: text(cfg, "AuthPrivateKeyPEM"),
        ..Default::default()
    }
}
fn kas_information(cfg: &Value) -> KasInfo {
    let url = if text(cfg, "KASURL").is_empty() { text(cfg, "PlatformURL").trim_end_matches('/').to_owned() + "/kas" } else { text(cfg, "KASURL") };
    KasInfo { url, public_key: text(cfg, "KASPublicKeyPEM"), kid: text(cfg, "KID"), algorithm: text(cfg, "KASAlgorithm"), default: false }
}
fn tdf_options(cfg: &Value) -> TdfOptions {
    TdfOptions { disable_autoconfigure: true, kas_information: vec![kas_information(cfg)], attributes: vec![ATTRIBUTE.into()], segment_size: 16384, wrapping_key_algorithm: text(cfg, "KASAlgorithm"), ..Default::default() }
}
fn reader_options(cfg: &Value) -> ReaderOptions {
    let mut allow = vec![text(cfg, "PlatformURL")];
    if !text(cfg, "KASURL").is_empty() {
        allow.push(text(cfg, "KASURL"));
    }
    for r in cfg["AllowedKAS"].as_array().map(|a| a.as_slice()).unwrap_or_default() {
        allow.push(text(r, "URL"));
    }
    ReaderOptions { kas_allowlist: Some(allow), session_key_type: text(cfg, "SessionAlgorithm"), ..Default::default() }
}
fn report(e: &Error) -> Value {
    json!({"kind": e.kind.as_str(), "op": e.op, "httpStatus": e.http_status, "connectCode": e.connect_code, "assertionId": e.assertion_id, "message": e.message})
}
fn expect_failure<T>(label: &str, result: Result<T, Error>) -> Value {
    match result {
        Ok(_) => panic!("{label} returned successful output"),
        Err(e) => report(&e),
    }
}
/// A TDF with an unsigned assertion (HS256 over the payload key) and one signed with a caller HS256 secret,
/// read with the right key by id, with a wrong key and with no key (assertion_failure), and with verification off.
fn assertions(sdk: &Sdk, cfg: &Value, run: &Path) -> Result<(), Error> {
    let secret = format!("assertion-secret-{:x}", now());
    let first_value = r#"{"ocl":{"pol":"2ccf11cb-6c9a-4e49-9746-a7f0a295945d"}}"#;
    let configs = vec![
        AssertionConfig {
            id: "unsigned".into(),
            type_: "handling".into(),
            scope: "tdo".into(),
            applies_to_state: "encrypted".into(),
            statement: Statement { format: "json+stanag5636".into(), schema: "urn:nato:stanag:5636:A:1:elements:json".into(), value: first_value.into() },
            signing_key: None,
        },
        AssertionConfig {
            id: "signed".into(),
            type_: "other".into(),
            scope: "payload".into(),
            applies_to_state: "unencrypted".into(),
            statement: Statement { format: "json".into(), schema: "urn:example:assertion:v1".into(), value: r#"{"note":"independent consumer"}"#.into() },
            signing_key: Some(AssertionKey::hs256(&secret)),
        },
    ];
    let payload = b"assertion payload".to_vec();
    let object = sdk.create_tdf(&payload, &TdfOptions { assertions: configs, ..tdf_options(cfg) })?;
    let found = object.manifest()?.assertions;
    object.close()?;
    let archive = object.bytes.clone();
    check(found.len() == 2 && found[0].id == "unsigned" && found[1].id == "signed" && found[1].binding.method == "jws" && !found[1].binding.signature.is_empty(), "assertions in manifest");
    check(found[0].statement.value == first_value, "statement value kept");
    let with_keys = |default_key: Option<AssertionKey>, keys: &[(&str, AssertionKey)]| ReaderOptions {
        assertion_verification_keys: AssertionVerificationKeys { default_key, keys: keys.iter().map(|(id, k)| (id.to_string(), k.clone())).collect() },
        ..reader_options(cfg)
    };
    check(sdk.decrypt_bytes(&archive, &with_keys(None, &[("signed", AssertionKey::hs256(&secret))]))? == payload, "decrypt with the right key");
    let fails = |label: &str, options: &ReaderOptions, id: &str| {
        let error = expect_failure(label, sdk.decrypt_bytes(&archive, options));
        check(error["kind"] == "assertion_failure" && error["assertionId"] == id, &format!("{label}: {error}"));
        error
    };
    let wrong = fails("wrong key", &with_keys(None, &[("signed", AssertionKey::hs256("not the secret"))]), "signed");
    let missing = fails("missing key", &reader_options(cfg), "signed");
    fails("wrong default key", &with_keys(Some(AssertionKey::hs256("not the secret")), &[]), "unsigned");
    check(sdk.decrypt_bytes(&archive, &ReaderOptions { disable_assertion_verification: true, ..reader_options(cfg) })? == payload, "decrypt with verification disabled");
    fs::write(run.join("assertions.generated.tdf"), &archive).unwrap();
    let ids: Vec<Value> = found.iter().map(|a| json!({"id": a.id, "method": a.binding.method})).collect();
    fs::write(run.join("assertions.json"), json!({"assertions": ids, "wrong_key": wrong, "missing_key": missing}).to_string()).unwrap();
    Ok(())
}
/// Polls a future for at most the duration, then drops it (the Rust cancellation idiom).
fn poll_for<F: Future>(future: F, limit: Duration) -> Option<F::Output> {
    poll_until(future, limit, || false)
}
/// Polls a future until it completes, the duration passes or stop() holds, then drops it.
fn poll_until<F: Future>(future: F, limit: Duration, stop: impl Fn() -> bool) -> Option<F::Output> {
    struct Noop;
    impl Wake for Noop {
        fn wake(self: Arc<Self>) {}
    }
    let waker = Waker::from(Arc::new(Noop));
    let mut cx = Context::from_waker(&waker);
    let mut future = pin!(future);
    let start = Instant::now();
    loop {
        if let Poll::Ready(v) = future.as_mut().poll(&mut cx) {
            return Some(v);
        }
        if start.elapsed() > limit || stop() {
            return None;
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

/// xtest's test_self.py through the service clients: list namespaces, create a
/// namespace and an attribute, read the attribute back by id, through the
/// namespace's list and through the raw JSON call, and check a missing
/// attribute's error.
fn services(sdk: &Sdk, cfg: &Value, name: &str, run: &Path) -> Result<(), Error> {
    let ns = format!("gcrs{}.com", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
    let before = sdk.namespaces().list_namespaces(&namespaces::ListNamespacesRequest::default())?;
    let namespace = sdk.namespaces().create_namespace(&namespaces::CreateNamespaceRequest { name: ns.clone(), ..Default::default() })?.namespace.unwrap_or_default();
    check(!namespace.id.is_empty() && namespace.name == ns, "created namespace");
    let after = poll_for(sdk.namespaces().list_namespaces_async(&namespaces::ListNamespacesRequest::default()), Duration::from_secs(60)).expect("async list namespaces")?;
    check(after.namespaces.iter().any(|n| n.id == namespace.id) && after.namespaces.len() == before.namespaces.len() + 1, "namespace listed");
    let created = sdk.attributes().create_attribute(&attributes::CreateAttributeRequest {
        namespace_id: namespace.id.clone(),
        name: "free".into(),
        rule: AttributeRuleTypeEnum::AnyOf as i32,
        values: vec!["1".into(), "2".into(), "3".into()],
        ..Default::default()
    })?;
    let by_id = |id: String| attributes::GetAttributeRequest { identifier: Some(attributes::get_attribute_request::Identifier::AttributeId(id)), ..Default::default() };
    let attribute = sdk.attributes().get_attribute(&by_id(created.attribute.unwrap_or_default().id))?.attribute.unwrap_or_default();
    let values: Vec<String> = attribute.values.iter().map(|v| v.value.clone()).collect();
    check(
        attribute.fqn == format!("https://{ns}/attr/free") && attribute.rule == AttributeRuleTypeEnum::AnyOf as i32 && values == ["1", "2", "3"] && attribute.namespace.as_ref().is_some_and(|n| n.name == ns),
        "attribute read back",
    );
    let listed = sdk.attributes().list_attributes(&attributes::ListAttributesRequest { namespace: ns.clone(), ..Default::default() })?;
    check(listed.attributes.len() == 1 && listed.attributes[0].id == attribute.id, "attribute listed");
    let raw: Value = serde_json::from_str(&sdk.call("policy.attributes.AttributesService", "GetAttribute", &json!({"attributeId": attribute.id}).to_string())?).unwrap();
    check(raw["attribute"]["fqn"] == attribute.fqn.as_str(), "raw JSON call");
    let missing = expect_failure("missing attribute", sdk.attributes().get_attribute(&by_id("00000000-0000-0000-0000-000000000000".into())));
    check(missing["connectCode"] == "not_found" && missing["httpStatus"] == 404, &format!("missing attribute error: {missing}"));
    let helpers = helpers(sdk, cfg, name, &ns, &attribute.fqn)?;
    let autoconfigure = autoconfigure(sdk, cfg, &attribute)?;
    let report = json!({"namespace": ns, "namespaces_before": before.namespaces.len(), "namespaces_after": after.namespaces.len(), "attribute_fqn": attribute.fqn, "values": values, "not_found": missing, "helpers": helpers, "autoconfigure": autoconfigure});
    fs::write(run.join("services.json"), report.to_string()).unwrap();
    Ok(())
}

/// Encrypts under a value of the attribute created, looked up (attributes) and offline (attribute_values): the attribute has no grants, so the plan is the base key or the default KAS.
fn autoconfigure(sdk: &Sdk, cfg: &Value, attribute: &opentdf_tdf3::protocol::policy::Attribute) -> Result<Value, Error> {
    let value = format!("{}/value/1", attribute.fqn);
    let offline = opentdf_tdf3::protocol::policy::Value {
        fqn: value.clone(),
        value: "1".into(),
        attribute: Some(opentdf_tdf3::protocol::policy::Attribute { fqn: attribute.fqn.clone(), rule: attribute.rule, namespace: attribute.namespace.clone(), ..Default::default() }),
        ..Default::default()
    };
    let mut report = json!({"key": tdf::new_attribute_value_fqn(&value)?.key});
    for (label, options) in [
        ("looked_up", TdfOptions { kas_information: vec![kas_information(cfg)], attributes: vec![value.clone()], ..Default::default() }),
        ("offline", TdfOptions { kas_information: vec![kas_information(cfg)], attribute_values: vec![offline.clone()], ..Default::default() }),
    ] {
        let object = sdk.create_tdf(label.as_bytes(), &options)?;
        let kaos = object.manifest()?.encryption_information.key_access;
        object.close()?;
        report[label] = json!({"kaos": kaos.len(), "kas": kaos[0].url, "sid": kaos[0].sid});
    }
    Ok(report)
}

/// The reference's attribute helpers on the attribute services created, Sdk::auth, and an Sdk whose platform endpoint is unreachable but whose core connection is the platform.
fn helpers(sdk: &Sdk, cfg: &Value, name: &str, ns: &str, fqn: &str) -> Result<Value, Error> {
    let listed: Vec<String> = sdk.list_attributes(Some(ns))?.into_iter().map(|a| a.fqn).collect();
    sdk.validate_attributes(&[format!("{fqn}/value/1"), format!("{fqn}/value/2")])?;
    let validate_missing = expect_failure("validate missing", sdk.validate_attributes(&[format!("{fqn}/value/1"), format!("{fqn}/value/9")]));
    let entity = opentdf_tdf3::protocol::authorization::Entity {
        id: "e1".into(),
        entity_type: Some(opentdf_tdf3::protocol::authorization::entity::EntityType::ClientId("opentdf-sdk".into())),
        ..Default::default()
    };
    let token = sdk.auth().access_token()?;
    let over = Sdk::new("http://127.0.0.1:9", Options { core_endpoint: text(cfg, "PlatformURL"), ..options_of(cfg, name, true) })?;
    let override_report = json!({"endpoint": over.conn()?.endpoint, "attribute_exists": over.attribute_exists(fqn)?});
    over.close()?;
    Ok(json!({
        "attribute_exists": sdk.attribute_exists(fqn)?,
        "attribute_missing": sdk.attribute_exists(&format!("{fqn}-missing"))?,
        "value_exists": sdk.attribute_value_exists(&format!("{fqn}/value/1"))?,
        "value_missing": poll_for(sdk.attribute_value_exists_async(&format!("{fqn}/value/9")), Duration::from_secs(60)).expect("async value exists")?,
        "listed": listed,
        "validate_ok": true,
        "validate_missing": validate_missing,
        "entity_attributes": sdk.get_entity_attributes(Some(&entity))?,
        "access_token": {"present": !token.value.is_empty(), "scheme": token.scheme},
        "override": override_report,
    }))
}

/// 100 encrypt and decrypt calls on one SDK, independent inputs and results, every handle closed; then the SDK is closed and later calls fail.
fn repeat(sdk: &Sdk, cfg: &Value, name: &str) -> Result<(), Error> {
    let original = vec![0u8, 255, 128, 1];
    let mut archive = vec![];
    for i in 0..100u8 {
        let mut input = original.clone();
        input.push(i);
        let object = sdk.create_tdf(&input, &TdfOptions { metadata: format!("round {i}"), mime_type: "application/日本語".into(), ..tdf_options(cfg) })?;
        check(object.size()? == object.bytes.len() as i64, "size");
        check(object.manifest()?.payload.mime_type == "application/日本語", "UTF-8 manifest");
        object.close()?;
        archive = object.bytes.clone();
        input[0] = 9;
        let reader = sdk.load_tdf(&archive, &reader_options(cfg))?;
        check(reader.data_attributes()?.len() == 1, "attributes");
        reader.close()?;
        let plain = sdk.decrypt_bytes(&archive, &reader_options(cfg))?;
        let mut expected = original.clone();
        expected.push(i);
        check(plain == expected, "round trip");
    }
    let wrong = expect_failure("wrong credentials", Sdk::new(&text(cfg, "PlatformURL"), Options { client_secret: "wrong".into(), ..options_of(cfg, name, true) }).and_then(|s| sdk_decrypt(&s, &archive, cfg)));
    check(wrong["kind"] == "auth" && wrong["httpStatus"] == 401, &format!("wrong credentials rejected: {wrong}"));
    let started = Arc::new(AtomicBool::new(false));
    let released = Arc::new(AtomicBool::new(false));
    let (s, r) = (started.clone(), released.clone());
    let held: tdf::Provider = Arc::new(move |request: ProviderRequest| {
        s.store(true, Ordering::Release);
        while !request.cancellation.is_canceled() {
            std::thread::sleep(Duration::from_millis(2))
        }
        r.store(true, Ordering::Release);
        Err(ProviderError::Rejected("stopped".into()))
    });
    let holder = Sdk::new(&text(cfg, "PlatformURL"), Options { token_provider: Some(held), ..options_of(cfg, name, false) })?;
    // Dropping the pending future once the provider has started is the
    // cancellation; the provider then observes it and releases.
    let pending = holder.decrypt_bytes_async(&archive, &reader_options(cfg));
    let watch = started.clone();
    let outcome = poll_until(pending, Duration::from_secs(20), move || watch.load(Ordering::Acquire));
    check(outcome.is_none(), "held provider returned before cancellation");
    check(started.load(Ordering::Acquire), "provider started");
    let deadline = Instant::now() + Duration::from_secs(20);
    while !released.load(Ordering::Acquire) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(5));
    }
    check(released.load(Ordering::Acquire), "dropped future cancelled the provider");
    holder.close()?;
    let async_archive = poll_for(sdk.encrypt_bytes_async(b"async\0\xff", &tdf_options(cfg)), Duration::from_secs(60)).expect("async encrypt")?;
    check(poll_for(sdk.decrypt_bytes_async(&async_archive, &reader_options(cfg)), Duration::from_secs(60)).expect("async decrypt")? == b"async\0\xff", "async round trip");
    sdk.close()?;
    check(expect_failure("closed", sdk.decrypt_bytes(&archive, &reader_options(cfg)))["kind"] == "closed", "closed handle");
    Ok(())
}
fn sdk_decrypt(s: &Sdk, archive: &[u8], cfg: &Value) -> Result<Vec<u8>, Error> {
    let r = s.decrypt_bytes(archive, &reader_options(cfg));
    let _ = s.close();
    r
}

/// One controlled case against the harness fixture (tests/interop/delivery/controlled_platform.py).
fn controlled(run: &Path, cfg: &Value, name: &str) -> Result<(), Error> {
    let url = text(cfg, "PlatformURL");
    let offline = TdfOptions { disable_autoconfigure: true, kas_information: vec![KasInfo { url: "http://localhost:9999/malicious/kas".into(), public_key: text(cfg, "KASPublicKeyPEM"), kid: "k1".into(), algorithm: "rsa:2048".into(), default: false }], attributes: vec!["https://example.com/attr/a/value/b".into()], ..Default::default() };
    let credentials = || Options { client_id: "opentdf".into(), client_secret: "secret".into(), ..Default::default() };
    let error = match name {
        "unreachable" => expect_failure(name, Sdk::new(&url, Options { validate_connection: true, ..Default::default() })),
        "caller-deadline" => match poll_for(Sdk::new_async(&url, credentials()), Duration::from_secs(1)) {
            Some(result) => expect_failure(name, result),
            None => json!({"kind": "canceled", "op": "", "httpStatus": 0, "connectCode": "", "message": "future dropped before completion"}),
        },
        "source-deadline" => expect_failure(name, Sdk::new(&url, Options { timeout_millis: 100, ..credentials() })),
        "token-401" => {
            let sdk = Sdk::new(&url, credentials())?;
            let archive = sdk.encrypt_bytes(b"offline payload", &offline)?;
            let error = expect_failure(name, sdk.decrypt_bytes(&archive, &ReaderOptions { ignore_allowlist: true, ..Default::default() }));
            sdk.close()?;
            error
        }
        "offline" => {
            let mut rows = BTreeMap::new();
            let sdk = Sdk::new(&url, credentials())?;
            let archive = sdk.encrypt_bytes(b"offline payload", &offline)?;
            rows.insert("allowlist", expect_failure("allowlist", sdk.decrypt_bytes(&archive, &ReaderOptions { kas_allowlist: Some(vec![url.clone()]), ..Default::default() })));
            rows.insert("invalid-tdf", expect_failure("invalid-tdf", sdk.decrypt_bytes(b"not a tdf", &ReaderOptions::default())));
            rows.insert("unsupported-option", expect_failure("unsupported-option", sdk.create_tdf(b"x", &TdfOptions { target_mode: "4.2.2".into(), ..Default::default() })));
            for _ in 0..100 {
                let reader = sdk.load_tdf(&archive, &ReaderOptions { ignore_allowlist: true, ..Default::default() })?;
                check(reader.manifest()?.encryption_information.key_access[0].kid == "k1", "manifest");
                reader.close()?;
            }
            sdk.close()?;
            rows.insert("closed-handle", expect_failure("closed-handle", sdk.decrypt_bytes(&archive, &ReaderOptions::default())));
            rows.insert("handles", json!({"readers": 100, "closed": true}));
            check(tdf::is_valid_tdf(&archive)? && !tdf::version()?.is_empty() && tdf::supported_features()?.iter().any(|f| f == "hexless"), "free functions");
            fs::write(run.join("offline.json"), json!(rows).to_string()).unwrap();
            return Ok(());
        }
        _ => expect_failure(name, Sdk::new(&url, credentials())),
    };
    fs::write(run.join(format!("{name}.error.json")), error.to_string()).unwrap();
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let run = Path::new(&args[2]);
    let mode = &args[3];
    let name = &args[4];
    let cfg: Value = serde_json::from_slice(&fs::read(run.join("config.json")).unwrap()).unwrap();
    let result: Result<(), Error> = if mode == "controlled" {
        controlled(run, &cfg, name)
    } else {
        (|| {
            let sdk = Sdk::new(&text(&cfg, "PlatformURL"), options_of(&cfg, name, true))?;
            match mode.as_str() {
                "encrypt" => {
                    let mut options = tdf_options(&cfg);
                    if (name == "metadata" || name.ends_with("-metadata")) && name != "empty-metadata" && !name.ends_with("-empty-metadata") {
                        options.metadata = "{\"source\":\"independent metadata\",\"count\":7}".into();
                    }
                    if name == "denied" || name.ends_with("-denied") {
                        options.attributes = vec!["https://example.com/attr/attr1/value/value2".into()];
                    }
                    let split = name == "split" || name.ends_with("-split");
                    if split {
                        options.kas_information = vec![kas_information(&cfg), kas_information(&cfg)];
                    }
                    let object = sdk.create_tdf(&fs::read(run.join(format!("{name}.input"))).unwrap(), &options)?;
                    check(object.size()? == object.bytes.len() as i64 && object.manifest()?.encryption_information.key_access.len() == if split { 2 } else { 1 }, "object");
                    object.close()?;
                    fs::write(run.join(format!("{name}.generated.tdf")), &object.bytes).unwrap();
                }
                "decrypt" => {
                    let input = fs::read(run.join(format!("{name}.tdf"))).unwrap();
                    let reader = sdk.load_tdf(&input, &reader_options(&cfg))?;
                    let manifest = reader.manifest()?;
                    let has_metadata = manifest.encryption_information.key_access.first().is_some_and(|k| !k.encrypted_metadata.is_empty());
                    let metadata = if has_metadata {
                        reader.init()?;
                        reader.unencrypted_metadata()?
                    } else {
                        vec![]
                    };
                    reader.close()?;
                    let payload = sdk.decrypt_bytes(&input, &reader_options(&cfg))?;
                    fs::write(run.join(format!("{name}.out")), payload).unwrap();
                    fs::write(run.join(format!("{name}.metadata")), metadata).unwrap();
                    fs::write(run.join(format!("{name}.manifest")), format!("{manifest:?}")).unwrap();
                    fs::write(run.join(format!("{name}.presence")), if has_metadata { "true" } else { "false" }).unwrap();
                }
                "negative" => {
                    let error = expect_failure("negative", sdk.decrypt_bytes(&fs::read(run.join(format!("{name}.tdf"))).unwrap(), &reader_options(&cfg)));
                    fs::write(run.join(format!("{name}.error.json")), error.to_string()).unwrap();
                }
                "assertions" => assertions(&sdk, &cfg, run)?,
                "repeat" => {
                    repeat(&sdk, &cfg, name)?;
                    return Ok(());
                }
                "services" => services(&sdk, &cfg, name, run)?,
                _ => panic!("unknown mode"),
            }
            sdk.close()
        })()
    };
    match result {
        Ok(()) => println!("PASS generated native library {mode} {name}"),
        Err(e) => {
            eprintln!("consumer failure {mode} {name}: {}", report(&e));
            std::process::exit(1)
        }
    }
}
