mod create;

use crate::{auth::permissions, shared::SharedTrait};
use axum::{routing::post, Router};

pub fn routes<S: SharedTrait>() -> Router {
    Router::new().route(
        "/create",
        post(create::create_app::<S>).layer(permissions![S, "iam.app.create"]),
    )
}
