use crate::shared::Shared;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::actions;
use sea_orm::{entity::EntityTrait, DbErr};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetResponse {
    id: String,
    name: String,
    secure: bool,
}
pub async fn get_action(
    Extension(shared): Extension<Shared>,
    Path(id): Path<String>,
) -> Result<Json<GetResponse>, GetError> {
    let res = actions::Entity::find_by_id(id)
        .one(&shared.db)
        .await?
        .ok_or(GetError::NotFoundError)?;

    Ok(Json(GetResponse {
        id: res.id,
        name: res.name,
        secure: res.secure,
    }))
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
    #[error("not found")]
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
