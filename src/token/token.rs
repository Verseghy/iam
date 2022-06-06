use std::default::Default;
use std::ops::Add;

use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};

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

pub fn create_encoding_key() -> EncodingKey {
    EncodingKey::from_rsa_pem(
        std::env::var("JWT_RSA_PRIVATE")
            .expect("JWT_RSA_PRIVATE not set")
            .as_ref(),
    )
    .expect("JWT_RSA_PRIVATE invalid")
}

pub fn create_decoding_key() -> DecodingKey {
    DecodingKey::from_rsa_pem(
        std::env::var("JWT_RSA_PUBLIC")
            .expect("JWT_RSA_PUBLIC not set")
            .as_ref(),
    )
    .expect("JWT_RSA_PUBLIC invalid")
}
