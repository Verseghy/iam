mod jwks;

use crate::state::StateTrait;
use axum::{routing::get, Router};

pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new().route("/jwks.json", get(jwks::get::<S>))
}
