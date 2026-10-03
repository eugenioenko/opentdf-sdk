use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use opentdf_tdf3::*;
use serde_json::{json, Value};
use std::{
    fs,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};
fn text(v: &Value, k: &str) -> String {
    v[k].as_str().unwrap_or_default().to_owned()
}
fn config(v: &Value) -> Config {
    Config {
        platform_url: text(v, "PlatformURL"),
        kas_url: text(v, "KASURL"),
        allowed_kas: v["AllowedKAS"]
            .as_array()
            .map(|a| {
                a.iter()
                    .map(|v| KASRoute {
                        url: text(v, "URL"),
                        api_base_url: text(v, "APIBaseURL"),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        issuer_url: text(v, "IssuerURL"),
        token_url: text(v, "TokenURL"),
        client_id: text(v, "ClientID"),
        client_secret: text(v, "ClientSecret"),
        token_provider_name: text(v, "TokenProviderName"),
        allow_http: v["AllowHTTP"].as_bool().unwrap_or(false),
        timeout_millis: v["TimeoutMillis"].as_i64().unwrap_or(0),
        kas_public_key_pem: text(v, "KASPublicKeyPEM"),
        kid: text(v, "KID"),
        kas_algorithm: text(v, "KASAlgorithm"),
        session_algorithm: text(v, "SessionAlgorithm"),
        auth_private_key_pem: text(v, "AuthPrivateKeyPEM"),
        auth_algorithm: text(v, "AuthAlgorithm"),
        dpop: v["DPoP"].as_bool().unwrap_or(false),
    }
}
fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
fn b64(b: impl AsRef<[u8]>) -> String {
    URL_SAFE_NO_PAD.encode(b)
}
fn token(c: &Config, name: &str, r: ProviderRequest) -> Result<AccessToken, ProviderError> {
    if name == "invalid-token" || name == "expired-token" {
        return Ok(AccessToken {
            value: name.into(),
            scheme: "Bearer".into(),
            expires_at: now() + if name == "expired-token" { -1 } else { 300 },
            confirmation_jkt: String::new(),
        });
    }
    if name == "provider-reject" {
        return Err(ProviderError::Rejected("provider rejected".into()));
    }
    let endpoint = "http://localhost:8888/auth/realms/opentdf/protocol/openid-connect/token";
    let key = if c.dpop {
        Some(if name == "mismatched-provider" {
            openssl::pkey::PKey::from_ec_key(
                openssl::ec::EcKey::generate(
                    &openssl::ec::EcGroup::from_curve_name(openssl::nid::Nid::X9_62_PRIME256V1)
                        .unwrap(),
                )
                .unwrap(),
            )
            .unwrap()
        } else {
            openssl::pkey::PKey::private_key_from_pem(c.auth_private_key_pem.as_bytes()).unwrap()
        })
    } else {
        None
    };
    let jwk=key.as_ref().map(|p|{if p.id()==openssl::pkey::Id::RSA{let k=p.rsa().unwrap();json!({"e":b64(k.e().to_vec()),"kty":"RSA","n":b64(k.n().to_vec())})}else{let k=p.ec_key().unwrap();let mut x=openssl::bn::BigNum::new().unwrap();let mut y=openssl::bn::BigNum::new().unwrap();k.public_key().affine_coordinates_gfp(k.group(),&mut x,&mut y,&mut openssl::bn::BigNumContext::new().unwrap()).unwrap();json!({"crv":"P-256","kty":"EC","x":b64(x.to_vec_padded(32).unwrap()),"y":b64(y.to_vec_padded(32).unwrap())})}});
    let thumb = jwk
        .as_ref()
        .map(|j| b64(openssl::sha::sha256(j.to_string().as_bytes())))
        .unwrap_or_default();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let result=runtime.block_on(async{
 let client=reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).pool_max_idle_per_host(0).timeout(Duration::from_secs(15)).build().unwrap();let mut nonce=String::new();
 for _ in 0..3{if r.cancellation.is_canceled(){return Err(ProviderError::Rejected("provider stopped".into()))};let mut request=client.post(endpoint).header("Content-Type","application/x-www-form-urlencoded");
 if let Some(key)=&key{let mut random=[0;16];openssl::rand::rand_bytes(&mut random).unwrap();let mut claims=json!({"htu":endpoint,"htm":"POST","iat":now(),"jti":b64(random)});if !nonce.is_empty(){claims["nonce"]=json!(nonce)};let data=format!("{}.{}",b64(json!({"typ":"dpop+jwt","alg":if key.id()==openssl::pkey::Id::RSA{"RS256"}else{"ES256"},"jwk":jwk}).to_string()),b64(claims.to_string()));let mut signer=openssl::sign::Signer::new(openssl::hash::MessageDigest::sha256(),key).unwrap();signer.update(data.as_bytes()).unwrap();let mut sig=signer.sign_to_vec().unwrap();if key.id()!=openssl::pkey::Id::RSA{let s=openssl::ecdsa::EcdsaSig::from_der(&sig).unwrap();sig=s.r().to_vec_padded(32).unwrap();sig.extend(s.s().to_vec_padded(32).unwrap())};request=request.header("DPoP",format!("{}.{}",data,b64(sig)))}
 let response=request.body("grant_type=client_credentials&client_id=opentdf-sdk&client_secret=secret").send().await.map_err(|_|ProviderError::Rejected("provider transport".into()))?;let status=response.status();let next=response.headers().get("DPoP-Nonce").and_then(|s|s.to_str().ok()).unwrap_or_default().to_owned();let body=response.bytes().await.map_err(|_|ProviderError::Rejected("provider body".into()))?;if (status.as_u16()==400||status.as_u16()==401)&&!next.is_empty()&&next!=nonce{nonce=next;continue};if status.as_u16()!=200||body.len()>128*1024{return Err(ProviderError::Rejected("provider status".into()))};let v:Value=serde_json::from_slice(&body).map_err(|_|ProviderError::Rejected("provider JSON".into()))?;return Ok(AccessToken{value:text(&v,"access_token"),scheme:if c.dpop{"DPoP"}else{"Bearer"}.into(),expires_at:now()+v["expires_in"].as_i64().unwrap(),confirmation_jkt:thumb.clone()})};Err(ProviderError::Rejected("provider nonce retries".into()))
 });
    drop(runtime);
    result
}
fn call(c: &Config, name: &str) -> CallOptions {
    let mut o = CallOptions::default();
    if !c.token_provider_name.is_empty() {
        let cfg = c.clone();
        let name = name.to_owned();
        o.providers.insert(
            c.token_provider_name.clone(),
            token_provider(move |r| token(&cfg, &name, r)),
        );
    }
    o
}
fn opts(name: &str) -> EncryptOptions {
    let metadata = name == "metadata" || name.ends_with("-metadata");
    let empty = name == "empty-metadata" || name.ends_with("-empty-metadata");
    EncryptOptions {
        attributes: vec![format!(
            "https://example.com/attr/attr1/value/{}",
            if name == "denied" || name.ends_with("-denied") {
                "value2"
            } else {
                "value1"
            }
        )],
        segment_size: 16384,
        has_segment_size: true,
        segment_hash_algorithm: if name == "hs256" || name.ends_with("-hs256") {
            "HS256".into()
        } else {
            String::new()
        },
        metadata: if metadata && !empty {
            b"{\"source\":\"independent metadata\",\"count\":7}".to_vec()
        } else {
            vec![]
        },
        include_metadata: metadata,
        ..Default::default()
    }
}
fn kind(e: &LibraryError) -> &'static str {
    match e.kind {
        ErrorKind::Source => "source",
        ErrorKind::Canceled => "canceled",
        ErrorKind::DeadlineExceeded => "deadline_exceeded",
        ErrorKind::SourcePanic => "source_panic",
        ErrorKind::SourceFatal => "source_fatal",
        ErrorKind::HostFault => "host_fault",
        ErrorKind::InvalidArgument => "invalid_argument",
    }
}
fn write_error(run: &Path, name: &str, e: &LibraryError) {
    fs::write(run.join(format!("{name}.error.json")),json!({"kind":kind(e),"code":e.text("Code"),"operation":e.text("Operation"),"httpStatus":e.integer("HTTPStatus"),"causeCategory":e.text("CauseCategory"),"serverCode":e.text("ServerCode"),"serverMessage":e.text("ServerMessage")}).to_string()).unwrap()
}
fn repeat(c: &Config) -> Result<(), LibraryError> {
    let mut payload = vec![0, 255, 128, 1];
    let mut option = opts("metadata");
    option.metadata = vec![0, 255];
    option.mime_type = "application/日本語".into();
    let work = encrypt(
        c.clone(),
        payload.clone(),
        option.clone(),
        call(c, "repeat"),
    );
    payload.fill(120);
    option.metadata.fill(120);
    let archive = work.wait()?;
    let a = decrypt(c.clone(), archive.clone(), call(c, "repeat")).wait()?;
    let b = decrypt(c.clone(), archive.clone(), call(c, "repeat")).wait()?;
    assert_eq!(a, b);
    assert_eq!(a.Payload, vec![0, 255, 128, 1]);
    assert_eq!(a.Metadata, vec![0, 255]);
    assert!(String::from_utf8_lossy(&a.ManifestJSON).contains("日本語"));
    let mut cfg = c.clone();
    cfg.token_provider_name = "held".into();
    let started = Arc::new(AtomicBool::new(false));
    let released = Arc::new(AtomicBool::new(false));
    let s = started.clone();
    let r = released.clone();
    let mut option = CallOptions::default();
    option.providers.insert(
        "held".into(),
        Arc::new(move |request| {
            s.store(true, Ordering::Release);
            while !request.cancellation.is_canceled() {
                std::thread::sleep(Duration::from_millis(2))
            }
            std::thread::sleep(Duration::from_millis(40));
            r.store(true, Ordering::Release);
            Err(ProviderError::Rejected("stopped".into()))
        }),
    );
    let active = decrypt(cfg.clone(), archive.clone(), option);
    for _ in 0..10000 {
        if started.load(Ordering::Acquire) {
            break;
        };
        std::thread::sleep(Duration::from_millis(2))
    }
    assert!(started.load(Ordering::Acquire));
    let queued = decrypt(c.clone(), archive.clone(), call(c, "repeat"));
    queued.cancel();
    assert_eq!(queued.wait().unwrap_err().kind, ErrorKind::Canceled);
    assert!(!released.load(Ordering::Acquire));
    active.cancel();
    assert_eq!(active.wait().unwrap_err().kind, ErrorKind::Canceled);
    assert!(released.load(Ordering::Acquire));
    assert_eq!(
        decrypt(c.clone(), archive, call(c, "repeat"))
            .wait()?
            .Payload,
        a.Payload
    );
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let archive = encrypt(
            c.clone(),
            b"async\0\xff".to_vec(),
            opts("binary"),
            call(c, "repeat"),
        )
        .await?;
        assert_eq!(
            decrypt(c.clone(), archive, call(c, "repeat"))
                .await?
                .Payload,
            b"async\0\xff"
        );
        Ok(())
    })
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let run = Path::new(&args[2]);
    let mode = &args[3];
    let name = &args[4];
    let raw: Value = serde_json::from_slice(&fs::read(run.join("config.json")).unwrap()).unwrap();
    let mut c = config(&raw);
    let result: Result<(), LibraryError> = match mode.as_str() {
        "smoke" => {
            let mut input = vec![0, 255, 128];
            let mut cfg = Config::default();
            cfg.platform_url = "http://invalid.example".into();
            cfg.kas_url = "http://invalid.example/kas".into();
            cfg.allow_http = true;
            cfg.issuer_url = "http://invalid.example/issuer".into();
            cfg.client_id = "native-proof".into();
            cfg.client_secret = "test-only".into();
            cfg.kas_public_key_pem =
                String::from_utf8(fs::read(run.join("rsa.public.pem")).unwrap()).unwrap();
            cfg.kid = "explicit-native-proof".into();
            let mut options = opts("empty-metadata");
            let work = encrypt(cfg, input.clone(), options.clone(), CallOptions::default());
            input.fill(7);
            options.metadata.push(7);
            work.wait().map(|archive|{assert!(!archive.is_empty());assert_eq!(decrypt(Config::default(),vec![0],CallOptions::default()).wait().unwrap_err().kind,ErrorKind::Source);println!("PASS installed Cargo package native Config/Vec/Result boundary and explicit-key TDF creation");})
        }
        "encrypt" => encrypt(
            c.clone(),
            fs::read(run.join(format!("{name}.input"))).unwrap(),
            opts(name),
            call(&c, name),
        )
        .wait()
        .map(|v| fs::write(run.join(format!("{name}.generated.tdf")), v).unwrap()),
        "decrypt" => decrypt(
            c.clone(),
            fs::read(run.join(format!("{name}.tdf"))).unwrap(),
            call(&c, name),
        )
        .wait()
        .map(|v| {
            for (suffix, bytes) in [
                ("out", v.Payload),
                ("metadata", v.Metadata),
                ("manifest", v.ManifestJSON),
            ] {
                fs::write(run.join(format!("{name}.{suffix}")), bytes).unwrap()
            }
            fs::write(
                run.join(format!("{name}.presence")),
                if v.HasMetadata { "true" } else { "false" },
            )
            .unwrap()
        }),
        "negative" => {
            let e = decrypt(
                c.clone(),
                fs::read(run.join(format!("{name}.tdf"))).unwrap(),
                call(&c, name),
            )
            .wait()
            .unwrap_err();
            write_error(run, name, &e);
            Ok(())
        }
        "repeat" => repeat(&c),
        "controlled" => {
            c.token_provider_name = "controlled".into();
            let mut o = CallOptions::default();
            o.providers.insert(
                "controlled".into(),
                token_provider(|_| {
                    Ok(AccessToken {
                        value: "negative-fixture-token".into(),
                        scheme: "Bearer".into(),
                        expires_at: now() + 300,
                        confirmation_jkt: String::new(),
                    })
                }),
            );
            let work = decrypt(c, fs::read(run.join("archive.tdf")).unwrap(), o);
            let cancel = work.cancellation();
            let stop = if name == "active-cancel" {
                let run = run.to_owned();
                Some(std::thread::spawn(move || {
                    for _ in 0..7500 {
                        if run.join("entered").exists() {
                            cancel.cancel();
                            return;
                        };
                        std::thread::sleep(Duration::from_millis(2))
                    }
                    panic!("transport not entered")
                }))
            } else {
                None
            };
            let e = work.wait().unwrap_err();
            let expected = match name.as_str() {
                "http401" => "unauthenticated",
                "http403" => "denied",
                "redirect" => "http_status",
                "content-type" => "invalid_content_type",
                "malformed" => "invalid_json",
                "bom" => "invalid_destination",
                _ => "transport",
            };
            if name == "active-cancel" {
                assert_eq!(e.kind, ErrorKind::Canceled);
                assert_eq!(e.text("CauseCategory"), "canceled")
            } else {
                assert_eq!(e.text("Code"), expected)
            };
            if name == "http401" || name == "http403" {
                assert_eq!(e.text("ServerCode"), "fixture-rejected");
                assert_eq!(e.text("ServerMessage"), "診断 café")
            };
            write_error(run, name, &e);
            if let Some(s) = stop {
                s.join().unwrap()
            };
            assert_eq!(
                decrypt(Config::default(), vec![0], CallOptions::default())
                    .wait()
                    .unwrap_err()
                    .kind,
                ErrorKind::Source
            );
            Ok(())
        }
        _ => panic!("unknown mode"),
    };
    if let Err(e) = result {
        eprintln!(
            "consumer failure {mode} {name}: kind={} code={} operation={} cause={} diagnostic={}",
            kind(&e),
            e.text("Code"),
            e.text("Operation"),
            e.text("CauseCategory"),
            e.message
        );
        std::process::exit(1)
    }
}
