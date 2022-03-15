use crate::password;
use actix_web::{http::StatusCode, route, web, HttpResponse, ResponseError};
use entity::users;
use sea_orm::{
    entity::{ActiveModelTrait, ColumnTrait, EntityTrait},
    query::QueryFilter,
    ActiveValue, DatabaseConnection, DbErr,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RegisterRequest {
    #[validate(email, length(max = 256))]
    email: String,
    #[validate(length(max = 256))]
    password: String,
}

#[route("/v1/login", method = "POST")]
pub async fn login(
    req: web::Json<RegisterRequest>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, LoginError> {
    let req = req.into_inner();

    req.validate().map_err(|_| LoginError::BadInputData)?;

    let res = users::Entity::find()
        .filter(users::Column::Email.eq(req.email.clone()))
        .one(db.get_ref())
        .await
        .map_err(LoginError::DatabaseError)?
        .ok_or(LoginError::NoUser)?;

    let (valid, rehash) =
        password::validate(&res.password, &req.password).map_err(LoginError::ValidationError)?;

    if let Some(Ok(hash)) = rehash {
        let mut action: users::ActiveModel = res.into();
        action.password = ActiveValue::Set(hash);

        action
            .update(db.get_ref())
            .await
            .map_err(LoginError::DatabaseError)?;
    }

    if valid {
        Ok(HttpResponse::new(StatusCode::OK))
    } else {
        Err(LoginError::WrongPassword)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("no user with this email")]
    NoUser,
    #[error("database error")]
    DatabaseError(DbErr),
    #[error("failed to validate input data")]
    BadInputData,
    #[error("failed to validate password")]
    ValidationError(password::ValidateError),
    #[error("wrong password")]
    WrongPassword,
    // #[error("failed to rehash")]
    // FailedToRehash,
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::NoUser => StatusCode::UNAUTHORIZED,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BadInputData => StatusCode::BAD_REQUEST,
            Self::ValidationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::WrongPassword => StatusCode::UNAUTHORIZED,
            // Self::FailedToRehash => StatusCode::OK,
        }
    }
}
