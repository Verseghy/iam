use crate::{json::Json, shared::SharedTrait};
use axum::{extract::Path, Extension};
use iam_common::{error::Result, user::UserInfo};

pub async fn get_user<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Path(id): Path<String>,
) -> Result<Json<UserInfo>> {
    Ok(Json(iam_common::user::get_user(shared.db(), &id).await?))
}
