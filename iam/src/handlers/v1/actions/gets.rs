use crate::{json::Json, shared::SharedTrait};
use axum::Extension;
use iam_common::error::Result;
use iam_entity::actions;
use sea_orm::entity::EntityTrait;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Action {
    id: String,
    name: String,
    secure: bool,
}

pub async fn list_actions<S: SharedTrait>(
    Extension(shared): Extension<S>,
) -> Result<Json<Vec<Action>>> {
    let res = actions::Entity::find().all(shared.db()).await?;

    Ok(Json(
        res.into_iter()
            .map(|x| Action {
                id: x.id,
                name: x.name,
                secure: x.secure,
            })
            .collect(),
    ))
}
