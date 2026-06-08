use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{app::request_context::RequestContext};

#[derive(Debug, serde::Serialize)]
pub enum AppError {
    // Base errors
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    InvalidInput(String),

    // Specific errors
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

        // Pull request id / method / path from the task-local context that the
        // `capture_request_context` middleware set for this request.
        let ctx = RequestContext::current();

        let json_error_response = JsonErrorResponse {
            request_id: ctx.as_ref().map(|c| c.request_id.clone()).unwrap_or_default(),
            method: ctx.as_ref().map(|c| c.method.clone()).unwrap_or_default(),
            path: ctx.as_ref().map(|c| c.path.clone()).unwrap_or_default(),
            error_code: self,
            message,
        };

        (status_code, Json(json_error_response)).into_response()
    }
}

#[derive(Debug, serde::Serialize)]
pub struct JsonErrorResponse {
    pub request_id: String,
    pub method: String,
    pub path: String,
    pub error_code: AppError,
    pub message: Option<String>,
}

