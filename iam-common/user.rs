use crate::error::{self, Result};
use iam_entity::users;
use sea_orm::{ConnectionTrait, EntityTrait, FromQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromQueryResult, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub async fn get_user<D>(db: &D, id: &str) -> Result<UserInfo>
where
    D: ConnectionTrait,
{
    users::Entity::find_by_id(id.to_owned())
        .into_model()
        .one(db)
        .await?
        .ok_or(error::USER_NOT_FOUND)
}
