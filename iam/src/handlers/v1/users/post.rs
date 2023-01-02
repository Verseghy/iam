use crate::{json::Json, shared::SharedTrait, utils::set_option};
use axum::{http::StatusCode, Extension};
use common::{error::Result, password};
use entity::users;
use sea_orm::{entity::EntityTrait, Set};
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct UpdateUserRequest {
    id: String,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

pub async fn update_user<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<StatusCode> {
    let hash = match req.password {
        Some(pwd) => Some(password::hash(&pwd)?),
        None => None,
    };

    let user = users::ActiveModel {
        id: Set(req.id.clone()),
        name: set_option(req.name),
        email: set_option(req.email),
        password: set_option(hash),
        ..Default::default()
    };

    users::Entity::insert(user).exec(shared.db()).await?;

    Ok(StatusCode::NO_CONTENT)
}
