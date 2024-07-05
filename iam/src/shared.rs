use iam_common::{database, token::Jwt};
use sea_orm::DbConn;
use std::sync::Arc;

pub trait SharedTrait: Clone + Send + Sync + 'static {
    type Db: sea_orm::ConnectionTrait + sea_orm::TransactionTrait;
    type Jwt: iam_common::token::JwtTrait;

    fn db(&self) -> &Self::Db;
    fn jwt(&self) -> &Self::Jwt;
}

pub struct SharedInner {
    pub db: DbConn,
    pub jwt: Jwt,
}

#[derive(Clone)]
pub struct Shared {
    inner: Arc<SharedInner>,
}

impl SharedTrait for Shared {
    type Db = DbConn;
    type Jwt = Jwt;

    fn db(&self) -> &DbConn {
        &self.inner.db
    }

    fn jwt(&self) -> &Jwt {
        &self.inner.jwt
    }
}

pub async fn create_shared() -> Shared {
    Shared {
        inner: Arc::new(SharedInner {
            db: database::connect().await,
            jwt: Jwt::from_env(),
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
        jwt: Option<Jwt>,
    }

    #[derive(Clone)]
    pub struct MockShared {
        inner: Arc<MockSharedInner>,
    }

    impl MockShared {
        pub fn builder() -> MockSharedInner {
            MockSharedInner {
                db: None,
                jwt: None,
            }
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

        pub fn jwt(mut self, jwt: Jwt) -> Self {
            self.jwt = Some(jwt);
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
        type Jwt = Jwt;

        fn db(&self) -> &DbConn {
            self.inner.db.as_ref().expect("database not set")
        }

        fn jwt(&self) -> &Jwt {
            self.inner.jwt.as_ref().expect("jwt not set")
        }
    }
}
