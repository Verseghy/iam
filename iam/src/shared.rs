use crate::token::Jwt;
use common::database;
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor};
use rand::{rngs::StdRng, SeedableRng};
use redis::aio::ConnectionManager;
use sea_orm::DbConn;
use std::sync::Arc;

pub type SmtpTransport = AsyncSmtpTransport<Tokio1Executor>;

pub struct SharedInner {
    pub db: DbConn,
    pub redis: ConnectionManager,
    pub smtp: SmtpTransport,
    pub jwt: Jwt,
    pub rng: StdRng,
}

pub type Shared = Arc<SharedInner>;

pub async fn create_shared() -> Shared {
    Arc::new(SharedInner {
        db: database::connect().await,
        redis: database::connect_redis().await,
        smtp: create_smtp_transport(),
        jwt: Jwt::new(),
        rng: StdRng::from_entropy(),
    })
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
