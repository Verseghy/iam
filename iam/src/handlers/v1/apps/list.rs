use crate::{json::Json, shared::SharedTrait};
use axum::Extension;
use common::{error::Result, Id};
use entity::apps;
use sea_orm::EntityTrait;
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct App {
    id: Id,
    name: String,
}

pub async fn list_apps<S: SharedTrait>(Extension(shared): Extension<S>) -> Result<Json<Vec<App>>> {
    let apps = apps::Entity::find()
        .all(shared.db())
        .await?
        .into_iter()
        .map(|app| App {
            id: serde_json::from_value(json!(app.id)).unwrap(),
            name: app.name,
        })
        .collect();

    Ok(Json(apps))
}
