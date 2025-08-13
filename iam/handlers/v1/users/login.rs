use crate::{
    json::{Json, ValidatedJson},
    state::StateTrait,
};
use axum::extract::State;
use iam_common::{
    error::{self, Result},
    password,
};
use iam_entity::users;
use sea_orm::{
    entity::{ActiveModelTrait, ColumnTrait, EntityTrait},
    query::QueryFilter,
    ActiveValue,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct LoginRequest {
    #[validate(email, length(max = 256))]
    email: String,
    #[validate(length(max = 256))]
    password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    token: String,
}

pub async fn login<S: StateTrait>(
    State(state): State<S>,
    ValidatedJson(req): ValidatedJson<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let res = users::Entity::find()
        .filter(users::Column::Email.eq(req.email.clone()))
        .one(state.db())
        .await?;

    let Some(res) = res else {
        // Still compute hash if the user is not in the database to avoid timing attacks
        _ = password::hash(&req.password);
        return Err(error::INVALID_EMAIL_OR_PASSWORD);
    };

    let (valid, rehash) = password::validate(&res.password, &req.password)?;

    if let Some(Ok(hash)) = rehash {
        let mut action: users::ActiveModel = res.clone().into();
        action.password = ActiveValue::Set(hash);

        action.update(state.db()).await?;
    }

    if !valid {
        return Err(error::INVALID_EMAIL_OR_PASSWORD);
    }

    crate::audit!(action = "login", user = res.id);

    let token = state.key_manager().jwt().encode(&res.id);

    Ok(Json(LoginResponse { token }))
}
