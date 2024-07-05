use ed25519_dalek::{pkcs8::EncodePrivateKey, SigningKey};
use jose_jwk::{Jwk, Okp, OkpCurves, Parameters};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use rand::rngs::OsRng;

pub struct Key {
    pub(super) jwk: Jwk,
    pub(super) encoding: EncodingKey,
    pub(super) decoding: DecodingKey,
}

impl Key {
    pub(super) fn generate() -> Key {
        let private_key = SigningKey::generate(&mut OsRng);
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
