use crate::{
    json::{Json, ValidatedJson},
    shared::SharedTrait,
    utils::{Error, Result},
};
use axum::Extension;
use common::{
    password,
    token::{self, JwtTrait},
};
use entity::users;
use sea_orm::{
    entity::{ActiveModelTrait, ColumnTrait, EntityTrait},
    query::QueryFilter,
    ActiveValue,
};
use serde::{Deserialize, Serialize};
use std::default::Default;
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
    ValidatedJson(mut req): ValidatedJson<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    req.email = req.email.to_lowercase();

    let res = users::Entity::find()
        .filter(users::Column::Email.eq(req.email.clone()))
        .one(shared.db())
        .await?
        .ok_or_else(|| Error::bad_request("invalid email or password"))?;

    let (valid, rehash) =
        password::validate(&res.password, &req.password).map_err(Error::internal)?;

    if let Some(Ok(hash)) = rehash {
        let mut action: users::ActiveModel = res.clone().into();
        action.password = ActiveValue::Set(hash);

        action.update(shared.db()).await?;
    }

    if !valid {
        return Err(Error::bad_request("invalid email or password"));
    }

    let claims = token::Claims {
        subject: res.id.to_string(),
        ..Default::default()
    };

    let token = shared.jwt().encode(&claims).map_err(Error::internal)?;

    crate::audit!(action = "login", user = res.id.to_string(),);

    Ok(Json(LoginResponse { token }))
}
