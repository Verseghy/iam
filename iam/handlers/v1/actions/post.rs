use crate::{json::Json, state::StateTrait, utils::set_option};
use axum::{extract::State, http::StatusCode};
use iam_common::error::Result;
use iam_entity::actions;
use sea_orm::{entity::EntityTrait, Set};
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct UpdateActionRequest {
    id: String,
    name: Option<String>,
    secure: Option<bool>,
}

pub async fn update_action<S: StateTrait>(
    State(state): State<S>,
    Json(req): Json<UpdateActionRequest>,
) -> Result<StatusCode> {
    let action = actions::ActiveModel {
        id: Set(req.id),
        name: set_option(req.name),
        secure: set_option(req.secure),
        ..Default::default()
    };

    actions::Entity::update(action).exec(state.db()).await?;

    Ok(StatusCode::NO_CONTENT)
}
