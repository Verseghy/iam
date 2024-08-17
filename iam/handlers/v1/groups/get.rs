use crate::{json::Json, shared::SharedTrait};
use axum::{extract::Path, Extension};
use iam_common::error::{self, Result};
use iam_entity::groups;
use sea_orm::{entity::EntityTrait, FromQueryResult};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct GetResponse {
    id: String,
    name: String,
}
pub async fn get_group<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Path(id): Path<String>,
) -> Result<Json<GetResponse>> {
    let res = groups::Entity::find_by_id(id)
        .into_model::<GetResponse>()
        .one(shared.db())
        .await?
        .ok_or(error::GROUP_NOT_FOUND)?;

    Ok(Json(res))
}
