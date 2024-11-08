use iam_common::Id;
use iam_entity::actions::{self, Entity};
use sea_orm::{ActiveModelTrait, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let actions = [
            ("iam.action.add", true),
            ("iam.action.get", true),
            ("iam.action.update", true),
            ("iam.action.list", true),
            ("iam.action.delete", true),
            ("iam.group.add", true),
            ("iam.group.get", true),
            ("iam.group.list", true),
            ("iam.group.delete", true),
            ("iam.group.edit", true),
            ("iam.user.add", true),
            ("iam.user.get", true),
            ("iam.user.list", true),
            ("iam.user.update", true),
            ("iam.user.invite", true),
            ("iam.user.delete", true),
            ("iam.policy.assign", true),
        ];

        for (name, secure) in actions {
            let model = actions::ActiveModel {
                id: Set(Id::new_action().to_string()),
                name: Set(name.to_string()),
                secure: Set(secure),
                ..Default::default()
            };

            model.insert(manager.get_connection()).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
