use sea_orm_migration::{prelude::*, sea_orm::TransactionTrait};

use crate::utils::{add_action, delete_action};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;

        add_action(&txn, "iam.apps.get", true).await?;
        add_action(&txn, "iam.apps.list", true).await?;
        add_action(&txn, "iam.apps.update", true).await?;
        add_action(&txn, "iam.apps.delete", true).await?;

        txn.commit().await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;

        delete_action(&txn, "iam.apps.get").await?;
        delete_action(&txn, "iam.apps.list").await?;
        delete_action(&txn, "iam.apps.update").await?;
        delete_action(&txn, "iam.apps.delete").await?;

        txn.commit().await
    }
}
