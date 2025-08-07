mod internal;
mod oauth2;
mod v1;
mod well_known;

use crate::state::StateTrait;
use axum::{routing::get, Router};

pub fn routes<S: StateTrait>(state: S) -> Router<S> {
    Router::new()
        .nest("/oauth2", oauth2::routes::<S>())
        .nest("/v1", v1::routes(state))
        .nest("/internal", internal::routes::<S>())
        .nest("/.well-known", well_known::routes::<S>())
        .route("/ready", get(|| async {}))
        .route("/live", get(|| async {}))
}
