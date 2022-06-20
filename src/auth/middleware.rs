use crate::token::{get_claims, GetClaimsError};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::StatusCode,
    web::Data,
    Error, HttpMessage, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;
use std::future::{ready, Ready};

pub struct PermissionsChecked;

pub struct Permission {
    permissions: &'static [&'static str],
}

impl Permission {
    pub fn new(permissions: &'static [&'static str]) -> Self {
        Self { permissions }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Permission
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionMiddleware {
            service,
            permissions: self.permissions,
        }))
    }
}

pub struct PermissionMiddleware<S> {
    permissions: &'static [&'static str],
    service: S,
}

impl<S, B> Service<ServiceRequest> for PermissionMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let decoding_key = req
            .app_data::<Data<DecodingKey>>()
            .expect("No DecodingKey!");

        let claims = get_claims(req.headers(), decoding_key);

        let database = req
            .app_data::<Data<DatabaseConnection>>()
            .expect("No database!")
            .clone();

        req.extensions_mut().insert(PermissionsChecked);

        let fut = self.service.call(req);
        let permissions = self.permissions;

        Box::pin(async move {
            let claims = claims.map_err(ValidationError::from)?;
            super::permission::check(claims.subject.as_str(), permissions, database.get_ref())
                .await
                .map_err(ValidationError::from)?;

            fut.await
        })
    }
}

#[derive(Debug, thiserror::Error)]
enum ValidationError {
    #[error("get claims error: {0}")]
    GetClaimsError(#[from] GetClaimsError),
    #[error("check error: {0}")]
    CheckError(#[from] super::permission::CheckError),
}

impl ResponseError for ValidationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::GetClaimsError(GetClaimsError::InvalidToken(_)) => StatusCode::UNAUTHORIZED,
            Self::GetClaimsError(GetClaimsError::NoAuthorizationHeader) => StatusCode::BAD_REQUEST,
            Self::GetClaimsError(GetClaimsError::NotUTF8Header(_)) => StatusCode::BAD_REQUEST,
            Self::GetClaimsError(GetClaimsError::NotBearerToken) => StatusCode::BAD_REQUEST,
            Self::CheckError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

macro_rules! permissions {
    ($($permission:literal),+ $(,)?) => {
        $crate::auth::middleware::Permission::new(&[$($permission),+])
    }
}

pub(crate) use permissions;
