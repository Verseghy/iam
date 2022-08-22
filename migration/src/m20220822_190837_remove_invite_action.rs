use entity::actions;
use sea_orm::sea_query::{DeleteStatement, Expr};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .exec_stmt(
                DeleteStatement::new()
                    .from_table(actions::Entity.into_table_ref())
                    .and_where(Expr::col(actions::Column::Name).eq("iam.user.invite"))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        todo!();
    }
}
