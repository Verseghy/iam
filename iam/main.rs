use dotenvy::dotenv;
use iam_common::Config;
use std::fs::File;
use tracing_subscriber::{filter, layer::SubscriberExt, prelude::*, util::SubscriberInitExt};

fn audit_filter(metadata: &tracing::Metadata<'_>) -> bool {
    if metadata.is_event() {
        return metadata.target().starts_with("audit");
    }
    metadata.name().starts_with("audit")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = Config::from_env()?;

    let stdout_log = tracing_subscriber::fmt::layer()
        .compact()
        .with_filter(filter::LevelFilter::DEBUG);

    let file = File::create("audit.json")?;
    let json_log = tracing_subscriber::fmt::layer()
        .json()
        .with_level(false)
        .with_span_list(false)
        .with_current_span(false)
        .with_target(false)
        .with_writer(file)
        .flatten_event(true)
        .with_filter(filter::filter_fn(audit_filter));

    tracing_subscriber::registry()
        .with(stdout_log)
        .with(json_log)
        .init();

    iam::run(config).await
}
