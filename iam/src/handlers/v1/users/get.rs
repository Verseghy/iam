use crate::{json::Json, shared::Shared, utils::Error};
use axum::{extract::Path, Extension};
use entity::users;
use sea_orm::entity::EntityTrait;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetUserResponse {
    id: String,
    name: String,
    email: String,
}

pub async fn get_user(
    Extension(shared): Extension<Shared>,
    Path(id): Path<String>,
) -> Result<Json<GetUserResponse>, Error> {
    let res = users::Entity::find_by_id(id)
        .one(&shared.db)
        .await?
        .ok_or_else(|| Error::not_found("user not found"))?;

    Ok(Json(GetUserResponse {
        id: res.id,
        name: res.name,
        email: res.email,
    }))
}
