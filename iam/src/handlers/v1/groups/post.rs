use crate::{shared::Shared, utils::set_option};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::groups;
use sea_orm::{entity::EntityTrait, DbErr, Set};
use serde::Deserialize;
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct UpdateGroupRequest {
    id: String,
    name: Option<String>,
}

pub async fn update_group(
    Extension(shared): Extension<Shared>,
    Json(req): Json<UpdateGroupRequest>,
) -> Result<StatusCode, PostError> {
    let group = groups::ActiveModel {
        id: Set(req.id),
        name: set_option(req.name),
        ..Default::default()
    };

    groups::Entity::update(group).exec(&shared.db).await?;

    Ok(StatusCode::NO_CONTENT)
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
