use iam_common::{database, keys::KeyManager};
use sea_orm::DbConn;
use std::sync::Arc;

pub trait SharedTrait: Clone + Send + Sync + 'static {
    type Db: sea_orm::ConnectionTrait + sea_orm::TransactionTrait;

    fn db(&self) -> &Self::Db;
    fn key_manager(&self) -> &KeyManager;
}

pub struct SharedInner {
    pub db: DbConn,
    pub key_manager: KeyManager,
}

#[derive(Clone)]
pub struct Shared {
    inner: Arc<SharedInner>,
}

impl SharedTrait for Shared {
    type Db = DbConn;

    fn db(&self) -> &DbConn {
        &self.inner.db
    }

    fn key_manager(&self) -> &KeyManager {
        &self.inner.key_manager
    }
}

pub async fn create_shared() -> Shared {
    Shared {
        inner: Arc::new(SharedInner {
            db: database::connect().await,
            key_manager: KeyManager::new(),
        }),
    }
}

#[cfg(test)]
pub mod mock {
    #![allow(unused)]

    use super::*;
    use sea_orm::MockDatabase;

    pub struct MockSharedInner {
        db: Option<DbConn>,
    }

    #[derive(Clone)]
    pub struct MockShared {
        inner: Arc<MockSharedInner>,
    }

    impl MockShared {
        pub fn builder() -> MockSharedInner {
            MockSharedInner { db: None }
        }

        pub fn empty() -> Self {
            Self::builder().build()
        }
    }

    impl MockSharedInner {
        pub fn db(mut self, db: MockDatabase) -> Self {
            self.db = Some(db.into_connection());
            self
        }

        pub fn build(mut self) -> MockShared {
            MockShared {
                inner: Arc::new(self),
            }
        }
    }

    impl SharedTrait for MockShared {
        type Db = DbConn;

        fn db(&self) -> &DbConn {
            self.inner.db.as_ref().expect("database not set")
        }

        fn key_manager(&self) -> &KeyManager {
            todo!()
        }
    }
}
