use axum::routing::get;

use crate::payload::{health_check_response::HealthCheckResponse, ping_pong_respose::PingPongResponse};

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/v1/health", get(health_check))
        .route("/v1/ping", get(ping))
}

async fn health_check() -> HealthCheckResponse {
    HealthCheckResponse {
        status: "ok".to_string(),
    }
}

async fn ping() -> PingPongResponse {
    PingPongResponse {
        message: "pong".to_string(),
    }
}

    