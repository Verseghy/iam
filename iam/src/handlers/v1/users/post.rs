use crate::{password, shared::Shared, util::set_option};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::users;
use sea_orm::{entity::EntityTrait, DbErr, Set};
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
) -> Result<StatusCode, PostError> {
    let hash = match req.password {
        Some(pwd) => Some(password::encrypt(&pwd)?),
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

#[derive(Debug, thiserror::Error)]
pub enum PostError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
    #[error("unknown error")]
    HashError(#[from] argon2::Error),
}

impl IntoResponse for PostError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
