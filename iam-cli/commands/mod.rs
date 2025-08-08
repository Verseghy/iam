pub mod create_app;
pub mod migrate;
pub mod password_hash;
pub mod seed;
pub mod setup;

use clap::{crate_authors, crate_name, crate_version, Command};

pub fn commands() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(create_app::command())
        .subcommand(password_hash::command())
        .subcommand(seed::command())
        .subcommand(setup::command())
        .subcommand(migrate::command())
}

#[test]
fn verify_cli() {
    commands().debug_assert();
}
