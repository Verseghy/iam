mod create;
mod list;
mod login;

use crate::{auth::permissions, state::StateTrait};
use axum::{
    routing::{get, post},
    Router,
};

pub fn routes<S: StateTrait>(state: S) -> Router<S> {
    Router::new()
        .route(
            "/create",
            post(create::create_app::<S>).layer(permissions(state.clone(), &["iam.apps.create"])),
        )
        .route("/login", post(login::login_app::<S>))
        .route(
            "/",
            get(list::list_apps::<S>).layer(permissions(state, &["iam.apps.list"])),
        )
}
