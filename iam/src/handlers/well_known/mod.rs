mod jwks;

use crate::shared::SharedTrait;
use axum::{routing::get, Router};

pub fn routes<S: SharedTrait>() -> Router {
    Router::new().route("/jwks.json", get(jwks::get::<S>))
}
