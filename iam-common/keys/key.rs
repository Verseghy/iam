use jose_jwk::{Ec, EcCurves, Jwk, Parameters};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use ring::{
    rand::SystemRandom,
    signature::{ECDSA_P256_SHA256_ASN1_SIGNING, EcdsaKeyPair, KeyPair},
};
use std::path::Path;
use tokio::fs;

pub struct Key {
    pub(super) jwk: Jwk,
    pub(super) encoding: EncodingKey,
    pub(super) decoding: DecodingKey,
}

impl Key {
    pub async fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let file_content = fs::read(path).await?;
        Self::from_pkcs8_der(&file_content)
    }

    fn from_pkcs8_der(der: &[u8]) -> anyhow::Result<Self> {
        let key_pair =
            EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_ASN1_SIGNING, der, &SystemRandom::new())?;

        let public_key = key_pair.public_key().as_ref();

        let jwk = Jwk {
            key: jose_jwk::Key::Ec(Ec {
                crv: EcCurves::P256,
                x: public_key[1..33].to_vec().into(),
                y: public_key[33..65].to_vec().into(),
                d: None,
            }),
            prm: Parameters {
                // TODO: figure out keyid
                kid: Some("jwt".to_owned()),
                ..Default::default()
            },
        };

        let encoding = EncodingKey::from_ec_der(der);
        let decoding = DecodingKey::from_ec_der(public_key);

        Ok(Key {
            jwk,
            encoding,
            decoding,
        })
    }

    pub fn get_alg(&self) -> Algorithm {
        Algorithm::ES256
    }
}
