use crate::{json::Json, shared::Shared, utils::Error};
use axum::Extension;
use entity::users;
use sea_orm::{entity::EntityTrait, FromQueryResult, QuerySelect};
use serde::Serialize;

#[derive(Serialize, Debug, FromQueryResult)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

pub async fn list_users(Extension(shared): Extension<Shared>) -> Result<Json<Vec<User>>, Error> {
    let res = users::Entity::find()
        .select_only()
        .column(users::Column::Id)
        .column(users::Column::Name)
        .column(users::Column::Email)
        .into_model::<User>()
        .all(&shared.db)
        .await?;

    Ok(Json(res))
}
