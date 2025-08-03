mod token;

use crate::shared::SharedTrait;
use axum::{routing::post, Router};

pub fn routes<S: SharedTrait>() -> Router {
    Router::new().route("/token", post(token::token::<S>))
}
