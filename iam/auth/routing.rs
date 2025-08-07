use crate::state::StateTrait;
use axum::{
    extract::Request,
    handler::Handler,
    response::{IntoResponse, Response},
    routing::{MethodFilter, MethodRouter},
};
use iam_common::{error, keys::jwt::Claims};
use std::{convert::Infallible, future::Future, pin::Pin, sync::Arc};

#[derive(Debug, Clone)]
struct AuthHandler<H> {
    actions: &'static [&'static str],
    inner: H,
}

impl<T, S, H> Handler<T, S> for AuthHandler<H>
where
    T: 'static,
    H: Handler<T, S>,
    S: Clone + Send + Sync + StateTrait + 'static,
{
    type Future = Pin<Box<dyn Future<Output = Response> + Send>>;

    fn call(self, req: Request, state: S) -> Self::Future {
        let Some(claims) = req.extensions().get::<Arc<Claims>>() else {
            return Box::pin(async move { error::INVALID_AUTH_HEADER.into_response() });
        };

        let claims = claims.clone();

        Box::pin(async move {
            if let Err(err) = super::check(&claims.sub, self.actions, state.db()).await {
                return err.into_response();
            }

            self.inner.call(req, state).await
        })
    }
}

pub fn auth_on<H, T, S>(
    filter: MethodFilter,
    handler: H,
    actions: &'static [&'static str],
) -> MethodRouter<S, Infallible>
where
    H: Handler<T, S>,
    T: 'static,
    S: Clone + Send + Sync + StateTrait + 'static,
{
    let auth_handler = AuthHandler {
        actions,
        inner: handler,
    };

    MethodRouter::new().on(filter, auth_handler)
}

macro_rules! impl_method {
    (
        $name:ident, $method:ident
    ) => {
        #[inline]
        pub fn $name<H, T, S>(
            handler: H,
            actions: &'static [&'static str],
        ) -> MethodRouter<S, Infallible>
        where
            H: Handler<T, S>,
            T: 'static,
            S: Clone + Send + Sync + StateTrait + 'static,
        {
            auth_on(MethodFilter::$method, handler, actions)
        }
    };
}

// impl_method!(auth_connect, CONNECT);
impl_method!(auth_delete, DELETE);
impl_method!(auth_get, GET);
// impl_method!(auth_head, HEAD);
// impl_method!(auth_options, OPTIONS);
// impl_method!(auth_patch, PATCH);
impl_method!(auth_post, POST);
impl_method!(auth_put, PUT);
// impl_method!(auth_trace, TRACE);
