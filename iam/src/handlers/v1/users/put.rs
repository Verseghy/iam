use crate::{json::Json, shared::SharedTrait, utils::Error};
use axum::Extension;
use common::{create_user_id, password};
use entity::users;
use sea_orm::{entity::EntityTrait, Set};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct AddUserRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Debug)]
pub struct AddUserResponse {
    id: String,
}

pub async fn add_user<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<AddUserRequest>,
) -> Result<Json<AddUserResponse>, Error> {
    let id = create_user_id();

    let hash = password::hash(&req.password).map_err(Error::internal)?;

    let user = users::ActiveModel {
        id: Set(id.clone()),
        name: Set(req.name),
        email: Set(req.email),
        password: Set(hash),
        ..Default::default()
    };

    users::Entity::insert(user).exec(shared.db()).await?;

    Ok(Json(AddUserResponse { id }))
}
