mod delete;
mod gets;
mod id;
mod login;
mod post;
mod register;

use crate::{auth::permissions, state::StateTrait};
use axum::{
    handler::Handler,
    routing::{post, MethodRouter},
    Router,
};

pub fn routes<S: StateTrait>(state: S) -> Router<S> {
    Router::new()
        .route("/login", post(login::login::<S>))
        .route(
            "/",
            MethodRouter::new()
                .get(gets::list_users::<S>.layer(permissions(state.clone(), &["iam.user.list"])))
                .post(
                    post::update_user::<S>.layer(permissions(state.clone(), &["iam.user.update"])),
                )
                .delete(
                    delete::delete_user::<S>
                        .layer(permissions(state.clone(), &["iam.user.delete"])),
                ),
        )
        .route("/register", post(register::register::<S>))
        .nest("/{user_id}", id::routes(state))
}
