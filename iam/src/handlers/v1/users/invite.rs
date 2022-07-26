use crate::{shared::Shared, validate::ValidatedJson};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use lettre::{
    message::{Mailbox, Message},
    transport::smtp::Error as SmtpError,
    AsyncTransport,
};
use rand::{distributions::Alphanumeric, Rng};
use redis::AsyncCommands;
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct InviteRequest {
    #[validate(email, length(max = 256))]
    email: String,
}

pub async fn invite(
    Extension(shared): Extension<Shared>,
    ValidatedJson(req): ValidatedJson<InviteRequest>,
) -> Result<StatusCode, InviteError> {
    let key = format!("/invites/{}", &req.email);

    let mut redis = shared.redis.clone();

    // If already invited fail
    if redis.exists(&key).await? {
        return Err(InviteError::AlreadyExists);
    }

    let token: String = shared
        .rng
        .clone()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    redis.set_ex(&key, &token, 60 * 60 * 24).await?;

    // send mail
    let email = Message::builder()
        // SAFETY: address can be unwrapped becase earlier we checked that it is a valid email
        .to(Mailbox::new(None, req.email.parse().unwrap()))
        .from(Mailbox::new(
            None,
            "verseghy@test.verseghy.net".parse().unwrap(),
        ))
        .subject("Meghívó")
        .body(format!("token: {token}"))?;

    shared.smtp.send(email).await?;

    Ok(StatusCode::OK)
}

#[derive(Debug, thiserror::Error)]
pub enum InviteError {
    #[error("user already invited")]
    AlreadyExists,
    #[error("redis error {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("email error")]
    EmailError(#[from] lettre::error::Error),
    #[error("smtp error")]
    SmtpError(#[from] SmtpError),
}

impl IntoResponse for InviteError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::AlreadyExists => StatusCode::BAD_REQUEST,
            Self::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmailError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SmtpError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
