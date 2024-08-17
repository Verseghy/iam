use crate::{json::Json, shared::SharedTrait};
use axum::{http::StatusCode, Extension};
use iam_common::error::{self, Result};
use iam_entity::users;
use sea_orm::entity::EntityTrait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteUserRequest {
    id: String,
}

pub async fn delete_user<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<DeleteUserRequest>,
) -> Result<StatusCode> {
    let res = users::Entity::delete_by_id(req.id)
        .exec(shared.db())
        .await?;

    if res.rows_affected == 0 {
        return Err(error::USER_NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
