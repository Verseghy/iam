use crate::{
    error::{self, Result},
    Id,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use iam_entity::apps;
use rand::distributions::{Alphanumeric, DistString};
use sea_orm::{ConnectionTrait, EntityTrait, Set};

pub fn parse_token(token: &str) -> Result<(String, String)> {
    let decoded = STANDARD
        .decode(token)
        .map_err(|_| error::APP_INVALID_TOKEN)?;
    let decoded_string = String::from_utf8(decoded).map_err(|_| error::APP_INVALID_TOKEN)?;

    let (id, password) = decoded_string
        .split_once(':')
        .ok_or(error::APP_INVALID_TOKEN)?;

    Ok((id.to_owned(), password.to_owned()))
}

pub async fn create<D>(db: &D, name: &str) -> Result<(Id, String)>
where
    D: ConnectionTrait,
{
    let id = Id::new_app();

    let password = {
        let mut rng = rand::thread_rng();
        Alphanumeric.sample_string(&mut rng, 32)
    };

    let hashed_password = crate::password::hash(&password)?;

    let app = apps::ActiveModel {
        id: Set(id.to_string()),
        name: Set(name.to_owned()),
        password: Set(hashed_password),
        ..Default::default()
    };

    apps::Entity::insert(app).exec(db).await?;

    let secret = STANDARD.encode(format!("{}:{}", id.to_string(), password));
    Ok((id, secret))
}
