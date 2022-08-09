use crate::{json::Json, shared::Shared, utils::Error};
use axum::{extract::Path, Extension};
use entity::actions;
use sea_orm::entity::EntityTrait;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetResponse {
    id: String,
    name: String,
    secure: bool,
}
pub async fn get_action(
    Extension(shared): Extension<Shared>,
    Path(id): Path<String>,
) -> Result<Json<GetResponse>, Error> {
    let res = actions::Entity::find_by_id(id)
        .one(&shared.db)
        .await?
        .ok_or_else(|| Error::not_found("action not found"))?;

    Ok(Json(GetResponse {
        id: res.id,
        name: res.name,
        secure: res.secure,
    }))
}
