use opentdf_tdf3::*;
use serde_json::{json, Value};
use std::{fs, path::Path, time::Instant};

fn text(v: &Value, k: &str) -> String {
    v[k].as_str().unwrap_or_default().into()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let run = Path::new(&args[1]);
    assert_eq!(args[2], "e2e");
    let size = &args[3];
    let n: i32 = args[4].parse().unwrap();
    let raw: Value = serde_json::from_slice(&fs::read(run.join("private.json")).unwrap()).unwrap();
    let r = &raw["Config"];
    let input = fs::read(run.join(format!("{size}.input"))).unwrap();
    let cfg = Config {
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
    let options = EncryptOptions {
        attributes: vec!["https://example.com/attr/attr1/value/value1".into()],
        segment_size: 2 << 20,
        has_segment_size: true,
        segment_hash_algorithm: "GMAC".into(),
        ..Default::default()
    };
    let token = text(&raw, "Token");
    let expires = raw["Expires"].as_i64().unwrap();
    let provider = token_provider(move |_| {
        Ok(AccessToken {
            value: token.clone(),
            scheme: "Bearer".into(),
            expires_at: expires,
            confirmation_jkt: String::new(),
        })
    });
    let mut samples = vec![];
    for i in -1..n {
        // Prepare consuming API arguments and harness configuration untimed.
        let owned = input.clone();
        let encrypt_cfg = cfg.clone();
        let decrypt_cfg = cfg.clone();
        let encrypt_options = options.clone();
        let mut encrypt_call = CallOptions::default();
        let mut decrypt_call = CallOptions::default();
        encrypt_call
            .providers
            .insert("access-token".into(), provider.clone());
        decrypt_call
            .providers
            .insert("access-token".into(), provider.clone());
        let start = Instant::now();
        let archive = encrypt(encrypt_cfg, owned, encrypt_options, encrypt_call)
            .wait()
            .unwrap();
        // The decrypt API consumes its Vec. Retention for the independent oracle
        // requires this clone; its cost is deliberately inside the interval.
        let output = decrypt(decrypt_cfg, archive.clone(), decrypt_call)
            .wait()
            .unwrap()
            .Payload;
        let elapsed = start.elapsed().as_secs_f64() * 1000.;
        assert_eq!(input, output);
        fs::write(run.join(format!("rust-{size}-{i}.tdf")), archive).unwrap();
        if i >= 0 {
            samples.push(elapsed);
        }
    }
    println!(
        "{}",
        json!({"samples_ms": samples, "correct": true, "kas_calls_expected": n + 1, "archive_retention_clone_timed": true})
    );
}
