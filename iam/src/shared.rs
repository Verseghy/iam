use crate::token::Jwt;
use common::database;
use lettre::{
    transport::smtp::{authentication::Credentials, Error as SmtpError},
    AsyncSmtpTransport, Tokio1Executor,
};
use rand::{rngs::StdRng, SeedableRng};
use redis::aio::ConnectionManager;
use sea_orm::DbConn;
use std::sync::Arc;

pub type SmtpTransport = AsyncSmtpTransport<Tokio1Executor>;

pub trait SharedTrait: Clone + Send + Sync + 'static {
    type Db: sea_orm::ConnectionTrait;
    type Redis: redis::AsyncCommands + Clone;
    type Smtp: lettre::AsyncTransport<Error = SmtpError> + Sync;
    type Jwt: crate::token::JwtTrait;
    type Rng: rand::Rng + Clone;

    fn db(&self) -> &Self::Db;
    fn redis(&self) -> &Self::Redis;
    fn smtp(&self) -> &Self::Smtp;
    fn jwt(&self) -> &Self::Jwt;
    fn rng(&self) -> &Self::Rng;
}

pub struct SharedInner {
    pub db: DbConn,
    pub redis: ConnectionManager,
    pub smtp: SmtpTransport,
    pub jwt: Jwt,
    pub rng: StdRng,
}

#[derive(Clone)]
pub struct Shared {
    inner: Arc<SharedInner>,
}

impl SharedTrait for Shared {
    type Db = DbConn;
    type Redis = ConnectionManager;
    type Smtp = SmtpTransport;
    type Jwt = Jwt;
    type Rng = StdRng;

    fn db(&self) -> &DbConn {
        &self.inner.db
    }

    fn redis(&self) -> &ConnectionManager {
        &self.inner.redis
    }

    fn smtp(&self) -> &SmtpTransport {
        &self.inner.smtp
    }

    fn jwt(&self) -> &Jwt {
        &self.inner.jwt
    }

    fn rng(&self) -> &StdRng {
        &self.inner.rng
    }
}

pub async fn create_shared() -> Shared {
    Shared {
        inner: Arc::new(SharedInner {
            db: database::connect().await,
            redis: database::connect_redis().await,
            smtp: create_smtp_transport(),
            jwt: Jwt::new(),
            rng: StdRng::from_entropy(),
        }),
    }
}

fn create_smtp_transport() -> SmtpTransport {
    AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
        &std::env::var("SMTP_HOST").expect("SMTP_HOST not set"),
    )
    .unwrap()
    .credentials(Credentials::new(
        std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME not set"),
        std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not set"),
    ))
    .build()
}
