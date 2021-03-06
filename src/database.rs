use redis::aio::ConnectionManager;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::log::LevelFilter;

pub async fn connect() -> DatabaseConnection {
    tracing::info!("Trying to connect to database");

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let mut opts = ConnectOptions::new(url);
    opts.sqlx_logging_level(LevelFilter::Debug);

    let db = Database::connect(opts).await.unwrap();

    tracing::info!("Connected to database");

    db
}

pub async fn connect_redis() -> ConnectionManager {
    let url = std::env::var("REDIS_URL").expect("REDIS_URL not set");
    let client = redis::Client::open(url)
        .map_err(|err| format!("failed to connect to redis: {:?}", err))
        .unwrap();

    ConnectionManager::new(client)
        .await
        .map_err(|err| format!("failed to connect to redis: {:?}", err))
        .unwrap()
}
