//! otdfgc is the native Rust CLI over the generated crate. It implements the
//! xtest CLI contract of docs/full-sdk-plan.md exactly as the Go otdfgc does:
//! one SDK per invocation, created with --host and the client credentials;
//! every flag this build cannot honour exits 2 as unsupported.
use opentdf_tdf3::{self as tdf, Error, Kind, KasInfo, Options, ReaderOptions, Sdk, TdfOptions};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::process::ExitCode;

const EXIT_SUCCESS: u8 = 0;
const EXIT_ERROR: u8 = 1;
const EXIT_USAGE: u8 = 2;
const TARGET: &str = "rust";
const USAGE: &str = "Usage:
  otdfgc encrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--attr <fqn,...>] [--mime-type <t>] [--wrapping-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]
      [--target-mode 4.3.0] [--policy-mode encrypted] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc decrypt <in> --out <file> --host <url> --with-client-creds <json>
      [--no-verify-assertions] [--session-key-algorithm rsa:2048|ec:secp256r1|rsa:4096|ec:secp384r1|ec:secp521r1]
      [--kas-allowlist <url,...|*>] [--dpop ES256|RS256] [--dpop-key <pem>] [--tls-no-verify]
  otdfgc inspect <in>
  otdfgc supports <feature>
  otdfgc --version [--json]
";
const COMMON: &[(&str, bool)] = &[("out", false), ("host", false), ("with-client-creds", false), ("with-client-creds-file", false), ("dpop", false), ("dpop-key", false), ("tls-no-verify", true), ("log-level", false)];
const ENCRYPT_FLAGS: &[(&str, bool)] = &[("attr", false), ("mime-type", false), ("with-assertions", false), ("wrapping-key-algorithm", false), ("target-mode", false), ("policy-mode", false), ("ecdsa-binding", true)];
const DECRYPT_FLAGS: &[(&str, bool)] = &[("with-assertion-verification-keys", false), ("no-verify-assertions", true), ("session-key-algorithm", false), ("kas-allowlist", false)];
const DPOP_ALGORITHMS: &[&str] = &["ES256", "ES384", "ES512", "RS256", "RS384", "RS512"];
// Rules naming the flags this build cannot honour, optionally only for values other than the allowed ones.
const ENCRYPT_UNSUPPORTED: &[(&str, &[&str])] = &[("with-assertions", &[]), ("ecdsa-binding", &[]), ("target-mode", &["4.3.0"]), ("policy-mode", &["encrypted"]), ("wrapping-key-algorithm", &["rsa:2048", "ec:secp256r1", "rsa:4096", "ec:secp384r1", "ec:secp521r1"]), ("dpop", DPOP_ALGORITHMS), ("with-client-creds-file", &[])];
const DECRYPT_UNSUPPORTED: &[(&str, &[&str])] = &[("with-assertion-verification-keys", &[]), ("session-key-algorithm", &["rsa:2048", "ec:secp256r1", "rsa:4096", "ec:secp384r1", "ec:secp521r1"]), ("dpop", DPOP_ALGORITHMS), ("with-client-creds-file", &[])];

#[derive(Default)]
struct Parsed {
    values: HashMap<String, String>,
    set: HashSet<String>,
    positional: Vec<String>,
}
impl Parsed {
    fn value(&self, name: &str) -> &str {
        self.values.get(name).map(String::as_str).unwrap_or("")
    }
    fn flag(&self, name: &str) -> bool {
        self.set.contains(name)
    }
}

fn parse(args: &[String], spec: &[(&str, bool)]) -> Result<Parsed, String> {
    let specs: HashMap<&str, bool> = COMMON.iter().chain(spec).copied().collect();
    let mut p = Parsed::default();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if !a.starts_with('-') || a == "-" {
            p.positional.push(a.clone());
            i += 1;
            continue;
        }
        let mut name = a.trim_start_matches('-').to_owned();
        let mut value = String::new();
        let mut inline = false;
        if let Some((n, v)) = name.clone().split_once('=') {
            name = n.to_owned();
            value = v.to_owned();
            inline = true;
        }
        if name == "o" {
            name = "out".to_owned();
        }
        let Some(&is_bool) = specs.get(name.as_str()) else { return Err(format!("unknown flag: {a}")) };
        if is_bool {
            if inline && value != "true" && value != "false" {
                return Err(format!("flag --{name} takes no value"));
            }
            if !inline || value == "true" {
                p.set.insert(name);
            }
            i += 1;
            continue;
        }
        if !inline {
            if i + 1 >= args.len() {
                return Err(format!("flag --{name} needs a value"));
            }
            i += 1;
            value = args[i].clone();
        }
        p.values.insert(name.clone(), value);
        p.set.insert(name);
        i += 1;
    }
    Ok(p)
}

