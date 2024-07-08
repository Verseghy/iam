use async_trait::async_trait;
use sea_orm::{
    ConnectOptions, ConnectionTrait, DatabaseConnection, DbBackend, DbErr, ExecResult, QueryResult,
    Statement,
};
use std::sync::Arc;
use tokio::runtime::{self, Runtime};
use tracing::log::LevelFilter;

#[derive(Clone)]
pub struct Database {
    runtime: Arc<Runtime>,
    conn: Arc<DatabaseConnection>,
}

impl Database {
    pub async fn connect(uri: &str) -> Self {
        let runtime = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let conn = {
            let mut opts = ConnectOptions::new(uri.to_owned());
            opts.sqlx_logging_level(LevelFilter::Debug);

            sea_orm::Database::connect(opts)
                .await
                .expect("failed to connect to database")
        };

        Self {
            runtime: Arc::new(runtime),
            conn: Arc::new(conn),
        }
    }
}

#[async_trait]
impl ConnectionTrait for Database {
    fn get_database_backend(&self) -> DbBackend {
        self.conn.get_database_backend()
    }

    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        let this: &'static Self = unsafe { std::mem::transmute(self) };
        self.runtime.spawn(this.conn.execute(stmt)).await.unwrap()
    }

    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        let this: &'static Self = unsafe { std::mem::transmute(self) };
        self.runtime.spawn(this.conn.query_one(stmt)).await.unwrap()
    }

    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        let this: &'static Self = unsafe { std::mem::transmute(self) };
        self.runtime.spawn(this.conn.query_all(stmt)).await.unwrap()
    }
}
