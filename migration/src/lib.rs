pub use sea_schema::migration::MigratorTrait;
use sea_schema::migration::{async_trait::async_trait, MigrationTrait};

mod m20220311_151913_create_users;
mod m20220311_152016_create_actions;

pub struct Migrator;

#[async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220311_151913_create_users::Migration),
            Box::new(m20220311_152016_create_actions::Migration),
        ]
    }
}
