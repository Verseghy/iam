use crate::{auth, json::Json, state::StateTrait};
use axum::{extract::State, http::StatusCode};
use iam_common::{error::Result, Id};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Request {
    actions: Vec<String>,
    user: Id,
}

pub async fn decision<S: StateTrait>(
    State(state): State<S>,
    Json(req): Json<Request>,
) -> Result<StatusCode> {
    let actions: Vec<&str> = req.actions.iter().map(|x| x.as_str()).collect();

    auth::check(&req.user.to_string(), &actions, state.db()).await?;

    Ok(StatusCode::NO_CONTENT)
}
