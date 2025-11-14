use crate::{
    Id,
    error::{self, Result},
};
use base64::prelude::*;
use iam_entity::apps;
use rand::distr::{Alphanumeric, SampleString};
use sea_orm::{ConnectionTrait, EntityTrait, Set};

pub fn parse_token(token: &str) -> Result<(String, String)> {
    let decoded = BASE64_STANDARD_NO_PAD.decode(token).map_err(|err| {
        tracing::warn!("token is not valid base64: {err}, {token}");
        error::APP_INVALID_TOKEN
    })?;

    let decoded_string = String::from_utf8(decoded).map_err(|_| {
        tracing::warn!("decoded token is not valid utf8");
        error::APP_INVALID_TOKEN
    })?;

    let (id, password) = decoded_string.split_once(':').ok_or_else(|| {
        tracing::warn!("token doesn't contain a ':'");
        error::APP_INVALID_TOKEN
    })?;

    Ok((id.to_owned(), password.to_owned()))
}

pub async fn create<D>(db: &D, name: &str) -> Result<(Id, String)>
where
    D: ConnectionTrait,
{
    let id = Id::new_app();

    let password = Alphanumeric.sample_string(&mut rand::rng(), 32);
    let hashed_password = crate::password::hash(&password)?;

    let app = apps::ActiveModel {
        id: Set(id.to_string()),
        name: Set(name.to_owned()),
        password: Set(hashed_password),
        ..Default::default()
    };

    apps::Entity::insert(app).exec(db).await?;

    let secret = BASE64_STANDARD_NO_PAD.encode(format!("{id}:{password}"));
    Ok((id, secret))
}
