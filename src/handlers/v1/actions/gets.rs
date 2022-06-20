use actix_web::{http::StatusCode, web, Responder, ResponseError};
use entity::actions;

use sea_orm::{entity::EntityTrait, DatabaseConnection, DbErr};
use serde::Serialize;

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

pub async fn gets(db: web::Data<DatabaseConnection>) -> Result<impl Responder, GetsError> {
    let res = actions::Entity::find().all(db.get_ref()).await?;

    Ok(web::Json(GetsResponse {
        actions: res
            .into_iter()
            .map(|x| Action {
                id: x.id,
                name: x.name,
                secure: x.secure,
            })
            .collect(),
    }))
}

#[derive(Debug, thiserror::Error)]
pub enum GetsError {
    #[error("database error")]
    DatabaseError(#[from] DbErr),
}

impl ResponseError for GetsError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
