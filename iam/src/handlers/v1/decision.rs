use crate::{auth, json::Json, shared::SharedTrait};
use axum::{http::StatusCode, Extension};
use iam_common::{error::Result, Id};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Request {
    actions: Vec<String>,
    user: Id,
}

pub async fn decision<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<Request>,
) -> Result<StatusCode> {
    let actions: Vec<&str> = req.actions.iter().map(|x| x.as_str()).collect();

    auth::check(&req.user.to_string(), &actions, shared.db()).await?;

    Ok(StatusCode::NO_CONTENT)
}
