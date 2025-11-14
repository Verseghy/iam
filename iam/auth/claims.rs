use crate::state::StateTrait;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use headers::{
    HeaderMapExt,
    authorization::{Authorization, Bearer},
};
use std::sync::Arc;

pub async fn get_claims<S: StateTrait>(
    State(state): State<S>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = request.into_parts();
    let token = parts.headers.typed_get::<Authorization<Bearer>>();
    let mut request = Request::from_parts(parts, body);

    if let Some(token) = token
        && let Ok(claims) = state.key_manager().jwt().get_claims(token.token())
    {
        request.extensions_mut().insert(Arc::new(claims));
    }

    Ok(next.run(request).await)
}
