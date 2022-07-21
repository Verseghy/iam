mod internal;
mod v1;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .nest("/v1", v1::routes())
        .nest("/internal", internal::routes())
}
