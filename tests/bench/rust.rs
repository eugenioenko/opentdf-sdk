use opentdf_tdf3::*;
use serde_json::{json, Value};
use std::{fs, path::Path, time::Instant};
fn text(v: &Value, k: &str) -> String {
    v[k].as_str().unwrap_or_default().into()
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let run = Path::new(&args[1]);
    let op = &args[2];
    let size = &args[3];
    let n: i32 = args[4].parse().unwrap();
    let raw: Value = serde_json::from_slice(&fs::read(run.join("private.json")).unwrap()).unwrap();
    let r = &raw["Config"];
    let input = fs::read(run.join(format!("{size}.input"))).unwrap();
    let archive = if op == "decrypt" {
        fs::read(run.join(format!("{size}.reference.tdf"))).unwrap()
    } else {
        vec![]
    };
    let mut samples = vec![];
    for i in -1..n {
        // Rust's consuming Vec API needs an owned input each call; prepare it untimed.
        let owned = if op == "encrypt" {
            input.clone()
        } else {
            archive.clone()
        };
        let start = Instant::now();
        let c = Config {
            platform_url: text(r, "PlatformURL"),
            kas_url: text(r, "KASURL"),
            allowed_kas: vec![KASRoute {
                url: "http://localhost:8080/kas".into(),
                api_base_url: "http://localhost:8080".into(),
            }],
            allow_http: true,
            kas_public_key_pem: text(r, "KASPublicKeyPEM"),
            kid: text(r, "KID"),
            kas_algorithm: "rsa:2048".into(),
            session_algorithm: "rsa:2048".into(),
            auth_algorithm: "ES256".into(),
            token_provider_name: "access-token".into(),
            ..Default::default()
        };
        let token = text(&raw, "Token");
        let expires = raw["Expires"].as_i64().unwrap();
        let mut call = CallOptions::default();
        call.providers.insert(
            "access-token".into(),
            token_provider(move |_| {
                Ok(AccessToken {
                    value: token.clone(),
                    scheme: "Bearer".into(),
                    expires_at: expires,
                    confirmation_jkt: String::new(),
                })
            }),
        );
        let output = if op == "encrypt" {
            encrypt(
                c,
                owned,
                EncryptOptions {
                    attributes: vec!["https://example.com/attr/attr1/value/value1".into()],
                    segment_size: 2 << 20,
                    has_segment_size: true,
                    segment_hash_algorithm: "GMAC".into(),
                    ..Default::default()
                },
                call,
            )
            .wait()
            .unwrap()
        } else {
            decrypt(c, owned, call).wait().unwrap().Payload
        };
        let ms = start.elapsed().as_secs_f64() * 1000.;
        if op == "decrypt" {
            assert_eq!(input, output)
        } else {
            fs::write(run.join(format!("rust-{size}-{i}.tdf")), output).unwrap()
        };
        if i >= 0 {
            samples.push(ms)
        };
    }
    println!("{}", json!({"samples_ms":samples,"correct":true}));
}
