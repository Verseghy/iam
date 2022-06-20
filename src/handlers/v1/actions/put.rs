use std::default::Default;
use crate::{password, token};
use actix_web::{http::StatusCode, route, web, ResponseError, Responder, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header, errors::{Error as JWTError}, Algorithm};
use entity::actions;
use sea_orm::{entity::{ActiveModelTrait, ColumnTrait, EntityTrait}, query::QueryFilter, ActiveValue, DatabaseConnection, DbErr, NotSet, Set};
use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::id::create_id;

#[derive(Deserialize, Debug)]
pub struct AddActionRequest {
    action: String,
    secure: bool,
}

#[derive(Serialize, Debug)]
pub struct AddActionResponse {
    id: String
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

    actions::Entity::insert(action)
        .exec(db.get_ref())
        .await?;

    Ok(web::Json(AddActionResponse{
        id,
    }))
}

#[derive(Debug, thiserror::Error)]
pub enum PutError {
    #[error("database error")]
    DatabaseError(#[from]DbErr),
}

impl ResponseError for PutError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
