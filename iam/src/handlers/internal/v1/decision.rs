use crate::{
    auth::{self, CheckError},
    json::Json,
    shared::Shared,
    token::Claims,
    utils::Error,
};
use axum::{
    debug_handler,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct DecisionRequest {
    action_list: Vec<Action>,
}

#[derive(Deserialize, Debug)]
pub struct Action {
    name: String,
}

#[derive(Serialize, Debug)]
pub struct DecisionResponse {
    failed: String,
}

#[debug_handler]
pub async fn decision(
    Extension(shared): Extension<Shared>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(req): Json<DecisionRequest>,
) -> Result<Response, Error> {
    let actions: Vec<&str> = req.action_list.iter().map(|x| x.name.as_str()).collect();

    match auth::check(&claims.subject, &actions, &shared.db).await {
        Err(CheckError::NoPermission(failed)) => {
            Ok(Json(DecisionResponse { failed }).into_response())
        }
        Err(err) => Err(Error::internal(err)),
        Ok(_) => Ok(StatusCode::NO_CONTENT.into_response()),
    }
}
