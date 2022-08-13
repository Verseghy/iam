mod delete;
mod get;
mod gets;
mod invite;
mod login;
mod post;
mod put;

use crate::{auth::permissions, shared::SharedTrait};
use axum::{
    handler::Handler,
    routing::{get, post, MethodRouter},
    Router,
};

pub fn routes<S: SharedTrait>() -> Router {
    Router::new()
        .route(
            "/invite",
            post(invite::invite::<S>).route_layer(permissions![S, "iam.user.invite"]),
        )
        .route("/login", post(login::login::<S>))
        .route(
            "/:user_id",
            get(get::get_user::<S>).layer(permissions![S, "iam.user.get"]),
        )
        .route(
            "/",
            MethodRouter::new()
                .get(gets::list_users::<S>.layer(permissions![S, "iam.user.list"]))
                .put(put::add_user::<S>.layer(permissions![S, "iam.user.add"]))
                .post(post::update_user::<S>.layer(permissions![S, "iam.user.update"]))
                .delete(delete::delete_user::<S>.layer(permissions![S, "iam.user.delete"])),
        )
}
