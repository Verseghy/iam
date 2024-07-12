use super::Key;
use crate::error::{self, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String,
    pub sub: String,
    pub exp: i64,
    pub nbf: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new<S: Into<String>>(subject: S) -> Self {
        let issuer = env::var("HOSTNAME").unwrap_or_else(|_| "dev".to_owned());

        let now = Utc::now();
        let timestamp = now.timestamp();

        let exp = (now + Duration::weeks(1)).timestamp();

        Self {
            iss: issuer,
            sub: subject.into(),
            exp,
            nbf: timestamp,
            iat: timestamp,
        }
    }
}

pub struct Jwt<'a>(&'a Key);

impl<'a> Jwt<'a> {
    #[inline]
    pub(super) const fn new(key: &'a Key) -> Self {
        Self(key)
    }

    pub fn encode(&self, claims: &Claims) -> String {
        let header = Header::new(self.0.get_alg());
        jsonwebtoken::encode(&header, claims, &self.0.encoding).unwrap()
    }

    pub fn get_claims(&self, token: &str) -> Result<Claims> {
        let validation = Validation::new(self.0.get_alg());

        jsonwebtoken::decode(token, &self.0.decoding, &validation)
            .map(|data| data.claims)
            .inspect_err(|err| tracing::warn!(token, error = err.to_string(), "invalid token"))
            .map_err(|_| error::JWT_INVALID_TOKEN)
    }
}
