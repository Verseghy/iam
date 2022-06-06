use chrono::NaiveDateTime;
use entity::groups::{Column, Entity};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220416_053618_create_groups"
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
                    .col(ColumnDef::new(Column::Id).string().primary_key())
                    .col(ColumnDef::new(Column::Name).string_len(64).not_null())
                    .col(
                        ColumnDef::new(Column::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into()),
                    )
                    .col(
                        ColumnDef::new(Column::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".into())
                            .extra("ON UPDATE CURRENT_TIMESTAMP".into()),
                    )
                    .col(
                        ColumnDef::new(Column::DeletedAt)
                            .date_time()
                            .default(None::<NaiveDateTime>),
                    )
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
