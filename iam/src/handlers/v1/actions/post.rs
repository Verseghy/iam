use crate::{shared::Shared, utils::set_option};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::actions;
use sea_orm::{entity::EntityTrait, DbErr, Set};
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct UpdateActionRequest {
    id: String,
    name: Option<String>,
    secure: Option<bool>,
}

pub async fn update_action(
    Extension(shared): Extension<Shared>,
    Json(req): Json<UpdateActionRequest>,
) -> Result<StatusCode, PostError> {
    let action = actions::ActiveModel {
        id: Set(req.id),
        name: set_option(req.name),
        secure: set_option(req.secure),
        ..Default::default()
    };

    actions::Entity::update(action).exec(&shared.db).await?;

    Ok(StatusCode::OK)
}

#[derive(Debug, thiserror::Error)]
pub enum PostError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
}

impl IntoResponse for PostError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
