use crate::shared::Shared;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::groups;
use sea_orm::{entity::EntityTrait, DbErr, FromQueryResult};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct GetResponse {
    id: String,
    name: String,
}
pub async fn get_group(
    Extension(shared): Extension<Shared>,
    Path(id): Path<String>,
) -> Result<Json<GetResponse>, GetError> {
    let res = groups::Entity::find_by_id(id)
        .into_model::<GetResponse>()
        .one(&shared.db)
        .await?
        .ok_or(GetError::NotFound)?;

    Ok(Json(res))
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
    #[error("not found")]
    NotFound,
}

impl IntoResponse for GetError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound => StatusCode::NOT_FOUND,
        };
        (status_code, self.to_string()).into_response()
    }
}
