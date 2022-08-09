use crate::{json::ValidatedJson, shared::Shared, utils::Error};
use axum::{http::StatusCode, Extension};
use lettre::{
    message::{Mailbox, Message},
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
) -> Result<StatusCode, Error> {
    let key = format!("/invites/{}", &req.email);

    let mut redis = shared.redis.clone();

    // If already invited fail
    if redis.exists(&key).await? {
        return Err(Error::bad_request("user already exists"));
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
        .body(format!("token: {token}"))
        .unwrap();

    shared.smtp.send(email).await?;

    Ok(StatusCode::OK)
}