fn unsupported(p: &Parsed, rules: &[(&str, &[&str])]) -> String {
    for (name, allowed) in rules {
        if !p.flag(name) {
            continue;
        }
        if allowed.is_empty() {
            return format!("--{name}");
        }
        if !allowed.contains(&p.value(name)) {
            return format!("--{name} {}", p.value(name));
        }
    }
    String::new()
}

fn split_list(s: &str) -> Vec<String> {
    s.split(',').map(str::trim).filter(|x| !x.is_empty()).map(str::to_owned).collect()
}

// describe renders an SDK failure: the reference SDK's wording plus otdfctl's "required obligations" suffix.
fn describe(e: &Error) -> String {
    let mut msg = e.message.clone();
    if !e.required_obligations.is_empty() {
        msg += &format!("\nrequired obligations: [{}]", e.required_obligations.join(" "));
    }
    msg
}
fn describe_new(e: &Error, host: &str) -> String {
    match &e.kind {
        Kind::Auth => format!("Failed to authenticate with flag-provided client credentials. ({})", e.message),
        Kind::Transport | Kind::DeadlineExceeded => format!("Failed to connect to the platform. Is the platform accepting connections at '{host}'? ({})", e.message),
        _ if e.message.contains("platform unreachable or not responding") => format!("Failed to connect to the platform. Is the platform accepting connections at '{host}'? ({})", e.message),
        _ => e.message.clone(),
    }
}

