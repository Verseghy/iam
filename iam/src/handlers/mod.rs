mod internal;
mod v1;

use crate::shared::SharedTrait;
use axum::Router;

pub fn routes<S: SharedTrait>() -> Router {
    Router::new()
        .nest("/v1", v1::routes::<S>())
        .nest("/internal", internal::routes::<S>())
}
