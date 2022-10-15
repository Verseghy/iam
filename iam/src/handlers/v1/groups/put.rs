use crate::{json::Json, shared::SharedTrait, utils::Result};
use axum::Extension;
use common::create_group_id;
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
    id: String,
}

pub async fn add_group<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<AddGroupRequest>,
) -> Result<Json<AddGroupResponse>> {
    let id = create_group_id();

    let group = groups::ActiveModel {
        name: Set(req.name),
        ..Default::default()
    };

    groups::Entity::insert(group).exec(shared.db()).await?;

    Ok(Json(AddGroupResponse { id }))
}
