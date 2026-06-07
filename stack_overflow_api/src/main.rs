use axum::{Json, Router, routing::get};
use serde::Serialize;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::app::app_config::AppConfig;

mod app;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let app_config = AppConfig::init();

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind(app_config.get_http_addr())
        .await
        .expect("Failed to bind to address");
    info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn root() -> Json<Health> {
    Json(Health { status: "ok" })
}
