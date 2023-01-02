use crate::{json::Json, shared::SharedTrait, utils::set_option};
use axum::{http::StatusCode, Extension};
use common::error::Result;
use entity::groups;
use sea_orm::{entity::EntityTrait, Set};
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct UpdateGroupRequest {
    id: String,
    name: Option<String>,
}

pub async fn update_group<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<UpdateGroupRequest>,
) -> Result<StatusCode> {
    let group = groups::ActiveModel {
        id: Set(req.id),
        name: set_option(req.name),
        ..Default::default()
    };

    groups::Entity::update(group).exec(shared.db()).await?;

    Ok(StatusCode::NO_CONTENT)
}
