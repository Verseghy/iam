use std::sync::Arc;

use crate::{json::Json, SharedTrait};
use axum::{extract::Path, Extension};
use iam_common::{
    error::{self, Result},
    token::Claims,
};
use iam_entity::{actions, users};
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
) -> Result<Json<Vec<Action>>> {
    if id != claims.subject {
        return Err(error::NO_PERMISSION);
    }

    Ok(Json(
        <users::Entity as Related<actions::Entity>>::find_related()
            .filter(users::Column::Id.eq(id))
            .into_model::<Action>()
            .all(shared.db())
            .await?,
    ))
}
