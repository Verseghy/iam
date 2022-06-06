pub use sea_orm_migration::MigratorTrait;
use sea_orm_migration::{async_trait::async_trait, MigrationTrait};

mod m20220311_151913_create_users;
mod m20220311_152016_create_actions;
mod m20220416_053618_create_groups;
mod m20220416_054159_create_pivot_actions_groups;
mod m20220416_054659_create_pivot_actions_users;
mod m20220416_054675935_create_pivot_users_groups;
mod m20220416_060135_add_iam_actions;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220311_151913_create_users::Migration),
            Box::new(m20220311_152016_create_actions::Migration),
            Box::new(m20220416_053618_create_groups::Migration),
            Box::new(m20220416_054159_create_pivot_actions_groups::Migration),
            Box::new(m20220416_054659_create_pivot_actions_users::Migration),
            Box::new(m20220416_054675935_create_pivot_users_groups::Migration),
            Box::new(m20220416_060135_add_iam_actions::Migration),
        ]
    }
}
