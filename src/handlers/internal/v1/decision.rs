use crate::{
    auth::{self, CheckError},
    shared::Shared,
    token::Claims,
};
use axum::{
    debug_handler,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
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
) -> Result<Response, DecisionError> {
    let actions: Vec<&str> = req.action_list.iter().map(|x| x.name.as_str()).collect();

    match auth::check(&claims.subject, &actions, &shared.db).await {
        Err(CheckError::NoPermission(failed)) => {
            Ok(Json(DecisionResponse { failed }).into_response())
        }
        Err(err) => Err(DecisionError::from(err)),
        Ok(_) => Ok(StatusCode::NO_CONTENT.into_response()),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecisionError {
    #[error("check error: {0}")]
    CheckError(#[from] CheckError),
}

impl IntoResponse for DecisionError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::CheckError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
