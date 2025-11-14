use crate::{StateTrait, json::Json};
use axum::{
    Extension,
    extract::{Path, State},
};
use iam_common::{
    error::{self, Result},
    keys::jwt::Claims,
};
use iam_entity::{actions, users};
use sea_orm::{ColumnTrait, FromQueryResult, Related, query::QueryFilter};
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, FromQueryResult, Serialize)]
pub struct Action {
    id: String,
    name: String,
    secure: bool,
}

pub async fn get_actions<S: StateTrait>(
    State(state): State<S>,
    Path(id): Path<String>,
    Extension(claims): Extension<Arc<Claims>>,
) -> Result<Json<Vec<Action>>> {
    if id != claims.sub {
        return Err(error::NO_PERMISSION);
    }

    Ok(Json(
        <users::Entity as Related<actions::Entity>>::find_related()
            .filter(users::Column::Id.eq(id))
            .into_model::<Action>()
            .all(state.db())
            .await?,
    ))
}
