use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use iam::{database, routes};
use std::{io, net::SocketAddr};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().unwrap();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .init();

    let database = Data::new(database::connect().await);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::info!("Listening on port {}", addr.port());
    HttpServer::new(move || App::new().app_data(database.clone()).configure(routes))
        .bind(addr)?
        .run()
        .await
}
