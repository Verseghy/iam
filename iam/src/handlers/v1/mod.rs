mod actions;
mod apps;
mod groups;
mod users;

use crate::shared::SharedTrait;
use axum::Router;

pub fn routes<S: SharedTrait>() -> Router {
    Router::new()
        .nest("/actions", actions::routes::<S>())
        .nest("/users", users::routes::<S>())
        .nest("/groups", groups::routes::<S>())
        .nest("/apps", apps::routes::<S>())
}
