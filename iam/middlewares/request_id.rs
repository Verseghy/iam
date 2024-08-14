use hyper::Request;
use pin_project_lite::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::Span;

#[derive(Debug, Clone, Copy)]
pub struct TraceRequestIdLayer;

impl<S> Layer<S> for TraceRequestIdLayer {
    type Service = TraceRequestId<S>;

    fn layer(&self, inner: S) -> Self::Service {
        TraceRequestId { inner }
    }
}

#[derive(Debug, Clone)]
pub struct TraceRequestId<S> {
    inner: S,
}

impl<S, B> Service<Request<B>> for TraceRequestId<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = TraceRequestIdResponseFuture<S::Future>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<B>) -> Self::Future {
        let span = request.headers().get("x-request-id").and_then(|id| {
            id.to_str()
                .ok()
                .map(|id| tracing::debug_span!("request_id", request_id = id))
        });

        TraceRequestIdResponseFuture {
            future: self.inner.call(request),
            span,
        }
    }
}

pin_project! {
    #[derive(Debug)]
    pub struct TraceRequestIdResponseFuture<F> {
        #[pin]
        future: F,
        span: Option<Span>,
    }
}

impl<F> Future for TraceRequestIdResponseFuture<F>
where
    F: Future,
{
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if let Some(span) = this.span {
            let _enter = span.enter();
            this.future.poll(cx)
        } else {
            this.future.poll(cx)
        }
    }
}
