use crate::{json::Json, shared::Shared, utils::Error};
use axum::{http::StatusCode, Extension};
use entity::users;
use sea_orm::entity::EntityTrait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteUserRequest {
    id: String,
}

pub async fn delete_user(
    Extension(shared): Extension<Shared>,
    Json(req): Json<DeleteUserRequest>,
) -> Result<StatusCode, Error> {
    let res = users::Entity::delete_by_id(req.id).exec(&shared.db).await?;

    if res.rows_affected == 0 {
        Err(Error::not_found("user not found"))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
