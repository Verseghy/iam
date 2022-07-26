use crate::shared::Shared;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::groups;
use sea_orm::{entity::EntityTrait, DbErr, FromQueryResult, QuerySelect};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct Group {
    id: String,
    name: String,
}

pub async fn list_groups(
    Extension(shared): Extension<Shared>,
) -> Result<Json<Vec<Group>>, GetsError> {
    let res = groups::Entity::find()
        .select_only()
        .column(groups::Column::Id)
        .column(groups::Column::Name)
        .into_model::<Group>()
        .all(&shared.db)
        .await?;

    Ok(Json(res))
}

#[derive(Debug, thiserror::Error)]
pub enum GetsError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
}

impl IntoResponse for GetsError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
