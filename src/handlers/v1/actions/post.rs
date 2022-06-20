use std::default::Default;
use crate::{password, token};
use actix_web::{http::StatusCode, route, web, ResponseError, Responder, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header, errors::{Error as JWTError}, Algorithm};
use entity::actions;
use sea_orm::{entity::{ActiveModelTrait, ColumnTrait, EntityTrait}, query::QueryFilter, ActiveValue, DatabaseConnection, DbErr, NotSet, Set};
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Deserialize, Debug)]
pub struct UpdateActionRequest {
    id: String,
    name: Option<String>,
    secure: Option<bool>,
}

pub async fn post(
    req: web::Json<UpdateActionRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<impl Responder, PostError> {
    let req = req.into_inner();

    let action = actions::ActiveModel {
        id: Set(req.id),
        name: if let Some(name) = req.name { Set(name) } else { NotSet },
        secure: if let Some(secure) = req.secure { Set(secure) } else { NotSet },
        ..Default::default()
    };

    actions::Entity::update(action)
        .exec(db.get_ref())
        .await?;

    Ok(HttpResponse::new(StatusCode::OK))
}

#[derive(Debug, thiserror::Error)]
pub enum PostError {
    #[error("database error")]
    DatabaseError(#[from]DbErr),
}

impl ResponseError for PostError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
