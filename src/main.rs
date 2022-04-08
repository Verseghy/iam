use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use iam::{database, routes};
use lettre::{
    transport::smtp::{authentication::Credentials, AsyncSmtpTransport},
    Tokio1Executor,
};
use std::{io, net::SocketAddr};

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

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .init();

    let database = Data::new(database::connect().await);
    let redis = Data::new(database::connect_redis().await);
    let smtp_transport = Data::new(create_smtp_transport());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Listening on port {}", addr.port());
    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .app_data(redis.clone())
            .app_data(smtp_transport.clone())
            .configure(routes)
    })
    .bind(addr)?
    .run()
    .await
}
