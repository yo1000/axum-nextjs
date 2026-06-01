mod config_props;
mod application_context;
mod domain;
mod presentation;
mod infrastructure;
mod application;
mod security;
mod transaction;

use crate::application_context::ApplicationContext;
use crate::presentation::router::create_router;
use axum::serve;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    let props = Arc::new(config_props::load()?);
    let ctx = Arc::new(ApplicationContext::new(Arc::clone(&props)).await?);

    let router = create_router(ctx);
    let listener = TcpListener::bind(format!("0.0.0.0:{:?}", props.server.port)).await?;
    info!("Server started on http://0.0.0.0:{:?}", props.server.port);

    serve(listener, router).await?;
    Ok(())
}
