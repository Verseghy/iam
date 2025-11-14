mod utils;

pub use sea_orm_migration::MigratorTrait;
use sea_orm_migration::{MigrationTrait, async_trait::async_trait};

mod m20220311_151913_create_users;
mod m20220311_152016_create_actions;
mod m20220416_053618_create_groups;
mod m20220416_054159_create_pivot_actions_groups;
mod m20220416_054659_create_pivot_actions_users;
mod m20220416_054675935_create_pivot_users_groups;
mod m20220416_060135_add_iam_actions;
mod m20220822_190837_remove_invite_action;
mod m20221007_103449_create_app;
mod m20221007_211858_apps_permissions;
mod m20230226_221912_apps_permission2;

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
            Box::new(m20220822_190837_remove_invite_action::Migration),
            Box::new(m20221007_103449_create_app::Migration),
            Box::new(m20221007_211858_apps_permissions::Migration),
            Box::new(m20230226_221912_apps_permission2::Migration),
        ]
    }
}
