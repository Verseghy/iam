mod invite;
mod login;

use crate::auth::permissions;
use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/invite",
            post(invite::invite).route_layer(permissions!["iam.user.invite"]),
        )
        .route("/login", post(login::login))
}
