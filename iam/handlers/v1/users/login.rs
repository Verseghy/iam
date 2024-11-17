use crate::{
    json::{Json, ValidatedJson},
    shared::SharedTrait,
};
use axum::Extension;
use iam_common::{
    error::{self, Result},
    keys::jwt::Claims,
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

pub async fn login<S: SharedTrait>(
    Extension(shared): Extension<S>,
    ValidatedJson(req): ValidatedJson<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    let res = users::Entity::find()
        .filter(users::Column::Email.eq(req.email.clone()))
        .one(shared.db())
        .await?
        .ok_or(error::INVALID_EMAIL_OR_PASSWORD)?;

    let (valid, rehash) = password::validate(&res.password, &req.password)?;

    if let Some(Ok(hash)) = rehash {
        let mut action: users::ActiveModel = res.clone().into();
        action.password = ActiveValue::Set(hash);

        action.update(shared.db()).await?;
    }

    if !valid {
        return Err(error::INVALID_EMAIL_OR_PASSWORD);
    }

    crate::audit!(action = "login", user = res.id);

    let token = shared.key_manager().jwt().encode(&Claims::new(res.id));

    Ok(Json(LoginResponse { token }))
}
