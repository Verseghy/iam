use actix_web::{http::StatusCode, web, Responder, ResponseError};
use entity::actions;

use sea_orm::{entity::EntityTrait, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};

use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct GetRequest {
    id: String,
}

#[derive(Serialize, Debug)]
pub struct GetResponse {
    id: String,
    name: String,
    secure: bool,
}

pub async fn get(
    req: web::Query<GetRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<impl Responder, GetError> {
    let req = req.into_inner();

    let res = actions::Entity::find_by_id(req.id)
        .one(db.get_ref())
        .await?
        .ok_or(GetError::NotFoundError)?;

    Ok(web::Json(GetResponse {
        id: res.id,
        name: res.name,
        secure: res.secure,
    }))
}

#[derive(Debug, thiserror::Error)]
pub enum GetError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
    #[error("not found")]
    NotFoundError,
}

impl ResponseError for GetError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFoundError => StatusCode::NOT_FOUND,
        }
    }
}
