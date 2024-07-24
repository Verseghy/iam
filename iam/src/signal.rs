#[cfg(not(unix))]
use futures_util::future::BoxFuture;
#[cfg(unix)]
use smallvec::SmallVec;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
#[cfg(unix)]
use tokio::signal::unix::Signal;

pub struct TerminateSignal {
    #[cfg(unix)]
    signals: SmallVec<[Signal; 3]>,
    #[cfg(not(unix))]
    signals: BoxFuture<'static, std::io::Result<()>>,
}

impl TerminateSignal {
    pub fn new() -> Self {
        tracing::debug!("Registering signal listeners");

        #[cfg(unix)]
        {
            use tokio::signal::unix::{self, SignalKind};

            let signals = [
                SignalKind::interrupt(),
                SignalKind::terminate(),
                SignalKind::quit(),
            ];

            let signals = signals
                .into_iter()
                .filter_map(|signal| {
                    unix::signal(signal)
                        .inspect_err(|err| {
                            tracing::warn!(
                                "Failed to initialize signal listener: {signal:?}, error: {err:?}"
                            );
                        })
                        .ok()
                })
                .collect();

            TerminateSignal { signals }
        }

        #[cfg(not(unix))]
        {
            use tokio::signal::ctrl_c;

            TerminateSignal {
                signals: Box::pin(ctrl_c()),
            }
        }
    }
}

impl Future for TerminateSignal {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        #[cfg(unix)]
        {
            for signal in &mut self.signals {
                if signal.poll_recv(cx).is_ready() {
                    tracing::debug!("Got terminate signal");
                    return Poll::Ready(());
                }
            }
        }

        #[cfg(not(unix))]
        {
            if self.signals.as_mut().poll(cx).is_ready() {
                tracing::debug!("Got terminate signal");
                return Poll::Ready(());
            }
        }

        Poll::Pending
    }
}
