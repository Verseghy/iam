use crate::shared::Shared;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use common::create_action_id;
use entity::actions;
use sea_orm::{entity::EntityTrait, DbErr, Set};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct AddActionRequest {
    action: String,
    secure: bool,
}

#[derive(Serialize, Debug)]
pub struct AddActionResponse {
    id: String,
}

pub async fn add_action(
    Extension(shared): Extension<Shared>,
    Json(req): Json<AddActionRequest>,
) -> Result<Json<AddActionResponse>, PutError> {
    let id = create_action_id();

    let action = actions::ActiveModel {
        id: Set(id.clone()),
        name: Set(req.action),
        secure: Set(req.secure),
        ..Default::default()
    };

    actions::Entity::insert(action).exec(&shared.db).await?;

    Ok(Json(AddActionResponse { id }))
}

#[derive(Debug, thiserror::Error)]
pub enum PutError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
}

impl IntoResponse for PutError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
