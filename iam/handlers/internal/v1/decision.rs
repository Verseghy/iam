use crate::{auth, json::Json, state::StateTrait};
use axum::{extract::State, http::StatusCode, Extension};
use iam_common::{error::Result, keys::jwt::Claims};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct DecisionRequest {
    action_list: Vec<Action>,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    name: String,
}

pub async fn decision<S: StateTrait>(
    State(state): State<S>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(req): Json<DecisionRequest>,
) -> Result<StatusCode> {
    let actions: Vec<&str> = req.action_list.iter().map(|x| x.name.as_str()).collect();

    auth::check(&claims.sub, &actions, state.db()).await?;

    Ok(StatusCode::NO_CONTENT)
}
