use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

impl IntoResponse for HealthCheckResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
