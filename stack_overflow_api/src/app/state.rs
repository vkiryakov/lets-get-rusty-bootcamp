use std::time::Duration;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

use crate::app::config::AppConfig;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {

    // Init app state
    #[tracing::instrument(name = "Initializing application state", skip(config))]
    pub async fn init(config: &AppConfig) -> anyhow::Result<Self> {
        // Initialize the database connection pool
        let db_config = config.get_database();
        let db = PgPoolOptions::new()
            .max_connections(db_config.max_connections)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&db_config.url)
            .await?;
        info!("Database connection pool initialized with max {} connections", db_config.max_connections);

        Ok(Self { db })
    }
}
