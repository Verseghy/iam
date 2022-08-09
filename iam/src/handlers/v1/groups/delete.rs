use crate::{json::Json, shared::Shared, utils::Error};
use axum::{http::StatusCode, Extension};
use entity::groups;
use sea_orm::entity::EntityTrait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteGroupRequest {
    id: String,
}

pub async fn delete_group(
    Extension(shared): Extension<Shared>,
    Json(req): Json<DeleteGroupRequest>,
) -> Result<StatusCode, Error> {
    let res = groups::Entity::delete_by_id(req.id)
        .exec(&shared.db)
        .await?;

    if res.rows_affected == 0 {
        Err(Error::not_found("group not found"))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
