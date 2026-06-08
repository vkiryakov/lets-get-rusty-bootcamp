use tracing::info;

mod app;
mod handlers;
mod dto;

#[tokio::main]
async fn main() {
    // Init app
    dotenvy::dotenv().ok();
    let app_config = app::config::AppConfig::init();
    app::logger::init_logger(&app_config);

    // Init TCP listener
    let listener = tokio::net::TcpListener::bind(app_config.get_http_addr())
        .await
        .expect("Failed to bind to address");
    info!("Listening on {}", listener.local_addr().unwrap());

    // Start the server
    let router = app::router::create_router();
    axum::serve(listener, router).await.unwrap();
}
