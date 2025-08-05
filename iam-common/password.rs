use crate::error::{self, Result};
use argon2::Config;
use rand::Rng;

pub enum HashType {
    Bcrypt,
    Argon2,
}

pub type HashError = argon2::Error;

pub fn hash(password: &str) -> Result<String> {
    let mut salt = [0u8; 16];
    rand::rng().fill(&mut salt);

    argon2::hash_encoded(password.as_bytes(), &salt, &Config::owasp5())
        .map_err(|_| error::FAILED_PASSWORD_HASH)
}

pub fn validate(hashed: &str, password: &str) -> Result<(bool, Option<Result<String>>)> {
    let hash_type = if hashed.starts_with("$2y$") {
        HashType::Bcrypt
    } else if hashed.starts_with("$argon2id")
        || hashed.starts_with("$argon2d")
        || hashed.starts_with("$argon2i")
    {
        HashType::Argon2
    } else {
        return Ok((false, None));
    };

    match hash_type {
        HashType::Bcrypt => {
            let rehashed_password = hash(password);
            Ok((
                bcrypt::verify(password, hashed).map_err(|_| error::FAILED_PASSWORD_HASH)?,
                Some(rehashed_password),
            ))
        }
        HashType::Argon2 => Ok((
            argon2::verify_encoded(hashed, password.as_bytes())
                .map_err(|_| error::FAILED_PASSWORD_HASH)?,
            None,
        )),
    }
}
