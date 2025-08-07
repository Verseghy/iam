use crate::{json::Json, state::StateTrait, utils::set_option};
use axum::{extract::State, http::StatusCode};
use iam_common::error::Result;
use iam_entity::groups;
use sea_orm::{entity::EntityTrait, Set};
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct UpdateGroupRequest {
    id: String,
    name: Option<String>,
}

pub async fn update_group<S: StateTrait>(
    State(state): State<S>,
    Json(req): Json<UpdateGroupRequest>,
) -> Result<StatusCode> {
    let group = groups::ActiveModel {
        id: Set(req.id),
        name: set_option(req.name),
        ..Default::default()
    };

    groups::Entity::update(group).exec(state.db()).await?;

    Ok(StatusCode::NO_CONTENT)
}
