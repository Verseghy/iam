use crate::{json::Json, state::StateTrait};
use axum::extract::{Path, State};
use iam_common::{error::Result, user::UserInfo};

pub async fn get_user<S: StateTrait>(
    State(state): State<S>,
    Path(id): Path<String>,
) -> Result<Json<UserInfo>> {
    Ok(Json(iam_common::user::get_user(state.db(), &id).await?))
}
