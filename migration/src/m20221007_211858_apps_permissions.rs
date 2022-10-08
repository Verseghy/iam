use sea_orm_migration::prelude::*;
use crate::utils::{add_action, delete_action};
use sea_orm::TransactionTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;

        add_action(&txn, "iam.apps.create", true).await?;

        txn.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;

        delete_action(&txn, "iam.apps.create").await?;

        txn.commit().await
    }
}
