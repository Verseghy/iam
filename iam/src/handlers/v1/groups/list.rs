use crate::{json::Json, shared::SharedTrait};
use axum::Extension;
use iam_common::error::Result;
use iam_entity::groups;
use sea_orm::{entity::EntityTrait, FromQueryResult, QuerySelect};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct Group {
    id: String,
    name: String,
}

pub async fn list_groups<S: SharedTrait>(
    Extension(shared): Extension<S>,
) -> Result<Json<Vec<Group>>> {
    let res = groups::Entity::find()
        .select_only()
        .column(groups::Column::Id)
        .column(groups::Column::Name)
        .into_model::<Group>()
        .all(shared.db())
        .await?;

    Ok(Json(res))
}
