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
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Jwt {
    pub fn new() -> Self {
        Self {
            encoding: EncodingKey::from_rsa_pem(
                std::env::var("JWT_RSA_PRIVATE")
                    .expect("JWT_RSA_PRIVATE not set")
                    .as_ref(),
            )
            .expect("JWT_RSA_PRIVATE invalid"),
            decoding: DecodingKey::from_rsa_pem(
                std::env::var("JWT_RSA_PUBLIC")
                    .expect("JWT_RSA_PUBLIC not set")
                    .as_ref(),
            )
            .expect("JWT_RSA_PUBLIC invalid"),
        }
    }
}

impl Default for Jwt {
    fn default() -> Self {
        Self::new()
    }
}

pub trait JwtTrait {
    fn get_claims(&self, token: &str) -> Result<Claims>;
    fn encode(&self, claims: &Claims) -> Result<String>;
}

impl JwtTrait for Jwt {
    fn get_claims(&self, token: &str) -> Result<Claims> {
        Ok(jsonwebtoken::decode(token, &self.decoding, &VALIDATION)
            .map_err(|_| error::JWT_INVALID_TOKEN)?
            .claims)
    }

    fn encode(&self, claims: &Claims) -> Result<String> {
        jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &self.encoding)
            .map_err(|_| error::JWT_INVALID_TOKEN)
    }
}
