use serde::Deserialize;

use crate::utils::Either;

#[derive(Debug, Deserialize)]
pub struct ErrorMessage {
    pub code: String,
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error returned from iam: {0:?}")]
    Iam(ErrorMessage),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[inline]
pub fn unwrap_res<T>(either: Either<T, ErrorMessage>) -> Result<T> {
    match either {
        Either::Left(t) => Ok(t),
        Either::Right(err) => Err(Error::Iam(err)),
    }
}
