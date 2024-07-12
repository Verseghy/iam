mod internal;
mod v1;
mod well_known;

use crate::shared::SharedTrait;
use axum::Router;

pub fn routes<S: SharedTrait>() -> Router {
    Router::new()
        .nest("/v1", v1::routes::<S>())
        .nest("/internal", internal::routes::<S>())
        .nest("/.well-known", well_known::routes::<S>())
}