// A minimal JSON reader for the credentials and a writer for inspect.
#[derive(Debug)]
enum Json {
    Null,
    Bool(bool),
    Number(String),
    Str(String),
    List(Vec<Json>),
    Object(Vec<(String, Json)>),
}
struct JsonParser<'a> {
    s: &'a [u8],
    i: usize,
}
impl<'a> JsonParser<'a> {
    fn ws(&mut self) {
        while self.i < self.s.len() && (self.s[self.i] as char).is_ascii_whitespace() {
            self.i += 1;
        }
    }
    fn expect(&mut self, c: u8) -> Result<(), String> {
        if self.i < self.s.len() && self.s[self.i] == c {
            self.i += 1;
            Ok(())
        } else {
            Err(format!("expected {}", c as char))
        }
    }
    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out: Vec<u8> = vec![];
        loop {
            let Some(&c) = self.s.get(self.i) else { return Err("unterminated string".into()) };
            self.i += 1;
            match c {
                b'"' => return String::from_utf8(out).map_err(|e| e.to_string()),
                b'\\' => {
                    let Some(&e) = self.s.get(self.i) else { return Err("unterminated escape".into()) };
                    self.i += 1;
                    match e {
                        b'n' => out.push(b'\n'),
                        b't' => out.push(b'\t'),
                        b'r' => out.push(b'\r'),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        b'u' => {
                            let hex = std::str::from_utf8(self.s.get(self.i..self.i + 4).ok_or("bad escape")?).map_err(|e| e.to_string())?;
                            let code = u32::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
                            self.i += 4;
                            let mut buffer = [0; 4];
                            out.extend_from_slice(char::from_u32(code).unwrap_or('\u{fffd}').encode_utf8(&mut buffer).as_bytes());
                        }
                        other => out.push(other),
                    }
                }
                other => out.push(other),
            }
        }
    }
    fn value(&mut self) -> Result<Json, String> {
        self.ws();
        let Some(&c) = self.s.get(self.i) else { return Err("unexpected end".into()) };
        match c {
            b'{' => {
                self.i += 1;
                let mut out = vec![];
                self.ws();
                if self.s.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(Json::Object(out));
                }
                loop {
                    self.ws();
                    let key = self.string()?;
                    self.ws();
                    self.expect(b':')?;
                    out.push((key, self.value()?));
                    self.ws();
                    if self.s.get(self.i) == Some(&b',') {
                        self.i += 1;
                        continue;
                    }
                    self.expect(b'}')?;
                    return Ok(Json::Object(out));
                }
            }
            b'[' => {
                self.i += 1;
                let mut out = vec![];
                self.ws();
                if self.s.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(Json::List(out));
                }
                loop {
                    out.push(self.value()?);
                    self.ws();
                    if self.s.get(self.i) == Some(&b',') {
                        self.i += 1;
                        continue;
                    }
                    self.expect(b']')?;
                    return Ok(Json::List(out));
                }
            }
            b'"' => Ok(Json::Str(self.string()?)),
            b't' if self.s[self.i..].starts_with(b"true") => { self.i += 4; Ok(Json::Bool(true)) }
            b'f' if self.s[self.i..].starts_with(b"false") => { self.i += 5; Ok(Json::Bool(false)) }
            b'n' if self.s[self.i..].starts_with(b"null") => { self.i += 4; Ok(Json::Null) }
            _ => {
                let start = self.i;
                while self.i < self.s.len() && b"+-0123456789.eE".contains(&self.s[self.i]) {
                    self.i += 1;
                }
                if start == self.i {
                    return Err(format!("unexpected character {}", c as char));
                }
                Ok(Json::Number(String::from_utf8_lossy(&self.s[start..self.i]).into_owned()))
            }
        }
    }
}
fn parse_json(s: &[u8]) -> Result<Json, String> {
    let mut p = JsonParser { s, i: 0 };
    let v = p.value()?;
    p.ws();
    if p.i != s.len() {
        return Err("trailing data".into());
    }
    Ok(v)
}
fn json_string(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
}
fn write_json(out: &mut String, v: &Json, pretty: bool, depth: usize) {
    let indent = |out: &mut String, depth: usize| {
        if pretty {
            out.push('\n');
            out.push_str(&"  ".repeat(depth));
        }
    };
    match v {
        Json::Null => out.push_str("null"),
        Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Json::Number(n) => out.push_str(n),
        Json::Str(s) => json_string(out, s),
        Json::List(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                indent(out, depth + 1);
                write_json(out, item, pretty, depth + 1);
            }
            indent(out, depth);
            out.push(']');
        }
        Json::Object(fields) => {
            if fields.is_empty() {
                out.push_str("{}");
                return;
            }
            out.push('{');
            for (i, (k, item)) in fields.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                indent(out, depth + 1);
                json_string(out, k);
                out.push_str(if pretty { ": " } else { ":" });
                write_json(out, item, pretty, depth + 1);
            }
            indent(out, depth);
            out.push('}');
        }
    }
}

// Builds the SDK from --host, --with-client-creds and the DPoP and TLS flags; the SDK discovers everything else.
fn new_sdk(p: &Parsed) -> Result<(Sdk, String), (u8, String)> {
    let host = p.value("host").trim_end_matches('/').to_owned();
    if host.is_empty() {
        return Err((EXIT_USAGE, format!("--host is required\n{USAGE}")));
    }
    let creds = p.value("with-client-creds");
    if creds.is_empty() {
        return Err((EXIT_USAGE, format!("--with-client-creds is required\n{USAGE}")));
    }
    let (id, secret) = match parse_json(creds.as_bytes()) {
        Ok(Json::Object(fields)) => {
            let get = |name: &str| fields.iter().find(|(k, _)| k == name).and_then(|(_, v)| if let Json::Str(s) = v { Some(s.clone()) } else { None }).unwrap_or_default();
            (get("clientId"), get("clientSecret"))
        }
        _ => (String::new(), String::new()),
    };
    if id.is_empty() || secret.is_empty() {
        return Err((EXIT_USAGE, "--with-client-creds must be JSON with clientId and clientSecret".into()));
    }
    let mut options = Options {
        client_id: id,
        client_secret: secret,
        insecure_plaintext: host.to_lowercase().starts_with("http://"),
        insecure_skip_verify: p.flag("tls-no-verify"),
        dpop_algorithm: p.value("dpop").to_owned(),
        timeout_millis: 120_000,
        ..Default::default()
    };
    if !p.value("dpop-key").is_empty() {
        if p.value("dpop").is_empty() {
            return Err((EXIT_USAGE, "--dpop-key needs --dpop".into()));
        }
        options.dpop_key_pem = fs::read_to_string(p.value("dpop-key")).map_err(|e| (EXIT_ERROR, format!("Failed to read DPoP key {}: {e}", p.value("dpop-key"))))?;
    }
    Sdk::new(&host, options).map(|sdk| (sdk, host.clone())).map_err(|e| (EXIT_ERROR, describe_new(&e, &host)))
}

