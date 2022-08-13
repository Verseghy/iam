use crate::{shared::SharedTrait, token::JwtTrait};
use axum::{
    extract::{RequestParts, TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

pub async fn get_claims<S: SharedTrait, B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode>
where
    B: Send,
{
    let mut request_parts = RequestParts::new(request);
    let token = request_parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await;
    let shared = request_parts.extensions().get::<S>().expect("No Shared");

    if let Ok(token) = token {
        if let Ok(claims) = shared.jwt().get_claims(token.token()) {
            request_parts.extensions_mut().insert(Arc::new(claims));
        }
    }

    let request = request_parts.try_into_request().expect("body extracted");
    Ok(next.run(request).await)
}
