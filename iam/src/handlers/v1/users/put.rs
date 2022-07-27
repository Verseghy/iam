use crate::shared::Shared;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use common::create_user_id;
use common::password::{self, HashError};
use entity::users;
use sea_orm::{entity::EntityTrait, DbErr, Set};
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

pub async fn add_user(
    Extension(shared): Extension<Shared>,
    Json(req): Json<AddUserRequest>,
) -> Result<Json<AddUserResponse>, PutError> {
    let id = create_user_id();

    let hash = password::hash(&req.password)?;

    let user = users::ActiveModel {
        id: Set(id.clone()),
        name: Set(req.name),
        email: Set(req.email),
        password: Set(hash),
        ..Default::default()
    };

    users::Entity::insert(user).exec(&shared.db).await?;

    Ok(Json(AddUserResponse { id }))
}

#[derive(Debug, thiserror::Error)]
pub enum PutError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
    #[error("unknown error")]
    HashError(#[from] HashError),
}

impl IntoResponse for PutError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::HashError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
