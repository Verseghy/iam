use crate::{
    json::Json,
    shared::Shared,
    utils::{set_option, Error},
};
use axum::{http::StatusCode, Extension};
use common::password;
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

pub async fn update_user(
    Extension(shared): Extension<Shared>,
    Json(req): Json<UpdateUserRequest>,
) -> Result<StatusCode, Error> {
    let hash = match req.password {
        Some(pwd) => Some(password::hash(&pwd).map_err(Error::internal)?),
        None => None,
    };

    let user = users::ActiveModel {
        id: Set(req.id.clone()),
        name: set_option(req.name),
        email: set_option(req.email),
        password: set_option(hash),
        ..Default::default()
    };

    users::Entity::insert(user).exec(&shared.db).await?;

    Ok(StatusCode::NO_CONTENT)
}
