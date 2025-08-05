use base64::{prelude::BASE64_STANDARD, Engine};
use ed25519_dalek::{pkcs8::EncodePrivateKey, SecretKey, SigningKey, SECRET_KEY_LENGTH};
use jose_jwk::{Jwk, Okp, OkpCurves, Parameters};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use rand::RngCore;
use std::env;

pub struct Key {
    pub(super) jwk: Jwk,
    pub(super) encoding: EncodingKey,
    pub(super) decoding: DecodingKey,
}

impl Key {
    #[allow(unused)]
    pub(super) fn generate() -> Self {
        let mut secret = SecretKey::default();
        rand::rng().fill_bytes(&mut secret);
        Self::from_private_key(&secret)
    }

    pub(super) fn from_private_key(secret_key: &SecretKey) -> Self {
        let private_key = SigningKey::from_bytes(secret_key);
        let public_key = private_key.verifying_key();

        let bytes = Box::new(public_key.to_bytes()) as Box<[u8]>;

        let jwk = Jwk {
            key: jose_jwk::Key::Okp(Okp {
                crv: OkpCurves::Ed25519,
                x: bytes.into(),
                d: None,
            }),
            prm: Parameters {
                kid: Some("jwt".to_owned()),
                ..Default::default()
            },
        };

        let encoding = EncodingKey::from_ed_der(private_key.to_pkcs8_der().unwrap().as_bytes());
        let decoding = DecodingKey::from_ed_der(public_key.as_bytes());

        Key {
            jwk,
            encoding,
            decoding,
        }
    }

    // TODO: this is a temporary solution
    pub(super) fn from_env() -> Self {
        let key = env::var("IAM_JWT_SECRET_KEY").unwrap();
        let key = BASE64_STANDARD.decode(key).unwrap();
        assert_eq!(key.len(), SECRET_KEY_LENGTH);
        Self::from_private_key(&key.try_into().unwrap())
    }

    pub fn get_alg(&self) -> Algorithm {
        match self.jwk.key {
            jose_jwk::Key::Okp(Okp {
                crv: OkpCurves::Ed25519,
                ..
            }) => Algorithm::EdDSA,
            _ => {
                panic!("unsupported key type");
            }
        }
    }
}
