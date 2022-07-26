use crate::{password, shared::Shared};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::users;
use sea_orm::{entity::EntityTrait, ActiveValue, DbErr, Set, Value};
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct UpdateUserRequest {
    id: String,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

fn set<T>(value: Option<T>) -> ActiveValue<T>
where
    T: Into<Value>,
{
    match value {
        Some(v) => ActiveValue::Set(v),
        None => ActiveValue::NotSet,
    }
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
        name: set(req.name),
        email: set(req.email),
        password: set(hash),
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
