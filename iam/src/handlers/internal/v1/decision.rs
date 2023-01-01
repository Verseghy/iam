use crate::{
    auth::{self, CheckError},
    json::Json,
    shared::SharedTrait,
    utils::{Error, Result},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension,
};
use common::token::Claims;
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

pub async fn decision<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Extension(claims): Extension<Arc<Claims>>,
    Json(req): Json<DecisionRequest>,
) -> Result<Response> {
    let actions: Vec<&str> = req.action_list.iter().map(|x| x.name.as_str()).collect();

    match auth::check(&claims.subject, &actions, shared.db()).await {
        Err(CheckError::NoPermission(failed)) => {
            Ok(Json(DecisionResponse { failed }).into_response())
        }
        Err(err) => Err(Error::internal(err)),
        Ok(_) => Ok(StatusCode::NO_CONTENT.into_response()),
    }
}
