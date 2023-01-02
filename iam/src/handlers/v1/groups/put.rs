use crate::{json::Json, shared::SharedTrait};
use axum::Extension;
use common::{error::Result, Id};
use entity::groups;
use sea_orm::{entity::EntityTrait, Set};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct AddGroupRequest {
    name: String,
}

#[derive(Serialize, Debug)]
pub struct AddGroupResponse {
    id: Id,
}

pub async fn add_group<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<AddGroupRequest>,
) -> Result<Json<AddGroupResponse>> {
    let id = Id::new_group();

    let group = groups::ActiveModel {
        name: Set(req.name),
        ..Default::default()
    };

    groups::Entity::insert(group).exec(shared.db()).await?;

    Ok(Json(AddGroupResponse { id }))
}
