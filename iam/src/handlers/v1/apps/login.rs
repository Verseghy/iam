use crate::{json::Json, SharedTrait};
use axum::Extension;
use iam_common::{
    error::{self, Result},
    keys::jwt::Claims,
};
use iam_entity::apps;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Request {
    token: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    token: String,
}

pub async fn login_app<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(request): Json<Request>,
) -> Result<Json<Response>> {
    let (id, password) = iam_common::app::parse_token(&request.token)?;

    tracing::debug!(id, "app login");

    let res = apps::Entity::find_by_id(id.clone())
        .one(shared.db())
        .await?
        .ok_or(error::APP_INVALID_TOKEN)?;

    let (valid, _) = iam_common::password::validate(&res.password, &password)?;

    if !valid {
        return Err(error::APP_INVALID_TOKEN);
    }

    let token = shared.key_manager().jwt().encode(&Claims::new(id));

    Ok(Json(Response { token }))
}
