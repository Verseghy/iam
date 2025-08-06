mod actions;
mod get;

use crate::{auth::permissions, state::StateTrait};
use axum::{routing::get, Router};

pub fn routes<S: StateTrait>(state: S) -> Router<S> {
    Router::new()
        .route(
            "/",
            get(get::get_user::<S>).layer(permissions(state, &["iam.user.get"])),
        )
        .route("/actions", get(actions::get_actions::<S>))
}
