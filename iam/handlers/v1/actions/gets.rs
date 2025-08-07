use crate::{json::Json, state::StateTrait};
use axum::extract::State;
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

pub async fn list_actions<S: StateTrait>(State(state): State<S>) -> Result<Json<Vec<Action>>> {
    let res = actions::Entity::find().all(state.db()).await?;

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
