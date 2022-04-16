use std::default::Default;
use crate::{password, token};
use actix_web::{http::StatusCode, route, web, ResponseError, Responder};
use jsonwebtoken::{encode, EncodingKey, Header, errors::{Error as JWTError}, Algorithm};
use entity::users;
use sea_orm::{
    entity::{ActiveModelTrait, ColumnTrait, EntityTrait},
    query::QueryFilter,
    ActiveValue, DatabaseConnection, DbErr,
};
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginRequest {
    #[validate(email, length(max = 256))]
    email: String,
    #[validate(length(max = 256))]
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    token: String
}

#[route("/login", method = "POST")]
pub async fn login(
    req: web::Json<LoginRequest>,
    db: web::Data<DatabaseConnection>,
    encoding_key: web::Data<EncodingKey>,
) -> Result<impl Responder, LoginError> {
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
        let mut action: users::ActiveModel = res.clone().into();
        action.password = ActiveValue::Set(hash);

        action
            .update(db.get_ref())
            .await
            .map_err(LoginError::DatabaseError)?;
    }

    if !valid {
        return Err(LoginError::WrongPassword);
    }

    let claims = &token::Claims{
        subject: res.id.to_string(),
        ..Default::default()
    };

    let token = encode(&Header::new(Algorithm::RS256), claims, &encoding_key)?;

    Ok(web::Json(LoginResponse{
        token
    }))
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
    #[error("failed to generate JWT token")]
    TokenError(#[from] JWTError),
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
            Self::TokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // Self::FailedToRehash => StatusCode::OK,
        }
    }
}
