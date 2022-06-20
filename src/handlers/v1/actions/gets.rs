use std::default::Default;
use crate::{password, token};
use actix_web::{http::StatusCode, route, web, ResponseError, Responder};
use jsonwebtoken::{encode, EncodingKey, Header, errors::{Error as JWTError}, Algorithm};
use entity::{actions, users};
use sea_orm::{
    entity::{ActiveModelTrait, ColumnTrait, EntityTrait},
    query::QueryFilter,
    ActiveValue, DatabaseConnection, DbErr,
};
use sea_orm::ActiveValue::Set;
use serde::{Serialize, Deserialize};
use validator::Validate;

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

pub async fn gets(
    db: web::Data<DatabaseConnection>,
) -> Result<impl Responder, GetsError> {
    let res = actions::Entity::find()
        .all(db.get_ref())
        .await?;

    Ok(web::Json(GetsResponse{
        actions: res.into_iter().map(|x| {
            Action{
                id: x.id,
                name: x.name,
                secure: x.secure
            }
        }).collect()
    }))
}

#[derive(Debug, thiserror::Error)]
pub enum GetsError {
    #[error("database error")]
    DatabaseError(#[from]DbErr),
}

impl ResponseError for GetsError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
