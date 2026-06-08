use tracing::info;

mod app;
mod handlers;
mod dto;
mod repository;

#[tokio::main]
async fn main() {
    // Init app
    dotenvy::dotenv().ok();
    let app_config = app::config::AppConfig::init();
    app::logger::init_logger(&app_config);

    // Init shared state (database connection pool, ...)
    let state = app::state::AppState::init(&app_config)
        .await
        .expect("Failed to initialize application state");

    // Apply pending database migrations.
    sqlx::migrate!("./migrations")
        .run(&state.db)
        .await
        .expect("Failed to run database migrations");
    info!("Database migrations applied");

    // Init TCP listener
    let listener = tokio::net::TcpListener::bind(app_config.get_http_addr())
        .await
        .expect("Failed to bind to address");
    info!("Listening on {}", listener.local_addr().unwrap());

    // Start the server
    let router = app::router::create_router(state);
    axum::serve(listener, router).await.unwrap();
}
