mod delete;
mod get;
mod gets;
mod post;
mod put;

use crate::{
    auth::routing::{auth_delete, auth_get, auth_post, auth_put},
    state::StateTrait,
};
use axum::Router;

#[rustfmt::skip]
pub fn routes<S: StateTrait>() -> Router<S> {
    Router::new()
        .route("/{action_id}", auth_get(get::get_action::<S>, &["iam.action.get"]))
        .route("/", auth_get(gets::list_actions::<S>, &["iam.action.list"]))
        .route("/", auth_post(post::update_action::<S>, &["iam.action.update"]))
        .route("/", auth_put(put::add_action::<S>, &["iam.action.add"]))
        .route("/", auth_delete(delete::delete_action::<S>, &["iam.action.delete"]))
}
