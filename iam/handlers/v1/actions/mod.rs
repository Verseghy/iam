mod delete;
mod get;
mod gets;
mod post;
mod put;

use crate::{auth::permissions, state::StateTrait};
use axum::{
    handler::Handler,
    routing::{get, MethodRouter},
    Router,
};

pub fn routes<S: StateTrait>(state: S) -> Router<S> {
    Router::new()
        .route(
            "/{action_id}",
            get(get::get_action::<S>.layer(permissions(state.clone(), &["iam.action.get"]))),
        )
        .route(
            "/",
            MethodRouter::new()
                .get(
                    gets::list_actions::<S>.layer(permissions(state.clone(), &["iam.action.list"])),
                )
                .post(
                    post::update_action::<S>
                        .layer(permissions(state.clone(), &["iam.action.update"])),
                )
                .put(put::add_action::<S>.layer(permissions(state.clone(), &["iam.action.add"])))
                .delete(
                    delete::delete_action::<S>
                        .layer(permissions(state.clone(), &["iam.action.delete"])),
                ),
        )
}
