use actix_web::{http::StatusCode, web, HttpResponse, Responder, ResponseError};
use entity::actions;

use sea_orm::{entity::EntityTrait, DatabaseConnection, DbErr};
use serde::Deserialize;

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
