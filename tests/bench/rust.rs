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
    let warmups: i32 = args.get(5).map(|s| s.parse().unwrap()).unwrap_or(1);
    let bulk_warmups: i32 = args.get(6).map(|s| s.parse().unwrap()).unwrap_or(0);
    assert!(n > 0 && warmups > 0 && bulk_warmups >= 0);
    let raw: Value = serde_json::from_slice(&fs::read(run.join("private.json")).unwrap()).unwrap();
    let r = &raw["Config"];
    let input = fs::read(run.join(format!("{size}.input"))).unwrap();
    let bulk_input = if bulk_warmups > 0 && size != "50" {
        fs::read(run.join("50.input")).unwrap()
    } else {
        vec![]
    };
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
    let mut warmup_history = vec![];
    let mut bulk_history = vec![];
    for i in -bulk_warmups - warmups..n {
        let pair_input = if i < -warmups && size != "50" {
            &bulk_input
        } else {
            &input
        };
        // Prepare consuming API arguments and harness configuration untimed.
        let owned = pair_input.clone();
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
        assert_eq!(*pair_input, output);
        if i == -1 || i >= 0 {
            fs::write(run.join(format!("rust-{size}-{i}.tdf")), archive).unwrap();
        }
        if i >= 0 {
            samples.push(elapsed);
        } else if i < -warmups {
            bulk_history.push(elapsed);
        } else {
            warmup_history.push(elapsed);
        }
    }
    println!(
        "{}",
        json!({"samples_ms": samples, "warmup_ms": warmup_history, "bulk_warmup_ms": bulk_history, "warmup_count": warmups, "bulk_warmup_count": bulk_warmups, "correct": true, "kas_calls_expected": n + warmups + bulk_warmups, "archive_retention_clone_timed": true})
    );
}
