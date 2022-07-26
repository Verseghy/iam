mod actions;
mod users;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .nest("/actions", actions::routes())
        .nest("/users", users::routes())
}
