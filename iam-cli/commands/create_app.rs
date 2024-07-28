use clap::{Arg, ArgAction, ArgMatches, Command};
use libiam::testing::apps::create_app;
use sea_orm::Database;

pub fn command() -> Command {
    Command::new("create-app")
        .about("Registers an app in the IAM and returns the secret")
        .arg(
            Arg::new("name")
                .long("name")
                .required(true)
                .help("The name of the app to be created"),
        )
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
    let name = matches.get_one::<String>("name").unwrap();
    let db_url = matches.get_one::<String>("database").unwrap();

    let database = Database::connect(db_url).await?;

    let (id, secret) = create_app(&database, name).await;

    println!("id: {id}\nsecret: {secret}");

    Ok(())
}
