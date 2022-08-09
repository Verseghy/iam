use crate::{json::Json, shared::Shared, utils::Error};
use axum::Extension;
use common::create_action_id;
use entity::actions;
use sea_orm::{entity::EntityTrait, Set};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct AddActionRequest {
    action: String,
    secure: bool,
}

#[derive(Serialize, Debug)]
pub struct AddActionResponse {
    id: String,
}

pub async fn add_action(
    Extension(shared): Extension<Shared>,
    Json(req): Json<AddActionRequest>,
) -> Result<Json<AddActionResponse>, Error> {
    let id = create_action_id();

    let action = actions::ActiveModel {
        id: Set(id.clone()),
        name: Set(req.action),
        secure: Set(req.secure),
        ..Default::default()
    };

    actions::Entity::insert(action).exec(&shared.db).await?;

    Ok(Json(AddActionResponse { id }))
}
