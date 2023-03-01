use iam_common::Id;
use iam_entity::actions::{Column, Entity};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Entity)
                    .columns(vec![Column::Id, Column::Name, Column::Secure])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.action.add".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.action.get".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.action.update".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.action.list".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.action.delete".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.group.add".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.group.get".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.group.list".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.group.delete".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.group.edit".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.user.add".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.user.get".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.user.list".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.user.update".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.user.invite".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.user.delete".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        Id::new_action().to_string().into(),
                        "iam.policy.assign".into(),
                        true.into(),
                    ])
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await
    }
}
