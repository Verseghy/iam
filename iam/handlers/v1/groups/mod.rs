mod delete;
mod get;
mod list;
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
        .route("/{group_id}", auth_get(get::get_group::<S>, &["iam.group.get"]))
        .route("/", auth_get(list::list_groups::<S>, &["iam.group.list"]))
        .route("/", auth_post(post::update_group::<S>, &["iam.group.update"]))
        .route("/", auth_put(put::add_group::<S>, &["iam.group.add"]))
        .route("/", auth_delete(delete::delete_group::<S>, &["iam.group.delete"]))
}
