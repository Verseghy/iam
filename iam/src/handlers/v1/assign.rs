use crate::{json::Json, utils::Error, SharedTrait};
use axum::{http::StatusCode, Extension};
use entity::{pivot_actions_users, pivot_users_groups};
use sea_orm::{EntityTrait, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    user: String,
    action: Option<String>,
    group: Option<String>,
}

pub async fn assign<S: SharedTrait>(
    Extension(shared): Extension<S>,
    Json(request): Json<Request>,
) -> Result<StatusCode, Error> {
    if request.action.is_none() && request.group.is_none() {
        return Err(Error::bad_request("no action or group"));
    }

    if request.action.is_some() && request.action.is_some() {
        return Err(Error::bad_request("action and group at the same time"));
    }

    if let Some(action_id) = request.action {
        let model = pivot_actions_users::ActiveModel {
            user_id: Set(request.user),
            action_id: Set(action_id),
        };

        pivot_actions_users::Entity::insert(model)
            .exec(shared.db())
            .await?;

        return Ok(StatusCode::NO_CONTENT);
    } else if let Some(group_id) = request.group {
        let model = pivot_users_groups::ActiveModel {
            user_id: Set(request.user),
            group_id: Set(group_id),
        };

        pivot_users_groups::Entity::insert(model)
            .exec(shared.db())
            .await?;

        return Ok(StatusCode::NO_CONTENT);
    }

    unreachable!()
}
