use crate::shared::SharedTrait;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Extension};
use headers::{
    authorization::{Authorization, Bearer},
    HeaderMapExt,
};
use std::sync::Arc;

pub async fn get_claims<S: SharedTrait>(
    Extension(shared): Extension<S>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = request.into_parts();
    let token = parts.headers.typed_get::<Authorization<Bearer>>();
    let mut request = Request::from_parts(parts, body);

    if let Some(token) = token {
        if let Ok(claims) = shared.key_manager().jwt().get_claims(token.token()) {
            request.extensions_mut().insert(Arc::new(claims));
        }
    }

    Ok(next.run(request).await)
}
