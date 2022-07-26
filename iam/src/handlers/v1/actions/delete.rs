use crate::shared::Shared;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::actions;
use sea_orm::{entity::EntityTrait, DbErr};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DeleteActionRequest {
    id: String,
}

pub async fn delete_action(
    Extension(shared): Extension<Shared>,
    Json(req): Json<DeleteActionRequest>,
) -> Result<StatusCode, DeleteError> {
    actions::Entity::delete_by_id(req.id)
        .exec(&shared.db)
        .await?;

    Ok(StatusCode::OK)
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
}

impl IntoResponse for DeleteError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
