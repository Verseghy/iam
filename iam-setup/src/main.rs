use std::{collections::BTreeMap, env};

use anyhow::Context;
use k8s_openapi::api::core::v1::Secret;
use kube::{
    api::{ObjectMeta, PostParams},
    Api, Client,
};
use libiam::{
    testing::{self, actions::assign_action_to_user},
    Iam, User,
};
use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::OsRng,
};

async fn generate_mysql_password(client: Client) -> anyhow::Result<()> {
    const SECRET_NAME: &str = "mysql";
    const SECRET_KEY: &str = "MYSQL_ROOT_PASSWORD";

    let secrets: Api<Secret> = Api::default_namespaced(client);

    if secrets
        .get_opt(SECRET_NAME)
        .await
        .context("Failed to query secret")?
        .is_some()
    {
        println!("Mysql password already exists.");
        return Ok(());
    }

    let mysql_password = Alphanumeric.sample_string(&mut OsRng, 64);

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
                    map.insert(SECRET_KEY.to_owned(), mysql_password);
                    map
                }),
                ..Default::default()
            },
        )
        .await
        .context("Failed to create secret")?;

    Ok(())
}

async fn create_admin_user(client: Client) -> anyhow::Result<()> {
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

    let iam_url = env::var("IAM_URL").context("IAM_URL is not set")?;
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL is not set")?;

    let iam = Iam::new(&iam_url);
    let db = testing::Database::connect(&database_url).await;

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

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    generate_mysql_password(client.clone()).await?;
    create_admin_user(client).await?;

    Ok(())
}
