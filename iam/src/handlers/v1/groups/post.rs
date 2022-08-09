use crate::{
    json::Json,
    shared::Shared,
    utils::{set_option, Error},
};
use axum::{http::StatusCode, Extension};
use entity::groups;
use sea_orm::{entity::EntityTrait, Set};
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct UpdateGroupRequest {
    id: String,
    name: Option<String>,
}

pub async fn update_group(
    Extension(shared): Extension<Shared>,
    Json(req): Json<UpdateGroupRequest>,
) -> Result<StatusCode, Error> {
    let group = groups::ActiveModel {
        id: Set(req.id),
        name: set_option(req.name),
        ..Default::default()
    };

    groups::Entity::update(group).exec(&shared.db).await?;

    Ok(StatusCode::NO_CONTENT)
}
