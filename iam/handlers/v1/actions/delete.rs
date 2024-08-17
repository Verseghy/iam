use crate::{json::Json, shared::SharedTrait};
use axum::{http::StatusCode, Extension};
use iam_common::error::Result;
use iam_entity::actions;
use sea_orm::entity::EntityTrait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteActionRequest {
    id: String,
}

pub async fn delete_action<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<DeleteActionRequest>,
) -> Result<StatusCode> {
    actions::Entity::delete_by_id(req.id)
        .exec(shared.db())
        .await?;

    Ok(StatusCode::OK)
}
