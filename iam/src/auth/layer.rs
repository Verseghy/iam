use super::permission::{self, CheckError};
use crate::{shared::SharedTrait, token::Claims, utils::Error};
use hyper::Request;
use std::sync::Arc;

pub async fn validate<S: SharedTrait, B>(
    request: &Request<B>,
    actions: &[&str],
) -> Result<(), Error>
where
    B: Send + Sync + 'static,
{
    let shared = request.extensions().get::<S>().expect("No Shared");

    let claims = request
        .extensions()
        .get::<Arc<Claims>>()
        .ok_or_else(|| Error::unauthorized("missing or invalid authorization header"))?;

    permission::check(claims.subject.as_str(), actions, shared.db())
        .await
        .map_err(|err| match err {
            CheckError::DatabaseError(err) => Error::internal(err),
            CheckError::NoPermission(perm) => Error::forbidden(format!("no permission: {perm}")),
        })?;

    Ok(())
}

macro_rules! permissions {
    ($shared:ty, $($actions:literal),+ $(,)?) => {
        ::tower_http::auth::AsyncRequireAuthorizationLayer::new(move |request: ::hyper::Request<::hyper::Body>| async move {
            use ::axum::response::IntoResponse;

            match $crate::auth::validate::<$shared, ::hyper::Body>(&request, &[$($actions),+]).await {
                Ok(_) => Ok(request),
                Err(err) => Err(err.into_response()),
            }
        })
    }
}

pub(crate) use permissions;
