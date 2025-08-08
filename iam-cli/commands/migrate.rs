use clap::{Arg, ArgAction, ArgMatches, Command};
use iam_migration::MigratorTrait;
use sea_orm::Database;

pub fn command() -> Command {
    Command::new("migrate")
        .about("Apply the pending migrations on the database")
        .arg(
            Arg::new("database")
                .long("database")
                .short('D')
                .env("DATABASE_URL")
                .action(ArgAction::Set)
                .required(true)
                .help("URL of the database. Defaults to the environment variable DATABASE_URL"),
        )
}

pub async fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let database_url = matches.get_one::<String>("database").unwrap();
    let db = Database::connect(database_url.as_str()).await?;

    iam_migration::Migrator::up(&db, None).await?;

    Ok(())
}
