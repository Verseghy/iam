use crate::{json::Json, state::StateTrait};
use axum::extract::State;
use iam_common::{Id, error::Result};
use iam_entity::apps;
use sea_orm::EntityTrait;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct App {
    id: Id,
    name: String,
}

pub async fn list_apps<S: StateTrait>(State(state): State<S>) -> Result<Json<Vec<App>>> {
    let apps = apps::Entity::find()
        .all(state.db())
        .await?
        .into_iter()
        .map(|app| App {
            id: serde_json::from_value(json!(app.id)).unwrap(),
            name: app.name,
        })
        .collect();

    Ok(Json(apps))
}
