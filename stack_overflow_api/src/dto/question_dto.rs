use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateQuestionRequest {
    #[validate(length(min = 1, max = 255, message = "title must not be empty"))]
    pub title: String,
    #[validate(length(min = 1, max = 5000, message = "body must not be empty"))]
    pub body: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuestionResponse {
    pub id: Uuid
}

impl CreateQuestionResponse {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}

impl IntoResponse for CreateQuestionResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::CREATED, axum::Json(self)).into_response()
    }
}

#[derive(Debug, Serialize)]
pub struct QuestionResponse {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize)]
pub struct QuestionsListResponse {
    pub questions: Vec<QuestionResponse>,
}

impl IntoResponse for QuestionsListResponse {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::OK, axum::Json(self)).into_response()
    }
}