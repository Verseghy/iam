use crate::{StateTrait, json::Json};
use axum::{extract::State, http::StatusCode};
use iam_common::error::{self, Result};
use iam_entity::{pivot_actions_users, pivot_users_groups};
use sea_orm::{EntityTrait, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    user: String,
    action: Option<String>,
    group: Option<String>,
}

pub async fn assign<S: StateTrait>(
    State(state): State<S>,
    Json(request): Json<Request>,
) -> Result<StatusCode> {
    if request.action.is_none() && request.group.is_none() {
        return Err(error::ASSIGN_NO_ACTION_OR_GROUP);
    }

    if request.action.is_some() && request.group.is_some() {
        return Err(error::ASSIGN_ACTION_AND_GROUP_SAME_TIME);
    }

    if let Some(action_id) = request.action {
        let model = pivot_actions_users::ActiveModel {
            user_id: Set(request.user),
            action_id: Set(action_id),
        };

        pivot_actions_users::Entity::insert(model)
            .exec(state.db())
            .await?;

        return Ok(StatusCode::NO_CONTENT);
    } else if let Some(group_id) = request.group {
        let model = pivot_users_groups::ActiveModel {
            user_id: Set(request.user),
            group_id: Set(group_id),
        };

        pivot_users_groups::Entity::insert(model)
            .exec(state.db())
            .await?;

        return Ok(StatusCode::NO_CONTENT);
    }

    unreachable!()
}
