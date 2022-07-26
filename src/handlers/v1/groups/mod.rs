mod delete;
mod get;
mod list;
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
        .route(
            "/:group_id",
            get(get::get_group.layer(permissions!["iam.group.get"])),
        )
        .route(
            "/",
            MethodRouter::new()
                .get(list::list_groups.layer(permissions!["iam.group.list"]))
                .post(post::update_group.layer(permissions!["iam.group.update"]))
                .put(put::add_group.layer(permissions!["iam.group.add"]))
                .delete(delete::delete_group.layer(permissions!["iam.group.delete"])),
        )
}
