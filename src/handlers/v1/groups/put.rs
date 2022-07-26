use crate::{id::create_id, shared::Shared};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::groups;
use sea_orm::{entity::EntityTrait, DbErr, Set};
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Deserialize, Debug)]
pub struct AddGroupRequest {
    name: String,
}

#[derive(Serialize, Debug)]
pub struct AddGroupResponse {
    id: String,
}

pub async fn add_group(
    Extension(shared): Extension<Shared>,
    Json(req): Json<AddGroupRequest>,
) -> Result<Json<AddGroupResponse>, PutError> {
    let id = format!("GroupID-{}", create_id());

    let group = groups::ActiveModel {
        name: Set(req.name),
        ..Default::default()
    };

    groups::Entity::insert(group).exec(&shared.db).await?;

    Ok(Json(AddGroupResponse { id }))
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
