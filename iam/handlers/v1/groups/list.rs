use crate::{json::Json, state::StateTrait};
use axum::extract::State;
use iam_common::error::Result;
use iam_entity::groups;
use sea_orm::{FromQueryResult, QuerySelect, entity::EntityTrait};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct Group {
    id: String,
    name: String,
}

pub async fn list_groups<S: StateTrait>(State(state): State<S>) -> Result<Json<Vec<Group>>> {
    let res = groups::Entity::find()
        .select_only()
        .column(groups::Column::Id)
        .column(groups::Column::Name)
        .into_model::<Group>()
        .all(state.db())
        .await?;

    Ok(Json(res))
}
