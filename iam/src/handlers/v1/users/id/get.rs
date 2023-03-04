use crate::{json::Json, shared::SharedTrait};
use axum::{extract::Path, Extension};
use iam_common::{error::Result, user::UserInfo};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetUserResponse {
    id: String,
    name: String,
    email: String,
}

pub async fn get_user<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Path(id): Path<String>,
) -> Result<Json<UserInfo>> {
    Ok(Json(iam_common::user::get_user(shared.db(), &id).await?))
}
