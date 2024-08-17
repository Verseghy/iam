mod delete;
mod get;
mod gets;
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
            "/:action_id",
            get(get::get_action::<S>.layer(permissions::<S>(&["iam.action.get"]))),
        )
        .route(
            "/",
            MethodRouter::new()
                .get(gets::list_actions::<S>.layer(permissions::<S>(&["iam.action.list"])))
                .post(post::update_action::<S>.layer(permissions::<S>(&["iam.action.update"])))
                .put(put::add_action::<S>.layer(permissions::<S>(&["iam.action.add"])))
                .delete(delete::delete_action::<S>.layer(permissions::<S>(&["iam.action.delete"]))),
        )
}
