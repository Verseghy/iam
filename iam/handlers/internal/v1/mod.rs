mod decision;

use crate::state::StateTrait;
use axum::{routing::post, Router};

pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new().route("/decision", post(decision::decision::<S>))
}
