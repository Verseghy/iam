use crate::error::{self, Result};
use base64::{engine::general_purpose::STANDARD, Engine};

pub fn parse_token(token: &str) -> Result<(String, String)> {
    let decoded = STANDARD
        .decode(token)
        .map_err(|_| error::APP_INVALID_TOKEN)?;
    let decoded_string = String::from_utf8(decoded).map_err(|_| error::APP_INVALID_TOKEN)?;

    let (id, password) = decoded_string
        .split_once(':')
        .ok_or(error::APP_INVALID_TOKEN)?;

    Ok((id.to_owned(), password.to_owned()))
}
