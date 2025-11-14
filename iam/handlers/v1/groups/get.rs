use crate::{json::Json, state::StateTrait};
use axum::extract::{Path, State};
use iam_common::error::{self, Result};
use iam_entity::groups;
use sea_orm::{FromQueryResult, entity::EntityTrait};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct GetResponse {
    id: String,
    name: String,
}
pub async fn get_group<S: StateTrait>(
    State(state): State<S>,
    Path(id): Path<String>,
) -> Result<Json<GetResponse>> {
    let res = groups::Entity::find_by_id(id)
        .into_model::<GetResponse>()
        .one(state.db())
        .await?
        .ok_or(error::GROUP_NOT_FOUND)?;

    Ok(Json(res))
}
