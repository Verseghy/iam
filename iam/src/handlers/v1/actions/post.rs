use crate::{
    json::Json,
    shared::Shared,
    utils::{set_option, Error},
};
use axum::{http::StatusCode, Extension};
use entity::actions;
use sea_orm::{entity::EntityTrait, Set};
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct UpdateActionRequest {
    id: String,
    name: Option<String>,
    secure: Option<bool>,
}

pub async fn update_action(
    Extension(shared): Extension<Shared>,
    Json(req): Json<UpdateActionRequest>,
) -> Result<StatusCode, Error> {
    let action = actions::ActiveModel {
        id: Set(req.id),
        name: set_option(req.name),
        secure: set_option(req.secure),
        ..Default::default()
    };

    actions::Entity::update(action).exec(&shared.db).await?;

    Ok(StatusCode::NO_CONTENT)
}
