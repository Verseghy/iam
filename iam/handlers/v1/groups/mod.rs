mod delete;
mod get;
mod list;
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
            "/{group_id}",
            get(get::get_group::<S>.layer(permissions(state.clone(), &["iam.group.get"]))),
        )
        .route(
            "/",
            MethodRouter::new()
                .get(list::list_groups::<S>.layer(permissions(state.clone(), &["iam.group.list"])))
                .post(
                    post::update_group::<S>
                        .layer(permissions(state.clone(), &["iam.group.update"])),
                )
                .put(put::add_group::<S>.layer(permissions(state.clone(), &["iam.group.add"])))
                .delete(delete::delete_group::<S>.layer(permissions(state, &["iam.group.delete"]))),
        )
}
