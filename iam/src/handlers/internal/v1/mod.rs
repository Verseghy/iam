mod decision;

use crate::shared::SharedTrait;
use axum::{routing::post, Router};

pub fn routes<S: SharedTrait>() -> Router {
    Router::new().route("/decision", post(decision::decision::<S>))
}
