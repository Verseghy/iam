use entity::pivot_actions_users::{Column, Entity};
use sea_schema::migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220416_054659_create_pivot_actions_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Entity)
                    .if_not_exists()
                    .col(ColumnDef::new(Column::ActionId).string())
                    .col(ColumnDef::new(Column::UserId).string())
                    .primary_key(Index::create().col(Column::ActionId).col(Column::UserId))
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
