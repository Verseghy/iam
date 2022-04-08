use crate::password;
use actix_web::{http::StatusCode, route, web, HttpResponse, ResponseError};
use entity::{invited_users, users};
use sea_orm::{
    entity::{ActiveModelTrait, ColumnTrait, EntityTrait},
    query::QueryFilter,
    ActiveValue, DatabaseConnection, DbErr,
};
use serde::Deserialize;
use validator::Validate;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::sea_query::ColumnSpec::Default;
use crate::handlers::v1::users::invite;

/*
TODO: invites expire
*/

#[derive(Deserialize, Debug, Validate)]
pub struct InviteRequest {
    #[validate(email, length(max = 256))]
    email: String,
}

#[route("/invite", method = "POST")]
pub async fn invite(
    req: web::Json<InviteRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, InviteError> {
    let req = req.into_inner();

    req.validate().map_err(|_| InviteError::BadInputData)?;

    // If already invited fail
    let res = !invited_users::Entity::find()
        .filter(invited_users::Column::Email.eq(&req.email))
        .one(db.get_ref())
        .await
        .map_err(InviteError::DatabaseError)?;
    if let Ok(_) = res {
        return Err(InviteError::AlreadyExists);
    }

    let token: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let action = invited_users::ActiveModel {
        email: Set(req.email),
        token: Set(token),
        ..Default::default()
    };
    invited_users::Entity::insert(action).exec(db.get_ref()).await.map_err(InviteError::DatabaseError)?;

    // send mail

    Ok(HttpResponse::new(StatusCode::OK))
}

#[derive(Debug, thiserror::Error)]
pub enum InviteError {
    #[error("user already invited")]
    AlreadyExists,
    #[error("database error")]
    DatabaseError(DbErr),
    #[error("failed to validate input data")]
    BadInputData,
}

impl ResponseError for InviteError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::AlreadyExists => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadInputData => StatusCode::BAD_REQUEST,
        }
    }
}
