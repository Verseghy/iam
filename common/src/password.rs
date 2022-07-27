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

pub type HashError = argon2::Error;

pub fn encrypt(password: &str) -> Result<String, HashError> {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill(&mut salt);

    argon2::hash_encoded(password.as_bytes(), &salt, &ARGON2_CONFIG)
}

pub fn validate(
    hash: &str,
    password: &str,
) -> Result<(bool, Option<argon2::Result<String>>), ValidateError> {
    let hash_type = if hash.starts_with("$2y$") {
        HashType::Bcrypt
    } else if hash.starts_with("$argon2id")
        || hash.starts_with("$argon2d")
        || hash.starts_with("$argon2i")
    {
        HashType::Argon2
    } else {
        return Err(ValidateError::UnknownHashType);
    };

    match hash_type {
        HashType::Bcrypt => {
            let rehashed_password = encrypt(password);
            Ok((bcrypt::verify(password, hash)?, Some(rehashed_password)))
        }
        HashType::Argon2 => Ok((argon2::verify_encoded(hash, password.as_bytes())?, None)),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ValidateError {
    #[error("bcrypt error")]
    BCryptError(#[from] bcrypt::BcryptError),
    #[error("argon2 error")]
    Argon2Error(#[from] argon2::Error),
    #[error("unknown hash type")]
    UnknownHashType,
}
