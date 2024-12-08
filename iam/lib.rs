mod audit;
mod auth;
mod handlers;
mod json;
mod middlewares;
mod shared;
mod signal;
mod utils;

use axum::{
    error_handling::HandleErrorLayer,
    extract::Request,
    http::{header::AUTHORIZATION, StatusCode},
    middleware,
    response::{IntoResponse, Response},
    BoxError, Router, ServiceExt,
};
use middlewares::TraceRequestIdLayer;
use shared::{Shared, SharedTrait};
use signal::TerminateSignal;
use std::{
    error::Error,
    iter::once,
    net::{Ipv4Addr, SocketAddr},
    time::Duration,
};
use tokio::net::TcpListener;
use tower::{timeout::error::Elapsed, ServiceBuilder};
use tower_http::{cors::CorsLayer, normalize_path::NormalizePath, ServiceBuilderExt};
use utils::MakeUuidRequestId;

async fn handle_error(err: BoxError) -> Response {
    if err.is::<Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "Request took too long").into_response()
    } else {
        tracing::error!("Internal server error: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

fn middlewares<S: SharedTrait>(shared: S, router: Router) -> Router {
    let middlewares = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_error))
        .timeout(Duration::from_secs(10))
        .sensitive_headers(once(AUTHORIZATION))
        .set_x_request_id(MakeUuidRequestId)
        .trace_for_http()
        .layer(TraceRequestIdLayer)
        .compression()
        .decompression()
        .layer(CorsLayer::permissive())
        .add_extension(shared)
        .layer(middleware::from_fn(auth::get_claims::<S>))
        .propagate_x_request_id()
        .into_inner();

    router.layer(middlewares)
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3001));
    let shared = shared::create_shared().await;

    let app = middlewares(shared, handlers::routes::<Shared>());
    let app = NormalizePath::trim_trailing_slash(app);

    tracing::info!("Listening on port {}", addr.port());

    let listener = TcpListener::bind(&addr).await?;

    Ok(
        axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
            .with_graceful_shutdown(TerminateSignal::new())
            .await?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared::mock::MockShared;
    use axum::{body::Body, routing::get};
    use hyper::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn has_x_request_id() {
        let svc = middlewares(
            MockShared::empty(),
            Router::new().route("/", get(|| async {})),
        );

        let res = svc.oneshot(Request::new(Body::empty())).await.unwrap();

        let mut values = res.headers().get_all("x-request-id").iter();
        assert!(values.next().is_some());
        assert_eq!(values.next(), None);
    }

    #[tokio::test]
    async fn custom_x_request_id() {
        let svc = middlewares(
            MockShared::empty(),
            Router::new().route("/", get(|| async {})),
        );

        let res = svc
            .oneshot(
                Request::builder()
                    .header("x-request-id", "test-id")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        let mut values = res.headers().get_all("x-request-id").iter();
        assert_eq!(values.next().unwrap(), "test-id");
        assert_eq!(values.next(), None);
    }
}
