mod delete;
mod gets;
mod id;
mod login;
mod post;
mod register;

use crate::{
    auth::routing::{auth_delete, auth_get, auth_post},
    state::StateTrait,
};
use axum::{Router, routing::post};

#[rustfmt::skip]
pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new()
        .route("/login", post(login::login::<S>))
        .route("/", auth_get(gets::list_users::<S>, &["iam.user.list"]))
        .route("/", auth_post(post::update_user::<S>, &["iam.user.update"]))
        .route("/", auth_delete(delete::delete_user::<S>, &["iam.user.delete"]))
        .route("/register", post(register::register::<S>))
        .nest("/{user_id}", id::routes::<S>())
}
