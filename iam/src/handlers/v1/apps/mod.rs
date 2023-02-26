mod create;
mod list;
mod login;

use crate::{auth::permissions, shared::SharedTrait};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes<S: SharedTrait>() -> Router {
    Router::new()
        .route(
            "/create",
            post(create::create_app::<S>).layer(permissions![S, "iam.apps.create"]),
        )
        .route("/login", post(login::login_app::<S>))
        .route(
            "/",
            get(list::list_apps::<S>).layer(permissions![S, "iam.apps.list"]),
        )
}
