use crate::{json::Json, state::StateTrait};
use axum::{extract::State, http::StatusCode};
use iam_common::error::Result;
use iam_entity::actions;
use sea_orm::entity::EntityTrait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteActionRequest {
    id: String,
}

pub async fn delete_action<S: StateTrait>(
    State(state): State<S>,
    Json(req): Json<DeleteActionRequest>,
) -> Result<StatusCode> {
    actions::Entity::delete_by_id(req.id)
        .exec(state.db())
        .await?;

    Ok(StatusCode::OK)
}
