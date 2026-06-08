use axum::response::IntoResponse;

#[derive(Debug, serde::Serialize)]
pub struct JsonErrorResponse {
    pub request_id: String,
    pub method: String,
    pub path: String,
    pub error_code: AppError,
    pub message: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct DataResponse<T> {
    pub request_id: String,
    pub data: T,
}

impl<T> IntoResponse for DataResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }    
}

#[derive(Debug, serde::Serialize)]
pub struct ListResponse<T> {
    pub request_id: String,
    pub total_count: usize,
    pub limit: usize,
    pub offset: usize,
    pub items: Vec<T>,
}

impl<T> IntoResponse for ListResponse<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}