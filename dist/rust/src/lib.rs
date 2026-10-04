//! Owned Rust TDF3 façade over the generated shared implementation.
mod generated;
pub use generated::{
    CallOptions, Cancellation, ErrorKind, HostWire, LibraryError, Operation, ProviderError,
    ProviderRequest,
};
use std::sync::Arc;
#[derive(Clone, Debug, Default)]
pub struct KASRoute {
    pub url: String,
    pub api_base_url: String,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    pub platform_url: String,
    pub kas_url: String,
    pub allowed_kas: Vec<KASRoute>,
    pub issuer_url: String,
    pub token_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub token_provider_name: String,
    pub allow_http: bool,
    pub timeout_millis: i64,
    pub kas_public_key_pem: String,
    pub kid: String,
    pub kas_algorithm: String,
    pub session_algorithm: String,
    pub auth_private_key_pem: String,
    pub auth_algorithm: String,
    pub dpop: bool,
}
impl From<Config> for generated::Config {
    fn from(c: Config) -> Self {
        Self {
            PlatformURL: c.platform_url.into_bytes(),
            KASURL: c.kas_url.into_bytes(),
            AllowedKAS: c
                .allowed_kas
                .into_iter()
                .map(|r| generated::KASRoute {
                    URL: r.url.into_bytes(),
                    APIBaseURL: r.api_base_url.into_bytes(),
                })
                .collect(),
            IssuerURL: c.issuer_url.into_bytes(),
            TokenURL: c.token_url.into_bytes(),
            ClientID: c.client_id.into_bytes(),
            ClientSecret: c.client_secret.into_bytes(),
            TokenProviderName: c.token_provider_name.into_bytes(),
            AllowHTTP: c.allow_http,
            TimeoutMillis: c.timeout_millis,
            KASPublicKeyPEM: c.kas_public_key_pem.into_bytes(),
            KID: c.kid.into_bytes(),
            KASAlgorithm: c.kas_algorithm.into_bytes(),
            SessionAlgorithm: c.session_algorithm.into_bytes(),
            AuthPrivateKeyPEM: c.auth_private_key_pem.into_bytes(),
            AuthAlgorithm: c.auth_algorithm.into_bytes(),
            DPoP: c.dpop,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct EncryptOptions {
    pub policy_base64: String,
    pub attributes: Vec<String>,
    pub dissem: Vec<String>,
    pub segment_size: i64,
    pub has_segment_size: bool,
    pub segment_hash_algorithm: String,
    pub mime_type: String,
    pub metadata: Vec<u8>,
    pub include_metadata: bool,
}
impl From<EncryptOptions> for generated::EncryptOptions {
    fn from(o: EncryptOptions) -> Self {
        Self {
            PolicyBase64: o.policy_base64.into_bytes(),
            Attributes: o.attributes.into_iter().map(String::into_bytes).collect(),
            Dissem: o.dissem.into_iter().map(String::into_bytes).collect(),
            SegmentSize: o.segment_size,
            HasSegmentSize: o.has_segment_size,
            SegmentHashAlgorithm: o.segment_hash_algorithm.into_bytes(),
            MimeType: o.mime_type.into_bytes(),
            Metadata: o.metadata,
            IncludeMetadata: o.include_metadata,
        }
    }
}
pub type Decrypted = generated::Decrypted;
/// Inputs move into the operation before submission. Clone before calling if needed.
pub fn encrypt(
    config: Config,
    payload: Vec<u8>,
    options: EncryptOptions,
    call: CallOptions,
) -> Operation<Vec<u8>> {
    generated::Encrypt(config.into(), payload, options.into(), call)
}
pub fn decrypt(config: Config, archive: Vec<u8>, call: CallOptions) -> Operation<Decrypted> {
    generated::Decrypt(config.into(), archive, call)
}
#[derive(Clone, Debug)]
pub struct AccessToken {
    pub value: String,
    pub scheme: String,
    pub expires_at: i64,
    pub confirmation_jkt: String,
}
/// Typed provider callbacks run on native workers outside runtime locks.
pub fn token_provider(
    f: impl Fn(ProviderRequest) -> Result<AccessToken, ProviderError> + Send + Sync + 'static,
) -> generated::Provider {
    Arc::new(move |r| {
        let token = f(r)?;
        if token.expires_at < 0 {
            return Err(ProviderError::Rejected("negative expiry".into()));
        };
        Ok(format!(
            "{{\"value\":{},\"scheme\":{},\"expiresAt\":\"{}\",\"confirmationJKT\":{}}}",
            json_string(&token.value),
            json_string(&token.scheme),
            token.expires_at,
            json_string(&token.confirmation_jkt)
        )
        .into_bytes())
    })
}
fn json_string(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < ' ' => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
