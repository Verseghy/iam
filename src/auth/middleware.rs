use crate::token::Claims;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::{
        header::{ToStrError, AUTHORIZATION},
        StatusCode,
    },
    web::Data,
    Error, HttpMessage, ResponseError,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use once_cell::sync::Lazy;
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
        let claims = get_claims(&req);

        let database = req
            .app_data::<Data<DatabaseConnection>>()
            .expect("No database!")
            .clone();

        req.extensions_mut().insert(PermissionsChecked);

        let fut = self.service.call(req);
        let permissions = self.permissions;

        Box::pin(async move {
            super::permission::check(claims?.subject.as_str(), permissions, database.get_ref())
                .await
                .map_err(ValidationError::from)?;

            fut.await
        })
    }
}

static VALIDATION: Lazy<Validation> = Lazy::new(|| {
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&["https://verseghy-gimnazium.net"]);
    validation.leeway = 5;

    validation
});

fn get_claims(req: &ServiceRequest) -> Result<Claims, ValidationError> {
    let header = req
        .headers()
        .get(AUTHORIZATION)
        .ok_or(ValidationError::NoAuthorizationHeader)?
        .to_str()?;

    let token = header
        .strip_prefix("Bearer: ")
        .ok_or(ValidationError::NotBearerToken)?;

    let decoding_key = req
        .app_data::<Data<DecodingKey>>()
        .expect("No decoding key!");

    Ok(jsonwebtoken::decode(token, decoding_key, &*VALIDATION)?.claims)
}

#[derive(Debug, thiserror::Error)]
enum ValidationError {
    #[error("invalid token")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),
    #[error("no authorization header")]
    NoAuthorizationHeader,
    #[error("not a utf-8 header")]
    NotUTF8Header(#[from] ToStrError),
    #[error("not Bearer token")]
    NotBearerToken,
    #[error("check error: {0}")]
    CheckError(#[from] super::permission::CheckError),
}

impl ResponseError for ValidationError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            Self::NoAuthorizationHeader => StatusCode::BAD_REQUEST,
            Self::NotUTF8Header(_) => StatusCode::BAD_REQUEST,
            Self::NotBearerToken => StatusCode::BAD_REQUEST,
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
