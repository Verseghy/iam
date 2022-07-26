use crate::shared::Shared;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::groups;
use sea_orm::{entity::EntityTrait, DbErr};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteGroupRequest {
    id: String,
}

pub async fn delete_group(
    Extension(shared): Extension<Shared>,
    Json(req): Json<DeleteGroupRequest>,
) -> Result<StatusCode, DeleteError> {
    let res = groups::Entity::delete_by_id(req.id)
        .exec(&shared.db)
        .await?;

    if res.rows_affected == 0 {
        Err(DeleteError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
    #[error("user not found")]
    NotFound,
}

impl IntoResponse for DeleteError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
        };
        (status_code, self.to_string()).into_response()
    }
}
