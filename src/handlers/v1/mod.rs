mod actions;
mod users;

use axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(actions::routes())
        .nest("/users", users::routes())
}
