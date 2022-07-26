mod decision;

use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new().route("/decision", post(decision::decision))
}
