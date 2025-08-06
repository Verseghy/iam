use crate::{
    json::{Json, ValidatedJson},
    StateTrait,
};
use axum::{extract::State, http::StatusCode};
use iam_common::{error::Result, Id};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct Request {
    #[validate(length(max = 256))]
    name: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    id: Id,
    secret: String,
}

pub async fn create_app<S: StateTrait>(
    State(state): State<S>,
    ValidatedJson(req): ValidatedJson<Request>,
) -> Result<(StatusCode, Json<Response>)> {
    let (id, secret) = iam_common::app::create(state.db(), &req.name).await?;

    Ok((StatusCode::CREATED, Json(Response { id, secret })))
}
