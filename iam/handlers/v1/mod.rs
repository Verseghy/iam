mod actions;
mod apps;
mod assign;
mod decision;
mod groups;
mod users;

use crate::{auth::permissions, shared::SharedTrait};
use axum::{handler::Handler, routing::post, Router};

pub fn routes<S: SharedTrait>() -> Router {
    Router::new()
        .nest("/actions", actions::routes::<S>())
        .nest("/users", users::routes::<S>())
        .nest("/groups", groups::routes::<S>())
        .nest("/apps", apps::routes::<S>())
        .route(
            "/assign",
            post(assign::assign::<S>.layer(permissions::<S>(&["iam.policy.assign"]))),
        )
        .route("/decision", post(decision::decision::<S>))
}
