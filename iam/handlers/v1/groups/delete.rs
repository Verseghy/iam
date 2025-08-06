use crate::{json::Json, state::StateTrait};
use axum::{extract::State, http::StatusCode};
use iam_common::error::{self, Result};
use iam_entity::groups;
use sea_orm::entity::EntityTrait;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteGroupRequest {
    id: String,
}

pub async fn delete_group<S: StateTrait>(
    State(state): State<S>,
    Json(req): Json<DeleteGroupRequest>,
) -> Result<StatusCode> {
    let res = groups::Entity::delete_by_id(req.id)
        .exec(state.db())
        .await?;

    if res.rows_affected == 0 {
        return Err(error::GROUP_NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
