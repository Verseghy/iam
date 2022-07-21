use crate::shared::Shared;
use hyper::{Body, Request};
use std::{
    sync::Arc,
    task::{Context, Poll},
};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct GetClaimsLayer;

impl<S> Layer<S> for GetClaimsLayer {
    type Service = GetClaims<S>;

    fn layer(&self, service: S) -> Self::Service {
        GetClaims { service }
    }
}

#[derive(Clone)]
pub struct GetClaims<S> {
    service: S,
}

impl<S> Service<Request<Body>> for GetClaims<S>
where
    S: Service<Request<Body>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let shared = req.extensions().get::<Shared>().expect("No Shared");

        if let Ok(claims) = crate::token::get_claims(req.headers(), &shared.jwt.decoding) {
            req.extensions_mut().insert(Arc::new(claims));
        }

        self.service.call(req)
    }
}
