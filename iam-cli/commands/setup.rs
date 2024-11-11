use anyhow::Context;
use clap::{Arg, ArgAction, ArgMatches, Command};
use k8s_openapi::api::core::v1::Secret;
use kube::{
    api::{ObjectMeta, PostParams},
    Api, Client,
};
use libiam::{testing::actions::assign_action_to_user, Iam, User};
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::OsRng,
};
use sea_orm::Database;
use std::collections::BTreeMap;

pub fn command() -> Command {
    Command::new("setup")
        .about("Creates admin user")
        .arg(
            Arg::new("database")
                .long("database")
                .short('D')
                .env("DATABASE_URL")
                .action(ArgAction::Set)
                .required(true)
                .help("URL of the database. Defaults to the environment variable DATABASE_URL"),
        )
        .arg(
            Arg::new("iam")
                .long("iam")
                .short('I')
                .env("IAM_URL")
                .action(ArgAction::Set)
                .required(true)
                .help("URL of the IAM. Defaults to the environment variable IAM_URL"),
        )
}

pub async fn run(matches: &ArgMatches) -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    create_admin_user(matches, client).await?;

    Ok(())
}

async fn create_admin_user(matches: &ArgMatches, client: Client) -> anyhow::Result<()> {
    const SECRET_NAME: &str = "iam";
    const ADMIN_EMAIL: &str = "admin@admin.admin";

    let secrets: Api<Secret> = Api::default_namespaced(client);

    if secrets
        .get_opt(SECRET_NAME)
        .await
        .context("Failed to query iam secret")?
        .is_some()
    {
        println!("iam secret already exists.");
        return Ok(());
    }

    let iam_url = matches.get_one::<String>("iam").unwrap();
    let database_url = matches.get_one::<String>("database").unwrap();

    let iam = Iam::new(iam_url);
    let db = Database::connect(database_url.as_str()).await?;

    let admin_password = Alphanumeric.sample_string(&mut OsRng, 64);
    let user = User::register(&iam, "admin", ADMIN_EMAIL, &admin_password).await?;

    assign_action_to_user(&db, "iam.policy.assign", &user.id().to_string()).await;

    secrets
        .create(
            &PostParams::default(),
            &Secret {
                metadata: ObjectMeta {
                    name: Some(SECRET_NAME.to_owned()),
                    ..Default::default()
                },
                string_data: Some({
                    let mut map = BTreeMap::new();
                    map.insert("IAM_EMAIL".to_owned(), ADMIN_EMAIL.to_owned());
                    map.insert("IAM_PASSWORD".to_owned(), admin_password);
                    map
                }),
                ..Default::default()
            },
        )
        .await
        .context("Failed to create secret")?;

    Ok(())
}
