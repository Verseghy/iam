mod delete;
mod get;
mod list;
mod post;
mod put;

use crate::{auth::permissions, shared::SharedTrait};
use axum::{
    handler::Handler,
    routing::{get, MethodRouter},
    Router,
};

pub fn routes<S: SharedTrait>() -> Router {
    Router::new()
        .route(
            "/:group_id",
            get(get::get_group::<S>.layer(permissions::<S>(&["iam.group.get"]))),
        )
        .route(
            "/",
            MethodRouter::new()
                .get(list::list_groups::<S>.layer(permissions::<S>(&["iam.group.list"])))
                .post(post::update_group::<S>.layer(permissions::<S>(&["iam.group.update"])))
                .put(put::add_group::<S>.layer(permissions::<S>(&["iam.group.add"])))
                .delete(delete::delete_group::<S>.layer(permissions::<S>(&["iam.group.delete"]))),
        )
}
