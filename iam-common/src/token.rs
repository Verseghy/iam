use crate::error::{self, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{default::Default, ops::Add};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    #[serde(rename = "iss")]
    pub issuer: String,
    #[serde(rename = "sub")]
    pub subject: String,
    #[serde(rename = "aud")]
    pub audience: Vec<String>,
    #[serde(rename = "exp")]
    pub expires_at: i64,
    #[serde(rename = "nbf")]
    pub not_before: i64,
    #[serde(rename = "iat")]
    pub issued_at: i64,
}

impl Default for Claims {
    fn default() -> Self {
        Claims {
            issuer: std::env::var("HOSTNAME").unwrap_or_else(|_| "dev".to_string()),
            audience: vec!["https://verseghy-gimnazium.net".to_string()],
            expires_at: Utc::now().add(Duration::weeks(1)).timestamp(),
            not_before: Utc::now().timestamp(),
            issued_at: Utc::now().timestamp(),
            subject: String::new(),
        }
    }
}

static VALIDATION: Lazy<Validation> = Lazy::new(|| {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["https://verseghy-gimnazium.net"]);
    validation.leeway = 5;

    validation
});

pub struct Jwt {
    encoding: Option<EncodingKey>,
    decoding: DecodingKey,
}

impl Jwt {
    pub fn new(encoding: Option<&[u8]>, decoding: &[u8]) -> Self {
        let encoding = encoding.map(|k| EncodingKey::from_rsa_pem(k).expect("invalid public key"));
        let decoding = DecodingKey::from_rsa_pem(decoding).expect("invalid private key");

        Self { encoding, decoding }
    }

    pub fn from_env() -> Self {
        let encoding = std::env::var("JWT_RSA_PRIVATE").expect("JWT_RSA_PRIVATE not set");
        let decoding = std::env::var("JWT_RSA_PUBLIC").expect("JWT_RSA_PUBLIC not set");

        Self::new(Some(encoding.as_ref()), decoding.as_ref())
    }
}

impl Default for Jwt {
    fn default() -> Self {
        Self::from_env()
    }
}

pub trait JwtTrait {
    fn get_claims(&self, token: &str) -> Result<Claims>;
    fn encode(&self, claims: &Claims) -> Result<String>;
}

impl JwtTrait for Jwt {
    fn get_claims(&self, token: &str) -> Result<Claims> {
        match jsonwebtoken::decode(token, &self.decoding, &VALIDATION) {
            Ok(decode) => Ok(decode.claims),
            Err(error) => {
                tracing::warn!(token, error = error.to_string(), "invalid token");
                Err(error::JWT_INVALID_TOKEN)
            }
        }
    }

    fn encode(&self, claims: &Claims) -> Result<String> {
        let Some(encoding) = &self.encoding else {
            return Err(error::INTERNAL);
        };

        jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, encoding)
            .map_err(|_| error::JWT_INVALID_TOKEN)
    }
}
