use crate::{json::Json, shared::SharedTrait};
use axum::Extension;
use iam_common::error::Result;
use iam_entity::users;
use sea_orm::{entity::EntityTrait, FromQueryResult, QuerySelect};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

pub async fn list_users<S: SharedTrait>(
    Extension(shared): Extension<S>,
) -> Result<Json<Vec<User>>> {
    let res = users::Entity::find()
        .select_only()
        .column(users::Column::Id)
        .column(users::Column::Name)
        .column(users::Column::Email)
        .into_model::<User>()
        .all(shared.db())
        .await?;

    Ok(Json(res))
}
