use crate::{
    json::Json,
    shared::SharedTrait,
    utils::{Error, Result},
};
use axum::{http::StatusCode, Extension};
use entity::groups;
use sea_orm::entity::EntityTrait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteGroupRequest {
    id: String,
}

pub async fn delete_group<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<DeleteGroupRequest>,
) -> Result<StatusCode> {
    let res = groups::Entity::delete_by_id(req.id)
        .exec(shared.db())
        .await?;

    if res.rows_affected == 0 {
        Err(Error::not_found("group not found"))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
