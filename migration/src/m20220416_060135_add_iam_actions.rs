use common::create_id;
use entity::actions::{Column, Entity};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220416_060135_add_iam_actions"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Entity)
                    .columns(vec![Column::Id, Column::Name, Column::Secure])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.action.add".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.action.get".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.action.update".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.action.list".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.action.delete".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.group.add".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.group.get".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.group.list".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.group.delete".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.group.edit".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.user.add".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.user.get".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.user.list".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.user.update".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.user.invite".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
                        "iam.user.delete".into(),
                        true.into(),
                    ])
                    .values_panic(vec![
                        format!("ActionID-{}", create_id()).into(),
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
