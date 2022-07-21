use crate::shared::Shared;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::actions;
use sea_orm::{entity::EntityTrait, DbErr};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetsResponse {
    actions: Vec<Action>,
}

#[derive(Serialize, Debug)]
struct Action {
    id: String,
    name: String,
    secure: bool,
}

pub async fn list_actions(
    Extension(shared): Extension<Shared>,
) -> Result<Json<GetsResponse>, GetsError> {
    let res = actions::Entity::find().all(&shared.db).await?;

    Ok(Json(GetsResponse {
        actions: res
            .into_iter()
            .map(|x| Action {
                id: x.id,
                name: x.name,
                secure: x.secure,
            })
            .collect(),
    }))
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
