mod actions;
mod apps;
mod assign;
mod decision;
mod groups;
mod users;

use crate::{auth::routing::auth_post, state::StateTrait};
use axum::{routing::post, Router};

#[rustfmt::skip]
pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new()
        .nest("/actions", actions::routes::<S>())
        .nest("/users", users::routes::<S>())
        .nest("/groups", groups::routes::<S>())
        .nest("/apps", apps::routes::<S>())
        .route("/assign", auth_post(assign::assign::<S>, &["iam.policy.assign"]))
        .route("/decision", post(decision::decision::<S>))
}
