use iam_entity::{actions, pivot_actions_groups, pivot_actions_users};
use sea_orm::{ConnectionTrait, TransactionTrait, entity::prelude::*};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let txn = manager.get_connection().begin().await?;

        delete_action(&txn, "iam.user.invite").await?;
        delete_action(&txn, "iam.user.add").await?;

        txn.commit().await?;
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

async fn delete_action(txn: &impl ConnectionTrait, name: &str) -> Result<(), DbErr> {
    let action_id = actions::Entity::find_by_name(name)
        .one(txn)
        .await?
        .expect("no such action")
        .id;

    actions::Entity::delete_by_id(action_id.clone())
        .exec(txn)
        .await?;

    pivot_actions_groups::Entity::delete_many()
        .filter(pivot_actions_groups::Column::ActionId.eq(action_id.clone()))
        .exec(txn)
        .await?;

    pivot_actions_users::Entity::delete_many()
        .filter(pivot_actions_users::Column::ActionId.eq(action_id.clone()))
        .exec(txn)
        .await?;

    Ok(())
}
