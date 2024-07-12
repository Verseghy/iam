use super::permission;
use crate::shared::SharedTrait;
use hyper::Request;
use iam_common::{
    error::{self, Result},
    keys::jwt::Claims,
};
use std::sync::Arc;

pub async fn validate<S: SharedTrait, B>(request: &Request<B>, actions: &[&str]) -> Result<()>
where
    B: Send + Sync + 'static,
{
    let shared = request.extensions().get::<S>().expect("No Shared");

    let claims = request
        .extensions()
        .get::<Arc<Claims>>()
        .ok_or(error::INVALID_AUTH_HEADER)?;

    permission::check(claims.sub.as_str(), actions, shared.db()).await?;

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
