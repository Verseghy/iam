use crate::{json::Json, shared::Shared, utils::Error};
use axum::Extension;
use entity::groups;
use sea_orm::{entity::EntityTrait, FromQueryResult, QuerySelect};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct Group {
    id: String,
    name: String,
}

pub async fn list_groups(Extension(shared): Extension<Shared>) -> Result<Json<Vec<Group>>, Error> {
    let res = groups::Entity::find()
        .select_only()
        .column(groups::Column::Id)
        .column(groups::Column::Name)
        .into_model::<Group>()
        .all(&shared.db)
        .await?;

    Ok(Json(res))
}
