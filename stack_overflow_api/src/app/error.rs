use axum::{Json, http::StatusCode, response::IntoResponse};

#[derive(Debug, serde::Serialize)]
pub enum AppError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    InvalidInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status_code: StatusCode;
        let message: Option<String>;

        match &self {
            AppError::NotFound(msg) => {
                status_code = StatusCode::NOT_FOUND;
                message = Some(msg.clone());
            }
            AppError::InternalServerError(msg) => {
                status_code = StatusCode::INTERNAL_SERVER_ERROR;
                message = Some(msg.clone());
            }
            AppError::BadRequest(msg) => {
                status_code = StatusCode::BAD_REQUEST;
                message = Some(msg.clone());
            }
            AppError::InvalidInput(msg) => {
                status_code = StatusCode::UNPROCESSABLE_ENTITY;
                message = Some(msg.clone());
            }
        }

        let json_error_response = JsonErrorResponse {
            request_id: "some_request_id".to_string(), // You can generate a unique request ID here
            method: "GET".to_string(), // You can capture the actual HTTP method here
            path: "/some/path".to_string(), // You can capture the actual request path here
            error_code: self,
            message,
        };

        (status_code, Json(json_error_response)).into_response()
    }
}

#[derive(Debug, serde::Serialize)]
struct JsonErrorResponse {
    pub request_id: String,
    pub method: String,
    pub path: String,
    pub error_code: AppError,
    pub message: Option<String>,
}