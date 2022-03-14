use argon2::{Config, ThreadMode, Variant, Version};
use rand::Rng;

static ARGON2_CONFIG: Config = Config {
    ad: &[],
    hash_length: 32,
    lanes: 1,
    mem_cost: 37 * 1024,
    secret: &[],
    thread_mode: ThreadMode::Sequential,
    time_cost: 1,
    variant: Variant::Argon2id,
    version: Version::Version13,
};

pub enum HashType {
    Bcrypt,
    Argon2,
}

pub fn encrypt(password: &str) -> argon2::Result<String> {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill(&mut salt);

    argon2::hash_encoded(password.as_bytes(), &salt, &ARGON2_CONFIG)
}

pub fn validate(
    hash_type: HashType,
    hash: &String,
    password: &String,
) -> Result<bool, ValidateError> {
    match hash_type {
        HashType::Bcrypt => Ok(bcrypt::verify(password.as_bytes(), &hash)?),
        HashType::Argon2 => Ok(argon2::verify_encoded(&hash, password.as_bytes())?),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ValidateError {
    #[error("bcrypt error")]
    BCryptError(#[from] bcrypt::BcryptError),
    #[error("argon2 error")]
    Argon2Error(#[from] argon2::Error),
}
