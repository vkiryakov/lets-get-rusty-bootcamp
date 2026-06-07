use axum::{routing::get, Json, Router};
use serde::Serialize;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let addr = std::env::var("APP_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());

    let app = Router::new().route("/", get(root));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn root() -> Json<Health> {
    Json(Health { status: "ok" })
}
