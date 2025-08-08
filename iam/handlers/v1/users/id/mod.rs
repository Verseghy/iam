mod actions;
mod get;

use crate::{auth::routing::auth_get, state::StateTrait};
use axum::{routing::get, Router};

#[rustfmt::skip]
pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new()
        .route("/", auth_get(get::get_user::<S>, &["iam.user.get"]))
        .route("/actions", get(actions::get_actions::<S>))
}
