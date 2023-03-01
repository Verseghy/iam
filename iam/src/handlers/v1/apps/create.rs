use crate::{
    json::{Json, ValidatedJson},
    SharedTrait,
};
use axum::{http::StatusCode, Extension};
use iam_common::{error::Result, Id};
use iam_entity::apps;
use rand::distributions::{Alphanumeric, DistString};
use sea_orm::{EntityTrait, Set};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct Request {
    #[validate(length(max = 256))]
    name: String,
}

#[derive(Serialize, Debug)]
pub struct Response {
    id: Id,
    secret: String,
}

pub async fn create_app<S: SharedTrait>(
    Extension(shared): Extension<S>,
    ValidatedJson(req): ValidatedJson<Request>,
) -> Result<(StatusCode, Json<Response>)> {
    let id = Id::new_app();
    let password = Alphanumeric.sample_string(&mut shared.rng().clone(), 32);

    let hashed_password = iam_common::password::hash(&password)?;

    let app = apps::ActiveModel {
        id: Set(id.to_string()),
        name: Set(req.name),
        password: Set(hashed_password),
        ..Default::default()
    };

    apps::Entity::insert(app).exec(shared.db()).await?;

    let secret = base64::encode(format!("{}:{}", id.to_string(), password));

    Ok((StatusCode::CREATED, Json(Response { id, secret })))
}
