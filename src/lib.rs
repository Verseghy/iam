mod audit;
mod auth;
pub mod database;
mod handlers;
pub mod id;
pub mod password;
mod shared;
mod token;
mod util;
mod validate;

#[cfg(test)]
pub(crate) mod mock;

use axum::{
    error_handling::HandleErrorLayer,
    http::{header::AUTHORIZATION, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    BoxError, Router, Server,
};
use std::{
    error::Error,
    iter::once,
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};
use tower::{timeout::error::Elapsed, ServiceBuilder};
use tower_http::{
    cors::{Any, CorsLayer},
    ServiceBuilderExt,
};

pub async fn handle_error(err: BoxError) -> Response {
    if err.is::<Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "Request took too long").into_response()
    } else {
        tracing::error!("Internal server error: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

pub async fn shutdown_signals() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};

        signal(SignalKind::terminate())
            .expect("fail to set signal handler")
            .recv()
            .await
            .expect("fail SIGTERM")
    }

    #[cfg(not(unix))]
    {
        std::future::pending().await
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3001));
    let shared = shared::create_shared().await;

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let middlewares = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(10))
        .sensitive_headers(once(AUTHORIZATION))
        .trace_for_http()
        .compression()
        .decompression()
        .layer(cors_layer)
        .add_extension(shared)
        .layer(middleware::from_fn(auth::get_claims))
        .into_inner();

    let router = Router::new()
        .nest("/", handlers::routes())
        .layer(middlewares);

    tracing::info!("Listening on port {}", addr.port());

    Ok(Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signals())
        .await?)
}
