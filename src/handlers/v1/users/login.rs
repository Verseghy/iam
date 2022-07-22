use crate::{password, shared::Shared, token, validate::ValidatedJson};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use entity::users;
use jsonwebtoken::errors::Error as JWTError;
use sea_orm::{
    entity::{ActiveModelTrait, ColumnTrait, EntityTrait},
    query::QueryFilter,
    ActiveValue, DbErr,
};
use serde::{Deserialize, Serialize};
use std::default::Default;
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
    token: String,
}

pub async fn login(
    Extension(shared): Extension<Shared>,
    ValidatedJson(mut req): ValidatedJson<LoginRequest>,
) -> Result<Json<LoginResponse>, LoginError> {
    req.email = req.email.to_lowercase();

    let res = users::Entity::find()
        .filter(users::Column::Email.eq(req.email.clone()))
        .one(&shared.db)
        .await
        .map_err(LoginError::DatabaseError)?
        .ok_or(LoginError::NoUser)?;

    let (valid, rehash) =
        password::validate(&res.password, &req.password).map_err(LoginError::ValidationError)?;

    if let Some(Ok(hash)) = rehash {
        let mut action: users::ActiveModel = res.clone().into();
        action.password = ActiveValue::Set(hash);

        action
            .update(&shared.db)
            .await
            .map_err(LoginError::DatabaseError)?;
    }

    if !valid {
        return Err(LoginError::WrongPassword);
    }

    let claims = token::Claims {
        subject: res.id.to_string(),
        ..Default::default()
    };

    let token = shared.jwt.encode(&claims)?;

    crate::audit!(action = "login", user = res.id.to_string(),);

    Ok(Json(LoginResponse { token }))
}

#[derive(Debug, thiserror::Error)]
pub enum LoginError {
    #[error("no user with this email")]
    NoUser,
    #[error("database error")]
    DatabaseError(DbErr),
    #[error("failed to validate password")]
    ValidationError(password::ValidateError),
    #[error("wrong password")]
    WrongPassword,
    #[error("failed to generate JWT token")]
    TokenError(#[from] JWTError),
    // #[error("failed to rehash")]
    // FailedToRehash,
}

impl IntoResponse for LoginError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::NoUser => StatusCode::UNAUTHORIZED,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::WrongPassword => StatusCode::UNAUTHORIZED,
            Self::TokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // Self::FailedToRehash => StatusCode::OK,
        };
        (status_code, self.to_string()).into_response()
    }
}
