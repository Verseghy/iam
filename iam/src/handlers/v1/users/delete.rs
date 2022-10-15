use crate::{
    json::Json,
    shared::SharedTrait,
    utils::{Error, Result},
};
use axum::{http::StatusCode, Extension};
use entity::users;
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
        Err(Error::not_found("user not found"))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
