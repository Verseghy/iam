mod get;

use crate::{auth::permissions, shared::SharedTrait};
use axum::{routing::get, Router};

pub fn routes<S: SharedTrait>() -> Router {
    Router::new()
        .route(
            "/",
            get(get::get_user::<S>).layer(permissions![S, "iam.user.get"]),
        )
}
