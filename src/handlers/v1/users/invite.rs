use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
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

pub async fn invite<REDIS, SMTP, RNG>(
    req: web::Json<InviteRequest>,
    redis: web::Data<REDIS>,
    smtp_transport: web::Data<SMTP>,
    rng: web::Data<RNG>,
) -> Result<HttpResponse, InviteError>
where
    REDIS: AsyncCommands + Clone,
    SMTP: AsyncTransport<Error = SmtpError> + Sync + Clone,
    RNG: Rng + Clone,
{
    let req = req.into_inner();
    let mut redis = redis.get_ref().clone();
    let smtp_transport = smtp_transport.get_ref().clone();

    req.validate().map_err(|_| InviteError::BadInputData)?;

    let key = format!("/invites/{}", &req.email);

    // If already invited fail
    if redis.exists(&key).await? {
        return Err(InviteError::AlreadyExists);
    }

    let token: String = (*rng.get_ref())
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

    smtp_transport.send(email).await?;

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::mock::{assert_cmds, redis_cmd};
    use actix_web::web;
    use rand::rngs::mock::StepRng;
    use redis::Value;

    #[actix_web::test]
    async fn some_test() {
        let redis = web::Data::new(crate::mock::MockRedis::new(vec![Value::Nil, Value::Okay]));
        let smtp = web::Data::new(crate::mock::MockSmtpTransport::new(true));
        let rng = web::Data::new(StepRng::new(0, 1));

        let req = web::Json(InviteRequest {
            email: "asd@asd.asd".to_owned(),
        });

        let _res = invite(req, redis.clone(), smtp.clone(), rng).await;

        assert_cmds(
            &redis.cmds(),
            &[
                redis_cmd(&["EXISTS", "/invites/asd@asd.asd"]),
                redis_cmd(&[
                    "SETEX",
                    "/invites/asd@asd.asd",
                    "86400",
                    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
                ]),
            ],
        );
    }
}
