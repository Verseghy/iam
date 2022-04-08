use actix_web::{http::StatusCode, route, web, HttpResponse, ResponseError};
use lettre::{
    message::{Mailbox, Message},
    transport::smtp::{AsyncSmtpTransport, Error as SmtpError},
    AsyncTransport, Tokio1Executor,
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

#[route("/invite", method = "POST")]
pub async fn invite(
    req: web::Json<InviteRequest>,
    redis: web::Data<redis::aio::ConnectionManager>,
    smtp_transport: web::Data<AsyncSmtpTransport<Tokio1Executor>>,
) -> Result<HttpResponse, InviteError> {
    let req = req.into_inner();
    let mut redis = redis.get_ref().clone();
    let smtp_transport = smtp_transport.get_ref().clone();

    req.validate().map_err(|_| InviteError::BadInputData)?;

    let key = format!("/invites/{}", &req.email);

    // If already invited fail
    if redis.exists(&key).await? {
        return Err(InviteError::AlreadyExists);
    }

    let token: String = rand::thread_rng()
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

    let result = smtp_transport.send(email).await?;

    tracing::debug!("{result:?}");

    Ok(HttpResponse::new(StatusCode::OK))
}

#[derive(Debug, thiserror::Error)]
pub enum InviteError {
    #[error("user already invited")]
    AlreadyExists,
    #[error("failed to validate input data")]
    BadInputData,
    #[error("redis error {0}")]
    RedisError(#[from] redis::RedisError),
    #[error("email error")]
    EmailError(#[from] lettre::error::Error),
    #[error("smtp error")]
    SmtpError(#[from] SmtpError),
}

impl ResponseError for InviteError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::AlreadyExists => StatusCode::BAD_REQUEST,
            Self::BadInputData => StatusCode::BAD_REQUEST,
            Self::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmailError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::SmtpError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
