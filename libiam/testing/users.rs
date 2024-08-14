pub use iam_common::user::UserInfo;
use sea_orm::ConnectionTrait;

pub async fn get_user<D>(db: &D, id: &str) -> UserInfo
where
    D: ConnectionTrait,
{
    iam_common::user::get_user(db, id)
        .await
        .expect("failed to get user")
}
