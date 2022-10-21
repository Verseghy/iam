use crate::{
    json::Json,
    shared::SharedTrait,
    utils::{Error, Result},
};
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

pub async fn get_user<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Path(id): Path<String>,
) -> Result<Json<GetUserResponse>> {
    let res = users::Entity::find_by_id(id)
        .one(shared.db())
        .await?
        .ok_or_else(|| Error::not_found("user not found"))?;

    Ok(Json(GetUserResponse {
        id: res.id,
        name: res.name,
        email: res.email,
    }))
}
