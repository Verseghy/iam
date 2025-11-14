use crate::{json::Json, state::StateTrait};
use axum::extract::State;
use iam_common::error::Result;
use iam_entity::users;
use sea_orm::{FromQueryResult, QuerySelect, entity::EntityTrait};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

pub async fn list_users<S: StateTrait>(State(state): State<S>) -> Result<Json<Vec<User>>> {
    let res = users::Entity::find()
        .select_only()
        .column(users::Column::Id)
        .column(users::Column::Name)
        .column(users::Column::Email)
        .into_model::<User>()
        .all(state.db())
        .await?;

    Ok(Json(res))
}
