use super::Key;
use crate::error::{self, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: i64,
    pub nbf: i64,
    pub iat: i64,
}

pub struct Jwt<'a> {
    key: &'a Key,
    issuer: &'a str,
}

impl<'a> Jwt<'a> {
    pub(super) fn new(key: &'a Key, issuer: &'a str) -> Self {
        Self { key, issuer }
    }

    pub fn encode(&self, subject: &str) -> String {
        let now = Utc::now();
        let timestamp = now.timestamp();

        let claims = Claims {
            iss: self.issuer.to_owned(),
            sub: subject.to_owned(),
            exp: (now + Duration::weeks(1)).timestamp(),
            nbf: timestamp,
            iat: timestamp,
        };

        let header = Header::new(self.key.get_alg());
        jsonwebtoken::encode(&header, &claims, &self.key.encoding).unwrap()
    }

    pub fn get_claims(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::new(self.key.get_alg());
        validation.set_issuer(&[self.issuer]);

        jsonwebtoken::decode(token, &self.key.decoding, &validation)
            .map(|data| data.claims)
            .inspect_err(|err| tracing::warn!(token, error = err.to_string(), "invalid token"))
            .map_err(|_| error::JWT_INVALID_TOKEN)
    }
}
