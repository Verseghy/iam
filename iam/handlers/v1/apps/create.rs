use crate::{
    json::{Json, ValidatedJson},
    SharedTrait,
};
use axum::{http::StatusCode, Extension};
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

pub async fn create_app<S: SharedTrait>(
    Extension(shared): Extension<S>,
    ValidatedJson(req): ValidatedJson<Request>,
) -> Result<(StatusCode, Json<Response>)> {
    let (id, secret) = iam_common::app::create(shared.db(), &req.name).await?;

    Ok((StatusCode::CREATED, Json(Response { id, secret })))
}
