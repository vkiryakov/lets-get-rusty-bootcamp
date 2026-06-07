use tracing::info;

use crate::app::{app_config::AppConfig, app_logger};

mod app;
mod handlers;
mod payload;
mod router;

#[tokio::main]
async fn main() {
    // Init app
    dotenvy::dotenv().ok();

    let app_config = AppConfig::init();
    app_logger::init_logger(&app_config);

    let app = router::create_router();
    // Init TCP listener
    let listener = tokio::net::TcpListener::bind(app_config.get_http_addr())
        .await
        .expect("Failed to bind to address");
    info!("Listening on {}", listener.local_addr().unwrap());

    // Start the server
    axum::serve(listener, app).await.unwrap();
}
