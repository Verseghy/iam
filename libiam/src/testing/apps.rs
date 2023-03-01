use iam_common::Id;
use sea_orm::ConnectionTrait;

pub async fn create_app<D>(db: &D, name: &str) -> (Id, String)
where
    D: ConnectionTrait,
{
    iam_common::app::create(db, name)
        .await
        .expect("failed to create app")
}
