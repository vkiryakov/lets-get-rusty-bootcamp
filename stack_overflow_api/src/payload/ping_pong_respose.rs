use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct PingPongResponse {
    pub message: String,
}

impl IntoResponse for PingPongResponse {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

