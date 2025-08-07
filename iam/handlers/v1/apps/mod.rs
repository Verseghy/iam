mod create;
mod list;
mod login;

use crate::{
    auth::routing::{auth_get, auth_post},
    state::StateTrait,
};
use axum::{routing::post, Router};

#[rustfmt::skip]
pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new()
        .route("/create", auth_post(create::create_app::<S>, &["iam.apps.create"]))
        .route("/login", post(login::login_app::<S>))
        .route("/", auth_get(list::list_apps::<S>, &["iam.apps.list"]))
}