fn write_out(path: &str, data: &[u8], mode: u32) -> std::io::Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let tmp = format!("{path}.tmp");
    fs::write(&tmp, data)?;
    fs::set_permissions(&tmp, fs::Permissions::from_mode(mode))?;
    if let Err(e) = fs::rename(&tmp, path) {
        let _ = fs::remove_file(&tmp);
        return Err(e);
    }
    Ok(())
}

fn version(args: &[String], stdout: &mut dyn Write, stderr: &mut dyn Write) -> u8 {
    let mut as_json = false;
    for a in args {
        if a == "--json" {
            as_json = true;
        } else {
            let _ = writeln!(stderr, "unknown flag: {a}");
            return EXIT_USAGE;
        }
    }
    let info = (|| -> Result<_, Error> { Ok((tdf::version()?, tdf::tdf_spec_version()?, tdf::supported_features()?)) })();
    let (version, schema, features) = match info {
        Ok(v) => v,
        Err(e) => {
            let _ = writeln!(stderr, "Failed to read version: {}", describe(&e));
            return EXIT_ERROR;
        }
    };
    if as_json {
        let json = Json::Object(vec![
            ("sdk_version".into(), Json::Str(version)),
            ("schema_version".into(), Json::Str(schema)),
            ("supported_features".into(), Json::List(features.into_iter().map(Json::Str).collect())),
            ("target".into(), Json::Str(TARGET.into())),
        ]);
        let mut out = String::new();
        write_json(&mut out, &json, false, 0);
        let _ = writeln!(stdout, "{out}");
        return EXIT_SUCCESS;
    }
    let _ = writeln!(stdout, "otdfgc ({TARGET}) sdk {version} tdf-spec {schema} features {}", features.join(","));
    EXIT_SUCCESS
}

fn supports(args: &[String], stdout: &mut dyn Write, stderr: &mut dyn Write) -> u8 {
    if args.len() != 1 || args[0].starts_with('-') {
        let _ = writeln!(stderr, "usage: otdfgc supports <feature>");
        return EXIT_USAGE;
    }
    match tdf::supported_features() {
        Ok(features) if features.iter().any(|f| *f == args[0]) => EXIT_SUCCESS,
        Ok(_) => {
            let _ = writeln!(stdout, "{} unsupported", args[0]);
            EXIT_ERROR
        }
        Err(e) => {
            let _ = writeln!(stderr, "Failed to read supported features: {}", describe(&e));
            EXIT_ERROR
        }
    }
}

