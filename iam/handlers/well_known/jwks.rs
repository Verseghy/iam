use crate::{json::Json, state::StateTrait};
use axum::extract::State;
use iam_common::keys::JwkSet;

pub async fn get<S: StateTrait>(State(state): State<S>) -> Json<JwkSet> {
    let set = state.key_manager().jwks();
    Json(set)
}
