use crate::{json::Json, shared::Shared, utils::Error};
use axum::Extension;
use entity::actions;
use sea_orm::entity::EntityTrait;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GetsResponse {
    actions: Vec<Action>,
}

#[derive(Serialize, Debug)]
struct Action {
    id: String,
    name: String,
    secure: bool,
}

pub async fn list_actions(
    Extension(shared): Extension<Shared>,
) -> Result<Json<GetsResponse>, Error> {
    let res = actions::Entity::find().all(&shared.db).await?;

    Ok(Json(GetsResponse {
        actions: res
            .into_iter()
            .map(|x| Action {
                id: x.id,
                name: x.name,
                secure: x.secure,
            })
            .collect(),
    }))
}