// inspect reads the manifest through the handle API: an SDK with a provided
// platform configuration needs no platform, and the loaded reader's manifest
// is printed keyed by the reference's JSON tags.
fn inspect(args: &[String], stdout: &mut dyn Write, stderr: &mut dyn Write) -> u8 {
    if args.len() != 1 || args[0].starts_with('-') {
        let _ = writeln!(stderr, "usage: otdfgc inspect <in>");
        return EXIT_USAGE;
    }
    let data = match fs::read(&args[0]) {
        Ok(d) => d,
        Err(e) => {
            let _ = writeln!(stderr, "not a valid TDF: {e}");
            return EXIT_ERROR;
        }
    };
    let manifest = (|| -> Result<_, Error> {
        let sdk = Sdk::new("http://localhost", Options { platform_configuration: Some(BTreeMap::from([("platform_issuer".to_owned(), "http://localhost".to_owned())])), insecure_plaintext: true, ..Default::default() })?;
        let reader = sdk.load_tdf(&data, &ReaderOptions { ignore_allowlist: true, ..Default::default() })?;
        let manifest = reader.manifest();
        reader.close()?;
        sdk.close()?;
        manifest
    })();
    let m = match manifest {
        Ok(m) => m,
        Err(e) => {
            let _ = writeln!(stderr, "not a valid TDF: {}", e.message);
            return EXIT_ERROR;
        }
    };
    let s = |v: &str| Json::Str(v.to_owned());
    let n = |v: i64| Json::Number(v.to_string());
    let e = &m.encryption_information;
    let i = &e.integrity_information;
    let json = Json::Object(vec![
        ("encryptionInformation".into(), Json::Object(vec![
            ("type".into(), s(&e.type_)),
            ("policy".into(), s(&e.policy)),
            ("keyAccess".into(), Json::List(e.key_access.iter().map(|k| Json::Object(vec![
                ("type".into(), s(&k.type_)), ("url".into(), s(&k.url)), ("protocol".into(), s(&k.protocol)), ("wrappedKey".into(), s(&k.wrapped_key)),
                ("policyBinding".into(), Json::Object(vec![("alg".into(), s(&k.policy_binding.alg)), ("hash".into(), s(&k.policy_binding.hash))])),
                ("encryptedMetadata".into(), s(&k.encrypted_metadata)), ("kid".into(), s(&k.kid)), ("sid".into(), s(&k.sid)), ("schemaVersion".into(), s(&k.schema_version)), ("ephemeralPublicKey".into(), s(&k.ephemeral_public_key)),
            ])).collect())),
            ("method".into(), Json::Object(vec![("algorithm".into(), s(&e.method.algorithm)), ("iv".into(), s(&e.method.iv)), ("isStreamable".into(), Json::Bool(e.method.is_streamable))])),
            ("integrityInformation".into(), Json::Object(vec![
                ("rootSignature".into(), Json::Object(vec![("alg".into(), s(&i.root_signature.alg)), ("sig".into(), s(&i.root_signature.sig))])),
                ("segmentHashAlg".into(), s(&i.segment_hash_alg)), ("segmentSizeDefault".into(), n(i.segment_size_default)), ("encryptedSegmentSizeDefault".into(), n(i.encrypted_segment_size_default)),
                ("segments".into(), Json::List(i.segments.iter().map(|g| Json::Object(vec![("hash".into(), s(&g.hash)), ("segmentSize".into(), n(g.segment_size)), ("encryptedSegmentSize".into(), n(g.encrypted_segment_size))])).collect())),
            ])),
        ])),
        ("payload".into(), Json::Object(vec![("type".into(), s(&m.payload.type_)), ("url".into(), s(&m.payload.url)), ("protocol".into(), s(&m.payload.protocol)), ("mimeType".into(), s(&m.payload.mime_type)), ("isEncrypted".into(), Json::Bool(m.payload.is_encrypted))])),
        ("assertions".into(), Json::List(m.assertions.iter().map(|a| Json::Object(vec![
            ("id".into(), s(&a.id)), ("type".into(), s(&a.type_)), ("scope".into(), s(&a.scope)), ("appliesToState".into(), s(&a.applies_to_state)),
            ("statement".into(), Json::Object(vec![("format".into(), s(&a.statement.format)), ("schema".into(), s(&a.statement.schema)), ("value".into(), s(&a.statement.value))])),
            ("binding".into(), Json::Object(vec![("method".into(), s(&a.binding.method)), ("signature".into(), s(&a.binding.signature))])),
        ])).collect())),
        ("schemaVersion".into(), s(&m.schema_version)),
    ]);
    let mut out = String::new();
    write_json(&mut out, &json, true, 0);
    let _ = writeln!(stdout, "{out}");
    EXIT_SUCCESS
}

