pub use jose_jwk::JwkSet;

use crate::keys::{jwt::Jwt, Key};
use std::iter;

pub struct KeyManager {
    jwt_key: Key,
}

impl KeyManager {
    pub fn new() -> Self {
        Self {
            jwt_key: Key::from_env(),
        }
    }

    pub fn jwt(&self) -> Jwt<'_> {
        Jwt::new(&self.jwt_key)
    }

    pub fn jwks(&self) -> JwkSet {
        let keys = iter::once(self.jwt_key.jwk.clone());

        JwkSet {
            keys: keys.collect(),
        }
    }
}

impl Default for KeyManager {
    fn default() -> Self {
        Self::new()
    }
}
