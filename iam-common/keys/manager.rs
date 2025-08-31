pub use jose_jwk::JwkSet;

use crate::{
    keys::{jwt::Jwt, Key},
    Config,
};
use std::iter;

pub struct KeyManager {
    jwt_issuer: String,
    jwt_key: Key,
}

impl KeyManager {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let jwt_key = Key::from_file(&config.iam_signing_key_file).await?;

        Ok(Self {
            jwt_issuer: config.issuer_host.clone(),
            jwt_key,
        })
    }

    pub fn jwt(&self) -> Jwt<'_> {
        Jwt::new(&self.jwt_key, &self.jwt_issuer)
    }

    pub fn jwks(&self) -> JwkSet {
        let keys = iter::once(self.jwt_key.jwk.clone());

        JwkSet {
            keys: keys.collect(),
        }
    }
}
