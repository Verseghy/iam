use crate::{
    json::{Json, ValidatedJson},
    utils::Error,
    SharedTrait,
};
use axum::{http::StatusCode, Extension};
use common::Id;
use entity::apps;
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
) -> Result<(StatusCode, Json<Response>), Error> {
    let id = Id::new_app();
    let password = Alphanumeric.sample_string(&mut shared.rng().clone(), 32);

    let hashed_password = common::password::hash(&password).map_err(Error::internal)?;

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
