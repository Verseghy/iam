mod internal;
mod oauth2;
mod v1;
mod well_known;

use crate::state::StateTrait;
use axum::{Router, routing::get};

#[rustfmt::skip]
pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new()
        .nest("/oauth2", oauth2::routes::<S>())
        .nest("/v1", v1::routes::<S>())
        .nest("/internal", internal::routes::<S>())
        .nest("/.well-known", well_known::routes::<S>())
        .route("/ready", get(|| async {}))
        .route("/live", get(|| async {}))
}
