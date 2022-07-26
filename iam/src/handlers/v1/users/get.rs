use crate::shared::Shared;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::users;
use sea_orm::{entity::EntityTrait, DbErr};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetUserResponse {
    id: String,
    name: String,
    email: String,
}

pub async fn get_user(
    Extension(shared): Extension<Shared>,
    Path(id): Path<String>,
) -> Result<Json<GetUserResponse>, GetError> {
    let res = users::Entity::find_by_id(id)
        .one(&shared.db)
        .await?
        .ok_or(GetError::NotFoundError)?;

    Ok(Json(GetUserResponse {
        id: res.id,
        name: res.name,
        email: res.email,
    }))
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
    #[error("user not found")]
    NotFoundError,
}

impl IntoResponse for GetError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFoundError => StatusCode::NOT_FOUND,
        };
        (status_code, self.to_string()).into_response()
    }
}
