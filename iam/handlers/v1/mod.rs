mod actions;
mod apps;
mod assign;
mod decision;
mod groups;
mod users;

use crate::{auth::permissions, state::StateTrait};
use axum::{handler::Handler, routing::post, Router};

pub fn routes<S: StateTrait>(state: S) -> Router<S> {
    Router::new()
        .nest("/actions", actions::routes(state.clone()))
        .nest("/users", users::routes(state.clone()))
        .nest("/groups", groups::routes(state.clone()))
        .nest("/apps", apps::routes(state.clone()))
        .route(
            "/assign",
            post(assign::assign::<S>.layer(permissions(state, &["iam.policy.assign"]))),
        )
        .route("/decision", post(decision::decision::<S>))
}
