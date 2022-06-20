mod auth;
mod database;
mod handlers;
pub mod id;
mod password;
mod token;

#[cfg(test)]
pub(crate) mod mock;

use actix_cors::Cors;
use actix_web::{web::Data, App, HttpServer};
use handlers::routes;
use lettre::{
    transport::smtp::{authentication::Credentials, AsyncSmtpTransport},
    Tokio1Executor,
};
use rand::{rngs::SmallRng, SeedableRng};
use std::{io, net::SocketAddr};

pub async fn run() -> io::Result<()> {
    let database = Data::new(database::connect().await);
    let redis = Data::new(database::connect_redis().await);
    let smtp_transport = Data::new(create_smtp_transport());
    let jwt_private = Data::new(token::create_encoding_key());
    let jwt_public = Data::new(token::create_decoding_key());
    let rng = Data::new(SmallRng::from_entropy());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Listening on port {}", addr.port());
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .send_wildcard();

        App::new()
            .wrap(cors)
            .app_data(database.clone())
            .app_data(redis.clone())
            .app_data(smtp_transport.clone())
            .app_data(jwt_private.clone())
            .app_data(jwt_public.clone())
            .app_data(rng.clone())
            .configure(routes)
    })
    .bind(addr)?
    .run()
    .await
}

fn create_smtp_transport() -> AsyncSmtpTransport<Tokio1Executor> {
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
