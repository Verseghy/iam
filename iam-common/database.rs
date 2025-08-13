use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log::LevelFilter;

pub async fn connect(db_url: &str) -> DatabaseConnection {
    tracing::info!("Trying to connect to database");

    let mut opts = ConnectOptions::new(db_url);
    opts.sqlx_logging_level(LevelFilter::Debug);

    let db = Database::connect(opts).await.unwrap();

    tracing::info!("Connected to database");

    db
}
