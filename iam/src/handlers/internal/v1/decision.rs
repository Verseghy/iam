use crate::{auth, json::Json, shared::SharedTrait};
use axum::{http::StatusCode, Extension};
use common::{error::Result, token::Claims};
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

pub async fn decision<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(req): Json<DecisionRequest>,
) -> Result<StatusCode> {
    let actions: Vec<&str> = req.action_list.iter().map(|x| x.name.as_str()).collect();

    auth::check(&claims.subject, &actions, shared.db()).await?;

    Ok(StatusCode::NO_CONTENT)
}
