use super::permission;
use crate::state::StateTrait;
use axum::{
    body::Body,
    extract::Request,
    response::{IntoResponse, Response},
};
use futures_util::future::BoxFuture;
use iam_common::{error, keys::jwt::Claims};
use std::sync::Arc;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

#[derive(Debug, Clone)]
pub struct Auth<S> {
    actions: &'static [&'static str],
    state: S,
}

impl<S: StateTrait> AsyncAuthorizeRequest<Body> for Auth<S> {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = BoxFuture<'static, Result<Request, Response>>;

    fn authorize(&mut self, request: Request) -> Self::Future {
        let Some(claims) = request.extensions().get::<Arc<Claims>>() else {
            return Box::pin(async move { Err(error::INVALID_AUTH_HEADER.into_response()) });
        };

        let state = self.state.clone();
        let claims = claims.clone();
        let actions = self.actions;

        Box::pin(async move {
            if let Err(err) = permission::check(claims.sub.as_str(), actions, state.db()).await {
                return Err(err.into_response());
            };

            Ok(request)
        })
    }
}

pub fn permissions<S: StateTrait>(
    state: S,
    actions: &'static [&'static str],
) -> AsyncRequireAuthorizationLayer<Auth<S>> {
    AsyncRequireAuthorizationLayer::new(Auth { actions, state })
}
