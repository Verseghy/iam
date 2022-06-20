use crate::id::create_id;

use actix_web::{http::StatusCode, web, Responder, ResponseError};
use entity::actions;

use sea_orm::{entity::EntityTrait, DatabaseConnection, DbErr, Set};
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

pub async fn put(
    req: web::Json<AddActionRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<impl Responder, PutError> {
    let req = req.into_inner();

    let id = format!("ActionID-{}", create_id());

    let action = actions::ActiveModel {
        id: Set(id.clone()),
        name: Set(req.action),
        secure: Set(req.secure),
        ..Default::default()
    };

    actions::Entity::insert(action).exec(db.get_ref()).await?;

    Ok(web::Json(AddActionResponse { id }))
}

#[derive(Debug, thiserror::Error)]
pub enum PutError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
}

impl ResponseError for PutError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
