use crate::shared::Shared;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::users;
use sea_orm::{entity::EntityTrait, DbErr, FromQueryResult, QuerySelect};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

pub async fn list_users(
    Extension(shared): Extension<Shared>,
) -> Result<Json<Vec<User>>, GetsError> {
    let res = users::Entity::find()
        .select_only()
        .column(users::Column::Id)
        .column(users::Column::Name)
        .column(users::Column::Email)
        .into_model::<User>()
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
