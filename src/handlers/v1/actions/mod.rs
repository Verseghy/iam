mod delete;
mod get;
mod gets;
mod post;
mod put;

use crate::auth::permissions;
use axum::{
    handler::Handler,
    routing::{get, MethodRouter},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .nest("/action", {
            Router::new()
                .route(
                    "/:action_id",
                    get(get::get_action.layer(permissions!["iam.action.get"])),
                )
                .route(
                    "/",
                    MethodRouter::new()
                        .post(post::update_action.layer(permissions!["iam.action.update"]))
                        .put(put::add_action.layer(permissions!["iam.action.add"]))
                        .delete(delete::delete_action.layer(permissions!["iam.action.delete"])),
                )
        })
        .nest("/actions", {
            Router::new().route(
                "/",
                get(gets::list_actions).route_layer(permissions!["iam.action.list"]),
            )
        })
}
