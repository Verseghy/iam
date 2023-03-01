use crate::{json::Json, shared::SharedTrait};
use axum::Extension;
use iam_common::{error::Result, Id};
use iam_entity::actions;
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
    id: Id,
}

pub async fn add_action<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<AddActionRequest>,
) -> Result<Json<AddActionResponse>> {
    let id = Id::new_action();

    let action = actions::ActiveModel {
        id: Set(id.to_string()),
        name: Set(req.action),
        secure: Set(req.secure),
        ..Default::default()
    };

    actions::Entity::insert(action).exec(shared.db()).await?;

    Ok(Json(AddActionResponse { id }))
}
