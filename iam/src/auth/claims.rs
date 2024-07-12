use crate::shared::SharedTrait;
use axum::{
    extract::{FromRequestParts, TypedHeader},
    headers::authorization::{Authorization, Bearer},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use std::sync::Arc;

pub async fn get_claims<S: SharedTrait, B>(
    Extension(shared): Extension<S>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode>
where
    B: Send,
{
    let (mut parts, body) = request.into_parts();
    let token = TypedHeader::<Authorization<Bearer>>::from_request_parts(&mut parts, &()).await;
    let mut request = Request::from_parts(parts, body);

    if let Ok(token) = token {
        if let Ok(claims) = shared.key_manager().jwt().get_claims(token.token()) {
            request.extensions_mut().insert(Arc::new(claims));
        }
    }

    Ok(next.run(request).await)
}
