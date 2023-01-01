use std::sync::Arc;

use crate::{json::Json, utils::Error, SharedTrait};
use axum::{extract::Path, Extension};
use common::token::Claims;
use entity::{actions, users};
use sea_orm::{query::QueryFilter, ColumnTrait, FromQueryResult, Related};
use serde::Serialize;

#[derive(Debug, FromQueryResult, Serialize)]
pub struct Action {
    id: String,
    name: String,
    secure: bool,
}

pub async fn get_actions<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Path(id): Path<String>,
    Extension(claims): Extension<Arc<Claims>>,
) -> Result<Json<Vec<Action>>, Error> {
    if id != claims.subject {
        return Err(Error::forbidden("no permission"));
    }

    Ok(Json(
        <users::Entity as Related<actions::Entity>>::find_related()
            .filter(users::Column::Id.eq(id))
            .into_model::<Action>()
            .all(shared.db())
            .await?,
    ))
}
