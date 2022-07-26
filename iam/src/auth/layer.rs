use super::permission::{self, CheckError};
use crate::{shared::Shared, token::Claims};
use axum::{body::BoxBody, http::StatusCode, response::IntoResponse};
use hyper::{Request, Response};
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("get claims error")]
    GetClaimsError,
    #[error("check error: {0}")]
    CheckError(#[from] permission::CheckError),
}

impl IntoResponse for ValidationError {
    fn into_response(self) -> Response<BoxBody> {
        let status_code = match self {
            Self::GetClaimsError => StatusCode::UNAUTHORIZED,
            Self::CheckError(CheckError::DatabaseError(_)) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::CheckError(CheckError::NoPermission(_)) => StatusCode::FORBIDDEN,
        };
        (status_code, self.to_string()).into_response()
    }
}

pub async fn validate<B>(request: &Request<B>, actions: &[&str]) -> Result<(), ValidationError>
where
    B: Send + Sync + 'static,
{
    let shared = request.extensions().get::<Shared>().expect("No Shared");

    let claims = request
        .extensions()
        .get::<Arc<Claims>>()
        .ok_or(ValidationError::GetClaimsError)?;

    permission::check(claims.subject.as_str(), actions, &shared.db).await?;

    Ok(())
}

macro_rules! permissions {
    ($($actions:literal),+ $(,)?) => {
        ::tower_http::auth::AsyncRequireAuthorizationLayer::new(move |request: ::hyper::Request<::hyper::Body>| async move {
            use ::axum::response::IntoResponse;

            match $crate::auth::validate(&request, &[$($actions),+]).await {
                Ok(_) => Ok(request),
                Err(err) => Err(err.into_response()),
            }
        })
    }
}

pub(crate) use permissions;
