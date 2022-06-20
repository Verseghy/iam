use crate::handlers::v1::actions::post::PostError::DatabaseError;
use crate::{password, token};
use actix_web::{http::StatusCode, route, web, HttpResponse, Responder, ResponseError};
use entity::actions;
use jsonwebtoken::{encode, errors::Error as JWTError, Algorithm, EncodingKey, Header};
use sea_orm::{
    entity::{ActiveModelTrait, ColumnTrait, EntityTrait},
    query::QueryFilter,
    ActiveValue, DatabaseConnection, DbErr, NotSet, Set,
};
use serde::{Deserialize, Serialize};
use std::default::Default;
use validator::Validate;

#[derive(Deserialize, Debug)]
pub struct DeleteActionRequest {
    id: String,
}

pub async fn delete(
    req: web::Json<DeleteActionRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<impl Responder, DeleteError> {
    let req = req.into_inner();

    actions::Entity::delete_by_id(req.id)
        .exec(db.get_ref())
        .await?;

    Ok(HttpResponse::new(StatusCode::OK))
}

#[derive(Debug, thiserror::Error)]
pub enum DeleteError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
}

impl ResponseError for DeleteError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
