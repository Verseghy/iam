mod delete;
mod get;
mod gets;
mod invite;
mod login;
mod post;
mod put;

use crate::auth::permissions;
use axum::{
    handler::Handler,
    routing::{get, post, MethodRouter},
    Router,
};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/invite",
            post(invite::invite).route_layer(permissions!["iam.user.invite"]),
        )
        .route("/login", post(login::login))
        .route(
            "/:user_id",
            get(get::get_user).layer(permissions!["iam.user.get"]),
        )
        .route(
            "/",
            MethodRouter::new()
                .get(gets::list_users.layer(permissions!["iam.user.list"]))
                .put(put::add_user.layer(permissions!["iam.user.add"]))
                .post(post::update_user.layer(permissions!["iam.user.update"]))
                .delete(delete::delete_user.layer(permissions!["iam.user.delete"])),
        )
}
