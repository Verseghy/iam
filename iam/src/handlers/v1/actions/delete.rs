use crate::{json::Json, shared::Shared, utils::Error};
use axum::{http::StatusCode, Extension};
use entity::actions;
use sea_orm::entity::EntityTrait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteActionRequest {
    id: String,
}

pub async fn delete_action(
    Extension(shared): Extension<Shared>,
    Json(req): Json<DeleteActionRequest>,
) -> Result<StatusCode, Error> {
    actions::Entity::delete_by_id(req.id)
        .exec(&shared.db)
        .await?;

    Ok(StatusCode::OK)
}
