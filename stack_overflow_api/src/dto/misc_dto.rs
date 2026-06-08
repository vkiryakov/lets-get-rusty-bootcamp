use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

impl IntoResponse for HealthCheckResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingPongResponse {
    pub message: String,
}

impl IntoResponse for PingPongResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
