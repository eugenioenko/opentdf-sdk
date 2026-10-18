//! Rust handle-API benchmark: one Sdk per process, as the reference consumer.
use opentdf_tdf3::{self as tdf, AccessToken, KasInfo, Options, ReaderOptions, Sdk, TdfOptions};
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
    let token = text(&raw, "Token");
    let expires = raw["Expires"].as_i64().unwrap();
    let sdk = Sdk::new(
        &text(r, "PlatformURL"),
        Options {
            insecure_plaintext: true,
            token_endpoint: text(r, "TokenURL"),
            token_provider: Some(tdf::token_provider(move |_| {
                Ok(AccessToken { value: token.clone(), scheme: "Bearer".into(), expires_at: expires, confirmation_jkt: String::new() })
            })),
            ..Default::default()
        },
    )
    .unwrap();
    let options = TdfOptions {
        disable_autoconfigure: true,
        kas_information: vec![KasInfo { url: text(r, "KASURL"), public_key: text(r, "KASPublicKeyPEM"), kid: text(r, "KID"), algorithm: "rsa:2048".into(), default: false }],
        wrapping_key_algorithm: "rsa:2048".into(),
        segment_size: 2 << 20,
        attributes: vec!["https://example.com/attr/attr1/value/value1".into()],
        ..Default::default()
    };
    // load_tdf reuses the Sdk's default RSA2048 session. Do not generate a replacement.
    let reader_options = ReaderOptions { kas_allowlist: Some(vec![text(r, "KASURL")]), ..Default::default() };
    let mut samples = vec![];
    let mut warmup_history = vec![];
    let mut bulk_history = vec![];
    for i in -bulk_warmups - warmups..n {
        let pair_input = if i < -warmups && size != "50" { &bulk_input } else { &input };
        let start = Instant::now();
        let archive = sdk.encrypt_bytes(pair_input, &options).unwrap();
        let output = sdk.decrypt_bytes(&archive, &reader_options).unwrap();
        let elapsed = start.elapsed().as_secs_f64() * 1000.;
        assert_eq!(*pair_input, output);
        if i == -1 || i >= 0 {
            fs::write(run.join(format!("rust-{size}-{i}.tdf")), &archive).unwrap();
        }
        if i >= 0 {
            samples.push(elapsed);
        } else if i < -warmups {
            bulk_history.push(elapsed);
        } else {
            warmup_history.push(elapsed);
        }
    }
    sdk.close().unwrap();
    println!(
        "{}",
        json!({"samples_ms": samples, "warmup_ms": warmup_history, "bulk_warmup_ms": bulk_history, "warmup_count": warmups, "bulk_warmup_count": bulk_warmups, "correct": true, "kas_calls_expected": n + warmups + bulk_warmups})
    );
}
