use axum::extract::State;
use tracing::info;

use crate::{
    app::{error::AppError, extractors::valid_json::ValidJson, state::AppState},
    dto::question_dto::{CreateQuestionRequest, CreateQuestionResponse, QuestionsListResponse},
};

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/v1/questions", axum::routing::post(create_question))
        .route("/v1/questions", axum::routing::get(get_questions))
}

/// Handler for creating a new question.
async fn create_question(
    State(state): State<AppState>,
    ValidJson(payload): ValidJson<CreateQuestionRequest>,
) -> Result<CreateQuestionResponse, AppError> {

    info!("Got request to create question {:?}", payload);

    // Logic to create a new question
    Err(AppError::InternalServerError(
        "Question creation not implemented".to_string(),
    ))
}

/// Handler for retrieving a list of questions.
async fn get_questions(
    State(state): State<AppState>,
) -> Result<QuestionsListResponse, AppError> {
    // Logic to retrieve a list of questions
    Err(AppError::InternalServerError(
        "Question retrieval not implemented".to_string(),
    ))
}
