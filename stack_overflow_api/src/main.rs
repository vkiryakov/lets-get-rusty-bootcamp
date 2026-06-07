use axum::{Json, Router, routing::get};
use serde::Serialize;
use tracing::info;

use crate::app::{app_config::AppConfig, app_logger};

mod app;

#[tokio::main]
async fn main() {
    // Init app
    dotenvy::dotenv().ok();

    let app_config = AppConfig::init();
    app_logger::init_logger(&app_config);

    let app = Router::new().route("/", get(root));

    // Init TCP listener
    let listener = tokio::net::TcpListener::bind(app_config.get_http_addr())
        .await
        .expect("Failed to bind to address");
    info!("Listening on {}", listener.local_addr().unwrap());

    // Start the server
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn root() -> Json<Health> {
    Json(Health { status: "ok" })
}
