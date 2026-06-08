use axum::response::IntoResponse;



pub enum AppError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    InvalidInput(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound(message) => (axum::http::StatusCode::NOT_FOUND, message).into_response(),
            AppError::InternalServerError(message) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, message).into_response(),
            AppError::BadRequest(message) => (axum::http::StatusCode::BAD_REQUEST, message).into_response(),
            AppError::InvalidInput(message) => (axum::http::StatusCode::UNPROCESSABLE_ENTITY, message).into_response(),
        }
    }
}


pub struct JsonErrorResponse {
    pub request_id: String,
    pub method: String,
    pub path: String,
    pub error_code: AppError,
    pub message: Option<String>,
}

