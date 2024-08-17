use crate::{json::Json, shared::SharedTrait};
use axum::Extension;
use iam_common::keys::JwkSet;

pub async fn get<S: SharedTrait>(Extension(shared): Extension<S>) -> Json<JwkSet> {
    let set = shared.key_manager().jwks();
    Json(set)
}
