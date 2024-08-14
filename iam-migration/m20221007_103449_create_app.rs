use chrono::NaiveDateTime;
use iam_entity::{
    apps::{Column, Entity},
    pivot_apps_actions, pivot_apps_groups,
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::Id).string().primary_key())
                    .col(ColumnDef::new(Column::Name).string().not_null())
                    .col(ColumnDef::new(Column::Password).string().not_null())
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP")
                            .extra("ON UPDATE CURRENT_TIMESTAMP"),
                    )
                    .col(
                        ColumnDef::new(Column::DeletedAt)
                            .date_time()
                            .default(None::<NaiveDateTime>),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(pivot_apps_actions::Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(pivot_apps_actions::Column::AppId).string())
                    .col(ColumnDef::new(pivot_apps_actions::Column::ActionId).string())
                    .primary_key(
                        Index::create()
                            .col(pivot_apps_actions::Column::AppId)
                            .col(pivot_apps_actions::Column::ActionId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(pivot_apps_groups::Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(pivot_apps_groups::Column::AppId).string())
                    .col(ColumnDef::new(pivot_apps_groups::Column::GroupId).string())
                    .primary_key(
                        Index::create()
                            .col(pivot_apps_groups::Column::AppId)
                            .col(pivot_apps_groups::Column::GroupId),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Entity).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(pivot_apps_actions::Entity).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(pivot_apps_groups::Entity).to_owned())
            .await?;

        Ok(())
    }
}