fn encrypt(args: &[String], stderr: &mut dyn Write) -> u8 {
    let p = match parse(args, ENCRYPT_FLAGS) {
        Ok(p) => p,
        Err(e) => {
            let _ = write!(stderr, "{e}\n{USAGE}");
            return EXIT_USAGE;
        }
    };
    let reason = unsupported(&p, ENCRYPT_UNSUPPORTED);
    if !reason.is_empty() {
        let _ = writeln!(stderr, "unsupported in this build: {reason}");
        return EXIT_USAGE;
    }
    if p.positional.len() != 1 || p.value("out").is_empty() {
        let _ = write!(stderr, "encrypt needs exactly one input file and --out\n{USAGE}");
        return EXIT_USAGE;
    }
    let input = match fs::read(&p.positional[0]) {
        Ok(d) => d,
        Err(e) => {
            let _ = writeln!(stderr, "Failed to read {}: {e}", p.positional[0]);
            return EXIT_ERROR;
        }
    };
    let (sdk, host) = match new_sdk(&p) {
        Ok(v) => v,
        Err((code, message)) => {
            let _ = writeln!(stderr, "{}", message.trim_end_matches('\n'));
            return code;
        }
    };
    // As otdfctl: the platform's KAS is the default key access server and attributes drive autoconfigure.
    let options = TdfOptions {
        kas_information: vec![KasInfo::new(&format!("{host}/kas"))],
        mime_type: p.value("mime-type").to_owned(),
        attributes: split_list(p.value("attr")),
        wrapping_key_algorithm: p.value("wrapping-key-algorithm").to_owned(),
        target_mode: p.value("target-mode").to_owned(),
        ..Default::default()
    };
    let result = sdk.encrypt_bytes(&input, &options);
    let _ = sdk.close();
    let data = match result {
        Ok(d) => d,
        Err(e) => {
            let _ = writeln!(stderr, "Failed to encrypt: {}", describe(&e));
            return EXIT_ERROR;
        }
    };
    if let Err(e) = write_out(p.value("out"), &data, 0o644) {
        let _ = writeln!(stderr, "Failed to write encrypted file {}: {e}", p.value("out"));
        return EXIT_ERROR;
    }
    EXIT_SUCCESS
}

fn decrypt(args: &[String], stderr: &mut dyn Write) -> u8 {
    let p = match parse(args, DECRYPT_FLAGS) {
        Ok(p) => p,
        Err(e) => {
            let _ = write!(stderr, "{e}\n{USAGE}");
            return EXIT_USAGE;
        }
    };
    let reason = unsupported(&p, DECRYPT_UNSUPPORTED);
    if !reason.is_empty() {
        let _ = writeln!(stderr, "unsupported in this build: {reason}");
        return EXIT_USAGE;
    }
    if p.positional.len() != 1 || p.value("out").is_empty() {
        let _ = write!(stderr, "decrypt needs exactly one input file and --out\n{USAGE}");
        return EXIT_USAGE;
    }
    let input = match fs::read(&p.positional[0]) {
        Ok(d) => d,
        Err(e) => {
            let _ = writeln!(stderr, "Failed to read file: {e}");
            return EXIT_ERROR;
        }
    };
    let (sdk, _) = match new_sdk(&p) {
        Ok(v) => v,
        Err((code, message)) => {
            let _ = writeln!(stderr, "{}", message.trim_end_matches('\n'));
            return code;
        }
    };
    // otdfctl: "*" alone ignores the allowlist; any other list is explicit.
    let allowlist = split_list(p.value("kas-allowlist"));
    let ignore = allowlist.len() == 1 && allowlist[0] == "*";
    let options = ReaderOptions {
        disable_assertion_verification: p.flag("no-verify-assertions"),
        session_key_type: p.value("session-key-algorithm").to_owned(),
        ignore_allowlist: ignore,
        kas_allowlist: if !ignore && !allowlist.is_empty() { Some(allowlist) } else { None },
        ..Default::default()
    };
    let result = sdk.decrypt_bytes(&input, &options);
    let _ = sdk.close();
    let plaintext = match result {
        Ok(d) => d,
        Err(e) => {
            let _ = writeln!(stderr, "Failed to decrypt file: {}", describe(&e));
            return EXIT_ERROR;
        }
    };
    if let Err(e) = write_out(p.value("out"), &plaintext, 0o600) {
        let _ = writeln!(stderr, "Failed to write decrypted data to file: {e}");
        return EXIT_ERROR;
    }
    EXIT_SUCCESS
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let stdout = &mut std::io::stdout();
    let stderr = &mut std::io::stderr();
    if args.is_empty() {
        let _ = write!(stderr, "{USAGE}");
        return ExitCode::from(EXIT_USAGE);
    }
    let rest = &args[1..];
    let code = match args[0].as_str() {
        "--version" | "version" => version(rest, stdout, stderr),
        "supports" => supports(rest, stdout, stderr),
        "inspect" => inspect(rest, stdout, stderr),
        "encrypt" => encrypt(rest, stderr),
        "decrypt" => decrypt(rest, stderr),
        "help" | "--help" | "-h" => {
            let _ = write!(stdout, "{USAGE}");
            EXIT_SUCCESS
        }
        other => {
            let _ = write!(stderr, "unknown command: {other}\n{USAGE}");
            EXIT_USAGE
        }
    };
    ExitCode::from(code)
}
