use async_trait::async_trait;
use futures::{channel::mpsc, StreamExt};
use sea_orm::{
    ConnectOptions, ConnectionTrait, DbBackend, DbErr, ExecResult, QueryResult, Statement,
};
use tokio::{runtime, sync::oneshot};
use tracing::log::LevelFilter;

#[derive(Clone)]
pub struct Database {
    channel: mpsc::UnboundedSender<Message>,
}

impl Database {
    pub async fn connect(uri: &str) -> Self {
        let (tx, mut rx) = mpsc::unbounded();

        let uri = uri.to_owned();
        std::thread::spawn(move || {
            let rt = runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to create tokio runtime");

            rt.block_on(async move {
                let mut opts = ConnectOptions::new(uri);
                opts.sqlx_logging_level(LevelFilter::Debug);

                let conn = sea_orm::Database::connect(opts)
                    .await
                    .expect("failed to connect to database");

                while let Some(msg) = rx.next().await {
                    match msg {
                        Message::Execute(tx, stmt) => {
                            let res = conn.execute(stmt).await;
                            tx.send(res).unwrap();
                        }
                        Message::QueryOne(tx, stmt) => {
                            let res = conn.query_one(stmt).await;
                            tx.send(res).unwrap();
                        }
                        Message::QueryAll(tx, stmt) => {
                            let res = conn.query_all(stmt).await;
                            tx.send(res).unwrap();
                        }
                    }
                }
            });
        });

        Self { channel: tx }
    }
}

pub enum Message {
    Execute(oneshot::Sender<Result<ExecResult, DbErr>>, Statement),
    QueryOne(
        oneshot::Sender<Result<Option<QueryResult>, DbErr>>,
        Statement,
    ),
    QueryAll(oneshot::Sender<Result<Vec<QueryResult>, DbErr>>, Statement),
}

#[async_trait]
impl ConnectionTrait for Database {
    fn get_database_backend(&self) -> DbBackend {
        DbBackend::MySql
    }

    async fn execute(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
        let (tx, rx) = oneshot::channel();
        self.channel
            .unbounded_send(Message::Execute(tx, stmt))
            .unwrap();
        rx.await.unwrap()
    }

    async fn query_one(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
        let (tx, rx) = oneshot::channel();
        self.channel
            .unbounded_send(Message::QueryOne(tx, stmt))
            .unwrap();
        rx.await.unwrap()
    }

    async fn query_all(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
        let (tx, rx) = oneshot::channel();
        self.channel
            .unbounded_send(Message::QueryAll(tx, stmt))
            .unwrap();
        rx.await.unwrap()
    }
}
