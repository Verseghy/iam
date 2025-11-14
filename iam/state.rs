use iam_common::{Config, database, keys::KeyManager};
use sea_orm::DbConn;
use std::sync::Arc;

pub trait StateTrait: Clone + Send + Sync + 'static {
    type Db: sea_orm::ConnectionTrait + sea_orm::TransactionTrait;

    fn db(&self) -> &Self::Db;
    fn key_manager(&self) -> &KeyManager;
}

pub struct StateInner {
    pub db: DbConn,
    pub key_manager: KeyManager,
}

#[derive(Clone)]
pub struct State {
    inner: Arc<StateInner>,
}

impl StateTrait for State {
    type Db = DbConn;

    fn db(&self) -> &DbConn {
        &self.inner.db
    }

    fn key_manager(&self) -> &KeyManager {
        &self.inner.key_manager
    }
}

pub async fn create_state(config: Config) -> anyhow::Result<State> {
    Ok(State {
        inner: Arc::new(StateInner {
            db: database::connect(&config.database_url).await,
            key_manager: KeyManager::new(&config).await?,
        }),
    })
}

#[cfg(test)]
pub mod mock {
    #![allow(unused)]

    use super::*;
    use sea_orm::MockDatabase;

    pub struct MockStateInner {
        db: Option<DbConn>,
    }

    #[derive(Clone)]
    pub struct MockState {
        inner: Arc<MockStateInner>,
    }

    impl MockState {
        pub fn builder() -> MockStateInner {
            MockStateInner { db: None }
        }

        pub fn empty() -> Self {
            Self::builder().build()
        }
    }

    impl MockStateInner {
        pub fn db(mut self, db: MockDatabase) -> Self {
            self.db = Some(db.into_connection());
            self
        }

        pub fn build(mut self) -> MockState {
            MockState {
                inner: Arc::new(self),
            }
        }
    }

    impl StateTrait for MockState {
        type Db = DbConn;

        fn db(&self) -> &DbConn {
            self.inner.db.as_ref().expect("database not set")
        }

        fn key_manager(&self) -> &KeyManager {
            todo!()
        }
    }
}
