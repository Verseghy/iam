mod token;

use crate::state::StateTrait;
use axum::{routing::post, Router};

pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new().route("/token", post(token::token::<S>))
}
